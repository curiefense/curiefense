use serde::{ser::SerializeSeq, Serialize};
use std::{marker::PhantomData, time::Instant};

use crate::{config::hostmap::SecurityPolicy, utils::json::BigTableKV};

#[derive(Default, Debug, Clone)]
pub struct TimingInfo {
    secpol: Option<u64>,
    mapping: Option<u64>,
    flow: Option<u64>,
    limit: Option<u64>,
    acl: Option<u64>,
    content_filter: Option<u64>,
}

impl Serialize for TimingInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut mp = serializer.serialize_seq(None)?;
        mp.serialize_element(&BigTableKV {
            name: "secpol",
            value: &self.secpol,
        })?;
        mp.serialize_element(&BigTableKV {
            name: "mapping",
            value: &self.mapping,
        })?;
        mp.serialize_element(&BigTableKV {
            name: "flow",
            value: &self.flow,
        })?;
        mp.serialize_element(&BigTableKV {
            name: "limit",
            value: &self.limit,
        })?;
        mp.serialize_element(&BigTableKV {
            name: "acl",
            value: &self.acl,
        })?;
        mp.serialize_element(&BigTableKV {
            name: "content_filter",
            value: &self.content_filter,
        })?;
        mp.end()
    }
}

impl TimingInfo {
    pub fn max_value(&self) -> u64 {
        let mut max_value: u64 = 0;
        if let Some(value) = self.secpol {
            max_value = value.max(max_value);
        }
        if let Some(value) = self.mapping {
            max_value = value.max(max_value);
        }
        if let Some(value) = self.flow {
            max_value = value.max(max_value);
        }
        if let Some(value) = self.limit {
            max_value = value.max(max_value);
        }
        if let Some(value) = self.acl {
            max_value = value.max(max_value);
        }
        if let Some(value) = self.content_filter {
            max_value = value.max(max_value);
        }
        max_value
    }
}

pub struct BStageInit;
pub struct BStageSecpol;
#[derive(Clone)]
pub struct BStageMapped;
#[derive(Clone)]
pub struct BStageFlow;
#[derive(Clone)]
pub struct BStageLimit;
pub struct BStageAcl;
pub struct BStageContentFilter;

#[derive(Debug, Default, Clone)]
pub struct SecpolStats {
    // stage secpol
    pub acl_enabled: bool,
    pub content_filter_enabled: bool,
    pub limit_amount: usize,
    pub globalfilters_amount: usize,
}

impl SecpolStats {
    pub fn build(policy: &SecurityPolicy, globalfilters_amount: usize) -> Self {
        SecpolStats {
            acl_enabled: policy.acl_active,
            content_filter_enabled: policy.content_filter_active,
            limit_amount: policy.limits.len(),
            globalfilters_amount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    start: Instant,
    pub revision: String,
    pub processing_stage: usize,
    pub secpol: SecpolStats,

    // stage mapped
    globalfilters_active: usize,
    globalfilters_total: usize,

    // stage flow
    flow_active: usize,
    flow_total: usize,

    // stage limit
    limit_active: usize,
    limit_total: usize,

    // stage acl
    acl_active: usize,

    // stage content filter
    pub content_filter_total: usize,
    content_filter_triggered: usize,
    content_filter_active: usize,

    pub timing: TimingInfo,
}

impl Stats {
    pub fn new(start: Instant, revision: String) -> Self {
        Stats {
            start,
            revision,
            processing_stage: 0,
            secpol: SecpolStats::default(),

            globalfilters_active: 0,
            globalfilters_total: 0,

            flow_active: 0,
            flow_total: 0,

            limit_active: 0,
            limit_total: 0,

            acl_active: 0,

            content_filter_total: 0,
            content_filter_triggered: 0,
            content_filter_active: 0,
            timing: TimingInfo::default(),
        }
    }
}

// the builder uses a phantom data structure to make sure we did not forget to update the stats from a previous stage
#[derive(Debug, Clone)]
pub struct StatsCollect<A> {
    stats: Stats,
    phantom: PhantomData<A>,
}

impl StatsCollect<BStageInit> {
    pub fn new(start: Instant, revision: String) -> Self {
        StatsCollect {
            stats: Stats::new(start, revision),
            phantom: PhantomData,
        }
    }

    pub fn secpol(self, secpol: SecpolStats) -> StatsCollect<BStageSecpol> {
        let mut stats = self.stats;
        stats.processing_stage = 1;
        stats.secpol = secpol;
        stats.timing.secpol = Some(stats.start.elapsed().as_micros() as u64);
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }

    pub fn content_filter_only(self) -> StatsCollect<BStageAcl> {
        let mut stats = self.stats;
        stats.processing_stage = 5;
        stats.timing.acl = Some(stats.start.elapsed().as_micros() as u64);
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }
}

impl StatsCollect<BStageSecpol> {
    pub fn mapped(self, globalfilters_total: usize, globalfilters_active: usize) -> StatsCollect<BStageMapped> {
        let mut stats = self.stats;
        stats.processing_stage = 2;
        stats.globalfilters_total = globalfilters_total;
        stats.globalfilters_active = globalfilters_active;
        stats.timing.mapping = Some(stats.start.elapsed().as_micros() as u64);
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }

    pub fn early_exit(self) -> Stats {
        self.stats
    }
}

impl StatsCollect<BStageMapped> {
    pub fn mapped_stage_build(self) -> Stats {
        self.stats
    }

    pub fn no_flow(self) -> StatsCollect<BStageFlow> {
        let mut stats = self.stats;
        stats.processing_stage = 3;
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }

    pub fn flow(self, flow_total: usize, flow_active: usize) -> StatsCollect<BStageFlow> {
        let mut stats = self.stats;
        stats.processing_stage = 3;
        stats.flow_total = flow_total;
        stats.flow_active = flow_active;
        stats.timing.flow = Some(stats.start.elapsed().as_micros() as u64);
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }
}

impl StatsCollect<BStageFlow> {
    pub fn flow_stage_build(self) -> Stats {
        self.stats
    }

    pub fn no_limit(self) -> StatsCollect<BStageLimit> {
        let mut stats = self.stats;
        stats.processing_stage = 4;
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }

    pub fn limit(self, limit_total: usize, limit_active: usize) -> StatsCollect<BStageLimit> {
        let mut stats = self.stats;
        stats.processing_stage = 4;
        stats.limit_total = limit_total;
        stats.limit_active = limit_active;
        stats.timing.limit = Some(stats.start.elapsed().as_micros() as u64);
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }
}

impl StatsCollect<BStageLimit> {
    pub fn limit_stage_build(self) -> Stats {
        self.stats
    }

    pub fn acl(self, acl_active: usize) -> StatsCollect<BStageAcl> {
        let mut stats = self.stats;
        stats.processing_stage = 5;
        stats.acl_active = acl_active;
        stats.timing.acl = Some(stats.start.elapsed().as_micros() as u64);
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }
}

impl StatsCollect<BStageAcl> {
    pub fn acl_stage_build(self) -> Stats {
        self.stats
    }

    pub fn no_content_filter(self) -> StatsCollect<BStageContentFilter> {
        let mut stats = self.stats;
        stats.processing_stage = 6;
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }

    pub fn cf_no_match(self, total: usize) -> StatsCollect<BStageContentFilter> {
        let mut stats = self.stats;
        stats.processing_stage = 6;
        stats.content_filter_total = total;
        stats.timing.content_filter = Some(stats.start.elapsed().as_micros() as u64);
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }

    pub fn cf_matches(self, total: usize, triggered: usize, active: usize) -> StatsCollect<BStageContentFilter> {
        let mut stats = self.stats;
        stats.processing_stage = 6;
        stats.content_filter_total = total;
        stats.content_filter_active = active;
        stats.content_filter_triggered = triggered;
        stats.timing.content_filter = Some(stats.start.elapsed().as_micros() as u64);
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }
}

impl StatsCollect<BStageContentFilter> {
    pub fn cf_stage_build(self) -> Stats {
        self.stats
    }
}
