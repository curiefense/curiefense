use async_std::sync::Mutex;
use chrono::Utc;
use lazy_static::lazy_static;
use pdatastructs::hyperloglog::HyperLogLog;
use serde::Serialize;
use serde_json::Value;
use std::collections::{btree_map::Entry, BTreeMap, HashMap};
use std::hash::Hash;

use crate::config::raw::RawActionType;
use crate::utils::MaskedRequestInfo;

use super::{Decision, Location, Tags};

lazy_static! {
    static ref AGGREGATED: Mutex<HashMap<AggregationKey, BTreeMap<i64, AggregatedCounters>>> =
        Mutex::new(HashMap::new());
    static ref SAMPLES_KEPT: i64 = std::env::var("AGGREGATED_SAMPLES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);
    static ref SAMPLE_DURATION: i64 = std::env::var("SAMPLE_DURATION")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);
    static ref TOP_AMOUNT: usize = std::env::var("AGGREGATED_TOP")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(25);
    static ref HYPERLOGLOG_SIZE: usize = std::env::var("AGGREGATED_HLL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);
    static ref PLANET_NAME: String = std::env::var("CF_PLANET_NAME").ok().unwrap_or_default();
    static ref EMPTY_AGGREGATED_DATA: AggregatedCounters = AggregatedCounters::default();
}

#[derive(Debug, Default)]
struct Arp<T> {
    active: T,
    report: T,
    pass: T,
}

#[derive(Clone, Copy)]
enum ArpCursor {
    Active,
    Report,
    Pass,
}

impl<T> Arp<T> {
    fn get(&self, cursor: ArpCursor) -> &T {
        match cursor {
            ArpCursor::Active => &self.active,
            ArpCursor::Report => &self.report,
            ArpCursor::Pass => &self.pass,
        }
    }

    fn get_mut(&mut self, cursor: ArpCursor) -> &mut T {
        match cursor {
            ArpCursor::Active => &mut self.active,
            ArpCursor::Report => &mut self.report,
            ArpCursor::Pass => &mut self.pass,
        }
    }
}

impl<T: Serialize> Arp<T> {
    fn serialize(&self, mp: &mut serde_json::Map<String, Value>, prefix: &str) {
        mp.insert(
            format!("{}active", prefix),
            serde_json::to_value(self.get(ArpCursor::Active)).unwrap_or(Value::Null),
        );
        mp.insert(
            format!("{}reported", prefix),
            serde_json::to_value(self.get(ArpCursor::Report)).unwrap_or(Value::Null),
        );
        mp.insert(
            format!("{}passed", prefix),
            serde_json::to_value(self.get(ArpCursor::Pass)).unwrap_or(Value::Null),
        );
    }
}

/// Helper structure to display both the Autonomous System number and name in
/// aggregated data.
///
/// It is just a wrapper around a u32, with an additional string description.
#[derive(Debug, Default, Clone)]
struct AutonomousSystem {
    number: u32,
    company_name: Option<String>,
}

impl PartialEq for AutonomousSystem {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}
impl Eq for AutonomousSystem {}

impl PartialOrd for AutonomousSystem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.number.partial_cmp(&other.number)
    }
}

impl Ord for AutonomousSystem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

impl Hash for AutonomousSystem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

impl Serialize for AutonomousSystem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(company_name) = &self.company_name {
            serializer.serialize_str(&format!("{} ({})", self.number, company_name))
        } else {
            serializer.serialize_str(self.number.to_string().as_str())
        }
    }
}

#[derive(Debug, Default)]
struct AggregatedCounters {
    status: Bag<u32>,
    status_classes: Bag<u8>,
    methods: Bag<String>,
    bytes_sent: IntegerMetric,

    // by decision
    hits: usize,
    requests: Arp<usize>,
    requests_triggered_globalfilter_active: usize,
    requests_triggered_globalfilter_report: usize,
    requests_triggered_cf_active: usize,
    requests_triggered_cf_report: usize,
    requests_triggered_restriction_active: usize,
    requests_triggered_restriction_report: usize,
    requests_triggered_acl_active: usize,
    requests_triggered_acl_report: usize,
    requests_triggered_ratelimit_active: usize,
    requests_triggered_ratelimit_report: usize,

    authority: Arp<TopN<String>>,
    aclid: Arp<TopN<String>>,
    cfid: Arp<TopN<String>>,

    location: Arp<AggSection>,
    ruleid: Arp<TopN<String>>,
    risk_level: Arp<Bag<u8>>,
    top_tags: Arp<TopN<String>>,
    top_country_human: TopN<String>,
    top_country_bot: TopN<String>,
    top_rtc: Arp<TopN<String>>,

    bot: usize,
    human: usize,
    challenge: usize,

    // per request
    /// Processing time in microseconds
    processing_time: IntegerMetric,
    ip: Metric<String>,
    session: Metric<String>,
    uri: Metric<String>,
    user_agent: Metric<String>,
    country: Metric<String>,
    asn: Metric<AutonomousSystem>,
    headers_amount: Bag<usize>,
    cookies_amount: Bag<usize>,
    args_amount: Bag<usize>,

    // x by y
    ip_per_uri: UniqueTopNBy<String, String>,
    uri_per_ip: UniqueTopNBy<String, String>,
    session_per_uri: UniqueTopNBy<String, String>,
    uri_per_session: UniqueTopNBy<String, String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct AggregationKey {
    proxy: Option<String>,
    secpolid: String,
    secpolentryid: String,
    branch: String,
}

/// structure used for serialization
#[derive(Serialize)]
struct KV<K: Serialize, V: Serialize> {
    key: K,
    value: V,
}

/// implementation adapted from https://github.com/blt/quantiles/blob/master/src/misra_gries.rs
#[derive(Debug)]
struct TopN<N> {
    k: usize,
    counters: BTreeMap<N, usize>,
}

impl<N: Eq + Ord> Default for TopN<N> {
    fn default() -> Self {
        Self {
            k: *TOP_AMOUNT * 2,
            counters: Default::default(),
        }
    }
}

impl<N: Ord> TopN<N> {
    fn inc(&mut self, n: N) {
        let counters_len = self.counters.len();
        let mut counted = false;

        match self.counters.entry(n) {
            Entry::Occupied(mut item) => {
                *item.get_mut() += 1;
                counted = true;
            }
            Entry::Vacant(slot) => {
                if counters_len < self.k {
                    slot.insert(1);
                    counted = true;
                }
            }
        }

        if !counted {
            self.counters.retain(|_, v| {
                *v -= 1;
                *v != 0
            });
        }
    }
}

impl<N: Eq + Serialize> Serialize for TopN<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // collect top N
        let mut v = self
            .counters
            .iter()
            .map(|(k, v)| KV { key: k, value: *v })
            .collect::<Vec<_>>();
        v.sort_by(|a, b| b.value.cmp(&a.value));

        serializer.collect_seq(v.iter().take(*TOP_AMOUNT))
    }
}

#[derive(Debug, Default)]
struct Bag<N> {
    inner: HashMap<N, usize>,
}

impl<N: Eq + std::hash::Hash + std::fmt::Display> Bag<N> {
    fn inc(&mut self, n: N) {
        self.insert(n, 1);
    }

    fn insert(&mut self, n: N, amount: usize) {
        let entry = self.inner.entry(n).or_default();
        *entry += amount;
    }

    fn sorted_to_value(v: Vec<(String, usize)>) -> Value {
        Value::Array(
            v.into_iter()
                .take(*TOP_AMOUNT)
                .map(|(k, v)| {
                    let mut mp = serde_json::Map::new();
                    mp.insert("key".into(), Value::String(k));
                    mp.insert("value".into(), Value::Number(serde_json::Number::from(v)));
                    Value::Object(mp)
                })
                .collect(),
        )
    }

    fn serialize_top(&self) -> Value {
        let mut v = self.inner.iter().map(|(k, v)| (k.to_string(), *v)).collect::<Vec<_>>();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        Self::sorted_to_value(v)
    }

    fn serialize_max(&self) -> Value {
        let mut v = self.inner.iter().map(|(k, v)| (k.to_string(), *v)).collect::<Vec<_>>();
        v.sort_by(|a, b| b.0.cmp(&a.0));
        Self::sorted_to_value(v)
    }
}

impl<N: Serialize + Eq + std::hash::Hash> Serialize for Bag<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.inner.iter().map(|(k, v)| KV { key: k, value: v }))
    }
}

#[derive(Debug)]
struct Metric<T: Eq + Clone + std::hash::Hash> {
    unique: HyperLogLog<T>,
    unique_b: Arp<HyperLogLog<T>>,
    top: Arp<TopN<T>>,
}

impl<T: Ord + Clone + std::hash::Hash> Default for Metric<T> {
    fn default() -> Self {
        Self {
            unique: HyperLogLog::new(*HYPERLOGLOG_SIZE),
            unique_b: Arp {
                pass: HyperLogLog::new(*HYPERLOGLOG_SIZE),
                active: HyperLogLog::new(*HYPERLOGLOG_SIZE),
                report: HyperLogLog::new(*HYPERLOGLOG_SIZE),
            },
            top: Default::default(),
        }
    }
}

impl<T: Ord + std::hash::Hash + Clone> Metric<T> {
    fn inc(&mut self, n: &T, cursor: ArpCursor) {
        self.unique.add(n);
        self.unique_b.get_mut(cursor).add(n);
        self.top.get_mut(cursor).inc(n.clone());
    }
}

impl<T: Eq + Clone + std::hash::Hash + Serialize> Metric<T> {
    fn serialize_map(&self, tp: &str, mp: &mut serde_json::Map<String, Value>) {
        mp.insert(
            format!("unique_{}", tp),
            Value::Number(serde_json::Number::from(self.unique.count())),
        );
        mp.insert(
            format!("unique_{}_active", tp),
            Value::Number(serde_json::Number::from(self.unique_b.get(ArpCursor::Active).count())),
        );
        mp.insert(
            format!("unique_{}_reported", tp),
            Value::Number(serde_json::Number::from(self.unique_b.get(ArpCursor::Report).count())),
        );
        mp.insert(
            format!("unique_{}_passed", tp),
            Value::Number(serde_json::Number::from(self.unique_b.get(ArpCursor::Pass).count())),
        );
        mp.insert(
            format!("top_{}_active", tp),
            serde_json::to_value(self.top.get(ArpCursor::Active)).unwrap_or(Value::Null),
        );
        mp.insert(
            format!("top_{}_reported", tp),
            serde_json::to_value(self.top.get(ArpCursor::Report)).unwrap_or(Value::Null),
        );
        mp.insert(
            format!("top_{}_passed", tp),
            serde_json::to_value(self.top.get(ArpCursor::Pass)).unwrap_or(Value::Null),
        );
    }
}

#[derive(Debug, Default)]
struct UniqueTopNBy<N, B: std::hash::Hash> {
    inner: HashMap<N, HyperLogLog<B>>,
}

impl<N: Eq + std::hash::Hash, B: Eq + std::hash::Hash> UniqueTopNBy<N, B> {
    fn add(&mut self, n: N, by: &B) {
        let entry = self
            .inner
            .entry(n)
            .or_insert_with(|| HyperLogLog::new(*HYPERLOGLOG_SIZE));
        entry.add(by);
    }
}

impl<N: Ord + std::hash::Hash + Serialize, B: Eq + std::hash::Hash> Serialize for UniqueTopNBy<N, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut content = self
            .inner
            .iter()
            .map(|(n, lgs)| KV {
                key: n,
                value: lgs.count(),
            })
            .collect::<Vec<_>>();
        content.sort_by(|a, b| b.value.cmp(&a.value));
        serializer.collect_seq(content.into_iter().take(*TOP_AMOUNT))
    }
}

#[derive(Debug)]
struct IntegerMetric {
    min: i64,
    max: i64,
    total: i64,
    n_sample: u64,
}

impl Default for IntegerMetric {
    fn default() -> Self {
        IntegerMetric {
            min: i64::MAX,
            max: i64::MIN,
            total: 0,
            n_sample: 0,
        }
    }
}

impl IntegerMetric {
    fn increment(&mut self, sample: i64) {
        self.n_sample += 1;
        self.min = self.min.min(sample);
        self.max = self.max.max(sample);
        self.total += sample;
    }

    fn average(&self) -> f64 {
        if self.n_sample == 0 {
            return 0.0;
        }
        self.total as f64 / self.n_sample as f64
    }

    fn to_json(&self) -> Value {
        if self.n_sample == 0 {
            // Even if min and max are u64, both u64 and f64 are represented as Number is JSON.
            return serde_json::json!({ "min": 0, "max": 0, "average": 0.0 });
        }
        serde_json::json!({
            "min": self.min,
            "max": self.max,
            "average": self.average(),
        })
    }
}

#[derive(Debug, Default, Serialize)]
pub struct AggSection {
    headers: usize,
    uri: usize,
    args: usize,
    body: usize,
    attrs: usize,
    plugins: usize,
}

fn is_autotag_prefix(s: &str) -> bool {
    matches!(
        s,
        "securitypolicy"
            | "securitypolicy-entry"
            | "aclid"
            | "aclname"
            | "contentfilterid"
            | "contentfiltername"
            | "cf-rule-id"
            | "cf-rule-category"
            | "cf-rule-subcategory"
            | "cf-rule-risk"
            | "fc-id"
            | "fc-name"
            | "limit-id"
            | "limit-name"
            | "headers"
            | "cookies"
            | "args"
            | "host"
            | "ip"
            | "geo-continent-name"
            | "geo-continent-code"
            | "geo-city"
            | "geo-org"
            | "geo-country"
            | "geo-region"
            | "network"
            | "geo-subregion"
            | "geo-asn"
    )
}

impl AggregatedCounters {
    fn increment(
        &mut self,
        dec: &Decision,
        rcode: Option<u32>,
        rinfo: &MaskedRequestInfo,
        tags: &Tags,
        bytes_sent: Option<usize>,
    ) {
        self.hits += 1;

        let mut blocked = false;
        let mut skipped = false;
        let mut acl_blocked = false;
        let mut acl_report = false;
        let mut cf_blocked = false;
        let mut cf_report = false;
        for r in &dec.reasons {
            use super::Initiator::*;
            let this_blocked = match r.action {
                RawActionType::Skip => {
                    skipped = true;
                    false
                }
                RawActionType::Monitor => false,
                RawActionType::Custom | RawActionType::Challenge | RawActionType::Ichallenge => {
                    blocked = true;
                    true
                }
            };
            match &r.initiator {
                GlobalFilter => {
                    if this_blocked {
                        self.requests_triggered_globalfilter_active += 1;
                    } else {
                        self.requests_triggered_globalfilter_report += 1;
                    }
                }
                Acl { tags: _, stage } => {
                    if this_blocked {
                        acl_blocked = true;
                        self.requests_triggered_acl_active += 1;
                        if stage == &crate::interface::AclStage::DenyBot {
                            self.challenge += 1;
                        }
                    } else {
                        acl_report = true;
                        self.requests_triggered_acl_report += 1;
                    }
                }
                Phase01Fail(_) => (),
                Phase02 => {
                    if this_blocked {
                        self.requests_triggered_acl_active += 1;
                    } else {
                        self.requests_triggered_acl_report += 1;
                    }
                    self.challenge += 1;
                }
                Limit { threshold: _ } => {
                    if this_blocked {
                        self.requests_triggered_ratelimit_active += 1;
                    } else {
                        self.requests_triggered_ratelimit_report += 1;
                    }
                }

                ContentFilter { ruleid, risk_level } => {
                    let cursor = if this_blocked {
                        cf_blocked = true;
                        self.requests_triggered_cf_active += 1;
                        ArpCursor::Active
                    } else {
                        cf_report = true;
                        self.requests_triggered_cf_report += 1;
                        ArpCursor::Report
                    };
                    self.ruleid.get_mut(cursor).inc(ruleid.clone());
                    self.risk_level.get_mut(cursor).inc(*risk_level);
                }
                Restriction { .. } => {
                    if this_blocked {
                        self.requests_triggered_restriction_active += 1;
                    } else {
                        self.requests_triggered_restriction_report += 1;
                    }
                }
            }
            for loc in std::iter::once(&r.location).chain(r.extra_locations.iter()) {
                let aggloc = if this_blocked {
                    self.location.get_mut(ArpCursor::Active)
                } else {
                    self.location.get_mut(ArpCursor::Report)
                };
                match loc {
                    Location::Body => aggloc.body += 1,
                    Location::Attributes => aggloc.attrs += 1,
                    Location::Uri => aggloc.uri += 1,
                    Location::Headers => aggloc.headers += 1,
                    Location::UriArgumentValue(_, _)
                    | Location::RefererArgumentValue(_, _)
                    | Location::BodyArgumentValue(_, _)
                    | Location::BodyArgument(_)
                    | Location::RefererArgument(_)
                    | Location::UriArgument(_) => aggloc.args += 1,
                    Location::Request => (),
                    Location::Ip => aggloc.attrs += 1,
                    Location::Pathpart(_) | Location::PathpartValue(_, _) => aggloc.uri += 1,
                    Location::Header(_)
                    | Location::HeaderValue(_, _)
                    | Location::RefererPath
                    | Location::RefererPathpart(_)
                    | Location::RefererPathpartValue(_, _) => aggloc.headers += 1,
                    Location::Cookies | Location::Cookie(_) | Location::CookieValue(_, _) => aggloc.headers += 1,
                    Location::Plugins | Location::Plugin(_) | Location::PluginValue(_, _) => aggloc.plugins += 1,
                }
            }
        }
        blocked &= !skipped;
        acl_report |= acl_blocked & !skipped;
        acl_blocked &= !skipped;
        cf_report |= cf_blocked & !skipped;
        cf_blocked &= !skipped;

        let acl_cursor = if acl_blocked {
            ArpCursor::Active
        } else if acl_report {
            ArpCursor::Report
        } else {
            ArpCursor::Pass
        };
        let cf_cursor = if cf_blocked {
            ArpCursor::Active
        } else if cf_report {
            ArpCursor::Report
        } else {
            ArpCursor::Pass
        };

        let cursor = if blocked {
            ArpCursor::Active
        } else if dec.reasons.is_empty() || skipped {
            ArpCursor::Pass
        } else {
            ArpCursor::Report
        };

        self.aclid
            .get_mut(acl_cursor)
            .inc(rinfo.rinfo.secpolicy.acl_profile.id.to_string());
        self.cfid
            .get_mut(cf_cursor)
            .inc(rinfo.rinfo.secpolicy.content_filter_profile.id.to_string());
        *self.requests.get_mut(cursor) += 1;
        self.authority.get_mut(cursor).inc(rinfo.rinfo.host.to_string());
        let top_tags = self.top_tags.get_mut(cursor);

        let mut human = false;
        for tag in tags.tags.keys() {
            match tag.as_str() {
                "all" => (),
                "bot" => self.bot += 1,
                "human" => {
                    human = true;
                    self.human += 1
                }
                tg => match tg.split_once(':') {
                    None => top_tags.inc(tg.to_string()),
                    Some(("rtc", rtc)) => self.top_rtc.get_mut(cursor).inc(rtc.to_string()),
                    Some((prefix, _)) => {
                        if !is_autotag_prefix(prefix) {
                            top_tags.inc(tg.to_string())
                        }
                    }
                },
            }
        }

        if let Some(code) = rcode {
            self.status.inc(code);
            self.status_classes.inc((code / 100) as u8);
        }
        if let Some(bytes_sent) = bytes_sent {
            self.bytes_sent.increment(bytes_sent as i64);
        }

        self.methods.inc(rinfo.rinfo.meta.method.clone());

        if let Some(processing_time) = Utc::now().signed_duration_since(rinfo.timestamp).num_microseconds() {
            self.processing_time.increment(processing_time)
        }

        self.ip.inc(&rinfo.rinfo.geoip.ipstr, cursor);
        self.session.inc(&rinfo.session, cursor);
        self.uri.inc(&rinfo.rinfo.qinfo.uri, cursor);
        if let Some(user_agent) = &rinfo.headers.get("user-agent") {
            self.user_agent.inc(user_agent, cursor);
        }
        if let Some(country) = &rinfo.rinfo.geoip.country_iso {
            self.country.inc(country, cursor);
            if human {
                self.top_country_human.inc(country.to_string());
            } else {
                self.top_country_bot.inc(country.to_string());
            }
        }
        if let Some(asn) = &rinfo.rinfo.geoip.asn {
            self.asn.inc(
                &AutonomousSystem {
                    number: *asn,
                    company_name: rinfo.rinfo.geoip.company.clone(),
                },
                cursor,
            );
        }

        self.args_amount.inc(rinfo.rinfo.qinfo.args.len());
        self.cookies_amount.inc(rinfo.cookies.len());
        self.headers_amount.inc(rinfo.headers.len());

        self.ip_per_uri
            .add(rinfo.rinfo.geoip.ipstr.clone(), &rinfo.rinfo.qinfo.uri);
        self.uri_per_ip
            .add(rinfo.rinfo.qinfo.uri.clone(), &rinfo.rinfo.geoip.ipstr);
        self.session_per_uri.add(rinfo.session.clone(), &rinfo.rinfo.qinfo.uri);
        self.uri_per_session.add(rinfo.rinfo.qinfo.uri.clone(), &rinfo.session);
    }
}

fn serialize_counters(e: &AggregatedCounters) -> Value {
    let mut content = serde_json::Map::new();

    content.insert("hits".into(), Value::Number(serde_json::Number::from(e.hits)));
    content.insert(
        "active".into(),
        Value::Number(serde_json::Number::from(*e.requests.get(ArpCursor::Active))),
    );
    content.insert(
        "reported".into(),
        Value::Number(serde_json::Number::from(*e.requests.get(ArpCursor::Report))),
    );
    content.insert(
        "passed".into(),
        Value::Number(serde_json::Number::from(*e.requests.get(ArpCursor::Pass))),
    );
    content.insert("bot".into(), Value::Number(serde_json::Number::from(e.bot)));
    content.insert("human".into(), Value::Number(serde_json::Number::from(e.human)));
    content.insert("challenge".into(), Value::Number(serde_json::Number::from(e.challenge)));

    e.location.serialize(&mut content, "section_");
    e.ruleid.serialize(&mut content, "top_ruleid_");
    e.top_rtc.serialize(&mut content, "top_rtc_");
    e.aclid.serialize(&mut content, "top_aclid_");
    e.authority.serialize(&mut content, "top_authority_");
    content.insert(
        "risk_level_active".into(),
        serde_json::to_value(e.risk_level.get(ArpCursor::Active)).unwrap_or(Value::Null),
    );
    content.insert(
        "risk_level_report".into(),
        serde_json::to_value(e.risk_level.get(ArpCursor::Report)).unwrap_or(Value::Null),
    );
    content.insert(
        "requests_triggered_globalfilter_active".into(),
        Value::Number(serde_json::Number::from(e.requests_triggered_globalfilter_active)),
    );
    content.insert(
        "requests_triggered_globalfilter_report".into(),
        Value::Number(serde_json::Number::from(e.requests_triggered_globalfilter_report)),
    );
    content.insert(
        "requests_triggered_restriction_active".into(),
        Value::Number(serde_json::Number::from(e.requests_triggered_restriction_active)),
    );
    content.insert(
        "requests_triggered_restriction_report".into(),
        Value::Number(serde_json::Number::from(e.requests_triggered_restriction_report)),
    );
    content.insert(
        "requests_triggered_cf_active".into(),
        Value::Number(serde_json::Number::from(e.requests_triggered_cf_active)),
    );
    content.insert(
        "requests_triggered_cf_report".into(),
        Value::Number(serde_json::Number::from(e.requests_triggered_cf_report)),
    );
    content.insert(
        "requests_triggered_acl_active".into(),
        Value::Number(serde_json::Number::from(e.requests_triggered_acl_active)),
    );
    content.insert(
        "requests_triggered_acl_report".into(),
        Value::Number(serde_json::Number::from(e.requests_triggered_acl_report)),
    );
    content.insert(
        "requests_triggered_ratelimit_active".into(),
        Value::Number(serde_json::Number::from(e.requests_triggered_ratelimit_active)),
    );
    content.insert(
        "requests_triggered_ratelimit_report".into(),
        Value::Number(serde_json::Number::from(e.requests_triggered_ratelimit_report)),
    );

    content.insert("processing_time".into(), e.processing_time.to_json());
    content.insert("bytes_sent".into(), e.bytes_sent.to_json());
    e.ip.serialize_map("ip", &mut content);
    e.session.serialize_map("session", &mut content);
    e.uri.serialize_map("uri", &mut content);
    e.user_agent.serialize_map("user_agent", &mut content);
    e.country.serialize_map("country", &mut content);
    e.asn.serialize_map("asn", &mut content);

    content.insert("status".into(), e.status.serialize_top());
    content.insert("status_classes".into(), e.status_classes.serialize_top());
    content.insert("methods".into(), e.methods.serialize_top());

    e.top_tags.serialize(&mut content, "top_tags_");
    content.insert("top_request_per_cookies".into(), e.cookies_amount.serialize_top());
    content.insert("top_request_per_args".into(), e.args_amount.serialize_top());
    content.insert("top_request_per_headers".into(), e.headers_amount.serialize_top());
    content.insert("top_max_cookies_per_request".into(), e.cookies_amount.serialize_max());
    content.insert("top_max_args_per_request".into(), e.args_amount.serialize_max());
    content.insert("top_max_headers_per_request".into(), e.headers_amount.serialize_max());

    content.insert(
        "top_ip_per_unique_uri".into(),
        serde_json::to_value(&e.ip_per_uri).unwrap_or(Value::Null),
    );
    content.insert(
        "top_uri_per_unique_ip".into(),
        serde_json::to_value(&e.uri_per_ip).unwrap_or(Value::Null),
    );
    content.insert(
        "top_session_per_unique_uri".into(),
        serde_json::to_value(&e.session_per_uri).unwrap_or(Value::Null),
    );
    content.insert(
        "top_uri_per_unique_session".into(),
        serde_json::to_value(&e.uri_per_session).unwrap_or(Value::Null),
    );

    Value::Object(content)
}

fn serialize_entry(sample: i64, hdr: &AggregationKey, counters: &AggregatedCounters) -> Value {
    let naive_dt =
        chrono::NaiveDateTime::from_timestamp_opt(sample * *SAMPLE_DURATION, 0).unwrap_or(chrono::NaiveDateTime::MIN);
    let timestamp: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_utc(naive_dt, chrono::Utc);
    let mut content = serde_json::Map::new();

    content.insert(
        "timestamp".into(),
        serde_json::to_value(timestamp).unwrap_or_else(|_| Value::String("??".into())),
    );
    content.insert(
        "proxy".into(),
        hdr.proxy
            .as_ref()
            .map(|s| Value::String(s.clone()))
            .unwrap_or(Value::Null),
    );
    content.insert("secpolid".into(), Value::String(hdr.secpolid.clone()));
    content.insert("secpolentryid".into(), Value::String(hdr.secpolentryid.clone()));
    content.insert("branch".into(), Value::String(hdr.branch.clone()));
    content.insert("planet_name".into(), Value::String(PLANET_NAME.clone()));
    content.insert("counters".into(), serialize_counters(counters));
    Value::Object(content)
}

fn prune_old_values<A>(amp: &mut HashMap<AggregationKey, BTreeMap<i64, A>>, cursample: i64) {
    for (_, mp) in amp.iter_mut() {
        #[allow(clippy::needless_collect)]
        let keys: Vec<i64> = mp.keys().copied().collect();
        for k in keys.into_iter() {
            if k <= cursample - *SAMPLES_KEPT {
                mp.remove(&k);
            }
        }
    }
}

/// displays the Nth samples of aggregated data
pub async fn aggregated_values() -> String {
    let mut guard = AGGREGATED.lock().await;
    let timestamp = chrono::Utc::now().timestamp();
    let cursample = timestamp / *SAMPLE_DURATION;
    // first, prune excess data
    prune_old_values(&mut guard, cursample);
    let timerange = || 1 + cursample - *SAMPLES_KEPT..=cursample;

    let entries: Vec<Value> = guard
        .iter()
        .flat_map(|(hdr, v)| {
            let range = if !v.is_empty() {
                timerange().collect()
            } else {
                Vec::new()
            };
            range
                .into_iter()
                .map(move |secs| serialize_entry(secs, hdr, v.get(&secs).unwrap_or(&EMPTY_AGGREGATED_DATA)))
        })
        .collect();
    let entries = if entries.is_empty() {
        let proxy = crate::config::CONFIGS
            .config
            .read()
            .ok()
            .and_then(|cfg| cfg.container_name.clone());

        timerange()
            .map(|ts| {
                serialize_entry(
                    ts,
                    &AggregationKey {
                        proxy: proxy.clone(),
                        secpolid: "__default__".to_string(),
                        secpolentryid: "__default__".to_string(),
                        branch: "-".to_string(),
                    },
                    &AggregatedCounters::default(),
                )
            })
            .collect()
    } else {
        entries
    };

    serde_json::to_string(&entries).unwrap_or_else(|_| "[]".into())
}

/// non asynchronous version of aggregated_values
pub fn aggregated_values_block() -> String {
    async_std::task::block_on(aggregated_values())
}

/// adds new data to the aggregator
pub async fn aggregate(
    dec: &Decision,
    rcode: Option<u32>,
    rinfo: &MaskedRequestInfo,
    tags: &Tags,
    bytes_sent: Option<usize>,
) {
    let seconds = rinfo.timestamp.timestamp();
    let sample = seconds / *SAMPLE_DURATION;
    let branch_tag = tags
        .inner()
        .keys()
        .filter_map(|t| t.strip_prefix("branch:"))
        .next()
        .unwrap_or("-");
    let key = AggregationKey {
        proxy: rinfo.rinfo.container_name.clone(),
        secpolid: rinfo.rinfo.secpolicy.policy.id.to_string(),
        secpolentryid: rinfo.rinfo.secpolicy.entry.id.to_string(),
        branch: branch_tag.to_string(),
    };
    let mut guard = AGGREGATED.lock().await;
    prune_old_values(&mut guard, sample);
    let entry_hdrs = guard.entry(key).or_default();
    let entry = entry_hdrs.entry(sample).or_default();
    entry.increment(dec, rcode, rinfo, tags, bytes_sent);
}
