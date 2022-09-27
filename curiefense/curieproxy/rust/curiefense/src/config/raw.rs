use serde::de::{self, Deserializer, SeqAccess, Visitor};
/// this module contains types that map to the the JSON configuration format of curiefense configuration files
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::interface::SimpleAction;
use crate::logs::Logs;

/// a datatype used to represent u64 that are sometimes represented as strings
#[derive(Debug, Clone, Copy)]
pub struct Repru64 {
    pub inner: u64,
}

impl<'de> Deserialize<'de> for Repru64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ParseVisitor;

        impl<'de> Visitor<'de> for ParseVisitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "expecting an unsigned integer")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                s.parse()
                    .map_err(|_| de::Error::invalid_value(serde::de::Unexpected::Str(s), &self))
            }

            fn visit_u64<E>(self, i: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(i)
            }
        }

        deserializer
            .deserialize_any(ParseVisitor)
            .map(|inner| Repru64 { inner })
    }
}

impl Serialize for Repru64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

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
    #[serde(default)]
    pub session: Vec<HashMap<String, String>>,
    pub id: Option<String>, // set to name if absent
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
    pub action: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum RawGlobalFilterRule {
    Rel(RawGlobalFilterRelation),
    Entry(RawGlobalFilterEntry),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawGlobalFilterRelation {
    pub relation: Relation,
    pub entries: Vec<RawGlobalFilterRule>,
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
    Region,
    SubRegion,
    Method,
    Ip,
    Company,
    Authority,
    Tag,
    SecurityPolicyId,
    SecurityPolicyEntryId,
}

/// a special datatype for deserializing tuples with 2 elements, and optional extra elements
#[derive(Debug, Serialize, Clone)]
pub struct RawGlobalFilterEntry {
    pub tp: GlobalFilterEntryType,
    pub vl: serde_json::Value,
    pub comment: Option<String>,
}

impl<'de> Deserialize<'de> for RawGlobalFilterEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyTupleVisitor;

        impl<'de> Visitor<'de> for MyTupleVisitor {
            type Value = RawGlobalFilterEntry;

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
                let comment = seq.next_element().ok().flatten();

                Ok(RawGlobalFilterEntry { tp, vl, comment })
            }
        }

        deserializer.deserialize_seq(MyTupleVisitor)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawLimit {
    pub id: String,
    pub name: String,
    pub timeframe: Repru64,
    #[serde(default)]
    pub key: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub thresholds: Vec<RawLimitThreshold>,
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    pub pairwith: HashMap<String, String>,
    #[serde(default)]
    pub global: bool, // global flag, if true this rule applies to all profiles
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawLimitThreshold {
    pub limit: Repru64,
    pub action: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawAction {
    pub id: String,
    #[serde(rename = "type", default)]
    pub type_: RawActionType,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub params: RawActionParams,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum RawActionType {
    Skip,
    Monitor,
    Custom,
    Challenge,
}

impl std::default::Default for RawActionType {
    fn default() -> Self {
        RawActionType::Custom
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct RawActionParams {
    pub status: Option<u32>,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawAclProfile {
    pub id: String,
    pub name: String,
    pub allow: HashSet<String>,
    pub allow_bot: HashSet<String>,
    pub deny: HashSet<String>,
    pub deny_bot: HashSet<String>,
    pub passthrough: HashSet<String>,
    pub force_deny: HashSet<String>,
    pub action: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AclProfile {
    pub id: String,
    pub name: String,
    pub allow: HashSet<String>,
    pub allow_bot: HashSet<String>,
    pub deny: HashSet<String>,
    pub deny_bot: HashSet<String>,
    pub passthrough: HashSet<String>,
    pub force_deny: HashSet<String>,
    pub action: SimpleAction,
    pub tags: HashSet<String>,
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
            action: SimpleAction::default(),
            tags: HashSet::new(),
        }
    }

    pub fn resolve(logs: &mut Logs, actions: &HashMap<String, SimpleAction>, acl: RawAclProfile) -> Self {
        let id = acl.id;
        let action = match acl.action {
            None => {
                logs.warning(|| format!("Could not find the action in acl profile {}, using default action", id));
                SimpleAction::default()
            }
            Some(aid) => actions.get(&aid).cloned().unwrap_or_else(|| {
                logs.error(|| format!("Could not resolve action {} in acl profile {}", aid, id));
                SimpleAction::default()
            }),
        };
        AclProfile {
            id,
            name: acl.name,
            allow: acl.allow,
            allow_bot: acl.allow_bot,
            deny: acl.deny,
            deny_bot: acl.deny_bot,
            passthrough: acl.passthrough,
            force_deny: acl.force_deny,
            action,
            tags: acl.tags.into_iter().collect(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    MultipartForm, // multipart/form-data
    UrlEncoded,    // application/x-www-form-urlencoded
    Json,
    Xml,
    Graphql, // application/graphql
}

impl ContentType {
    pub const VALUES: [ContentType; 5] = [
        ContentType::Json,
        ContentType::MultipartForm,
        ContentType::UrlEncoded,
        ContentType::Xml,
        ContentType::Graphql,
    ];
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawContentFilterProfile {
    pub id: String,
    pub name: String,
    pub ignore_alphanum: bool,
    pub args: RawContentFilterProperties,
    pub headers: RawContentFilterProperties,
    pub cookies: RawContentFilterProperties,
    #[serde(default)]
    pub path: RawContentFilterProperties,
    #[serde(default)]
    pub allsections: RawContentFilterProperties,
    #[serde(default)]
    pub decoding: ContentFilterDecoding,
    #[serde(default)]
    pub active: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
    #[serde(default)]
    pub report: Vec<String>,
    pub masking_seed: String,
    #[serde(default)]
    pub content_type: Vec<ContentType>,
    #[serde(default)]
    pub ignore_body: bool,
    pub max_body_size: Option<usize>,
    pub max_body_depth: Option<usize>,
    #[serde(default)]
    pub referer_as_uri: bool,
    pub action: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MinRisk(pub u8);
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MaxCount(pub usize);
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MaxLength(pub usize);

impl Default for MaxCount {
    fn default() -> Self {
        MaxCount(42)
    }
}
impl Default for MaxLength {
    fn default() -> Self {
        MaxLength(2048)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct RawContentFilterProperties {
    pub names: Vec<RawContentFilterEntryMatch>,
    pub regex: Vec<RawContentFilterEntryMatch>,
    #[serde(default)]
    pub max_count: MaxCount,
    #[serde(default)]
    pub max_length: MaxLength,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct ContentFilterDecoding {
    #[serde(default)]
    pub base64: bool,
    #[serde(default)]
    pub dual: bool,
    #[serde(default)]
    pub html: bool,
    #[serde(default)]
    pub unicode: bool,
}

impl Default for ContentFilterDecoding {
    fn default() -> Self {
        ContentFilterDecoding {
            base64: true,
            dual: true,
            html: false,
            unicode: false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawContentFilterEntryMatch {
    pub key: String,
    pub reg: Option<String>,
    pub restrict: bool,
    pub mask: Option<bool>,
    #[serde(default)]
    pub exclusions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentFilterRule {
    pub id: String,
    pub operand: String,
    pub risk: u8,
    pub category: String,
    pub subcategory: String,
    #[serde(default)]
    pub tags: HashSet<String>,
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
    pub tags: Vec<String>,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawManifest {
    pub meta: RawMetaManifest,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawMetaManifest {
    pub id: String,
    pub version: String,
}
