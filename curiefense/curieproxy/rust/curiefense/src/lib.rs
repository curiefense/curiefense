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

use std::sync::Arc;

use analyze::{APhase0, CfRulesArg};
use body::body_too_large;
use config::virtualtags::VirtualTags;
use config::with_config;
use grasshopper::Grasshopper;
use interface::stats::{SecpolStats, Stats, StatsCollect};
use interface::{Action, ActionType, AnalyzeResult, BlockReason, Decision, Location, Tags};
use logs::Logs;
use securitypolicy::match_securitypolicy;
use simple_executor::{Executor, Progress, Task};
use tagging::tag_request;
use utils::{map_request, RawRequest, RequestInfo};

use crate::config::hostmap::SecurityPolicy;

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
            logs.debug("Could not find useragent!");
        }
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
    let _x = Box::from_raw(ptr);
}

pub fn inspect_generic_request_map<GH: Grasshopper>(
    configpath: &str,
    mgh: Option<&GH>,
    raw: RawRequest,
    logs: &mut Logs,
) -> AnalyzeResult {
    async_std::task::block_on(inspect_generic_request_map_async(configpath, mgh, raw, logs))
}

// generic entry point when the request map has already been parsed
pub fn inspect_generic_request_map_init<GH: Grasshopper>(
    configpath: &str,
    mgh: Option<&GH>,
    raw: RawRequest,
    logs: &mut Logs,
) -> Result<APhase0, AnalyzeResult> {
    let start = chrono::Utc::now();

    // insert the all tag here, to make sure it is always present, even in the presence of early errors
    let tags = Tags::from_slice(&[(String::from("all"), Location::Request)], VirtualTags::default());

    logs.debug(|| format!("Inspection starts (grasshopper active: {})", mgh.is_some()));

    #[allow(clippy::large_enum_variant)]
    enum RequestMappingResult<A> {
        NoSecurityPolicy,
        BodyTooLarge((Action, BlockReason), RequestInfo),
        Res(A),
    }

    // do all config queries in the lambda once
    // there is a lot of copying taking place, to minimize the lock time
    // this decision should be backed with benchmarks

    let ((mut ntags, globalfilter_dec, stats), flows, reqinfo, is_human) =
        match with_config(configpath, logs, |slogs, cfg| {
            let mmapinfo = match_securitypolicy(&raw.get_host(), &raw.meta.path, cfg, slogs);
            match mmapinfo {
                Some(secpolicy) => {
                    // this part is where we use the configuration as much as possible, while we have a lock on it

                    // check if the body is too large
                    // if the body is too large, we store the "too large" action for later use, and set the max depth to 0
                    let body_too_large = if let Some(body) = raw.mbody {
                        if body.len() > secpolicy.content_filter_profile.max_body_size
                            && !secpolicy.content_filter_profile.ignore_body
                        {
                            Some(body_too_large(
                                secpolicy.content_filter_profile.max_body_size,
                                body.len(),
                            ))
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    let stats = StatsCollect::new(cfg.revision.clone())
                        .secpol(SecpolStats::build(&secpolicy, cfg.globalfilters.len()));
                    // if the max depth is equal to 0, the body will not be parsed
                    let reqinfo = map_request(slogs, secpolicy, cfg.container_name.clone(), &raw, Some(start));

                    if let Some(action) = body_too_large {
                        return RequestMappingResult::BodyTooLarge(action, reqinfo);
                    }

                    let nflows = cfg.flows.clone();

                    // without grasshopper, default to being human
                    let is_human = if let Some(gh) = mgh {
                        challenge_verified(gh, &reqinfo, slogs)
                    } else {
                        false
                    };

                    let ntags = tag_request(stats, is_human, &cfg.globalfilters, &reqinfo, &cfg.virtual_tags);
                    RequestMappingResult::Res((ntags, nflows, reqinfo, is_human))
                }
                None => RequestMappingResult::NoSecurityPolicy,
            }
        }) {
            Some(RequestMappingResult::Res(x)) => x,
            Some(RequestMappingResult::BodyTooLarge((action, br), rinfo)) => {
                return Err(AnalyzeResult {
                    decision: Decision::action(action, vec![br]),
                    tags,
                    rinfo,
                    stats: Stats::default(),
                });
            }
            Some(RequestMappingResult::NoSecurityPolicy) => {
                logs.debug("No security policy found");
                let mut secpol = SecurityPolicy::default();
                secpol.content_filter_profile.ignore_body = true;
                let rinfo = map_request(logs, Arc::new(secpol), None, &raw, Some(start));
                return Err(AnalyzeResult {
                    decision: Decision::pass(Vec::new()),
                    tags,
                    rinfo,
                    stats: Stats::default(),
                });
            }
            None => {
                logs.debug("Something went wrong during security policy searching");
                let mut secpol = SecurityPolicy::default();
                secpol.content_filter_profile.ignore_body = true;
                let rinfo = map_request(logs, Arc::new(secpol), None, &raw, Some(start));
                return Err(AnalyzeResult {
                    decision: Decision::pass(Vec::new()),
                    tags,
                    rinfo,
                    stats: Stats::default(),
                });
            }
        };
    ntags.extend(tags);

    Ok(APhase0 {
        stats,
        itags: ntags,
        reqinfo,
        is_human,
        globalfilter_dec,
        flows,
    })
}

// generic entry point when the request map has already been parsed
pub async fn inspect_generic_request_map_async<GH: Grasshopper>(
    configpath: &str,
    mgh: Option<&GH>,
    raw: RawRequest<'_>,
    logs: &mut Logs,
) -> AnalyzeResult {
    match inspect_generic_request_map_init(configpath, mgh, raw, logs) {
        Err(res) => res,
        Ok(p0) => analyze::analyze(logs, mgh, p0, CfRulesArg::Global).await,
    }
}
