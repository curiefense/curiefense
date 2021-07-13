use anyhow::Context;
use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use iprange::IpRange;
use regex::Regex;
use serde_json::{from_value, Value};
use std::net::IpAddr;

use crate::config::raw::{
    ProfilingEntryType, RawProfilingSSection, RawProfilingSSectionEntry, RawProfilingSection, Relation,
};
use crate::interface::{SimpleAction, Tags};
use crate::logs::Logs;

#[derive(Debug, Clone)]
pub struct ProfilingSection {
    pub tags: Tags,
    pub relation: Relation,
    pub sections: Vec<ProfilingSSection>,
    pub action: Option<SimpleAction>,
}

#[derive(Debug, Clone)]
pub struct ProfilingSSection {
    pub relation: Relation,
    pub entries: Vec<ProfilingEntry>,
}

#[derive(Debug, Clone)]
pub struct ProfilingEntry {
    pub negated: bool,
    pub entry: ProfilingEntryE,
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
pub enum ProfilingEntryE {
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
pub fn optimize_ipranges(rel: Relation, unoptimized: Vec<ProfilingEntry>) -> Vec<ProfilingEntry> {
    let mut p4: Vec<Ipv4Net> = Vec::new();
    let mut n4: Vec<Ipv4Net> = Vec::new();
    let mut p6: Vec<Ipv6Net> = Vec::new();
    let mut n6: Vec<Ipv6Net> = Vec::new();
    let mut other: Vec<ProfilingEntry> = Vec::new();

    // separate ip entries into postive/negative stacks
    // there is a way to do it in a much more optimal way by traversing the vector once
    // hopefuly this will be simpler to understand
    for e in unoptimized {
        match e.entry {
            ProfilingEntryE::Network(IpNet::V4(r4)) => {
                if e.negated {
                    n4.push(r4)
                } else {
                    p4.push(r4)
                }
            }
            ProfilingEntryE::Network(IpNet::V6(r6)) => {
                if e.negated {
                    n6.push(r6)
                } else {
                    p6.push(r6)
                }
            }
            ProfilingEntryE::Ip(IpAddr::V4(i4)) => {
                let r4 = Ipv4Net::from(i4);
                if e.negated {
                    n4.push(r4)
                } else {
                    p4.push(r4)
                }
            }
            ProfilingEntryE::Ip(IpAddr::V6(i6)) => {
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
        other.push(ProfilingEntry {
            negated: false,
            entry: ProfilingEntryE::Range4(match rel {
                Relation::And => intersection(p4),
                Relation::Or => union(p4),
            }),
        });
    }
    if !n4.is_empty() {
        other.push(ProfilingEntry {
            negated: true,
            entry: ProfilingEntryE::Range4(match rel {
                Relation::And => union(n4),
                Relation::Or => intersection(n4),
            }),
        });
    }
    if !p6.is_empty() {
        other.push(ProfilingEntry {
            negated: false,
            entry: ProfilingEntryE::Range6(match rel {
                Relation::And => intersection(p6),
                Relation::Or => union(p6),
            }),
        });
    }
    if !n6.is_empty() {
        other.push(ProfilingEntry {
            negated: true,
            entry: ProfilingEntryE::Range6(match rel {
                Relation::And => union(n6),
                Relation::Or => intersection(n6),
            }),
        });
    }

    other
}

impl ProfilingSection {
    // what an ugly function :(
    pub fn resolve(logs: &mut Logs, rawprofiling: Vec<RawProfilingSection>) -> Vec<ProfilingSection> {
        /// build a profiling entry for "single" conditions
        fn single<F>(conv: F, val: Value) -> anyhow::Result<ProfilingEntry>
        where
            F: FnOnce(&str) -> anyhow::Result<ProfilingEntryE>,
        {
            let sval: String = from_value(val)?;
            Ok(match &sval.strip_prefix('!') {
                None => ProfilingEntry {
                    negated: false,
                    entry: conv(&sval)?,
                },
                Some(nval) => ProfilingEntry {
                    negated: true,
                    entry: conv(nval)?,
                },
            })
        }

        /// build a profiling entry for "single" conditions that match strings
        fn single_re<F>(logs: &mut Logs, conv: F, val: Value) -> anyhow::Result<ProfilingEntry>
        where
            F: FnOnce(SingleEntry) -> ProfilingEntryE,
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

        /// build a profiling entry for "pair" conditions
        fn pair<F>(logs: &mut Logs, conv: F, val: Value) -> anyhow::Result<ProfilingEntry>
        where
            F: FnOnce(PairEntry) -> ProfilingEntryE,
        {
            let (k, v): (String, String) = match from_value::<(String, String, Value)>(val.clone()) {
                Err(_) => from_value(val)?,
                Ok((k, v, _)) => (k, v),
            };
            Ok(match &v.strip_prefix('!') {
                None => ProfilingEntry {
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
                Some(nval) => ProfilingEntry {
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
        fn convert_entry(logs: &mut Logs, tp: ProfilingEntryType, val: Value) -> anyhow::Result<ProfilingEntry> {
            match tp {
                ProfilingEntryType::Ip => single(
                    |rawip| {
                        Ok(if rawip.contains('/') {
                            ProfilingEntryE::Network(rawip.parse().with_context(|| format!("net: {}", rawip))?)
                        } else {
                            ProfilingEntryE::Ip(rawip.parse().with_context(|| format!("ip: {}", rawip))?)
                        })
                    },
                    val,
                ),
                ProfilingEntryType::Args => pair(logs, ProfilingEntryE::Args, val),
                ProfilingEntryType::Cookies => pair(logs, ProfilingEntryE::Cookies, val),
                ProfilingEntryType::Headers => pair(logs, ProfilingEntryE::Header, val),
                ProfilingEntryType::Path => single_re(logs, ProfilingEntryE::Path, val),
                ProfilingEntryType::Query => single_re(logs, ProfilingEntryE::Query, val),
                ProfilingEntryType::Uri => single_re(logs, ProfilingEntryE::Uri, val),
                ProfilingEntryType::Country => single_re(logs, ProfilingEntryE::Country, val),
                ProfilingEntryType::Method => single_re(logs, ProfilingEntryE::Method, val),
                ProfilingEntryType::Asn => single(|rawasn| Ok(ProfilingEntryE::Asn(rawasn.parse()?)), val),
            }
        }
        fn convert_subsection(logs: &mut Logs, ss: RawProfilingSSection) -> anyhow::Result<ProfilingSSection> {
            // convert all entries individually
            let rentries: anyhow::Result<Vec<ProfilingEntry>> = ss
                .entries
                .into_iter()
                .map(|RawProfilingSSectionEntry { tp, vl, comment }| {
                    convert_entry(logs, tp, vl).with_context(|| format!("Entry type={:?} comment={:?}", tp, comment))
                })
                .collect();
            Ok(ProfilingSSection {
                relation: ss.relation,
                entries: optimize_ipranges(ss.relation, rentries?),
            })
        }
        fn convert_section(logs: &mut Logs, s: RawProfilingSection) -> anyhow::Result<ProfilingSection> {
            let sname = &s.name;
            let sid = &s.id;
            let rsubsections: anyhow::Result<Vec<ProfilingSSection>> = s
                .rule
                .sections
                .into_iter()
                .map(|ss| convert_subsection(logs, ss))
                .collect();
            let subsections: Vec<ProfilingSSection> = rsubsections
                .with_context(|| format!("profiling configuration error in section id={}, name={}", sid, sname))?;
            let action = match &s.action {
                Some(ma) => Some(SimpleAction::resolve(ma).with_context(|| "when resolving the action entry")?),
                None => None,
            };
            Ok(ProfilingSection {
                tags: Tags::from_slice(&s.tags),
                relation: s.rule.relation,
                sections: subsections,
                action,
            })
        }

        let mut out = Vec::new();

        for rp in rawprofiling.into_iter().filter(|s| s.active) {
            match convert_section(logs, rp) {
                Err(rr) => logs.error(rr),
                Ok(profile) => out.push(profile),
            }
        }

        out
    }
}
