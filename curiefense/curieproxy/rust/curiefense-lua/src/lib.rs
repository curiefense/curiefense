pub mod userdata;

use curiefense::analyze::analyze_finish;
use curiefense::analyze::analyze_flows;
use curiefense::analyze::analyze_init;
use curiefense::analyze::APhase1;
use curiefense::analyze::APhase2I;
use curiefense::analyze::APhase2O;
use curiefense::analyze::APhase3;
use curiefense::analyze::CfRulesArg;
use curiefense::analyze::InitResult;
use curiefense::config::reload_config;
use curiefense::grasshopper::DynGrasshopper;
use curiefense::grasshopper::GHMode;
use curiefense::grasshopper::GHQuery;
use curiefense::grasshopper::GHResponse;
use curiefense::grasshopper::Grasshopper;
use curiefense::grasshopper::PrecisionLevel;
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
    humanity: PrecisionLevel,
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
/// * humanity, PrecisionLevel, only used for the test functions
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
    let shumanity: Option<String> = match FromLua::from_lua(vhumanity, lua) {
        Err(rr) => return Err(format!("Could not convert the humanity argument: {}", rr)),
        Ok(h) => h,
    };
    let humanity = match shumanity.as_deref() {
        Some("active") => PrecisionLevel::Active,
        Some("passive") => PrecisionLevel::Passive,
        Some("interactive") => PrecisionLevel::Interactive,
        Some("mobileSdk") => PrecisionLevel::MobileSdk,
        Some("invalid") => PrecisionLevel::Invalid,
        None => PrecisionLevel::Invalid,
        Some(x) => return Err(format!("Invalid humanity precision level {}", x)),
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
                lua_args.meta,
                lua_args.headers,
                lua_args.lua_body.as_ref().map(|b| b.as_bytes()),
                lua_args.str_ip,
                Some(grasshopper),
                lua_args.secpolid,
                lua_args.plugins,
            )
            .map(|r| r.mask());
            Ok(LuaInspectionResult(res))
        }
        Err(rr) => Ok(LuaInspectionResult(Err(rr))),
    }
}

/// ****************************************
/// Lua interface for the "async dialog" API
/// ****************************************
fn lua_inspect_init(lua: &Lua, args: LuaTable) -> LuaResult<LInitResult<APhase1>> {
    match lua_convert_args(lua, args) {
        Ok(lua_args) => {
            let grasshopper = &DynGrasshopper {};
            let res = inspect_init(
                lua_args.loglevel,
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

fn lua_inspect_flows(lua: &Lua, args: (LuaValue, LuaValue)) -> LuaResult<LInitResult<APhase2I>> {
    let (lpr1, lflow_results) = args;
    let pr1: LInitResult<APhase1> = FromLua::from_lua(lpr1, lua)?;
    let lflow_results: Vec<LuaFlowResult> = FromLua::from_lua(lflow_results, lua)?;
    let flow_results = lflow_results.into_iter().map(|lf| lf.0).collect();
    Ok(match pr1 {
        LInitResult::P0Result(r) => LInitResult::P0Result(r),
        LInitResult::P0Error(r) => LInitResult::P0Error(r),
        LInitResult::P1(mut logs, bp1) => {
            let p2o = APhase2O::from_phase1(*bp1, flow_results);
            let p2i = analyze_flows(&mut logs, p2o);
            LInitResult::P1(logs, Box::new(p2i))
        }
    })
}

/// This is the processing function, that will an analysis result
fn lua_inspect_process(lua: &Lua, args: (LuaValue, LuaValue)) -> LuaResult<LuaInspectionResult> {
    let (lpred, llimit_results) = args;
    let lerr = |msg| Ok(LuaInspectionResult(Err(msg)));
    let pred: LInitResult<APhase2I> = match FromLua::from_lua(lpred, lua) {
        Err(rr) => return lerr(format!("Could not convert the pred(2I) argument: {}", rr)),
        Ok(m) => m,
    };
    let rlimit_results: Result<Vec<LuaLimitResult>, mlua::Error> = FromLua::from_lua(llimit_results, lua);
    let limit_results = match rlimit_results {
        Err(rr) => return lerr(format!("Could not convert the limit_result argument: {}", rr)),
        Ok(m) => m.into_iter().map(|n| n.0).collect(),
    };

    let (mut logs, p2) = match pred {
        LInitResult::P0Result(_) => {
            return lerr("The first parameter is an inspection result, and should not have been used here!".to_string())
        }
        LInitResult::P0Error(rr) => return lerr(format!("The first parameter is an error: {}", rr)),
        LInitResult::P1(logs, p2) => (logs, p2),
    };
    let p3 = APhase3::from_phase2(*p2, limit_results);
    let grasshopper = &DynGrasshopper {};
    let res = analyze_finish(&mut logs, Some(grasshopper), CfRulesArg::Global, p3);
    Ok(LuaInspectionResult(
        Ok(InspectionResult::from_analyze(logs, res).mask()),
    ))
}

fn lua_reload_conf(lua: &Lua, args: (LuaValue, LuaValue)) -> LuaResult<Option<String>> {
    let (lfilename, lconfigpath) = args;

    let raw_files: Option<String> = match FromLua::from_lua(lfilename, lua) {
        Err(rr) => return Ok(Some(format!("Could not convert the files arguments: {}", rr))),
        Ok(raw) => raw,
    };

    let files: Vec<String> = match raw_files {
        None => vec![],
        Some(raw_files) => match serde_json::from_str(&raw_files) {
            Err(rr) => {
                return Ok(Some(format!(
                    "Could not parse the files argument as valid json array: {}",
                    rr
                )))
            }
            Ok(files) => files,
        },
    };
    let configpath: String = match lconfigpath {
        LuaNil => String::from("/cf-config/current/config"),
        v => match FromLua::from_lua(v, lua) {
            Err(rr) => return Ok(Some(format!("Could not parse configpath argument to string: {}", rr))),
            Ok(path) => path,
        },
    };

    reload_config(&configpath, files);
    Ok(None)
}

struct DummyGrasshopper {
    humanity: PrecisionLevel,
}

impl Grasshopper for DummyGrasshopper {
    fn is_human(&self, _input: GHQuery) -> Result<PrecisionLevel, String> {
        Ok(self.humanity)
    }

    fn verify_challenge(&self, _headers: HashMap<&str, &str>) -> Result<String, String> {
        if self.humanity == PrecisionLevel::Invalid {
            Err("Bad".to_string())
        } else {
            Ok("OK".to_string())
        }
    }

    fn init_challenge(&self, _input: GHQuery, _mode: GHMode) -> Result<GHResponse, String> {
        Ok(GHResponse::invalid())
    }

    fn should_provide_app_sig(&self, _headers: HashMap<&str, &str>) -> Result<GHResponse, String> {
        Ok(GHResponse::invalid())
    }

    fn handle_bio_report(&self, _input: GHQuery, _precision_leve: PrecisionLevel) -> Result<GHResponse, String> {
        Err("not implemented".into())
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
                humanity: lua_args.humanity,
            };
            let res = inspect_request(
                lua_args.meta,
                lua_args.headers,
                lua_args.lua_body.as_ref().map(|b| b.as_bytes()),
                lua_args.str_ip,
                Some(&gh),
                lua_args.secpolid,
                lua_args.plugins,
            )
            .map(|r| r.mask());
            Ok(LuaInspectionResult(res))
        }
        Err(rr) => Ok(LuaInspectionResult(Err(rr))),
    }
}

/// Rust-native inspection top level function
#[allow(clippy::too_many_arguments)]
fn inspect_request<GH: Grasshopper>(
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
    let dec = inspect_generic_request_map(grasshopper, raw, &mut logs, selected_secpol.as_deref(), plugins);

    Ok(InspectionResult::from_analyze(logs, dec))
}
/// Rust-native functions for the dialog system
#[allow(clippy::too_many_arguments)]
fn inspect_init<GH: Grasshopper>(
    loglevel: LogLevel,
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

    let p0 = match inspect_generic_request_map_init(grasshopper, raw, &mut logs, selected_secpol.as_deref(), plugins) {
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
    exports.set("inspect_request_flows", lua.create_function(lua_inspect_flows)?)?;
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
        reload_config("../../cf-config/", Vec::new());

        let cfg = with_config(&mut logs, |_, c| c.clone());
        if cfg.is_some() {
            match logs.logs.len() {
                4 => {
                    assert!(logs.logs[0].message.to_string().contains("CFGLOAD logs start"));
                    assert!(logs.logs[1]
                        .message
                        .to_string()
                        .contains("When loading manifest.json: No such file or directory"));
                    assert!(logs.logs[2].message.to_string().contains("Loaded profile"));
                    assert!(logs.logs[3].message.to_string().contains("CFGLOAD logs end"));
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
