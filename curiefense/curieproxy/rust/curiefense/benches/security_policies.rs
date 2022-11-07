use curiefense::config::contentfilter::ContentFilterProfile;
use curiefense::config::hostmap::*;
use curiefense::config::matchers::Matching;
use curiefense::config::raw::AclProfile;
use curiefense::config::Config;
use curiefense::interface::SimpleAction;
use curiefense::logs::Logs;
use curiefense::securitypolicy::match_securitypolicy;

use criterion::*;
use std::collections::HashSet;
use std::sync::Arc;

fn gen_bogus_config(sz: usize) -> Config {
    let mut def = Config::empty();
    def.securitypolicies = (0..sz)
        .map(|i| {
            Matching::from_str(
                &format!("^dummyhost_{}$", i),
                HostMap {
                    name: format!("Dummy hostmap {}", i),
                    entries: Vec::new(),
                    default: None,
                },
            )
            .unwrap()
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
        action: SimpleAction::default(),
        tags: HashSet::new(),
    };

    let dummy_entries: Vec<Matching<Arc<SecurityPolicy>>> = (0..sz)
        .map(|i| {
            Matching::from_str(
                &format!("/dummy/url/{}", i),
                Arc::new(SecurityPolicy {
                    policy: PolicyId {
                        id: "__default__".to_string(),
                        name: "__default__".to_string(),
                    },
                    entry: PolicyId {
                        id: format!("id{}", i),
                        name: format!("Dummy securitypolicy {}", i),
                    },
                    tags: Vec::new(),
                    acl_active: false,
                    acl_profile: acl_profile.clone(),
                    content_filter_active: false,
                    content_filter_profile: ContentFilterProfile::default_from_seed("seed"),
                    session: Vec::new(),
                    session_ids: Vec::new(),
                    limits: Vec::new(),
                }),
            )
            .unwrap()
        })
        .collect();

    def.default = Some(HostMap {
        name: "__default__".into(),
        entries: dummy_entries,
        default: Some(Arc::new(SecurityPolicy {
            policy: PolicyId {
                id: "__default__".into(),
                name: "__default__".into(),
            },
            entry: PolicyId {
                id: "default".into(),
                name: "selected".into(),
            },
            tags: Vec::new(),
            acl_active: false,
            acl_profile,
            content_filter_active: false,
            content_filter_profile: ContentFilterProfile::default_from_seed("seed"),
            session: Vec::new(),
            session_ids: Vec::new(),
            limits: Vec::new(),
        })),
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
                let umap = match_securitypolicy("my.host.name", "/non/matching/path", black_box(&cfg), &mut logs, None)
                    .unwrap();
                assert_eq!(umap.entry.name, "selected");
            })
        });
    }
}

criterion_group!(benches, forms_string_map);
criterion_main!(benches);
