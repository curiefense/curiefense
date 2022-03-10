use hyperscan::Matching;
use libinjection::{sqli, xss};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

use crate::config::contentfilter::{
    ContentFilterEntryMatch, ContentFilterProfile, ContentFilterRule, ContentFilterRules, ContentFilterSection,
    Section, SectionIdx,
};
use crate::config::utils::XDataSource;
use crate::interface::{Action, ActionType};
use crate::requestfields::RequestField;
use crate::utils::{masker, RequestInfo};
use crate::Logs;

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
                        let mut groups = Vec::new();
                        for (group_id, group_name) in &sig.groups {
                            groups.push(json!({
                                "content_filter_group_id": group_id,
                                "content_filter_group_name": group_name,
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
                            "sig_risk": sig.risk,
                            "sig_msg": sig.msg,
                            "content_filter_groups": json!(groups),
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
                "value": "XSS",
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

fn get_section(idx: SectionIdx, rinfo: &RequestInfo) -> &RequestField {
    use SectionIdx::*;
    match idx {
        Headers => &rinfo.headers,
        Cookies => &rinfo.cookies,
        Args => &rinfo.rinfo.qinfo.args,
        Path => &rinfo.rinfo.qinfo.path_as_map,
    }
}

/// Runs the Content Filter part of curiefense
pub fn content_filter_check(
    logs: &mut Logs,
    rinfo: &RequestInfo,
    profile: &ContentFilterProfile,
    hsdb: std::sync::RwLockReadGuard<Option<ContentFilterRules>>,
) -> Result<(), ContentFilterBlock> {
    use SectionIdx::*;
    let mut omit = Default::default();

    // check section profiles
    for idx in &[Path, Headers, Cookies, Args] {
        section_check(
            *idx,
            profile.sections.get(*idx),
            get_section(*idx, rinfo),
            profile.ignore_alphanum,
            &mut omit,
        )?;
    }

    let mut hca_keys: HashMap<String, (SectionIdx, String)> = HashMap::new();

    // run libinjection on non-whitelisted sections
    for idx in &[Path, Headers, Cookies, Args] {
        // note that there is no risk check with injection, every match triggers a block.
        injection_check(*idx, get_section(*idx, rinfo), &omit, &mut hca_keys)?;
    }

    // finally, hyperscan check
    match hyperscan(logs, hca_keys, hsdb, &omit.exclusions, &profile.sections) {
        Err(rr) => {
            logs.error(format!("Hyperscan failed {}", rr));
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
    if idx != SectionIdx::Path && params.len() > section.max_count {
        return Err(ContentFilterBlock::TooManyEntries(idx));
    }

    for (name, value) in params.iter() {
        // skip decoded parameters for length checks
        if !name.ends_with(":decoded") && value.len() > section.max_length {
            return Err(ContentFilterBlock::EntryTooLarge(idx, name.to_string()));
        }

        // automatically ignored
        if ignore_alphanum && value.chars().all(|c| c.is_ascii_alphanumeric()) {
            omit.entries.at(idx).insert(name.to_string());
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
                omit.entries.at(idx).insert(name.to_string());
            } else if name_entry.restrict {
                return Err(ContentFilterBlock::Mismatch(ContentFilterMatched::new(
                    idx,
                    name.to_string(),
                    value.to_string(),
                )));
            } else if !name_entry.exclusions.is_empty() {
                omit.exclusions
                    .at(idx)
                    .insert(name.to_string(), name_entry.exclusions.clone());
            }
            Ok(())
        };

        // check name rules
        if let Some(entry) = section.names.get(name) {
            check_entry(entry)?;
            // if an argument was matched by exact check, we do not try to match it against regex rules
            continue;
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
                            ContentFilterMatched::new(idx, name.to_string(), value.to_string()),
                            fp,
                        ));
                    }
                }
                if let Some(b) = xss(value) {
                    if b {
                        return Err(ContentFilterBlock::Xss(ContentFilterMatched::new(
                            idx,
                            name.to_string(),
                            value.to_string(),
                        )));
                    }
                }
            }

            hca_keys.insert(value.to_string(), (idx, name.to_string()));
        }
    }

    Ok(())
}

fn hyperscan(
    logs: &mut Logs,
    hca_keys: HashMap<String, (SectionIdx, String)>,
    hsdb: std::sync::RwLockReadGuard<Option<ContentFilterRules>>,
    exclusions: &Section<HashMap<String, HashSet<String>>>,
    sections: &Section<ContentFilterSection>,
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
                None => logs.error(format!("INVALID INDEX ??? {}", id)),
                Some(sig) => {
                    logs.debug(format!("signature matched {:?}", sig));
                    if sig.risk >= sections.get(sid).min_risk
                        && exclusions.get(sid).get(&name).map(|ex| ex.contains(&sig.id)) != Some(true)
                    {
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

fn mask_section(masking_seed: &[u8], sec: &mut RequestField, section: &ContentFilterSection) -> HashSet<XDataSource> {
    let match_value = |e: &ContentFilterEntryMatch, v: &str| e.reg.as_ref().map(|re| re.is_match(v)).unwrap_or(true);

    let to_mask: Vec<String> = sec
        .iter()
        .filter(|&(name, value)| {
            if let Some(e) = section.names.get(name) {
                e.mask && match_value(e, value)
            } else {
                section
                    .regex
                    .iter()
                    .any(|(re, e)| e.mask && re.is_match(name) && match_value(e, value))
            }
        })
        .map(|(name, _)| name.to_string())
        .collect();
    to_mask.iter().flat_map(|n| sec.mask(masking_seed, n)).collect()
}

pub fn masking(masking_seed: &[u8], req: RequestInfo, profile: &ContentFilterProfile) -> RequestInfo {
    let mut ri = req;
    mask_section(masking_seed, &mut ri.headers, profile.sections.get(SectionIdx::Headers));
    let mut to_mask = HashSet::new();

    to_mask.extend(mask_section(
        masking_seed,
        &mut ri.cookies,
        profile.sections.get(SectionIdx::Cookies),
    ));
    to_mask.extend(mask_section(
        masking_seed,
        &mut ri.rinfo.qinfo.args,
        profile.sections.get(SectionIdx::Args),
    ));
    to_mask.extend(mask_section(
        masking_seed,
        &mut ri.rinfo.qinfo.path_as_map,
        profile.sections.get(SectionIdx::Path),
    ));
    to_mask.extend(mask_section(
        masking_seed,
        &mut ri.rinfo.qinfo.path_as_map,
        profile.sections.get(SectionIdx::Path),
    ));
    for x in to_mask {
        match x {
            XDataSource::Uri => {
                ri.rinfo.qinfo.query = masker(masking_seed, &ri.rinfo.qinfo.query);
                ri.rinfo.qinfo.uri = masker(masking_seed, &ri.rinfo.qinfo.uri);
            }
            XDataSource::CookieHeader => {
                ri.headers.mask(masking_seed, "cookie");
            }
        }
    }
    ri
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::utils::DataSource;
    use crate::utils::{map_request, RequestMeta};
    use crate::{Logs, RawRequest};

    fn test_request_info() -> RequestInfo {
        let meta = RequestMeta {
            authority: Some("myhost".to_string()),
            method: "GET".to_string(),
            path: "/foo?arg1=avalue1&arg2=avalue2".to_string(),
            extra: HashMap::default(),
        };
        let mut logs = Logs::default();
        let headers = [("h1", "value1"), ("h2", "value2")]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let raw_request = RawRequest {
            ipstr: "1.2.3.4".into(),
            mbody: None,
            headers,
            meta,
        };
        map_request(&mut logs, &[], &raw_request)
    }

    #[test]
    fn no_masking() {
        let rinfo = test_request_info();
        let profile = ContentFilterProfile::default_from_seed("test");
        let masked = masking(b"test", rinfo.clone(), &profile);
        assert_eq!(rinfo.headers, masked.headers);
        assert_eq!(rinfo.cookies, masked.cookies);
        assert_eq!(rinfo.rinfo.qinfo.args, masked.rinfo.qinfo.args);
    }

    fn maskentry() -> ContentFilterEntryMatch {
        ContentFilterEntryMatch {
            restrict: false,
            mask: true,
            exclusions: HashSet::default(),
            reg: None,
        }
    }

    #[test]
    fn masking_all_args_re() {
        let rinfo = test_request_info();
        let mut profile = ContentFilterProfile::default_from_seed("test");
        let asection = profile.sections.at(SectionIdx::Args);
        asection.regex = vec![(regex::Regex::new(".").unwrap(), maskentry())];
        let masked = masking(b"test", rinfo.clone(), &profile);
        assert_eq!(rinfo.headers, masked.headers);
        assert_eq!(rinfo.cookies, masked.cookies);
        assert_eq!(
            RequestField::raw_create(
                &[],
                &[
                    ("arg1", &DataSource::X(XDataSource::Uri), "MASKED{fac0029}"),
                    ("arg2", &DataSource::X(XDataSource::Uri), "MASKED{7ce2d8d}")
                ]
            ),
            masked.rinfo.qinfo.args
        );
    }

    #[test]
    fn masking_re_arg1() {
        let rinfo = test_request_info();
        let mut profile = ContentFilterProfile::default_from_seed("test");
        let asection = profile.sections.at(SectionIdx::Args);
        asection.regex = vec![(regex::Regex::new("1").unwrap(), maskentry())];
        let masked = masking(b"test", rinfo.clone(), &profile);
        assert_eq!(rinfo.headers, masked.headers);
        assert_eq!(rinfo.cookies, masked.cookies);
        assert_eq!(
            RequestField::raw_create(
                &[],
                &[
                    ("arg1", &DataSource::X(XDataSource::Uri), "MASKED{fac0029}"),
                    ("arg2", &DataSource::X(XDataSource::Uri), "avalue2")
                ]
            ),
            masked.rinfo.qinfo.args
        );
    }

    #[test]
    fn masking_named_arg1() {
        let rinfo = test_request_info();
        let mut profile = ContentFilterProfile::default_from_seed("test");
        let asection = profile.sections.at(SectionIdx::Args);
        asection.names = ["arg1"].iter().map(|k| (k.to_string(), maskentry())).collect();
        let masked = masking(b"test", rinfo.clone(), &profile);
        assert_eq!(rinfo.headers, masked.headers);
        assert_eq!(rinfo.cookies, masked.cookies);
        assert_eq!(
            RequestField::raw_create(
                &[],
                &[
                    ("arg1", &DataSource::X(XDataSource::Uri), "MASKED{fac0029}"),
                    ("arg2", &DataSource::X(XDataSource::Uri), "avalue2")
                ]
            ),
            masked.rinfo.qinfo.args
        );
    }

    #[test]
    fn masking_all_args_names() {
        let rinfo = test_request_info();
        let mut profile = ContentFilterProfile::default_from_seed("test");
        let asection = profile.sections.at(SectionIdx::Args);
        asection.names = ["arg1", "arg2"].iter().map(|k| (k.to_string(), maskentry())).collect();
        let masked = masking(b"test", rinfo.clone(), &profile);
        assert_eq!(rinfo.headers, masked.headers);
        assert_eq!(rinfo.cookies, masked.cookies);
        assert_eq!(
            RequestField::raw_create(
                &[],
                &[
                    ("arg1", &DataSource::X(XDataSource::Uri), "MASKED{fac0029}"),
                    ("arg2", &DataSource::X(XDataSource::Uri), "MASKED{7ce2d8d}")
                ]
            ),
            masked.rinfo.qinfo.args
        );
    }
}
