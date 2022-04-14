use crate::requestfields::RequestField;
use crate::{Action, ActionType, Decision};
use serde_json::json;
use std::collections::HashMap;

pub trait Grasshopper {
    fn js_app(&self) -> Option<String>;
    fn js_bio(&self) -> Option<String>;
    fn parse_rbzid(&self, rbzid: &str, seed: &str) -> Option<bool>;
    fn gen_new_seed(&self, seed: &str) -> Option<String>;
    fn verify_workproof(&self, workproof: &str, seed: &str) -> Option<String>;
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

pub fn gh_fail_decision(reason: &str) -> Decision {
    Decision::Action(Action {
        atype: ActionType::Block,
        block_mode: true,
        ban: false,
        reason: json!({"initiator": "phase01", "reason": reason}),
        headers: None,
        status: 500,
        content: "internal_error".to_string(),
        extra_tags: None,
    })
}

pub fn challenge_phase01<GH: Grasshopper>(gh: &GH, ua: &str, tags: Vec<String>) -> Decision {
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
    Decision::Action(Action {
        atype: ActionType::Block,
        block_mode: true,
        ban: false,
        reason: if tags.is_empty() {
            // this happens for rate limit / flow control / tag action
            json!({"initiator": "phase01", "reason": "challenge"})
        } else {
            // this only happens for acl challenges
            json!({"initiator": "phase01", "reason": "challenge", "tags": tags})
        },
        headers: Some(hdrs),
        status: 247,
        content,
        extra_tags: Some(["challenge_phase01"].iter().map(|s| s.to_string()).collect()),
    })
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

    Some(Decision::Action(Action {
        atype: ActionType::Block,
        block_mode: true,
        ban: false,
        reason: json!({"initiator": "phase02", "reason": "challenge"}),
        headers: Some(nheaders),
        status: 248,
        content: "{}".to_string(),
        extra_tags: Some(["challenge_phase02"].iter().map(|s| s.to_string()).collect()),
    }))
}
