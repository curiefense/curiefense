pub mod acl;
pub mod body;
pub mod config;
pub mod flow;
pub mod interface;
pub mod limit;
pub mod logs;
pub mod maxmind;
pub mod redis;
pub mod requestfields;
pub mod session;
pub mod tagging;
pub mod urlmap;
pub mod utils;
pub mod waf;

use interface::Tags;
use serde_json::json;

use acl::{check_acl, AclDecision, AclResult, BotHuman};
use config::{with_config, HSDB};
use flow::flow_check;
use interface::{challenge_phase01, challenge_phase02, Action, ActionType, Decision, Grasshopper};
use limit::limit_check;
use logs::Logs;
use tagging::tag_request;
use urlmap::match_urlmap;
use utils::RequestInfo;
use waf::waf_check;

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

fn challenge_verified<GH: Grasshopper>(gh: &GH, reqinfo: &RequestInfo) -> bool {
    if let Some(rbzid) = reqinfo.cookies.get("rbzid") {
        if let Some(ua) = reqinfo.headers.get("user-agent") {
            return gh.parse_rbzid(&rbzid.replace('-', "="), ua).unwrap_or(false);
        }
    }
    false
}

// generic entry point when the request map has already been parsed
pub fn inspect_generic_request_map<GH: Grasshopper>(
  configpath: &str,
  mgh: Option<GH>,
  reqinfo: &RequestInfo,
  itags: Tags,
  logs: &mut Logs,
) -> (Decision, Tags) {
  let mut tags = itags;

  logs.debug("Inspection starts");
  // do all config queries in the lambda once
  // there is a lot of copying taking place, to minimize the lock time
  // this decision should be backed with benchmarks
  let ((nm, urlmap), ntags, flows) = match with_config(configpath, logs, |slogs, cfg| {
    let murlmap = match_urlmap(&reqinfo, cfg, slogs).map(|(nm, um)| (nm, um.clone()));
    let nflows = cfg.flows.clone();
    let ntags = tag_request(&cfg, &reqinfo);
    (murlmap, ntags, nflows)
  }) {
    Some((Some(stuff), itags, iflows)) => (stuff, itags, iflows),
    _ => {
      return (Decision::Pass, Tags::default());
    }
  };
  logs.debug("request tagged");
  tags.extend(ntags);
  tags.insert_qualified("urlmap", &nm);
  tags.insert_qualified("urlmap-entry", &urlmap.name);
  tags.insert_qualified("aclid", &urlmap.acl_profile.id);
  tags.insert_qualified("aclname", &urlmap.acl_profile.name);
  tags.insert_qualified("wafid", &urlmap.waf_profile.name);

  match flow_check(&flows, &reqinfo, &tags) {
    Err(rr) => logs.error(rr),
    Ok(Decision::Pass) => {}
    // TODO, check for monitor
    Ok(a) => return (a, tags),
  }
  logs.debug("flow checks done");

  if let Some(dec) = mgh.as_ref().and_then(|gh| {
    reqinfo
      .rinfo
      .qinfo
      .uri
      .as_ref()
      .and_then(|uri| challenge_phase02(gh, uri, &reqinfo.headers))
  }) {
    // TODO, check for monitor
    return (dec, tags);
  }

  // limit checks
  let limit_check = limit_check(logs, &urlmap.name, &reqinfo, &urlmap.limits, &mut tags);
  if let Decision::Action(_) = limit_check {
    // limit hit!
    return (limit_check, tags);
  }
  logs.debug(format!("limit checks done ({} limits)", urlmap.limits.len()));

  // store the check_acl result here
  let blockcode: Option<(i32, Vec<String>)> = match check_acl(&tags, &urlmap.acl_profile) {
    AclResult::Bypass(dec) => {
      if dec.allowed {
        logs.debug("ACL bypass detected");
        return (Decision::Pass, tags);
      } else {
        logs.debug("ACL force block detected");
        Some((0, dec.tags))
      }
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
      // if grasshopper is available, run these tests
      if let Some(gh) = mgh {
        if !challenge_verified(&gh, &reqinfo) {
          logs.debug("ACL challenge detected");
          match reqinfo.headers.get("user-agent") {
            None => Some((3, dtags)),
            Some(ua) => return (challenge_phase01(&gh, ua, dtags), tags),
          }
        } else {
          None
        }
      } else {
        None
      }
    }
    _ => None,
  };
  logs.debug(format!("ACL checks done {:?}", blockcode));

  // if the acl is active, and we had a block result, immediately block
  if urlmap.acl_active {
    if let Some((cde, tgs)) = blockcode {
      return (acl_block(true, cde, &tgs), tags);
    }
  }

  // otherwise, run waf_check
  let waf_result = match HSDB.read() {
    Ok(rd) => waf_check(&reqinfo, &urlmap.waf_profile, rd),
    Err(rr) => {
      logs.error(format!("Could not get lock on HSDB: {}", rr));
      Ok(())
    }
  };
  logs.debug("WAF checks done");

  (
    match waf_result {
      Ok(()) => {
        // if waf was ok, but we had an acl decision, return the monitored acl decision for logged purposes
        if let Some((cde, tgs)) = blockcode {
          acl_block(false, cde, &tgs)
        } else {
          Decision::Pass
        }
      }
      Err(wb) => {
        let mut action = wb.to_action();
        action.block_mode = urlmap.waf_active;
        Decision::Action(action)
      }
    },
    tags,
  )
}
