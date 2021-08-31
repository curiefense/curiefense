use crate::config::raw::{RawContentFilterEntryMatch, RawContentFilterProfile, RawContentFilterProperties, ContentFilterRule};
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

// TODO: undefined data structures
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

fn mk_entry_match(em: RawContentFilterEntryMatch) -> anyhow::Result<(String, ContentFilterEntryMatch)> {
    Ok((
        em.key,
        ContentFilterEntryMatch {
            restrict: em.restrict,
            exclusions: em
                .exclusions
                .into_iter()
                .map(|mp| mp.into_iter().map(|(a, _)| a))
                .flatten()
                .collect(),
            reg: em.reg.map(|s| Regex::new(&s)).transpose()?, // lol not Haskell
        },
    ))
}

fn mk_section(props: RawContentFilterProperties, max_length: usize, max_count: usize) -> anyhow::Result<ContentFilterSection> {
    let mnames: anyhow::Result<HashMap<String, ContentFilterEntryMatch>> = props.names.into_iter().map(mk_entry_match).collect();
    let mregex: anyhow::Result<Vec<(Regex, ContentFilterEntryMatch)>> = props
        .regex
        .into_iter()
        .map(|e| {
            let (s, v) = mk_entry_match(e)?;
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

fn convert_entry(entry: RawContentFilterProfile) -> anyhow::Result<(String, ContentFilterProfile)> {
    Ok((
        entry.id.clone(),
        ContentFilterProfile {
            id: entry.id,
            name: entry.name,
            ignore_alphanum: entry.ignore_alphanum,
            sections: Section {
                headers: mk_section(entry.headers, entry.max_header_length, entry.max_headers_count)?,
                cookies: mk_section(entry.cookies, entry.max_cookie_length, entry.max_cookies_count)?,
                args: mk_section(entry.args, entry.max_arg_length, entry.max_args_count)?,
            },
        },
    ))
}

impl ContentFilterProfile {
    pub fn resolve(logs: &mut Logs, raw: Vec<RawContentFilterProfile>) -> HashMap<String, ContentFilterProfile> {
        let mut out = HashMap::new();
        for rp in raw {
            let id = rp.id.clone();
            match convert_entry(rp) {
                Ok((k, v)) => {
                    out.insert(k, v);
                }
                Err(rr) => logs.error(format!("content filter id {}: {:?}", id, rr)),
            }
        }
        out
    }
}

fn convert_rule(entry: &ContentFilterRule) -> anyhow::Result<Pattern> {
    Pattern::with_flags(
        &entry.operand,
        CompileFlags::MULTILINE | CompileFlags::DOTALL | CompileFlags::CASELESS,
    )
}

pub fn resolve_rules(raws: Vec<ContentFilterRule>) -> anyhow::Result<ContentFilterRules> {
    let patterns: anyhow::Result<Vec<Pattern>> = raws.iter().map(convert_rule).collect();
    let ptrns: Patterns = Patterns::from_iter(patterns?);
    Ok(ContentFilterRules {
        db: ptrns.build::<Vectored>()?,
        ids: raws,
    })
}
