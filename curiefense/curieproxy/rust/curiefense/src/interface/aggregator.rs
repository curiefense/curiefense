use async_std::sync::Mutex;
use chrono::Utc;
use lazy_static::lazy_static;
use pdatastructs::hyperloglog::HyperLogLog;
use serde::Serialize;
use serde_json::Value;
use std::collections::{btree_map::Entry, BTreeMap, HashMap};

use crate::utils::RequestInfo;

use super::{BDecision, Decision, Location, Tags};

lazy_static! {
    static ref AGGREGATED: Mutex<HashMap<AggregationKey, BTreeMap<i64, AggregatedCounters>>> =
        Mutex::new(HashMap::new());
    static ref SECONDS_KEPT: i64 = std::env::var("AGGREGATED_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    static ref TOP_AMOUNT: usize = std::env::var("AGGREGATED_TOP")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(25);
    static ref HYPERLOGLOG_SIZE: usize = std::env::var("AGGREGATED_HLL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);
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
    requests_triggered_acl_active: usize,
    requests_triggered_acl_report: usize,
    requests_triggered_ratelimit_active: usize,
    requests_triggered_ratelimit_report: usize,

    aclid: Arp<TopN<String>>,
    secpolid: Arp<TopN<String>>,
    secpolentryid: Arp<TopN<String>>,
    cfid: Arp<TopN<String>>,

    location: Arp<AggSection>,
    ruleid: Arp<TopN<String>>,
    risk_level: Arp<Bag<u8>>,
    top_tags: Arp<TopN<String>>,
    top_country_human: TopN<String>,
    top_country_bot: TopN<String>,

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
    asn: Metric<u32>,
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
            format!("unique_active_{}", tp),
            Value::Number(serde_json::Number::from(self.unique_b.get(ArpCursor::Active).count())),
        );
        mp.insert(
            format!("unique_reported_{}", tp),
            Value::Number(serde_json::Number::from(self.unique_b.get(ArpCursor::Report).count())),
        );
        mp.insert(
            format!("unique_passed_{}", tp),
            Value::Number(serde_json::Number::from(self.unique_b.get(ArpCursor::Pass).count())),
        );
        mp.insert(
            format!("top_active_{}", tp),
            serde_json::to_value(&self.top.get(ArpCursor::Active)).unwrap_or(Value::Null),
        );
        mp.insert(
            format!("top_reported_{}", tp),
            serde_json::to_value(&self.top.get(ArpCursor::Report)).unwrap_or(Value::Null),
        );
        mp.insert(
            format!("top_passed_{}", tp),
            serde_json::to_value(&self.top.get(ArpCursor::Pass)).unwrap_or(Value::Null),
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
}

impl AggregatedCounters {
    fn increment(
        &mut self,
        dec: &Decision,
        rcode: Option<u32>,
        rinfo: &RequestInfo,
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
            let this_blocked = match r.decision {
                BDecision::Skip => {
                    skipped = true;
                    false
                }
                BDecision::Monitor => false,
                BDecision::AlterRequest => false,
                BDecision::InitiatorInactive => false,
                BDecision::Blocking => {
                    blocked = true;
                    true
                }
            };
            match &r.initiator {
                GlobalFilter { id: _, name: _ } => {
                    if this_blocked {
                        self.requests_triggered_globalfilter_active += 1;
                    } else {
                        self.requests_triggered_globalfilter_report += 1;
                    }
                }
                Acl { tags: _, stage: _ } => {
                    if this_blocked {
                        acl_blocked = true;
                        self.requests_triggered_acl_active += 1;
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
                Limit {
                    id: _,
                    name: _,
                    threshold: _,
                } => {
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
                BodyTooDeep { actual: _, expected: _ }
                | BodyMissing
                | BodyMalformed(_)
                | Sqli(_)
                | Xss
                | Restricted
                | TooManyEntries { actual: _, expected: _ }
                | EntryTooLarge { actual: _, expected: _ } => {
                    if this_blocked {
                        self.requests_triggered_cf_active += 1;
                    } else {
                        self.requests_triggered_cf_report += 1;
                    }
                }
            }
            for loc in &r.location {
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
                    Location::Path | Location::Pathpart(_) | Location::PathpartValue(_, _) => aggloc.uri += 1,
                    Location::Header(_)
                    | Location::HeaderValue(_, _)
                    | Location::RefererPath
                    | Location::RefererPathpart(_)
                    | Location::RefererPathpartValue(_, _) => aggloc.headers += 1,
                    Location::Cookies | Location::Cookie(_) | Location::CookieValue(_, _) => aggloc.headers += 1,
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
        self.secpolid
            .get_mut(cursor)
            .inc(rinfo.rinfo.secpolicy.policy.id.to_string());
        self.secpolentryid
            .get_mut(cursor)
            .inc(rinfo.rinfo.secpolicy.entry.id.to_string());
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
                tg => {
                    if !tg.contains(':') {
                        top_tags.inc(tg.to_string())
                    }
                }
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
            self.asn.inc(asn, cursor);
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
    e.aclid.serialize(&mut content, "top_aclid_");
    e.secpolid.serialize(&mut content, "top_secpolid_");
    e.secpolentryid.serialize(&mut content, "top_secpolentryid_");
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

fn serialize_entry(secs: i64, hdr: &AggregationKey, counters: &AggregatedCounters) -> Value {
    let naive_dt = chrono::NaiveDateTime::from_timestamp(secs, 0);
    let timestamp: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_utc(naive_dt, chrono::Utc);
    let mut content = serde_json::Map::new();

    content.insert(
        "timestamp".into(),
        serde_json::to_value(&timestamp).unwrap_or_else(|_| Value::String("??".into())),
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
    content.insert("counters".into(), serialize_counters(counters));
    Value::Object(content)
}

fn prune_old_values<A>(amp: &mut HashMap<AggregationKey, BTreeMap<i64, A>>, curtime: i64) {
    for (_, mp) in amp.iter_mut() {
        let keys: Vec<i64> = mp.keys().copied().collect();
        for k in keys.iter() {
            if *k <= curtime - *SECONDS_KEPT {
                mp.remove(k);
            }
        }
    }
}

/// displays the Nth last seconds of aggregated data
pub async fn aggregated_values() -> String {
    let mut guard = AGGREGATED.lock().await;
    let timestamp = chrono::Utc::now().timestamp();
    // first, prune excess data
    prune_old_values(&mut guard, timestamp);
    let timerange = || 1 + timestamp - *SECONDS_KEPT..=timestamp;

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
        let proxy = crate::config::CONFIG
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
    rinfo: &RequestInfo,
    tags: &Tags,
    bytes_sent: Option<usize>,
) {
    let seconds = rinfo.timestamp.timestamp();
    let key = AggregationKey {
        proxy: rinfo.rinfo.container_name.clone(),
        secpolid: rinfo.rinfo.secpolicy.policy.id.to_string(),
        secpolentryid: rinfo.rinfo.secpolicy.entry.id.to_string(),
    };
    let mut guard = AGGREGATED.lock().await;
    prune_old_values(&mut guard, seconds);
    let entry_hdrs = guard.entry(key).or_default();
    let entry = entry_hdrs.entry(seconds).or_default();
    entry.increment(dec, rcode, rinfo, tags, bytes_sent);
}
