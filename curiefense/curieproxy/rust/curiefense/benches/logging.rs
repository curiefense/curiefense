use criterion::*;
use curiefense::analyze::{analyze, APhase0, CfRulesArg};
use curiefense::config::contentfilter::{ContentFilterProfile, ContentFilterRules};
use curiefense::config::hostmap::{PolicyId, SecurityPolicy};
use curiefense::config::raw::AclProfile;
use curiefense::grasshopper::DummyGrasshopper;
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
        acl_active: true,
        acl_profile: AclProfile::default(),
        content_filter_active: true,
        content_filter_profile: ContentFilterProfile::default_from_seed("seedqszqsdqsdd"),
        limits: Vec::new(),
        session: Vec::new(),
    });
    let mut logs = Logs::new(LogLevel::Debug);
    let reqinfo = map_request(
        &mut logs,
        &secpolicy.policy.id,
        &secpolicy.entry.id,
        &secpolicy.session,
        &secpolicy.content_filter_profile.masking_seed,
        &secpolicy.content_filter_profile.decoding,
        &secpolicy.content_filter_profile.content_type,
        secpolicy.content_filter_profile.referer_as_uri,
        10,
        secpolicy.content_filter_profile.ignore_body,
        &raw,
        None,
    );

    let stats = StatsCollect::new("QSDQSDQSD".into()).secpol(SecpolStats::build(&secpolicy, 0));
    let (itags, _, stats) = tag_request(stats, false, &[], &reqinfo);
    let p0 = APhase0 {
        flows: HashMap::new(),
        globalfilter_dec: SimpleDecision::Pass,
        is_human: false,
        itags,
        reqinfo,
        securitypolicy: secpolicy,
        stats,
    };
    let rules = ContentFilterRules::empty();
    let result = async_std::task::block_on(analyze(
        &mut logs,
        Some(&DummyGrasshopper {}),
        p0,
        CfRulesArg::Get(Some(&rules)),
    ));
    c.bench_with_input(BenchmarkId::new("log_json", "empty_request"), &result, |b, r| {
        b.iter(|| async_std::task::block_on(r.decision.log_json(&r.rinfo, &r.tags, &r.stats, &logs, HashMap::new())))
    });
}

criterion_group!(logging, logging_empty);
criterion_main!(logging);
