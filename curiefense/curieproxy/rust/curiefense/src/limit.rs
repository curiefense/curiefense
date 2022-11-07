use crate::interface::stats::{BStageFlow, BStageLimit, StatsCollect};
use crate::logs::Logs;
use crate::redis::REDIS_KEY_PREFIX;
use redis::aio::ConnectionManager;

use crate::config::limit::Limit;
use crate::config::limit::LimitThreshold;
use crate::interface::{stronger_decision, BlockReason, Location, SimpleDecision, Tags};
use crate::utils::{select_string, RequestInfo};

fn build_key(reqinfo: &RequestInfo, tags: &Tags, limit: &Limit) -> Option<String> {
    let mut key = limit.id.clone();
    for kpart in limit.key.iter().map(|r| select_string(reqinfo, r, Some(tags))) {
        key += &kpart?;
    }
    Some(format!("{}{:X}", *REDIS_KEY_PREFIX, md5::compute(key)))
}

#[allow(clippy::too_many_arguments)]
fn limit_pure_react(tags: &mut Tags, limit: &Limit, threshold: &LimitThreshold) -> SimpleDecision {
    tags.insert_qualified("limit-id", &limit.id, Location::Request);
    tags.insert_qualified("limit-name", &limit.name, Location::Request);
    let action = threshold.action.clone();
    let decision = action.atype.to_bdecision();
    SimpleDecision::Action(
        action,
        vec![BlockReason::limit(
            limit.id.clone(),
            limit.name.clone(),
            threshold.limit,
            decision,
        )],
    )
}

fn limit_match(tags: &Tags, elem: &Limit) -> bool {
    if elem.exclude.iter().any(|e| tags.contains(e)) {
        return false;
    }
    if !(elem.include.is_empty() || elem.include.iter().any(|e| tags.contains(e))) {
        return false;
    }
    true
}

/// an item that needs to be checked in redis
#[derive(Clone)]
pub struct LimitCheck {
    pub key: String,
    pub pairwith: Option<String>,
    pub limit: Limit,
}

impl LimitCheck {
    pub fn zero_limits(&self) -> bool {
        self.limit.thresholds.iter().all(|t| t.limit == 0)
    }
}

/// generate information that needs to be checked in redis for limit checks
pub fn limit_info(logs: &mut Logs, reqinfo: &RequestInfo, limits: &[Limit], tags: &Tags) -> Vec<LimitCheck> {
    let mut out = Vec::new();
    for limit in limits {
        if !limit_match(tags, limit) {
            continue;
        }
        let key = match build_key(reqinfo, tags, limit) {
            // if we can't build the key, it usually means that a header is missing.
            // If that is the case, we continue to the next limit.
            None => continue,
            Some(k) => k,
        };
        let pairwith = match &limit.pairwith {
            None => None,
            Some(sel) => match select_string(reqinfo, sel, Some(tags)) {
                None => continue,
                Some(x) => Some(x),
            },
        };
        logs.debug(|| format!("checking limit[{}/{:?}] {:?}", key, pairwith, limit));
        out.push(LimitCheck {
            key,
            pairwith,
            limit: limit.clone(),
        })
    }
    out
}

#[derive(Clone)]
pub struct LimitResult {
    pub limit: Limit,
    pub curcount: i64,
}

pub fn limit_build_query(pipe: &mut redis::Pipeline, checks: &[LimitCheck]) {
    for check in checks {
        let key = &check.key;
        if !check.zero_limits() {
            match &check.pairwith {
                None => {
                    pipe.cmd("INCR").arg(key).cmd("TTL").arg(key);
                }
                Some(pv) => {
                    pipe.cmd("SADD")
                        .arg(key)
                        .arg(pv)
                        .ignore()
                        .cmd("SCARD")
                        .arg(key)
                        .cmd("TTL")
                        .arg(key);
                }
            };
        }
    }
}

pub async fn limit_resolve_query<I: Iterator<Item = Option<i64>>>(
    logs: &mut Logs,
    redis: &mut ConnectionManager,
    iter: &mut I,
    checks: Vec<LimitCheck>,
) -> anyhow::Result<Vec<LimitResult>> {
    let mut out = Vec::new();
    let mut pipe = redis::pipe();

    for check in checks {
        let (curcount, expire) = if check.zero_limits() {
            (1, 0)
        } else {
            let curcount = match iter.next() {
                None => anyhow::bail!("Empty iterator when getting curcount for {:?}", check.limit),
                Some(r) => r.unwrap_or(0),
            };
            let expire = match iter.next() {
                None => anyhow::bail!("Empty iterator when getting expire for {:?}", check.limit),
                Some(r) => r.unwrap_or(-1),
            };
            (curcount, expire)
        };
        logs.debug(|| format!("limit {} curcount={} expire={}", check.limit.id, curcount, expire));
        if expire < 0 {
            pipe.cmd("EXPIRE").arg(&check.key).arg(&check.limit.timeframe);
        }
        pipe.query_async(redis).await?;
        out.push(LimitResult {
            limit: check.limit,
            curcount,
        })
    }
    Ok(out)
}

/// performs the redis requests and compute the proper reactions based on
pub fn limit_process(
    stats: StatsCollect<BStageFlow>,
    nlimits: usize,
    results: &[LimitResult],
    tags: &mut Tags,
) -> (SimpleDecision, StatsCollect<BStageLimit>) {
    let mut out = SimpleDecision::Pass;
    for result in results {
        if result.curcount > 0 {
            for threshold in &result.limit.thresholds {
                // Only one action with highest limit larger than current
                // counter will be applied, all the rest will be skipped.
                if result.curcount > threshold.limit as i64 {
                    out = stronger_decision(out, limit_pure_react(tags, &result.limit, threshold));
                }
            }
        }
    }

    (out, stats.limit(nlimits, results.len()))
}
