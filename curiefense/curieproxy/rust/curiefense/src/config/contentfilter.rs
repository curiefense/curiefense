use crate::config::matchers::Matching;
use crate::config::raw::{
    ContentType, RawContentFilterEntryMatch, RawContentFilterProfile, RawContentFilterProperties, RawContentFilterRule,
};
use crate::interface::{RawTags, SimpleAction};
use crate::logs::Logs;

use hyperscan::prelude::{pattern, Builder, CompileFlags, Pattern, Patterns, VectoredDatabase};
use hyperscan::Vectored;
use regex::{Regex, RegexBuilder};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Section<A> {
    pub headers: A,
    pub cookies: A,
    pub args: A,
    pub path: A,
    pub plugins: A,
}

#[derive(Debug, Clone)]
pub struct ContentFilterProfile {
    pub id: String,
    pub name: String,
    pub active: HashSet<String>,
    pub ignore: HashSet<String>,
    pub report: HashSet<String>,
    pub ignore_alphanum: bool,
    pub sections: Section<ContentFilterSection>,
    pub decoding: Vec<Transformation>,
    pub masking_seed: Vec<u8>,
    pub content_type: Vec<ContentType>,
    pub ignore_body: bool,
    pub max_body_size: usize,
    pub max_body_depth: usize,
    pub referer_as_uri: bool,
    pub graphql_path: String,
    pub action: SimpleAction,
    pub tags: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct ContentFilterRule {
    pub id: String,
    pub operand: String,
    pub risk: u8,
    pub category: String,
    pub subcategory: String,
    pub tags: HashSet<String>,
    pub pattern: Pattern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transformation {
    Base64Decode,
    HtmlEntitiesDecode,
    UnicodeDecode,
    UrlDecode,
}

impl ContentFilterProfile {
    pub fn default_from_seed(seed: &str) -> Self {
        ContentFilterProfile {
            id: "__default__".to_string(),
            name: "default contentfilter".to_string(),
            ignore_alphanum: true,
            sections: Section {
                headers: ContentFilterSection {
                    max_count: 42,
                    max_length: 1024,
                    names: HashMap::new(),
                    regex: Vec::new(),
                },
                args: ContentFilterSection {
                    max_count: 512,
                    max_length: 1024,
                    names: HashMap::new(),
                    regex: Vec::new(),
                },
                cookies: ContentFilterSection {
                    max_count: 42,
                    max_length: 1024,
                    names: HashMap::new(),
                    regex: Vec::new(),
                },
                path: ContentFilterSection {
                    max_count: 42,
                    max_length: 1024,
                    names: HashMap::new(),
                    regex: Vec::new(),
                },
                plugins: ContentFilterSection {
                    max_count: usize::MAX,
                    max_length: usize::MAX,
                    names: HashMap::new(),
                    regex: Vec::new(),
                },
            },
            decoding: vec![Transformation::Base64Decode, Transformation::UrlDecode],
            masking_seed: seed.as_bytes().to_vec(),
            active: HashSet::default(),
            ignore: HashSet::default(),
            report: HashSet::default(),
            content_type: Vec::new(),
            ignore_body: false,
            max_body_size: usize::MAX,
            max_body_depth: usize::MAX,
            referer_as_uri: false,
            graphql_path: "".to_string(),
            action: SimpleAction::default(),
            tags: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContentFilterSection {
    pub max_count: usize,
    pub max_length: usize,
    pub names: HashMap<String, ContentFilterEntryMatch>,
    pub regex: Vec<(Regex, ContentFilterEntryMatch)>,
}

#[derive(Debug, Clone)]
pub struct ContentFilterEntryMatch {
    pub reg: Option<Matching<String>>,
    pub restrict: bool,
    pub mask: bool,
    pub exclusions: HashSet<String>,
}

#[derive(Debug, Clone, Eq, Serialize, PartialEq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SectionIdx {
    Headers,
    Cookies,
    Args,
    Path,
    Plugins,
}

pub const ALL_SECTION_IDX: [SectionIdx; 5] = [
    SectionIdx::Headers,
    SectionIdx::Cookies,
    SectionIdx::Args,
    SectionIdx::Path,
    SectionIdx::Plugins,
];

pub const ALL_SECTION_IDX_NO_PLUGINS: [SectionIdx; 4] = [
    SectionIdx::Headers,
    SectionIdx::Cookies,
    SectionIdx::Args,
    SectionIdx::Path,
];

impl<A> Section<A> {
    pub fn get(&self, idx: SectionIdx) -> &A {
        match idx {
            SectionIdx::Headers => &self.headers,
            SectionIdx::Cookies => &self.cookies,
            SectionIdx::Args => &self.args,
            SectionIdx::Path => &self.path,
            SectionIdx::Plugins => &self.plugins,
        }
    }

    pub fn at(&mut self, idx: SectionIdx) -> &mut A {
        match idx {
            SectionIdx::Headers => &mut self.headers,
            SectionIdx::Cookies => &mut self.cookies,
            SectionIdx::Args => &mut self.args,
            SectionIdx::Path => &mut self.path,
            SectionIdx::Plugins => &mut self.plugins,
        }
    }
}

impl<A> Default for Section<A>
where
    A: Default,
{
    fn default() -> Self {
        Section {
            headers: Default::default(),
            cookies: Default::default(),
            args: Default::default(),
            path: Default::default(),
            plugins: Default::default(),
        }
    }
}

pub struct ContentFilterRules {
    pub db: VectoredDatabase,
    pub ids: Vec<ContentFilterRule>,
}

impl ContentFilterRules {
    pub fn empty() -> Self {
        let pattern: Pattern = pattern! { "^TEST$" };
        ContentFilterRules {
            db: pattern.build().unwrap(),
            ids: Vec::new(),
        }
    }
}

const fn nonzero(value: usize) -> usize {
    if value == 0 {
        usize::MAX
    } else {
        value
    }
}

fn mk_entry_match(
    em: RawContentFilterEntryMatch,
    lowercase_key: bool,
) -> anyhow::Result<(String, ContentFilterEntryMatch)> {
    let reg = match em.reg {
        None => None,
        Some(s) => {
            if s.is_empty() {
                None
            } else {
                Some(Matching::from_str(&s, s.clone())?)
            }
        }
    };

    Ok((
        if lowercase_key {
            em.key.to_ascii_lowercase()
        } else {
            em.key
        },
        ContentFilterEntryMatch {
            restrict: em.restrict,
            mask: em.mask.unwrap_or(false),
            exclusions: em.exclusions.into_iter().collect::<HashSet<_>>(),
            reg,
        },
    ))
}

fn mk_section(
    allsections: &RawContentFilterProperties,
    props: RawContentFilterProperties,
    lowercase_key: bool,
) -> anyhow::Result<ContentFilterSection> {
    // allsections entries are iterated first, so that they are replaced by entries in prop in case of colision
    // however, max_count and max_length in allsections are ignored
    let mnames: anyhow::Result<HashMap<String, ContentFilterEntryMatch>> = allsections
        .names
        .iter()
        .cloned()
        .chain(props.names.into_iter())
        .map(|em| mk_entry_match(em, lowercase_key))
        .collect();
    let mregex: anyhow::Result<Vec<(Regex, ContentFilterEntryMatch)>> = allsections
        .regex
        .iter()
        .cloned()
        .chain(props.regex.into_iter())
        .map(|e| {
            let (s, v) = mk_entry_match(e, lowercase_key)?;
            let re = RegexBuilder::new(&s).case_insensitive(true).build()?;
            Ok((re, v))
        })
        .collect();
    Ok(ContentFilterSection {
        max_count: nonzero(props.max_count.0),
        max_length: nonzero(props.max_length.0),
        names: mnames?,
        regex: mregex?,
    })
}

fn convert_entry(
    logs: &mut Logs,
    actions: &HashMap<String, SimpleAction>,
    entry: RawContentFilterProfile,
) -> anyhow::Result<(String, ContentFilterProfile)> {
    let mut decoding = Vec::new();
    // default order
    if entry.decoding.base64 {
        decoding.push(Transformation::Base64Decode)
    }
    if entry.decoding.dual {
        decoding.push(Transformation::UrlDecode)
    }
    if entry.decoding.html {
        decoding.push(Transformation::HtmlEntitiesDecode)
    }
    if entry.decoding.unicode {
        decoding.push(Transformation::UnicodeDecode)
    }
    let max_body_size = nonzero(entry.max_body_size.unwrap_or(usize::MAX));
    let max_body_depth = nonzero(entry.max_body_depth.unwrap_or(usize::MAX));
    let id = entry.id;
    let action = match entry.action {
        None => SimpleAction::default(),
        Some(aid) => actions.get(&aid).cloned().unwrap_or_else(|| {
            logs.error(|| {
                format!(
                    "Could not resolve action {} when resolving content filter entry {}",
                    aid, id,
                )
            });
            SimpleAction::default()
        }),
    };
    Ok((
        id.clone(),
        ContentFilterProfile {
            id,
            name: entry.name,
            ignore_alphanum: entry.ignore_alphanum,
            sections: Section {
                headers: mk_section(&entry.allsections, entry.headers, true)?,
                cookies: mk_section(&entry.allsections, entry.cookies, false)?,
                args: mk_section(&entry.allsections, entry.args, false)?,
                path: mk_section(&entry.allsections, entry.path, false)?,
                plugins: mk_section(&entry.allsections, entry.plugins, false)?,
            },
            decoding,
            masking_seed: entry.masking_seed.as_bytes().to_vec(),
            active: entry.active.into_iter().collect(),
            ignore: entry.ignore.into_iter().collect(),
            report: entry.report.into_iter().collect(),
            content_type: entry.content_type,
            ignore_body: entry.ignore_body,
            max_body_size,
            max_body_depth,
            referer_as_uri: entry.referer_as_uri,
            graphql_path: entry.graphql_path,
            action,
            tags: entry.tags.into_iter().collect(),
        },
    ))
}

impl ContentFilterProfile {
    pub fn resolve(
        logs: &mut Logs,
        actions: &HashMap<String, SimpleAction>,
        raw: Vec<RawContentFilterProfile>,
    ) -> HashMap<String, ContentFilterProfile> {
        let mut out = HashMap::new();
        for rp in raw {
            let id = rp.id.clone();
            match convert_entry(logs, actions, rp) {
                Ok((k, v)) => {
                    out.insert(k, v);
                }
                Err(rr) => logs.error(|| format!("content filter id {}: {:?}", id, rr)),
            }
        }
        out
    }
}

pub fn convert_rule(entry: RawContentFilterRule) -> anyhow::Result<ContentFilterRule> {
    // try to catch pattern compilation errors and log them, ignoring the bad pattern
    let pattern = Pattern::with_flags(
        &entry.operand,
        CompileFlags::MULTILINE | CompileFlags::DOTALL | CompileFlags::CASELESS,
    )
    .map_err(|rr| {
        anyhow::anyhow!(
            "when converting content filter rule {}, pattern {:?}: {}",
            &entry.id,
            &entry.operand,
            rr
        )
    })?;
    Patterns::from_iter(std::iter::once(pattern.clone()))
        .build::<Vectored>()
        .map_err(|rr| {
            anyhow::anyhow!(
                "when converting content filter rule {}, pattern {:?}: {}",
                &entry.id,
                &entry.operand,
                rr
            )
        })?;
    Ok(ContentFilterRule {
        id: entry.id,
        operand: entry.operand,
        risk: entry.risk,
        category: entry.category,
        subcategory: entry.subcategory,
        tags: entry.tags,
        pattern,
    })
}

pub fn rule_tags(sig: &ContentFilterRule) -> (RawTags, RawTags) {
    let mut new_specific_tags = RawTags::default();
    new_specific_tags.insert_qualified("cf-rule-id", &sig.id);

    let mut new_tags = RawTags::default();
    new_tags.insert_qualified("cf-rule-risk", &format!("{}", sig.risk));
    new_tags.insert_qualified("cf-rule-category", &sig.category);
    new_tags.insert_qualified("cf-rule-subcategory", &sig.subcategory);
    for t in &sig.tags {
        new_tags.insert(t);
    }
    (new_specific_tags, new_tags)
}

pub fn resolve_rules(
    logs: &mut Logs,
    profiles: &HashMap<String, ContentFilterProfile>,
    rules: Vec<ContentFilterRule>,
) -> HashMap<String, ContentFilterRules> {
    // extend the rule tags with the group tags
    // should a given rule be kept for a given profile
    let rule_kept = |r: &ContentFilterRule, prof: &ContentFilterProfile| -> bool {
        let (spec_tags, all_tags) = rule_tags(r);
        // not pretty :)
        if spec_tags.has_intersection(&prof.ignore) {
            return false;
        }
        if all_tags.has_intersection(&prof.ignore) {
            return false;
        }
        if spec_tags.has_intersection(&prof.active) {
            return true;
        }
        if all_tags.has_intersection(&prof.active) {
            return true;
        }
        if spec_tags.has_intersection(&prof.report) {
            return true;
        }
        if all_tags.has_intersection(&prof.report) {
            return true;
        }
        false
    };

    let build_from_profile = |prof: &ContentFilterProfile| -> anyhow::Result<ContentFilterRules> {
        let ids: Vec<ContentFilterRule> = rules.iter().filter(|r| rule_kept(r, prof)).cloned().collect();
        if ids.is_empty() {
            return Err(anyhow::anyhow!("no rules were selected, empty profile"));
        }
        Patterns::from_iter(ids.iter().map(|i| i.pattern.clone()))
            .build::<Vectored>()
            .map(|db| ContentFilterRules { db, ids })
    };

    let mut out: HashMap<String, ContentFilterRules> = HashMap::new();

    for v in profiles.values() {
        match build_from_profile(v) {
            Ok(p) => {
                logs.debug(|| format!("Loaded profile {} with {} rules", v.id, p.ids.len()));
                out.insert(v.id.to_string(), p);
            }
            Err(rr) => logs.warning(|| format!("When building profile {}, error: {}", v.id, rr)),
        }
    }

    out
}
