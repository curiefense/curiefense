use anyhow::Context;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::config::matchers::{
    decode_request_selector_condition, RequestSelector, RequestSelectorCondition, SelectorType,
};
use crate::config::raw::{RawLimit, RawLimitSelector};
use crate::interface::SimpleAction;
use crate::logs::Logs;

#[derive(Debug, Clone)]
pub struct Limit {
    pub id: String,
    pub name: String,
    pub timeframe: u64,
    pub thresholds: Vec<LimitThreshold>,
    pub exclude: HashSet<String>,
    pub include: HashSet<String>,
    pub pairwith: Option<RequestSelector>,
    pub key: Vec<RequestSelector>,
}

#[derive(Debug, Clone)]
pub struct LimitThreshold {
    pub limit: u64,
    pub action: SimpleAction,
}

pub fn resolve_selectors(rawsel: RawLimitSelector) -> anyhow::Result<Vec<RequestSelectorCondition>> {
    let mk_selectors = |tp: SelectorType, mp: HashMap<String, String>| {
        mp.into_iter()
            .map(move |(v, cond)| decode_request_selector_condition(tp, &v, &cond))
    };
    mk_selectors(SelectorType::Args, rawsel.args)
        .chain(mk_selectors(SelectorType::Cookies, rawsel.cookies))
        .chain(mk_selectors(SelectorType::Headers, rawsel.headers))
        .chain(mk_selectors(SelectorType::Attrs, rawsel.attrs))
        .collect()
}

impl Limit {
    fn convert(
        logs: &mut Logs,
        actions: &HashMap<String, SimpleAction>,
        rawlimit: RawLimit,
    ) -> anyhow::Result<(String, Limit)> {
        let mkey: anyhow::Result<Vec<RequestSelector>> = rawlimit
            .key
            .into_iter()
            .map(RequestSelector::resolve_selector_map)
            .collect();
        let key = mkey.with_context(|| "when converting the key entry")?;
        let pairwith = RequestSelector::resolve_selector_map(rawlimit.pairwith).ok();
        let mut thresholds: Vec<LimitThreshold> = Vec::new();
        let id = rawlimit.id;
        for thr in rawlimit.thresholds {
            let action = actions.get(&thr.action).cloned().unwrap_or_else(|| {
                logs.error(|| format!("Could not resolve action {} in limit {}", thr.action, id));
                SimpleAction::default()
            });

            thresholds.push(LimitThreshold {
                limit: thr.limit.inner,
                action,
            })
        }
        thresholds.sort_unstable_by(limit_order);
        Ok((
            id.clone(),
            Limit {
                id,
                name: rawlimit.name,
                timeframe: rawlimit.timeframe.inner,
                include: rawlimit.include.into_iter().collect(),
                exclude: rawlimit.exclude.into_iter().collect(),
                thresholds,
                pairwith,
                key,
            },
        ))
    }

    pub fn resolve(
        logs: &mut Logs,
        actions: &HashMap<String, SimpleAction>,
        rawlimits: Vec<RawLimit>,
    ) -> (HashMap<String, Limit>, Vec<Limit>) {
        let mut out = HashMap::new();
        let mut globals = Vec::new();
        for rl in rawlimits {
            let curid = rl.id.clone();
            let global = rl.global;
            match Limit::convert(logs, actions, rl) {
                Ok((nm, lm)) => {
                    if global {
                        globals.push(lm.clone())
                    }
                    out.insert(nm, lm);
                }
                Err(rr) => logs.error(|| format!("limit id {}: {:?}", curid, rr)),
            }
        }
        (out, globals)
    }
}

/// order limits in descending order, so that highest comes first
pub fn limit_order(a: &LimitThreshold, b: &LimitThreshold) -> Ordering {
    b.limit.cmp(&a.limit)
}

#[cfg(test)]
mod tests {
    use crate::interface::SimpleActionT;

    use super::*;

    #[test]
    fn test_limit_ordering() {
        fn mklimit(v: u64) -> LimitThreshold {
            LimitThreshold {
                limit: v,
                action: SimpleAction {
                    atype: SimpleActionT::Custom {
                        content: "test".to_string(),
                    },
                    headers: None,
                    status: v as u32,
                    extra_tags: None,
                },
            }
        }
        let l1 = mklimit(0);
        let l2 = mklimit(8);
        let l3 = mklimit(4);
        let l4 = mklimit(1);
        let mut lvec = vec![l3, l2, l1, l4];
        lvec.sort_unstable_by(limit_order);
        let status: Vec<u64> = lvec.into_iter().map(|l| l.limit).collect();
        let expected: Vec<u64> = vec![8, 4, 1, 0];
        assert_eq!(status, expected);
    }
}
