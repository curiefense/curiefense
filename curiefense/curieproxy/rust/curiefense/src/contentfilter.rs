use hyperscan::Matching;
use libinjection::{sqli, xss};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

use crate::config::raw::ContentFilterRule;
use crate::config::contentfilter::{Section, SectionIdx, ContentFilterEntryMatch, ContentFilterProfile, ContentFilterSection, ContentFilterRules};
use crate::interface::{Action, ActionType};
use crate::requestfields::RequestField;
use crate::utils::RequestInfo;

#[derive(Debug, Clone)]
pub struct ContentFilterMatched {
    pub section: SectionIdx,
    pub name: String,
    pub value: String,
}

impl ContentFilterMatched {
    fn new(section: SectionIdx, name: String, value: String) -> Self {
        ContentFilterMatched { section, name, value }
    }
}

#[derive(Debug, Clone)]
pub struct ContentFilterMatch {
    pub matched: ContentFilterMatched,
    pub ids: Vec<ContentFilterRule>,
}

#[derive(Debug, Clone)]
pub enum ContentFilterBlock {
    TooManyEntries(SectionIdx),
    EntryTooLarge(SectionIdx, String),
    Mismatch(ContentFilterMatched),
    SqlInjection(ContentFilterMatched, String), // fingerprint
    Xss(ContentFilterMatched),
    Policies(Vec<ContentFilterMatch>),
}

impl ContentFilterBlock {
    pub fn to_action(&self) -> Action {
        let reason = match self {
            ContentFilterBlock::Policies(ids) => ids
                .first()
                .and_then(|e| {
                    e.ids.first().map(|sig| {
                        let groups = Vec::new();
                        for group in sig.groups {
                            groups.push(json!({
                                "content_filter_group_id": group.id,
                                "content_filter_group_name": group.name,
                            }));
                        }
                        json!({
                            "section": e.matched.section,
                            "name": e.matched.name,
                            "value": e.matched.value,
                            "initiator": "content_filter",
                            "sig_category": sig.category,
                            "sig_subcategory": sig.subcategory,
                            "sig_operand": sig.operand,
                            "sig_id": sig.id,
                            "sig_severity": sig.severity,
                            "sig_msg": sig.msg,
                            "content_filter_groups:": groups,
                        })
                    })
                })
                .unwrap_or(Value::Null),
            ContentFilterBlock::TooManyEntries(idx) => json!({
                "section": idx,
                "initiator": "content_filter",
                "value": "Too many entries"
            }),
            ContentFilterBlock::EntryTooLarge(idx, nm) => json!({
                "section": idx,
                "name": nm,
                "initiator": "content_filter",
                "value": "Entry too large"
            }),
            ContentFilterBlock::SqlInjection(wmatch, fp) => json!({
                "section": wmatch.section,
                "name": wmatch.name,
                "initiator": "content_filter",
                "value": "SQLi",
                "matched": wmatch.value,
                "fingerprint": fp
            }),
            ContentFilterBlock::Xss(wmatch) => json!({
                "section": wmatch.section,
                "name": wmatch.name,
                "initiator": "content_filter",
                "value": "WSS",
                "matched": wmatch.value
            }),
            ContentFilterBlock::Mismatch(wmatch) => json!({
                "section": wmatch.section,
                "name": wmatch.name,
                "initiator": "content_filter",
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

/// Runs the Content Filter part of curiefense
pub fn content_filter_check(
    rinfo: &RequestInfo,
    profile: &ContentFilterProfile,
    hsdb: std::sync::RwLockReadGuard<Option<ContentFilterRules>>,
) -> Result<(), ContentFilterBlock> {
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
    section: &ContentFilterSection,
    params: &RequestField,
    ignore_alphanum: bool,
    omit: &mut Omitted,
) -> Result<(), ContentFilterBlock> {
    if params.len() > section.max_count {
        return Err(ContentFilterBlock::TooManyEntries(idx));
    }

    for (name, value) in params.iter() {
        if value.len() > section.max_length {
            return Err(ContentFilterBlock::EntryTooLarge(idx, name.clone()));
        }

        // automatically ignored
        if ignore_alphanum && value.chars().all(|c| c.is_ascii_alphanumeric()) {
            omit.entries.at(idx).insert(name.clone());
            continue;
        }

        // logic for checking an entry
        let mut check_entry = |name_entry: &ContentFilterEntryMatch| {
            let matched = if let Some(re) = &name_entry.reg {
                re.is_match(value)
            } else {
                false
            };
            if matched {
                omit.entries.at(idx).insert(name.clone());
            } else if name_entry.restrict {
                return Err(ContentFilterBlock::Mismatch(ContentFilterMatched::new(idx, name.clone(), value.clone())));
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
) -> Result<(), ContentFilterBlock> {
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
                        return Err(ContentFilterBlock::SqlInjection(
                            ContentFilterMatched::new(idx, name.clone(), value.clone()),
                            fp,
                        ));
                    }
                }
                if let Some(b) = xss(value) {
                    if b {
                        return Err(ContentFilterBlock::Xss(ContentFilterMatched::new(idx, name.clone(), value.clone())));
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
    hsdb: std::sync::RwLockReadGuard<Option<ContentFilterRules>>,
    exclusions: &Section<HashMap<String, HashSet<String>>>,
) -> anyhow::Result<Option<ContentFilterBlock>> {
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
            matches.push(ContentFilterMatch {
                matched: ContentFilterMatched::new(sid, name, k),
                ids,
            })
        }
    }
    Ok(if matches.is_empty() {
        None
    } else {
        Some(ContentFilterBlock::Policies(matches))
    })
}
