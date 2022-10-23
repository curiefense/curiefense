use chrono::{DateTime, Utc};
use ipnet::IpNet;
use itertools::Itertools;
use maxminddb::geoip2::country;
use serde_json::json;
use sha2::{Digest, Sha224};
use std::collections::HashMap;
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
use crate::interface::stats::Stats;
use crate::interface::{AnalyzeResult, Decision, Location, Tags};
use crate::logs::Logs;
use crate::maxmind::{get_asn, get_city, get_country};
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
pub enum BodyDecodingResult {
    NoBody,
    ProperlyDecoded,
    DecodingFailed(String),
}

fn parse_uri(
    args: &mut RequestField,
    path_as_map: &mut RequestField,
    path: &str,
    mode: ParseUriMode,
) -> (String, String) {
    let prefix = mode.prefix();
    let (qpath, query) = match path.splitn(2, '?').collect_tuple() {
        Some((qpath, query)) => {
            parse_query_params(args, query, mode);
            (qpath.to_string(), query.to_string())
        }
        None => (path.to_string(), String::new()),
    };
    path_as_map.add(
        format!("{}path", prefix),
        match mode {
            ParseUriMode::Uri => Location::Path,
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
        if let Err(rr) = parse_body(logs, &mut args, max_depth, mcontent_type, accepted_types, body) {
            logs.debug(|| format!("Body parsing failed: {}", rr));
            // if the body could not be parsed, store it in an argument, as if it was text
            args.add(
                "RAW_BODY".to_string(),
                Location::Body,
                String::from_utf8_lossy(body).to_string(),
            );
            BodyDecodingResult::DecodingFailed(rr)
        } else {
            BodyDecodingResult::ProperlyDecoded
        }
    } else {
        BodyDecodingResult::NoBody
    };
    logs.debug("body parsed");

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
    pub query: String,
    /// URL decoded path, if decoding worked
    pub uri: String,
    pub args: RequestField,
    pub path_as_map: RequestField,
    pub body_decoding: BodyDecodingResult,
}

#[derive(Debug, Clone)]
pub struct GeoIp {
    pub ipstr: String,
    pub ip: Option<IpAddr>,
    pub location: Option<(f64, f64)>, // (lat, lon)
    pub in_eu: Option<bool>,
    pub city_name: Option<String>,
    pub country_iso: Option<String>,
    pub country_name: Option<String>,
    pub continent_name: Option<String>,
    pub continent_code: Option<String>,
    pub asn: Option<u32>,
    pub company: Option<String>,
    pub region: Option<String>,
    pub subregion: Option<String>,
    pub network: Option<String>,
    pub is_anonymous_proxy: Option<bool>,
    pub is_satellite_provider: Option<bool>,
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
        out.insert("is_anon", json!(self.is_anonymous_proxy));
        out.insert("is_sat", json!(self.is_satellite_provider));

        out
    }
}

#[derive(Debug, Clone, arbitrary::Arbitrary)]
pub struct RequestMeta {
    pub authority: Option<String>,
    pub method: String,
    pub path: String,
    pub requestid: Option<String>,
    /// this field only exists for gradual Lua interop
    /// TODO: remove when complete
    pub extra: HashMap<String, String>,
}

impl RequestMeta {
    pub fn from_map(attrs: HashMap<String, String>) -> Result<Self, &'static str> {
        let mut mattrs = attrs;
        let authority = mattrs.remove("authority");
        let requestid = mattrs.remove("x-request-id");
        let method = mattrs.remove("method").ok_or("missing method field")?;
        let path = mattrs.remove("path").ok_or("missing path field")?;
        Ok(RequestMeta {
            authority,
            method,
            path,
            extra: mattrs,
            requestid,
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
pub struct RequestInfo {
    pub timestamp: DateTime<Utc>,
    pub cookies: RequestField,
    pub headers: RequestField,
    pub rinfo: RInfo,
    pub session: String,
    pub session_ids: HashMap<String, String>,
}

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
            ("query", Some(self.rinfo.qinfo.query)),
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

#[derive(Debug, Clone)]
pub struct InspectionResult {
    pub decision: Decision,
    pub rinfo: Option<RequestInfo>,
    pub tags: Option<Tags>,
    pub err: Option<String>,
    pub logs: Logs,
    pub stats: Stats,
}

impl InspectionResult {
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
}

pub fn find_geoip(logs: &mut Logs, ipstr: String) -> GeoIp {
    let pip = ipstr.parse();
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
        asn: None,
        company: None,
        region: None,
        subregion: None,
        network: None,
        is_anonymous_proxy: None,
        is_satellite_provider: None,
    };

    let ip = match pip {
        Ok(x) => x,
        Err(rr) => {
            logs.error(|| format!("When parsing ip {}", rr));
            return geoip;
        }
    };

    let get_name = |mmap: &Option<std::collections::BTreeMap<&str, &str>>| {
        mmap.as_ref().and_then(|mp| mp.get("en")).map(|s| s.to_lowercase())
    };

    if let Ok((asninfo, _)) = get_asn(ip) {
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
    let extract_traits = |g: &mut GeoIp, mcnt: Option<country::Traits>| {
        if let Some(traits) = mcnt {
            g.is_anonymous_proxy = traits.is_anonymous_proxy;
            g.is_satellite_provider = traits.is_satellite_provider;
        }
    };

    // first put country data in the geoip
    if let Ok((cnty, network)) = get_country(ip) {
        extract_continent(&mut geoip, cnty.continent);
        extract_country(&mut geoip, cnty.country);
        extract_network(&mut geoip, network);
        extract_traits(&mut geoip, cnty.traits);
    }

    // potentially overwrite some with the city data
    if let Ok((cty, network)) = get_city(ip) {
        extract_continent(&mut geoip, cty.continent);
        extract_country(&mut geoip, cty.country);
        extract_network(&mut geoip, network);
        extract_traits(&mut geoip, cty.traits);
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

    geoip.ip = Some(ip);
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

    let dummy_reqinfo = RequestInfo {
        timestamp: ts.unwrap_or_else(Utc::now),
        cookies,
        headers,
        rinfo,
        session: String::new(),
        session_ids: HashMap::new(),
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
    let session_ids =
        std::iter::once(("sessionid".into(), session.clone()))
            .chain(secpolicy.session_ids.iter().filter_map(|s| {
                select_string(&dummy_reqinfo, s, None).map(|str| (s.to_string(), session_string(&str)))
            }))
            .collect();

    RequestInfo {
        timestamp: dummy_reqinfo.timestamp,
        cookies: dummy_reqinfo.cookies,
        headers: dummy_reqinfo.headers,
        rinfo: dummy_reqinfo.rinfo,
        session,
        session_ids,
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
        RequestSelector::Ip => Some(&reqinfo.rinfo.geoip.ipstr).map(Selected::Str),
        RequestSelector::Network => reqinfo.rinfo.geoip.network.as_ref().map(Selected::Str),
        RequestSelector::Uri => Some(&reqinfo.rinfo.qinfo.uri).map(Selected::Str),
        RequestSelector::Path => Some(&reqinfo.rinfo.qinfo.qpath).map(Selected::Str),
        RequestSelector::Query => {
            let q = &reqinfo.rinfo.qinfo.query;
            // an empty query string is considered missing
            if q.is_empty() {
                None
            } else {
                Some(Selected::Str(q))
            }
        }
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
        assert_eq!(qinfo.query, "xa%20=12&bbbb=12%28&cccc&b64=YXJndW1lbnQ%3D");

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
        assert_eq!(qinfo.query, "");

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
                extra: HashMap::new(),
            },
            mbody: None,
        };
        let mut logs = Logs::new(crate::logs::LogLevel::Debug);
        let mut secpol = SecurityPolicy::empty();
        secpol.content_filter_profile.referer_as_uri = true;
        let ri = map_request(&mut logs, Arc::new(secpol), None, &raw, None);
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
        expected_path.add("path".to_string(), Location::Path, "/this/is/the/path".to_string());
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
