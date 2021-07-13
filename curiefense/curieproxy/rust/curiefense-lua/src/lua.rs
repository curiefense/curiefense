/// lua interfaces
use crate::Grasshopper;
use mlua::prelude::{LuaFunction, LuaTable};

pub struct Luagrasshopper<'t>(pub LuaTable<'t>);

impl Grasshopper for Luagrasshopper<'_> {
    fn js_app(&self) -> Option<String> {
        self.0.get("js_app").and_then(|f: LuaFunction| f.call(())).ok()
    }
    fn js_bio(&self) -> Option<String> {
        self.0.get("js_bio").and_then(|f: LuaFunction| f.call(())).ok()
    }
    fn parse_rbzid(&self, rbzid: &str, seed: &str) -> Option<bool> {
        self.0
            .get("parse_rbzid")
            .and_then(|f: LuaFunction| f.call((rbzid, seed)))
            .ok()
    }
    fn gen_new_seed(&self, seed: &str) -> Option<String> {
        self.0.get("gen_new_seed").and_then(|f: LuaFunction| f.call(seed)).ok()
    }
    fn verify_workproof(&self, workproof: &str, seed: &str) -> Option<String> {
        self.0
            .get("verify_workproof")
            .and_then(|f: LuaFunction| f.call((workproof, seed)))
            .ok()
    }
}
