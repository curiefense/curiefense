use serde::{Deserialize, Serialize};

use crate::interface::BlockReason;
use crate::logs::Logs;
use crate::utils::RequestInfo;
use crate::{Action, ActionType, Decision};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum PrecisionLevel {
    Active,
    Passive,
    Interactive,
    MobileSdk,
    Emulator,
    Invalid,
}

impl PrecisionLevel {
    pub fn is_human(&self) -> bool {
        (*self != PrecisionLevel::Invalid) && (*self != PrecisionLevel::Emulator)
    }

    pub fn is_interactive(&self) -> bool {
        (*self == PrecisionLevel::Interactive) || (*self == PrecisionLevel::MobileSdk)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GHMode {
    Passive,
    Active,
    Interactive,
}

#[derive(Serialize, Debug)]
pub struct GHQuery<'t> {
    pub headers: HashMap<&'t str, &'t str>,
    pub cookies: HashMap<&'t str, &'t str>,
    pub ip: &'t str,
    pub protocol: &'t str,
}

#[derive(Serialize, Debug)]
pub struct GHBioQuery<'t> {
    pub headers: HashMap<&'t str, &'t str>,
    pub cookies: HashMap<&'t str, &'t str>,
    //todo add args and use this struct
}

#[derive(Deserialize, Debug, Clone)]
pub struct GHResponse {
    pub precision_level: PrecisionLevel,
    pub str_response: String,
    pub headers: HashMap<String, String>,
    status_code: u32,
}

impl GHResponse {
    pub fn invalid() -> Self {
        Self {
            precision_level: PrecisionLevel::Invalid,
            str_response: "invalid".to_string(),
            headers: HashMap::new(),
            status_code: 200,
        }
    }
}

pub trait Grasshopper {
    fn is_human(&self, input: GHQuery) -> Result<PrecisionLevel, String>;
    fn init_challenge(&self, input: GHQuery, mode: GHMode) -> Result<GHResponse, String>;
    fn verify_challenge(&self, headers: HashMap<&str, &str>) -> Result<String, String>;
    fn should_provide_app_sig(&self, headers: HashMap<&str, &str>) -> Result<GHResponse, String>;
    fn handle_bio_report(&self, input: GHQuery, precision_level: PrecisionLevel) -> Result<GHResponse, String>;
}

mod imported {
    use super::{GHMode, PrecisionLevel};
    use std::os::raw::c_char;
    extern "C" {
        pub fn is_human(
            c_input_data: *const c_char,
            success: *mut bool,
            precision_level: *mut PrecisionLevel,
        ) -> *mut c_char;
        pub fn init_challenge(c_input_data: *const c_char, mode: GHMode, success: *mut bool) -> *mut c_char;
        pub fn verify_challenge(c_headers: *const c_char, success: *mut bool) -> *mut c_char;
        pub fn should_provide_app_sig(c_headers: *const c_char, success: *mut bool) -> *mut c_char;
        pub fn handle_bio_report(
            c_input_data: *const c_char,
            precision_level: PrecisionLevel,
            success: *mut bool,
        ) -> *mut c_char;
        pub fn free_string(s: *mut c_char);
    }
}

pub struct DummyGrasshopper {}

// use this when grasshopper can't be used
impl Grasshopper for DummyGrasshopper {
    fn should_provide_app_sig(&self, _headers: HashMap<&str, &str>) -> Result<GHResponse, String> {
        Err("not implemented".into())
    }

    fn verify_challenge(&self, _headers: HashMap<&str, &str>) -> Result<String, String> {
        Err("not implemented".into())
    }

    fn init_challenge(&self, _input: GHQuery, _mode: GHMode) -> Result<GHResponse, String> {
        Err("not implemented".into())
    }

    fn is_human(&self, _input: GHQuery) -> Result<PrecisionLevel, String> {
        Err("not implemented".into())
    }

    fn handle_bio_report(&self, _input: GHQuery, _precision_leve: PrecisionLevel) -> Result<GHResponse, String> {
        Err("not implemented".into())
    }
}

#[derive(Clone)]
pub struct DynGrasshopper {}

impl Grasshopper for DynGrasshopper {
    fn is_human(&self, input: GHQuery) -> Result<PrecisionLevel, String> {
        unsafe {
            let encoded_input = serde_json::to_vec(&input).map_err(|rr| rr.to_string())?;
            let cinput =
                CString::new(encoded_input).map_err(|_| "null character in JSON encoded string?!?".to_string())?;
            let mut success = false;
            let mut precision_level = PrecisionLevel::Invalid;
            let r = imported::is_human(cinput.as_ptr(), &mut success, &mut precision_level);
            if success {
                if r.is_null() {
                    Ok(precision_level)
                } else {
                    Err("Grasshopper unexpectedly returned a non null pointer on success!".to_string())
                }
            } else {
                let cstr = CStr::from_ptr(r);
                let o = cstr.to_string_lossy().to_string();
                imported::free_string(r);
                Err(o)
            }
        }
    }

    fn handle_bio_report(&self, input: GHQuery, precision_level: PrecisionLevel) -> Result<GHResponse, String> {
        unsafe {
            let encoded_input = serde_json::to_vec(&input).map_err(|rr| rr.to_string())?;
            let cinput =
                CString::new(encoded_input).map_err(|_| "null character in JSON encoded string?!?".to_string())?;
            let mut success = false;
            let r = imported::handle_bio_report(cinput.as_ptr(), precision_level, &mut success);
            let cstr = CStr::from_ptr(r);
            if success {
                let reply: GHResponse = serde_json::from_slice(cstr.to_bytes()).unwrap();
                imported::free_string(r);
                Ok(reply)
            } else {
                let o = cstr.to_string_lossy().to_string();
                imported::free_string(r);
                Err(o)
            }
        }
    }

    fn init_challenge(&self, input: GHQuery, mode: GHMode) -> Result<GHResponse, String> {
        unsafe {
            let encoded_input = serde_json::to_vec(&input).map_err(|rr| rr.to_string())?;
            let cinput =
                CString::new(encoded_input).map_err(|_| "null character in JSON encoded string?!?".to_string())?;
            let mut success = false;
            let r = imported::init_challenge(cinput.as_ptr(), mode, &mut success);
            let cstr = CStr::from_ptr(r);
            if success {
                let reply: GHResponse = serde_json::from_slice(cstr.to_bytes()).unwrap();
                imported::free_string(r);
                Ok(reply)
            } else {
                let o = cstr.to_string_lossy().to_string();
                imported::free_string(r);
                Err(o)
            }
        }
    }

    fn verify_challenge(&self, headers: HashMap<&str, &str>) -> Result<String, String> {
        unsafe {
            let encoded_headers = serde_json::to_vec(&headers).map_err(|rr| rr.to_string())?;
            let c_headers =
                CString::new(encoded_headers).map_err(|_| "null character in JSON encoded string?!?".to_string())?;
            let mut success = false;
            let r = imported::verify_challenge(c_headers.as_ptr(), &mut success);
            let cstr = CStr::from_ptr(r);
            let o = cstr.to_string_lossy().to_string();
            imported::free_string(r);
            if success {
                Ok(o)
            } else {
                Err(o)
            }
        }
    }

    fn should_provide_app_sig(&self, headers: HashMap<&str, &str>) -> Result<GHResponse, String> {
        unsafe {
            let encoded_headers = serde_json::to_vec(&headers).map_err(|rr| rr.to_string())?;
            let c_headers =
                CString::new(encoded_headers).map_err(|_| "null character in JSON encoded string?!?".to_string())?;
            let mut success = false;
            let r = imported::should_provide_app_sig(c_headers.as_ptr(), &mut success);
            let cstr = CStr::from_ptr(r);
            if success {
                let reply: GHResponse = serde_json::from_slice(cstr.to_bytes()).unwrap();
                imported::free_string(r);
                Ok(reply)
            } else {
                let o = cstr.to_string_lossy().to_string();
                imported::free_string(r);
                Err(o)
            }
        }
    }
}

pub fn gh_fail_decision(reason: &str) -> Decision {
    Decision::action(
        Action {
            atype: ActionType::Block,
            block_mode: true,
            headers: None,
            status: 500,
            content: "internal_error".to_string(),
            extra_tags: None,
        },
        vec![BlockReason::phase01_unknown(reason)],
    )
}

pub fn challenge_phase01<GH: Grasshopper>(
    gh: &GH,
    logs: &mut Logs,
    rinfo: &RequestInfo,
    reasons: Vec<BlockReason>,
    mode: GHMode,
) -> Decision {
    let query = GHQuery {
        headers: rinfo.headers.as_map(),
        cookies: rinfo.cookies.as_map(),
        ip: &rinfo.rinfo.geoip.ipstr,
        protocol: rinfo.rinfo.meta.protocol.as_deref().unwrap_or("https"),
    };
    let gh_response = match gh.init_challenge(query, mode) {
        Ok(r) => {
            logs.debug(|| format!("Challenge phase01 finished. mode: {:?}", mode));
            r
        }
        Err(rr) => {
            logs.error(|| format!("Challenge phase01 error {}", rr));
            return gh_fail_decision(&rr);
        }
    };
    let ch_tag = if mode == GHMode::Interactive {
        "ichallenge"
    } else {
        "challenge"
    };
    Decision::action(
        Action {
            atype: ActionType::Block,
            block_mode: true,
            headers: Some(gh_response.headers),
            status: 247,
            content: gh_response.str_response,
            extra_tags: Some(["challenge_phase01", ch_tag].iter().map(|s| s.to_string()).collect()),
        },
        reasons,
    )
}

pub fn challenge_phase02<GH: Grasshopper>(
    gh: &GH,
    logs: &mut Logs,
    reqinfo: &RequestInfo,
    reasons: Vec<BlockReason>,
) -> Option<Decision> {
    if !reqinfo
        .rinfo
        .qinfo
        .uri
        .starts_with("/7060ac19f50208cbb6b45328ef94140a612ee92387e015594234077b4d1e64f1")
    {
        return None;
    }

    let verified = match gh.verify_challenge(reqinfo.headers.as_map()) {
        Ok(r) => {
            logs.debug("Challenge phase02 finished");
            r
        }
        Err(rr) => {
            logs.error(|| format!("Challenge phase02 error {}", rr));
            return None;
        }
    };

    let mut nheaders = HashMap::<String, String>::new();
    let mut cookie = "rbzid=".to_string();
    cookie += &verified.replace('=', "-");
    cookie += "; Path=/; HttpOnly";

    nheaders.insert("Set-Cookie".to_string(), cookie);

    Some(Decision::action(
        Action {
            atype: ActionType::Block,
            block_mode: true,
            headers: Some(nheaders),
            status: 248,
            content: "{}".to_string(),
            extra_tags: Some(["challenge_phase02"].iter().map(|s| s.to_string()).collect()),
        },
        reasons,
    ))
}

pub fn check_app_sig<GH: Grasshopper>(
    gh: &GH,
    logs: &mut Logs,
    reqinfo: &RequestInfo,
    reasons: Vec<BlockReason>,
) -> Option<Decision> {
    if !reqinfo
        .rinfo
        .qinfo
        .uri
        .starts_with("/74d8-ffc3-0f63-4b3c-c5c9-5699-6d5b-3a1")
    {
        return None;
    }

    let gh_response = match gh.should_provide_app_sig(reqinfo.headers.as_map()) {
        Ok(r) => r,
        Err(rr) => {
            logs.error(|| format!("check_app_sig error {}", rr));
            return None;
        }
    };
    Some(Decision::action(
        Action {
            atype: ActionType::Block,
            block_mode: true,
            headers: Some(gh_response.headers),
            status: gh_response.status_code,
            content: "{}".to_string(),
            extra_tags: Some(["check_app_sig"].iter().map(|s| s.to_string()).collect()),
        },
        reasons,
    ))
}

pub fn handle_bio_reports<GH: Grasshopper>(
    gh: &GH,
    logs: &mut Logs,
    reqinfo: &RequestInfo,
    precision_level: PrecisionLevel,
    reasons: Vec<BlockReason>,
) -> Option<Decision> {
    if !reqinfo
        .rinfo
        .qinfo
        .uri
        .starts_with("/8d47-ffc3-0f63-4b3c-c5c9-5699-6d5b-3a1")
    {
        return None;
    }
    let query = GHQuery {
        //todo need args...
        headers: reqinfo.headers.as_map(),
        cookies: reqinfo.cookies.as_map(),
        //todo can remove these 2
        ip: &reqinfo.rinfo.geoip.ipstr,
        protocol: reqinfo.rinfo.meta.protocol.as_deref().unwrap_or("https"),
    };
    let gh_response = match gh.handle_bio_report(query, precision_level) {
        Ok(r) => r,
        Err(rr) => {
            logs.error(|| format!("handle_bio_reports error {}", rr));
            return None;
        }
    };
    Some(Decision::action(
        Action {
            atype: ActionType::Block,
            block_mode: true,
            headers: Some(gh_response.headers),
            status: gh_response.status_code, //todo?
            content: gh_response.str_response,
            extra_tags: Some(["handle_bio_reports"].iter().map(|s| s.to_string()).collect()),
        },
        reasons,
    ))
}
