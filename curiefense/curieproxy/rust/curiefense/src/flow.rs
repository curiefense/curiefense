use std::collections::HashMap;

use crate::config::flow::{FlowElement, SequenceKey};
use crate::config::utils::RequestSelector;
use crate::interface::{SimpleDecision, Tags};
use crate::utils::{check_selector_cond, select_string, RequestInfo};

fn session_sequence_key(ri: &RequestInfo) -> SequenceKey {
    let host_part: &str = ri
        .rinfo
        .meta
        .authority
        .as_ref()
        .or_else(|| ri.headers.get("host"))
        .map(|x| x.as_str())
        .unwrap_or("nil");
    SequenceKey(ri.rinfo.meta.method.to_string() + host_part + &ri.rinfo.qinfo.qpath)
}

fn build_redis_key(reqinfo: &RequestInfo, key: &[RequestSelector], entry_id: &str, entry_name: &str) -> String {
    let mut tohash = entry_id.to_string() + entry_name;
    for kpart in key.iter().filter_map(|r| select_string(reqinfo, r)) {
        tohash += &kpart;
    }
    format!("{:X}", md5::compute(tohash))
}

fn flow_match(reqinfo: &RequestInfo, tags: &Tags, elem: &FlowElement) -> bool {
    if elem.exclude.iter().any(|e| tags.contains(e)) {
        return false;
    }
    if !(elem.include.is_empty() || elem.include.iter().any(|e| tags.contains(e))) {
        return false;
    }
    elem.select.iter().all(|e| check_selector_cond(reqinfo, tags, e))
}

fn check_flow(
    cnx: &mut redis::Connection,
    redis_key: &str,
    step: u32,
    ttl: u64,
    is_last: bool,
) -> anyhow::Result<bool> {
    // first, read from REDIS how many steps already passed
    let mlistlen: Option<usize> = redis::cmd("LLEN").arg(redis_key).query(cnx)?;
    let listlen = mlistlen.unwrap_or(0);

    if is_last {
        Ok(step as usize == listlen)
    } else {
        if step as usize == listlen {
            let (_, mexpire): ((), Option<i64>) = redis::pipe()
                .cmd("LPUSH")
                .arg(redis_key)
                .arg("foo")
                .cmd("TTL")
                .arg(redis_key)
                .query(cnx)?;
            let expire = mexpire.unwrap_or(-1);
            if expire < 0 {
                let _: () = redis::cmd("EXPIRE").arg(redis_key).arg(ttl).query(cnx)?;
            }
        }
        // never block if not the last step!
        Ok(true)
    }
}

pub fn flow_check(
    flows: &HashMap<SequenceKey, Vec<FlowElement>>,
    reqinfo: &RequestInfo,
    tags: &Tags,
) -> anyhow::Result<SimpleDecision> {
    let sequence_key = session_sequence_key(reqinfo);
    match flows.get(&sequence_key) {
        None => Ok(SimpleDecision::Pass),
        Some(elems) => {
            let mut bad = SimpleDecision::Pass;
            // do not establish the connection if unneeded
            let mut cnx = crate::redis::redis_conn()?;
            for elem in elems.iter().filter(|e| flow_match(reqinfo, tags, e)) {
                let redis_key = build_redis_key(reqinfo, &elem.key, &elem.id, &elem.name);
                if check_flow(&mut cnx, &redis_key, elem.step, elem.ttl, elem.is_last)? {
                    return Ok(SimpleDecision::Pass);
                } else {
                    bad = SimpleDecision::Action(elem.action.clone(), Some(serde_json::json!({"initiator":"flow_check"})));
                }
            }
            Ok(bad)
        }
    }
}
