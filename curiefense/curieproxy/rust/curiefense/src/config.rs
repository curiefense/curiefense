pub mod flow;
pub mod hostmap;
pub mod limit;
pub mod profiling;
pub mod raw;
pub mod utils;
pub mod waf;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::SystemTime;

use crate::logs::Logs;
use flow::{flow_resolve, FlowElement, SequenceKey};
use hostmap::{HostMap, UrlMap};
use limit::{limit_order, Limit};
use profiling::ProfilingSection;
use raw::{AclProfile, RawFlowEntry, RawHostMap, RawLimit, RawProfilingSection, RawUrlMap, RawWafProfile};
use utils::Matching;
use waf::{resolve_signatures, WafProfile, WafSignatures};

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::empty());
    pub static ref HSDB: RwLock<Option<WafSignatures>> = RwLock::new(None);
}

pub fn with_config<R, F>(basepath: &str, logs: &mut Logs, f: F) -> Option<R>
where
    F: FnOnce(&mut Logs, &Config) -> R,
{
    let (newconfig, newhsdb) = match CONFIG.read() {
        Ok(cfg) => match cfg.reload(logs, basepath) {
            None => return Some(f(logs, &cfg)),
            Some(cfginfo) => cfginfo,
        },
        Err(rr) =>
        // read failed :(
        {
            logs.error(rr);
            return None;
        }
    };
    let r = f(logs, &newconfig);
    match CONFIG.write() {
        Ok(mut w) => *w = newconfig,
        Err(rr) => logs.error(rr),
    };
    match HSDB.write() {
        Ok(mut dbw) => *dbw = Some(newhsdb),
        Err(rr) => logs.error(rr),
    };
    Some(r)
}

pub fn with_config_default_path<R, F>(logs: &mut Logs, f: F) -> Option<R>
where
    F: FnOnce(&mut Logs, &Config) -> R,
{
    with_config("/config/current/config", logs, f)
}

#[derive(Debug, Clone)]
pub struct Config {
    pub urlmaps: Vec<Matching<HostMap>>,
    pub profiling: Vec<ProfilingSection>,
    pub default: Option<HostMap>,
    pub last_mod: SystemTime,
    pub container_name: Option<String>,
    pub flows: HashMap<SequenceKey, Vec<FlowElement>>,
}

fn from_map<V: Clone>(mp: &HashMap<String, V>, k: &str) -> Result<V, String> {
    mp.get(k).cloned().ok_or_else(|| {
        let all_keys: String = mp.keys().map(|s| s.as_str()).collect::<Vec<&str>>().join(",");
        format!("id not found: {}, all ids are: {}", k, all_keys)
    })
}

#[allow(clippy::too_many_arguments)]
impl Config {
    fn resolve_url_maps(
        logs: &mut Logs,
        rawmaps: Vec<RawUrlMap>,
        limits: &HashMap<String, Limit>,
        acls: &HashMap<String, AclProfile>,
        wafprofiles: &HashMap<String, WafProfile>,
    ) -> (Vec<Matching<UrlMap>>, Option<UrlMap>) {
        let mut default: Option<UrlMap> = None;
        let mut entries: Vec<Matching<UrlMap>> = Vec::new();

        for rawmap in rawmaps {
            let acl_profile: AclProfile = match acls.get(&rawmap.acl_profile) {
                Some(p) => p.clone(),
                None => {
                    logs.warning(format!("Unknown ACL profile {}", &rawmap.acl_profile));
                    AclProfile::default()
                }
            };
            let waf_profile: WafProfile = match wafprofiles.get(&rawmap.waf_profile) {
                Some(p) => p.clone(),
                None => {
                    logs.warning(format!("Unknown WAF profile {}", &rawmap.waf_profile));
                    WafProfile::default()
                }
            };
            let mut olimits: Vec<Limit> = Vec::new();
            for lid in rawmap.limit_ids {
                match from_map(&limits, &lid) {
                    Ok(lm) => olimits.push(lm),
                    Err(rr) => logs.error(format!("When resolving limits in rawmap {}, {}", rawmap.name, rr)),
                }
            }
            // limits 0 are tried first, than in decreasing order of the limit field
            olimits.sort_unstable_by(limit_order);
            let mapname = rawmap.name.clone();
            let urlmap = UrlMap {
                acl_active: rawmap.acl_active,
                acl_profile,
                waf_active: rawmap.waf_active,
                waf_profile,
                limits: olimits,
                name: rawmap.name,
            };
            if rawmap.match_ == "__default__" || (rawmap.match_ == "/" && urlmap.name == "default") {
                if default.is_some() {
                    logs.warning("Multiple __default__ maps");
                }
                default = Some(urlmap);
            } else {
                match Regex::new(&rawmap.match_) {
                    Err(rr) => logs.warning(format!(
                        "Invalid regex {} in entry {}: {}",
                        &rawmap.match_, &mapname, rr
                    )),
                    Ok(matcher) => entries.push(Matching { matcher, inner: urlmap }),
                };
            }
        }
        entries.sort_by_key(|x: &Matching<UrlMap>| usize::MAX - x.matcher.as_str().len());
        (entries, default)
    }

    fn resolve(
        logs: &mut Logs,
        last_mod: SystemTime,
        rawmaps: Vec<RawHostMap>,
        rawlimits: Vec<RawLimit>,
        rawprofiling: Vec<RawProfilingSection>,
        rawacls: Vec<AclProfile>,
        rawwafprofiles: Vec<RawWafProfile>,
        container_name: Option<String>,
        rawflows: Vec<RawFlowEntry>,
    ) -> Config {
        let mut default: Option<HostMap> = None;
        let mut urlmaps: Vec<Matching<HostMap>> = Vec::new();

        let limits = Limit::resolve(logs, rawlimits);
        let wafprofiles = WafProfile::resolve(logs, rawwafprofiles);
        let acls = rawacls.into_iter().map(|a| (a.id.clone(), a)).collect();

        // build the entries while looking for the default entry
        for rawmap in rawmaps {
            let (entries, default_entry) = Config::resolve_url_maps(logs, rawmap.map, &limits, &acls, &wafprofiles);
            if default_entry.is_none() {
                logs.warning(format!(
                    "HostMap entry '{}', id '{}' does not have a default entry",
                    rawmap.name, rawmap.id
                ));
            }
            let mapname = rawmap.name.clone();
            let hostmap = HostMap {
                id: rawmap.id,
                name: rawmap.name,
                entries,
                default: default_entry,
            };
            if rawmap.match_ == "__default__" {
                if default.is_some() {
                    logs.error(format!(
                        "HostMap entry '{}', id '{}' has several default entries",
                        hostmap.name, hostmap.id
                    ));
                }
                default = Some(hostmap);
            } else {
                match Regex::new(&rawmap.match_) {
                    Err(rr) => logs.error(format!("Invalid regex {} in entry {}: {}", &rawmap.match_, mapname, rr)),
                    Ok(matcher) => urlmaps.push(Matching {
                        matcher,
                        inner: hostmap,
                    }),
                }
            }
        }

        let profiling = ProfilingSection::resolve(logs, rawprofiling);

        let flows = flow_resolve(logs, rawflows);

        Config {
            urlmaps,
            profiling,
            default,
            last_mod,
            container_name,
            flows,
        }
    }

    fn load_config_file<A: serde::de::DeserializeOwned>(logs: &mut Logs, base: &Path, fname: &str) -> Vec<A> {
        let mut path = base.to_path_buf();
        path.push(fname);
        let fullpath = path.to_str().unwrap_or(fname).to_string();
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(rr) => {
                logs.error(format!("when loading {}: {}", fullpath, rr));
                return Vec::new();
            }
        };
        let values: Vec<serde_json::Value> = match serde_json::from_reader(std::io::BufReader::new(file)) {
            Ok(vs) => vs,
            Err(rr) => {
                // if it is not a json array, abort early and do not resolve anything
                logs.error(format!("when loading {}: {}", fullpath, rr));
                return Vec::new();
            }
        };
        let mut out = Vec::new();
        for value in values {
            // for each entry, try to resolve it as a raw configuration value, failing otherwise
            match serde_json::from_value(value) {
                Err(rr) => logs.error(format!("when loading {}: {}", fullpath, rr)),
                Ok(v) => out.push(v),
            }
        }
        out
    }

    pub fn reload(&self, logs: &mut Logs, basepath: &str) -> Option<(Config, WafSignatures)> {
        let last_mod = std::fs::metadata(basepath)
            .and_then(|x| x.modified())
            .unwrap_or_else(|rr| {
                logs.error(format!("Could not get last modified time for {}: {}", basepath, rr));
                SystemTime::now()
            });
        if self.last_mod == last_mod {
            return None;
        }

        logs.debug("Loading new configuration - CFGLOAD");
        let mut bjson = PathBuf::from(basepath);
        bjson.push("json");

        let urlmap = Config::load_config_file(logs, &bjson, "urlmap.json");
        let profiling = Config::load_config_file(logs, &bjson, "profiling-lists.json");
        let limits = Config::load_config_file(logs, &bjson, "limits.json");
        let acls = Config::load_config_file(logs, &bjson, "acl-profiles.json");
        let wafprofiles = Config::load_config_file(logs, &bjson, "waf-profiles.json");
        let wafsignatures = Config::load_config_file(logs, &bjson, "waf-signatures.json");
        let flows = Config::load_config_file(logs, &bjson, "flow-control.json");

        let container_name = std::fs::read_to_string("/etc/hostname")
            .ok()
            .map(|s| s.trim().to_string());
        let hsdb = resolve_signatures(wafsignatures).unwrap_or_else(|rr| {
            logs.error(rr);
            WafSignatures::empty()
        });
        let config = Config::resolve(
            logs,
            last_mod,
            urlmap,
            limits,
            profiling,
            acls,
            wafprofiles,
            container_name,
            flows,
        );
        Some((config, hsdb))
    }

    pub fn empty() -> Config {
        Config {
            urlmaps: Vec::new(),
            profiling: Vec::new(),
            last_mod: SystemTime::UNIX_EPOCH,
            default: None,
            container_name: None,
            flows: HashMap::new(),
        }
    }
}
