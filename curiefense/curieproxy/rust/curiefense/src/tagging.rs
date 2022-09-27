use crate::config::globalfilter::{
    GlobalFilterEntry, GlobalFilterEntryE, GlobalFilterRule, GlobalFilterSection, PairEntry, SingleEntry,
};
use crate::config::raw::Relation;
use crate::interface::stats::{BStageMapped, BStageSecpol, StatsCollect};
use crate::interface::{stronger_decision, BlockReason, Location, SimpleDecision, Tags};
use crate::requestfields::RequestField;
use crate::utils::RequestInfo;
use std::collections::HashSet;
use std::net::IpAddr;

struct MatchResult {
    matched: HashSet<Location>,
    matching: bool,
}

fn check_rule(rinfo: &RequestInfo, tags: &Tags, rel: &GlobalFilterRule) -> MatchResult {
    match rel {
        GlobalFilterRule::Rel(rl) => match rl.relation {
            Relation::And => {
                let mut matched = HashSet::new();
                for sub in &rl.entries {
                    let res = check_rule(rinfo, tags, sub);
                    if !res.matching {
                        return MatchResult {
                            matched: HashSet::new(),
                            matching: false,
                        };
                    }
                    matched.extend(res.matched);
                }
                MatchResult {
                    matched,
                    matching: true,
                }
            }
            Relation::Or => {
                for sub in &rl.entries {
                    let res = check_rule(rinfo, tags, sub);
                    if res.matching {
                        return res;
                    }
                }
                MatchResult {
                    matched: HashSet::new(),
                    matching: false,
                }
            }
        },
        GlobalFilterRule::Entry(e) => check_entry(rinfo, tags, e),
    }
}

fn check_pair<F>(pr: &PairEntry, s: &RequestField, locf: F) -> Option<HashSet<Location>>
where
    F: Fn(&str) -> Location,
{
    s.get(&pr.key).and_then(|v| {
        if &pr.exact == v || pr.re.as_ref().map(|re| re.is_match(v)).unwrap_or(false) {
            Some(std::iter::once(locf(v)).collect())
        } else {
            None
        }
    })
}

fn check_single(pr: &SingleEntry, s: &str, loc: Location) -> Option<HashSet<Location>> {
    if pr.exact == s || pr.re.as_ref().map(|re| re.is_match(s)).unwrap_or(false) {
        Some(std::iter::once(loc).collect())
    } else {
        None
    }
}

fn check_entry(rinfo: &RequestInfo, tags: &Tags, sub: &GlobalFilterEntry) -> MatchResult {
    fn bool(loc: Location, b: bool) -> Option<HashSet<Location>> {
        if b {
            Some(std::iter::once(loc).collect())
        } else {
            None
        }
    }
    fn mbool(loc: Location, mb: Option<bool>) -> Option<HashSet<Location>> {
        bool(loc, mb.unwrap_or(false))
    }
    let r = match &sub.entry {
        GlobalFilterEntryE::Always(false) => None,
        GlobalFilterEntryE::Always(true) => Some(std::iter::once(Location::Request).collect()),
        GlobalFilterEntryE::Ip(addr) => mbool(Location::Ip, rinfo.rinfo.geoip.ip.map(|i| &i == addr)),
        GlobalFilterEntryE::Network(net) => mbool(Location::Ip, rinfo.rinfo.geoip.ip.map(|i| net.contains(&i))),
        GlobalFilterEntryE::Range4(net4) => bool(
            Location::Ip,
            match rinfo.rinfo.geoip.ip {
                Some(IpAddr::V4(ip4)) => net4.contains(&ip4),
                _ => false,
            },
        ),
        GlobalFilterEntryE::Range6(net6) => bool(
            Location::Ip,
            match rinfo.rinfo.geoip.ip {
                Some(IpAddr::V6(ip6)) => net6.contains(&ip6),
                _ => false,
            },
        ),
        GlobalFilterEntryE::Path(pth) => check_single(pth, &rinfo.rinfo.qinfo.qpath, Location::Path),
        GlobalFilterEntryE::Query(qry) => check_single(qry, &rinfo.rinfo.qinfo.query, Location::Path),
        GlobalFilterEntryE::Uri(uri) => check_single(uri, &rinfo.rinfo.qinfo.uri, Location::Uri),
        GlobalFilterEntryE::Country(cty) => rinfo
            .rinfo
            .geoip
            .country_iso
            .as_ref()
            .and_then(|ccty| check_single(cty, ccty.to_lowercase().as_ref(), Location::Ip)),
        GlobalFilterEntryE::Region(cty) => rinfo
            .rinfo
            .geoip
            .region
            .as_ref()
            .and_then(|ccty| check_single(cty, ccty.to_lowercase().as_ref(), Location::Ip)),
        GlobalFilterEntryE::SubRegion(cty) => rinfo
            .rinfo
            .geoip
            .subregion
            .as_ref()
            .and_then(|ccty| check_single(cty, ccty.to_lowercase().as_ref(), Location::Ip)),
        GlobalFilterEntryE::Method(mtd) => check_single(mtd, &rinfo.rinfo.meta.method, Location::Request),
        GlobalFilterEntryE::Header(hdr) => check_pair(hdr, &rinfo.headers, |h| {
            Location::HeaderValue(hdr.key.clone(), h.to_string())
        }),
        GlobalFilterEntryE::Args(arg) => check_pair(arg, &rinfo.rinfo.qinfo.args, |a| {
            Location::UriArgumentValue(arg.key.clone(), a.to_string())
        }),
        GlobalFilterEntryE::Cookies(arg) => check_pair(arg, &rinfo.cookies, |c| {
            Location::CookieValue(arg.key.clone(), c.to_string())
        }),
        GlobalFilterEntryE::Asn(asn) => mbool(Location::Ip, rinfo.rinfo.geoip.asn.map(|casn| casn == *asn)),
        GlobalFilterEntryE::Company(cmp) => rinfo
            .rinfo
            .geoip
            .company
            .as_ref()
            .and_then(|ccmp| check_single(cmp, ccmp.as_str(), Location::Ip)),
        GlobalFilterEntryE::Authority(at) => check_single(at, &rinfo.rinfo.host, Location::Request),
        GlobalFilterEntryE::Tag(tg) => tags.get(&tg.exact).cloned(),
        GlobalFilterEntryE::SecurityPolicyId(id) => {
            if &rinfo.rinfo.policyid == id {
                Some(std::iter::once(Location::Request).collect())
            } else {
                None
            }
        }
        GlobalFilterEntryE::SecurityPolicyEntryId(id) => {
            if &rinfo.rinfo.entryid == id {
                Some(std::iter::once(Location::Request).collect())
            } else {
                None
            }
        }
    };
    match r {
        Some(matched) => MatchResult {
            matched,
            matching: !sub.negated,
        },
        None => MatchResult {
            matched: HashSet::new(),
            matching: sub.negated,
        },
    }
}

pub fn tag_request(
    stats: StatsCollect<BStageSecpol>,
    is_human: bool,
    globalfilters: &[GlobalFilterSection],
    rinfo: &RequestInfo,
) -> (Tags, SimpleDecision, StatsCollect<BStageMapped>) {
    let mut tags = Tags::default();
    if is_human {
        tags.insert("human", Location::Request);
    } else {
        tags.insert("bot", Location::Request);
    }
    tags.insert_qualified("headers", &rinfo.headers.len().to_string(), Location::Headers);
    tags.insert_qualified("cookies", &rinfo.cookies.len().to_string(), Location::Cookies);
    tags.insert_qualified("args", &rinfo.rinfo.qinfo.args.len().to_string(), Location::Request);
    tags.insert_qualified("host", &rinfo.rinfo.host, Location::Request);
    tags.insert_qualified("ip", &rinfo.rinfo.geoip.ipstr, Location::Ip);
    tags.insert_qualified(
        "geo-continent-name",
        rinfo.rinfo.geoip.continent_name.as_deref().unwrap_or("nil"),
        Location::Ip,
    );
    tags.insert_qualified(
        "geo-continent-code",
        rinfo.rinfo.geoip.continent_code.as_deref().unwrap_or("nil"),
        Location::Ip,
    );
    tags.insert_qualified(
        "geo-city",
        rinfo.rinfo.geoip.city_name.as_deref().unwrap_or("nil"),
        Location::Ip,
    );
    tags.insert_qualified(
        "geo-org",
        rinfo.rinfo.geoip.company.as_deref().unwrap_or("nil"),
        Location::Ip,
    );
    tags.insert_qualified(
        "geo-country",
        rinfo.rinfo.geoip.country_name.as_deref().unwrap_or("nil"),
        Location::Ip,
    );
    tags.insert_qualified(
        "geo-region",
        rinfo.rinfo.geoip.region.as_deref().unwrap_or("nil"),
        Location::Ip,
    );
    tags.insert_qualified(
        "geo-subregion",
        rinfo.rinfo.geoip.subregion.as_deref().unwrap_or("nil"),
        Location::Ip,
    );
    match rinfo.rinfo.geoip.asn {
        None => {
            tags.insert_qualified("geo-asn", "nil", Location::Ip);
        }
        Some(asn) => {
            let sasn = asn.to_string();
            tags.insert_qualified("geo-asn", &sasn, Location::Ip);
        }
    }
    let mut matched = 0;
    let mut decision = SimpleDecision::Pass;
    for psection in globalfilters {
        let mtch = check_rule(rinfo, &tags, &psection.rule);
        if mtch.matching {
            matched += 1;
            let rtags = psection.tags.clone().with_locs(&mtch.matched);
            tags.extend(rtags);
            if let Some(a) = &psection.action {
                let curdec = SimpleDecision::Action(
                    a.clone(),
                    vec![BlockReason::global_filter(
                        psection.id.clone(),
                        psection.name.clone(),
                        a.atype.to_bdecision(),
                    )],
                );
                decision = stronger_decision(decision, curdec);
            }
        }
    }
    (tags, decision, stats.mapped(globalfilters.len(), matched))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::globalfilter::optimize_ipranges;
    use crate::config::globalfilter::GlobalFilterRelation;
    use crate::logs::Logs;
    use crate::utils::map_request;
    use crate::utils::RawRequest;
    use crate::utils::RequestMeta;
    use regex::Regex;
    use std::collections::HashMap;

    fn mk_rinfo() -> RequestInfo {
        let raw_headers = [
            ("content-type", "/sson"),
            ("x-forwarded-for", "52.78.12.56"),
            (":method", "GET"),
            (":authority", "localhost:30081"),
            (":path", "/adminl%20e?lol=boo&bar=bze&%20encoded=%20%20%20"),
            ("x-forwarded-proto", "http"),
            ("x-request-id", "af36dcec-524d-4d21-b90e-22d5798a6300"),
            ("accept", "*/*"),
            ("user-agent", "curl/7.58.0"),
            ("x-envoy-internal", "true"),
        ];
        let mut headers = HashMap::<String, String>::new();
        let mut attrs = HashMap::<String, String>::new();

        for (k, v) in raw_headers.iter() {
            match k.strip_prefix(':') {
                None => {
                    headers.insert(k.to_string(), v.to_string());
                }
                Some(ak) => {
                    attrs.insert(ak.to_string(), v.to_string());
                }
            }
        }
        let meta = RequestMeta::from_map(attrs).unwrap();
        let mut logs = Logs::default();
        map_request(
            &mut logs,
            "a",
            "b",
            &[],
            b"CHANGEME",
            &[],
            &[],
            false,
            500,
            false,
            &RawRequest {
                ipstr: "52.78.12.56".to_string(),
                headers,
                meta,
                mbody: None,
            },
        )
    }

    fn t_check_entry(negated: bool, entry: GlobalFilterEntryE) -> MatchResult {
        check_entry(&mk_rinfo(), &Tags::default(), &GlobalFilterEntry { negated, entry })
    }

    fn single_re(input: &str) -> SingleEntry {
        SingleEntry {
            exact: input.to_string(),
            re: Regex::new(input).ok(),
        }
    }

    fn double_re(key: &str, input: &str) -> PairEntry {
        PairEntry {
            key: key.to_string(),
            exact: input.to_string(),
            re: Regex::new(input).ok(),
        }
    }

    #[test]
    fn check_entry_ip_in() {
        let r = t_check_entry(false, GlobalFilterEntryE::Ip("52.78.12.56".parse().unwrap()));
        assert!(r.matching);
    }
    #[test]
    fn check_entry_ip_in_neg() {
        let r = t_check_entry(true, GlobalFilterEntryE::Ip("52.78.12.56".parse().unwrap()));
        assert!(!r.matching);
    }
    #[test]
    fn check_entry_ip_out() {
        let r = t_check_entry(false, GlobalFilterEntryE::Ip("52.78.12.57".parse().unwrap()));
        assert!(!r.matching);
    }

    #[test]
    fn check_path_in() {
        let r = t_check_entry(false, GlobalFilterEntryE::Path(single_re(".*adminl%20e.*")));
        assert!(r.matching);
    }

    #[test]
    fn check_path_in_not_partial_match() {
        let r = t_check_entry(false, GlobalFilterEntryE::Path(single_re("adminl%20e")));
        assert!(r.matching);
    }

    #[test]
    fn check_path_out() {
        let r = t_check_entry(false, GlobalFilterEntryE::Path(single_re(".*adminl e.*")));
        assert!(!r.matching);
    }

    #[test]
    fn check_headers_exact() {
        let r = t_check_entry(false, GlobalFilterEntryE::Header(double_re("accept", "*/*")));
        assert!(r.matching);
    }

    #[test]
    fn check_headers_match() {
        let r = t_check_entry(false, GlobalFilterEntryE::Header(double_re("user-agent", "^curl.*")));
        assert!(r.matching);
    }

    fn mk_globalfilterentries(lst: &[&str]) -> Vec<GlobalFilterRule> {
        lst.iter()
            .map(|e| match e.strip_prefix('!') {
                None => GlobalFilterEntry {
                    negated: false,
                    entry: GlobalFilterEntryE::Network(e.parse().unwrap()),
                },
                Some(sub) => GlobalFilterEntry {
                    negated: true,
                    entry: GlobalFilterEntryE::Network(sub.parse().unwrap()),
                },
            })
            .map(GlobalFilterRule::Entry)
            .collect()
    }

    fn optimize(ss: &GlobalFilterRule) -> GlobalFilterRule {
        match ss {
            GlobalFilterRule::Rel(rl) => {
                let mut entries = optimize_ipranges(rl.relation, rl.entries.clone());
                if entries.is_empty() {
                    GlobalFilterRule::Entry(GlobalFilterEntry {
                        negated: false,
                        entry: GlobalFilterEntryE::Always(rl.relation == Relation::And),
                    })
                } else if entries.len() == 1 {
                    entries.pop().unwrap()
                } else {
                    GlobalFilterRule::Rel(GlobalFilterRelation {
                        relation: rl.relation,
                        entries,
                    })
                }
            }
            GlobalFilterRule::Entry(e) => GlobalFilterRule::Entry(e.clone()),
        }
        /*
        GlobalFilterSection {
            relation: ss.relation,
            entries: optimize_ipranges(ss.relation, ss.entries.clone()),
        }
        */
    }

    fn check_iprange(rel: Relation, input: &[&str], samples: &[(&str, bool)]) {
        let entries = mk_globalfilterentries(input);
        let ssection = GlobalFilterRule::Rel(GlobalFilterRelation { entries, relation: rel });
        let optimized = optimize(&ssection);
        let tags = Tags::default();

        let mut ri = mk_rinfo();
        for (ip, expected) in samples {
            ri.rinfo.geoip.ip = Some(ip.parse().unwrap());
            assert_eq!(check_rule(&ri, &tags, &ssection).matching, *expected);
            assert_eq!(check_rule(&ri, &tags, &optimized).matching, *expected);
        }
    }

    #[test]
    fn ipranges_simple() {
        let entries = ["192.168.1.0/24"];
        let samples = [
            ("10.0.4.1", false),
            ("192.168.0.23", false),
            ("192.168.1.23", true),
            ("192.170.2.45", false),
        ];
        check_iprange(Relation::And, &entries, &samples);
    }

    #[test]
    fn ipranges_intersected() {
        let entries = ["192.168.0.0/23", "192.168.1.0/24"];
        let samples = [
            ("10.0.4.1", false),
            ("192.168.0.23", false),
            ("192.168.1.23", true),
            ("192.170.2.45", false),
        ];
        check_iprange(Relation::And, &entries, &samples);
    }

    #[test]
    fn ipranges_simple_substraction() {
        let entries = ["192.168.0.0/23", "!192.168.1.0/24"];
        let samples = [
            ("10.0.4.1", false),
            ("192.168.0.23", true),
            ("192.168.1.23", false),
            ("192.170.2.45", false),
        ];
        check_iprange(Relation::And, &entries, &samples);
    }

    #[test]
    fn ipranges_simple_union() {
        let entries = ["192.168.0.0/24", "192.168.1.0/24"];
        let samples = [
            ("10.0.4.1", false),
            ("192.168.0.23", true),
            ("192.168.1.23", true),
            ("192.170.2.45", false),
        ];
        check_iprange(Relation::Or, &entries, &samples);
    }

    #[test]
    fn ipranges_larger_union() {
        let entries = ["192.168.0.0/24", "192.168.2.0/24", "10.1.0.0/16", "10.4.0.0/16"];
        let samples = [
            ("10.4.4.1", true),
            ("10.2.2.1", false),
            ("192.168.0.23", true),
            ("192.168.1.23", false),
            ("192.170.2.45", false),
        ];
        check_iprange(Relation::Or, &entries, &samples);
    }

    #[test]
    fn optimization_works() {
        let entries = mk_globalfilterentries(&["127.0.0.1/8", "192.168.0.1/24"]);
        let ssection = GlobalFilterRule::Rel(GlobalFilterRelation {
            entries,
            relation: Relation::Or,
        });
        let optimized = optimize(&ssection);
        match optimized {
            GlobalFilterRule::Rel(r) => panic!("expected a single entry, but got {:?}", r),
            GlobalFilterRule::Entry(_) => (),
        }
    }
}
