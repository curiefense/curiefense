/// this file contains all the data type that are used when interfacing with a proxy
use crate::config::raw::{RawAction, RawActionType};
use crate::grasshopper::{challenge_phase01, Grasshopper};
use crate::logs::Logs;
use crate::requestfields::RequestField;
use crate::utils::RequestInfo;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum SimpleDecision {
    Pass,
    Action(SimpleAction, serde_json::Value),
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

    /// is the action blocking (not passed to the underlying server)
    pub fn is_blocking(&self) -> bool {
        match self {
            Decision::Pass => false,
            Decision::Action(a) => a.atype.is_blocking(),
        }
    }

    /// is the action final (no further processing)
    pub fn is_final(&self) -> bool {
        match self {
            Decision::Pass => false,
            Decision::Action(a) => a.atype.is_final(),
        }
    }
}

/// a newtype representing tags, to make sure they are tagified when inserted
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
        Tags(slice.iter().map(|s| tagify(s)).collect())
    }

    pub fn contains(&self, s: &str) -> bool {
        self.0.contains(s)
    }

    pub fn as_hash_ref(&self) -> &HashSet<String> {
        &self.0
    }

    pub fn selector(&self) -> String {
        let mut tvec: Vec<&str> = self.0.iter().map(|s| s.as_ref()).collect();
        tvec.sort_unstable();
        tvec.join("*")
    }

    pub fn intersect(&self, other: &HashSet<String>) -> HashSet<String> {
        self.0.intersection(other).cloned().collect()
    }

    pub fn has_intersection(&self, other: &HashSet<String>) -> bool {
        self.0.intersection(other).next().is_some()
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
    Monitor,
    RequestHeader(HashMap<String, String>),
    Response(String),
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
            Response(_) => 3,
            RequestHeader(_) => 2,
            Monitor => 1,
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
            reason: serde_json::value::Value::Null,
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
            RawActionType::RequestHeader => {
                SimpleActionT::RequestHeader(rawaction.params.headers.clone().unwrap_or_default())
            }
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
        action.block_mode = action.atype.is_blocking();
        action.status = self.status;
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
        headers: &RequestField,
        reason: serde_json::Value,
    ) -> Decision {
        let mut action = match self.to_action(is_human) {
            None => match (mgh, headers.get("user-agent")) {
                (Some(gh), Some(ua)) => return challenge_phase01(gh, ua, Vec::new()),
                _ => Action::default(),
            },
            Some(a) => a,
        };
        action.reason = reason;
        Decision::Action(action)
    }

    pub fn to_decision_no_challenge(&self, reason: serde_json::Value) -> Decision {
        let mut action = match self.to_action(true) {
            None => Action::default(),
            Some(a) => a,
        };
        action.reason = reason;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tag_selector() {
        let tags = Tags::from_slice(&["ccc".to_string(), "bbb".to_string(), "aaa".to_string()]);
        assert_eq!(tags.selector(), "aaa*bbb*ccc");
    }

    #[test]
    fn tag_selector_r() {
        let tags = Tags::from_slice(&["aaa".to_string(), "ccc".to_string(), "bbb".to_string()]);
        assert_eq!(tags.selector(), "aaa*bbb*ccc");
    }
}
