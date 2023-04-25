use criterion::*;
use curiefense::analyze::{analyze, APhase0, CfRulesArg};
use curiefense::config::contentfilter::{ContentFilterProfile, ContentFilterRules};
use curiefense::config::hostmap::{PolicyId, SecurityPolicy};
use curiefense::config::raw::AclProfile;
use curiefense::config::virtualtags::VirtualTags;
use curiefense::contentfilter::masking;
use curiefense::grasshopper::{DummyGrasshopper, PrecisionLevel};
use curiefense::interface::{SecpolStats, SimpleDecision, StatsCollect};
use curiefense::logs::{LogLevel, Logs};
use curiefense::tagging::tag_request;
use curiefense::utils::{map_request, RawRequest, RequestMeta};
use std::collections::HashMap;
use std::sync::Arc;

fn logging_empty(c: &mut Criterion) {
    let mut headers = HashMap::new();
    headers.insert("content-type".into(), "application/json".into());
    let raw = RawRequest {
        ipstr: "1.2.3.4".into(),
        headers,
        meta: RequestMeta {
            authority: Some("x.com".into()),
            method: "GET".into(),
            path: "/some/path/to?x=1&y=2&z=ZHFzcXNkcXNk".into(),
            requestid: None,
            extra: HashMap::new(),
            protocol: None,
        },
        mbody: Some(b"{\"zzz\":45}"),
    };
    let secpolicy = Arc::new(SecurityPolicy {
        policy: PolicyId {
            id: "__default__".into(),
            name: "__default__".into(),
        },
        entry: PolicyId {
            id: "__default__".into(),
            name: "__default__".into(),
        },
        tags: Vec::new(),
        acl_active: true,
        acl_profile: AclProfile::default(),
        content_filter_active: true,
        content_filter_profile: ContentFilterProfile::default_from_seed("seedqszqsdqsdd"),
        limits: Vec::new(),
        session: Vec::new(),
        session_ids: Vec::new(),
    });
    let mut logs = Logs::new(LogLevel::Debug);
    let stats =
        StatsCollect::new(std::time::Instant::now(), "QSDQSDQSD".into()).secpol(SecpolStats::build(&secpolicy, 0));
    let reqinfo = map_request(&mut logs, secpolicy, None, &raw, None, HashMap::new());
    let (itags, globalfilter_dec, stats) =
        tag_request(stats, PrecisionLevel::Invalid, &[], &reqinfo, &VirtualTags::default());
    let p0 = APhase0 {
        flows: HashMap::new(),
        globalfilter_dec,
        precision_level: PrecisionLevel::Invalid,
        itags,
        reqinfo,
        stats,
    };
    let rules = ContentFilterRules::empty();
    let result = async_std::task::block_on(analyze(
        &mut logs,
        Some(&DummyGrasshopper {}),
        p0,
        CfRulesArg::Get(Some(&rules)),
    ));
    let result = result.mask();
    c.bench_with_input(BenchmarkId::new("log_json", "empty_request"), &result, |b, r| {
        b.iter(|| async_std::task::block_on(r.decision.log_json(&r.rinfo, &r.tags, &r.stats, &logs, HashMap::new())))
    });
}

criterion_group!(logging, logging_empty);
criterion_main!(logging);
