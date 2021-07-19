/// This module exposes a session based API for the matching system
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

use crate::acl::{check_acl, AclResult};
use crate::config::hostmap::UrlMap;
use crate::config::{with_config_default_path, Config, CONFIG, HSDB};
use crate::flow::flow_check;
use crate::interface::{Decision, Tags};
use crate::limit::limit_check;
use crate::logs::Logs;
use crate::requestfields::RequestField;
use crate::tagging::tag_request;
use crate::urlmap::match_urlmap;
use crate::utils::{find_geoip, QueryInfo, RInfo, RequestInfo, RequestMeta};
use crate::waf::waf_check;

// Session stuff, the key is the session id
lazy_static! {
    static ref RAW: RwLock<HashMap<Uuid, serde_json::Value>> = RwLock::new(HashMap::new());
    static ref RINFOS: RwLock<HashMap<Uuid, RequestInfo>> = RwLock::new(HashMap::new());
    static ref TAGS: RwLock<HashMap<Uuid, Tags>> = RwLock::new(HashMap::new());
    static ref URLMAP: RwLock<HashMap<Uuid, UrlMap>> = RwLock::new(HashMap::new());
}

/// json representation of the useful fields in the request map
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JRequestMap {
    headers: RequestField,
    cookies: RequestField,
    args: RequestField,
    attrs: JAttrs,
}

/// json representation of the useful fields in attrs
#[derive(Debug, Deserialize, Serialize, Clone)]
struct JAttrs {
    path: String,
    method: String,
    ip: String,
    query: String,
    authority: Option<String>,
    uri: String,
    tags: HashMap<String, serde_json::Value>,
}

impl JRequestMap {
    pub fn into_request_info(self) -> (RequestInfo, Tags) {
        let host: String = self
            .headers
            .get("host")
            .cloned()
            .or_else(|| self.attrs.authority.clone())
            .unwrap_or_else(|| "unknown".to_string());

        // TODO, get geoip data from the encoded request, not from the ip
        let geoip = find_geoip(self.attrs.ip);
        let meta = RequestMeta {
            authority: self.attrs.authority,
            method: self.attrs.method,
            path: self.attrs.uri.clone(), // this is wrong, uri should be url-encoded back
            extra: HashMap::new(),
        };
        let qinfo = QueryInfo {
            qpath: self.attrs.path,
            query: self.attrs.query,
            uri: Some(self.attrs.uri),
            args: self.args,
        };
        let vtags: Vec<String> = self.attrs.tags.into_iter().map(|(k, _)| k).collect();
        (
            RequestInfo {
                cookies: self.cookies,
                headers: self.headers,
                rinfo: RInfo {
                    meta,
                    geoip,
                    qinfo,
                    host,
                },
            },
            Tags::from_slice(&vtags),
        )
    }
}

pub fn init_config() -> (bool, Vec<String>) {
    let mut logs = Logs::default();
    with_config_default_path(&mut logs, |_, _| {});
    let is_ok = logs.logs.is_empty();
    (is_ok, logs.to_stringvec())
}

pub fn clean_session(session_id: &str) -> anyhow::Result<()> {
    let uuid: Uuid = session_id.parse()?;
    if let Ok(mut w) = RINFOS.write() {
        w.remove(&uuid);
    }
    if let Ok(mut w) = TAGS.write() {
        w.remove(&uuid);
    }
    if let Ok(mut w) = URLMAP.write() {
        w.remove(&uuid);
    }
    Ok(())
}

pub fn session_serialize_request_map(session_id: &str) -> anyhow::Result<serde_json::Value> {
    let uuid: Uuid = session_id.parse()?;
    // get raw request first
    let raw: serde_json::Value = match RAW.read() {
        Ok(raws) => match raws.get(&uuid) {
            Some(v) => v.clone(),
            None => return Err(anyhow::anyhow!("Could not get RAW {}", uuid)),
        },
        Err(rr) => return Err(anyhow::anyhow!("Could not get read lock on RAW {}", rr)),
    };

    // get the tags
    let tags = with_tags(uuid, |tgs| Ok(tgs.clone()))?;

    update_tags(raw, tags)
}

/// update the tags in the JSON-encoded request_map
pub fn update_tags(rawjson: serde_json::Value, tags: Tags) -> anyhow::Result<serde_json::Value> {
    let mut raw = rawjson;
    let tags_map: HashMap<String, u32> = tags.as_hash_ref().iter().map(|k| (k.clone(), 1)).collect();

    // update the tags
    let attrs = raw.get_mut("attrs").ok_or_else(|| anyhow::anyhow!("No attrs field"))?;
    let attrs_o = attrs
        .as_object_mut()
        .ok_or_else(|| anyhow::anyhow!("Attrs was not an object"))?;
    attrs_o.insert("tags".to_string(), serde_json::to_value(tags_map)?);

    Ok(raw)
}

/// initializes a session from a json-encoded request map
pub fn session_init(encoded_request_map: &str) -> anyhow::Result<String> {
    let jvalue: serde_json::Value = serde_json::from_str(encoded_request_map)?;
    let jmap: JRequestMap = serde_json::from_value(jvalue.clone())?;
    let (rinfo, tags) = jmap.into_request_info();

    let uuid = Uuid::new_v4();

    let mut raw = RAW
        .write()
        .map_err(|rr| anyhow::anyhow!("Could not get RAW write lock {}", rr))?;
    raw.insert(uuid, jvalue);
    let mut rinfos = RINFOS
        .write()
        .map_err(|rr| anyhow::anyhow!("Could not get RINFOS write lock {}", rr))?;
    rinfos.insert(uuid, rinfo);
    let mut wtags = TAGS
        .write()
        .map_err(|rr| anyhow::anyhow!("Could not get TAGS write lock {}", rr))?;
    wtags.insert(uuid, tags);

    Ok(format!("{}", uuid))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionUrlMap {
    pub name: String,
    pub acl_profile: String,
    pub waf_profile: String,
    pub acl_active: bool,
    pub waf_active: bool,
    pub limit_ids: Vec<String>,
    pub urlmap: String,
}

/// returns a RawUrlMap object (minus the match field), and updates the internal structure for the url map
pub fn session_match_urlmap(session_id: &str) -> anyhow::Result<SessionUrlMap> {
    let mut logs = Logs::default();
    let uuid: Uuid = session_id.parse()?;
    // this is done this way in order to release the config lock before writing the tags
    // this might not be optimal though, perhaps it is faster to keep the locks and avoir copies
    let (hostmap_name, urlmap) = with_config(|cfg| {
        with_request_info(uuid, |rinfo| match match_urlmap(&rinfo, &cfg, &mut logs) {
            Some((hn, urlmap)) => {
                let mut wurlmap = URLMAP
                    .write()
                    .map_err(|rr| anyhow::anyhow!("Could not get TAGS write lock {}", rr))?;
                wurlmap.insert(uuid, urlmap.clone());
                Ok((hn, urlmap.clone()))
            }
            None => Err(anyhow::anyhow!("No matching URL map")),
        })
    })?;
    with_tags_mut(uuid, |tags| {
        tags.insert_qualified("urlmap", &hostmap_name);
        tags.insert_qualified("urlmap-entry", &urlmap.name);
        tags.insert_qualified("aclid", &urlmap.acl_profile.id);
        tags.insert_qualified("aclname", &urlmap.acl_profile.name);
        tags.insert_qualified("wafid", &urlmap.waf_profile.id);
        tags.insert_qualified("wafname", &urlmap.waf_profile.name);
        Ok(())
    })?;
    let raw_urlmap = SessionUrlMap {
        name: urlmap.name,
        acl_profile: urlmap.acl_profile.id,
        waf_profile: urlmap.waf_profile.id,
        acl_active: urlmap.acl_active,
        waf_active: urlmap.waf_active,
        limit_ids: urlmap.limits.into_iter().map(|l| l.id).collect(),
        urlmap: hostmap_name,
    };
    Ok(raw_urlmap)
}

pub fn session_tag_request(session_id: &str) -> anyhow::Result<bool> {
    let uuid: Uuid = session_id.parse()?;

    // TODO: humanity is assumed
    let new_tags = with_config(|cfg| with_request_info(uuid, |rinfo| Ok(tag_request(true, &cfg, &rinfo))))?;
    with_tags_mut(uuid, |tgs| {
        // TODO: the decision is ignored, but this is going to be deprecated
        tgs.extend(new_tags.0);
        Ok(())
    })?;
    Ok(true)
}

pub fn session_limit_check(session_id: &str) -> anyhow::Result<Decision> {
    let uuid: Uuid = session_id.parse()?;

    // copy limits, without keeping a read lock
    let limits = with_urlmap(uuid, |urlmap| Ok(urlmap.limits.clone()))?;
    let mut logs = Logs::default();

    let sdecision = with_request_info(uuid, |rinfo| {
        with_urlmap(uuid, |urlmap| {
            with_tags_mut(uuid, |mut tags| {
                Ok(limit_check(&mut logs, &urlmap.name, &rinfo, &limits, &mut tags))
            })
        })
    });
    Ok(sdecision?.into_decision_no_challenge())
}

pub fn session_acl_check(session_id: &str) -> anyhow::Result<AclResult> {
    let uuid: Uuid = session_id.parse()?;

    with_urlmap(uuid, |urlmap| {
        with_tags(uuid, |tags| Ok(check_acl(tags, &urlmap.acl_profile)))
    })
}

pub fn session_waf_check(session_id: &str) -> anyhow::Result<Decision> {
    let uuid: Uuid = session_id.parse()?;

    let hsdb = HSDB.read().map_err(|rr| anyhow::anyhow!("{}", rr))?;

    with_request_info(uuid, |rinfo| {
        with_urlmap(uuid, |urlmap| {
            Ok(match waf_check(rinfo, &urlmap.waf_profile, hsdb) {
                Ok(()) => Decision::Pass,
                Err(rr) => Decision::Action(rr.to_action()),
            })
        })
    })
}

pub fn session_flow_check(session_id: &str) -> anyhow::Result<Decision> {
    let uuid: Uuid = session_id.parse()?;
    let mut logs = Logs::default();

    let sdecision = with_config(|cfg| {
        with_request_info(uuid, |rinfo| {
            with_tags_mut(uuid, |tags| flow_check(&mut logs, &cfg.flows, rinfo, tags))
        })
    });
    Ok(sdecision?.into_decision_no_challenge())
}
// HELPERS

fn with_config<F, A>(f: F) -> anyhow::Result<A>
where
    F: FnOnce(&Config) -> anyhow::Result<A>,
{
    match CONFIG.read() {
        Ok(cfg) => f(&cfg),
        Err(rr) => Err(anyhow::anyhow!("Could not get configuration read lock {}", rr)),
    }
}

fn with_request_info<F, A>(uuid: Uuid, f: F) -> anyhow::Result<A>
where
    F: FnOnce(&RequestInfo) -> anyhow::Result<A>,
{
    let infos = RINFOS
        .read()
        .map_err(|rr| anyhow::anyhow!("Could not get RINFOS read lock {}", rr))?;
    let rinfo = infos.get(&uuid).ok_or_else(|| anyhow::anyhow!("Unknown session id"))?;
    f(rinfo)
}

fn with_urlmap<F, A>(uuid: Uuid, f: F) -> anyhow::Result<A>
where
    F: FnOnce(&UrlMap) -> anyhow::Result<A>,
{
    let maps = URLMAP
        .read()
        .map_err(|rr| anyhow::anyhow!("Could not get URLMAP read lock {}", rr))?;
    let umap = maps.get(&uuid).ok_or_else(|| anyhow::anyhow!("Unknown session id"))?;
    f(umap)
}

fn with_tags<F, A>(uuid: Uuid, f: F) -> anyhow::Result<A>
where
    F: FnOnce(&Tags) -> anyhow::Result<A>,
{
    let tags = TAGS
        .read()
        .map_err(|rr| anyhow::anyhow!("Could not get TAGS read lock {}", rr))?;
    let tag = tags.get(&uuid).ok_or_else(|| anyhow::anyhow!("Unknown session id"))?;
    f(tag)
}

fn with_tags_mut<F, A>(uuid: Uuid, f: F) -> anyhow::Result<A>
where
    F: FnOnce(&mut Tags) -> anyhow::Result<A>,
{
    let mut tags = TAGS
        .write()
        .map_err(|rr| anyhow::anyhow!("Could not get TAGS read lock {}", rr))?;
    let tag = tags
        .get_mut(&uuid)
        .ok_or_else(|| anyhow::anyhow!("Unknown session id"))?;
    f(tag)
}
