use std::collections::HashMap;
use std::sync::Arc;

use crate::config::raw::RawVirtualTag;
use crate::interface::tagify;
use crate::logs::Logs;

pub type VirtualTags = Arc<HashMap<String, Vec<String>>>;

pub fn vtags_resolve(_logs: &mut Logs, rawentries: Vec<RawVirtualTag>) -> VirtualTags {
    let mut out: HashMap<String, Vec<String>> = HashMap::new();

    for rawentry in rawentries {
        for matchentry in rawentry.vmatch.into_iter() {
            let vtag = tagify(matchentry.vtag.as_str());
            for rawtag in matchentry.tags.into_iter() {
                let tag = tagify(rawtag.as_str());
                let vtags = out.entry(tag).or_insert_with(Vec::new);
                vtags.push(vtag.clone());
            }
        }
    }

    Arc::new(out)
}
