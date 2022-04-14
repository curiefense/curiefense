use std::collections::HashMap;

/* this module exposes an incremental interface to analyzing requests

   It works on the assumption that the `RequestMeta` can always be
   computed during the first stage of parsing. In particular, this means
   the `host` header is always present during that stage. This seems to be
   the case for envoy in its external processing mode.
*/

use crate::{
    analyze::{analyze, CfRulesArg},
    body::body_too_large,
    challenge_verified,
    config::{
        contentfilter::ContentFilterRules,
        contentfilter::SectionIdx,
        flow::{FlowElement, SequenceKey},
        globalfilter::GlobalFilterSection,
        hostmap::SecurityPolicy,
        Config,
    },
    contentfilter::ContentFilterBlock,
    grasshopper::Grasshopper,
    interface::{Action, Decision, Tags},
    logs::{LogLevel, Logs},
    securitypolicy::match_securitypolicy,
    tagging::tag_request,
    utils::{map_request, RawRequest, RequestInfo, RequestMeta},
};

pub struct IData {
    pub logs: Logs,
    meta: RequestMeta,
    headers: HashMap<String, String>,
    secpol: SecurityPolicy,
    body: Option<Vec<u8>>,
    trusted_hops: u32,
}

pub fn inspect_init(
    config: &Config,
    loglevel: LogLevel,
    meta: RequestMeta,
    trusted_hops: u32,
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
        Some((_, secpol)) => Ok(IData {
            logs,
            meta,
            headers: HashMap::new(),
            secpol: secpol.clone(),
            body: None,
            trusted_hops,
        }),
    }
}

/// called when the content filter policy is violated
/// no tags are returned though!
fn early_block(idata: IData, action: Action) -> (Logs, (Decision, Tags, RequestInfo)) {
    let mut logs = idata.logs;
    let secpolicy = idata.secpol;
    let rawrequest = RawRequest {
        ipstr: extract_ip(idata.trusted_hops as usize, &idata.headers),
        headers: idata.headers,
        meta: idata.meta,
        mbody: idata.body.as_deref(),
    };
    let reqinfo = map_request(
        &mut logs,
        &secpolicy.content_filter_profile.decoding,
        &secpolicy.content_filter_profile.content_type,
        0,
        &rawrequest,
    );
    (logs, (Decision::Action(action), Tags::default(), reqinfo))
}

/// incrementally add headers, can exit early if there are too many headers, or they are too large
///
/// other properties are not checked at this point (restrict for example), this early check purely exists as an anti DOS measure
pub fn add_header(
    idata: IData,
    new_headers: HashMap<String, String>,
) -> Result<IData, (Logs, (Decision, Tags, RequestInfo))> {
    let mut dt = idata;
    if dt.secpol.content_filter_active {
        let hdrs = &dt.secpol.content_filter_profile.sections.headers;
        if dt.headers.len() + new_headers.len() > hdrs.max_count {
            return Err(early_block(
                dt,
                ContentFilterBlock::TooManyEntries(SectionIdx::Headers).to_action(),
            ));
        }
        for (k, v) in new_headers {
            let kl = k.to_lowercase();
            if kl == "content-length" {
                if let Ok(content_length) = v.parse::<usize>() {
                    let max_size = dt.secpol.content_filter_profile.max_body_size;
                    if content_length > max_size {
                        return Err(early_block(dt, body_too_large(max_size, content_length)));
                    }
                }
            }
            if v.len() > hdrs.max_length {
                return Err(early_block(
                    dt,
                    ContentFilterBlock::EntryTooLarge(SectionIdx::Headers, kl).to_action(),
                ));
            }
            dt.headers.insert(kl, v);
        }
    } else {
        dt.headers.extend(new_headers);
    }
    Ok(dt)
}

/// TODO, incremental filtering of body based on the security policy (mainly body length)
pub fn add_body(idata: IData, new_body: Vec<u8>) -> Result<IData, (Logs, (Decision, Tags, RequestInfo))> {
    let mut dt = idata;
    let cur_body_size = dt.body.as_ref().map(|v| v.len()).unwrap_or(0);
    let new_size = cur_body_size + new_body.len();
    let max_size = dt.secpol.content_filter_profile.max_body_size;
    if new_size > max_size {
        return Err(early_block(dt, body_too_large(max_size, new_size)));
    }

    match dt.body.as_mut() {
        None => dt.body = Some(new_body),
        Some(b) => b.extend(new_body),
    }
    Ok(dt)
}

pub async fn finalize<GH: Grasshopper>(
    idata: IData,
    mgh: Option<GH>,
    globalfilters: &[GlobalFilterSection],
    flows: &HashMap<SequenceKey, Vec<FlowElement>>,
    mcfrules: Option<&HashMap<String, ContentFilterRules>>,
) -> ((Decision, Tags, RequestInfo), Logs) {
    let mut logs = idata.logs;
    let secpolicy = idata.secpol;
    let rawrequest = RawRequest {
        ipstr: extract_ip(idata.trusted_hops as usize, &idata.headers),
        headers: idata.headers,
        meta: idata.meta,
        mbody: idata.body.as_deref(),
    };
    let reqinfo = map_request(
        &mut logs,
        &secpolicy.content_filter_profile.decoding,
        &secpolicy.content_filter_profile.content_type,
        secpolicy.content_filter_profile.max_body_depth,
        &rawrequest,
    );

    // without grasshopper, default to being human
    let is_human = if let Some(gh) = &mgh {
        challenge_verified(gh, &reqinfo, &mut logs)
    } else {
        false
    };

    let (mut tags, globalfilter_dec) = tag_request(is_human, globalfilters, &reqinfo);
    tags.insert("all");
    let dec = analyze(
        &mut logs,
        mgh,
        tags,
        &secpolicy.name,
        &secpolicy,
        reqinfo,
        is_human,
        globalfilter_dec,
        flows,
        mcfrules
            .map(|cfrules| CfRulesArg::Get(cfrules.get(&secpolicy.content_filter_profile.id)))
            .unwrap_or(CfRulesArg::Global),
    )
    .await;
    (dec, logs)
}

fn extract_ip(trusted_hops: usize, headers: &HashMap<String, String>) -> String {
    let detect_ip = |xff: &str| -> String {
        let splitted = xff.split(',').collect::<Vec<_>>();
        if trusted_hops < splitted.len() {
            splitted[splitted.len() - trusted_hops]
        } else {
            splitted[0]
        }
        .to_string()
    };
    headers
        .get("x-forwarded-for")
        .map(|s| detect_ip(s.as_str()))
        .unwrap_or_else(|| "1.1.1.1".to_string())
}

#[cfg(test)]
mod test {
    use crate::config::{contentfilter::ContentFilterProfile, hostmap::HostMap, raw::AclProfile};
    use std::time::SystemTime;

    use super::*;

    fn empty_config(cf: ContentFilterProfile) -> Config {
        Config {
            securitypolicies: Vec::new(),
            globalfilters: Vec::new(),
            default: Some(HostMap {
                id: "__default__".to_string(),
                name: "default".to_string(),
                entries: Vec::new(),
                default: Some(SecurityPolicy {
                    name: "default".to_string(),
                    acl_active: false,
                    acl_profile: AclProfile::default(),
                    content_filter_active: true,
                    content_filter_profile: cf,
                    limits: Vec::new(),
                }),
            }),
            last_mod: SystemTime::now(),
            container_name: None,
            flows: HashMap::new(),
            content_filter_profiles: HashMap::new(),
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
            },
            1,
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
        let idata = add_header(idata, HashMap::new()).unwrap();
        // adding one header
        let idata = add_header(idata, hashmap(&[("k1", "v1")])).unwrap();
        let idata = add_header(idata, hashmap(&[("k2", "v2")])).unwrap();
        let idata = add_header(idata, hashmap(&[("k3", "v3")])).unwrap();
        let idata = add_header(idata, hashmap(&[("k4", "v4")]));
        assert!(idata.is_err())
    }

    #[test]
    fn not_too_many_headers() {
        let mut cf = ContentFilterProfile::default_from_seed("seed");
        cf.sections.headers.max_count = 3;
        let cfg = empty_config(cf);
        let idata = mk_idata(&cfg);
        // adding no headers
        let idata = add_header(idata, HashMap::new()).unwrap();
        // adding one header
        let idata = add_header(idata, hashmap(&[("k1", "v1"), ("k2", "v2"), ("k3", "v3")]));
        assert!(idata.is_ok())
    }

    #[test]
    fn way_too_many_headers() {
        let mut cf = ContentFilterProfile::default_from_seed("seed");
        cf.sections.headers.max_count = 3;
        let cfg = empty_config(cf);
        let idata = mk_idata(&cfg);
        // adding no headers
        let idata = add_header(idata, HashMap::new()).unwrap();
        // adding one header
        let idata = add_header(
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
        let idata = add_header(idata, HashMap::new()).unwrap();
        // adding one header
        let idata = add_header(
            idata,
            hashmap(&[("k1", "v1"), ("k2", "v2"), ("k3", "v3"), ("k4", "v4"), ("k5", "v5")]),
        )
        .unwrap();
        let idata = add_header(idata, hashmap(&[("kn", "DQSQSDQSDQSDQSD")]));
        assert!(idata.is_err())
    }

    #[test]
    fn body_too_large_cl() {
        let mut cf = ContentFilterProfile::default_from_seed("seed");
        cf.max_body_size = 100;
        let cfg = empty_config(cf);
        let idata = mk_idata(&cfg);
        let idata = add_header(idata, hashmap(&[("content-length", "150"), ("k4", "v4"), ("k5", "v5")]));
        assert!(idata.is_err())
    }

    #[test]
    fn body_too_large_body() {
        let mut cf = ContentFilterProfile::default_from_seed("seed");
        cf.max_body_size = 100;
        let cfg = empty_config(cf);
        let idata = mk_idata(&cfg);
        let idata = add_header(idata, hashmap(&[("content-length", "90"), ("k4", "v4"), ("k5", "v5")])).unwrap();
        let idata = add_body(idata, vec![4, 5, 6, 8]).unwrap();
        let mut emptybody: Vec<u8> = Vec::new();
        emptybody.resize(50, 66);
        let idata = add_body(idata, emptybody.clone()).unwrap();
        let idata = add_body(idata, emptybody);
        assert!(idata.is_err())
    }
}
