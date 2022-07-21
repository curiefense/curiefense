use anyhow::Context;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::config::matchers::{
    decode_request_selector_condition, resolve_selector_raw, RequestSelector, RequestSelectorCondition, SelectorType,
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

pub fn resolve_selector_map(sel: HashMap<String, String>) -> anyhow::Result<RequestSelector> {
    if sel.len() != 1 {
        return Err(anyhow::anyhow!("invalid selector {:?}", sel));
    }
    let (key, val) = sel.into_iter().next().unwrap();
    resolve_selector_raw(&key, &val)
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
        let mkey: anyhow::Result<Vec<RequestSelector>> = rawlimit.key.into_iter().map(resolve_selector_map).collect();
        let key = mkey.with_context(|| "when converting the key entry")?;
        let pairwith = resolve_selector_map(rawlimit.pairwith).ok();
        let mut thresholds: Vec<LimitThreshold> = Vec::new();
        let id = rawlimit.id;
        for thr in rawlimit.thresholds {
            let action = actions.get(&thr.action).cloned().unwrap_or_else(|| {
                logs.error(|| format!("Could not resolve action {} in limit {}", thr.action, id));
                SimpleAction::default()
            });

            thresholds.push(LimitThreshold {
                limit: thr.limit.parse().with_context(|| "when converting the limit")?,
                action,
            })
        }
        thresholds.sort_unstable_by(limit_order);
        Ok((
            id.clone(),
            Limit {
                id,
                name: rawlimit.name,
                timeframe: rawlimit
                    .timeframe
                    .parse()
                    .with_context(|| "when converting the timeframe")?,
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
    ) -> HashMap<String, Limit> {
        let mut out = HashMap::new();
        for rl in rawlimits {
            let curid = rl.id.clone();
            match Limit::convert(logs, actions, rl) {
                Ok((nm, lm)) => {
                    out.insert(nm, lm);
                }
                Err(rr) => logs.error(|| format!("limit id {}: {:?}", curid, rr)),
            }
        }
        out
    }
}

/// order limits in descending order, so that highest comes first
pub fn limit_order(a: &LimitThreshold, b: &LimitThreshold) -> Ordering {
    b.limit.cmp(&a.limit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_ordering() {
        fn mklimit(name: &str, v: u64) -> LimitThreshold {
            LimitThreshold {
                limit: v,
                action: SimpleAction::from_reason(name.to_string()),
            }
        }
        let l1 = mklimit("l1", 0);
        let l2 = mklimit("l2", 8);
        let l3 = mklimit("l3", 4);
        let l4 = mklimit("l4", 1);
        let mut lvec = vec![l3, l2, l1, l4];
        lvec.sort_unstable_by(limit_order);
        let names: Vec<String> = lvec.into_iter().map(|l| l.action.reason).collect();
        let expected: Vec<String> = ["l2", "l3", "l4", "l1"].iter().map(|x| x.to_string()).collect();
        assert_eq!(names, expected);
    }
}
