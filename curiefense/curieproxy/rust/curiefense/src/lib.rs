pub mod acl;
pub mod analyze;
pub mod body;
pub mod config;
pub mod contentfilter;
pub mod flow;
pub mod grasshopper;
pub mod incremental;
pub mod interface;
pub mod limit;
pub mod logs;
pub mod maxmind;
pub mod redis;
pub mod requestfields;
pub mod securitypolicy;
pub mod simple_executor;
pub mod tagging;
pub mod utils;

use analyze::CfRulesArg;
use body::body_too_large;
use config::{with_config, HSDB};
use contentfilter::content_filter_check;
use grasshopper::Grasshopper;
use interface::Tags;
use interface::{Action, ActionType, Decision};
use logs::Logs;
use securitypolicy::match_securitypolicy;
use simple_executor::{Executor, Progress, Task};
use tagging::tag_request;
use utils::{map_request, RawRequest, RequestInfo};

fn challenge_verified<GH: Grasshopper>(gh: &GH, reqinfo: &RequestInfo, logs: &mut Logs) -> bool {
    if let Some(rbzid) = reqinfo.cookies.get("rbzid") {
        if let Some(ua) = reqinfo.headers.get("user-agent") {
            logs.debug(|| format!("Checking rbzid cookie {} with user-agent {}", rbzid, ua));
            return match gh.parse_rbzid(&rbzid.replace('-', "="), ua) {
                Some(b) => b,
                None => {
                    logs.error("Something when wrong when calling parse_rbzid");
                    false
                }
            };
        } else {
            logs.warning("Could not find useragent!");
        }
    } else {
        logs.warning("Could not find rbzid cookie!")
    }
    false
}

/// # Safety
///
/// Steps a valid executor
pub unsafe fn inspect_async_step(ptr: *mut Executor<Task<(Decision, Tags, Logs)>>) -> Progress<(Decision, Tags, Logs)> {
    match ptr.as_ref() {
        None => Progress::Error("Null ptr".to_string()),
        Some(r) => r.step(),
    }
}

/// # Safety
///
/// Frees the executor, should be run with the output of executor_init, and only once
pub unsafe fn inspect_async_free(ptr: *mut Executor<(Decision, Tags, Logs)>) {
    if ptr.is_null() {
        return;
    }
    Box::from_raw(ptr);
}

pub fn inspect_generic_request_map<GH: Grasshopper>(
    configpath: &str,
    mgh: Option<GH>,
    raw: RawRequest,
    logs: &mut Logs,
) -> (Decision, Tags, RequestInfo) {
    async_std::task::block_on(inspect_generic_request_map_async(configpath, mgh, raw, logs))
}

// generic entry point when the request map has already been parsed
pub async fn inspect_generic_request_map_async<GH: Grasshopper>(
    configpath: &str,
    mgh: Option<GH>,
    raw: RawRequest<'_>,
    logs: &mut Logs,
) -> (Decision, Tags, RequestInfo) {
    let mut tags = Tags::default();

    // insert the all tag here, to make sure it is always present, even in the presence of early errors
    tags.insert("all");

    logs.debug(|| format!("Inspection starts (grasshopper active: {})", mgh.is_some()));

    #[allow(clippy::large_enum_variant)]
    enum RequestMappingResult<A> {
        NoSecurityPolicy,
        BodyTooLarge(Action, RequestInfo),
        Res(A),
    }

    // do all config queries in the lambda once
    // there is a lot of copying taking place, to minimize the lock time
    // this decision should be backed with benchmarks

    let ((nm, securitypolicy), (ntags, globalfilter_dec), flows, reqinfo, is_human) =
        match with_config(configpath, logs, |slogs, cfg| {
            let mmapinfo =
                match_securitypolicy(&raw.get_host(), &raw.meta.path, cfg, slogs).map(|(nm, um)| (nm, um.clone()));
            match mmapinfo {
                Some((nm, secpolicy)) => {
                    // this part is where we use the configuration as much as possible, while we have a lock on it
                    let pmax_depth = secpolicy.content_filter_profile.max_body_depth;

                    // check if the body is too large
                    // if the body is too large, we store the "too large" action for later use, and set the max depth to 0
                    let (body_too_large, max_depth) = if let Some(body) = raw.mbody {
                        if body.len() > secpolicy.content_filter_profile.max_body_size {
                            (
                                Some(body_too_large(
                                    secpolicy.content_filter_profile.max_body_size,
                                    body.len(),
                                )),
                                0,
                            )
                        } else {
                            (None, pmax_depth)
                        }
                    } else {
                        (None, pmax_depth)
                    };

                    // if the max depth is equal to 0, the body will not be parsed
                    let reqinfo = map_request(
                        slogs,
                        &secpolicy.content_filter_profile.decoding,
                        &secpolicy.content_filter_profile.content_type,
                        max_depth,
                        &raw,
                    );

                    if let Some(action) = body_too_large {
                        return RequestMappingResult::BodyTooLarge(action, reqinfo);
                    }

                    let nflows = cfg.flows.clone();

                    // without grasshopper, default to being human
                    let is_human = if let Some(gh) = &mgh {
                        challenge_verified(gh, &reqinfo, slogs)
                    } else {
                        false
                    };

                    let ntags = tag_request(is_human, &cfg.globalfilters, &reqinfo);
                    RequestMappingResult::Res(((nm, secpolicy), ntags, nflows, reqinfo, is_human))
                }
                None => RequestMappingResult::NoSecurityPolicy,
            }
        }) {
            Some(RequestMappingResult::Res(x)) => x,
            Some(RequestMappingResult::BodyTooLarge(action, rinfo)) => {
                return (Decision::Action(action), tags, rinfo);
            }
            Some(RequestMappingResult::NoSecurityPolicy) => {
                logs.debug("No security policy found");
                return (Decision::Pass, tags, map_request(logs, &[], &[], 0, &raw));
            }
            None => {
                logs.debug("Something went wrong during security policy searching");
                return (Decision::Pass, tags, map_request(logs, &[], &[], 0, &raw));
            }
        };

    tags.extend(ntags);

    analyze::analyze(
        logs,
        mgh,
        tags,
        &nm,
        &securitypolicy,
        reqinfo,
        is_human,
        globalfilter_dec,
        &flows,
        CfRulesArg::Global,
    )
    .await
}

// generic entry point when the request map has already been parsed
pub fn content_filter_check_generic_request_map(
    configpath: &str,
    raw: &RawRequest,
    content_filter_id: &str,
    logs: &mut Logs,
) -> (Decision, RequestInfo, Tags) {
    let mut tags = Tags::default();
    logs.debug("Content Filter inspection starts");
    let waf_profile = match with_config(configpath, logs, |_slogs, cfg| {
        cfg.content_filter_profiles.get(content_filter_id).cloned()
    }) {
        Some(Some(prof)) => prof,
        _ => {
            logs.error("Content Filter profile not found");
            return (Decision::Pass, map_request(logs, &[], &[], 25, raw), tags);
        }
    };

    if let Some(body) = raw.mbody {
        if body.len() > waf_profile.max_body_size {
            logs.error("body too large, exiting early");
            let reqinfo = map_request(logs, &waf_profile.decoding, &[], 0, raw);
            return (
                Decision::Action(body_too_large(waf_profile.max_body_size, body.len())),
                reqinfo,
                tags,
            );
        }
    }

    let reqinfo = map_request(logs, &waf_profile.decoding, &[], waf_profile.max_body_depth, raw);

    let waf_result = match HSDB.read() {
        Ok(rd) => content_filter_check(logs, &mut tags, &reqinfo, &waf_profile, rd.get(content_filter_id)),
        Err(rr) => {
            logs.error(|| format!("Could not get lock on HSDB: {}", rr));
            Ok(())
        }
    };
    logs.debug("Content Filter checks done");

    (
        match waf_result {
            Ok(()) => Decision::Pass,
            Err(wb) => Decision::Action(wb.to_action()),
        },
        reqinfo,
        tags,
    )
}
