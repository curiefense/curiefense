use serde_json::json;
use std::collections::HashMap;

use crate::acl::{check_acl, AclDecision, AclResult, BotHuman};
use crate::config::flow::{FlowElement, SequenceKey};
use crate::config::hostmap::SecurityPolicy;
use crate::config::HSDB;
use crate::contentfilter::{content_filter_check, masking};
use crate::flow::flow_check;
use crate::grasshopper::{challenge_phase01, challenge_phase02, Grasshopper};
use crate::interface::{Action, ActionType, Decision, SimpleDecision, Tags};
use crate::limit::limit_check;
use crate::logs::Logs;
use crate::utils::{BodyDecodingResult, RequestInfo};

fn acl_block(blocking: bool, code: i32, tags: &[String]) -> Decision {
    Decision::Action(Action {
        atype: if blocking {
            ActionType::Block
        } else {
            ActionType::Monitor
        },
        block_mode: blocking,
        ban: false,
        status: 403,
        headers: None,
        reason: json!({"action": code, "initiator": "acl", "reason": tags }),
        content: "access denied".to_string(),
        extra_tags: None,
    })
}

#[allow(clippy::too_many_arguments)]
pub async fn analyze<GH: Grasshopper>(
    logs: &mut Logs,
    mgh: Option<GH>,
    itags: Tags,
    secpolname: &str,
    securitypolicy: &SecurityPolicy,
    reqinfo: RequestInfo,
    is_human: bool,
    globalfilter_dec: SimpleDecision,
    flows: &HashMap<SequenceKey, Vec<FlowElement>>,
) -> (Decision, Tags, RequestInfo) {
    let mut tags = itags;
    let masking_seed = &securitypolicy.content_filter_profile.masking_seed;

    logs.debug("request tagged");
    tags.insert_qualified("securitypolicy", secpolname);
    tags.insert_qualified("securitypolicy-entry", &securitypolicy.name);
    tags.insert_qualified("aclid", &securitypolicy.acl_profile.id);
    tags.insert_qualified("aclname", &securitypolicy.acl_profile.name);
    tags.insert_qualified("contentfilterid", &securitypolicy.content_filter_profile.id);
    tags.insert_qualified("contentfiltername", &securitypolicy.content_filter_profile.name);

    if !securitypolicy.content_filter_profile.content_type.is_empty()
        && reqinfo.rinfo.qinfo.body_decoding != BodyDecodingResult::ProperlyDecoded
    {
        let error: &str = if let BodyDecodingResult::DecodingFailed(rr) = &reqinfo.rinfo.qinfo.body_decoding {
            rr
        } else {
            "Expected a body, but there were none"
        };
        // we expect the body to be properly decoded
        let action = Action {
            reason: json!({
                "initiator": "body_decoding",
                "error": error
            }),
            status: 403,
            ..Action::default()
        };
        return (
            Decision::Action(action),
            tags,
            masking(masking_seed, reqinfo, &securitypolicy.content_filter_profile),
        );
    }

    if let Some(dec) = mgh
        .as_ref()
        .and_then(|gh| challenge_phase02(gh, &reqinfo.rinfo.qinfo.uri, &reqinfo.headers))
    {
        return (
            dec,
            tags,
            masking(masking_seed, reqinfo, &securitypolicy.content_filter_profile),
        );
    }
    logs.debug("challenge phase2 ignored");

    if let SimpleDecision::Action(action, reason) = globalfilter_dec {
        logs.debug(|| format!("Global filter decision {:?}", reason));
        let decision = action.to_decision(is_human, &mgh, &reqinfo.headers, reason);
        if decision.is_final() {
            return (
                decision,
                tags,
                masking(masking_seed, reqinfo, &securitypolicy.content_filter_profile),
            );
        }
    }

    match flow_check(logs, flows, &reqinfo, &mut tags).await {
        Err(rr) => logs.error(|| rr.to_string()),
        Ok(SimpleDecision::Pass) => {}
        Ok(SimpleDecision::Action(a, reason)) => {
            let decision = a.to_decision(is_human, &mgh, &reqinfo.headers, reason);
            if decision.is_final() {
                return (
                    decision,
                    tags,
                    masking(masking_seed, reqinfo, &securitypolicy.content_filter_profile),
                );
            }
        }
    }
    logs.debug("flow checks done");

    // limit checks
    let limit_check = limit_check(logs, &securitypolicy.name, &reqinfo, &securitypolicy.limits, &mut tags);
    if let SimpleDecision::Action(action, reason) = limit_check.await {
        let decision = action.to_decision(is_human, &mgh, &reqinfo.headers, reason);
        if decision.is_final() {
            return (
                decision,
                tags,
                masking(masking_seed, reqinfo, &securitypolicy.content_filter_profile),
            );
        }
    }
    logs.debug(|| format!("limit checks done ({} limits)", securitypolicy.limits.len()));

    let acl_result = check_acl(&tags, &securitypolicy.acl_profile);
    logs.debug(|| format!("ACL result: {:?}", acl_result));
    // store the check_acl result here
    let blockcode: Option<(i32, Vec<String>)> = match acl_result {
        AclResult::Passthrough(dec) => {
            if dec.allowed {
                logs.debug("ACL passthrough detected");
                return (
                    Decision::Pass,
                    tags,
                    masking(masking_seed, reqinfo, &securitypolicy.content_filter_profile),
                );
            } else {
                logs.debug("ACL force block detected");
                Some((0, dec.tags))
            }
        }
        // bot blocked, human blocked
        // effect would be identical to the following case except for logging purpose
        AclResult::Match(BotHuman {
            bot: Some(AclDecision {
                allowed: false,
                tags: bot_tags,
            }),
            human: Some(AclDecision {
                allowed: false,
                tags: human_tags,
            }),
        }) => {
            logs.debug("ACL human block detected");
            Some((5, if is_human { human_tags } else { bot_tags }))
        }
        // human blocked, always block, even if it is a bot
        AclResult::Match(BotHuman {
            bot: _,
            human: Some(AclDecision {
                allowed: false,
                tags: dtags,
            }),
        }) => {
            logs.debug("ACL human block detected");
            Some((5, dtags))
        }
        // robot blocked, should be challenged
        AclResult::Match(BotHuman {
            bot: Some(AclDecision {
                allowed: false,
                tags: dtags,
            }),
            human: _,
        }) => {
            if is_human {
                None
            } else {
                match (reqinfo.headers.get("user-agent"), &mgh) {
                    (Some(ua), Some(gh)) => {
                        logs.debug("ACL challenge detected: challenged");
                        return (
                            challenge_phase01(gh, ua, dtags),
                            tags,
                            masking(masking_seed, reqinfo, &securitypolicy.content_filter_profile),
                        );
                    }
                    (gua, ggh) => {
                        logs.debug(|| {
                            format!(
                                "ACL challenge detected: can't challenge, ua={} gh={}",
                                gua.is_some(),
                                ggh.is_some()
                            )
                        });
                        Some((3, dtags))
                    }
                }
            }
        }
        _ => None,
    };
    logs.debug(|| format!("ACL checks done {:?}", blockcode));

    // if the acl is active, and we had a block result, immediately block
    if securitypolicy.acl_active {
        if let Some((cde, tgs)) = blockcode {
            return (
                acl_block(true, cde, &tgs),
                tags,
                masking(masking_seed, reqinfo, &securitypolicy.content_filter_profile),
            );
        }
    }

    // otherwise, run content_filter_check
    let content_filter_result = match HSDB.read() {
        Ok(rd) => content_filter_check(
            logs,
            &mut tags,
            &reqinfo,
            &securitypolicy.content_filter_profile,
            rd.get(&securitypolicy.content_filter_profile.id),
        ),
        Err(rr) => {
            logs.error(|| format!("Could not get lock on HSDB: {}", rr));
            Ok(())
        }
    };
    logs.debug("Content Filter checks done");

    (
        match content_filter_result {
            Ok(()) => {
                // if content filter was ok, but we had an acl decision, return the monitored acl decision for logged purposes
                if let Some((cde, tgs)) = blockcode {
                    acl_block(false, cde, &tgs)
                } else {
                    Decision::Pass
                }
            }
            Err(wb) => {
                let mut action = wb.to_action();
                action.block_mode &= securitypolicy.content_filter_active;
                Decision::Action(action)
            }
        },
        tags,
        masking(masking_seed, reqinfo, &securitypolicy.content_filter_profile),
    )
}
