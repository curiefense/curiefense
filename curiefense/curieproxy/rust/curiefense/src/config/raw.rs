use serde::de::{self, Deserializer, SeqAccess, Visitor};
/// this module contains types that map to the the JSON configuration format of curiefense configuration files
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// a mapping of the configuration file for security policy entries
/// it is called "securitypolicy" in the lua code
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawHostMap {
    #[serde(rename = "match")]
    pub match_: String,
    pub id: String,
    pub name: String,
    pub map: Vec<RawSecurityPolicy>,
}

/// a mapping of the configuration file for security policies
/// it is called "securitypolicy-entry" in the lua code
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawSecurityPolicy {
    #[serde(rename = "match")]
    pub match_: String,
    pub name: String,
    pub acl_profile: String,
    pub content_filter_profile: String,
    pub acl_active: bool,
    pub content_filter_active: bool,
    pub limit_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Relation {
    And,
    Or,
}

/// this is partial, as a ton of data is not needed
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawGlobalFilterSection {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub tags: Vec<String>,
    pub rule: RawGlobalFilterRule,
    pub action: Option<RawAction>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawGlobalFilterRule {
    pub relation: Relation,
    pub sections: Vec<RawGlobalFilterSSection>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum GlobalFilterEntryType {
    Args,
    Cookies,
    Headers,
    Path,
    Query,
    Uri,
    Asn,
    Country,
    Method,
    Ip,
    Company,
    Authority,
}

/// a special datatype for deserializing tuples with 2 elements, and optional extra elements
#[derive(Debug, Serialize, Clone)]
pub struct RawGlobalFilterSSectionEntry {
    pub tp: GlobalFilterEntryType,
    pub vl: serde_json::Value,
    pub comment: Option<String>,
}

impl<'de> Deserialize<'de> for RawGlobalFilterSSectionEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyTupleVisitor;

        impl<'de> Visitor<'de> for MyTupleVisitor {
            type Value = RawGlobalFilterSSectionEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a global filter section entry")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let tp = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let vl = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                // comment might not be present
                let comment = seq.next_element()?;

                Ok(RawGlobalFilterSSectionEntry { tp, vl, comment })
            }
        }

        deserializer.deserialize_seq(MyTupleVisitor)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawGlobalFilterSSection {
    pub relation: Relation,
    pub entries: Vec<RawGlobalFilterSSectionEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawLimit {
    pub id: String,
    pub name: String,
    pub limit: String,
    pub timeframe: String,
    #[serde(default)]
    pub key: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    pub pairwith: HashMap<String, String>,
    pub action: RawAction,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawLimitSelector {
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub cookies: HashMap<String, String>,
    #[serde(default)]
    pub args: HashMap<String, String>,
    #[serde(default)]
    pub attrs: HashMap<String, String>,
}

impl std::default::Default for RawLimitSelector {
    fn default() -> Self {
        RawLimitSelector {
            headers: HashMap::new(),
            cookies: HashMap::new(),
            args: HashMap::new(),
            attrs: HashMap::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawAction {
    #[serde(rename = "type", default)]
    pub type_: RawActionType,
    #[serde(default)]
    pub params: RawActionParams,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum RawActionType {
    Default,
    Ban,
    Response,
    Challenge,
    Redirect,
    Monitor,
    RequestHeader,
}

impl std::default::Default for RawActionType {
    fn default() -> Self {
        RawActionType::Default
    }
}

fn get_false() -> bool {
    false
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawActionParams {
    pub status: Option<String>,
    #[serde(default = "get_false")]
    pub block_mode: bool,
    pub action: Option<Box<RawAction>>,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
    pub reason: Option<String>,
    pub content: Option<String>,
    pub location: Option<String>,
    pub duration: Option<String>,
}

impl std::default::Default for RawActionParams {
    fn default() -> Self {
        RawActionParams {
            status: None,
            block_mode: true,
            action: None,
            headers: None,
            reason: None,
            content: None,
            location: None,
            duration: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AclProfile {
    pub id: String,
    pub name: String,
    pub allow: HashSet<String>,
    pub allow_bot: HashSet<String>,
    pub deny: HashSet<String>,
    pub deny_bot: HashSet<String>,
    pub passthrough: HashSet<String>,
    pub force_deny: HashSet<String>,
}

impl AclProfile {
    pub fn default() -> Self {
        AclProfile {
            id: "__default__".to_string(),
            name: "default-acl".to_string(),
            allow: HashSet::new(),
            allow_bot: HashSet::new(),
            deny: HashSet::new(),
            deny_bot: HashSet::new(),
            passthrough: HashSet::new(),
            force_deny: HashSet::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawContentFilterProfile {
    pub id: String,
    pub name: String,
    pub ignore_alphanum: bool,
    pub max_header_length: usize,
    pub max_cookie_length: usize,
    pub max_arg_length: usize,
    pub max_headers_count: usize,
    pub max_cookies_count: usize,
    pub max_args_count: usize,
    pub args: RawContentFilterProperties,
    pub headers: RawContentFilterProperties,
    pub cookies: RawContentFilterProperties,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawContentFilterProperties {
    pub names: Vec<RawContentFilterEntryMatch>,
    pub regex: Vec<RawContentFilterEntryMatch>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawContentFilterEntryMatch {
    pub key: String,
    pub reg: Option<String>,
    pub restrict: bool,
    pub exclusions: Option<HashMap<String, u64>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentFilterRule {
    pub id: String,
    pub name: String,
    pub msg: String,
    pub operand: String,
    pub severity: u8,
    pub certainity: u8,
    pub category: String,
    pub subcategory: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawFlowEntry {
    pub id: String,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub name: String,
    #[serde(default)]
    pub key: Vec<HashMap<String, String>>,
    pub active: bool,
    pub timeframe: u64,
    pub action: RawAction,
    pub sequence: Vec<RawFlowStep>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawFlowStep {
    pub method: String,
    pub uri: String,
    #[serde(default)]
    pub cookies: HashMap<String, String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub args: HashMap<String, String>,
}
