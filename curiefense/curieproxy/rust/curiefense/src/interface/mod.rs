use crate::config::hostmap::SecurityPolicy;
/// this file contains all the data type that are used when interfacing with a proxy
use crate::config::matchers::RequestSelector;
use crate::config::raw::{RawAction, RawActionType};
use crate::grasshopper::{challenge_phase01, GHMode, Grasshopper, PrecisionLevel};
use crate::logs::Logs;
use crate::utils::json::NameValue;
use crate::utils::templating::{parse_request_template, RequestTemplate, TVar, TemplatePart};
use crate::utils::{selector, GeoIp, MaskedRequestInfo, RequestInfo, RequestInfoG, Selected};
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
    if let Some(action) = &mut kept.maction {
        if action.atype == ActionType::Monitor {
            // if the kept action is monitor, the thrown action is monitor or pass, so we might need to merge headers
            let throw_headers = thrown.maction.and_then(|action| action.headers);
            if let Some(headers) = &mut action.headers {
                headers.extend(throw_headers.unwrap_or_default())
            } else {
                action.headers = throw_headers;
            }
        }
    }

    kept.reasons.extend(thrown.reasons);

    kept
}

/// identical to merge_decisions, but for simple decisions
pub fn stronger_decision(d1: SimpleDecision, d2: SimpleDecision) -> SimpleDecision {
    match (d1, d2) {
        (SimpleDecision::Pass, d2) => d2,
        (d1, SimpleDecision::Pass) => d1,
        (SimpleDecision::Action(mut s1, mut kept_reasons), SimpleDecision::Action(s2, br2)) => {
            kept_reasons.extend(br2);
            if s1.atype.priority() > s2.atype.priority() {
                SimpleDecision::Action(s1, kept_reasons)
            } else if s1.atype == SimpleActionT::Monitor && s2.atype == SimpleActionT::Monitor {
                s1.headers = match (s1.headers, s2.headers) {
                    (None, None) => None,
                    (Some(h1), None) => Some(h1),
                    (None, Some(h2)) => Some(h2),
                    (Some(mut h1), Some(h2)) => {
                        h1.extend(h2);
                        Some(h1)
                    }
                };
                SimpleDecision::Action(s1, kept_reasons)
            } else {
                SimpleDecision::Action(s2, kept_reasons)
            }
        }
    }
}

#[derive(Debug)]
pub struct AnalyzeResultG<A> {
    pub decision: Decision,
    pub tags: Tags,
    pub rinfo: RequestInfoG<A>,
    pub stats: Stats,
}

pub type AnalyzeResult = AnalyzeResultG<crate::utils::Raw>;
pub type MaskedAnalyzeResult = AnalyzeResultG<crate::utils::Masked>;

impl AnalyzeResult {
    pub fn mask(self) -> MaskedAnalyzeResult {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Decision {
    pub maction: Option<Action>,
    pub reasons: Vec<BlockReason>,
}

impl Decision {
    pub fn skip(id: String, name: String, initiator: Initiator, location: Location) -> Self {
        Decision {
            maction: None,
            reasons: vec![BlockReason {
                id,
                name,
                initiator,
                location,
                action: RawActionType::Skip,
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
            || self.reasons.iter().any(|r| r.action.is_final())
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
        rinfo: &MaskedRequestInfo,
        tags: &Tags,
        stats: &Stats,
        logs: &Logs,
        proxy: HashMap<String, String>,
    ) -> Vec<u8> {
        let (request_map, _) = jsonlog(
            self,
            Some(rinfo),
            self.maction.as_ref().map(|a| a.status),
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
    mrinfo: Option<&MaskedRequestInfo>,
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
                Err(_) => (b"null".to_vec(), now),
                Ok(y) => (y, now),
            }
        }
        None => (b"null".to_vec(), now),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn jsonlog_rinfo(
    dec: &Decision,
    rinfo: &MaskedRequestInfo,
    mut rcode: Option<u32>,
    tags: &Tags,
    stats: &Stats,
    logs: &Logs,
    proxy: HashMap<String, String>,
    now: &chrono::DateTime<chrono::Utc>,
) -> serde_json::Result<Vec<u8>> {
    let block_reason_desc = if dec.is_final() {
        BlockReason::block_reason_desc(&dec.reasons)
    } else {
        None
    };
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
    map_ser.serialize_entry("query", &rinfo.rinfo.qinfo.query)?;
    map_ser.serialize_entry("ip", &rinfo.rinfo.geoip.ip)?;
    map_ser.serialize_entry("method", &rinfo.rinfo.meta.method)?;
    map_ser.serialize_entry("response_code", &rcode)?;
    map_ser.serialize_entry("logs", logs)?;
    map_ser.serialize_entry("processing_stage", &stats.processing_stage)?;

    map_ser.serialize_entry("acl_triggers", get_trigger(&InitiatorKind::Acl))?;
    map_ser.serialize_entry("rl_triggers", get_trigger(&InitiatorKind::RateLimit))?;
    map_ser.serialize_entry("gf_triggers", get_trigger(&InitiatorKind::GlobalFilter))?;
    map_ser.serialize_entry("cf_triggers", get_trigger(&InitiatorKind::ContentFilter))?;
    map_ser.serialize_entry("cf_restrict_triggers", get_trigger(&InitiatorKind::Restriction))?;
    map_ser.serialize_entry("reason", &block_reason_desc)?;

    let branch_tag = tags.inner().keys().filter_map(|t| t.strip_prefix("branch:")).next();
    map_ser.serialize_entry("branch", &branch_tag)?;
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

    // If we have a monitor action, remove the return code to prevent tag
    // addition. This could be fixed with a better Action structure, but
    // requires more changes.
    if let Some(Action {
        atype: ActionType::Monitor,
        ..
    }) = &dec.maction
    {
        rcode = None;
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

    struct SecurityConfig<'t>(&'t Stats, &'t SecurityPolicy);
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
            mp.serialize_entry("rl_rules", &self.0.secpol.limit_amount)?;
            mp.serialize_entry("gf_rules", &self.0.secpol.globalfilters_amount)?;
            mp.serialize_entry("secpolid", &self.1.policy.id)?;
            mp.serialize_entry("secpolentryid", &self.1.entry.id)?;
            mp.end()
        }
    }
    map_ser.serialize_entry("security_config", &SecurityConfig(stats, &rinfo.rinfo.secpolicy))?;

    struct TriggerCounters<'t>(&'t HashMap<InitiatorKind, Vec<&'t BlockReason>>);
    impl<'t> Serialize for TriggerCounters<'t> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let stats_counter = |kd: InitiatorKind| -> usize {
                match self.0.get(&kd) {
                    None => 0,
                    Some(v) => v.len(),
                }
            };
            let acl = stats_counter(InitiatorKind::Acl);
            let global_filters = stats_counter(InitiatorKind::GlobalFilter);
            let rate_limit = stats_counter(InitiatorKind::RateLimit);
            let content_filters = stats_counter(InitiatorKind::ContentFilter);
            let restriction = stats_counter(InitiatorKind::Restriction);

            let mut mp = serializer.serialize_map(None)?;
            mp.serialize_entry("acl", &acl)?;
            mp.serialize_entry("gf", &global_filters)?;
            mp.serialize_entry("rl", &rate_limit)?;
            mp.serialize_entry("cf", &content_filters)?;
            mp.serialize_entry("cf_restrict", &restriction)?;
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
    mrinfo: Option<&MaskedRequestInfo>,
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
    pub atype: ActionType,
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
    Custom { content: String },
    Challenge { ch_level: GHMode },
}

impl SimpleActionT {
    fn priority(&self) -> u32 {
        use SimpleActionT::*;
        match self {
            Custom { content: _ } => 8,
            Challenge { ch_level: _ } => 6,
            Monitor => 1,
            Skip => 9,
        }
    }

    pub fn rate_limit_priority(&self) -> u32 {
        use SimpleActionT::*;
        match self {
            Custom { content: _ } => 8,
            Challenge { .. } => 6,
            Monitor => 1,
            // skip action should be ignored when using with rate limit
            Skip => 0,
        }
    }

    fn is_blocking(&self) -> bool {
        !matches!(self, SimpleActionT::Monitor)
    }

    pub fn to_raw(&self) -> RawActionType {
        match self {
            SimpleActionT::Skip => RawActionType::Skip,
            SimpleActionT::Monitor => RawActionType::Monitor,
            SimpleActionT::Custom { .. } => RawActionType::Custom,
            SimpleActionT::Challenge { ch_level } => {
                if ch_level == &GHMode::Active {
                    RawActionType::Challenge
                } else {
                    RawActionType::Ichallenge
                }
            }
        }
    }
}

// an action with its semantic meaning
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleAction {
    pub atype: SimpleActionT,
    pub headers: Option<HashMap<String, RequestTemplate>>,
    pub status: u32,
    pub extra_tags: Option<HashSet<String>>,
}

impl Default for SimpleAction {
    fn default() -> Self {
        SimpleAction {
            atype: SimpleActionT::default(),
            headers: None,
            status: 503,
            extra_tags: None,
        }
    }
}

impl Default for SimpleActionT {
    fn default() -> Self {
        SimpleActionT::Custom {
            content: "blocked".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Skip,
    Monitor,
    Block,
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

    pub fn priority(&self) -> u32 {
        match self {
            ActionType::Block => 6,
            ActionType::Monitor => 1,
            ActionType::Skip => 9,
        }
    }
}

impl std::default::Default for Action {
    fn default() -> Self {
        Action {
            atype: ActionType::Block,
            block_mode: true,
            status: 503,
            headers: None,
            content: "request denied".to_string(),
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
        let atype = match rawaction.type_ {
            RawActionType::Skip => SimpleActionT::Skip,
            RawActionType::Monitor => SimpleActionT::Monitor,
            RawActionType::Custom => SimpleActionT::Custom {
                content: rawaction.params.content.clone().unwrap_or_default(),
            },
            RawActionType::Challenge => SimpleActionT::Challenge {
                ch_level: GHMode::Active,
            },
            RawActionType::Ichallenge => SimpleActionT::Challenge {
                ch_level: GHMode::Interactive,
            },
        };
        let status = rawaction.params.status.unwrap_or(503);
        let headers = rawaction.params.headers.as_ref().map(|hm| {
            hm.iter()
                .map(|(k, v)| (k.to_string(), parse_request_template(v)))
                .collect()
        });
        let extra_tags = if rawaction.tags.is_empty() {
            None
        } else {
            Some(rawaction.tags.iter().cloned().collect())
        };

        Ok((
            id,
            SimpleAction {
                atype,
                status,
                headers,
                extra_tags,
            },
        ))
    }

    /// returns Err(reasons) when it is a challenge, Ok(decision) otherwise
    fn build_decision(
        &self,
        rinfo: &RequestInfo,
        tags: &Tags,
        precision_level: PrecisionLevel,
        reason: Vec<BlockReason>,
    ) -> Result<Decision, Vec<BlockReason>> {
        let mut action = Action::default();
        let mut reason = reason;
        action.block_mode = action.atype.is_blocking();
        action.status = self.status;
        action.headers = self.headers.as_ref().map(|hm| {
            hm.iter()
                .map(|(k, v)| (k.to_string(), render_template(rinfo, tags, v)))
                .collect()
        });
        match &self.atype {
            SimpleActionT::Skip => action.atype = ActionType::Skip,
            SimpleActionT::Monitor => action.atype = ActionType::Monitor,
            SimpleActionT::Custom { content } => {
                action.atype = ActionType::Block;
                action.content = content.clone();
            }
            SimpleActionT::Challenge { ch_level } => {
                let is_human = match ch_level {
                    GHMode::Passive => precision_level.is_human(),
                    GHMode::Active => precision_level.is_human(),
                    GHMode::Interactive => precision_level.is_interactive(),
                };
                if !is_human {
                    return Err(reason);
                }
                action.atype = ActionType::Monitor;
                // clean up challenge reasons
                for r in reason.iter_mut() {
                    r.action.inactive()
                }
            }
        }
        if action.atype == ActionType::Monitor {
            action.status = 200;
            action.block_mode = false;
        }
        Ok(Decision::action(action, reason))
    }

    pub fn to_decision<GH: Grasshopper>(
        &self,
        logs: &mut Logs,
        precision_level: PrecisionLevel,
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
        match self.build_decision(rinfo, tags, precision_level, reason) {
            Err(nreason) => match mgh {
                //if None-must be one of the challenge actions
                Some(gh) => {
                    let ch_mode = match &self.atype {
                        SimpleActionT::Challenge { ch_level } => *ch_level,
                        _ => GHMode::Active,
                    };
                    logs.debug(|| format!("Call challenge phase01 with mode: {:?}", ch_mode));
                    challenge_phase01(gh, logs, rinfo, nreason, ch_mode)
                }
                _ => Decision::action(Action::default(), nreason),
            },
            Ok(a) => a,
        }
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
