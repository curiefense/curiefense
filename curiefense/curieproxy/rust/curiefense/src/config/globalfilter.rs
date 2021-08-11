use anyhow::Context;
use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use iprange::IpRange;
use regex::Regex;
use serde_json::{from_value, Value};
use std::net::IpAddr;

use crate::config::raw::{
    GlobalFilterEntryType, RawGlobalFilterSSection, RawGlobalFilterSSectionEntry, RawGlobalFilterSection, Relation,
};
use crate::interface::{SimpleAction, Tags};
use crate::logs::Logs;

#[derive(Debug, Clone)]
pub struct GlobalFilterSection {
    pub tags: Tags,
    pub relation: Relation,
    pub sections: Vec<GlobalFilterSSection>,
    pub action: Option<SimpleAction>,
}

#[derive(Debug, Clone)]
pub struct GlobalFilterSSection {
    pub relation: Relation,
    pub entries: Vec<GlobalFilterEntry>,
}

#[derive(Debug, Clone)]
pub struct GlobalFilterEntry {
    pub negated: bool,
    pub entry: GlobalFilterEntryE,
}

#[derive(Debug, Clone)]
pub struct SingleEntry {
    pub exact: String,
    pub re: Option<Regex>,
}

#[derive(Debug, Clone)]
pub struct PairEntry {
    pub key: String,
    pub exact: String,
    pub re: Option<Regex>,
}

#[derive(Debug, Clone)]
pub enum GlobalFilterEntryE {
    // pairs
    Args(PairEntry),
    Cookies(PairEntry),
    Header(PairEntry),

    // ip/iprange
    Ip(IpAddr),
    Network(IpNet),
    Range4(IpRange<Ipv4Net>),
    Range6(IpRange<Ipv6Net>),

    // single - the string has to be kept because exact matches are performed as well as regex matches
    Path(SingleEntry),
    Query(SingleEntry),
    Uri(SingleEntry),
    Country(SingleEntry),
    Method(SingleEntry),
    Asn(u32),
}

/// tries to aggregate ip ranges
pub fn optimize_ipranges(rel: Relation, unoptimized: Vec<GlobalFilterEntry>) -> Vec<GlobalFilterEntry> {
    let mut p4: Vec<Ipv4Net> = Vec::new();
    let mut n4: Vec<Ipv4Net> = Vec::new();
    let mut p6: Vec<Ipv6Net> = Vec::new();
    let mut n6: Vec<Ipv6Net> = Vec::new();
    let mut other: Vec<GlobalFilterEntry> = Vec::new();

    // separate ip entries into postive/negative stacks
    // there is a way to do it in a much more optimal way by traversing the vector once
    // hopefuly this will be simpler to understand
    for e in unoptimized {
        match e.entry {
            GlobalFilterEntryE::Network(IpNet::V4(r4)) => {
                if e.negated {
                    n4.push(r4)
                } else {
                    p4.push(r4)
                }
            }
            GlobalFilterEntryE::Network(IpNet::V6(r6)) => {
                if e.negated {
                    n6.push(r6)
                } else {
                    p6.push(r6)
                }
            }
            GlobalFilterEntryE::Ip(IpAddr::V4(i4)) => {
                let r4 = Ipv4Net::from(i4);
                if e.negated {
                    n4.push(r4)
                } else {
                    p4.push(r4)
                }
            }
            GlobalFilterEntryE::Ip(IpAddr::V6(i6)) => {
                let r6 = Ipv6Net::from(i6);
                if e.negated {
                    n6.push(r6)
                } else {
                    p6.push(r6)
                }
            }
            _ => other.push(e),
        }
    }

    fn torange<N: iprange::IpNet>(n: N) -> IpRange<N> {
        let mut rng = IpRange::new();
        rng.add(n);
        rng
    }

    fn union<N: iprange::IpNet>(elems: Vec<N>) -> IpRange<N> {
        let mut out = IpRange::<N>::new();
        for e in elems {
            out.add(e);
        }
        out.simplify();
        out
    }
    fn intersection<N: iprange::IpNet>(elems: Vec<N>) -> IpRange<N> {
        // this is a bit convoluted but the first element of the fold must be
        // an element that is to be intersected, and not the empty set (as it
        // would always return the empty set)
        let mut i = elems.into_iter();
        match i.next() {
            None => {
                println!("invariant violated, elems is empty! Please report this.");
                IpRange::default()
            }
            Some(first) => i
                .map(torange)
                .fold(torange(first), |currange, p| currange.intersect(&p)),
        }
    }

    if !p4.is_empty() {
        other.push(GlobalFilterEntry {
            negated: false,
            entry: GlobalFilterEntryE::Range4(match rel {
                Relation::And => intersection(p4),
                Relation::Or => union(p4),
            }),
        });
    }
    if !n4.is_empty() {
        other.push(GlobalFilterEntry {
            negated: true,
            entry: GlobalFilterEntryE::Range4(match rel {
                Relation::And => union(n4),
                Relation::Or => intersection(n4),
            }),
        });
    }
    if !p6.is_empty() {
        other.push(GlobalFilterEntry {
            negated: false,
            entry: GlobalFilterEntryE::Range6(match rel {
                Relation::And => intersection(p6),
                Relation::Or => union(p6),
            }),
        });
    }
    if !n6.is_empty() {
        other.push(GlobalFilterEntry {
            negated: true,
            entry: GlobalFilterEntryE::Range6(match rel {
                Relation::And => union(n6),
                Relation::Or => intersection(n6),
            }),
        });
    }

    other
}

impl GlobalFilterSection {
    // what an ugly function :(
    pub fn resolve(logs: &mut Logs, rawglobalfilters: Vec<RawGlobalFilterSection>) -> Vec<GlobalFilterSection> {
        /// build a global filter entry for "single" conditions
        fn single<F>(conv: F, val: Value) -> anyhow::Result<GlobalFilterEntry>
        where
            F: FnOnce(&str) -> anyhow::Result<GlobalFilterEntryE>,
        {
            let sval: String = from_value(val)?;
            Ok(match &sval.strip_prefix('!') {
                None => GlobalFilterEntry {
                    negated: false,
                    entry: conv(&sval)?,
                },
                Some(nval) => GlobalFilterEntry {
                    negated: true,
                    entry: conv(nval)?,
                },
            })
        }

        /// build a global filter entry for "single" conditions that match strings
        fn single_re<F>(logs: &mut Logs, conv: F, val: Value) -> anyhow::Result<GlobalFilterEntry>
        where
            F: FnOnce(SingleEntry) -> GlobalFilterEntryE,
        {
            single(
                |s| {
                    Ok(conv(SingleEntry {
                        exact: s.to_string(),
                        re: match Regex::new(s) {
                            Ok(r) => Some(r),
                            Err(rr) => {
                                logs.error(format!("Bad regex {}: {}", s, rr));
                                None
                            }
                        },
                    }))
                },
                val,
            )
        }

        /// build a global filter entry for "pair" conditions
        fn pair<F>(logs: &mut Logs, conv: F, val: Value) -> anyhow::Result<GlobalFilterEntry>
        where
            F: FnOnce(PairEntry) -> GlobalFilterEntryE,
        {
            let (k, v): (String, String) = match from_value::<(String, String, Value)>(val.clone()) {
                Err(_) => from_value(val)?,
                Ok((k, v, _)) => (k, v),
            };
            Ok(match &v.strip_prefix('!') {
                None => GlobalFilterEntry {
                    negated: false,
                    entry: conv(PairEntry {
                        key: k,
                        re: match Regex::new(&v) {
                            Ok(r) => Some(r),
                            Err(rr) => {
                                logs.error(format!("Bad regex {}: {}", v, rr));
                                None
                            }
                        },
                        exact: v,
                    }),
                },
                Some(nval) => GlobalFilterEntry {
                    negated: true,
                    entry: conv(PairEntry {
                        key: k,
                        re: match Regex::new(nval) {
                            Ok(r) => Some(r),
                            Err(rr) => {
                                logs.error(format!("Bad regex {}: {}", nval, rr));
                                None
                            }
                        },
                        exact: nval.to_string(),
                    }),
                },
            })
        }

        // convert a json value
        fn convert_entry(logs: &mut Logs, tp: GlobalFilterEntryType, val: Value) -> anyhow::Result<GlobalFilterEntry> {
            match tp {
                GlobalFilterEntryType::Ip => single(
                    |rawip| {
                        Ok(if rawip.contains('/') {
                            GlobalFilterEntryE::Network(rawip.parse().with_context(|| format!("net: {}", rawip))?)
                        } else {
                            GlobalFilterEntryE::Ip(rawip.parse().with_context(|| format!("ip: {}", rawip))?)
                        })
                    },
                    val,
                ),
                GlobalFilterEntryType::Args => pair(logs, GlobalFilterEntryE::Args, val),
                GlobalFilterEntryType::Cookies => pair(logs, GlobalFilterEntryE::Cookies, val),
                GlobalFilterEntryType::Headers => pair(logs, GlobalFilterEntryE::Header, val),
                GlobalFilterEntryType::Path => single_re(logs, GlobalFilterEntryE::Path, val),
                GlobalFilterEntryType::Query => single_re(logs, GlobalFilterEntryE::Query, val),
                GlobalFilterEntryType::Uri => single_re(logs, GlobalFilterEntryE::Uri, val),
                GlobalFilterEntryType::Country => single_re(logs, GlobalFilterEntryE::Country, val),
                GlobalFilterEntryType::Method => single_re(logs, GlobalFilterEntryE::Method, val),
                GlobalFilterEntryType::Asn => single(|rawasn| Ok(GlobalFilterEntryE::Asn(rawasn.parse()?)), val),
            }
        }
        fn convert_subsection(logs: &mut Logs, ss: RawGlobalFilterSSection) -> anyhow::Result<GlobalFilterSSection> {
            // convert all entries individually
            let rentries: anyhow::Result<Vec<GlobalFilterEntry>> = ss
                .entries
                .into_iter()
                .map(|RawGlobalFilterSSectionEntry { tp, vl, comment }| {
                    convert_entry(logs, tp, vl).with_context(|| format!("Entry type={:?} comment={:?}", tp, comment))
                })
                .collect();
            Ok(GlobalFilterSSection {
                relation: ss.relation,
                entries: optimize_ipranges(ss.relation, rentries?),
            })
        }
        fn convert_section(logs: &mut Logs, s: RawGlobalFilterSection) -> anyhow::Result<GlobalFilterSection> {
            let sname = &s.name;
            let sid = &s.id;
            let rsubsections: anyhow::Result<Vec<GlobalFilterSSection>> = s
                .rule
                .sections
                .into_iter()
                .map(|ss| convert_subsection(logs, ss))
                .collect();
            let subsections: Vec<GlobalFilterSSection> = rsubsections
                .with_context(|| format!("global filter configuration error in section id={}, name={}", sid, sname))?;
            let action = match &s.action {
                Some(ma) => Some(SimpleAction::resolve(ma).with_context(|| "when resolving the action entry")?),
                None => None,
            };
            Ok(GlobalFilterSection {
                tags: Tags::from_slice(&s.tags),
                relation: s.rule.relation,
                sections: subsections,
                action,
            })
        }

        let mut out = Vec::new();

        for rgf in rawglobalfilters.into_iter().filter(|s| s.active) {
            match convert_section(logs, rgf) {
                Err(rr) => logs.error(rr),
                Ok(gfilter) => out.push(gfilter),
            }
        }

        out
    }
}
