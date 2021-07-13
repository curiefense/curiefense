use anyhow::Context;
use std::collections::{HashMap, HashSet};

use crate::config::limit::{resolve_selector_map, resolve_selectors};
use crate::config::raw::{RawFlowEntry, RawFlowStep, RawLimitSelector};
use crate::config::utils::{RequestSelector, RequestSelectorCondition};
use crate::interface::SimpleAction;
use crate::logs::Logs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SequenceKey(pub String);

#[derive(Debug, Clone)]
struct FlowEntry {
    id: String,
    include: HashSet<String>,
    exclude: HashSet<String>,
    name: String,
    key: Vec<RequestSelector>,
    active: bool,
    ttl: u64,
    action: SimpleAction,
    sequence: Vec<FlowStep>,
}

#[derive(Debug, Clone)]
struct FlowStep {
    sequence_key: SequenceKey,
    select: Vec<RequestSelectorCondition>,
}

/// This is the structure that is used during tests
/// invariant : later "steps" must be present before earlier steps
#[derive(Debug, Clone)]
pub struct FlowElement {
    /// the entry id
    pub id: String,
    /// the entry include set
    pub include: HashSet<String>,
    /// the entry exclude set
    pub exclude: HashSet<String>,
    /// the entry name, which should be unique
    pub name: String,
    /// the entry key selector
    pub key: Vec<RequestSelector>,
    /// the step number
    pub step: u32,
    /// the entry ttl
    pub ttl: u64,
    /// the entry action
    pub action: SimpleAction,
    /// the step selector
    pub select: Vec<RequestSelectorCondition>,
    /// marker for the last step
    pub is_last: bool,
}

impl FlowEntry {
    fn convert(rawentry: RawFlowEntry) -> anyhow::Result<FlowEntry> {
        let mkey: anyhow::Result<Vec<RequestSelector>> = rawentry.key.into_iter().map(resolve_selector_map).collect();
        let msequence: anyhow::Result<Vec<FlowStep>> = rawentry.sequence.into_iter().map(FlowStep::convert).collect();
        let sequence = msequence?;

        Ok(FlowEntry {
            id: rawentry.id,
            include: rawentry.include.into_iter().collect(),
            exclude: rawentry.exclude.into_iter().collect(),
            name: rawentry.name,
            active: rawentry.active,
            ttl: rawentry.ttl,
            action: SimpleAction::resolve(&rawentry.action).with_context(|| "when resolving the action entry")?,
            key: mkey?,
            sequence,
        })
    }
}

impl FlowStep {
    fn convert(rawstep: RawFlowStep) -> anyhow::Result<FlowStep> {
        let sequence_key = SequenceKey(
            rawstep.method
                + rawstep
                    .headers
                    .get("host")
                    .map(|s| s.as_str())
                    .unwrap_or("Missing host field")
                + &rawstep.uri,
        );
        let mut nheaders = rawstep.headers;
        nheaders.remove("host");
        let fake_selector = RawLimitSelector {
            args: rawstep.args,
            cookies: rawstep.cookies,
            attrs: HashMap::new(),
            headers: nheaders,
        };

        Ok(FlowStep {
            sequence_key,
            select: resolve_selectors(fake_selector)?,
        })
    }
}

pub fn flow_resolve(logs: &mut Logs, rawentries: Vec<RawFlowEntry>) -> HashMap<SequenceKey, Vec<FlowElement>> {
    let mut out: HashMap<SequenceKey, Vec<FlowElement>> = HashMap::new();

    // entries are created with steps in order
    for rawentry in rawentries {
        match FlowEntry::convert(rawentry) {
            Err(rr) => logs.warning(rr),
            Ok(entry) => {
                let nsteps = entry.sequence.len();
                for (stepid, step) in entry.sequence.into_iter().enumerate() {
                    let vc: &mut Vec<FlowElement> = out.entry(step.sequence_key).or_insert_with(Vec::new);
                    vc.push(FlowElement {
                        id: entry.id.clone(),
                        action: entry.action.clone(),
                        include: entry.include.clone(),
                        exclude: entry.exclude.clone(),
                        key: entry.key.clone(),
                        name: entry.name.clone(),
                        ttl: entry.ttl,
                        select: step.select,
                        step: stepid as u32,
                        is_last: stepid + 1 == nsteps,
                    })
                }
            }
        }
    }

    // reverse step order
    for (_, o) in out.iter_mut() {
        o.reverse()
    }

    out
}
