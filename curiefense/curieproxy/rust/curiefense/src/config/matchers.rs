use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequestSelector {
    Ip,
    Path,
    Query,
    Uri,
    Country,
    Method,
    Asn,
    Args(String),
    Cookie(String),
    Header(String),
    Company,
    Authority,
    Tags,
    SecpolId,
    SecpolEntryId,
}

#[derive(Debug, Clone)]
pub enum RequestSelectorCondition {
    N(RequestSelector, Regex),
    Tag(String),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SelectorType {
    Headers,
    Cookies,
    Args,
    Attrs,
}

fn resolve_selector_type(k: &str) -> anyhow::Result<SelectorType> {
    match k {
        "headers" => Ok(SelectorType::Headers),
        "cookies" => Ok(SelectorType::Cookies),
        "args" => Ok(SelectorType::Args),
        "arguments" => Ok(SelectorType::Args),
        "attrs" => Ok(SelectorType::Attrs),
        "attributes" => Ok(SelectorType::Attrs),
        _ => Err(anyhow::anyhow!("Unknown selector type {}", k)),
    }
}

impl RequestSelector {
    // all kind of selector related functions
    pub fn decode_attribute(s: &str) -> Option<Self> {
        match s {
            "ip" => Some(RequestSelector::Ip),
            "path" => Some(RequestSelector::Path),
            "query" => Some(RequestSelector::Query),
            "uri" => Some(RequestSelector::Uri),
            "country" => Some(RequestSelector::Country),
            "method" => Some(RequestSelector::Method),
            "asn" => Some(RequestSelector::Asn),
            "company" => Some(RequestSelector::Company),
            "authority" => Some(RequestSelector::Authority),
            "tags" => Some(RequestSelector::Tags),
            "secpolid" | "securitypolicyid" | "securitypolicy" => Some(RequestSelector::SecpolId),
            "secpolentryid" | "securitypolicyentryid" | "securitypolicyentry" => Some(RequestSelector::SecpolEntryId),
            _ => None,
        }
    }

    pub fn resolve_selector_raw(k: &str, v: &str) -> anyhow::Result<Self> {
        let st = resolve_selector_type(k)?;
        Self::resolve_selector(st, v)
    }

    pub fn resolve_selector(tp: SelectorType, v: &str) -> anyhow::Result<Self> {
        match tp {
            SelectorType::Headers => Ok(RequestSelector::Header(v.to_string())),
            SelectorType::Cookies => Ok(RequestSelector::Cookie(v.to_string())),
            SelectorType::Args => Ok(RequestSelector::Args(v.to_string())),
            SelectorType::Attrs => Self::decode_attribute(v).ok_or_else(|| anyhow::anyhow!("Unknown attribute {}", v)),
        }
    }

    pub fn resolve_selector_map(sel: HashMap<String, String>) -> anyhow::Result<Self> {
        if sel.len() != 1 {
            return Err(anyhow::anyhow!("invalid selector {:?}", sel));
        }
        let (key, val) = sel.into_iter().next().unwrap();
        Self::resolve_selector_raw(&key, &val)
    }
}

pub fn decode_request_selector_condition(
    tp: SelectorType,
    v: &str,
    cond: &str,
) -> anyhow::Result<RequestSelectorCondition> {
    if tp == SelectorType::Attrs && v == "tags" {
        Ok(RequestSelectorCondition::Tag(cond.to_string()))
    } else {
        let sel = RequestSelector::resolve_selector(tp, v)?;
        let re = Regex::new(cond)?;
        Ok(RequestSelectorCondition::N(sel, re))
    }
}

#[derive(Debug, Clone)]
pub struct Matching<A> {
    negated: bool,
    matcher: Regex,
    pub inner: A,
}

impl<A> Matching<A> {
    pub fn from_str(s: &str, inner: A) -> Result<Matching<A>, regex::Error> {
        Ok(match s.strip_prefix('!') {
            None => Matching {
                negated: false,
                matcher: Regex::from_str(s)?,
                inner,
            },
            Some(r) => Matching {
                negated: true,
                matcher: Regex::from_str(r)?,
                inner,
            },
        })
    }

    pub fn matches(&self, s: &str) -> bool {
        self.matcher.is_match(s) ^ self.negated
    }

    pub fn matcher_len(&self) -> usize {
        self.matcher.as_str().len()
    }
}
