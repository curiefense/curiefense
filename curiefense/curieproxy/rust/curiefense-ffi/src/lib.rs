use core::ffi::c_void;
use curiefense::grasshopper::{DummyGrasshopper, Grasshopper};
use curiefense::inspect_generic_request_map_async;
use curiefense::interface::{Decision, Tags};
use curiefense::logs::{LogLevel, Logs};
use curiefense::simple_executor::{new_executor_and_spawner, Executor, Progress, TaskCB};
use curiefense::utils::{RawRequest, RequestInfo, RequestMeta};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
#[repr(C)]
pub enum CFProgress {
    CFDone = 0,
    CFMore = 1,
    CFError = 2,
}

pub struct CHashmap {
    inner: HashMap<String, String>,
}

/// # Safety
///
/// New C hashmap
#[no_mangle]
pub unsafe extern "C" fn cf_hashmap_new() -> *mut CHashmap {
    Box::into_raw(Box::new(CHashmap { inner: HashMap::new() }))
}

/// # Safety
///
/// Insert into the hashmap. The key and value are not consumed by this API (it copies them).
#[no_mangle]
pub unsafe extern "C" fn cf_hashmap_insert(hm: *mut CHashmap, key: *const c_char, value: *const c_char) {
    let s_key = CStr::from_ptr(key).to_string_lossy().to_string();
    let s_value = CStr::from_ptr(value).to_string_lossy().to_string();
    if let Some(r) = hm.as_mut() {
        r.inner.insert(s_key, s_value);
    }
}

/// # Safety
///
/// Frees a hashmap, and all its content.
#[no_mangle]
pub unsafe extern "C" fn cf_hashmap_free(ptr: *mut CHashmap) {
    if ptr.is_null() {
        return;
    }
    Box::from_raw(ptr);
}

pub struct CFExec {
    inner: Executor<TaskCB<CFDecision>>,
}

// It is not that bad that the sizes are so distinct, as it will mostly be the "large" variant that we will use.
#[allow(clippy::large_enum_variant)]
pub enum CFResult {
    RR(String),
    OK(CFDecision),
}

#[derive(Debug)]
pub struct CFDecision {
    decision: Decision,
    tags: Tags,
    logs: Logs,
    reqinfo: RequestInfo,
}

/// # Safety
///
/// Returns false is the decision is to let pass, true otherwise.
#[no_mangle]
pub unsafe extern "C" fn curiefense_cfr_is_blocking(ptr: *const CFResult) -> bool {
    match ptr.as_ref() {
        None => false,
        Some(CFResult::RR(_)) => false,
        Some(CFResult::OK(r)) => match r.decision {
            Decision::Action(_) => true,
            Decision::Pass => false,
        },
    }
}

/// # Safety
///
/// Returns the status code of a blocking action.
#[no_mangle]
pub unsafe extern "C" fn curiefense_cfr_block_status(ptr: *const CFResult) -> u32 {
    match ptr.as_ref() {
        None => 0,
        Some(CFResult::RR(_)) => 0,
        Some(CFResult::OK(r)) => match &r.decision {
            Decision::Action(a) => a.status,
            Decision::Pass => 0,
        },
    }
}

/// # Safety
///
/// Returns the content length of a blocking action.
#[no_mangle]
pub unsafe extern "C" fn curiefense_cfr_block_contentlength(ptr: *const CFResult) -> usize {
    match ptr.as_ref() {
        None => 0,
        Some(CFResult::RR(_)) => 0,
        Some(CFResult::OK(r)) => match &r.decision {
            Decision::Action(a) => a.content.len(),
            Decision::Pass => 0,
        },
    }
}

/// # Safety
///
/// Copies the body of a blocking action. The input buffer must have a size that is larger than
/// what the curiefense_str_block_contentlength returned.
#[no_mangle]
pub unsafe extern "C" fn curiefense_cfr_block_content(ptr: *const CFResult, tgt: *mut c_uchar) {
    match ptr.as_ref() {
        None => (),
        Some(CFResult::RR(_)) => (),
        Some(CFResult::OK(r)) => match &r.decision {
            Decision::Action(a) => std::ptr::copy_nonoverlapping(a.content.as_ptr(), tgt, a.content.len()),
            Decision::Pass => (),
        },
    }
}

/// # Safety
///
/// Returns the log string, json encoded. Can be freed with curiefense_str_free.
#[no_mangle]
pub unsafe extern "C" fn curiefense_cfr_log(ptr: *mut CFResult, ln: *mut usize) -> *mut c_char {
    if ptr.is_null() {
        *ln = 0;
        return std::ptr::null_mut();
    }
    let cfr = Box::from_raw(ptr);
    let out: String = match *cfr {
        CFResult::OK(dec) => dec.decision.to_json(dec.reqinfo, dec.tags, dec.logs),
        CFResult::RR(rr) => rr,
    };
    *ln = out.len();
    match CString::new(out) {
        Err(_) => {
            *ln = 0;
            std::ptr::null_mut()
        }
        Ok(cs) => cs.into_raw(),
    }
}

/// # Safety
///
/// Populate the curiefense log string (json encoded)
#[no_mangle]
pub unsafe extern "C" fn curiefense_cfr_logs(
    ptr: *mut CFResult,
    cb: unsafe extern "C" fn(u8, *const c_char, *mut c_void),
    cb_data: *mut c_void,
) {
    match ptr.as_ref() {
        None => {
            let msg = CString::new("Null pointer".to_string()).unwrap();
            cb(LogLevel::Error as u8, msg.as_ptr(), cb_data);
        }
        Some(CFResult::RR(rr)) => {
            let msg = match CString::new(rr.clone()) {
                Err(_) => CString::new("Irrepresentable error".to_string()).unwrap(),
                Ok(errmsg) => errmsg,
            };
            cb(LogLevel::Error as u8, msg.as_ptr(), cb_data);
        }
        Some(CFResult::OK(cfdec)) => {
            for log in &cfdec.logs.logs {
                let msg_str = format!("{}µs - {}", log.elapsed_micros, log.message);
                let msg = match CString::new(msg_str) {
                    Err(_) => CString::new("Irrepresentable log".to_string()).unwrap(),
                    Ok(lgmsg) => lgmsg,
                };
                cb(log.level as u8, msg.as_ptr(), cb_data)
            }
        }
    }
}

/// # Safety
///
/// Returns the error, if available. The returned string can be freed with curiefense_str_free.
#[no_mangle]
pub unsafe extern "C" fn curiefense_cfr_error(ptr: *const CFResult) -> *mut c_char {
    let out: CString = match ptr.as_ref() {
        None => CString::new("Null pointer").unwrap(),
        Some(CFResult::RR(r)) => {
            CString::new(r.clone()).unwrap_or_else(|_| CString::new("Irrepresentable error").unwrap())
        }
        Some(_) => CString::new("No error".to_string()).unwrap(),
    };
    out.into_raw()
}

/// # Safety
///
/// Frees a string that has been returned by this API.
#[no_mangle]
pub unsafe extern "C" fn curiefense_str_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    drop(CString::from_raw(ptr));
}

/// Simple wrapper to return the reqinfo data
pub async fn inspect_wrapper<GH: Grasshopper>(
    logs: Logs,
    configpath: String,
    raw: RawRequest<'_>,
    mgh: Option<GH>,
) -> CFDecision {
    let mut mlogs = logs;
    let (decision, tags, reqinfo) = inspect_generic_request_map_async(&configpath, mgh, raw, &mut mlogs).await;
    CFDecision {
        decision,
        tags,
        logs: mlogs,
        reqinfo,
    }
}

/// # Safety
///
/// Initializes the inspection, returning an executor in case of success, or a null pointer in case of failure.
#[no_mangle]
pub unsafe extern "C" fn curiefense_async_init(
    loglevel: u8,
    raw_configpath: *const c_char,
    raw_meta: *mut CHashmap,
    raw_headers: *mut CHashmap,
    raw_ip: *const c_char,
    mbody: *const c_uchar,
    mbody_len: usize,
    cb: extern "C" fn(u64),
    data: u64,
) -> *mut CFExec {
    let lloglevel = match loglevel {
        0 => LogLevel::Debug,
        1 => LogLevel::Info,
        2 => LogLevel::Warning,
        3 => LogLevel::Error,
        _ => return std::ptr::null_mut(),
    };
    // convert the strings and loglevel
    let configpath = CStr::from_ptr(raw_configpath).to_string_lossy().to_string();
    let ip = CStr::from_ptr(raw_ip).to_string_lossy().to_string();

    // convert the hashmaps and turn them into the required types
    let meta = match raw_meta.as_mut() {
        None => return std::ptr::null_mut(),
        Some(rf) => match RequestMeta::from_map(Box::from_raw(rf).as_ref().inner.clone()) {
            Err(_) => return std::ptr::null_mut(),
            Ok(x) => x,
        },
    };
    let headers = match raw_headers.as_mut() {
        None => return std::ptr::null_mut(),
        Some(rf) => Box::from_raw(rf).as_ref().inner.clone(),
    };

    // retrieve the body
    let mbody = if mbody_len == 0 {
        None
    } else {
        Some(std::slice::from_raw_parts(mbody, mbody_len))
    };

    // create the requestinfo structure
    let logs = Logs::new(lloglevel);
    let raw_request = RawRequest {
        ipstr: ip,
        headers,
        meta,
        mbody,
    };
    let (executor, spawner) = new_executor_and_spawner::<TaskCB<CFDecision>>();
    spawner.spawn_cb(
        inspect_wrapper(logs, configpath, raw_request, Some(DummyGrasshopper {})),
        cb,
        data,
    );
    drop(spawner);
    Box::into_raw(Box::new(CFExec { inner: executor }))
}

/// # Safety
///
/// Steps a valid executor. Note that the executor is freed when CFDone is returned, and the pointer
/// is no longer valid.
#[no_mangle]
pub unsafe extern "C" fn curiefense_async_step(ptr: *mut CFExec, out: *mut *mut CFResult) -> CFProgress {
    *out = std::ptr::null_mut();
    match ptr.as_ref() {
        None => CFProgress::CFError,
        Some(r) => match r.inner.step() {
            Progress::Error(rr) => {
                *out = Box::into_raw(Box::new(CFResult::RR(rr)));
                CFProgress::CFError
            }
            Progress::Done(cfd) => {
                *out = Box::into_raw(Box::new(CFResult::OK(cfd)));
                let _ = Box::from_raw(ptr);
                CFProgress::CFDone
            }
            Progress::More => CFProgress::CFMore,
        },
    }
}

/// # Safety
///
/// Frees the executor, should be run with the output of executor_init, and only once.
/// Generally, you should wait until the step function returns CFDone, but you can use
/// this function to abort early.
#[no_mangle]
pub unsafe extern "C" fn curiefense_async_free(ptr: *mut CFExec) {
    if ptr.is_null() {
        return;
    }
    Box::from_raw(ptr);
}
