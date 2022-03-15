use crate::config::contentfilter::Transformation;
use crate::maxmind::get_country;
use itertools::Itertools;
use maxminddb::geoip2::model;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::net::IpAddr;

pub mod decoders;

use crate::body::parse_body;
use crate::config::utils::{DataSource, RequestSelector, RequestSelectorCondition, XDataSource};
use crate::interface::{Decision, Tags};
use crate::logs::Logs;
use crate::maxmind::{get_asn, get_city};
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
        cookies.add(k, DataSource::X(XDataSource::CookieHeader), v);
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
            headers.add(lk, DataSource::Root, v.clone());
        }
    }

    (headers, cookies)
}

/// parses query parameters, such as
fn parse_query_params(dec: &[Transformation], query: &str) -> RequestField {
    let mut rf = RequestField::new(dec);
    parse_urlencoded_params(&mut rf, query);
    rf
}

/// parses the request uri, storing the path and query parts (if possible)
/// returns the hashmap of arguments
fn map_args(
    logs: &mut Logs,
    dec: &[Transformation],
    path: &str,
    mcontent_type: Option<&str>,
    mbody: Option<&[u8]>,
) -> QueryInfo {
    // this is necessary to do this in this convoluted way so at not to borrow attrs
    let uri = match urldecode_str(path) {
        DecodingResult::NoChange => path.to_string(),
        DecodingResult::Changed(nuri) => nuri,
    };
    let (qpath, query, mut args) = match path.splitn(2, '?').collect_tuple() {
        Some((qpath, query)) => (qpath.to_string(), query.to_string(), parse_query_params(dec, query)),
        None => (path.to_string(), String::new(), RequestField::new(dec)),
    };

    if let Some(body) = mbody {
        if let Err(rr) = parse_body(logs, &mut args, mcontent_type, body) {
            // if the body could not be parsed, store it in an argument, as if it was text
            logs.error(rr);
            args.add(
                "RAW_BODY".to_string(),
                DataSource::Root,
                String::from_utf8_lossy(body).to_string(),
            );
        } else {
            logs.debug("body parsed");
        }
    }
    let mut path_as_map =
        RequestField::singleton(dec, "path".to_string(), DataSource::X(XDataSource::Uri), qpath.clone());
    for (i, p) in qpath.split('/').enumerate() {
        path_as_map.add(format!("part{}", i), DataSource::X(XDataSource::Uri), p.to_string());
    }

    QueryInfo {
        qpath,
        query,
        uri,
        args,
        path_as_map,
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
}

impl GeoIp {
    fn to_json(&self) -> HashMap<&'static str, serde_json::Value> {
        let mut out = HashMap::new();
        for k in &["location", "country", "continent", "city"] {
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
        out.insert("company", json!(self.company));
        out.insert("region", json!(self.region));
        out.insert("subregion", json!(self.subregion));

        out
    }
}

#[derive(Debug, Clone)]
pub struct RequestMeta {
    pub authority: Option<String>,
    pub method: String,
    pub path: String,
    /// this field only exists for gradual Lua interop
    /// TODO: remove when complete
    pub extra: HashMap<String, String>,
}

impl RequestMeta {
    pub fn from_map(attrs: HashMap<String, String>) -> Result<Self, &'static str> {
        let mut mattrs = attrs;
        let authority = mattrs.remove("authority");
        let method = mattrs.remove("method").ok_or("missing method field")?;
        let path = mattrs.remove("path").ok_or("missing path field")?;
        Ok(RequestMeta {
            authority,
            method,
            path,
            extra: mattrs,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RInfo {
    pub meta: RequestMeta,
    pub geoip: GeoIp,
    pub qinfo: QueryInfo,
    pub host: String,
}

#[derive(Debug, Clone)]
pub struct RequestInfo {
    pub cookies: RequestField,
    pub headers: RequestField,
    pub rinfo: RInfo,
}

impl RequestInfo {
    pub fn into_json(self, tags: Tags) -> serde_json::Value {
        let ipnum: Option<String> = self.rinfo.geoip.ip.as_ref().map(|i| match i {
            IpAddr::V4(a) => u32::from_be_bytes(a.octets()).to_string(),
            IpAddr::V6(a) => u128::from_be_bytes(a.octets()).to_string(),
        });
        let geo = self.rinfo.geoip.to_json();
        let mut attrs: HashMap<String, Option<String>> = [
            ("uri", Some(self.rinfo.qinfo.uri)),
            ("path", Some(self.rinfo.qinfo.qpath)),
            ("query", Some(self.rinfo.qinfo.query)),
            ("ip", Some(self.rinfo.geoip.ipstr)),
            ("ipnum", ipnum),
            ("authority", Some(self.rinfo.host)),
            ("method", Some(self.rinfo.meta.method)),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect();
        attrs.extend(
            self.rinfo
                .meta
                .extra
                .into_iter()
                .map(|(k, v)| (k, Some(v))),
        );
        serde_json::json!({
            "headers": self.headers.to_json(),
            "cookies": self.cookies.to_json(),
            "args": self.rinfo.qinfo.args.to_json(),
            "path": self.rinfo.qinfo.path_as_map.to_json(),
            "attrs": attrs,
            "tags": tags,
            "geo": geo
        })
    }
}

#[derive(Debug)]
pub struct InspectionResult {
    pub decision: Decision,
    pub rinfo: Option<RequestInfo>,
    pub tags: Option<Tags>,
    pub err: Option<String>,
    pub logs: Logs,
}

impl InspectionResult {
    pub fn into_json(self) -> (String, Option<String>) {
        // return the request map, but only if we have it !
        let resp = match self.rinfo {
            None => self.decision.to_json_raw(serde_json::Value::Null, self.logs),
            Some(rinfo) => self.decision.to_json(rinfo, self.tags.unwrap_or_default(), self.logs),
        };
        (resp, self.err)
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
    };

    let ip = match pip {
        Ok(x) => x,
        Err(rr) => {
            logs.error(format!("When parsing ip {}", rr));
            return geoip;
        }
    };

    let get_name = |mmap: Option<&std::collections::BTreeMap<String, String>>| {
        mmap.as_ref().and_then(|mp| mp.get("en")).map(|s| s.to_lowercase())
    };

    if let Ok(asninfo) = get_asn(ip) {
        geoip.asn = asninfo.autonomous_system_number;
        geoip.company = asninfo.autonomous_system_organization;
    }

    let extract_continent = |g: &mut GeoIp, mcnt: Option<model::Continent>| {
        if let Some(continent) = mcnt {
            g.continent_code = continent.code.clone();
            g.continent_name = get_name(continent.names.as_ref());
        }
    };

    let extract_country = |g: &mut GeoIp, mcnt: Option<model::Country>| {
        if let Some(country) = mcnt {
            g.in_eu = country.is_in_european_union;
            g.country_iso = country.iso_code.as_ref().map(|s| s.to_lowercase());
            g.country_name = get_name(country.names.as_ref());
        }
    };

    // first put country data in the geoip
    if let Ok(cnty) = get_country(ip) {
        extract_continent(&mut geoip, cnty.continent);
        extract_country(&mut geoip, cnty.country);
    }

    // potentially overwrite some with the city data
    if let Ok(cty) = get_city(ip) {
        extract_continent(&mut geoip, cty.continent);
        extract_country(&mut geoip, cty.country);
        geoip.location = cty
            .location
            .as_ref()
            .and_then(|l| l.latitude.and_then(|lat| l.longitude.map(|lon| (lat, lon))));
        if let Some(subs) = cty.subdivisions {
            match &subs[..] {
                [] => (),
                [region] => geoip.region = get_name(region.names.as_ref()),
                [region, subregion] => {
                    geoip.region = region.iso_code.clone();
                    geoip.subregion = subregion.iso_code.clone();
                }
                _ => logs.error(format!("Too many subdivisions were reported for {}", ip)),
            }
        }
        geoip.city_name = cty.city.as_ref().and_then(|c| get_name(c.names.as_ref()));
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

pub fn map_request(logs: &mut Logs, dec: &[Transformation], raw: &RawRequest) -> RequestInfo {
    let host = raw.get_host();

    logs.debug("map_request starts");
    let (headers, cookies) = map_headers(dec, &raw.headers);
    logs.debug("headers mapped");
    let geoip = find_geoip(logs, raw.ipstr.clone());
    logs.debug("geoip computed");
    let qinfo = map_args(logs, dec, &raw.meta.path, headers.get_str("content-type"), raw.mbody);
    logs.debug("args mapped");

    let rinfo = RInfo {
        meta: raw.meta.clone(),
        geoip,
        qinfo,
        host,
    };

    RequestInfo {
        cookies,
        headers,
        rinfo,
    }
}

enum Selected<'a> {
    OStr(String),
    Str(&'a String),
    U32(u32),
}

/// selects data from a request
///
/// the reason we return this selected type instead of something directly string-like is
/// to avoid copies, because in the Asn case there is no way to return a reference
fn selector<'a>(reqinfo: &'a RequestInfo, sel: &RequestSelector, tags: &Tags) -> Option<Selected<'a>> {
    match sel {
        RequestSelector::Args(k) => reqinfo.rinfo.qinfo.args.get(k).map(Selected::Str),
        RequestSelector::Header(k) => reqinfo.headers.get(k).map(Selected::Str),
        RequestSelector::Cookie(k) => reqinfo.cookies.get(k).map(Selected::Str),
        RequestSelector::Ip => Some(&reqinfo.rinfo.geoip.ipstr).map(Selected::Str),
        RequestSelector::Uri => Some(&reqinfo.rinfo.qinfo.uri).map(Selected::Str),
        RequestSelector::Path => Some(&reqinfo.rinfo.qinfo.qpath).map(Selected::Str),
        RequestSelector::Query => Some(&reqinfo.rinfo.qinfo.query).map(Selected::Str),
        RequestSelector::Method => Some(&reqinfo.rinfo.meta.method).map(Selected::Str),
        RequestSelector::Country => reqinfo.rinfo.geoip.country_iso.as_ref().map(Selected::Str),
        RequestSelector::Authority => Some(Selected::Str(&reqinfo.rinfo.host)),
        RequestSelector::Company => reqinfo.rinfo.geoip.company.as_ref().map(Selected::Str),
        RequestSelector::Asn => reqinfo.rinfo.geoip.asn.map(Selected::U32),
        RequestSelector::Tags => Some(Selected::OStr(tags.selector())),
    }
}

pub fn select_string(reqinfo: &RequestInfo, sel: &RequestSelector, tags: &Tags) -> Option<String> {
    selector(reqinfo, sel, tags).map(|r| match r {
        Selected::Str(s) => (*s).clone(),
        Selected::U32(n) => format!("{}", n),
        Selected::OStr(s) => s,
    })
}

pub fn check_selector_cond(reqinfo: &RequestInfo, tags: &Tags, sel: &RequestSelectorCondition) -> bool {
    match sel {
        RequestSelectorCondition::Tag(t) => tags.contains(t),
        RequestSelectorCondition::N(sel, re) => match selector(reqinfo, sel, tags) {
            None => false,
            Some(Selected::Str(s)) => re.is_match(s),
            Some(Selected::OStr(s)) => re.is_match(&s),
            Some(Selected::U32(s)) => re.is_match(&format!("{}", s)),
        },
    }
}

pub fn masker(seed: &[u8], value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(value.as_bytes());
    let bytes = hasher.finalize();
    let hash_str = format!("{:x}", bytes);
    format!("MASKED{{{}}}", &hash_str[0..8])
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
            None,
        );

        assert_eq!(qinfo.qpath, "/a/b/%20c");
        assert_eq!(qinfo.uri, "/a/b/ c?xa =12&bbbb=12(&cccc&b64=YXJndW1lbnQ=");
        assert_eq!(qinfo.query, "xa%20=12&bbbb=12%28&cccc&b64=YXJndW1lbnQ%3D");

        let expected_args: RequestField = RequestField::from_iterator(
            &[],
            [
                ("xa ", DataSource::X(XDataSource::Uri), "12"),
                ("bbbb", DataSource::X(XDataSource::Uri), "12("),
                ("cccc", DataSource::X(XDataSource::Uri), ""),
                ("b64", DataSource::X(XDataSource::Uri), "YXJndW1lbnQ="),
                ("b64:decoded", DataSource::DecodedFrom("b64".into()), "argument"),
                ("xa :decoded", DataSource::DecodedFrom("xa ".into()), "�"),
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
        let qinfo = map_args(&mut logs, &[], "/a/b", None, None);

        assert_eq!(qinfo.qpath, "/a/b");
        assert_eq!(qinfo.uri, "/a/b");
        assert_eq!(qinfo.query, "");

        assert_eq!(qinfo.args, RequestField::new(&[]));
    }
}
