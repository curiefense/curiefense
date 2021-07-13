use hyperscan::Matching;
use libinjection::{sqli, xss};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

use crate::config::raw::WafSignature;
use crate::config::waf::{Section, SectionIdx, WafEntryMatch, WafProfile, WafSection, WafSignatures};
use crate::interface::{Action, ActionType};
use crate::requestfields::RequestField;
use crate::utils::RequestInfo;

#[derive(Debug, Clone)]
pub struct WafMatched {
    pub section: SectionIdx,
    pub name: String,
    pub value: String,
}

impl WafMatched {
    fn new(section: SectionIdx, name: String, value: String) -> Self {
        WafMatched { section, name, value }
    }
}

#[derive(Debug, Clone)]
pub struct WafMatch {
    pub matched: WafMatched,
    pub ids: Vec<WafSignature>,
}

#[derive(Debug, Clone)]
pub enum WafBlock {
    TooManyEntries(SectionIdx),
    EntryTooLarge(SectionIdx, String),
    Mismatch(WafMatched),
    SqlInjection(WafMatched, String), // fingerprint
    Xss(WafMatched),
    Policies(Vec<WafMatch>),
}

impl WafBlock {
    pub fn to_action(&self) -> Action {
        let reason = match self {
            WafBlock::Policies(ids) => ids
                .first()
                .and_then(|e| {
                    e.ids.first().map(|sig| {
                        json!({
                            "section": e.matched.section,
                            "name": e.matched.name,
                            "value": e.matched.value,
                            "initiator": "waf",
                            "sig_category": sig.category,
                            "sig_subcategory": sig.subcategory,
                            "sig_operand": sig.operand,
                            "sig_id": sig.id,
                            "sig_severity": sig.severity,
                            "sig_msg": sig.msg,
                        })
                    })
                })
                .unwrap_or(Value::Null),
            WafBlock::TooManyEntries(idx) => json!({
                "section": idx,
                "initiator": "waf",
                "value": "Too many entries"
            }),
            WafBlock::EntryTooLarge(idx, nm) => json!({
                "section": idx,
                "name": nm,
                "initiator": "waf",
                "value": "Entry too large"
            }),
            WafBlock::SqlInjection(wmatch, fp) => json!({
                "section": wmatch.section,
                "name": wmatch.name,
                "initiator": "waf",
                "value": "SQLi",
                "matched": wmatch.value,
                "fingerprint": fp
            }),
            WafBlock::Xss(wmatch) => json!({
                "section": wmatch.section,
                "name": wmatch.name,
                "initiator": "waf",
                "value": "WSS",
                "matched": wmatch.value
            }),
            WafBlock::Mismatch(wmatch) => json!({
                "section": wmatch.section,
                "name": wmatch.name,
                "initiator": "waf",
                "value": wmatch.value,
                "msg": "Mismatch"
            }),
        };

        Action {
            atype: ActionType::Block,
            block_mode: true,
            ban: false,
            status: 403,
            headers: None,
            reason,
            content: "Access denied".to_string(),
            extra_tags: None,
        }
    }
}

struct Omitted {
    entries: Section<HashSet<String>>,
    exclusions: Section<HashMap<String, HashSet<String>>>,
}

impl Default for Omitted {
    fn default() -> Self {
        Omitted {
            entries: Default::default(),
            exclusions: Default::default(),
        }
    }
}

/// Runs the WAF part of curiefense
///
/// TODO:
/// * resolve hyperscan matches into policies
/// * handle excluded waf policies
pub fn waf_check(
    rinfo: &RequestInfo,
    profile: &WafProfile,
    hsdb: std::sync::RwLockReadGuard<Option<WafSignatures>>,
) -> Result<(), WafBlock> {
    use SectionIdx::*;
    let mut omit = Default::default();

    let getsection = |idx| match idx {
        Headers => &rinfo.headers,
        Cookies => &rinfo.cookies,
        Args => &rinfo.rinfo.qinfo.args,
    };

    // check section profiles
    for idx in &[Headers, Cookies, Args] {
        section_check(
            *idx,
            profile.sections.get(*idx),
            getsection(*idx),
            profile.ignore_alphanum,
            &mut omit,
        )?;
    }

    let mut hca_keys: HashMap<String, (SectionIdx, String)> = HashMap::new();

    // run libinjection on non-whitelisted sections
    for idx in &[Headers, Cookies, Args] {
        injection_check(*idx, getsection(*idx), &omit, &mut hca_keys)?;
    }

    // finally, hyperscan check
    match hyperscan(hca_keys, hsdb, &omit.exclusions) {
        Err(rr) => {
            println!("Hyperscan failed {}", rr);
            Ok(())
        }
        Ok(None) => Ok(()),
        Ok(Some(block)) => Err(block),
    }
}

/// checks a section (headers, args, cookies) against the policy
fn section_check(
    idx: SectionIdx,
    section: &WafSection,
    params: &RequestField,
    ignore_alphanum: bool,
    omit: &mut Omitted,
) -> Result<(), WafBlock> {
    if params.len() > section.max_count {
        return Err(WafBlock::TooManyEntries(idx));
    }

    for (name, value) in params.iter() {
        if value.len() > section.max_length {
            return Err(WafBlock::EntryTooLarge(idx, name.clone()));
        }

        // automatically ignored
        if ignore_alphanum && value.chars().all(|c| c.is_ascii_alphanumeric()) {
            omit.entries.at(idx).insert(name.clone());
            continue;
        }

        // logic for checking an entry
        let mut check_entry = |name_entry: &WafEntryMatch| {
            let matched = if let Some(re) = &name_entry.reg {
                re.is_match(value)
            } else {
                false
            };
            if matched {
                omit.entries.at(idx).insert(name.clone());
            } else if name_entry.restrict {
                return Err(WafBlock::Mismatch(WafMatched::new(idx, name.clone(), value.clone())));
            } else if !name_entry.exclusions.is_empty() {
                omit.exclusions
                    .at(idx)
                    .insert(name.clone(), name_entry.exclusions.clone());
            }
            Ok(())
        };

        // check name rules
        for entry in section.names.get(name).iter() {
            check_entry(entry)?;
        }

        // // check regex rules
        for entry in section
            .regex
            .iter()
            .filter_map(|(re, v)| if re.is_match(name) { Some(v) } else { None })
        {
            check_entry(entry)?;
        }
    }

    Ok(())
}

fn injection_check(
    idx: SectionIdx,
    params: &RequestField,
    omit: &Omitted,
    hca_keys: &mut HashMap<String, (SectionIdx, String)>,
) -> Result<(), WafBlock> {
    for (name, value) in params.iter() {
        if !omit.entries.get(idx).contains(name) {
            if !omit
                .exclusions
                .get(idx)
                .get(name)
                .map(|st| st.contains("libinjection"))
                .unwrap_or(false)
            {
                if let Some((b, fp)) = sqli(value) {
                    if b {
                        return Err(WafBlock::SqlInjection(
                            WafMatched::new(idx, name.clone(), value.clone()),
                            fp,
                        ));
                    }
                }
                if let Some(b) = xss(value) {
                    if b {
                        return Err(WafBlock::Xss(WafMatched::new(idx, name.clone(), value.clone())));
                    }
                }
            }

            hca_keys.insert(value.clone(), (idx, name.clone()));
        }
    }

    Ok(())
}

fn hyperscan(
    hca_keys: HashMap<String, (SectionIdx, String)>,
    hsdb: std::sync::RwLockReadGuard<Option<WafSignatures>>,
    exclusions: &Section<HashMap<String, HashSet<String>>>,
) -> anyhow::Result<Option<WafBlock>> {
    let sigs = match &*hsdb {
        None => return Err(anyhow::anyhow!("Hyperscan database not loaded")),
        Some(x) => x,
    };
    let scratch = sigs.db.alloc_scratch()?;
    let to_scan = hca_keys.keys().map(|s| s.as_bytes());
    let mut found = false;
    sigs.db.scan(to_scan, &scratch, |_, _, _, _| {
        found = true;
        Matching::Continue
    })?;
    if !found {
        return Ok(None);
    }

    let mut matches = Vec::new();

    // something matched! but what?
    for (k, (sid, name)) in hca_keys {
        let mut ids = Vec::new();
        sigs.db.scan(&[k.as_bytes()], &scratch, |id, _, _, _| {
            // TODO this is really ugly, the string hashmap should be converted into a numeric id, or it should be a string in the first place?
            match sigs.ids.get(id as usize) {
                None => println!("INVALID INDEX ??? {}", id),
                Some(sig) => {
                    if exclusions.get(sid).get(&name).map(|ex| ex.contains(&sig.id)) != Some(true) {
                        ids.push(sig.clone());
                    }
                }
            }
            Matching::Continue
        })?;
        if !ids.is_empty() {
            matches.push(WafMatch {
                matched: WafMatched::new(sid, name, k),
                ids,
            })
        }
    }
    Ok(if matches.is_empty() {
        None
    } else {
        Some(WafBlock::Policies(matches))
    })
}
