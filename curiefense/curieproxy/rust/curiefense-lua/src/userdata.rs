use std::collections::HashMap;

use curiefense::analyze::{APhase1, APhase2I};
use curiefense::flow::{FlowCheck, FlowResult, FlowResultType};
use curiefense::interface::Tags;
use curiefense::limit::{LimitCheck, LimitResult};
use curiefense::logs::Logs;
use curiefense::utils::{InspectionResult, MaskedInspectionResult};
use mlua::prelude::*;

/// Data type for the full Lua inspection procedure (including redis calls)
pub struct LuaInspectionResult(pub Result<MaskedInspectionResult, String>);
impl LuaInspectionResult {
    pub fn get_with_o<F, A>(&self, f: F) -> LuaResult<Option<A>>
    where
        F: FnOnce(&MaskedInspectionResult) -> Option<A>,
    {
        Ok(match &self.0 {
            Ok(res) => f(res),
            Err(_) => None,
        })
    }
    pub fn get_with<F, A>(&self, f: F) -> LuaResult<Option<A>>
    where
        F: FnOnce(&MaskedInspectionResult) -> A,
    {
        self.get_with_o(|r| Some(f(r)))
    }
}
impl mlua::UserData for LuaInspectionResult {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("error", |_, this| {
            Ok(match &this.0 {
                Ok(res) => res.err.clone(),
                Err(r) => Some(r.clone()),
            })
        });
        fields.add_field_method_get("blocking", |_, this| {
            Ok(match &this.0 {
                Ok(r) => r.decision.is_blocking(),
                Err(_) => false,
            })
        });
        fields.add_field_method_get("tags", |_, this| {
            this.get_with(|r| {
                r.tags
                    .as_ref()
                    .map(|tgs: &Tags| tgs.as_hash_ref().keys().cloned().collect::<Vec<_>>())
            })
        });
        fields.add_field_method_get("logs", |_, this| this.get_with(|r| r.logs.to_stringvec()));
        fields.add_field_method_get("response", |_, this| this.get_with(|r| r.decision.response_json()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("request_map", |lua, this, proxy: LuaValue| {
            let emr = match FromLua::from_lua(proxy, lua) {
                Err(_) | Ok(None) => this.get_with(|r| r.log_json_block(HashMap::new())),
                Ok(Some(proxy)) => this.get_with(|r| r.log_json_block(proxy)),
            };
            match emr {
                Err(rr) => Err(rr),
                Ok(None) => Ok(None),
                Ok(Some(v)) => Ok(Some(lua.create_string(&v)?)),
            }
        });
    }
}

/// Data type for the "dialog" API, phase 1, initialization
#[derive(Clone)]
pub enum LInitResult<T> {
    P0Result(Box<InspectionResult>),
    P0Error(String),
    P1(Logs, Box<T>),
}

impl<T> LInitResult<T> {
    fn get_with_o<F, A>(&self, f: F) -> LuaResult<Option<A>>
    where
        F: FnOnce(&InspectionResult) -> Option<A>,
    {
        Ok(match self {
            LInitResult::P0Result(p0) => f(p0),
            _ => None,
        })
    }
    fn get_with<F, A>(&self, f: F) -> LuaResult<Option<A>>
    where
        F: FnOnce(&InspectionResult) -> A,
    {
        self.get_with_o(|r| Some(f(r)))
    }
}

impl mlua::UserData for LInitResult<APhase1> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        use LInitResult::*;

        fields.add_field_method_get("decided", |_, this| Ok(!matches!(this, P1(_, _))));
        fields.add_field_method_get("error", |_, this| {
            Ok(match this {
                P0Result(res) => res.err.clone(),
                P0Error(r) => Some(r.clone()),
                P1(_, _) => None,
            })
        });
        fields.add_field_method_get("blocking", |_, this| {
            Ok(match this {
                P0Result(r) => r.decision.is_blocking(),
                _ => false,
            })
        });
        fields.add_field_method_get("tags", |_, this| {
            this.get_with(|r| {
                r.tags
                    .as_ref()
                    .map(|tgs: &Tags| tgs.as_hash_ref().keys().cloned().collect::<Vec<_>>())
            })
        });
        fields.add_field_method_get("logs", |_, this| this.get_with(|r| r.logs.to_stringvec()));
        fields.add_field_method_get("response", |_, this| this.get_with(|r| r.decision.response_json()));

        fields.add_field_method_get("flows", |_, this| {
            Ok(match this {
                P1(_, a1) => Some(a1.flows.iter().map(|f| LuaFlowCheck(f.clone())).collect::<Vec<_>>()),
                P0Result(_) => None,
                P0Error(_) => None,
            })
        });

        fields.add_field_method_get("desc", |_, this| {
            Ok(match this {
                P0Result(_) => "result".to_string(),
                P0Error(_) => "error".to_string(),
                P1(_, _) => "p1".to_string(),
            })
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("request_map", |lua, this, proxy: LuaValue| {
            let emr = match FromLua::from_lua(proxy, lua) {
                Err(_) | Ok(None) => this.get_with(|r| r.clone().mask().log_json_block(HashMap::new())),
                Ok(Some(proxy)) => this.get_with(|r| r.clone().mask().log_json_block(proxy)),
            };
            match emr {
                Err(rr) => Err(rr),
                Ok(None) => Ok(None),
                Ok(Some(v)) => Ok(Some(lua.create_string(&v)?)),
            }
        });
    }
}

impl mlua::UserData for LInitResult<APhase2I> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        use LInitResult::*;

        fields.add_field_method_get("decided", |_, this| Ok(!matches!(this, P1(_, _))));
        fields.add_field_method_get("error", |_, this| {
            Ok(match this {
                P0Result(res) => res.err.clone(),
                P0Error(r) => Some(r.clone()),
                P1(_, _) => None,
            })
        });
        fields.add_field_method_get("blocking", |_, this| {
            Ok(match this {
                P0Result(r) => r.decision.is_blocking(),
                _ => false,
            })
        });
        fields.add_field_method_get("tags", |_, this| {
            this.get_with(|r| {
                r.tags
                    .as_ref()
                    .map(|tgs: &Tags| tgs.as_hash_ref().keys().cloned().collect::<Vec<_>>())
            })
        });
        fields.add_field_method_get("logs", |_, this| this.get_with(|r| r.logs.to_stringvec()));
        fields.add_field_method_get("response", |_, this| this.get_with(|r| r.decision.response_json()));

        fields.add_field_method_get("limits", |_, this| {
            Ok(match this {
                P1(_, a1) => Some(a1.limits.iter().map(|f| LuaLimitCheck(f.clone())).collect::<Vec<_>>()),
                P0Result(_) => None,
                P0Error(_) => None,
            })
        });

        fields.add_field_method_get("desc", |_, this| {
            Ok(match this {
                P0Result(_) => "result".to_string(),
                P0Error(_) => "error".to_string(),
                P1(_, _) => "p1".to_string(),
            })
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("request_map", |lua, this, proxy: LuaValue| {
            let emr = match FromLua::from_lua(proxy, lua) {
                Err(_) | Ok(None) => this.get_with(|r| r.clone().mask().log_json_block(HashMap::new())),
                Ok(Some(proxy)) => this.get_with(|r| r.clone().mask().log_json_block(proxy)),
            };
            match emr {
                Err(rr) => Err(rr),
                Ok(None) => Ok(None),
                Ok(Some(v)) => Ok(Some(lua.create_string(&v)?)),
            }
        });
    }
}

/// wrapper for limit checks
#[derive(Clone)]
pub struct LuaLimitCheck(pub LimitCheck);
impl mlua::UserData for LuaLimitCheck {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("key", |_, this| Ok(this.0.key.clone()));
        fields.add_field_method_get("pairwith", |_, this| Ok(this.0.pairwith.clone()));
        fields.add_field_method_get("zero_limits", |_, this| Ok(this.0.zero_limits()));
        fields.add_field_method_get("timeframe", |_, this| Ok(this.0.limit.timeframe));
    }
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("result", |_, this, curcount| {
            Ok(LuaLimitResult(LimitResult {
                limit: this.0.limit.clone(),
                curcount,
            }))
        });
    }
}

/// wrapper for limit query results
#[derive(Clone)]
pub struct LuaLimitResult(pub LimitResult);
impl mlua::UserData for LuaLimitResult {}

/// wrapper for flow checks
#[derive(Clone)]
pub struct LuaFlowCheck(pub FlowCheck);
impl mlua::UserData for LuaFlowCheck {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("key", |_, this| Ok(this.0.redis_key.clone()));
        fields.add_field_method_get("step", |_, this| Ok(this.0.step));
        fields.add_field_method_get("is_last", |_, this| Ok(this.0.is_last));
        fields.add_field_method_get("name", |_, this| Ok(this.0.name.clone()));
        fields.add_field_method_get("tags", |_, this| Ok(this.0.tags.clone()));
        fields.add_field_method_get("timeframe", |_, this| Ok(this.0.timeframe));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("result", |_, this, tp: String| {
            let tp = match tp.as_str() {
                "lastok" => FlowResultType::LastOk,
                "lastblock" => FlowResultType::LastBlock,
                "nonlast" => FlowResultType::NonLast,
                _ => {
                    return Err(mlua::Error::ToLuaConversionError {
                        from: "String",
                        to: "FlowResultType",
                        message: Some(format!("unknown type: {}", tp)),
                    })
                }
            };
            Ok(LuaFlowResult(FlowResult {
                tp,
                id: this.0.id.clone(),
                name: this.0.name.clone(),
                tags: this.0.tags.clone(),
            }))
        });
    }
}

/// wrapper for flow query results
#[derive(Clone)]
pub struct LuaFlowResult(pub FlowResult);
impl mlua::UserData for LuaFlowResult {}
