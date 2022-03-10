use curiefense::config::contentfilter::ContentFilterProfile;
use curiefense::config::hostmap::*;
use curiefense::config::raw::AclProfile;
use curiefense::config::utils::Matching;
use curiefense::config::Config;
use curiefense::logs::Logs;
use curiefense::securitypolicy::match_securitypolicy;

use criterion::*;
use regex::Regex;
use std::collections::HashSet;

fn gen_bogus_config(sz: usize) -> Config {
    let mut def = Config::empty();
    def.securitypolicies = (0..sz)
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
        passthrough: HashSet::new(),
        force_deny: HashSet::new(),
    };

    let dummy_entries: Vec<Matching<SecurityPolicy>> = (0..sz)
        .map(|i| Matching {
            matcher: Regex::new(&format!("/dummy/url/{}", i)).unwrap(),
            inner: SecurityPolicy {
                name: format!("Dummy securitypolicy {}", i),
                acl_active: false,
                acl_profile: acl_profile.clone(),
                content_filter_active: false,
                content_filter_profile: ContentFilterProfile::default_from_seed("seed"),
                limits: Vec::new(),
            },
        })
        .collect();

    def.default = Some(HostMap {
        id: "__default__".into(),
        name: "__default__".into(),
        entries: dummy_entries,
        default: Some(SecurityPolicy {
            name: "selected".into(),
            acl_active: false,
            acl_profile,
            content_filter_active: false,
            content_filter_profile: ContentFilterProfile::default_from_seed("seed"),
            limits: Vec::new(),
        }),
    });

    def
}

fn forms_string_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("Security Policy search");
    for sz in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(sz), sz, |b, &size| {
            let cfg = gen_bogus_config(size);
            b.iter(|| {
                let mut logs = Logs::default();
                let (_, umap) =
                    match_securitypolicy("my.host.name", "/non/matching/path", black_box(&cfg), &mut logs).unwrap();
                assert_eq!(umap.name, "selected");
            })
        });
    }
}

criterion_group!(benches, forms_string_map);
criterion_main!(benches);
