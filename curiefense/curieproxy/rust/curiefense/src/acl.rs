use crate::config::raw::AclProfile;
use crate::interface::Tags;

use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Serialize)]
pub struct AclDecision {
    pub allowed: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub enum AclResult {
    /// bypass found
    Bypass(AclDecision),
    /// bots, human results
    Match(BotHuman),
}

#[derive(Debug, Serialize)]
pub struct BotHuman {
    pub bot: Option<AclDecision>,
    pub human: Option<AclDecision>,
}

pub fn check_acl(tags: &Tags, acl: &AclProfile) -> AclResult {
    let subcheck = |checks: &HashSet<String>, allowed: bool| {
        let tags: Vec<String> = checks.intersection(tags.as_hash_ref()).cloned().collect();
        if tags.is_empty() {
            None
        } else {
            Some(AclDecision { allowed, tags })
        }
    };
    subcheck(&acl.force_deny, false)
        .map(AclResult::Bypass)
        .or_else(|| subcheck(&acl.bypass, true).map(AclResult::Bypass))
        .unwrap_or_else(|| {
            let botresult = subcheck(&acl.allow_bot, true).or_else(|| subcheck(&acl.deny_bot, false));
            let humanresult = subcheck(&acl.allow, true).or_else(|| subcheck(&acl.deny, false));

            AclResult::Match(BotHuman {
                bot: botresult,
                human: humanresult,
            })
        })
}
