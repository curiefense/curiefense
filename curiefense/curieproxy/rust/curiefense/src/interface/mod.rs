/// this file contains all the data type that are used when interfacing with a proxy
use crate::config::matchers::RequestSelector;
use crate::config::raw::{RawAction, RawActionType};
use crate::grasshopper::{challenge_phase01, Grasshopper};
use crate::logs::Logs;
use crate::utils::json::NameValue;
use crate::utils::templating::{parse_request_template, RequestTemplate, TVar, TemplatePart};
use crate::utils::{selector, GeoIp, RequestInfo, Selected};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{HashMap, HashSet};

pub use self::block_reasons::*;
pub use self::stats::*;
pub use self::tagging::*;

pub mod aggregator;
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

/// Merge two decisions together.
///
/// If the two decisions have differents priorities, returns the one with
/// the highest one.
/// If the two decisions have the same priority and have action of type
/// Monitor, returns the first one, with headers merged from the two
/// decisions
/// If the two decisions have the same priority, but not actions of type
/// Monitor, retunrs the first decision.
///
/// In all cases, block reasons are always merged.
///
/// Priorities of actions are: Skip > Block > Monitor > None
pub fn merge_decisions(d1: Decision, d2: Decision) -> Decision {
    // Choose which decision to keep, and which decision to throw away
    let (mut kept, thrown) = {
        match (&d1.maction, &d2.maction) {
            (Some(a1), Some(a2)) => {
                if a1.atype.priority() >= a2.atype.priority() {
                    (d1, d2)
                } else {
                    (d2, d1)
                }
            }
            (None, Some(_)) => (d2, d1),
            (Some(_), None) | (None, None) => (d1, d2),
        }
    };

    // Merge headers if kept action is monitor
    if let Some(Action {
        atype: ActionType::Monitor { headers: kept_headers },
        ..
    }) = &mut kept.maction
    {
        if let Some(Action {
            atype: ActionType::Monitor {
                headers: Some(thrown_headers),
            },
            ..
        }) = thrown.maction
        {
            if let Some(k) = kept_headers {
                k.extend(thrown_headers)
            } else {
                *kept_headers = Some(thrown_headers)
            }
        }
    }

    kept.reasons.extend(thrown.reasons);

    kept
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
    pub fn skip(initiator: Initiator, location: Location) -> Self {
        Decision {
            maction: None,
            reasons: vec![BlockReason {
                initiator,
                location,
                decision: BDecision::Skip,
                extra_locations: Vec::new(),
                extra: serde_json::Value::Null,
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
        let action_desc = if self.is_blocking() { "custom_response" } else { "pass" };
        let response =
            serde_json::to_value(&self.maction).unwrap_or_else(|rr| serde_json::Value::String(rr.to_string()));
        let j = serde_json::json!({
            "action": action_desc,
            "response": response,
        });
        serde_json::to_string(&j).unwrap_or_else(|_| "{}".to_string())
    }

    pub async fn log_json(
        &self,
        rinfo: &RequestInfo,
        tags: &Tags,
        stats: &Stats,
        logs: &Logs,
        proxy: HashMap<String, String>,
    ) -> Vec<u8> {
        let (request_map, _) = jsonlog(
            self,
            Some(rinfo),
            self.maction.as_ref().and_then(|a| {
                if let ActionType::Block { status, .. } = a.atype {
                    Some(status)
                } else {
                    None
                }
            }),
            tags,
            stats,
            logs,
            proxy,
        )
        .await;
        request_map
    }
}

// helper function that reproduces the envoy log format
// this is the moment where we perform stats aggregation as we have the return code
pub async fn jsonlog(
    dec: &Decision,
    mrinfo: Option<&RequestInfo>,
    rcode: Option<u32>,
    tags: &Tags,
    stats: &Stats,
    logs: &Logs,
    proxy: HashMap<String, String>,
) -> (Vec<u8>, chrono::DateTime<chrono::Utc>) {
    let now = mrinfo.map(|i| i.timestamp).unwrap_or_else(chrono::Utc::now);
    let status_code = rcode.or_else(|| proxy.get("status").and_then(|stt_str| stt_str.parse().ok()));
    let bytes_sent = proxy.get("bytes_sent").and_then(|s| s.parse().ok());
    match mrinfo {
        Some(rinfo) => {
            aggregator::aggregate(dec, status_code, rinfo, tags, bytes_sent).await;
            match jsonlog_rinfo(dec, rinfo, status_code, tags, stats, logs, proxy, &now) {
                Err(rr) => {
                    println!("JSON creation error: {}", rr);
                    (b"null".to_vec(), now)
                }
                Ok(y) => (y, now),
            }
        }
        None => (b"null".to_vec(), now),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn jsonlog_rinfo(
    dec: &Decision,
    rinfo: &RequestInfo,
    rcode: Option<u32>,
    tags: &Tags,
    stats: &Stats,
    logs: &Logs,
    proxy: HashMap<String, String>,
    now: &chrono::DateTime<chrono::Utc>,
) -> serde_json::Result<Vec<u8>> {
    let block_reason_desc = BlockReason::block_reason_desc(&dec.reasons);
    let greasons = BlockReason::regroup(&dec.reasons);
    let get_trigger = |k: &InitiatorKind| -> &[&BlockReason] { greasons.get(k).map(|v| v.as_slice()).unwrap_or(&[]) };

    let mut outbuffer = Vec::<u8>::new();
    let mut ser = serde_json::Serializer::new(&mut outbuffer);
    let mut map_ser = ser.serialize_map(None)?;
    map_ser.serialize_entry("timestamp", now)?;
    //     map_ser.serialize_entry("@timestamp", now)?;
    map_ser.serialize_entry("curiesession", &rinfo.session)?;
    map_ser.serialize_entry("curiesession_ids", &NameValue::new(&rinfo.session_ids))?;
    let request_id = proxy.get("request_id").or(rinfo.rinfo.meta.requestid.as_ref());
    map_ser.serialize_entry("request_id", &request_id)?;
    map_ser.serialize_entry("arguments", &rinfo.rinfo.qinfo.args)?;
    map_ser.serialize_entry("path", &rinfo.rinfo.qinfo.qpath)?;
    map_ser.serialize_entry("path_parts", &rinfo.rinfo.qinfo.path_as_map)?;
    map_ser.serialize_entry("authority", &rinfo.rinfo.host)?;
    map_ser.serialize_entry("cookies", &rinfo.cookies)?;
    map_ser.serialize_entry("headers", &rinfo.headers)?;
    if !rinfo.plugins.is_empty() {
        map_ser.serialize_entry("plugins", &rinfo.plugins)?;
    }
    map_ser.serialize_entry("uri", &rinfo.rinfo.meta.path)?;
    map_ser.serialize_entry("ip", &rinfo.rinfo.geoip.ip)?;
    map_ser.serialize_entry("method", &rinfo.rinfo.meta.method)?;
    map_ser.serialize_entry("response_code", &rcode)?;
    map_ser.serialize_entry("logs", logs)?;
    map_ser.serialize_entry("processing_stage", &stats.processing_stage)?;

    map_ser.serialize_entry("acl_triggers", get_trigger(&InitiatorKind::Acl))?;
    map_ser.serialize_entry("rate_limit_triggers", get_trigger(&InitiatorKind::RateLimit))?;
    map_ser.serialize_entry("global_filter_triggers", get_trigger(&InitiatorKind::GlobalFilter))?;
    map_ser.serialize_entry("content_filter_triggers", get_trigger(&InitiatorKind::ContentFilter))?;
    map_ser.serialize_entry("restriction_triggers", get_trigger(&InitiatorKind::Restriction))?;
    map_ser.serialize_entry("reason", &block_reason_desc)?;

    // it's too bad one can't directly write the recursive structures from just the serializer object
    // that's why there are several one shot structures for nested data:
    struct LogTags<'t> {
        tags: &'t Tags,
        extra: Option<&'t HashSet<String>>,
        rcode: Option<u32>,
    }
    impl<'t> Serialize for LogTags<'t> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut code_vec: Vec<(&str, String)> = Vec::new();
            if let Some(code) = self.rcode {
                code_vec.push(("status", format!("{}", code)));
                code_vec.push(("status-class", format!("{}xx", code / 100)));
            }

            self.tags.serialize_with_extra(
                serializer,
                self.extra.iter().flat_map(|i| i.iter().map(|s| s.as_str())),
                code_vec.into_iter(),
            )
        }
    }

    map_ser.serialize_entry(
        "tags",
        &LogTags {
            tags,
            extra: dec.maction.as_ref().and_then(|a| a.extra_tags.as_ref()),
            rcode,
        },
    )?;

    struct LogProxy<'t> {
        p: &'t HashMap<String, String>,
        geo: &'t GeoIp,
        n: &'t Option<String>,
    }
    impl<'t> Serialize for LogProxy<'t> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut sq = serializer.serialize_seq(None)?;
            for (name, value) in self.p {
                sq.serialize_element(&crate::utils::json::BigTableKV { name, value })?;
            }
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_long",
                value: self.geo.location.as_ref().map(|x| x.0),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_lat",
                value: self.geo.location.as_ref().map(|x| x.1),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_as_name",
                value: self.geo.as_name.as_ref(),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_as_domain",
                value: self.geo.as_domain.as_ref(),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_as_type",
                value: self.geo.as_type.as_ref(),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_company_country",
                value: self.geo.company_country.as_ref(),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_company_domain",
                value: self.geo.company_domain.as_ref(),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_company_type",
                value: self.geo.company_type.as_ref(),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_mobile_carrier",
                value: self.geo.mobile_carrier_name.as_ref(),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_mobile_country",
                value: self.geo.mobile_country.as_ref(),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_mobile_mcc",
                value: self.geo.mobile_mcc.as_ref(),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "geo_mobile_mnc",
                value: self.geo.mobile_mnc.as_ref(),
            })?;
            sq.serialize_element(&crate::utils::json::BigTableKV {
                name: "container",
                value: self.n,
            })?;
            sq.end()
        }
    }
    map_ser.serialize_entry(
        "proxy",
        &LogProxy {
            p: &proxy,
            geo: &rinfo.rinfo.geoip,
            n: &rinfo.rinfo.container_name,
        },
    )?;

    struct SecurityConfig<'t>(&'t Stats);
    impl<'t> Serialize for SecurityConfig<'t> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut mp = serializer.serialize_map(None)?;
            mp.serialize_entry("revision", &self.0.revision)?;
            mp.serialize_entry("acl_active", &self.0.secpol.acl_enabled)?;
            mp.serialize_entry("cf_active", &self.0.secpol.content_filter_enabled)?;
            mp.serialize_entry("cf_rules", &self.0.content_filter_total)?;
            mp.serialize_entry("rate_limit_rules", &self.0.secpol.limit_amount)?;
            mp.serialize_entry("global_filters_active", &self.0.secpol.globalfilters_amount)?;
            mp.end()
        }
    }
    map_ser.serialize_entry("security_config", &SecurityConfig(stats))?;

    struct TriggerCounters<'t>(&'t HashMap<InitiatorKind, Vec<&'t BlockReason>>);
    impl<'t> Serialize for TriggerCounters<'t> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let stats_counter = |kd: InitiatorKind| -> (usize, usize) {
                match self.0.get(&kd) {
                    None => (0, 0),
                    Some(v) => (v.len(), v.iter().filter(|r| r.decision == BDecision::Blocking).count()),
                }
            };
            let (acl, acl_active) = stats_counter(InitiatorKind::Acl);
            let (global_filters, global_filters_active) = stats_counter(InitiatorKind::GlobalFilter);
            let (rate_limit, rate_limit_active) = stats_counter(InitiatorKind::RateLimit);
            let (content_filters, content_filters_active) = stats_counter(InitiatorKind::ContentFilter);

            let mut mp = serializer.serialize_map(None)?;
            mp.serialize_entry("acl", &acl)?;
            mp.serialize_entry("acl_active", &acl_active)?;
            mp.serialize_entry("global_filters", &global_filters)?;
            mp.serialize_entry("global_filters_active", &global_filters_active)?;
            mp.serialize_entry("rate_limit", &rate_limit)?;
            mp.serialize_entry("rate_limit_active", &rate_limit_active)?;
            mp.serialize_entry("content_filters", &content_filters)?;
            mp.serialize_entry("content_filters_active", &content_filters_active)?;
            mp.end()
        }
    }
    map_ser.serialize_entry("trigger_counters", &TriggerCounters(&greasons))?;

    struct EmptyMap;
    impl Serialize for EmptyMap {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mp = serializer.serialize_map(Some(0))?;
            mp.end()
        }
    }
    map_ser.serialize_entry("profiling", &stats.timing)?;
    SerializeMap::end(map_ser)?;
    Ok(outbuffer)
}

// blocking version
pub fn jsonlog_block(
    dec: &Decision,
    mrinfo: Option<&RequestInfo>,
    rcode: Option<u32>,
    tags: &Tags,
    stats: &Stats,
    logs: &Logs,
    proxy: HashMap<String, String>,
) -> (Vec<u8>, chrono::DateTime<chrono::Utc>) {
    async_std::task::block_on(jsonlog(dec, mrinfo, rcode, tags, stats, logs, proxy))
}

// an action, as formatted for outside consumption
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Action {
    #[serde(flatten)]
    pub atype: ActionType,
    pub extra_tags: Option<HashSet<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleActionT {
    Skip,
    Monitor {
        headers: Option<HashMap<String, RequestTemplate>>,
    },
    Custom {
        status: u32,
        headers: Option<HashMap<String, RequestTemplate>>,
        content: String,
    },
    Challenge,
}

impl SimpleActionT {
    fn priority(&self) -> u32 {
        use SimpleActionT::*;
        match self {
            Custom { .. } => 8,
            Challenge => 6,
            Monitor { .. } => 1,
            Skip => 9,
        }
    }

    pub fn rate_limit_priority(&self) -> u32 {
        use SimpleActionT::*;
        match self {
            Custom { .. } => 8,
            Challenge => 6,
            Monitor { .. } => 1,
            // skip action should be ignored when using with rate limit
            Skip => 0,
        }
    }

    fn is_blocking(&self) -> bool {
        !matches!(self, SimpleActionT::Monitor { .. })
    }

    pub fn to_bdecision(&self) -> BDecision {
        match self {
            SimpleActionT::Skip => BDecision::Skip,
            SimpleActionT::Monitor { .. } => BDecision::Monitor,
            SimpleActionT::Challenge | SimpleActionT::Custom { .. } => BDecision::Blocking,
        }
    }
}

// an action with its semantic meaning
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SimpleAction {
    pub atype: SimpleActionT,
    pub extra_tags: Option<HashSet<String>>,
}

impl Default for SimpleActionT {
    fn default() -> Self {
        SimpleActionT::Custom {
            status: 503,
            headers: None,
            content: "blocked".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum ActionType {
    Skip,
    Monitor {
        headers: Option<HashMap<String, String>>,
    },
    Block {
        status: u32,
        headers: Option<HashMap<String, String>>,
        content: String,
    },
}

impl ActionType {
    /// is the action blocking (not passed to the underlying server)
    pub fn is_blocking(&self) -> bool {
        matches!(self, ActionType::Block { .. })
    }

    /// is the action final (no further processing)
    pub fn is_final(&self) -> bool {
        !matches!(self, ActionType::Monitor { .. })
    }

    pub fn priority(&self) -> u32 {
        match self {
            ActionType::Block { .. } => 6,
            ActionType::Monitor { .. } => 1,
            ActionType::Skip => 9,
        }
    }
}

impl std::default::Default for Action {
    fn default() -> Self {
        Action {
            atype: ActionType::Block {
                status: 503,
                headers: None,
                content: "request denied".to_string(),
            },
            extra_tags: None,
        }
    }
}

impl SimpleAction {
    pub fn resolve_actions(logs: &mut Logs, rawactions: Vec<RawAction>) -> HashMap<String, Self> {
        let mut out = HashMap::new();
        for raction in rawactions {
            match Self::resolve(&raction) {
                Ok((id, action)) => {
                    out.insert(id, action);
                }
                Err(r) => logs.error(|| format!("Could not resolve action {}: {}", raction.id, r)),
            }
        }
        out
    }

    fn resolve(rawaction: &RawAction) -> anyhow::Result<(String, SimpleAction)> {
        let id = rawaction.id.clone();

        let status = rawaction.params.status.unwrap_or(503);
        let headers = rawaction.params.headers.as_ref().map(|hm| {
            hm.iter()
                .map(|(k, v)| (k.to_string(), parse_request_template(v)))
                .collect()
        });
        let content = rawaction.params.content.clone().unwrap_or_default();

        let atype = match rawaction.type_ {
            RawActionType::Skip => SimpleActionT::Skip,
            RawActionType::Monitor => SimpleActionT::Monitor { headers },
            RawActionType::Custom => SimpleActionT::Custom {
                status,
                headers,
                content,
            },
            RawActionType::Challenge => SimpleActionT::Challenge,
        };
        let extra_tags = if rawaction.tags.is_empty() {
            None
        } else {
            Some(rawaction.tags.iter().cloned().collect())
        };

        Ok((id, SimpleAction { atype, extra_tags }))
    }

    /// returns None when it is a challenge, Some(action) otherwise
    fn to_action(&self, rinfo: &RequestInfo, tags: &Tags, is_human: bool) -> Option<Action> {
        let mut action = Action::default();

        let resolve_headers = |hm: &HashMap<String, RequestTemplate>| -> HashMap<String, String> {
            hm.iter()
                .map(|(k, v)| (k.to_string(), render_template(rinfo, tags, v)))
                .collect()
        };

        action.atype = match &self.atype {
            SimpleActionT::Skip => ActionType::Skip,
            SimpleActionT::Monitor { headers } => ActionType::Monitor {
                headers: headers.as_ref().map(resolve_headers),
            },
            SimpleActionT::Custom {
                status,
                headers,
                content,
            } => ActionType::Block {
                status: *status,
                headers: headers.as_ref().map(resolve_headers),
                content: content.clone(),
            },
            SimpleActionT::Challenge => {
                if !is_human {
                    action.atype
                } else {
                    ActionType::Monitor { headers: None }
                }
            }
        };
        Some(action)
    }

    pub fn to_decision<GH: Grasshopper>(
        &self,
        is_human: bool,
        mgh: Option<&GH>,
        rinfo: &RequestInfo,
        tags: &mut Tags,
        reason: Vec<BlockReason>,
    ) -> Decision {
        for t in self.extra_tags.iter().flat_map(|s| s.iter()) {
            tags.insert(t, Location::Request);
        }
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
            TemplatePart::Var(TVar::Selector(RequestSelector::Tags)) => {
                out.push_str(&serde_json::to_string(&tags).unwrap_or_else(|_| "null".into()))
            }
            TemplatePart::Var(TVar::Tag(tagname)) => {
                out.push_str(if tags.contains(tagname) { "true" } else { "false" })
            }
            TemplatePart::Var(TVar::Selector(sel)) => match selector(rinfo, sel, Some(tags)) {
                None => out.push_str("nil"),
                Some(Selected::OStr(s)) => out.push_str(&s),
                Some(Selected::Str(s)) => out.push_str(s),
                Some(Selected::U32(v)) => out.push_str(&v.to_string()),
            },
        }
    }
    out
}
