/// this file contains all the data type that are used when interfacing with a proxy
use crate::config::contentfilter::SectionIdx;
use serde::ser::SerializeMap;
use serde::Serialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

use super::tagging::{Location, Tags};

#[derive(Debug, Clone, Copy, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AclStage {
    EnforceDeny,
    Bypass,
    AllowBot,
    DenyBot,
    Allow,
    Deny,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Initiator {
    GlobalFilter { id: String, name: String },
    Acl { tags: Vec<String>, stage: AclStage },
    ContentFilter { ruleid: String, risk_level: u8 },
    Limit { id: String, name: String, threshold: u64 },
    BodyTooDeep { actual: usize, expected: usize },
    BodyMissing,
    BodyMalformed(String),
    Phase01Fail(String),
    Phase02,
    Sqli(String),
    Xss,
    Restricted,
    TooManyEntries { actual: usize, expected: usize },
    EntryTooLarge { actual: usize, expected: usize },
}

impl std::fmt::Display for Initiator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Initiator::*;
        match self {
            GlobalFilter { id, name } => write!(f, "global filter {}[{}]", name, id),
            Acl { tags, stage } => write!(f, "acl {:?} {:?}", stage, tags),
            ContentFilter { ruleid, risk_level } => write!(f, "content filter {}[lvl{}]", ruleid, risk_level),
            Limit { id, name, threshold } => write!(f, "rate limit {}[{}] threshold={}", name, id, threshold),
            BodyTooDeep { actual: _, expected } => write!(f, "body too deep threshhold={}", expected),
            BodyMissing => write!(f, "body is missing"),
            BodyMalformed(r) => write!(f, "body is malformed: {}", r),
            Phase01Fail(r) => write!(f, "grasshopper phase 1 error: {}", r),
            Phase02 => write!(f, "grasshopper phase 2"),
            Sqli(fp) => write!(f, "sql injection {}", fp),
            Xss => write!(f, "xss"),
            Restricted => write!(f, "restricted parameter"),
            TooManyEntries { actual, expected } => {
                write!(f, "too many entries, entries={} threshold={}", actual, expected)
            }
            EntryTooLarge { actual, expected } => write!(f, "too large, size={} threshold={}", actual, expected),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InitiatorKind {
    Acl,
    RateLimit,
    GlobalFilter,
    ContentFilter,
}

impl Initiator {
    pub fn to_kind(&self) -> InitiatorKind {
        use InitiatorKind::*;
        match self {
            Initiator::GlobalFilter { id: _, name: _ } => GlobalFilter,
            Initiator::Acl { tags: _, stage: _ } => Acl,
            Initiator::ContentFilter {
                ruleid: _,
                risk_level: _,
            } => ContentFilter,
            Initiator::Limit {
                id: _,
                name: _,
                threshold: _,
            } => RateLimit,
            Initiator::BodyTooDeep { actual: _, expected: _ } => ContentFilter,
            Initiator::BodyMissing => ContentFilter,
            Initiator::BodyMalformed(_) => ContentFilter,
            Initiator::Phase01Fail(_) => Acl,
            Initiator::Phase02 => Acl,
            Initiator::Sqli(_) => ContentFilter,
            Initiator::Xss => ContentFilter,
            Initiator::Restricted => ContentFilter,
            Initiator::TooManyEntries { actual: _, expected: _ } => ContentFilter,
            Initiator::EntryTooLarge { actual: _, expected: _ } => ContentFilter,
        }
    }

    pub fn serialize_in_map<S: serde::Serializer>(
        &self,
        map: &mut <S as serde::Serializer>::SerializeMap,
    ) -> Result<(), S::Error> {
        match self {
            Initiator::GlobalFilter { id, name } => {
                map.serialize_entry("id", id)?;
                map.serialize_entry("name", name)?;
            }
            Initiator::Acl { tags, stage } => {
                map.serialize_entry("tags", tags)?;
                map.serialize_entry("type", stage)?;
            }
            Initiator::ContentFilter { ruleid, risk_level } => {
                map.serialize_entry("type", "signature")?;
                map.serialize_entry("ruleid", ruleid)?;
                map.serialize_entry("risk_level", risk_level)?;
            }
            Initiator::Limit { id, name, threshold } => {
                map.serialize_entry("id", id)?;
                map.serialize_entry("name", name)?;
                map.serialize_entry("threshold", threshold)?;
                map.serialize_entry("counter", &(threshold + 1))?;
            }
            Initiator::BodyTooDeep { actual, expected } => {
                map.serialize_entry("type", "body_too_deep")?;
                map.serialize_entry("actual", actual)?;
                map.serialize_entry("expected", expected)?;
            }
            Initiator::BodyMissing => {
                map.serialize_entry("type", "body_missing")?;
            }
            Initiator::BodyMalformed(_) => {
                map.serialize_entry("type", "body_malformed")?;
            }
            Initiator::Phase01Fail(r) => {
                map.serialize_entry("type", "phase1")?;
                map.serialize_entry("details", r)?;
            }
            Initiator::Phase02 => {
                map.serialize_entry("type", "phase2")?;
            }
            Initiator::Sqli(fp) => {
                map.serialize_entry("type", "sqli")?;
                map.serialize_entry("fingerprint", fp)?;
            }
            Initiator::Xss => {
                map.serialize_entry("type", "xss")?;
            }
            Initiator::Restricted => {
                map.serialize_entry("type", "restricted_content")?;
            }
            Initiator::TooManyEntries { actual, expected } => {
                map.serialize_entry("type", "too_many_entries")?;
                map.serialize_entry("actual", actual)?;
                map.serialize_entry("expected", expected)?;
            }
            Initiator::EntryTooLarge { actual, expected } => {
                map.serialize_entry("type", "entry_too_large")?;
                map.serialize_entry("actual", actual)?;
                map.serialize_entry("expected", expected)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BDecision {
    Skip,
    Monitor,
    AlterRequest,
    InitiatorInactive,
    Blocking,
}

impl std::fmt::Display for BDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BDecision::Skip => write!(f, "skip"),
            BDecision::Monitor => write!(f, "monitor"),
            BDecision::AlterRequest => write!(f, "alter_request"),
            BDecision::InitiatorInactive => write!(f, "inactive"),
            BDecision::Blocking => write!(f, "blocking"),
        }
    }
}

impl BDecision {
    pub fn inactive(&mut self) {
        if self == &BDecision::Blocking || self == &BDecision::AlterRequest {
            *self = BDecision::InitiatorInactive;
        }
    }
}

impl Serialize for BDecision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            BDecision::Skip => "skip",
            BDecision::Monitor => "monitor",
            BDecision::AlterRequest => "alter_request",
            BDecision::InitiatorInactive => "inactive",
            BDecision::Blocking => "block",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockReason {
    pub initiator: Initiator,
    pub location: HashSet<Location>,
    pub decision: BDecision,
}

impl Serialize for BlockReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map: <S as serde::Serializer>::SerializeMap = serializer.serialize_map(None)?;
        self.serialize_in_map::<S>(&mut map)?;
        map.end()
    }
}

impl std::fmt::Display for BlockReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} - [", self.decision, self.initiator)?;
        let mut comma = false;
        for l in &self.location {
            if comma {
                write!(f, ", ")?;
            }
            comma = true;
            write!(f, "{}", l)?;
        }
        write!(f, "]")
    }
}

impl BlockReason {
    pub fn block_reason_desc(reasons: &[Self]) -> Option<String> {
        reasons
            .iter()
            .find(|r| r.decision == BDecision::Blocking)
            .map(|r| r.to_string())
    }

    pub fn global_filter(id: String, name: String, decision: BDecision) -> Self {
        BlockReason::nodetails(Initiator::GlobalFilter { id, name }, decision)
    }

    pub fn limit(id: String, name: String, threshold: u64, decision: BDecision) -> Self {
        BlockReason::nodetails(Initiator::Limit { id, name, threshold }, decision)
    }

    pub fn phase01_unknown(reason: &str) -> Self {
        BlockReason::nodetails(Initiator::Phase01Fail(reason.to_string()), BDecision::Blocking)
    }

    pub fn phase02() -> Self {
        BlockReason::nodetails(Initiator::Phase02, BDecision::Blocking)
    }

    fn nodetails(initiator: Initiator, decision: BDecision) -> Self {
        BlockReason {
            initiator,
            location: Location::request(),
            decision,
        }
    }

    pub fn body_too_deep(actual: usize, expected: usize) -> Self {
        BlockReason {
            initiator: Initiator::BodyTooDeep { actual, expected },
            location: Location::body(),
            decision: BDecision::Blocking,
        }
    }
    pub fn body_too_large(actual: usize, expected: usize) -> Self {
        BlockReason {
            initiator: Initiator::EntryTooLarge { actual, expected },
            location: Location::body(),
            decision: BDecision::Blocking,
        }
    }
    pub fn body_missing() -> Self {
        BlockReason {
            initiator: Initiator::BodyMissing,
            location: Location::body(),
            decision: BDecision::Blocking,
        }
    }
    pub fn body_malformed(cause: &str) -> Self {
        BlockReason {
            initiator: Initiator::BodyMalformed(cause.to_string()),
            location: Location::body(),
            decision: BDecision::Blocking,
        }
    }
    pub fn sqli(location: Location, fp: String) -> Self {
        BlockReason {
            initiator: Initiator::Sqli(fp),
            location: std::iter::once(location).collect(),
            decision: BDecision::Blocking,
        }
    }
    pub fn xss(location: Location) -> Self {
        BlockReason {
            initiator: Initiator::Xss,
            location: std::iter::once(location).collect(),
            decision: BDecision::Blocking,
        }
    }
    pub fn too_many_entries(idx: SectionIdx, actual: usize, expected: usize) -> Self {
        BlockReason {
            initiator: Initiator::TooManyEntries { actual, expected },
            location: std::iter::once(Location::from_section(idx)).collect(),
            decision: BDecision::Blocking,
        }
    }
    pub fn entry_too_large(idx: SectionIdx, name: &str, actual: usize, expected: usize) -> Self {
        BlockReason {
            initiator: Initiator::EntryTooLarge { actual, expected },
            location: std::iter::once(Location::from_name(idx, name)).collect(),
            decision: BDecision::Blocking,
        }
    }
    pub fn restricted(location: Location) -> Self {
        BlockReason {
            initiator: Initiator::Restricted,
            location: std::iter::once(location).collect(),
            decision: BDecision::Blocking,
        }
    }
    pub fn acl(tags: Tags, stage: AclStage) -> Self {
        let mut tagv = Vec::new();
        let mut location = HashSet::new();
        for (k, v) in tags.0.into_iter() {
            tagv.push(k);
            location.extend(v);
        }
        let decision = match stage {
            AclStage::Allow | AclStage::Bypass | AclStage::AllowBot => BDecision::Monitor,
            AclStage::Deny | AclStage::EnforceDeny | AclStage::DenyBot => BDecision::Blocking,
        };

        BlockReason {
            initiator: Initiator::Acl { tags: tagv, stage },
            location,
            decision,
        }
    }

    pub fn regroup<'t>(reasons: &'t [Self]) -> HashMap<InitiatorKind, Vec<&'t Self>> {
        let mut out: HashMap<InitiatorKind, Vec<&'t Self>> = HashMap::new();

        for reason in reasons {
            let kind = reason.initiator.to_kind();
            let entry = out.entry(kind).or_default();
            entry.push(reason);
        }

        out
    }

    pub fn serialize_in_map<S: serde::Serializer>(
        &self,
        map: &mut <S as serde::Serializer>::SerializeMap,
    ) -> Result<(), S::Error> {
        self.initiator.serialize_in_map::<S>(map)?;
        for loc in &self.location {
            loc.serialize_with_parent::<S>(map)?;
        }
        map.serialize_entry("active", &Value::Bool(self.decision != BDecision::Monitor))?;
        Ok(())
    }
}

pub struct LegacyBlockReason<'t>(&'t BlockReason);

impl<'t> Serialize for LegacyBlockReason<'t> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map: <S as serde::Serializer>::SerializeMap = serializer.serialize_map(None)?;
        map.serialize_entry("initiator", &self.0.initiator.to_kind())?;
        self.0.serialize_in_map::<S>(&mut map)?;
        map.end()
    }
}
