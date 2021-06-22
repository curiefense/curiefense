use curiefense::config::hostmap::*;
use curiefense::config::raw::AclProfile;
use curiefense::config::utils::Matching;
use curiefense::config::waf::WafProfile;
use curiefense::config::Config;
use curiefense::logs::Logs;
use curiefense::requestfields::RequestField;
use curiefense::urlmap::match_urlmap;
use curiefense::utils::{GeoIp, QueryInfo, RInfo, RequestInfo, RequestMeta};

use criterion::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn gen_bogus_config(sz: usize) -> Config {
    let mut def = Config::empty();
    def.urlmaps = (0..sz)
        .map(|i| Matching {
            matcher: Regex::new(&format!("^dummyhost_{}$", i)).unwrap(),
            inner: HostMap {
                id: format!("abcd{}", i),
                name: format!("Dummy hostmap {}", i),
                entries: Vec::new(),
                default: None,
            },
        })
        .collect();

    let acl_profile = AclProfile {
        id: "dummy".into(),
        name: "dummy".into(),
        allow: HashSet::new(),
        allow_bot: HashSet::new(),
        deny: HashSet::new(),
        deny_bot: HashSet::new(),
        bypass: HashSet::new(),
        force_deny: HashSet::new(),
    };

    let dummy_entries: Vec<Matching<UrlMap>> = (0..sz)
        .map(|i| Matching {
            matcher: Regex::new(&format!("/dummy/url/{}", i)).unwrap(),
            inner: UrlMap {
                name: format!("Dummy urlmap {}", i),
                acl_active: false,
                acl_profile: acl_profile.clone(),
                waf_active: false,
                waf_profile: WafProfile::default(),
                limits: Vec::new(),
            },
        })
        .collect();

    def.default = Some(HostMap {
        id: "__default__".into(),
        name: "__default__".into(),
        entries: dummy_entries,
        default: Some(UrlMap {
            name: "selected".into(),
            acl_active: false,
            acl_profile,
            waf_active: false,
            waf_profile: WafProfile::default(),
            limits: Vec::new(),
        }),
    });

    def
}

fn gen_rinfo() -> RequestInfo {
    RequestInfo {
        cookies: RequestField::default(),
        headers: RequestField::default(),
        rinfo: RInfo {
            meta: RequestMeta {
                authority: Some("my.host.name".into()),
                method: "GET".into(),
                extra: HashMap::new(),
                path: "/non/matching/path".into(),
            },
            host: "my.host.name".into(),
            geoip: GeoIp {
                ipstr: "1.2.3.4".into(),
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
            },
            qinfo: QueryInfo {
                qpath: "/non/matching/path".into(),
                query: String::new(),
                uri: None,
                args: RequestField::default(),
            },
        },
    }
}

fn forms_string_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("Url map search");
    let rinfo = gen_rinfo();
    for sz in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(sz), sz, |b, &size| {
            let cfg = gen_bogus_config(size);
            b.iter(|| {
                let mut logs = Logs::default();
                let (_, umap) = match_urlmap(black_box(&rinfo), black_box(&cfg), &mut logs).unwrap();
                assert_eq!(umap.name, "selected");
            })
        });
    }
}

criterion_group!(benches, forms_string_map);
criterion_main!(benches);
