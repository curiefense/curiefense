use std::marker::PhantomData;

use crate::config::hostmap::SecurityPolicy;

pub struct BStageInit;
pub struct BStageSecpol;
#[derive(Clone)]
pub struct BStageMapped;
pub struct BStageFlow;
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

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub revision: String,
    pub processing_stage: usize,

    pub secpol: SecpolStats,

    // stage mapped
    pub globalfilters_active: usize,
    pub globalfilters_total: usize,

    // stage flow
    pub flow_active: usize,
    pub flow_total: usize,

    // stage limit
    pub limit_active: usize,
    pub limit_total: usize,

    // stage acl
    pub acl_active: usize,

    // stage content filter
    pub content_filter_total: usize,
    pub content_filter_triggered: usize,
    pub content_filter_active: usize,
}

// the builder uses a phantom data structure to make sure we did not forget to update the stats from a previous stage
#[derive(Debug, Clone)]
pub struct StatsCollect<A> {
    stats: Stats,
    phantom: PhantomData<A>,
}

impl StatsCollect<BStageInit> {
    pub fn new(revision: String) -> Self {
        StatsCollect {
            stats: Stats {
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
            },
            phantom: PhantomData,
        }
    }

    pub fn secpol(self, secpol: SecpolStats) -> StatsCollect<BStageSecpol> {
        let mut stats = self.stats;
        stats.processing_stage = 1;
        stats.secpol = secpol;
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
    }

    pub fn content_filter_only(self) -> StatsCollect<BStageAcl> {
        let mut stats = self.stats;
        stats.processing_stage = 5;
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
        StatsCollect {
            stats,
            phantom: PhantomData,
        }
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
