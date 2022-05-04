use hyperscan::Matching;
use lazy_static::lazy_static;
use libinjection::{sqli, xss};
use serde_json::json;
use std::collections::{HashMap, HashSet};

use crate::config::contentfilter::{
    rule_tags, ContentFilterEntryMatch, ContentFilterProfile, ContentFilterRules, ContentFilterSection, Section,
    SectionIdx,
};
use crate::config::raw::ContentFilterRule;
use crate::config::utils::XDataSource;
use crate::interface::{Action, ActionType, Tags};
use crate::requestfields::RequestField;
use crate::utils::RequestInfo;
use crate::Logs;

lazy_static! {
    pub static ref LIBINJECTION_SQLI_TAGS: HashSet<String> = [
        "cf-rule-id:libinjection-sqli",
        "cf-rule-category:libinjection",
        "cf-rule-subcategory:libinjection-sqli",
        "cf-rule-risk:libinjection",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    pub static ref LIBINJECTION_XSS_TAGS: HashSet<String> = [
        "cf-rule-id:libinjection-xss",
        "cf-rule-category:libinjection",
        "cf-rule-subcategory:libinjection-xss",
        "cf-rule-risk:libinjection",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

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
    Block(HashSet<String>),
    Monitor(HashSet<String>),
}

impl ContentFilterBlock {
    pub fn to_action(&self) -> Action {
        let reason = match self {
            ContentFilterBlock::Block(ids) => json!({
                "initiator": "content_filter",
                "tags": ids,
                "name": "block"
            }),
            ContentFilterBlock::Monitor(ids) => json!({
                "initiator": "content_filter",
                "tags": ids,
                "name": "monitor"
            }),
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
            ContentFilterBlock::Mismatch(wmatch) => json!({
                "section": wmatch.section,
                "name": wmatch.name,
                "initiator": "content_filter",
                "value": wmatch.value,
                "msg": "Mismatch"
            }),
        };
        let block_mode = !matches!(self, ContentFilterBlock::Monitor(_));

        Action {
            atype: ActionType::Block,
            block_mode,
            ban: false,
            status: 403,
            headers: None,
            reason,
            content: "Access denied".to_string(),
            extra_tags: None,
        }
    }
}

#[derive(Default)]
struct Omitted {
    entries: Section<HashSet<String>>,
    exclusions: Section<HashMap<String, HashSet<String>>>,
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
    tags: &mut Tags,
    rinfo: &RequestInfo,
    profile: &ContentFilterProfile,
    mhsdb: Option<&ContentFilterRules>,
) -> Result<(), ContentFilterBlock> {
    use SectionIdx::*;
    let mut omit = Default::default();

    // directly exit if omitted profile
    if tags.has_intersection(&profile.ignore) {
        logs.debug("content filter bypass because of global ignore");
        return Ok(());
    }

    // check section profiles
    for idx in &[Path, Headers, Cookies, Args] {
        section_check(
            logs,
            tags,
            *idx,
            profile.sections.get(*idx),
            get_section(*idx, rinfo),
            profile.ignore_alphanum,
            &mut omit,
        )?;
    }

    let kept = profile.active.union(&profile.report).cloned().collect::<HashSet<_>>();
    let test_xss = LIBINJECTION_XSS_TAGS.intersection(&profile.ignore).next().is_none()
        && LIBINJECTION_XSS_TAGS.intersection(&kept).next().is_some();
    let test_sqli = LIBINJECTION_SQLI_TAGS.intersection(&profile.ignore).next().is_none()
        && LIBINJECTION_SQLI_TAGS.intersection(&kept).next().is_some();

    let mut hca_keys: HashMap<String, (SectionIdx, String)> = HashMap::new();

    // list of non whitelisted entries
    for idx in &[Path, Headers, Cookies, Args] {
        let section_content = get_section(*idx, rinfo)
            .iter()
            .filter(|(name, _)| !omit.entries.get(*idx).contains(*name))
            .map(|(name, value)| (value.to_string(), (*idx, name.to_string())));
        hca_keys.extend(section_content);
    }

    injection_check(tags, &hca_keys, &omit, test_xss, test_sqli);

    let mut specific_tags = Tags::default();

    // finally, hyperscan check
    match mhsdb {
        Some(hsdb) => {
            if let Err(rr) = hyperscan(
                logs,
                tags,
                &mut specific_tags,
                hca_keys,
                hsdb,
                &kept,
                &profile.ignore,
                &omit.exclusions,
            ) {
                logs.error(|| rr.to_string())
            }
        }
        None => {
            logs.warning(||format!("no hsdb found for profile {}, it probably means that no rules were matched by the active/report/ignore", profile.id));
        }
    }

    let sactive = specific_tags.intersect(&profile.active);
    let sreport = specific_tags.intersect(&profile.report);
    tags.extend(specific_tags);

    if !sactive.is_empty() {
        return Err(ContentFilterBlock::Block(sactive));
    }
    if !sreport.is_empty() {
        return Err(ContentFilterBlock::Monitor(sreport));
    }

    let active = tags.intersect(&profile.active);
    if !active.is_empty() {
        return Err(ContentFilterBlock::Block(active));
    }

    let report = tags.intersect(&profile.report);
    if !report.is_empty() {
        return Err(ContentFilterBlock::Monitor(report));
    }

    Ok(())
}

/// checks a section (headers, args, cookies) against the policy
fn section_check(
    logs: &mut Logs,
    tags: &Tags,
    idx: SectionIdx,
    section: &ContentFilterSection,
    params: &RequestField,
    ignore_alphanum: bool,
    omit: &mut Omitted,
) -> Result<(), ContentFilterBlock> {
    if idx != SectionIdx::Path && params.len() > section.max_count {
        if section.max_count > 0 {
            return Err(ContentFilterBlock::TooManyEntries(idx));
        } else {
            logs.warning(|| format!("In section {:?}, param_count = 0", idx));
        }
    }

    for (name, value) in params.iter() {
        // skip decoded parameters for length checks
        if !name.ends_with(":decoded") && value.len() > section.max_length {
            if section.max_length > 0 {
                return Err(ContentFilterBlock::EntryTooLarge(idx, name.to_string()));
            } else {
                logs.warning(|| format!("In section {:?}, max_length = 0", idx));
            }
        }

        // automatically ignored
        if ignore_alphanum && value.chars().all(|c| c.is_ascii_alphanumeric()) {
            omit.entries.at(idx).insert(name.to_string());
            continue;
        }

        // logic for checking an entry
        let mut check_entry = |name_entry: &ContentFilterEntryMatch| {
            let matched = if let Some(re) = &name_entry.reg {
                re.matches(value)
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
            } else if tags.has_intersection(&name_entry.exclusions) {
                omit.entries.at(idx).insert(name.to_string());
            } else if !name_entry.exclusions.is_empty() {
                let entry = omit.exclusions.at(idx).entry(name.to_string()).or_default();
                entry.extend(name_entry.exclusions.iter().cloned());
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

/// TODO: This also populates the hca_keys map
/// this is stupid and needs to be changed
fn injection_check(
    tags: &mut Tags,
    hca_keys: &HashMap<String, (SectionIdx, String)>,
    omit: &Omitted,
    test_xss: bool,
    test_sqli: bool,
) {
    for (value, (idx, name)) in hca_keys.iter() {
        let omit_tags = omit.exclusions.get(*idx).get(name);
        let rtest_xss = test_xss
            && !omit_tags
                .map(|tgs| LIBINJECTION_XSS_TAGS.intersection(tgs).next().is_some())
                .unwrap_or(false);
        let rtest_sqli = test_sqli
            && !omit_tags
                .map(|tgs| LIBINJECTION_SQLI_TAGS.intersection(tgs).next().is_some())
                .unwrap_or(false);
        if rtest_sqli {
            if let Some((b, _)) = sqli(value) {
                if b {
                    tags.insert_qualified("cf-rule-id", "libinjection-sqli");
                    tags.insert_qualified("cf-rule-category", "libinjection");
                    tags.insert_qualified("cf-rule-subcategory", "libinjection-sqli");
                    tags.insert_qualified("cf-rule-risk", "libinjection");
                }
            }
        }
        if rtest_xss {
            if let Some(b) = xss(value) {
                if b {
                    tags.insert_qualified("cf-rule-id", "libinjection-xss");
                    tags.insert_qualified("cf-rule-category", "libinjection");
                    tags.insert_qualified("cf-rule-subcategory", "libinjection-xss");
                    tags.insert_qualified("cf-rule-risk", "libinjection");
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn hyperscan(
    logs: &mut Logs,
    tags: &mut Tags,
    specific_tags: &mut Tags,
    hca_keys: HashMap<String, (SectionIdx, String)>,
    sigs: &ContentFilterRules,
    global_kept: &HashSet<String>,
    global_ignore: &HashSet<String>,
    exclusions: &Section<HashMap<String, HashSet<String>>>,
) -> anyhow::Result<()> {
    let scratch = sigs.db.alloc_scratch()?;
    // TODO: use `intersperse` when this stabilizes
    let to_scan = hca_keys.keys().cloned().collect::<Vec<_>>().join("\n");
    let mut found = false;
    sigs.db.scan(&[to_scan], &scratch, |_, _, _, _| {
        found = true;
        Matching::Continue
    })?;
    logs.debug(|| format!("matching content filter signatures: {}", found));

    if !found {
        return Ok(());
    }

    // something matched! but what?
    for (k, (sid, name)) in hca_keys {
        sigs.db.scan(&[k.as_bytes()], &scratch, |id, _, _, _| {
            match sigs.ids.get(id as usize) {
                None => logs.error(|| format!("Should not happen, invalid hyperscan index {}", id)),
                Some(sig) => {
                    logs.debug(|| format!("signature matched {:?}", sig));

                    // new specific tags are singleton hashsets, but we use the Tags structure to make sure
                    // they are properly converted
                    let (new_specific_tags, new_tags) = rule_tags(sig);
                    if (new_tags.has_intersection(global_kept) || new_specific_tags.has_intersection(global_kept))
                        && exclusions
                            .get(sid)
                            .get(&name)
                            .map(|ex| new_tags.has_intersection(ex) || new_specific_tags.has_intersection(ex))
                            != Some(true)
                        && !new_tags.has_intersection(global_ignore)
                        && !new_specific_tags.has_intersection(global_ignore)
                    {
                        tags.extend(new_tags);
                        specific_tags.extend(new_specific_tags);
                    }
                }
            }
            Matching::Continue
        })?;
    }
    Ok(())
}

fn mask_section(masking_seed: &[u8], sec: &mut RequestField, section: &ContentFilterSection) -> HashSet<XDataSource> {
    let to_mask: Vec<String> = sec
        .iter()
        .filter(|&(name, _)| {
            if let Some(e) = section.names.get(name) {
                e.mask
            } else {
                section.regex.iter().any(|(re, e)| e.mask && re.is_match(name))
            }
        })
        .map(|(name, _)| name.to_string())
        .collect();
    to_mask.iter().flat_map(|n| sec.mask(masking_seed, n)).collect()
}

pub fn masking(masking_seed: &[u8], req: RequestInfo, profile: &ContentFilterProfile) -> RequestInfo {
    let mut ri = req;
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
        &mut ri.headers,
        profile.sections.get(SectionIdx::Headers),
    ));
    for x in to_mask {
        match x {
            // for now, do not mask the Uri
            XDataSource::Uri => (),
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
        map_request(&mut logs, &[], &[], &raw_request)
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
                    ("arg1", &DataSource::X(XDataSource::Uri), "MASKED{fac00299}"),
                    ("arg2", &DataSource::X(XDataSource::Uri), "MASKED{7ce2d8de}")
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
                    ("arg1", &DataSource::X(XDataSource::Uri), "MASKED{fac00299}"),
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
                    ("arg1", &DataSource::X(XDataSource::Uri), "MASKED{fac00299}"),
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
                    ("arg1", &DataSource::X(XDataSource::Uri), "MASKED{fac00299}"),
                    ("arg2", &DataSource::X(XDataSource::Uri), "MASKED{7ce2d8de}")
                ]
            ),
            masked.rinfo.qinfo.args
        );
    }
}
