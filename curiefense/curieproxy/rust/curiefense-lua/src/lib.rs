mod iptools;
mod lua;

use crate::lua::Luagrasshopper;

use anyhow::anyhow;
use curiefense::interface::Tags;
use curiefense::session::update_tags;
use curiefense::session::JRequestMap;
use curiefense::utils::RequestMeta;
use mlua::prelude::*;
use std::collections::HashMap;

use curiefense::inspect_generic_request_map;
use curiefense::interface::{Decision, Grasshopper};
use curiefense::logs::Logs;
use curiefense::session;
use curiefense::utils::{map_request, InspectionResult};

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
    use curiefense::config::with_config;

    #[test]
    fn config_load() {
        let mut logs = Logs::default();
        let cfg = with_config("../../config", &mut logs, |_, c| c.clone());
        if cfg.is_some() {
            match logs.logs.len() {
                3 => {
                    assert!(logs.logs[0].message.to_string().contains("CFGLOAD"));
                    assert!(logs.logs[1].message.to_string().contains("profiling-lists.json"));
                    assert!(logs.logs[2].message.to_string().contains("rbz-cloud-platforms"));
                }
                10 => {
                    assert!(logs.logs[0]
                        .message
                        .to_string()
                        .contains("../../config: No such file or directory"))
                }
                n => {
                    for r in logs.logs.iter() {
                        println!("{}", r.to_string());
                    }
                    panic!("Invalid amount of logs: {}", n);
                }
            }
        }
    }
}
