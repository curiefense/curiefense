use curiefense::config::reload_config;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::collections::HashMap;

use curiefense::grasshopper::DynGrasshopper;
use curiefense::inspect_generic_request_map;
use curiefense::logs::{LogLevel, Logs};
use curiefense::utils::RequestMeta;
use curiefense::utils::{InspectionResult, RawRequest};

#[pyfunction]
#[pyo3(name = "reload_config")]
fn py_reload_config(configpath: String, files: Vec<String>) {
    reload_config(&configpath, files);
}

#[pyfunction]
#[pyo3(name = "inspect_request")]
fn py_inspect_request(
    loglevel: String,
    meta: HashMap<String, String>,
    headers: HashMap<String, String>,
    mbody: Option<&[u8]>,
    ip: String,
    plugins: Option<HashMap<String, String>>,
) -> PyResult<(String, Vec<u8>)> {
    let real_loglevel = match loglevel.as_str() {
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" | "warning" => LogLevel::Warning,
        "err" | "error" => LogLevel::Error,
        _ => return Err(PyTypeError::new_err(format!("Can't recognize log level: {}", loglevel))),
    };
    let mut logs = Logs::new(real_loglevel);
    logs.debug("Inspection init");
    let rmeta: RequestMeta = RequestMeta::from_map(meta).map_err(PyTypeError::new_err)?;

    let raw = RawRequest {
        ipstr: ip,
        meta: rmeta,
        headers,
        mbody,
    };

    let grasshopper = DynGrasshopper {};
    let dec = inspect_generic_request_map(Some(&grasshopper), raw, &mut logs, None, plugins.unwrap_or_default());
    let res = InspectionResult {
        decision: dec.decision,
        tags: Some(dec.tags),
        logs,
        err: None,
        rinfo: Some(dec.rinfo),
        stats: dec.stats,
    }
    .mask();
    let response = res.decision.response_json();
    let request_map = res.log_json_block(HashMap::new());
    let merr = res.err;
    match merr {
        Some(rr) => Err(PyTypeError::new_err(rr)),
        None => Ok((response, request_map)),
    }
}

#[pyclass]
#[derive(Eq, PartialEq, Debug)]
struct MatchResult {
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize,
}

#[pyfunction]
fn rust_match(pattern: String, mmatch: Option<&str>) -> PyResult<Vec<MatchResult>> {
    let re = regex::RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .map_err(|rr| PyTypeError::new_err(rr.to_string()))?;
    if let Some(to_match) = mmatch {
        Ok(re
            .find_iter(to_match)
            .map(|m| MatchResult {
                start: m.start(),
                end: m.end(),
            })
            .collect())
    } else {
        Ok(Vec::new())
    }
}

#[pyfunction]
fn hyperscan_match(pattern: String, mmatch: Option<&str>) -> PyResult<Vec<MatchResult>> {
    use hyperscan::prelude::*;
    use hyperscan::BlockMode;
    let db: Database<BlockMode> =
        Database::compile(&pattern, CompileFlags::empty(), None).map_err(|rr| PyTypeError::new_err(rr.to_string()))?;
    let scratch = db.alloc_scratch().map_err(|rr| PyTypeError::new_err(rr.to_string()))?;

    if let Some(to_match) = mmatch {
        let mut out = Vec::new();
        db.scan(to_match, &scratch, |_, from, to, _| {
            out.push(MatchResult {
                start: from as usize,
                end: to as usize,
            });
            Matching::Continue
        })
        .map_err(|rr| PyTypeError::new_err(rr.to_string()))?;
        Ok(out)
    } else {
        Ok(Vec::new())
    }
}

#[pyfunction]
fn aggregated_data() -> PyResult<String> {
    Ok(curiefense::interface::aggregator::aggregated_values_block())
}

#[pymodule]
fn curiefense(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_inspect_request, m)?)?;
    m.add_function(wrap_pyfunction!(rust_match, m)?)?;
    m.add_function(wrap_pyfunction!(hyperscan_match, m)?)?;
    m.add_function(wrap_pyfunction!(aggregated_data, m)?)?;
    Ok(())
}
