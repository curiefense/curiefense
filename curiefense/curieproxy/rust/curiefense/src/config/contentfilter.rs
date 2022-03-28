use crate::config::raw::{
    RawContentFilterEntryMatch, RawContentFilterGroup, RawContentFilterProfile, RawContentFilterProperties,
    RawContentFilterRule,
};
use crate::logs::Logs;

use hyperscan::prelude::{pattern, Builder, CompileFlags, Pattern, Patterns, VectoredDatabase};
use hyperscan::Vectored;
use regex::Regex;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Section<A> {
    pub headers: A,
    pub cookies: A,
    pub args: A,
}

#[derive(Debug, Clone)]
pub struct ContentFilterProfile {
    pub id: String,
    pub name: String,
    pub ignore_alphanum: bool,
    pub sections: Section<ContentFilterSection>,
}

impl Default for ContentFilterProfile {
    fn default() -> Self {
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
            },
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
    pub reg: Option<Regex>,
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
}

impl<A> Section<A> {
    pub fn get(&self, idx: SectionIdx) -> &A {
        match idx {
            SectionIdx::Headers => &self.headers,
            SectionIdx::Cookies => &self.cookies,
            SectionIdx::Args => &self.args,
        }
    }

    pub fn at(&mut self, idx: SectionIdx) -> &mut A {
        match idx {
            SectionIdx::Headers => &mut self.headers,
            SectionIdx::Cookies => &mut self.cookies,
            SectionIdx::Args => &mut self.args,
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

#[derive(Debug, Clone)]
pub struct ContentFilterGroup {
    pub id: String,
    pub name: String,
    pub description: String,
    pub content_filter_rule_ids: HashSet<String>,
}

impl ContentFilterGroup {
    pub fn resolve(raws: Vec<RawContentFilterGroup>) -> HashMap<String, ContentFilterGroup> {
        raws.into_iter()
            .map(|raw| {
                (
                    raw.id.clone(),
                    ContentFilterGroup {
                        id: raw.id,
                        name: raw.name,
                        description: raw.description,
                        content_filter_rule_ids: match raw.content_filter_rule_ids {
                            Some(p) => p.into_iter().collect(),
                            None => HashSet::new(),
                        },
                    },
                )
            })
            .collect()
    }
}

fn mk_entry_match(
    em: RawContentFilterEntryMatch,
    content_filter_groups: &HashMap<String, ContentFilterGroup>,
) -> anyhow::Result<(String, ContentFilterEntryMatch)> {
    Ok((
        em.key,
        ContentFilterEntryMatch {
            restrict: em.restrict,
            mask: em.mask.unwrap_or(false),
            exclusions: em
                .exclusions
                .unwrap_or_default()
                .into_iter()
                .map(|(k, v)| {
                    if v == "rule" {
                        [k].iter().cloned().collect::<HashSet<String>>()
                    } else {
                        match content_filter_groups.get(&k) {
                            Some(p) => p.content_filter_rule_ids.clone(),
                            None => HashSet::new(),
                        }
                    }
                })
                .flatten()
                .collect::<HashSet<_>>(),
            reg: em.reg.map(|s| Regex::new(&s)).transpose()?, // lol not Haskell
        },
    ))
}

fn mk_section(
    props: RawContentFilterProperties,
    max_length: usize,
    max_count: usize,
    content_filter_groups: &HashMap<String, ContentFilterGroup>,
) -> anyhow::Result<ContentFilterSection> {
    let mnames: anyhow::Result<HashMap<String, ContentFilterEntryMatch>> = props
        .names
        .into_iter()
        .map(|e| mk_entry_match(e, content_filter_groups))
        .collect();
    let mregex: anyhow::Result<Vec<(Regex, ContentFilterEntryMatch)>> = props
        .regex
        .into_iter()
        .map(|e| {
            let (s, v) = mk_entry_match(e, content_filter_groups)?;
            let re = Regex::new(&s)?;
            Ok((re, v))
        })
        .collect();
    Ok(ContentFilterSection {
        max_count,
        max_length,
        names: mnames?,
        regex: mregex?,
    })
}

fn convert_entry(
    entry: RawContentFilterProfile,
    content_filter_groups: &HashMap<String, ContentFilterGroup>,
) -> anyhow::Result<(String, ContentFilterProfile)> {
    Ok((
        entry.id.clone(),
        ContentFilterProfile {
            id: entry.id,
            name: entry.name,
            ignore_alphanum: entry.ignore_alphanum,
            sections: Section {
                headers: mk_section(
                    entry.headers,
                    entry.max_header_length,
                    entry.max_headers_count,
                    content_filter_groups,
                )?,
                cookies: mk_section(
                    entry.cookies,
                    entry.max_cookie_length,
                    entry.max_cookies_count,
                    content_filter_groups,
                )?,
                args: mk_section(
                    entry.args,
                    entry.max_arg_length,
                    entry.max_args_count,
                    content_filter_groups,
                )?,
            },
        },
    ))
}

impl ContentFilterProfile {
    pub fn resolve(
        logs: &mut Logs,
        raw: Vec<RawContentFilterProfile>,
        content_filter_groups: &HashMap<String, ContentFilterGroup>,
    ) -> HashMap<String, ContentFilterProfile> {
        let mut out = HashMap::new();
        for rp in raw {
            let id = rp.id.clone();
            match convert_entry(rp, content_filter_groups) {
                Ok((k, v)) => {
                    out.insert(k, v);
                }
                Err(rr) => logs.error(format!("content filter id {}: {:?}", id, rr)),
            }
        }
        out
    }
}

#[derive(Debug, Clone)]
pub struct ContentFilterRule {
    pub id: String,
    pub name: String,
    pub msg: String,
    pub operand: String,
    pub severity: u8,
    pub certainity: u8,
    pub category: String,
    pub subcategory: String,
    pub groups: HashMap<String, String>,
}

fn convert_rule(entry: &ContentFilterRule) -> anyhow::Result<Pattern> {
    Pattern::with_flags(
        &entry.operand,
        CompileFlags::MULTILINE | CompileFlags::DOTALL | CompileFlags::CASELESS,
    )
}

pub fn resolve_rules(
    raws: Vec<RawContentFilterRule>,
    content_filter_groups: &HashMap<String, ContentFilterGroup>,
) -> anyhow::Result<ContentFilterRules> {
    let mut rule_id_groups: HashMap<String, HashMap<String, String>> = HashMap::new();
    for cfg in content_filter_groups.values() {
        for rule_id in cfg.content_filter_rule_ids.iter() {
            rule_id_groups
                .entry(rule_id.to_string())
                .or_default()
                .insert(cfg.id.clone(), cfg.name.clone());
        }
    }
    let rules: Vec<ContentFilterRule> = raws
        .into_iter()
        .map(|raw| ContentFilterRule {
            id: raw.id.clone(),
            name: raw.name,
            msg: raw.msg,
            operand: raw.operand,
            severity: raw.severity,
            certainity: raw.certainity,
            category: raw.category,
            subcategory: raw.subcategory,
            groups: match rule_id_groups.get(&raw.id).cloned() {
                Some(groups) => groups,
                None => HashMap::new(),
            },
        })
        .collect();
    let patterns: anyhow::Result<Vec<Pattern>> = rules.iter().map(convert_rule).collect();
    let ptrns: Patterns = Patterns::from_iter(patterns?);
    Ok(ContentFilterRules {
        db: ptrns.build::<Vectored>()?,
        ids: rules,
    })
}
