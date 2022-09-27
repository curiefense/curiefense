use criterion::*;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashSet;

use curiefense::acl::check_acl;
use curiefense::config::raw::AclProfile;
use curiefense::interface::{Location, SimpleAction, Tags};

fn tags_vec(sz: usize) -> Vec<(String, Location)> {
    (0..sz)
        .map(|_| {
            (
                rand::thread_rng()
                    .sample_iter(Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect(),
                Location::Request,
            )
        })
        .collect()
}

fn gen_tags(sz: usize) -> Tags {
    Tags::from_slice(&tags_vec(sz))
}

fn gen_profile(sz: usize) -> AclProfile {
    AclProfile {
        id: format!("{}{}{}", sz, sz, sz),
        name: sz.to_string(),
        allow: tags_vec(sz).into_iter().map(|p| p.0).collect(),
        deny: tags_vec(sz).into_iter().map(|p| p.0).collect(),
        allow_bot: tags_vec(sz).into_iter().map(|p| p.0).collect(),
        deny_bot: tags_vec(sz).into_iter().map(|p| p.0).collect(),
        passthrough: tags_vec(sz).into_iter().map(|p| p.0).collect(),
        force_deny: tags_vec(sz).into_iter().map(|p| p.0).collect(),
        action: SimpleAction::default(),
        tags: HashSet::new(),
    }
}

fn match_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_acl");
    for sz in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(sz), sz, |b, &size| {
            let prof = gen_profile(size);
            let tags = gen_tags(size);
            b.iter(|| check_acl(&tags, &prof))
        });
    }
}

criterion_group!(benches, match_bench);
criterion_main!(benches);
