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

    // do all config queries in the lambda once
    // there is a lot of copying taking place, to minimize the lock time
    // this decision should be backed with benchmarks

    let ((nm, securitypolicy), (ntags, globalfilter_dec), flows, reqinfo, is_human) =
        match with_config(configpath, logs, |slogs, cfg| {
            let mmapinfo =
                match_securitypolicy(&raw.get_host(), &raw.meta.path, cfg, slogs).map(|(nm, um)| (nm, um.clone()));
            match mmapinfo {
                Some((nm, secpolicy)) => {
                    let reqinfo = map_request(
                        slogs,
                        &secpolicy.content_filter_profile.decoding,
                        &secpolicy.content_filter_profile.content_type,
                        &raw,
                    );
                    let nflows = cfg.flows.clone();

                    // without grasshopper, default to being human
                    let is_human = if let Some(gh) = &mgh {
                        challenge_verified(gh, &reqinfo, slogs)
                    } else {
                        false
                    };

                    let ntags = tag_request(is_human, &cfg.globalfilters, &reqinfo);
                    Some(((nm, secpolicy), ntags, nflows, reqinfo, is_human))
                }
                None => {
                    slogs.error("Could not find a security policy");
                    None
                }
            }
        }) {
            Some(Some(x)) => x,
            Some(None) => {
                logs.debug("Something went wrong during request tagging");
                return (Decision::Pass, tags, map_request(logs, &[], &[], &raw));
            }
            None => {
                logs.debug("Something went wrong during security policy searching");
                return (Decision::Pass, tags, map_request(logs, &[], &[], &raw));
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
            return (Decision::Pass, map_request(logs, &[], &[], raw), tags);
        }
    };

    let reqinfo = map_request(logs, &waf_profile.decoding, &[], raw);

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
