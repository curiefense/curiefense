use crate::logs::Logs;
use crate::redis::{extract_bannable_action, get_ban_key, is_banned};
use redis::RedisResult;

use crate::config::limit::Limit;
use crate::config::limit::LimitThreshold;
use crate::interface::{SimpleActionT, SimpleDecision, Tags};
use crate::redis::redis_conn;
use crate::utils::{select_string, RequestInfo};

fn build_key(security_policy_name: &str, reqinfo: &RequestInfo, limit: &Limit) -> Option<String> {
    let mut key = security_policy_name.to_string() + &limit.id;
    for kpart in limit.key.iter().map(|r| select_string(reqinfo, r)) {
        key += &kpart?;
    }
    Some(format!("{:X}", md5::compute(key)))
}

fn limit_react(
    logs: &mut Logs,
    tags: &mut Tags,
    cnx: &mut redis::Connection,
    limit: &Limit,
    threshold: &LimitThreshold,
    key: String,
    ban_key: &str,
) -> SimpleDecision {
    tags.insert(&limit.name);
    let action = extract_bannable_action(cnx, logs, &threshold.action, &key, ban_key);
    SimpleDecision::Action(
        action,
        serde_json::json!({
            "initiator": "limit",
            "limitname": limit.name,
            "key": key
        }),
    )
}

fn redis_get_limit(
    cnx: &mut redis::Connection,
    key: &str,
    timeframe: u64,
    pairvalue: Option<String>,
) -> RedisResult<i64> {
    let (mcurrent, mexpire): (Option<i64>, Option<i64>) = match &pairvalue {
        None => redis::pipe().cmd("INCR").arg(key).cmd("TTL").arg(key).query(cnx)?,
        Some(pv) => redis::pipe()
            .cmd("SADD")
            .arg(key)
            .arg(pv)
            .ignore()
            .cmd("SCARD")
            .arg(key)
            .cmd("TTL")
            .arg(key)
            .query(cnx)?,
    };
    let current = mcurrent.unwrap_or(0);
    let expire = mexpire.unwrap_or(-1);

    if expire < 0 {
        let _: () = redis::cmd("EXPIRE").arg(key).arg(timeframe).query(cnx)?;
    }

    Ok(current)
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

pub fn limit_check(
    logs: &mut Logs,
    security_policy_name: &str,
    reqinfo: &RequestInfo,
    limits: &[Limit],
    tags: &mut Tags,
) -> SimpleDecision {
    // early return to avoid redis connection
    if limits.is_empty() {
        logs.debug("no limits to check");
        return SimpleDecision::Pass;
    }

    // we connect once for all limit tests
    let mut redis = match redis_conn() {
        Ok(c) => c,
        Err(rr) => {
            logs.error(format!("Could not connect to the redis server {}", rr));
            return SimpleDecision::Pass;
        }
    };

    for limit in limits {
        if !limit_match(tags, limit) {
            logs.debug(format!("limit {} excluded", limit.name));
            continue;
        }

        let key = match build_key(security_policy_name, reqinfo, limit) {
            // if we can't build the key, it usually means that a header is missing.
            // If that is the case, we continue to the next limit.
            None => continue,
            Some(k) => k,
        };
        let ban_key = get_ban_key(&key);
        logs.debug(format!("limit={:?} key={}", limit, key));

        if is_banned(&mut redis, &ban_key) {
            logs.debug("is banned!");
            tags.insert(&limit.name);
            let ban_threshold: &LimitThreshold = limit
                .thresholds
                .iter()
                .find(|t| matches!(t.action.atype, SimpleActionT::Ban(_, _)))
                .unwrap_or(&limit.thresholds[0]);
            return limit_react(logs, tags, &mut redis, limit, ban_threshold, key, &ban_key);
        }

        let pairvalue = limit.pairwith.as_ref().and_then(|sel| select_string(reqinfo, sel));

        match redis_get_limit(&mut redis, &key, limit.timeframe, pairvalue) {
            Err(rr) => logs.error(rr),
            Ok(current_count) => {
                for threshold in &limit.thresholds {
                    // Only one action with highest limit larger than current
                    // counter will be applied, all the rest will be skipped.
                    if current_count > threshold.limit as i64 {
                        return limit_react(logs, tags, &mut redis, limit, &threshold, key, &ban_key);
                    }
                }
            }
        }
    }
    SimpleDecision::Pass
}
