use crate::logs::Logs;
use redis::RedisResult;

use crate::config::limit::Limit;
use crate::interface::{SimpleActionT, SimpleDecision, Tags};
use crate::redis::redis_conn;
use crate::utils::{select_string, RequestInfo};

fn build_key(url_map_name: &str, reqinfo: &RequestInfo, limit: &Limit) -> Option<String> {
    let mut key = url_map_name.to_string() + &limit.id;
    for kpart in limit.key.iter().map(|r| select_string(reqinfo, r)) {
        key += &kpart?;
    }
    Some(format!("{:X}", md5::compute(key)))
}

fn get_ban_key(key: &str) -> String {
    format!("{:X}", md5::compute(format!("limit-ban-hash{}", key)))
}

fn is_banned(cnx: &mut redis::Connection, key: &str) -> bool {
    let ban_key = get_ban_key(&key);
    let q: redis::RedisResult<Option<u32>> = redis::cmd("GET").arg(&ban_key).query(cnx);
    q.unwrap_or(None).is_some()
}

fn limit_react(
    logs: &mut Logs,
    tags: &mut Tags,
    cnx: &mut redis::Connection,
    limit: &Limit,
    key: String,
) -> SimpleDecision {
    tags.insert(&limit.name);
    let action = if let SimpleActionT::Ban(subaction, ttl) = &limit.action.atype {
        logs.info(format!("Banned key {} for {}s", key, ttl));
        let ban_key = get_ban_key(&key);
        if let Err(rr) = redis::pipe()
            .cmd("SET")
            .arg(&ban_key)
            .arg(1)
            .cmd("EXPIRE")
            .arg(&ban_key)
            .arg(*ttl)
            .query::<()>(cnx)
        {
            println!("*** Redis error {}", rr);
        }
        *subaction.clone()
    } else {
        limit.action.clone()
    };
    SimpleDecision::Action(
        action,
        serde_json::json!({
            "initiator": "limit",
            "limitname": limit.name,
            "key": key
        }),
    )
}

fn redis_check_limit(
    cnx: &mut redis::Connection,
    key: &str,
    limit: u64,
    ttl: u64,
    pairvalue: Option<String>,
) -> RedisResult<bool> {
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
        let _: () = redis::cmd("EXPIRE").arg(key).arg(ttl).query(cnx)?;
    }
    Ok(current > limit as i64)
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
    url_map_name: &str,
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

        let key = match build_key(url_map_name, reqinfo, limit) {
            None => return SimpleDecision::Pass,
            Some(k) => k,
        };
        logs.debug(format!("limit={:?} key={}", limit, key));

        if limit.limit == 0 {
            logs.debug("limit=0");
            return limit_react(logs, tags, &mut redis, limit, key);
        }

        if is_banned(&mut redis, &key) {
            logs.debug("is banned!");
            tags.insert(&limit.name);
            return limit_react(logs, tags, &mut redis, limit, key);
        }

        let pairvalue = limit.pairwith.as_ref().and_then(|sel| select_string(reqinfo, sel));

        match redis_check_limit(&mut redis, &key, limit.limit, limit.ttl, pairvalue) {
            Err(rr) => logs.error(rr),
            Ok(true) => {
                return limit_react(logs, tags, &mut redis, limit, key);
            }
            Ok(false) => (),
        }
    }
    SimpleDecision::Pass
}
