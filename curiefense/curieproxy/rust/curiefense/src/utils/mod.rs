use chrono::{DateTime, Utc};
use ipnet::IpNet;
use itertools::Itertools;
use maxminddb::geoip2::country;
use serde_json::json;
use sha2::{Digest, Sha224};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::net::IpAddr;
use std::sync::Arc;

pub mod decoders;
pub mod json;
pub mod templating;
pub mod url;

use crate::body::parse_body;
use crate::config::contentfilter::Transformation;
use crate::config::hostmap::SecurityPolicy;
use crate::config::matchers::{RequestSelector, RequestSelectorCondition};
use crate::config::raw::ContentType;
use crate::config::virtualtags::VirtualTags;
use crate::contentfilter::masking;
use crate::geo::{
    get_ipinfo_asn, get_ipinfo_carrier, get_ipinfo_company, get_ipinfo_location, get_ipinfo_privacy, get_maxmind_asn,
    get_maxmind_city, get_maxmind_country, ipinfo_country_in_eu, ipinfo_resolve_continent, ipinfo_resolve_country_name,
    USE_IPINFO,
};
use crate::interface::stats::Stats;
use crate::interface::{AnalyzeResult, Decision, Location, Tags};
use crate::logs::Logs;
use crate::requestfields::RequestField;
use crate::utils::decoders::{parse_urlencoded_params, urldecode_str, DecodingResult};

pub fn cookie_map(cookies: &mut RequestField, cookie: &str) {
    // tries to split the cookie around "="
    fn to_kv(cook: &str) -> (String, String) {
        match cook.splitn(2, '=').collect_tuple() {
            Some((k, v)) => (k.to_string(), v.to_string()),
            None => (cook.to_string(), String::new()),
        }
    }
    for (k, v) in cookie.split("; ").map(to_kv) {
        let loc = Location::CookieValue(k.clone(), v.clone());
        cookies.add(k, loc, v);
    }
}

/// Parse raw headers and:
/// * lowercase the header name
/// * extract cookies
///
/// Returns (headers, cookies)
pub fn map_headers(dec: &[Transformation], rawheaders: &HashMap<String, String>) -> (RequestField, RequestField) {
    let mut cookies = RequestField::new(dec);
    let mut headers = RequestField::new(dec);
    for (k, v) in rawheaders {
        let lk = k.to_lowercase();
        if lk == "cookie" {
            cookie_map(&mut cookies, v);
        } else {
            let loc = Location::HeaderValue(lk.clone(), v.clone());
            headers.add(lk, loc, v.clone());
        }
    }

    (headers, cookies)
}

#[derive(Debug, Clone, Copy)]
enum ParseUriMode {
    Uri,
    Referer,
}

impl ParseUriMode {
    fn prefix(&self) -> &str {
        match self {
            ParseUriMode::Uri => "",
            ParseUriMode::Referer => "ref:",
        }
    }

    fn query_location(&self, k: String, v: String) -> Location {
        match self {
            ParseUriMode::Uri => Location::UriArgumentValue(k, v),
            ParseUriMode::Referer => Location::RefererArgumentValue(k, v),
        }
    }

    fn path_location(&self, p: usize, v: &str) -> Location {
        match self {
            ParseUriMode::Uri => Location::PathpartValue(p, v.to_string()),
            ParseUriMode::Referer => Location::RefererPathpartValue(p, v.to_string()),
        }
    }
}

/// parses query parameters
fn parse_query_params(rf: &mut RequestField, query: &str, mode: ParseUriMode) {
    parse_urlencoded_params(rf, query, mode.prefix(), |s1, s2| mode.query_location(s1, s2));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodyProblem {
    TooDeep,
    DecodingError(String, Option<String>),
}

impl std::fmt::Display for BodyProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyProblem::TooDeep => "too deep".fmt(f),
            BodyProblem::DecodingError(actual, expected) => match expected {
                Some(e) => write!(f, "actual:{} expected:{}", actual, e),
                None => actual.fmt(f),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodyDecodingResult {
    NoBody,
    ProperlyDecoded,
    DecodingFailed(BodyProblem),
}

fn parse_uri(
    args: &mut RequestField,
    path_as_map: &mut RequestField,
    path: &str,
    mode: ParseUriMode,
) -> (String, Option<String>) {
    let prefix = mode.prefix();
    let (qpath, query) = match path.splitn(2, '?').collect_tuple() {
        Some((qpath, query)) => {
            parse_query_params(args, query, mode);
            let nquery = "?".to_string() + query;
            (qpath.to_string(), Some(nquery))
        }
        None => (path.to_string(), None),
    };
    path_as_map.add(
        format!("{}path", prefix),
        match mode {
            ParseUriMode::Uri => Location::Uri,
            ParseUriMode::Referer => Location::Header("referer".to_string()),
        },
        qpath.clone(),
    );
    for (i, p) in qpath.split('/').enumerate() {
        if !p.is_empty() {
            path_as_map.add(format!("{}part{}", prefix, i), mode.path_location(i, p), p.to_string());
            if let DecodingResult::Changed(n) = urldecode_str(p) {
                path_as_map.add(format!("{}part{}:urldecoded", prefix, i), mode.path_location(i, p), n);
            }
        }
    }
    (qpath, query)
}

/// parses the request uri, storing the path and query parts (if possible)
/// returns the hashmap of arguments
fn map_args(
    logs: &mut Logs,
    dec: &[Transformation],
    path: &str,
    mcontent_type: Option<&str>,
    accepted_types: &[ContentType],
    mbody: Option<&[u8]>,
    max_depth: usize,
) -> QueryInfo {
    // this is necessary to do this in this convoluted way so at not to borrow attrs
    let uri = match urldecode_str(path) {
        DecodingResult::NoChange => path.to_string(),
        DecodingResult::Changed(nuri) => nuri,
    };
    let mut args = RequestField::new(dec);
    let mut path_as_map = RequestField::new(dec);
    let (qpath, query) = parse_uri(&mut args, &mut path_as_map, path, ParseUriMode::Uri);
    logs.debug("uri parsed");

    let body_decoding = if let Some(body) = mbody {
        logs.debug("body parsing start");
        if let Err(rr) = parse_body(logs, &mut args, max_depth, mcontent_type, accepted_types, body) {
            // if the body could not be parsed, store it in an argument, as if it was text
            args.add(
                "RAW_BODY".to_string(),
                Location::Body,
                String::from_utf8_lossy(body).to_string(),
            );
            logs.debug(|| format!("body parsing failed: {}", rr));
            BodyDecodingResult::DecodingFailed(rr)
        } else {
            logs.debug("body parsing succeeded");
            BodyDecodingResult::ProperlyDecoded
        }
    } else {
        logs.debug("no body to parse");
        BodyDecodingResult::NoBody
    };

    QueryInfo {
        qpath,
        query,
        uri,
        args,
        path_as_map,
        body_decoding,
    }
}

#[derive(Debug, Clone)]
/// data extracted from the query string
pub struct QueryInfo {
    /// the "path" portion of the raw query path
    pub qpath: String,
    /// the "query" portion of the raw query path
    pub query: Option<String>,
    /// URL decoded path, if decoding worked
    pub uri: String,
    pub args: RequestField,
    pub path_as_map: RequestField,
    pub body_decoding: BodyDecodingResult,
}

#[derive(Debug, Clone)]
pub struct GeoIp {
    // IP informations
    pub ipstr: String,
    pub ip: Option<IpAddr>,
    pub network: Option<String>,

    // Localisation informations
    pub location: Option<(f64, f64)>, // (lat, lon)
    pub continent_name: Option<String>,
    pub continent_code: Option<String>,
    pub country_iso: Option<String>,
    pub country_name: Option<String>,
    pub in_eu: Option<bool>,
    pub region: Option<String>,
    pub subregion: Option<String>,
    pub city_name: Option<String>,

    // Company informations
    pub company: Option<String>,
    pub company_country: Option<String>,
    pub company_domain: Option<String>,
    pub company_type: Option<String>,

    /// Autonomous System informations
    pub asn: Option<u32>,
    pub as_name: Option<String>,
    pub as_domain: Option<String>,
    pub as_type: Option<String>,

    // Mobile informations
    pub is_mobile: Option<bool>,
    pub mobile_carrier_name: Option<String>,
    pub mobile_country: Option<String>,
    pub mobile_mcc: Option<u32>,
    pub mobile_mnc: Option<u32>,

    // Privacy informations
    pub is_proxy: Option<bool>,
    pub is_satellite: Option<bool>,
    pub is_vpn: Option<bool>,
    pub is_tor: Option<bool>,
    pub is_relay: Option<bool>,
    pub is_hosting: Option<bool>,
    pub privacy_service: Option<String>,
}

impl GeoIp {
    fn to_json(&self) -> HashMap<&'static str, serde_json::Value> {
        let mut out = HashMap::new();
        for k in &["location", "country", "continent", "city", "network"] {
            out.insert(*k, json!({}));
        }

        if let Some(loc) = self.location {
            out.insert(
                "location",
                json!({
                    "lat": loc.0,
                    "lon": loc.1
                }),
            );
        }
        out.insert(
            "city",
            json!({ "name": match &self.city_name {
                None => "-",
                Some(n) => n
            } }),
        );

        out.insert("eu", json!(self.in_eu));
        out.insert(
            "country",
            json!({
                "name": self.country_name,
                "iso": self.country_iso
            }),
        );
        out.insert(
            "continent",
            json!({
                "name": self.continent_name,
                "code": self.continent_code
            }),
        );

        out.insert("asn", json!(self.asn));
        out.insert("network", json!(self.network));
        out.insert("company", json!(self.company));
        out.insert("region", json!(self.region));
        out.insert("subregion", json!(self.subregion));
        out.insert("is_anon", json!(self.is_proxy));
        out.insert("is_sat", json!(self.is_satellite));

        out
    }
}

#[derive(Debug, Clone, arbitrary::Arbitrary)]
pub struct RequestMeta {
    pub authority: Option<String>,
    pub method: String,
    pub path: String,
    pub requestid: Option<String>,
    pub protocol: Option<String>,
    /// this field only exists for gradual Lua interop
    /// TODO: remove when complete
    pub extra: HashMap<String, String>,
}

impl RequestMeta {
    pub fn from_map(attrs: HashMap<String, String>) -> Result<Self, &'static str> {
        let mut mattrs = attrs;
        let authority = mattrs.remove("authority");
        let requestid = mattrs.remove("x-request-id");
        let protocol = mattrs.remove("protocol");
        let method = mattrs.remove("method").ok_or("missing method field")?;
        let path = mattrs.remove("path").ok_or("missing path field")?;
        Ok(RequestMeta {
            authority,
            method,
            path,
            extra: mattrs,
            requestid,
            protocol,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RInfo {
    pub meta: RequestMeta,
    pub geoip: GeoIp,
    pub qinfo: QueryInfo,
    pub host: String,
    pub secpolicy: Arc<SecurityPolicy>,
    pub container_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RequestInfoG<A> {
    pub timestamp: DateTime<Utc>,
    pub cookies: RequestField,
    pub headers: RequestField,
    pub rinfo: RInfo,
    pub session: String,
    pub session_ids: HashMap<String, String>,
    pub plugins: RequestField,
    phantom: PhantomData<A>,
}

#[derive(Debug, Clone)]
pub struct Masked;
#[derive(Debug, Clone)]
pub struct Raw;

pub type RequestInfo = RequestInfoG<Raw>;
pub type MaskedRequestInfo = RequestInfoG<Masked>;

impl RequestInfo {
    pub fn into_json(self, tags: Tags) -> serde_json::Value {
        let mut v = self.into_json_notags();
        if let Some(m) = v.as_object_mut() {
            m.insert(
                "tags".to_string(),
                serde_json::to_value(tags).unwrap_or(serde_json::Value::Null),
            );
        }
        v
    }

    pub fn into_json_notags(self) -> serde_json::Value {
        let geo = self.rinfo.geoip.to_json();
        let mut attrs: HashMap<String, Option<String>> = [
            ("uri", Some(self.rinfo.qinfo.uri)),
            ("path", Some(self.rinfo.qinfo.qpath)),
            ("query", self.rinfo.qinfo.query),
            ("ip", Some(self.rinfo.geoip.ipstr)),
            ("authority", Some(self.rinfo.host)),
            ("method", Some(self.rinfo.meta.method)),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect();
        attrs.extend(self.rinfo.meta.extra.into_iter().map(|(k, v)| (k, Some(v))));
        serde_json::json!({
            "headers": self.headers,
            "cookies": self.cookies,
            "args": self.rinfo.qinfo.args,
            "path": self.rinfo.qinfo.path_as_map,
            "attributes": attrs,
            "geo": geo
        })
    }
}

impl MaskedRequestInfo {
    pub fn new(
        timestamp: DateTime<Utc>,
        cookies: RequestField,
        headers: RequestField,
        rinfo: RInfo,
        session: String,
        session_ids: HashMap<String, String>,
        plugins: RequestField,
    ) -> Self {
        Self {
            timestamp,
            cookies,
            headers,
            rinfo,
            session,
            session_ids,
            plugins,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InspectionResultG<A> {
    pub decision: Decision,
    pub rinfo: Option<RequestInfoG<A>>,
    pub tags: Option<Tags>,
    pub err: Option<String>,
    pub logs: Logs,
    pub stats: Stats,
}

pub type InspectionResult = InspectionResultG<Raw>;
pub type MaskedInspectionResult = InspectionResultG<Masked>;

impl InspectionResult {
    pub fn from_analyze(logs: Logs, dec: AnalyzeResult) -> Self {
        InspectionResult {
            decision: dec.decision,
            tags: Some(dec.tags),
            logs,
            err: None,
            rinfo: Some(dec.rinfo),
            stats: dec.stats,
        }
    }

    pub fn mask(self) -> MaskedInspectionResult {
        let (rinfo, decision) = match self.rinfo {
            Some(rinfo) => {
                let (a, b) = masking(rinfo, self.decision);
                (Some(a), b)
            }
            None => (None, self.decision),
        };

        MaskedInspectionResult {
            decision,
            rinfo,
            tags: self.tags,
            err: self.err,
            logs: self.logs,
            stats: self.stats,
        }
    }
}

impl MaskedInspectionResult {
    pub async fn log_json(&self, proxy: HashMap<String, String>) -> Vec<u8> {
        let dtags = Tags::new(&VirtualTags::default());
        let tags: &Tags = match &self.tags {
            Some(t) => t,
            None => &dtags,
        };

        match &self.rinfo {
            None => b"{}".to_vec(),
            Some(rinfo) => {
                self.decision
                    .log_json(rinfo, tags, &self.stats, &self.logs, proxy)
                    .await
            }
        }
    }

    // blocking version of log_json
    pub fn log_json_block(&self, proxy: HashMap<String, String>) -> Vec<u8> {
        async_std::task::block_on(self.log_json(proxy))
    }
}

pub fn find_geoip_maxmind(logs: &mut Logs, geoip: &mut GeoIp, ip: IpAddr) {
    let get_name = |mmap: &Option<std::collections::BTreeMap<&str, &str>>| {
        mmap.as_ref().and_then(|mp| mp.get("en")).map(|s| s.to_lowercase())
    };

    if let Ok((asninfo, _)) = get_maxmind_asn(ip) {
        geoip.asn = asninfo.autonomous_system_number;
        geoip.company = asninfo.autonomous_system_organization.map(|s| s.to_string());
    }

    let extract_continent = |g: &mut GeoIp, mcnt: Option<country::Continent>| {
        if let Some(continent) = mcnt {
            g.continent_code = continent.code.map(|s| s.to_string());
            g.continent_name = get_name(&continent.names);
        }
    };

    let extract_country = |g: &mut GeoIp, mcnt: Option<country::Country>| {
        if let Some(country) = mcnt {
            g.in_eu = country.is_in_european_union;
            g.country_iso = country.iso_code.as_ref().map(|s| s.to_lowercase());
            g.country_name = get_name(&country.names);
        }
    };

    let extract_network = |g: &mut GeoIp, network: Option<IpNet>| g.network = network.map(|n| format!("{}", n.trunc()));
    let extract_mm_traits = |g: &mut GeoIp, mcnt: Option<country::Traits>| {
        if let Some(traits) = mcnt {
            g.is_proxy = traits.is_anonymous_proxy;
            g.is_satellite = traits.is_satellite_provider;
        }
    };

    if let Ok((cnty, network)) = get_maxmind_country(ip) {
        extract_continent(geoip, cnty.continent);
        extract_country(geoip, cnty.country);
        extract_network(geoip, network);
        extract_mm_traits(geoip, cnty.traits);
    }

    if let Ok((cty, network)) = get_maxmind_city(ip) {
        extract_continent(geoip, cty.continent);
        extract_country(geoip, cty.country);
        extract_network(geoip, network);
        extract_mm_traits(geoip, cty.traits);
        geoip.location = cty
            .location
            .as_ref()
            .and_then(|l| l.latitude.and_then(|lat| l.longitude.map(|lon| (lat, lon))));
        if let Some(subs) = cty.subdivisions {
            match &subs[..] {
                [] => (),
                [region] => geoip.region = get_name(&region.names),
                [region, subregion] => {
                    geoip.region = region.iso_code.map(|s| s.to_string());
                    geoip.subregion = subregion.iso_code.map(|s| s.to_string());
                }
                _ => logs.error(|| format!("Too many subdivisions were reported for {}", ip)),
            }
        }
        geoip.city_name = cty.city.as_ref().and_then(|c| get_name(&c.names));
    }
}

// Network field priority: ASN > Carrier > Company > Location
pub fn find_geoip_ipinfo(_logs: &mut Logs, geoip: &mut GeoIp, ip: IpAddr) {
    let extract_string = |s: String| {
        if !s.is_empty() {
            Some(s)
        } else {
            None
        }
    };

    let extract_network = |g: &mut GeoIp, network: Option<IpNet>| g.network = network.map(|n| format!("{}", n.trunc()));

    if let Ok((loc, network)) = get_ipinfo_location(ip) {
        extract_network(geoip, network);
        geoip.city_name = Some(loc.city);
        geoip.country_name = ipinfo_resolve_country_name(loc.country.as_str());
        geoip.in_eu = Some(ipinfo_country_in_eu(loc.country.as_str()));
        if let Some(continent) = ipinfo_resolve_continent(loc.country.as_str()) {
            geoip.continent_code = Some(continent.code.to_string());
            geoip.continent_name = Some(continent.name.to_string());
        }
        geoip.country_iso = Some(loc.country);
        geoip.region = Some(loc.region);
        geoip.subregion = loc.postal_code; // TODO: this is not the exact same behaviour as maxmind
        if let (Ok(lat), Ok(lng)) = (loc.lat.parse(), loc.lng.parse()) {
            geoip.location = Some((lat, lng))
        };
    }

    if let Ok((privacy, _)) = get_ipinfo_privacy(ip) {
        geoip.is_vpn = privacy.vpn.parse().ok();
        geoip.is_proxy = privacy.proxy.parse().ok();
        geoip.is_tor = privacy.tor.parse().ok();
        geoip.is_relay = privacy.relay.parse().ok();
        geoip.is_hosting = privacy.hosting.parse().ok();
        geoip.privacy_service = extract_string(privacy.service)
    } else {
        geoip.is_vpn = Some(false);
        geoip.is_proxy = Some(false);
        geoip.is_tor = Some(false);
        geoip.is_relay = Some(false);
        geoip.is_hosting = Some(false);
    }

    if let Ok((company, network)) = get_ipinfo_company(ip) {
        extract_network(geoip, network);
        geoip.company = extract_string(company.name);
        geoip.company_country = extract_string(company.country);
        geoip.company_domain = extract_string(company.domain);
        geoip.company_type = extract_string(company.company_type);

        geoip.asn = company.asn.strip_prefix("AS").and_then(|asn| asn.parse().ok());
        geoip.as_name = extract_string(company.as_name);
        geoip.as_domain = extract_string(company.as_domain);
        geoip.as_type = extract_string(company.as_type);
    }

    if let Ok((carrier, _)) = get_ipinfo_carrier(ip) {
        geoip.is_mobile = Some(true);
        geoip.mobile_carrier_name = extract_string(carrier.carrier);
        geoip.mobile_country = extract_string(carrier.country_code);
        geoip.mobile_mcc = carrier.mcc.parse().ok();
        geoip.mobile_mnc = carrier.mnc.parse().ok();
        // do not re parse network using `extract_network` as it is already
        // well formatted.
        geoip.network = Some(carrier.network)
    }

    if let Ok((asn, _)) = get_ipinfo_asn(ip) {
        // TODO: always get Err here, should be fixed
        geoip.network = Some(asn.route);
        geoip.asn = asn.asn.parse().ok();
        geoip.as_name = Some(asn.name);
        geoip.as_domain = Some(asn.domain);
        geoip.as_type = Some(asn.asn_type);
    }
}

pub fn find_geoip(logs: &mut Logs, ipstr: String) -> GeoIp {
    let pip = ipstr.trim().parse();
    let mut geoip = GeoIp {
        ipstr,
        ip: None,
        location: None,
        in_eu: None,
        city_name: None,
        country_iso: None,
        country_name: None,
        continent_name: None,
        continent_code: None,
        region: None,
        subregion: None,
        network: None,
        company: None,
        company_country: None,
        company_domain: None,
        company_type: None,
        asn: None,
        as_domain: None,
        as_name: None,
        as_type: None,
        is_proxy: None,
        is_satellite: None,
        is_hosting: None,
        is_relay: None,
        is_tor: None,
        is_vpn: None,
        privacy_service: None,
        is_mobile: None,
        mobile_carrier_name: None,
        mobile_country: None,
        mobile_mcc: None,
        mobile_mnc: None,
    };

    let ip = match pip {
        Ok(x) => x,
        Err(rr) => {
            logs.error(|| format!("When parsing ip {}", rr));
            return geoip;
        }
    };

    geoip.ip = Some(ip);

    if *USE_IPINFO {
        find_geoip_ipinfo(logs, &mut geoip, ip);
    } else {
        find_geoip_maxmind(logs, &mut geoip, ip);
    }

    geoip
}

pub struct RawRequest<'a> {
    pub ipstr: String,
    pub headers: HashMap<String, String>,
    pub meta: RequestMeta,
    pub mbody: Option<&'a [u8]>,
}

impl<'a> RawRequest<'a> {
    pub fn get_host(&'a self) -> String {
        match self.meta.authority.as_ref().or_else(|| self.headers.get("host")) {
            Some(a) => a.clone(),
            None => "unknown".to_string(),
        }
    }
}

pub fn map_request(
    logs: &mut Logs,
    secpolicy: Arc<SecurityPolicy>,
    container_name: Option<String>,
    raw: &RawRequest,
    ts: Option<DateTime<Utc>>,
    plugins: HashMap<String, String>,
) -> RequestInfo {
    let host = raw.get_host();

    logs.debug("map_request starts");
    let (headers, cookies) = map_headers(&secpolicy.content_filter_profile.decoding, &raw.headers);
    logs.debug("headers mapped");
    let geoip = find_geoip(logs, raw.ipstr.clone());
    logs.debug("geoip computed");
    let mut qinfo = map_args(
        logs,
        &secpolicy.content_filter_profile.decoding,
        &raw.meta.path,
        headers.get_str("content-type"),
        &secpolicy.content_filter_profile.content_type,
        if secpolicy.content_filter_profile.ignore_body {
            None
        } else {
            raw.mbody
        },
        secpolicy.content_filter_profile.max_body_depth,
    );
    if secpolicy.content_filter_profile.referer_as_uri {
        if let Some(rf) = headers.get("referer") {
            parse_uri(
                &mut qinfo.args,
                &mut qinfo.path_as_map,
                url::drop_scheme(rf),
                ParseUriMode::Referer,
            );
        }
    }
    logs.debug("args mapped");

    let rinfo = RInfo {
        meta: raw.meta.clone(),
        geoip,
        qinfo,
        host,
        secpolicy: secpolicy.clone(),
        container_name,
    };

    let mut plugins_field = RequestField::new(&[]);
    for (k, v) in plugins {
        let l = Location::PluginValue(k.clone(), v.clone());
        plugins_field.add(k, l, v);
    }

    let dummy_reqinfo = RequestInfo {
        timestamp: ts.unwrap_or_else(Utc::now),
        cookies,
        headers,
        rinfo,
        session: String::new(),
        session_ids: HashMap::new(),
        plugins: plugins_field,
        phantom: PhantomData,
    };

    let raw_session = (if secpolicy.session.is_empty() {
        &[RequestSelector::Ip]
    } else {
        secpolicy.session.as_slice()
    })
    .iter()
    .filter_map(|s| select_string(&dummy_reqinfo, s, None))
    .next()
    .unwrap_or_else(|| "???".to_string());

    let session_string = |s: &str| {
        let mut hasher = Sha224::new();
        hasher.update(&secpolicy.content_filter_profile.masking_seed);
        hasher.update(s.as_bytes());
        let bytes = hasher.finalize();
        format!("{:x}", bytes)
    };

    let session = session_string(&raw_session);
    let session_ids = secpolicy
        .session_ids
        .iter()
        .filter_map(|s| select_string(&dummy_reqinfo, s, None).map(|str| (s.to_string(), session_string(&str))))
        .collect();

    RequestInfo {
        timestamp: dummy_reqinfo.timestamp,
        cookies: dummy_reqinfo.cookies,
        headers: dummy_reqinfo.headers,
        rinfo: dummy_reqinfo.rinfo,
        session,
        session_ids,
        plugins: dummy_reqinfo.plugins,
        phantom: PhantomData,
    }
}

pub enum Selected<'a> {
    OStr(String),    // owned
    Str(&'a String), // ref
    U32(u32),
}

/// selects data from a request
///
/// the reason we return this selected type instead of something directly string-like is
/// to avoid copies, because in the Asn case there is no way to return a reference
pub fn selector<'a>(reqinfo: &'a RequestInfo, sel: &RequestSelector, tags: Option<&Tags>) -> Option<Selected<'a>> {
    match sel {
        RequestSelector::Args(k) => reqinfo.rinfo.qinfo.args.get(k).map(Selected::Str),
        RequestSelector::Header(k) => reqinfo.headers.get(k).map(Selected::Str),
        RequestSelector::Cookie(k) => reqinfo.cookies.get(k).map(Selected::Str),
        RequestSelector::Plugins(k) => reqinfo.plugins.get(k).map(Selected::Str),
        RequestSelector::Ip => Some(&reqinfo.rinfo.geoip.ipstr).map(Selected::Str),
        RequestSelector::Network => reqinfo.rinfo.geoip.network.as_ref().map(Selected::Str),
        RequestSelector::Uri => Some(&reqinfo.rinfo.qinfo.uri).map(Selected::Str),
        RequestSelector::Path => Some(&reqinfo.rinfo.qinfo.qpath).map(Selected::Str),
        RequestSelector::Query => reqinfo.rinfo.qinfo.query.as_ref().map(Selected::Str),
        RequestSelector::Method => Some(&reqinfo.rinfo.meta.method).map(Selected::Str),
        RequestSelector::Country => reqinfo.rinfo.geoip.country_iso.as_ref().map(Selected::Str),
        RequestSelector::Authority => Some(Selected::Str(&reqinfo.rinfo.host)),
        RequestSelector::Company => reqinfo.rinfo.geoip.company.as_ref().map(Selected::Str),
        RequestSelector::Asn => reqinfo.rinfo.geoip.asn.map(Selected::U32),
        RequestSelector::Tags => tags.map(|tags| Selected::OStr(tags.selector())),
        RequestSelector::SecpolId => Some(Selected::Str(&reqinfo.rinfo.secpolicy.policy.id)),
        RequestSelector::SecpolEntryId => Some(Selected::Str(&reqinfo.rinfo.secpolicy.entry.id)),
        RequestSelector::Region => reqinfo.rinfo.geoip.region.as_ref().map(Selected::Str),
        RequestSelector::SubRegion => reqinfo.rinfo.geoip.subregion.as_ref().map(Selected::Str),
        RequestSelector::Session => Some(Selected::Str(&reqinfo.session)),
    }
}

pub fn select_string(reqinfo: &RequestInfo, sel: &RequestSelector, tags: Option<&Tags>) -> Option<String> {
    selector(reqinfo, sel, tags).map(|r| match r {
        Selected::Str(s) => (*s).clone(),
        Selected::U32(n) => format!("{}", n),
        Selected::OStr(s) => s,
    })
}

pub fn check_selector_cond(reqinfo: &RequestInfo, tags: &Tags, sel: &RequestSelectorCondition) -> bool {
    match sel {
        RequestSelectorCondition::Tag(t) => tags.contains(t),
        RequestSelectorCondition::N(sel, re) => match selector(reqinfo, sel, Some(tags)) {
            None => false,
            Some(Selected::Str(s)) => re.is_match(s),
            Some(Selected::OStr(s)) => re.is_match(&s),
            Some(Selected::U32(s)) => re.is_match(&format!("{}", s)),
        },
    }
}

pub fn masker(seed: &[u8], value: &str) -> String {
    let mut hasher = Sha224::new();
    hasher.update(seed);
    hasher.update(value.as_bytes());
    let bytes = hasher.finalize();
    let hash_str = format!("{:x}", bytes);
    format!("MASKED{{{}}}", &hash_str[0..8])
}

pub fn eat_errors<T: Default, R: std::fmt::Display>(logs: &mut Logs, rv: Result<T, R>) -> T {
    match rv {
        Err(rr) => {
            logs.error(|| rr.to_string());
            Default::default()
        }
        Ok(o) => o,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_args_full() {
        let mut logs = Logs::default();
        let qinfo = map_args(
            &mut logs,
            &[Transformation::Base64Decode],
            "/a/b/%20c?xa%20=12&bbbb=12%28&cccc&b64=YXJndW1lbnQ%3D",
            None,
            &[],
            None,
            500,
        );

        assert_eq!(qinfo.qpath, "/a/b/%20c");
        assert_eq!(qinfo.uri, "/a/b/ c?xa =12&bbbb=12(&cccc&b64=YXJndW1lbnQ=");
        assert_eq!(
            qinfo.query,
            Some("?xa%20=12&bbbb=12%28&cccc&b64=YXJndW1lbnQ%3D".to_string())
        );

        let expected_args: RequestField = RequestField::from_iterator(
            &[],
            [
                (
                    "xa ",
                    Location::UriArgumentValue("xa ".to_string(), "12".to_string()),
                    "12",
                ),
                (
                    "bbbb",
                    Location::UriArgumentValue("bbbb".to_string(), "12%28".to_string()),
                    "12(",
                ),
                (
                    "cccc",
                    Location::UriArgumentValue("cccc".to_string(), "".to_string()),
                    "",
                ),
                (
                    "b64",
                    Location::UriArgumentValue("b64".to_string(), "YXJndW1lbnQ%3D".to_string()),
                    "YXJndW1lbnQ=",
                ),
                (
                    "b64:decoded",
                    Location::UriArgumentValue("b64".to_string(), "YXJndW1lbnQ%3D".to_string()),
                    "argument",
                ),
            ]
            .iter()
            .map(|(k, ds, v)| (k.to_string(), ds.clone(), v.to_string())),
        );
        assert_eq!(qinfo.args.get("b64:decoded").map(|s| s.as_str()), Some("argument"));
        assert_eq!(qinfo.args.fields, expected_args.fields);
    }

    #[test]
    fn test_map_args_simple() {
        let mut logs = Logs::default();
        let qinfo = map_args(&mut logs, &[], "/a/b", None, &[], None, 500);

        assert_eq!(qinfo.qpath, "/a/b");
        assert_eq!(qinfo.uri, "/a/b");
        assert_eq!(qinfo.query, None);

        assert_eq!(qinfo.args, RequestField::new(&[]));
    }

    #[test]
    fn referer_a() {
        let raw = RawRequest {
            ipstr: "1.2.3.4".to_string(),
            headers: std::iter::once((
                "referer".to_string(),
                "http://another.site/with?arg1=a&arg2=b".to_string(),
            ))
            .collect(),
            meta: RequestMeta {
                authority: Some("main.site".to_string()),
                method: "GET".to_string(),
                path: "/this/is/the/path?arg1=x&arg2=y".to_string(),
                requestid: None,
                protocol: None,
                extra: HashMap::new(),
            },
            mbody: None,
        };
        let mut logs = Logs::new(crate::logs::LogLevel::Debug);
        let mut secpol = SecurityPolicy::empty();
        secpol.content_filter_profile.referer_as_uri = true;
        let ri = map_request(&mut logs, Arc::new(secpol), None, &raw, None, HashMap::new());
        let actual_args = ri.rinfo.qinfo.args;
        let actual_path = ri.rinfo.qinfo.path_as_map;
        let mut expected_args = RequestField::new(&[]);
        let mut expected_path = RequestField::new(&[]);
        let p = |k: &str, v: &str| match k.strip_prefix("ref:") {
            Some(p) => Location::RefererArgumentValue(p.to_string(), v.to_string()),
            None => Location::UriArgumentValue(k.to_string(), v.to_string()),
        };
        for (k, v) in &[("arg1", "x"), ("arg2", "y"), ("ref:arg1", "a"), ("ref:arg2", "b")] {
            expected_args.add(k.to_string(), p(k, v), v.to_string());
        }
        expected_path.add("path".to_string(), Location::Uri, "/this/is/the/path".to_string());
        for (p, v) in &[(1, "this"), (2, "is"), (3, "the"), (4, "path")] {
            expected_path.add(
                format!("part{}", p),
                Location::PathpartValue(*p, v.to_string()),
                v.to_string(),
            );
        }
        expected_path.add(
            "ref:path".to_string(),
            Location::Header("referer".to_string()),
            "/with".to_string(),
        );
        expected_path.add(
            "ref:part1".to_string(),
            Location::RefererPathpartValue(1, "with".to_string()),
            "with".to_string(),
        );
        assert_eq!(expected_args, actual_args);
        assert_eq!(expected_path, actual_path);
    }
}
