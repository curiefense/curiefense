mod iptools;
mod lua;

use crate::lua::Luagrasshopper;

use anyhow::anyhow;
use curiefense::interface::Tags;
use curiefense::session::update_tags;
use curiefense::session::JRequestMap;
use curiefense::utils::RequestMeta;
use mlua::prelude::*;
use serde_json::json;
use std::collections::HashMap;

use curiefense::acl::{check_acl, AclDecision, AclResult, BotHuman};
use curiefense::config::{with_config, HSDB};
use curiefense::flow::flow_check;
use curiefense::interface::{challenge_phase01, challenge_phase02, Action, ActionType, Decision, Grasshopper};
use curiefense::limit::limit_check;
use curiefense::logs::Logs;
use curiefense::session;
use curiefense::tagging::tag_request;
use curiefense::urlmap::match_urlmap;
use curiefense::utils::{map_request, InspectionResult, RequestInfo};
use curiefense::waf::waf_check;

/// Lua interface to the inspection function
///
/// args are
/// * meta (contains keys "method", "path", and optionally "authority")
/// * headers
/// * (opt) body
/// * ip addr
/// * (opt) grasshopper
#[allow(clippy::type_complexity)]
#[allow(clippy::unnecessary_wraps)]
fn lua_inspect_request(
    _lua: &Lua,
    args: (
        HashMap<String, String>, // meta
        HashMap<String, String>, // headers
        Option<LuaString>,       // maybe body
        String,                  // ip
        Option<LuaTable>,        // grasshopper
    ),
) -> LuaResult<(String, Option<String>)> {
    let (meta, headers, lua_body, str_ip, lua_grasshopper) = args;
    let grasshopper = lua_grasshopper.map(Luagrasshopper);

    // TODO: solve the lifetime issue for the &[u8] to reduce duplication
    let res = match lua_body {
        None => inspect_request("/config/current/config", meta, headers, None, str_ip, grasshopper),
        Some(body) => inspect_request(
            "/config/current/config",
            meta,
            headers,
            Some(body.as_bytes()),
            str_ip,
            grasshopper,
        ),
    };

    Ok(match res {
        Err(rr) => (
            Decision::Pass.to_json_raw(serde_json::Value::Null, Logs::default()),
            Some(rr),
        ),
        Ok(ir) => ir.into_json(),
    })
}

/// Rust-native inspection top level function
fn inspect_request<GH: Grasshopper>(
    configpath: &str,
    meta: HashMap<String, String>,
    headers: HashMap<String, String>,
    mbody: Option<&[u8]>,
    ip: String,
    grasshopper: Option<GH>,
) -> Result<InspectionResult, String> {
    let mut logs = Logs::default();
    logs.debug("Inspection init");
    let rmeta: RequestMeta = RequestMeta::from_map(meta)?;

    let reqinfo = map_request(&mut logs, ip, headers, rmeta, mbody)?;

    let (dec, tags) = inspect_generic_request_map(configpath, grasshopper, &reqinfo, Tags::default(), &mut logs);
    Ok(InspectionResult {
        decision: dec,
        tags: Some(tags),
        logs,
        err: None,
        rinfo: Some(reqinfo),
    })
}

/// Lua entry point, parameters are
///  * a JSON-encoded request_map
///  * the grasshopper lua module
pub fn inspect_request_map(_lua: &Lua, args: (String, Option<LuaTable>)) -> LuaResult<String> {
    let (encoded_request_map, lua_grasshopper) = args;
    let grasshopper = lua_grasshopper.map(Luagrasshopper);

    let jvalue: serde_json::Value = match serde_json::from_str(&encoded_request_map) {
        Ok(v) => v,
        Err(rr) => {
            let mut logs = Logs::default();
            logs.error(format!("Could not decode the request map: {}", rr));
            return Ok(Decision::Pass.to_json_raw(serde_json::Value::Null, logs));
        }
    };
    let jmap: JRequestMap = match serde_json::from_value(jvalue.clone()) {
        Ok(v) => v,
        Err(rr) => {
            let mut logs = Logs::default();
            logs.error(format!("Could not decode the request map: {}", rr));
            return Ok(Decision::Pass.to_json_raw(jvalue, logs));
        }
    };
    let (rinfo, itags) = jmap.into_request_info();

    let mut logs = Logs::default();
    let (res, tags) = inspect_generic_request_map("/config/current/config", grasshopper, &rinfo, itags, &mut logs);
    let updated_request_map = match update_tags(jvalue, tags) {
        Ok(v) => v,
        Err(rr) => {
            logs.error(rr);
            serde_json::Value::Null
        }
    };
    Ok(res.to_json_raw(updated_request_map, logs))
}

fn acl_block(blocking: bool, code: i32, tags: &[String]) -> Decision {
    Decision::Action(Action {
        atype: if blocking {
            ActionType::Block
        } else {
            ActionType::Monitor
        },
        block_mode: blocking,
        ban: false,
        status: 403,
        headers: None,
        reason: json!({"action": code, "initiator": "acl", "reason": tags }),
        content: "access denied".to_string(),
        extra_tags: None,
    })
}

fn challenge_verified<GH: Grasshopper>(gh: &GH, reqinfo: &RequestInfo) -> bool {
    if let Some(rbzid) = reqinfo.cookies.get("rbzid") {
        if let Some(ua) = reqinfo.headers.get("user-agent") {
            return gh.parse_rbzid(&rbzid.replace('-', "="), ua).unwrap_or(false);
        }
    }
    false
}

// generic entry point when the request map has already been parsed
fn inspect_generic_request_map<GH: Grasshopper>(
    configpath: &str,
    mgh: Option<GH>,
    reqinfo: &RequestInfo,
    itags: Tags,
    logs: &mut Logs,
) -> (Decision, Tags) {
    let mut tags = itags;

    logs.debug("Inspection starts");
    // do all config queries in the lambda once
    // there is a lot of copying taking place, to minimize the lock time
    // this decision should be backed with benchmarks
    let ((nm, urlmap), ntags, flows) = match with_config(configpath, logs, |slogs, cfg| {
        let murlmap = match_urlmap(&reqinfo, cfg, slogs).map(|(nm, um)| (nm, um.clone()));
        let nflows = cfg.flows.clone();
        let ntags = tag_request(&cfg, &reqinfo);
        (murlmap, ntags, nflows)
    }) {
        Some((Some(stuff), itags, iflows)) => (stuff, itags, iflows),
        _ => {
            return (Decision::Pass, Tags::default());
        }
    };
    logs.debug("request tagged");
    tags.extend(ntags);
    tags.insert_qualified("urlmap", &nm);
    tags.insert_qualified("urlmap-entry", &urlmap.name);
    tags.insert_qualified("aclid", &urlmap.acl_profile.id);
    tags.insert_qualified("aclname", &urlmap.acl_profile.name);
    tags.insert_qualified("wafid", &urlmap.waf_profile.name);

    match flow_check(&flows, &reqinfo, &tags) {
        Err(rr) => logs.error(rr),
        Ok(Decision::Pass) => {}
        // TODO, check for monitor
        Ok(a) => return (a, tags),
    }
    logs.debug("flow checks done");

    if let Some(dec) = mgh.as_ref().and_then(|gh| {
        reqinfo
            .rinfo
            .qinfo
            .uri
            .as_ref()
            .and_then(|uri| challenge_phase02(gh, uri, &reqinfo.headers))
    }) {
        // TODO, check for monitor
        return (dec, tags);
    }

    // limit checks
    let limit_check = limit_check(logs, &urlmap.name, &reqinfo, &urlmap.limits, &mut tags);
    if let Decision::Action(_) = limit_check {
        // limit hit!
        return (limit_check, tags);
    }
    logs.debug(format!("limit checks done ({} limits)", urlmap.limits.len()));

    // store the check_acl result here
    let blockcode: Option<(i32, Vec<String>)> = match check_acl(&tags, &urlmap.acl_profile) {
        AclResult::Bypass(dec) => {
            if dec.allowed {
                logs.debug("ACL bypass detected");
                return (Decision::Pass, tags);
            } else {
                logs.debug("ACL force block detected");
                Some((0, dec.tags))
            }
        }
        // human blocked, always block, even if it is a bot
        AclResult::Match(BotHuman {
            bot: _,
            human: Some(AclDecision {
                allowed: false,
                tags: dtags,
            }),
        }) => {
            logs.debug("ACL human block detected");
            Some((5, dtags))
        }
        // robot blocked, should be challenged
        AclResult::Match(BotHuman {
            bot: Some(AclDecision {
                allowed: false,
                tags: dtags,
            }),
            human: _,
        }) => {
            // if grasshopper is available, run these tests
            if let Some(gh) = mgh {
                if !challenge_verified(&gh, &reqinfo) {
                    logs.debug("ACL challenge detected");
                    match reqinfo.headers.get("user-agent") {
                        None => Some((3, dtags)),
                        Some(ua) => return (challenge_phase01(&gh, ua, dtags), tags),
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    };
    logs.debug(format!("ACL checks done {:?}", blockcode));

    // if the acl is active, and we had a block result, immediately block
    if urlmap.acl_active {
        if let Some((cde, tgs)) = blockcode {
            return (acl_block(true, cde, &tgs), tags);
        }
    }

    // otherwise, run waf_check
    let waf_result = match HSDB.read() {
        Ok(rd) => waf_check(&reqinfo, &urlmap.waf_profile, rd),
        Err(rr) => {
            logs.error(format!("Could not get lock on HSDB: {}", rr));
            Ok(())
        }
    };
    logs.debug("WAF checks done");

    (
        match waf_result {
            Ok(()) => {
                // if waf was ok, but we had an acl decision, return the monitored acl decision for logged purposes
                if let Some((cde, tgs)) = blockcode {
                    acl_block(false, cde, &tgs)
                } else {
                    Decision::Pass
                }
            }
            Err(wb) => {
                let mut action = wb.to_action();
                action.block_mode = urlmap.waf_active;
                Decision::Action(action)
            }
        },
        tags,
    )
}

/// wraps a result into a go-like pair
#[allow(clippy::unnecessary_wraps)]
fn lua_result<R>(v: anyhow::Result<R>) -> LuaResult<(Option<R>, Option<String>)> {
    Ok(match v {
        Ok(x) => (Some(x), None),
        Err(rr) => (None, Some(format!("{}", rr))),
    })
}

/// wraps a result into a go-like pair, PRINTING LOGS
#[allow(clippy::unnecessary_wraps)]
fn lua_log_result<F, R>(f: F) -> LuaResult<(Option<R>, Option<String>)>
where
    F: FnOnce(&mut Logs) -> anyhow::Result<R>,
{
    let mut logs = Logs::default();
    let v = f(&mut logs);
    for log in logs.logs {
        println!("{}", log.to_string());
    }
    Ok(match v {
        Ok(x) => (Some(x), None),
        Err(rr) => (None, Some(format!("{}", rr))),
    })
}

/// runs the passed function, assuming the argument is a string
fn with_str<F, R>(lua: &Lua, session_id: LuaValue, f: F) -> anyhow::Result<R>
where
    F: FnOnce(&str) -> anyhow::Result<R>,
{
    let decoded: String = FromLua::from_lua(session_id, lua).map_err(|rr| anyhow!("{}", rr))?;
    f(&decoded)
}

/// runs the underlying string using function, catching mlua errors
fn wrap_session<F, R>(lua: &Lua, session_id: LuaValue, f: F) -> LuaResult<(Option<R>, Option<String>)>
where
    F: FnOnce(&str) -> anyhow::Result<R>,
{
    lua_result(with_str(lua, session_id, f))
}

/// runs the underlying string using, json returning, function, catching mlua errors
fn wrap_session_json<F, R: serde::Serialize>(
    lua: &Lua,
    session_id: LuaValue,
    f: F,
) -> LuaResult<(Option<String>, Option<String>)>
where
    F: FnOnce(&mut Logs, &str) -> anyhow::Result<R>,
{
    lua_log_result(|logs| {
        with_str(lua, session_id, |s| {
            f(logs, s).and_then(|r| serde_json::to_string(&r).map_err(|rr| anyhow!("{}", rr)))
        })
    })
}

/// runs the underlying string using, Decision returning, function, catching mlua errors
fn wrap_session_decision<F>(lua: &Lua, session_id: LuaValue, f: F) -> LuaResult<(Option<String>, Option<String>)>
where
    F: FnOnce(&str) -> anyhow::Result<Decision>,
{
    lua_result(with_str(lua, session_id, |s| {
        f(s).and_then(|r| session::session_serialize_request_map(s).map(|v| r.to_json_raw(v, Logs::default())))
    }))
}

#[mlua::lua_module]
fn curiefense(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    // end-to-end inspection
    exports.set("inspect_request", lua.create_function(lua_inspect_request)?)?;

    // session functions
    exports.set(
        "init_config",
        lua.create_function(|_: &Lua, _: ()| Ok(session::init_config()))?,
    )?;
    exports.set(
        "session_init",
        lua.create_function(|lua: &Lua, encoded_request_map: LuaValue| {
            wrap_session(lua, encoded_request_map, session::session_init)
        })?,
    )?;
    exports.set(
        "session_clean",
        lua.create_function(|lua: &Lua, session_id: LuaValue| {
            wrap_session(lua, session_id, |s| session::clean_session(s).map(|()| true))
        })?,
    )?;
    exports.set(
        "session_serialize_request_map",
        lua.create_function(|lua: &Lua, session_id: LuaValue| {
            wrap_session_json(lua, session_id, |_, uuid| session::session_serialize_request_map(uuid))
        })?,
    )?;
    exports.set(
        "session_match_urlmap",
        lua.create_function(|lua: &Lua, session_id: LuaValue| {
            wrap_session_json(lua, session_id, |_, uuid| session::session_match_urlmap(uuid))
        })?,
    )?;
    exports.set(
        "session_tag_request",
        lua.create_function(|lua: &Lua, session_id: LuaValue| {
            wrap_session_json(lua, session_id, |_, uuid| session::session_tag_request(uuid))
        })?,
    )?;
    exports.set(
        "session_limit_check",
        lua.create_function(|lua: &Lua, session_id: LuaValue| {
            wrap_session_decision(lua, session_id, session::session_limit_check)
        })?,
    )?;
    exports.set(
        "session_acl_check",
        lua.create_function(|lua: &Lua, session_id: LuaValue| {
            wrap_session_json(lua, session_id, |_, uuid| session::session_acl_check(uuid))
        })?,
    )?;
    exports.set(
        "session_waf_check",
        lua.create_function(|lua: &Lua, session_id: LuaValue| {
            wrap_session_decision(lua, session_id, session::session_waf_check)
        })?,
    )?;
    exports.set(
        "session_flow_check",
        lua.create_function(|lua: &Lua, session_id: LuaValue| {
            wrap_session_decision(lua, session_id, session::session_flow_check)
        })?,
    )?;

    // iptools exports
    exports.set("new_ip_set", lua.create_function(iptools::new_ip_set)?)?;
    exports.set("new_sig_set", lua.create_function(iptools::new_sig_set)?)?;
    exports.set("new_geoipdb", lua.create_function(iptools::new_geoipdb)?)?;
    exports.set("modhash", lua.create_function(iptools::modhash)?)?;
    exports.set("iptonum", lua.create_function(iptools::iptonum)?)?;
    exports.set("decodeurl", lua.create_function(iptools::decodeurl)?)?;
    exports.set("encodeurl", lua.create_function(iptools::encodeurl)?)?;
    exports.set("test_regex", lua.create_function(iptools::test_regex)?)?;

    Ok(exports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_load() {
        let mut logs = Logs::default();
        let cfg = with_config("../../config", &mut logs, |_, c| c.clone());
        for r in logs.logs.iter() {
            println!("{}", r.to_string());
        }
        assert!(cfg.is_some());
        assert!(logs.logs.len() == 2);
        assert!(logs.logs[0].message.to_string().contains("profiling-lists.json"));
        assert!(logs.logs[1].message.to_string().contains("rbz-cloud-platforms"));
    }
}
