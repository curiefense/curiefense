use crate::interface::BlockReason;
use crate::requestfields::RequestField;
use crate::{Action, ActionType, Decision};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

pub trait Grasshopper {
    fn js_app(&self) -> Option<String>;
    fn js_bio(&self) -> Option<String>;
    fn parse_rbzid(&self, rbzid: &str, seed: &str) -> Option<bool>;
    fn gen_new_seed(&self, seed: &str) -> Option<String>;
    fn verify_workproof(&self, workproof: &str, seed: &str) -> Option<String>;
}

mod imported {
    use std::os::raw::c_char;
    extern "C" {
        pub fn verify_workproof(c_zebra: *const c_char, c_ua: *const c_char, success: *mut bool) -> *mut c_char;
        pub fn gen_new_seed(c_ua: *const c_char) -> *mut c_char;
        pub fn parse_rbzid(c_rbzid: *const c_char, c_ua: *const c_char) -> i8;
        pub fn js_app() -> *const i8;
        pub fn js_bio() -> *const i8;
        pub fn free_string(s: *mut c_char);
    }
}

pub struct DummyGrasshopper {}

// use this when grasshopper can't be used
impl Grasshopper for DummyGrasshopper {
    fn js_app(&self) -> Option<String> {
        Some("dummy_grasshopper_for_testing_only".to_string())
    }
    fn js_bio(&self) -> Option<String> {
        Some("dummy_grasshopper_for_testing_only".to_string())
    }
    fn parse_rbzid(&self, _rbzid: &str, _seed: &str) -> Option<bool> {
        Some(false)
    }
    fn gen_new_seed(&self, _seed: &str) -> Option<String> {
        Some("dummy_grasshopper_for_testing_only".to_string())
    }
    fn verify_workproof(&self, _workproof: &str, _seed: &str) -> Option<String> {
        None
    }
}

#[derive(Clone)]
pub struct DynGrasshopper {}

impl Grasshopper for DynGrasshopper {
    fn js_app(&self) -> Option<String> {
        unsafe {
            let v = imported::js_app();
            let c_v = CStr::from_ptr(v);
            let o = c_v.to_string_lossy().to_string();

            Some(o)
        }
    }
    fn js_bio(&self) -> Option<String> {
        unsafe {
            let v = imported::js_bio();
            let c_v = CStr::from_ptr(v);
            let o = c_v.to_string_lossy().to_string();

            Some(o)
        }
    }
    fn parse_rbzid(&self, rbzid: &str, seed: &str) -> Option<bool> {
        unsafe {
            let c_rbzid = CString::new(rbzid).ok()?;
            let c_seed = CString::new(seed).ok()?;
            match imported::parse_rbzid(c_rbzid.as_ptr(), c_seed.as_ptr()) {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        }
    }
    fn gen_new_seed(&self, seed: &str) -> Option<String> {
        unsafe {
            let c_seed = CString::new(seed).ok()?;
            let r = imported::gen_new_seed(c_seed.as_ptr());
            let cstr = CStr::from_ptr(r);
            let o = cstr.to_string_lossy().to_string();
            imported::free_string(r);
            Some(o)
        }
    }
    fn verify_workproof(&self, workproof: &str, seed: &str) -> Option<String> {
        unsafe {
            let c_workproof = CString::new(workproof).ok()?;
            let c_seed = CString::new(seed).ok()?;
            let mut success = false;
            let r = imported::verify_workproof(c_workproof.as_ptr(), c_seed.as_ptr(), &mut success);

            let cstr = CStr::from_ptr(r);
            let o = cstr.to_string_lossy().to_string();
            imported::free_string(r);
            Some(o)
        }
    }
}

pub fn gh_fail_decision(reason: &str) -> Decision {
    Decision::action(
        Action {
            atype: ActionType::Block {
                headers: None,
                status: 500,
                content: "internal_error".to_string(),
            },
            extra_tags: None,
        },
        vec![BlockReason::phase01_unknown(reason)],
    )
}

pub fn challenge_phase01<GH: Grasshopper>(gh: &GH, ua: &str, reasons: Vec<BlockReason>) -> Decision {
    let seed = match gh.gen_new_seed(ua) {
        None => return gh_fail_decision("could not call gen_new_seed"),
        Some(s) => s,
    };
    let chall_lib = match gh.js_app() {
        None => return gh_fail_decision("could not call chall_lib"),
        Some(s) => s,
    };
    let hdrs: HashMap<String, String> = [
        ("Content-Type", "text/html; charset=utf-8"),
        ("Expires", "Thu, 01 Aug 1978 00:01:48 GMT"),
        ("Cache-Control", "no-cache, private, no-transform, no-store"),
        ("Pragma", "no-cache"),
        (
            "P3P",
            "CP=\"IDC DSP COR ADM DEVi TAIi PSA PSD IVAi IVDi CONi HIS OUR IND CNT\"",
        ),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();

    let mut content = "<html><head><meta charset=\"utf-8\"><script>".to_string();
    content += &chall_lib;
    content += ";;window.rbzns={bereshit: \"1\", seed: \"";
    content += &seed;
    content += "\", storage:\"3\"};winsocks();";
    content += "</script></head><body></body></html>";

    // here humans are accepted, as they were not denied
    // (this would have been caught by the previous guard)
    Decision::action(
        Action {
            atype: ActionType::Block {
                headers: Some(hdrs),
                status: 247,
                content,
            },
            extra_tags: Some(["challenge_phase01"].iter().map(|s| s.to_string()).collect()),
        },
        reasons,
    )
}

fn extract_zebra(headers: &RequestField) -> Option<String> {
    for (k, v) in headers.iter() {
        if k.starts_with("x-zebra-") {
            return Some(v.replace('-', "="));
        }
    }
    None
}

pub fn challenge_phase02<GH: Grasshopper>(gh: &GH, uri: &str, headers: &RequestField) -> Option<Decision> {
    if !uri.starts_with("/7060ac19f50208cbb6b45328ef94140a612ee92387e015594234077b4d1e64f1/") {
        return None;
    }
    let ua = headers.get("user-agent")?;
    let workproof = extract_zebra(headers)?;
    let verified = gh.verify_workproof(&workproof, ua)?;
    let mut nheaders = HashMap::<String, String>::new();
    let mut cookie = "rbzid=".to_string();
    cookie += &verified.replace('=', "-");
    cookie += "; Path=/; HttpOnly";

    nheaders.insert("Set-Cookie".to_string(), cookie);

    Some(Decision::action(
        Action {
            atype: ActionType::Block {
                headers: Some(nheaders),
                status: 248,
                content: "{}".to_string(),
            },
            extra_tags: Some(["challenge_phase02"].iter().map(|s| s.to_string()).collect()),
        },
        vec![BlockReason::phase02()],
    ))
}
