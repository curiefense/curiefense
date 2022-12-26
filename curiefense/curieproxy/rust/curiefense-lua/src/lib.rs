pub mod userdata;

use curiefense::analyze::analyze_finish;
use curiefense::analyze::analyze_init;
use curiefense::analyze::APhase2;
use curiefense::analyze::CfRulesArg;
use curiefense::analyze::InitResult;
use curiefense::config::reload_config;
use curiefense::grasshopper::DynGrasshopper;
use curiefense::grasshopper::Grasshopper;
use curiefense::inspect_generic_request_map;
use curiefense::inspect_generic_request_map_init;
use curiefense::interface::aggregator::aggregated_values_block;
use curiefense::logs::LogLevel;
use curiefense::logs::Logs;
use curiefense::utils::RequestMeta;
use curiefense::utils::{InspectionResult, RawRequest};
use mlua::prelude::*;
use mlua::FromLua;
use std::collections::HashMap;
use userdata::LInitResult;
use userdata::LuaFlowResult;
use userdata::LuaLimitResult;

use userdata::LuaInspectionResult;

// ******************************************
// FULL CHECKS
// ******************************************

struct LuaArgs<'l> {
    meta: HashMap<String, String>,
    headers: HashMap<String, String>,
    lua_body: Option<LuaString<'l>>,
    str_ip: String,
    loglevel: LogLevel,
    secpolid: Option<String>,
    humanity: Option<bool>,
    configpath: String,
    plugins: HashMap<String, String>,
}

/// Lua function arguments:
///
/// All arguments are placed into a Lua table, where the keys are:
/// * loglevel, mandatory, can be debug, info, warn or err
/// * meta, table, contains keys "method", "path" and optionally "authority" and "x-request-id"
/// * headers, table
/// * body, optional string
/// * ip, string representation of the IP address
/// * hops, optional number. When set the IP is computed from the x-forwarded-for header, defaulting to the ip argument on failure
/// * secpolid, optional string. When set, bypass hostname matching for security policy selection
/// * configpath, path to the lua configuration files, defaults to /cf-config/current/config
/// * humanity, optional boolean, only used for the test functions
fn lua_convert_args<'l>(lua: &'l Lua, args: LuaTable<'l>) -> Result<LuaArgs<'l>, String> {
    let vloglevel = args.get("loglevel").map_err(|_| "Missing log level".to_string())?;
    let vmeta = args.get("meta").map_err(|_| "Missing meta argument".to_string())?;
    let vheaders = args.get("headers").map_err(|_| "Missing headers".to_string())?;
    let vlua_body = args.get("body").map_err(|_| "Missing body argument".to_string())?;
    let vstr_ip = args.get("ip").map_err(|_| "Missing ip argument".to_string())?;
    let vhops = args.get("hops").map_err(|_| "Missing hops argument".to_string())?;
    let vplugins = args
        .get("plugins")
        .map_err(|_| "Missing plugins argument".to_string())?;
    let vsecpolid = args
        .get("secpolid")
        .map_err(|_| "Missing log level argument".to_string())?;
    let vhumanity = args.get("human").map_err(|_| "Missing human argument".to_string())?;
    let vconfigpath = args.get("configpath").map_err(|_| "Missing config path".to_string())?;
    let loglevel = match String::from_lua(vloglevel, lua) {
        Err(rr) => return Err(format!("Could not convert the loglevel argument: {}", rr)),
        Ok(m) => match m.as_str() {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" | "warning" => LogLevel::Warning,
            "err" | "error" => LogLevel::Error,
            _ => return Err(format!("Invalid log level {}", m)),
        },
    };
    let meta = match FromLua::from_lua(vmeta, lua) {
        Err(rr) => return Err(format!("Could not convert the meta argument: {}", rr)),
        Ok(m) => m,
    };
    let headers = match FromLua::from_lua(vheaders, lua) {
        Err(rr) => return Err(format!("Could not convert the headers argument: {}", rr)),
        Ok(h) => h,
    };
    let lua_body: Option<LuaString> = match FromLua::from_lua(vlua_body, lua) {
        Err(rr) => return Err(format!("Could not convert the body argument: {}", rr)),
        Ok(b) => b,
    };
    let str_ip = match FromLua::from_lua(vstr_ip, lua) {
        Err(rr) => return Err(format!("Could not convert the ip argument: {}", rr)),
        Ok(i) => i,
    };
    let hops = match FromLua::from_lua(vhops, lua) {
        Err(rr) => return Err(format!("Could not convert the hops argument: {}", rr)),
        Ok(i) => i,
    };
    let secpolid = match FromLua::from_lua(vsecpolid, lua) {
        Err(rr) => return Err(format!("Could not convert the hops argument: {}", rr)),
        Ok(i) => i,
    };
    let ip = match hops {
        None => str_ip,
        Some(hops) => curiefense::incremental::extract_ip(hops, &headers).unwrap_or(str_ip),
    };
    let humanity = match FromLua::from_lua(vhumanity, lua) {
        Err(rr) => return Err(format!("Could not convert the humanity argument: {}", rr)),
        Ok(h) => h,
    };
    let configpath: Option<String> = match FromLua::from_lua(vconfigpath, lua) {
        Err(rr) => return Err(format!("Could not convert the config path argument: {}", rr)),
        Ok(p) => p,
    };
    let mplugins: Option<HashMap<String, HashMap<String, String>>> = match FromLua::from_lua(vplugins, lua) {
        Err(rr) => return Err(format!("Could not convert the plugins argument: {}", rr)),
        Ok(p) => p,
    };
    Ok(LuaArgs {
        meta,
        headers,
        lua_body,
        str_ip: ip,
        loglevel,
        secpolid,
        humanity,
        configpath: configpath.unwrap_or_else(|| "/cf-config/current/config".to_string()),
        plugins: mplugins
            .unwrap_or_default()
            .into_iter()
            .flat_map(|(plugin_name, values)| {
                values
                    .into_iter()
                    .map(move |(k, v)| (format!("{}.{}", &plugin_name, k), v))
            })
            .collect(),
    })
}

/// Lua interface to the inspection function
fn lua_inspect_request(lua: &Lua, args: LuaTable) -> LuaResult<LuaInspectionResult> {
    match lua_convert_args(lua, args) {
        Ok(lua_args) => {
            let grasshopper = &DynGrasshopper {};
            let res = inspect_request(
                &lua_args.configpath,
                lua_args.meta,
                lua_args.headers,
                lua_args.lua_body.as_ref().map(|b| b.as_bytes()),
                lua_args.str_ip,
                Some(grasshopper),
                lua_args.secpolid,
                lua_args.plugins,
            );
            Ok(LuaInspectionResult(res))
        }
        Err(rr) => Ok(LuaInspectionResult(Err(rr))),
    }
}

/// ****************************************
/// Lua interface for the "async dialog" API
/// ****************************************
fn lua_inspect_init(lua: &Lua, args: LuaTable) -> LuaResult<LInitResult> {
    match lua_convert_args(lua, args) {
        Ok(lua_args) => {
            let grasshopper = &DynGrasshopper {};
            let res = inspect_init(
                lua_args.loglevel,
                &lua_args.configpath,
                lua_args.meta,
                lua_args.headers,
                lua_args.lua_body.as_ref().map(|b| b.as_bytes()),
                lua_args.str_ip,
                Some(grasshopper),
                lua_args.secpolid,
                lua_args.plugins,
            );
            Ok(match res {
                Ok((r, logs)) => match r {
                    InitResult::Res(r) => LInitResult::P0Result(Box::new(InspectionResult::from_analyze(logs, r))),
                    InitResult::Phase1(p1) => LInitResult::P1(logs, Box::new(p1)),
                },
                Err(s) => LInitResult::P0Error(s),
            })
        }
        Err(rr) => Ok(LInitResult::P0Error(rr)),
    }
}

/// This is the processing function, that will an analysis result
fn lua_inspect_process(lua: &Lua, args: (LuaValue, LuaValue, LuaValue)) -> LuaResult<LuaInspectionResult> {
    let (lpred, lflow_results, llimit_results) = args;
    let lerr = |msg| Ok(LuaInspectionResult(Err(msg)));
    let pred = match FromLua::from_lua(lpred, lua) {
        Err(rr) => return lerr(format!("Could not convert the pred argument: {}", rr)),
        Ok(m) => m,
    };
    let rflow_results: Result<Vec<LuaFlowResult>, mlua::Error> = FromLua::from_lua(lflow_results, lua);
    let flow_results = match rflow_results {
        Err(rr) => return lerr(format!("Could not convert the flow_result argument: {}", rr)),
        Ok(m) => m.into_iter().map(|n| n.0).collect(),
    };
    let rlimit_results: Result<Vec<LuaLimitResult>, mlua::Error> = FromLua::from_lua(llimit_results, lua);
    let limit_results = match rlimit_results {
        Err(rr) => return lerr(format!("Could not convert the limit_result argument: {}", rr)),
        Ok(m) => m.into_iter().map(|n| n.0).collect(),
    };

    let (mut logs, p1) = match pred {
        LInitResult::P0Result(_) => {
            return lerr("The first parameter is an inspection result, and should not have been used here!".to_string())
        }
        LInitResult::P0Error(rr) => return lerr(format!("The first parameter is an error: {}", rr)),
        LInitResult::P1(logs, p1) => (logs, p1),
    };
    let p2 = APhase2::from_phase1(*p1, flow_results, limit_results);
    let grasshopper = &DynGrasshopper {};
    let res = analyze_finish(&mut logs, Some(grasshopper), CfRulesArg::Global, p2);
    Ok(LuaInspectionResult(Ok(InspectionResult::from_analyze(logs, res))))
}

fn lua_reload_conf(lua: &Lua, args: (LuaValue, LuaValue)) -> LuaResult<()> {
    let (lfilename, lconfigpath) = args;

    let raw_files: String = FromLua::from_lua(lfilename, lua).map_err(mlua::Error::external)?;
    let files: Vec<String> = serde_json::from_str(&raw_files).map_err(mlua::Error::external)?;
    let configpath: String = match lconfigpath {
        LuaNil => String::from("/cf-config/current/config"),
        v => FromLua::from_lua(v, lua).map_err(mlua::Error::external)?,
    };

    reload_config(&configpath, files);
    Ok(())
}

struct DummyGrasshopper {
    humanity: bool,
}

impl Grasshopper for DummyGrasshopper {
    fn js_app(&self) -> Option<std::string::String> {
        None
    }
    fn js_bio(&self) -> Option<std::string::String> {
        None
    }
    fn parse_rbzid(&self, _: &str, _: &str) -> Option<bool> {
        Some(self.humanity)
    }
    fn gen_new_seed(&self, _: &str) -> Option<std::string::String> {
        None
    }
    fn verify_workproof(&self, _: &str, _: &str) -> Option<std::string::String> {
        Some("ok".into())
    }
}

/// Lua TEST interface to the inspection function
/// allows settings the Grasshopper result!
#[allow(clippy::type_complexity)]
#[allow(clippy::unnecessary_wraps)]
fn lua_test_inspect_request(lua: &Lua, args: LuaTable) -> LuaResult<LuaInspectionResult> {
    match lua_convert_args(lua, args) {
        Ok(lua_args) => {
            let gh = DummyGrasshopper {
                humanity: lua_args.humanity.unwrap_or(false),
            };
            let res = inspect_request(
                &lua_args.configpath,
                lua_args.meta,
                lua_args.headers,
                lua_args.lua_body.as_ref().map(|b| b.as_bytes()),
                lua_args.str_ip,
                Some(&gh),
                lua_args.secpolid,
                lua_args.plugins,
            );
            Ok(LuaInspectionResult(res))
        }
        Err(rr) => Ok(LuaInspectionResult(Err(rr))),
    }
}

/// Rust-native inspection top level function
#[allow(clippy::too_many_arguments)]
fn inspect_request<GH: Grasshopper>(
    configpath: &str,
    meta: HashMap<String, String>,
    headers: HashMap<String, String>,
    mbody: Option<&[u8]>,
    ip: String,
    grasshopper: Option<&GH>,
    selected_secpol: Option<String>,
    plugins: HashMap<String, String>,
) -> Result<InspectionResult, String> {
    let mut logs = Logs::default();
    logs.debug("Inspection init");
    let rmeta: RequestMeta = RequestMeta::from_map(meta)?;

    let raw = RawRequest {
        ipstr: ip,
        meta: rmeta,
        headers,
        mbody,
    };
    let dec = inspect_generic_request_map(
        configpath,
        grasshopper,
        raw,
        &mut logs,
        selected_secpol.as_deref(),
        plugins,
    );

    Ok(InspectionResult::from_analyze(logs, dec))
}
/// Rust-native functions for the dialog system
#[allow(clippy::too_many_arguments)]
fn inspect_init<GH: Grasshopper>(
    loglevel: LogLevel,
    configpath: &str,
    meta: HashMap<String, String>,
    headers: HashMap<String, String>,
    mbody: Option<&[u8]>,
    ip: String,
    grasshopper: Option<&GH>,
    selected_secpol: Option<String>,
    plugins: HashMap<String, String>,
) -> Result<(InitResult, Logs), String> {
    let mut logs = Logs::new(loglevel);
    logs.debug("Inspection init");
    let rmeta: RequestMeta = RequestMeta::from_map(meta)?;

    let raw = RawRequest {
        ipstr: ip,
        meta: rmeta,
        headers,
        mbody,
    };

    let p0 = match inspect_generic_request_map_init(
        configpath,
        grasshopper,
        raw,
        &mut logs,
        selected_secpol.as_deref(),
        plugins,
    ) {
        Err(res) => return Ok((InitResult::Res(res), logs)),
        Ok(p0) => p0,
    };

    let r = analyze_init(&mut logs, grasshopper, p0);
    Ok((r, logs))
}

pub struct LuaInitResult {}

#[mlua::lua_module]
fn curiefense(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    // end-to-end inspection
    exports.set("inspect_request", lua.create_function(lua_inspect_request)?)?;
    exports.set("inspect_request_init", lua.create_function(lua_inspect_init)?)?;
    exports.set("inspect_request_process", lua.create_function(lua_inspect_process)?)?;
    exports.set(
        "aggregated_values",
        lua.create_function(|_, ()| Ok(aggregated_values_block()))?,
    )?;
    exports.set("lua_reload_conf", lua.create_function(lua_reload_conf)?)?;
    // end-to-end inspection (test)
    exports.set("test_inspect_request", lua.create_function(lua_test_inspect_request)?)?;

    Ok(exports)
}

#[cfg(test)]
mod tests {
    use super::*;
    use curiefense::config::with_config;

    #[test]
    fn config_load() {
        let mut logs = Logs::default();
        let cfg = with_config("../../cf-config", &mut logs, |_, c| c.clone());
        if cfg.is_some() {
            match logs.logs.len() {
                5 => {
                    assert!(logs.logs[0].message.to_string().contains("CFGLOAD logs start"));
                    assert!(logs.logs[2].message.to_string().contains("manifest.json"));
                    assert!(logs.logs[3].message.to_string().contains("Loaded profile"));
                    assert!(logs.logs[4].message.to_string().contains("CFGLOAD logs end"));
                }
                13 => {
                    assert!(logs.logs[2]
                        .message
                        .to_string()
                        .contains("../../cf-config: No such file or directory"))
                }
                n => {
                    for r in logs.logs.iter() {
                        eprintln!("{}", r);
                    }
                    panic!("Invalid amount of logs: {}", n);
                }
            }
        }
    }
}
