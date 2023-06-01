use std::collections::HashSet;

use crate::acl::check_acl;
use crate::config::contentfilter::ContentFilterRules;
use crate::config::flow::FlowMap;
use crate::config::CONFIGS;
use crate::contentfilter::{content_filter_check, masking};
use crate::flow::{flow_build_query, flow_info, flow_process, flow_resolve_query, FlowCheck, FlowResult};
use crate::grasshopper::{
    challenge_phase01, challenge_phase02, check_app_sig, handle_bio_reports, GHMode, Grasshopper, PrecisionLevel,
};
use crate::interface::stats::{BStageMapped, StatsCollect};
use crate::interface::{
    merge_decisions, AclStage, AnalyzeResult, BStageFlow, BlockReason, Decision, Location, SimpleDecision, Tags,
};
use crate::limit::{limit_build_query, limit_info, limit_process, limit_resolve_query, LimitCheck, LimitResult};
use crate::logs::Logs;
use crate::redis::redis_async_conn;
use crate::utils::{eat_errors, BodyDecodingResult, BodyProblem, RequestInfo};

/*

  Scanning advances using the following steps:

  APhase1
    |
    | analyze_query_flow
    v
  APhase2O
    |
    | analyse_flows
    v
  APhase2I
    |
    | analyze_query_limits
    v
  APhase3
    |
    | analyse_finish
    v
  Done
*/

pub enum CfRulesArg<'t> {
    Global,
    Get(Option<&'t ContentFilterRules>),
}

pub struct APhase0 {
    pub flows: FlowMap,
    pub globalfilter_dec: SimpleDecision,
    pub precision_level: PrecisionLevel,
    pub itags: Tags,
    pub reqinfo: RequestInfo,
    pub stats: StatsCollect<BStageMapped>,
}

#[derive(Clone)]
pub struct AnalysisInfo {
    precision_level: PrecisionLevel,
    p0_decision: Decision,
    reqinfo: RequestInfo,
    stats: StatsCollect<BStageMapped>,
    tags: Tags,
}

#[derive(Clone)]
pub struct AnalysisPhase<FLOW, LIMIT> {
    pub flows: FLOW,
    pub limits: LIMIT,
    info: AnalysisInfo,
}

impl<FLOW, LIMIT> AnalysisPhase<FLOW, LIMIT> {
    pub fn next<NFLOW, NLIMIT>(self, flows: NFLOW, limits: NLIMIT) -> AnalysisPhase<NFLOW, NLIMIT> {
        AnalysisPhase {
            flows,
            info: self.info,
            limits,
        }
    }
    pub fn new(flows: FLOW, limits: LIMIT, info: AnalysisInfo) -> Self {
        Self { flows, info, limits }
    }
}

pub type APhase1 = AnalysisPhase<Vec<FlowCheck>, ()>;

pub enum InitResult {
    Res(AnalyzeResult),
    Phase1(APhase1),
}

#[allow(clippy::too_many_arguments)]
pub fn analyze_init<GH: Grasshopper>(logs: &mut Logs, mgh: Option<&GH>, p0: APhase0) -> InitResult {
    let stats = p0.stats;
    let mut tags = p0.itags;
    let reqinfo = p0.reqinfo;
    let securitypolicy = &reqinfo.rinfo.secpolicy;
    let precision_level = p0.precision_level;
    let globalfilter_dec = p0.globalfilter_dec;

    tags.insert_qualified("securitypolicy", &securitypolicy.policy.name, Location::Request);
    tags.insert_qualified("securitypolicy-entry", &securitypolicy.entry.name, Location::Request);
    tags.insert_qualified("aclid", &securitypolicy.acl_profile.id, Location::Request);
    tags.insert_qualified("aclname", &securitypolicy.acl_profile.name, Location::Request);
    tags.insert_qualified(
        "contentfilterid",
        &securitypolicy.content_filter_profile.id,
        Location::Request,
    );
    tags.insert_qualified(
        "contentfiltername",
        &securitypolicy.content_filter_profile.name,
        Location::Request,
    );

    //if /c365 then call gh phase01 with mode passive
    if reqinfo.rinfo.qinfo.uri.starts_with("/c3650cdf") {
        if let Some(gh) = mgh {
            logs.debug("Call challenge phase01 with mode: Passive");
            let decision = challenge_phase01(gh, logs, &reqinfo, Vec::new(), GHMode::Passive);
            return InitResult::Res(AnalyzeResult {
                decision,
                tags,
                rinfo: masking(reqinfo),
                stats: stats.mapped_stage_build(),
            });
        } else {
            logs.debug("Passive challenge detected: can't challenge");
        };
    }

    if !securitypolicy.content_filter_profile.content_type.is_empty() {
        // note that having no body is perfectly OK
        if let BodyDecodingResult::DecodingFailed(rr) = &reqinfo.rinfo.qinfo.body_decoding {
            let reason = match rr {
                BodyProblem::DecodingError(actual, expected) => BlockReason::body_malformed(
                    securitypolicy.content_filter_profile.id.clone(),
                    securitypolicy.content_filter_profile.name.clone(),
                    securitypolicy.content_filter_profile.action.atype.to_raw(),
                    actual,
                    expected.as_deref(),
                ),
                BodyProblem::TooDeep => BlockReason::body_too_deep(
                    securitypolicy.content_filter_profile.id.clone(),
                    securitypolicy.content_filter_profile.name.clone(),
                    securitypolicy.content_filter_profile.action.atype.to_raw(),
                    securitypolicy.content_filter_profile.max_body_depth,
                ),
            };
            // we expect the body to be properly decoded
            let decision = securitypolicy.content_filter_profile.action.to_decision(
                logs,
                precision_level,
                mgh,
                &reqinfo,
                &mut tags,
                vec![reason],
            );
            // add extra tags
            for t in &securitypolicy.content_filter_profile.tags {
                tags.insert(t, Location::Body);
            }
            return InitResult::Res(AnalyzeResult {
                decision,
                tags,
                rinfo: masking(reqinfo),
                stats: stats.mapped_stage_build(),
            });
        }
    }

    //early extraction of the global filters block reasons, to be added to the special url requests' 'triggers' as well:
    let gf_reasons = if let SimpleDecision::Action(_action, reason) = &globalfilter_dec {
        reason.to_owned()
    } else {
        vec![]
    };

    //if /7060 then call gh phase02
    if reqinfo
        .rinfo
        .qinfo
        .uri
        .starts_with("/7060ac19f50208cbb6b45328ef94140a612ee92387e015594234077b4d1e64f1")
    {
        logs.debug("Call challenge phase02");
        if let Some(decision) = mgh.and_then(|gh| challenge_phase02(gh, logs, &reqinfo, gf_reasons.clone())) {
            return InitResult::Res(AnalyzeResult {
                decision,
                tags,
                rinfo: masking(reqinfo),
                stats: stats.mapped_stage_build(),
            });
        }
        logs.debug("challenge phase2 ignored");
    }

    if reqinfo
        .rinfo
        .qinfo
        .uri
        .starts_with("/74d8-ffc3-0f63-4b3c-c5c9-5699-6d5b-3a1")
    {
        if let Some(decision) = mgh.and_then(|gh| check_app_sig(gh, logs, &reqinfo, gf_reasons.clone())) {
            return InitResult::Res(AnalyzeResult {
                decision,
                tags,
                rinfo: masking(reqinfo),
                stats: stats.mapped_stage_build(),
            });
        }
        logs.debug("check_app_sig ignored");
    }

    if reqinfo
        .rinfo
        .qinfo
        .uri
        .starts_with("/8d47-ffc3-0f63-4b3c-c5c9-5699-6d5b-3a1f")
    {
        if let Some(decision) =
            mgh.and_then(|gh| handle_bio_reports(gh, logs, &reqinfo, precision_level, gf_reasons.clone()))
        {
            return InitResult::Res(AnalyzeResult {
                decision,
                tags,
                rinfo: masking(reqinfo),
                stats: stats.mapped_stage_build(),
            });
        }
        logs.debug("handle_bio_report ignored");
    }

    let decision = if let SimpleDecision::Action(action, reason) = globalfilter_dec {
        logs.debug(|| format!("Global filter decision {:?}", reason));
        let decision = action.to_decision(logs, precision_level, mgh, &reqinfo, &mut tags, reason);
        if decision.is_final() {
            return InitResult::Res(AnalyzeResult {
                decision,
                tags,
                rinfo: masking(reqinfo),
                stats: stats.mapped_stage_build(),
            });
        }
        // if the decision was not adopted, get the reason vector back
        // (this is because we passed it to action.to_decision)
        decision
    } else {
        Decision::pass(Vec::new())
    };

    let flow_checks = flow_info(logs, &p0.flows, &reqinfo, &tags);
    let info = AnalysisInfo {
        precision_level,
        p0_decision: decision,
        reqinfo,
        stats,
        tags,
    };
    InitResult::Phase1(APhase1::new(flow_checks, (), info))
}

pub type APhase2O = AnalysisPhase<Vec<FlowResult>, ()>;

pub type APhase2I = AnalysisPhase<StatsCollect<BStageFlow>, Vec<LimitCheck>>;

pub type APhase3 = AnalysisPhase<StatsCollect<BStageFlow>, Vec<LimitResult>>;

impl APhase2O {
    pub fn from_phase1(p1: APhase1, flow_results: Vec<FlowResult>) -> Self {
        Self {
            flows: flow_results,
            limits: (),
            info: p1.info,
        }
    }
}

impl APhase3 {
    pub fn from_phase2(p2: APhase2I, limit_results: Vec<LimitResult>) -> Self {
        Self {
            flows: p2.flows,
            limits: limit_results,
            info: p2.info,
        }
    }
}

pub async fn analyze_query_flows<'t>(logs: &mut Logs, p1: APhase1) -> APhase2O {
    let empty = |info| APhase2O {
        flows: Vec::new(),
        limits: (),
        info,
    };

    let info = p1.info;
    if p1.flows.is_empty() {
        return empty(info);
    }

    let mut redis = match redis_async_conn().await {
        Ok(c) => c,
        Err(rr) => {
            logs.error(|| format!("Could not connect to the redis server {}", rr));
            return empty(info);
        }
    };

    let mut pipe = redis::pipe();
    flow_build_query(&mut pipe, &p1.flows);
    let res: Result<Vec<Option<i64>>, _> = pipe.query_async(&mut redis).await;
    let mut lst = match res {
        Ok(l) => l.into_iter(),
        Err(rr) => {
            logs.error(|| format!("{}", rr));
            return empty(info);
        }
    };

    let flow_results = eat_errors(logs, flow_resolve_query(&mut redis, &mut lst, p1.flows).await);
    logs.debug("query - flow checks done");

    AnalysisPhase {
        flows: flow_results,
        limits: (),
        info,
    }
}

pub fn analyze_flows(logs: &mut Logs, p2: APhase2O) -> APhase2I {
    let mut info = p2.info;
    let stats = flow_process(info.stats.clone(), 0, &p2.flows, &mut info.tags);
    let limit_checks = limit_info(logs, &info.reqinfo, &info.reqinfo.rinfo.secpolicy.limits, &info.tags);
    APhase2I {
        flows: stats,
        limits: limit_checks,
        info,
    }
}

pub async fn analyze_query_limits<'t>(logs: &mut Logs, p2: APhase2I) -> APhase3 {
    let empty = |info, flows| APhase3 {
        flows,
        limits: Vec::new(),
        info,
    };

    let flows = p2.flows;

    let info = p2.info;
    if p2.limits.is_empty() {
        return empty(info, flows);
    }

    let mut redis = match redis_async_conn().await {
        Ok(c) => c,
        Err(rr) => {
            logs.error(|| format!("Could not connect to the redis server {}", rr));
            return empty(info, flows);
        }
    };

    let mut pipe = redis::pipe();
    limit_build_query(&mut pipe, &p2.limits);
    let res: Result<Vec<Option<i64>>, _> = pipe.query_async(&mut redis).await;
    let mut lst = match res {
        Ok(l) => l.into_iter(),
        Err(rr) => {
            logs.error(|| format!("{}", rr));
            return empty(info, flows);
        }
    };

    let limit_results_err = limit_resolve_query(logs, &mut redis, &mut lst, p2.limits).await;
    let limit_results = eat_errors(logs, limit_results_err);
    logs.debug("query - limit checks done");

    AnalysisPhase {
        flows,
        limits: limit_results,
        info,
    }
}

pub fn analyze_finish<GH: Grasshopper>(
    logs: &mut Logs,
    mgh: Option<&GH>,
    cfrules: CfRulesArg<'_>,
    p3: APhase3,
) -> AnalyzeResult {
    // destructure the info structure, so that each field can be consumed independently
    let info = p3.info;
    let mut tags = info.tags;
    let mut cumulated_decision = info.p0_decision;

    let precision_level = info.precision_level;
    let reqinfo = info.reqinfo;
    let secpol = &reqinfo.rinfo.secpolicy;

    let (limit_check, stats) = limit_process(p3.flows, 0, &p3.limits, &mut tags);

    if let SimpleDecision::Action(action, curbrs) = limit_check {
        let limit_decision = action.to_decision(logs, precision_level, mgh, &reqinfo, &mut tags, curbrs);
        cumulated_decision = merge_decisions(cumulated_decision, limit_decision);
        if cumulated_decision.is_final() {
            return AnalyzeResult {
                decision: cumulated_decision,
                tags,
                rinfo: masking(reqinfo),
                stats: stats.limit_stage_build(),
            };
        }
    }
    logs.debug("limit checks done");

    let acl_result = check_acl(&tags, &secpol.acl_profile);
    logs.debug(|| format!("ACL result: {}", acl_result));

    let acl_decision = acl_result.decision(precision_level.is_human());
    let stats = stats.acl(usize::from(acl_decision.is_some()));
    if let Some(decision) = acl_decision {
        let bypass = decision.stage == AclStage::Bypass;
        let mut br = BlockReason::acl(
            reqinfo.rinfo.secpolicy.acl_profile.id.clone(),
            reqinfo.rinfo.secpolicy.acl_profile.name.clone(),
            decision.tags,
            decision.stage,
        );
        // make the block reason inactive, unless it's a challenge, in which case it's always active
        if !secpol.acl_active && !decision.challenge {
            br.action.inactive();
        }
        let is_final = br.action.is_final();

        let acl_decision = Decision::pass(vec![br]);
        cumulated_decision = merge_decisions(cumulated_decision, acl_decision);

        // insert the extra tags
        if !secpol.acl_profile.tags.is_empty() {
            let locs = cumulated_decision
                .reasons
                .iter()
                .flat_map(|r| std::iter::once(&r.location).chain(r.extra_locations.iter()))
                .cloned()
                .collect::<HashSet<_>>();
            for t in &secpol.acl_profile.tags {
                tags.insert_locs(t, locs.clone());
            }
        }

        if secpol.acl_active && bypass {
            return AnalyzeResult {
                decision: cumulated_decision,
                tags,
                rinfo: masking(reqinfo),
                stats: stats.acl_stage_build(),
            };
        }

        let acl_block = |tags: &mut Tags, logs: &mut Logs| {
            secpol
                .acl_profile
                .action
                .to_decision(logs, precision_level, mgh, &reqinfo, tags, Vec::new())
        };

        // Send challenge, even if the acl is inactive in sec_pol.
        if decision.challenge {
            let decision = if let Some(gh) = mgh {
                logs.debug("Call challenge phase01 with mode: Active (acl)");
                challenge_phase01(gh, logs, &reqinfo, Vec::new(), GHMode::Active)
            } else {
                logs.debug("ACL challenge detected: can't challenge");
                acl_block(&mut tags, logs)
            };

            cumulated_decision = merge_decisions(cumulated_decision, decision);
            return AnalyzeResult {
                decision: cumulated_decision,
                tags,
                rinfo: masking(reqinfo),
                stats: stats.acl_stage_build(),
            };
        }

        if is_final {
            let decision = acl_block(&mut tags, logs);
            cumulated_decision = merge_decisions(cumulated_decision, decision);
            return AnalyzeResult {
                decision: cumulated_decision,
                tags,
                rinfo: masking(reqinfo),
                stats: stats.acl_stage_build(),
            };
        }
    };

    let mut cfcheck =
        |stats, mrls| content_filter_check(logs, stats, &mut tags, &reqinfo, &secpol.content_filter_profile, mrls);
    // otherwise, run content_filter_check
    let (content_filter_result, stats) = match cfrules {
        CfRulesArg::Global => match CONFIGS.hsdb.read() {
            Ok(rd) => cfcheck(stats, rd.get(&secpol.content_filter_profile.id)),
            Err(rr) => {
                logs.error(|| format!("Could not get lock on HSDB: {}", rr));
                (Ok(()), stats.no_content_filter())
            }
        },
        CfRulesArg::Get(r) => cfcheck(stats, r),
    };
    logs.debug("Content Filter checks done");

    let content_filter_decision = match content_filter_result {
        Ok(()) => Decision::pass(Vec::new()),
        Err(cfblock) => {
            // insert extra tags
            if !secpol.content_filter_profile.tags.is_empty() {
                let locs: HashSet<Location> = cfblock
                    .reasons
                    .iter()
                    .flat_map(|r| std::iter::once(&r.location).chain(r.extra_locations.iter()))
                    .cloned()
                    .collect();
                for t in &secpol.content_filter_profile.tags {
                    tags.insert_locs(t, locs.clone());
                }
            }
            let br = cfblock
                .reasons
                .into_iter()
                .map(|mut reason| {
                    if !secpol.content_filter_active {
                        reason.action.inactive();
                    }
                    reason
                })
                .collect();
            if cfblock.blocking {
                let mut dec = secpol.content_filter_profile.action.to_decision(
                    logs,
                    precision_level,
                    mgh,
                    &reqinfo,
                    &mut tags,
                    br,
                );
                if let Some(mut action) = dec.maction.as_mut() {
                    action.block_mode &= secpol.content_filter_active;
                }
                dec
            } else {
                Decision::pass(br)
            }
        }
    };

    cumulated_decision = merge_decisions(cumulated_decision, content_filter_decision);
    AnalyzeResult {
        decision: cumulated_decision,
        tags,
        rinfo: masking(reqinfo),
        stats: stats.cf_stage_build(),
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn analyze<GH: Grasshopper>(
    logs: &mut Logs,
    mgh: Option<&GH>,
    p0: APhase0,
    cfrules: CfRulesArg<'_>,
) -> AnalyzeResult {
    let init_result = analyze_init(logs, mgh, p0);
    match init_result {
        InitResult::Res(result) => result,
        InitResult::Phase1(p1) => {
            let p2i = analyze_query_flows(logs, p1).await;
            let p2o = analyze_flows(logs, p2i);
            let p3 = analyze_query_limits(logs, p2o).await;
            analyze_finish(logs, mgh, cfrules, p3)
        }
    }
}
