/// this file contains all the data type that are used when interfacing with a proxy
use crate::config::matchers::RequestSelector;
use crate::config::raw::{RawAction, RawActionType};
use crate::grasshopper::{challenge_phase01, Grasshopper};
use crate::logs::Logs;
use crate::utils::templating::{parse_request_template, RequestTemplate, TVar, TemplatePart};
use crate::utils::{selector, RequestInfo, Selected};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub use self::block_reasons::*;
pub use self::stats::*;
pub use self::tagging::*;

pub mod block_reasons;
pub mod stats;
pub mod tagging;

#[derive(Debug, Clone)]
pub enum SimpleDecision {
    Pass,
    Action(SimpleAction, Vec<BlockReason>),
}

pub fn stronger_decision(d1: SimpleDecision, d2: SimpleDecision) -> SimpleDecision {
    match (&d1, &d2) {
        (SimpleDecision::Pass, _) => d2,
        (_, SimpleDecision::Pass) => d1,
        (SimpleDecision::Action(s1, _), SimpleDecision::Action(s2, _)) => {
            if s1.atype.priority() >= s2.atype.priority() {
                d1
            } else {
                d2
            }
        }
    }
}

#[derive(Debug)]
pub struct AnalyzeResult {
    pub decision: Decision,
    pub tags: Tags,
    pub rinfo: RequestInfo,
    pub stats: Stats,
}

#[derive(Debug, Clone)]
pub struct Decision {
    pub maction: Option<Action>,
    pub reasons: Vec<BlockReason>,
}

impl Decision {
    pub fn skip(initiator: Initiator, location: HashSet<Location>) -> Self {
        Decision {
            maction: None,
            reasons: vec![BlockReason {
                initiator,
                location,
                decision: BDecision::Skip,
            }],
        }
    }

    pub fn pass(reasons: Vec<BlockReason>) -> Self {
        Decision { maction: None, reasons }
    }

    pub fn action(action: Action, reasons: Vec<BlockReason>) -> Self {
        Decision {
            maction: Some(action),
            reasons,
        }
    }

    /// is the action blocking (not passed to the underlying server)
    pub fn is_blocking(&self) -> bool {
        self.maction.as_ref().map(|a| a.atype.is_blocking()).unwrap_or(false)
    }

    /// is the action final (no further processing)
    pub fn is_final(&self) -> bool {
        self.maction.as_ref().map(|a| a.atype.is_final()).unwrap_or(false)
            || self.reasons.iter().any(|r| r.decision == BDecision::Skip)
    }

    pub fn response_json(&self) -> String {
        let action_desc = match self.maction {
            None => "pass",
            Some(_) => "custom_response",
        };
        let response =
            serde_json::to_value(&self.maction).unwrap_or_else(|rr| serde_json::Value::String(rr.to_string()));
        let j = serde_json::json!({
            "action": action_desc,
            "response": response,
        });
        serde_json::to_string(&j).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn log_json(&self, rinfo: &RequestInfo, tags: &Tags, logs: &Logs, stats: &Stats) -> String {
        let (request_map, _) = jsonlog(
            self,
            Some(rinfo),
            self.maction.as_ref().map(|a| a.status),
            tags,
            stats,
            logs,
        );
        serde_json::to_string(&request_map).unwrap_or_else(|_| "{}".to_string())
    }
}

// helper function that reproduces the envoy log format
pub fn jsonlog(
    dec: &Decision,
    mrinfo: Option<&RequestInfo>,
    rcode: Option<u32>,
    tags: &Tags,
    stats: &Stats,
    logs: &Logs,
) -> (serde_json::Value, chrono::DateTime<chrono::Utc>) {
    let now = chrono::Utc::now();
    let mut tgs = tags.clone();
    if let Some(action) = &dec.maction {
        if let Some(extra) = &action.extra_tags {
            for t in extra {
                tgs.insert(t, Location::Request);
            }
        }
    }
    let block_reason_desc = BlockReason::block_reason_desc(&dec.reasons);
    let greasons = BlockReason::regroup(&dec.reasons);
    let get_trigger = |k: &InitiatorKind| -> &[&BlockReason] { greasons.get(k).map(|v| v.as_slice()).unwrap_or(&[]) };

    let stats_counter = |kd: InitiatorKind| -> (usize, usize) {
        match greasons.get(&kd) {
            None => (0, 0),
            Some(v) => (v.len(), v.iter().filter(|r| r.decision == BDecision::Blocking).count()),
        }
    };
    let (acl, acl_active) = stats_counter(InitiatorKind::Acl);
    let (global_filters, global_filters_active) = stats_counter(InitiatorKind::GlobalFilter);
    let (flow_control, flow_control_active) = stats_counter(InitiatorKind::FlowControl);
    let (rate_limit, rate_limit_active) = stats_counter(InitiatorKind::GlobalFilter);
    let (content_filters, content_filters_active) = stats_counter(InitiatorKind::ContentFilter);

    let val = match mrinfo {
        Some(info) => serde_json::json!({
            "timestamp": now,
            "request_id": info.rinfo.meta.requestid,
            "security_config": {
                "revision": stats.revision,
                "acl_active": stats.secpol.acl_enabled,
                "cf_active": stats.secpol.content_filter_enabled,
                "cf_rules": stats.content_filter_total,
                "rate_limit_rules": stats.secpol.limit_amount,
                "global_filters_active": stats.secpol.globalfilters_amount
            },
            "arguments": info.rinfo.qinfo.args.to_json(),
            "path": info.rinfo.qinfo.qpath,
            "path_parts": info.rinfo.qinfo.path_as_map.to_json(),
            "authority": info.rinfo.meta.authority,
            "cookies": info.cookies.to_json(),
            "headers": info.headers.to_json(),
            "tags": tgs.to_json(),
            "uri": info.rinfo.meta.path,
            "ip": info.rinfo.geoip.ip,
            "method": info.rinfo.meta.method,
            "response_code": rcode,
            "logs": logs.to_stringvec(),

            "processing_stage": stats.processing_stage,
            "trigger_counters": {
                "acl": acl,
                "acl_active": acl_active,
                "global_filters": global_filters,
                "global_filters_active": global_filters_active,
                "flow_control": flow_control,
                "flow_control_active": flow_control_active,
                "rate_limit": rate_limit,
                "rate_limit_active": rate_limit_active,
                "content_filters": content_filters,
                "content_filters_active": content_filters_active,
            },
            "acl_triggers": get_trigger(&InitiatorKind::Acl),
            "rate_limit_triggers": get_trigger(&InitiatorKind::RateLimit),
            "flow_control_triggers": get_trigger(&InitiatorKind::FlowControl),
            "global_filter_triggers": get_trigger(&InitiatorKind::GlobalFilter),
            "content_filter_triggers": get_trigger(&InitiatorKind::ContentFilter),
            "proxy": {
                "location": info.rinfo.geoip.location
            },
            "reason": block_reason_desc,
            "profiling": {},
            "biometrics": {},
        }),
        None => serde_json::Value::Null,
    };

    (val, now)
}

// an action, as formatted for outside consumption
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Action {
    pub atype: ActionType,
    pub ban: bool,
    pub block_mode: bool,
    pub status: u32,
    pub headers: Option<HashMap<String, String>>,
    pub content: String,
    pub extra_tags: Option<HashSet<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleActionT {
    Skip,
    Monitor,
    RequestHeader(HashMap<String, RequestTemplate>),
    Response {
        content: String,
        content_type: Option<String>,
    },
    Redirect(String),
    Challenge,
    Default,
    Ban(Box<SimpleAction>, u64), // duration, ttl
}

impl SimpleActionT {
    fn priority(&self) -> u32 {
        use SimpleActionT::*;
        match self {
            Ban(sub, _) => sub.atype.priority(),
            Default => 8,
            Challenge => 6,
            Redirect(_) => 4,
            Response {
                content: _,
                content_type: _,
            } => 3,
            RequestHeader(_) => 2,
            Monitor => 1,
            Skip => 0,
        }
    }

    fn is_blocking(&self) -> bool {
        !matches!(self, SimpleActionT::RequestHeader(_) | SimpleActionT::Monitor)
    }

    pub fn to_bdecision(&self) -> BDecision {
        match self {
            SimpleActionT::Skip => BDecision::Skip,
            SimpleActionT::Monitor => BDecision::Monitor,
            SimpleActionT::Ban(sub, _) => sub.atype.to_bdecision(),
            SimpleActionT::RequestHeader(_) => BDecision::AlterRequest,
            SimpleActionT::Response {
                content: _,
                content_type: _,
            }
            | SimpleActionT::Redirect(_)
            | SimpleActionT::Challenge
            | SimpleActionT::Default => BDecision::Blocking,
        }
    }
}

// an action with its semantic meaning
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleAction {
    pub atype: SimpleActionT,
    pub status: u32,
    pub reason: String,
}

impl std::default::Default for SimpleActionT {
    fn default() -> Self {
        SimpleActionT::Default
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Skip,
    Monitor,
    Block,
    AlterHeaders,
}

impl ActionType {
    /// is the action blocking (not passed to the underlying server)
    pub fn is_blocking(&self) -> bool {
        matches!(self, ActionType::Block)
    }

    /// is the action final (no further processing)
    pub fn is_final(&self) -> bool {
        !matches!(self, ActionType::Monitor)
    }
}

impl std::default::Default for Action {
    fn default() -> Self {
        Action {
            atype: ActionType::Block,
            block_mode: true,
            ban: false,
            status: 503,
            headers: None,
            content: "request denied".to_string(),
            extra_tags: None,
        }
    }
}

impl SimpleAction {
    pub fn from_reason(reason: String) -> Self {
        SimpleAction {
            atype: SimpleActionT::default(),
            status: 503,
            reason,
        }
    }

    pub fn resolve(rawaction: &RawAction) -> anyhow::Result<SimpleAction> {
        let atype = match rawaction.type_ {
            RawActionType::Skip => SimpleActionT::Skip,
            RawActionType::Default => SimpleActionT::Default,
            RawActionType::Monitor => SimpleActionT::Monitor,
            RawActionType::Ban => SimpleActionT::Ban(
                Box::new(
                    rawaction
                        .params
                        .action
                        .as_ref()
                        .and_then(|x| SimpleAction::resolve(x).ok())
                        .unwrap_or_else(|| {
                            SimpleAction::from_reason(rawaction.params.reason.clone().unwrap_or_else(|| "?".into()))
                        }),
                ),
                rawaction
                    .params
                    .duration
                    .as_ref()
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(3600),
            ),
            RawActionType::RequestHeader => SimpleActionT::RequestHeader(
                rawaction
                    .params
                    .headers
                    .as_ref()
                    .map(|hm| {
                        hm.iter()
                            .map(|(k, v)| (k.to_string(), parse_request_template(v)))
                            .collect()
                    })
                    .unwrap_or_default(),
            ),
            RawActionType::Response => SimpleActionT::Response {
                content: rawaction
                    .params
                    .content
                    .clone()
                    .unwrap_or_else(|| "default content".into()),
                content_type: rawaction.params.content_type.clone(),
            },
            RawActionType::Challenge => SimpleActionT::Challenge,
            RawActionType::Redirect => SimpleActionT::Redirect(
                rawaction
                    .params
                    .location
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("no location for redirect in rule {:?}", rawaction))?,
            ),
        };
        let status = if let Some(sstatus) = &rawaction.params.status {
            match sstatus.parse::<u32>() {
                Ok(s) => s,
                Err(rr) => return Err(anyhow::anyhow!("Unparseable status: {} -> {}", sstatus, rr)),
            }
        } else {
            503
        };
        Ok(SimpleAction {
            atype,
            status,
            reason: rawaction.params.reason.clone().unwrap_or_else(|| "no reason".into()),
        })
    }

    /// returns None when it is a challenge, Some(action) otherwise
    fn to_action(&self, rinfo: &RequestInfo, tags: &Tags, is_human: bool) -> Option<Action> {
        let mut action = Action::default();
        action.block_mode = action.atype.is_blocking();
        action.status = self.status;
        match &self.atype {
            SimpleActionT::Default => {}
            SimpleActionT::Skip => action.atype = ActionType::Skip,
            SimpleActionT::Monitor => action.atype = ActionType::Monitor,
            SimpleActionT::Ban(sub, _) => {
                action = sub.to_action(rinfo, tags, is_human).unwrap_or_default();
                action.ban = true;
            }
            SimpleActionT::RequestHeader(hdrs) => {
                action.headers = Some(
                    hdrs.iter()
                        .map(|(k, template)| (k.to_string(), render_template(rinfo, tags, template)))
                        .collect(),
                );
                action.atype = ActionType::AlterHeaders;
            }
            SimpleActionT::Response { content, content_type } => {
                action.atype = ActionType::Block;
                action.content = content.clone();
                if let Some(ct) = content_type {
                    action.headers = Some(std::iter::once(("content-type".to_string(), ct.clone())).collect())
                }
            }
            SimpleActionT::Challenge => {
                if !is_human {
                    return None;
                }
                action.atype = ActionType::Monitor;
            }
            SimpleActionT::Redirect(to) => {
                let mut headers = HashMap::new();
                action.content = "You are being redirected".into();
                headers.insert("Location".into(), to.clone());
                action.atype = ActionType::Block;
                action.headers = Some(headers);
            }
        }
        Some(action)
    }

    pub fn to_decision<GH: Grasshopper>(
        &self,
        is_human: bool,
        mgh: &Option<GH>,
        rinfo: &RequestInfo,
        tags: &Tags,
        reason: Vec<BlockReason>,
    ) -> Decision {
        if self.atype == SimpleActionT::Skip {
            return Decision {
                maction: None,
                reasons: reason,
            };
        }
        let action = match self.to_action(rinfo, tags, is_human) {
            None => match (mgh, rinfo.headers.get("user-agent")) {
                (Some(gh), Some(ua)) => return challenge_phase01(gh, ua, reason),
                _ => Action::default(),
            },
            Some(a) => a,
        };
        Decision::action(action, reason)
    }

    pub fn is_blocking(&self) -> bool {
        self.atype.is_blocking()
    }
}

fn render_template(rinfo: &RequestInfo, tags: &Tags, template: &[TemplatePart<TVar>]) -> String {
    let mut out = String::new();
    for p in template {
        match p {
            TemplatePart::Raw(s) => out.push_str(s),
            TemplatePart::Var(TVar::Selector(RequestSelector::Tags)) => out.push_str(&tags.to_json().to_string()),
            TemplatePart::Var(TVar::Tag(tagname)) => {
                out.push_str(if tags.contains(tagname) { "true" } else { "false" })
            }
            TemplatePart::Var(TVar::Selector(sel)) => match selector(rinfo, sel, tags) {
                None => out.push_str("nil"),
                Some(Selected::OStr(s)) => out.push_str(&s),
                Some(Selected::Str(s)) => out.push_str(s),
                Some(Selected::U32(v)) => out.push_str(&v.to_string()),
            },
        }
    }
    out
}
