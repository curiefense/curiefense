use std::collections::HashMap;

/* this module exposes an incremental interface to analyzing requests

   It works on the assumption that the `RequestMeta` can always be
   computed during the first stage of parsing. In particular, this means
   the `host` header is always present during that stage. This seems to be
   the case for envoy in its external processing mode.
*/

use crate::{
    analyze::analyze,
    challenge_verified,
    config::{
        flow::{FlowElement, SequenceKey},
        globalfilter::GlobalFilterSection,
        hostmap::SecurityPolicy,
        Config,
    },
    grasshopper::Grasshopper,
    interface::{Decision, Tags},
    logs::{LogLevel, Logs},
    securitypolicy::match_securitypolicy,
    tagging::tag_request,
    utils::{map_request, RawRequest, RequestInfo, RequestMeta},
};

pub struct IData<'t> {
    logs: Logs,
    meta: RequestMeta,
    headers: HashMap<String, String>,
    secpol: &'t SecurityPolicy,
    body: Option<Vec<u8>>,
}

pub fn inspect_init(config: &Config, loglevel: LogLevel, meta: RequestMeta) -> Result<IData, String> {
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
            secpol,
            body: None,
        }),
    }
}

/// TODO, incremental filtering of headers based on the security policy
/// but this raise the question of the log content that will not be complete,
/// for example tagging will not have taken place yet
pub fn add_header(idata: &mut IData, new_headers: HashMap<String, String>) -> Option<(Decision, Tags, RequestInfo)> {
    idata.headers.extend(new_headers);
    None
}

/// TODO, incremental filtering of body based on the security policy (mainly body length)
/// but this raise the question of the log content that will not be complete,
/// for example tagging will not have taken place yet
pub fn add_body(idata: &mut IData, new_body: Vec<u8>) -> Option<(Decision, Tags, RequestInfo)> {
    match idata.body.as_mut() {
        None => idata.body = Some(new_body),
        Some(b) => b.extend(new_body),
    }
    None
}

pub async fn finalize<'t, GH: Grasshopper>(
    idata: IData<'t>,
    mgh: Option<GH>,
    globalfilters: &[GlobalFilterSection],
    trusted_hops: u32,
    flows: &HashMap<SequenceKey, Vec<FlowElement>>,
) -> (Decision, Tags, RequestInfo) {
    let mut logs = idata.logs;
    let secpolicy = idata.secpol;
    let rawrequest = RawRequest {
        ipstr: extract_ip(trusted_hops as usize, &idata.headers),
        headers: idata.headers,
        meta: idata.meta,
        mbody: idata.body.as_deref(),
    };
    let reqinfo = map_request(
        &mut logs,
        &secpolicy.content_filter_profile.decoding,
        &secpolicy.content_filter_profile.content_type,
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
    analyze(
        &mut logs,
        mgh,
        tags,
        &secpolicy.name,
        secpolicy,
        reqinfo,
        is_human,
        globalfilter_dec,
        flows,
    )
    .await
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
