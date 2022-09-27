use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::collections::HashMap;

use curiefense::grasshopper::DynGrasshopper;
use curiefense::inspect_generic_request_map;
use curiefense::logs::Logs;
use curiefense::utils::RequestMeta;
use curiefense::utils::{InspectionResult, RawRequest};

#[pyfunction]
#[pyo3(name = "inspect_request")]
fn py_inspect_request(
    configpath: &str,
    meta: HashMap<String, String>,
    headers: HashMap<String, String>,
    mbody: Option<&[u8]>,
    ip: String,
) -> PyResult<(String, String)> {
    let mut logs = Logs::default();
    logs.debug("Inspection init");
    let rmeta: RequestMeta = RequestMeta::from_map(meta).map_err(PyTypeError::new_err)?;

    let raw = RawRequest {
        ipstr: ip,
        meta: rmeta,
        headers,
        mbody,
    };

    let grasshopper = DynGrasshopper {};
    let dec = inspect_generic_request_map(configpath, Some(&grasshopper), raw, &mut logs);
    let res = InspectionResult {
        decision: dec.decision,
        tags: Some(dec.tags),
        logs,
        err: None,
        rinfo: Some(dec.rinfo),
        stats: dec.stats,
    };
    let response = res.decision.response_json();
    let request_map = res.log_json();
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
fn rust_match(pattern: &str, mmatch: Option<&str>) -> PyResult<Vec<MatchResult>> {
    let re = regex::Regex::new(pattern).map_err(|rr| PyTypeError::new_err(rr.to_string()))?;
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
fn hyperscan_match(pattern: &str, mmatch: Option<&str>) -> PyResult<Vec<MatchResult>> {
    use hyperscan::prelude::*;
    use hyperscan::BlockMode;
    let db: Database<BlockMode> =
        Database::compile(pattern, CompileFlags::empty(), None).map_err(|rr| PyTypeError::new_err(rr.to_string()))?;
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

#[pymodule]
fn curiefense(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_inspect_request, m)?)?;
    m.add_function(wrap_pyfunction!(rust_match, m)?)?;
    m.add_function(wrap_pyfunction!(hyperscan_match, m)?)?;
    Ok(())
}
