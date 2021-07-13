use crate::config::raw::{RawAction, RawActionType};
use crate::logs::Logs;
use crate::requestfields::RequestField;
use crate::utils::RequestInfo;
use serde::{Deserialize, Serialize};
use serde_json::json;
/// this file contains all the data type that are used when interfacing with a proxy
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum SimpleDecision {
    Pass,
    Action(SimpleAction, Option<serde_json::Value>),
}

#[derive(Debug, Clone)]
pub enum Decision {
    Pass,
    Action(Action),
}

impl Decision {
    pub fn to_json_raw(&self, request_map: serde_json::Value, logs: Logs) -> String {
        let (action_desc, response) = match self {
            Decision::Pass => ("pass", None),
            Decision::Action(a) => ("custom_response", Some(a)),
        };
        let j = serde_json::json!({
            "request_map": request_map,
            "action": action_desc,
            "response": response,
            "logs": logs.logs
        });
        serde_json::to_string(&j).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn to_json(&self, rinfo: RequestInfo, tags: Tags, logs: Logs) -> String {
        let mut tgs = tags;
        let (action_desc, response) = match self {
            Decision::Pass => ("pass", None),
            Decision::Action(a) => ("custom_response", Some(a)),
        };
        if let Decision::Action(a) = &self {
            if let Some(extra) = &a.extra_tags {
                for t in extra {
                    tgs.insert(t);
                }
            }
        }
        let request_map = rinfo.into_json(tgs);
        let j = serde_json::json!({
            "request_map": request_map,
            "action": action_desc,
            "response": response,
            "logs": logs.logs
        });
        serde_json::to_string(&j).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn is_blocking(&self) -> bool {
        match self {
            Decision::Pass => false,
            Decision::Action(a) => a.atype.is_blocking(),
        }
    }
}

/// a newtype representing tags, to make sure they are tagified when inserted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags(HashSet<String>);

fn tagify(tag: &str) -> String {
    fn filter_char(c: char) -> char {
        if c.is_ascii_alphanumeric() || c == ':' {
            c
        } else {
            '-'
        }
    }
    tag.to_lowercase().chars().map(filter_char).collect()
}

impl Default for Tags {
    fn default() -> Self {
        Tags(HashSet::new())
    }
}

impl Tags {
    pub fn insert(&mut self, value: &str) -> bool {
        self.0.insert(tagify(value))
    }

    pub fn insert_qualified(&mut self, id: &str, value: &str) -> bool {
        let mut to_insert = id.to_string();
        to_insert.push(':');
        to_insert += &tagify(value);
        self.0.insert(to_insert)
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0)
    }

    pub fn from_slice(slice: &[String]) -> Self {
        Tags(slice.iter().map(|s| tagify(&s)).collect())
    }

    pub fn contains(&self, s: &str) -> bool {
        self.0.contains(s)
    }

    pub fn as_hash_ref(&self) -> &HashSet<String> {
        &self.0
    }
}

// an action, as formatted for outside consumption
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    pub atype: ActionType,
    pub ban: bool,
    pub block_mode: bool,
    pub status: u32,
    pub headers: Option<HashMap<String, String>>,
    pub reason: serde_json::value::Value,
    pub content: String,
    pub extra_tags: Option<HashSet<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleActionT {
    Default,
    Monitor,
    Ban(Box<SimpleAction>, u64), // ttl
    RequestHeader(HashMap<String, String>),
    Response(String),
    Redirect(String),
    Challenge,
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
    Monitor,
    Block,
    AlterHeaders,
}

impl ActionType {
    pub fn is_blocking(&self) -> bool {
        matches!(self, ActionType::Block)
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
            reason: serde_json::value::Value::Null,
            content: "curiefense - request denied".to_string(),
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
            RawActionType::Default => SimpleActionT::Default,
            RawActionType::Monitor => SimpleActionT::Monitor,
            RawActionType::Ban => SimpleActionT::Ban(
                Box::new(
                    rawaction
                        .params
                        .action
                        .as_ref()
                        .map(|x| SimpleAction::resolve(x).ok())
                        .flatten()
                        .unwrap_or_else(|| {
                            SimpleAction::from_reason(rawaction.params.reason.clone().unwrap_or_else(|| "?".into()))
                        }),
                ),
                rawaction
                    .params
                    .ttl
                    .as_ref()
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(3600),
            ),
            RawActionType::RequestHeader => SimpleActionT::RequestHeader(HashMap::new()),
            RawActionType::Response => SimpleActionT::Response(
                rawaction
                    .params
                    .content
                    .clone()
                    .unwrap_or_else(|| "default content".into()),
            ),
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
    fn to_action(&self, is_human: bool) -> Option<Action> {
        let mut action = Action::default();
        match &self.atype {
            SimpleActionT::Default => {}
            SimpleActionT::Monitor => action.atype = ActionType::Monitor,
            SimpleActionT::Ban(sub, _) => {
                action = sub.to_action(is_human).unwrap_or_default();
                action.ban = true;
            }
            SimpleActionT::RequestHeader(hdrs) => {
                action.headers = Some(hdrs.clone());
                action.atype = ActionType::AlterHeaders;
            }
            SimpleActionT::Response(content) => {
                action.atype = ActionType::Block;
                action.content = content.clone();
            }
            SimpleActionT::Challenge => {
                if !is_human {
                    return None;
                }
                action.atype = ActionType::Monitor;
            }
            SimpleActionT::Redirect(to) => {
                let mut headers = HashMap::new();
                headers.insert("Location".into(), to.clone());
                action.atype = ActionType::Block;
                action.headers = Some(headers);
            }
        }
        action.block_mode = action.atype.is_blocking();
        action.status = self.status;
        Some(action)
    }

    pub fn to_decision<GH: Grasshopper>(
        &self,
        is_human: bool,
        mgh: &Option<GH>,
        headers: &RequestField,
        reason: Option<serde_json::Value>,
    ) -> Decision {
        let mut action = match self.to_action(is_human) {
            None => match (mgh, headers.get("user-agent")) {
                (Some(gh), Some(ua)) => return challenge_phase01(gh, ua, Vec::new()),
                _ => Action::default(),
            },
            Some(a) => a,
        };
        if let Some(r) = reason {
            action.reason = r;
        }
        Decision::Action(action)
    }

    pub fn to_decision_no_challenge(&self, reason: Option<serde_json::Value>) -> Decision {
        let mut action = match self.to_action(true) {
            None => Action::default(),
            Some(a) => a,
        };
        if let Some(r) = reason {
            action.reason = r;
        }
        Decision::Action(action)
    }
}

impl SimpleDecision {
    pub fn into_decision_no_challenge(self) -> Decision {
        match self {
            SimpleDecision::Pass => Decision::Pass,
            SimpleDecision::Action(action, reason) => action.to_decision_no_challenge(reason),
        }
    }
}

pub trait Grasshopper {
    fn js_app(&self) -> Option<String>;
    fn js_bio(&self) -> Option<String>;
    fn parse_rbzid(&self, rbzid: &str, seed: &str) -> Option<bool>;
    fn gen_new_seed(&self, seed: &str) -> Option<String>;
    fn verify_workproof(&self, workproof: &str, seed: &str) -> Option<String>;
}

pub fn gh_fail_decision(reason: &str) -> Decision {
    Decision::Action(Action {
        atype: ActionType::Block,
        block_mode: true,
        ban: false,
        reason: json!({"initiator": "phase01", "reason": reason}),
        headers: None,
        status: 500,
        content: "internal_error".to_string(),
        extra_tags: None,
    })
}

pub fn challenge_phase01<GH: Grasshopper>(gh: &GH, ua: &str, tags: Vec<String>) -> Decision {
    let seed = match gh.gen_new_seed(ua) {
        None => return gh_fail_decision("could not call gen_new_seed"),
        Some(s) => s,
    };
    let chall_lib = match gh.js_app() {
        None => return gh_fail_decision("could not call chall_lib"),
        Some(s) => s,
    };
    let hdrs: HashMap<String, String> = [
        ("Content-Type", "text/html; charset=utf-8"),
        ("Expires", "Thu, 01 Aug 1978 00:01:48 GMT"),
        ("Cache-Control", "no-cache, private, no-transform, no-store"),
        ("Pragma", "no-cache"),
        (
            "P3P",
            "CP=\"IDC DSP COR ADM DEVi TAIi PSA PSD IVAi IVDi CONi HIS OUR IND CNT\"",
        ),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();

    let mut content = "<html><head><meta charset=\"utf-8\"><script>".to_string();
    content += &chall_lib;
    content += ";;window.rbzns={bereshit: \"1\", seed: \"";
    content += &seed;
    content += "\", storage:\"3\"};winsocks();";
    content += "</script></head><body></body></html>";

    // here humans are accepted, as they were not denied
    // (this would have been caught by the previous guard)
    Decision::Action(Action {
        atype: ActionType::Block,
        block_mode: true,
        ban: false,
        reason: json!({"initiator": "phase01", "reason": "challenge", "tags": tags}),
        headers: Some(hdrs),
        status: 247,
        content,
        extra_tags: Some(["challenge_phase01"].iter().map(|s| s.to_string()).collect()),
    })
}

fn extract_zebra(headers: &RequestField) -> Option<String> {
    for (k, v) in headers.iter() {
        if k.starts_with("x-zebra-") {
            return Some(v.replace('-', "="));
        }
    }
    None
}

pub fn challenge_phase02<GH: Grasshopper>(gh: &GH, uri: &str, headers: &RequestField) -> Option<Decision> {
    if !uri.starts_with("/7060ac19f50208cbb6b45328ef94140a612ee92387e015594234077b4d1e64f1/") {
        return None;
    }
    let ua = headers.get("user-agent")?;
    let workproof = extract_zebra(headers)?;
    let verified = gh.verify_workproof(&workproof, ua)?;
    let mut nheaders = HashMap::<String, String>::new();
    let mut cookie = "rbzid=".to_string();
    cookie += &verified.replace('=', "-");
    cookie += "; Path=/; HttpOnly";

    nheaders.insert("Set-Cookie".to_string(), cookie);

    Some(Decision::Action(Action {
        atype: ActionType::Block,
        block_mode: true,
        ban: false,
        reason: json!({"initiator": "phase02", "reason": "challenge"}),
        headers: Some(nheaders),
        status: 248,
        content: "{}".to_string(),
        extra_tags: Some(["challenge_phase02"].iter().map(|s| s.to_string()).collect()),
    }))
}
