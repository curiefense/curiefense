/* this module exposes an incremental interface to analyzing requests

   It works on the assumption that the `RequestMeta` can always be
   computed during the first stage of parsing. In particular, this means
   the `host` header is always present during that stage. This seems to be
   the case for envoy in its external processing mode.
*/

use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};

use crate::{
    analyze::{analyze, APhase0, CfRulesArg},
    body::body_too_large,
    challenge_verified,
    config::{
        contentfilter::ContentFilterRules, contentfilter::SectionIdx, flow::FlowMap, globalfilter::GlobalFilterSection,
        hostmap::SecurityPolicy, virtualtags::VirtualTags, Config,
    },
    grasshopper::Grasshopper,
    interface::{
        stats::{BStageSecpol, SecpolStats, Stats, StatsCollect},
        Action, ActionType, AnalyzeResult, BlockReason, Decision, Location, Tags,
    },
    logs::{LogLevel, Logs},
    securitypolicy::match_securitypolicy,
    tagging::tag_request,
    utils::{map_request, RawRequest, RequestMeta},
};

pub enum IPInfo {
    Ip(String),
    Hops(usize),
}

pub struct IData {
    start: DateTime<Utc>,
    pub logs: Logs,
    meta: RequestMeta,
    headers: HashMap<String, String>,
    secpol: Arc<SecurityPolicy>,
    body: Option<Vec<u8>>,
    ipinfo: IPInfo,
    stats: StatsCollect<BStageSecpol>,
    container_name: Option<String>,
}

impl IData {
    fn ip(&self) -> String {
        match &self.ipinfo {
            IPInfo::Ip(s) => s.clone(),
            IPInfo::Hops(hops) => extract_ip(*hops, &self.headers).unwrap_or_else(|| "1.1.1.1".to_string()),
        }
    }
}

/// reproduces the original IP extraction algorithm, for envoy
pub fn extract_ip(trusted_hops: usize, headers: &HashMap<String, String>) -> Option<String> {
    let detect_ip = |xff: &str| -> String {
        let splitted = xff.split(',').collect::<Vec<_>>();
        if trusted_hops < splitted.len() {
            splitted[splitted.len() - trusted_hops]
        } else {
            splitted[0]
        }
        .to_string()
    };
    headers.get("x-forwarded-for").map(|s| detect_ip(s.as_str()))
}

pub fn inspect_init(
    config: &Config,
    loglevel: LogLevel,
    meta: RequestMeta,
    ipinfo: IPInfo,
    start: Option<DateTime<Utc>>,
) -> Result<IData, String> {
    let mut logs = Logs::new(loglevel);
    let mr = match_securitypolicy(
        meta.authority.as_deref().unwrap_or("localhost"),
        &meta.path,
        config,
        &mut logs,
    );
    match mr {
        None => Err("could not find a matching security policy".to_string()),
        Some(secpol) => {
            let stats = StatsCollect::new(config.revision.clone())
                .secpol(SecpolStats::build(&secpol, config.globalfilters.len()));
            Ok(IData {
                start: start.unwrap_or_else(Utc::now),
                logs,
                meta,
                headers: HashMap::new(),
                secpol,
                body: None,
                ipinfo,
                stats,
                container_name: config.container_name.clone(),
            })
        }
    }
}

/// called when the content filter policy is violated
/// no tags are returned though!
fn early_block(idata: IData, action: Action, br: BlockReason) -> (Logs, AnalyzeResult) {
    let ipstr = idata.ip();
    let mut logs = idata.logs;
    let secpolicy = idata.secpol;
    let rawrequest = RawRequest {
        ipstr,
        headers: idata.headers,
        meta: idata.meta,
        mbody: idata.body.as_deref(),
    };
    let reqinfo = map_request(
        &mut logs,
        secpolicy,
        idata.container_name,
        &rawrequest,
        Some(idata.start),
    );
    (
        logs,
        AnalyzeResult {
            decision: Decision::action(action, vec![br]),
            tags: Tags::new(&VirtualTags::default()),
            rinfo: reqinfo,
            stats: Stats::default(),
        },
    )
}

/// incrementally add headers, can exit early if there are too many headers, or they are too large
///
/// other properties are not checked at this point (restrict for example), this early check purely exists as an anti DOS measure
pub fn add_headers(idata: IData, new_headers: HashMap<String, String>) -> Result<IData, (Logs, AnalyzeResult)> {
    let mut dt = idata;
    for (k, v) in new_headers {
        dt = add_header(dt, k, v)?;
    }
    Ok(dt)
}

/// incrementally add a single header, can exit early if there are too many headers, or they are too large
///
/// other properties are not checked at this point (restrict for example), this early check purely exists as an anti DOS measure
pub fn add_header(idata: IData, key: String, value: String) -> Result<IData, (Logs, AnalyzeResult)> {
    let mut dt = idata;
    let cf_block = || Action {
        atype: ActionType::Block,
        block_mode: true,
        status: 403,
        headers: None,
        content: "Access denied".to_string(),
        extra_tags: None,
    };
    if dt.secpol.content_filter_active {
        let hdrs = &dt.secpol.content_filter_profile.sections.headers;
        if dt.headers.len() >= hdrs.max_count {
            let br = BlockReason::too_many_entries(SectionIdx::Headers, dt.headers.len() + 1, hdrs.max_count);
            return Err(early_block(dt, cf_block(), br));
        }
        let kl = key.to_lowercase();
        if kl == "content-length" {
            if let Ok(content_length) = value.parse::<usize>() {
                let max_size = dt.secpol.content_filter_profile.max_body_size;
                if content_length > max_size {
                    let (a, br) = body_too_large(max_size, content_length);
                    return Err(early_block(dt, a, br));
                }
            }
        }
        if value.len() > hdrs.max_length {
            let br = BlockReason::entry_too_large(SectionIdx::Headers, &kl, value.len(), hdrs.max_length);
            return Err(early_block(dt, cf_block(), br));
        }
        dt.headers.insert(kl, value);
    } else {
        dt.headers.insert(key.to_lowercase(), value);
    }
    Ok(dt)
}

pub fn add_body(idata: IData, new_body: &[u8]) -> Result<IData, (Logs, AnalyzeResult)> {
    let mut dt = idata;

    // ignore body when requested, even when the content filter is not active
    if dt.secpol.content_filter_profile.ignore_body {
        return Ok(dt);
    }

    let cur_body_size = dt.body.as_ref().map(|v| v.len()).unwrap_or(0);
    let new_size = cur_body_size + new_body.len();
    let max_size = dt.secpol.content_filter_profile.max_body_size;
    if dt.secpol.content_filter_active && new_size > max_size {
        let (a, br) = body_too_large(max_size, new_size);
        return Err(early_block(dt, a, br));
    }

    match dt.body.as_mut() {
        None => dt.body = Some(new_body.to_vec()),
        Some(b) => b.extend(new_body),
    }
    Ok(dt)
}

pub async fn finalize<GH: Grasshopper>(
    idata: IData,
    mgh: Option<&GH>,
    globalfilters: &[GlobalFilterSection],
    flows: &FlowMap,
    mcfrules: Option<&HashMap<String, ContentFilterRules>>,
    vtags: VirtualTags,
) -> (AnalyzeResult, Logs) {
    let ipstr = idata.ip();
    let mut logs = idata.logs;
    let secpolicy = idata.secpol;
    let rawrequest = RawRequest {
        ipstr,
        headers: idata.headers,
        meta: idata.meta,
        mbody: idata.body.as_deref(),
    };
    let cfrules = mcfrules
        .map(|cfrules| CfRulesArg::Get(cfrules.get(&secpolicy.content_filter_profile.id)))
        .unwrap_or(CfRulesArg::Global);
    let reqinfo = map_request(
        &mut logs,
        secpolicy,
        idata.container_name,
        &rawrequest,
        Some(idata.start),
    );

    // without grasshopper, default to being human
    let is_human = if let Some(gh) = mgh {
        challenge_verified(gh, &reqinfo, &mut logs)
    } else {
        false
    };

    let (mut tags, globalfilter_dec, stats) = tag_request(idata.stats, is_human, globalfilters, &reqinfo, &vtags);
    tags.insert("all", Location::Request);

    let dec = analyze(
        &mut logs,
        mgh,
        APhase0 {
            stats,
            itags: tags,
            reqinfo,
            is_human,
            globalfilter_dec,
            flows: flows.clone(),
        },
        cfrules,
    )
    .await;
    (dec, logs)
}

#[cfg(test)]
mod test {
    use crate::config::{
        contentfilter::ContentFilterProfile,
        hostmap::{HostMap, PolicyId},
        raw::AclProfile,
    };
    use std::time::SystemTime;

    use super::*;

    fn empty_config(cf: ContentFilterProfile) -> Config {
        Config {
            revision: "dummy".to_string(),
            securitypolicies: Vec::new(),
            globalfilters: Vec::new(),
            default: Some(HostMap {
                name: "default".to_string(),
                entries: Vec::new(),
                default: Some(Arc::new(SecurityPolicy {
                    policy: PolicyId {
                        id: "__default__".to_string(),
                        name: "default".to_string(),
                    },
                    entry: PolicyId {
                        id: "default".to_string(),
                        name: "default".to_string(),
                    },
                    acl_active: false,
                    acl_profile: AclProfile::default(),
                    content_filter_active: true,
                    content_filter_profile: cf,
                    session: Vec::new(),
                    session_ids: Vec::new(),
                    limits: Vec::new(),
                })),
            }),
            last_mod: SystemTime::now(),
            container_name: None,
            flows: HashMap::new(),
            content_filter_profiles: HashMap::new(),
            logs: Logs::default(),
            virtual_tags: Arc::new(HashMap::new()),
        }
    }

    fn hashmap(sl: &[(&str, &str)]) -> HashMap<String, String> {
        sl.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    fn mk_idata(cfg: &Config) -> IData {
        inspect_init(
            cfg,
            LogLevel::Debug,
            RequestMeta {
                authority: Some("authority".to_string()),
                method: "GET".to_string(),
                path: "/path/to/somewhere".to_string(),
                extra: HashMap::default(),
                requestid: None,
            },
            IPInfo::Ip("1.2.3.4".to_string()),
            None,
        )
        .unwrap()
    }

    #[test]
    fn too_many_headers_1() {
        let mut cf = ContentFilterProfile::default_from_seed("seed");
        cf.sections.headers.max_count = 3;
        let cfg = empty_config(cf);
        let idata = mk_idata(&cfg);
        // adding no headers
        let idata = add_headers(idata, HashMap::new()).unwrap();
        // adding one header
        let idata = add_headers(idata, hashmap(&[("k1", "v1")])).unwrap();
        let idata = add_headers(idata, hashmap(&[("k2", "v2")])).unwrap();
        let idata = add_headers(idata, hashmap(&[("k3", "v3")])).unwrap();
        let idata = add_headers(idata, hashmap(&[("k4", "v4")]));
        assert!(idata.is_err())
    }

    #[test]
    fn not_too_many_headers() {
        let mut cf = ContentFilterProfile::default_from_seed("seed");
        cf.sections.headers.max_count = 3;
        let cfg = empty_config(cf);
        let idata = mk_idata(&cfg);
        // adding no headers
        let idata = add_headers(idata, HashMap::new()).unwrap();
        // adding one header
        let idata = add_headers(idata, hashmap(&[("k1", "v1"), ("k2", "v2"), ("k3", "v3")]));
        assert!(idata.is_ok())
    }

    #[test]
    fn way_too_many_headers() {
        let mut cf = ContentFilterProfile::default_from_seed("seed");
        cf.sections.headers.max_count = 3;
        let cfg = empty_config(cf);
        let idata = mk_idata(&cfg);
        // adding no headers
        let idata = add_headers(idata, HashMap::new()).unwrap();
        // adding one header
        let idata = add_headers(
            idata,
            hashmap(&[("k1", "v1"), ("k2", "v2"), ("k3", "v3"), ("k4", "v4"), ("k5", "v5")]),
        );
        assert!(idata.is_err())
    }

    #[test]
    fn headers_too_large() {
        let mut cf = ContentFilterProfile::default_from_seed("seed");
        cf.sections.headers.max_length = 8;
        let cfg = empty_config(cf);
        let idata = mk_idata(&cfg);
        // adding no headers
        let idata = add_headers(idata, HashMap::new()).unwrap();
        // adding one header
        let idata = add_headers(
            idata,
            hashmap(&[("k1", "v1"), ("k2", "v2"), ("k3", "v3"), ("k4", "v4"), ("k5", "v5")]),
        )
        .unwrap();
        let idata = add_headers(idata, hashmap(&[("kn", "DQSQSDQSDQSDQSD")]));
        assert!(idata.is_err())
    }

    #[test]
    fn body_too_large_cl() {
        let mut cf = ContentFilterProfile::default_from_seed("seed");
        cf.max_body_size = 100;
        let cfg = empty_config(cf);
        let idata = mk_idata(&cfg);
        let idata = add_headers(idata, hashmap(&[("content-length", "150"), ("k4", "v4"), ("k5", "v5")]));
        assert!(idata.is_err())
    }

    #[test]
    fn body_too_large_body() {
        let mut cf = ContentFilterProfile::default_from_seed("seed");
        cf.max_body_size = 100;
        let cfg = empty_config(cf);
        let idata = mk_idata(&cfg);
        let idata = add_headers(idata, hashmap(&[("content-length", "90"), ("k4", "v4"), ("k5", "v5")])).unwrap();
        let idata = add_body(idata, &[4, 5, 6, 8]).unwrap();
        let mut emptybody: Vec<u8> = Vec::new();
        emptybody.resize(50, 66);
        let idata = add_body(idata, &emptybody).unwrap();
        let idata = add_body(idata, &emptybody);
        match idata {
            Ok(_) => panic!("should have failed"),
            Err((_, ar)) => assert_eq!(
                ar.rinfo.session,
                "a1f8270abe976ebef4cca2cb3c16c4ab38ca9219818d241f0ecc3d21"
            ),
        }
    }
}
