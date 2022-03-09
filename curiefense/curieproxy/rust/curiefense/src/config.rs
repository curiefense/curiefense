pub mod contentfilter;
pub mod flow;
pub mod globalfilter;
pub mod hostmap;
pub mod limit;
pub mod raw;
pub mod utils;

use crate::config::limit::Limit;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::SystemTime;

use crate::logs::Logs;
use contentfilter::{resolve_rules, ContentFilterGroup, ContentFilterProfile, ContentFilterRules};
use flow::{flow_resolve, FlowElement, SequenceKey};
use globalfilter::GlobalFilterSection;
use hostmap::{HostMap, SecurityPolicy};
use raw::{
    AclProfile, RawContentFilterGroup, RawContentFilterProfile, RawFlowEntry, RawGlobalFilterSection, RawHostMap,
    RawLimit, RawSecurityPolicy,
};
use utils::Matching;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::empty());
    pub static ref HSDB: RwLock<Option<ContentFilterRules>> = RwLock::new(None);
}

pub fn with_config<R, F>(basepath: &str, logs: &mut Logs, f: F) -> Option<R>
where
    F: FnOnce(&mut Logs, &Config) -> R,
{
    let (newconfig, newhsdb) = match CONFIG.read() {
        Ok(cfg) => match cfg.reload(logs, basepath) {
            None => return Some(f(logs, &cfg)),
            Some(cfginfo) => cfginfo,
        },
        Err(rr) =>
        // read failed :(
        {
            logs.error(rr);
            return None;
        }
    };
    let r = f(logs, &newconfig);
    match CONFIG.write() {
        Ok(mut w) => *w = newconfig,
        Err(rr) => logs.error(rr),
    };
    match HSDB.write() {
        Ok(mut dbw) => *dbw = Some(newhsdb),
        Err(rr) => logs.error(rr),
    };
    Some(r)
}

pub fn with_config_default_path<R, F>(logs: &mut Logs, f: F) -> Option<R>
where
    F: FnOnce(&mut Logs, &Config) -> R,
{
    with_config("/config/current/config", logs, f)
}

#[derive(Debug, Clone)]
pub struct Config {
    pub securitypolicies: Vec<Matching<HostMap>>,
    pub globalfilters: Vec<GlobalFilterSection>,
    pub default: Option<HostMap>,
    pub last_mod: SystemTime,
    pub container_name: Option<String>,
    pub flows: HashMap<SequenceKey, Vec<FlowElement>>,
    pub content_filter_profiles: HashMap<String, ContentFilterProfile>,
    pub content_filter_groups: HashMap<String, ContentFilterGroup>,
}

fn from_map<V: Clone>(mp: &HashMap<String, V>, k: &str) -> Result<V, String> {
    mp.get(k).cloned().ok_or_else(|| {
        let all_keys: String = mp.keys().map(|s| s.as_str()).collect::<Vec<&str>>().join(",");
        format!("id not found: {}, all ids are: {}", k, all_keys)
    })
}

#[allow(clippy::too_many_arguments)]
impl Config {
    fn resolve_security_policies(
        logs: &mut Logs,
        rawmaps: Vec<RawSecurityPolicy>,
        limits: &HashMap<String, Limit>,
        acls: &HashMap<String, AclProfile>,
        contentfilterprofiles: &HashMap<String, ContentFilterProfile>,
    ) -> (Vec<Matching<SecurityPolicy>>, Option<SecurityPolicy>) {
        let mut default: Option<SecurityPolicy> = None;
        let mut entries: Vec<Matching<SecurityPolicy>> = Vec::new();

        for rawmap in rawmaps {
            let acl_profile: AclProfile = match acls.get(&rawmap.acl_profile) {
                Some(p) => p.clone(),
                None => {
                    logs.warning(format!("Unknown ACL profile {}", &rawmap.acl_profile));
                    AclProfile::default()
                }
            };
            let content_filter_profile: ContentFilterProfile =
                match contentfilterprofiles.get(&rawmap.content_filter_profile) {
                    Some(p) => p.clone(),
                    None => {
                        logs.error(format!(
                            "Unknown Content Filter profile {}",
                            &rawmap.content_filter_profile
                        ));
                        continue
                    }
                };
            let mut olimits: Vec<Limit> = Vec::new();
            for lid in rawmap.limit_ids {
                match from_map(limits, &lid) {
                    Ok(lm) => olimits.push(lm),
                    Err(rr) => logs.error(format!("When resolving limits in rawmap {}, {}", rawmap.name, rr)),
                }
            }
            let mapname = rawmap.name.clone();
            let securitypolicy = SecurityPolicy {
                acl_active: rawmap.acl_active,
                acl_profile,
                content_filter_active: rawmap.content_filter_active,
                content_filter_profile,
                limits: olimits,
                name: rawmap.name,
            };
            if rawmap.match_ == "__default__" || (rawmap.match_ == "/" && securitypolicy.name == "default") {
                if default.is_some() {
                    logs.warning("Multiple __default__ maps");
                }
                default = Some(securitypolicy);
            } else {
                match Regex::new(&rawmap.match_) {
                    Err(rr) => logs.warning(format!(
                        "Invalid regex {} in entry {}: {}",
                        &rawmap.match_, &mapname, rr
                    )),
                    Ok(matcher) => entries.push(Matching {
                        matcher,
                        inner: securitypolicy,
                    }),
                };
            }
        }
        entries.sort_by_key(|x: &Matching<SecurityPolicy>| usize::MAX - x.matcher.as_str().len());
        (entries, default)
    }

    fn resolve(
        logs: &mut Logs,
        last_mod: SystemTime,
        rawmaps: Vec<RawHostMap>,
        rawlimits: Vec<RawLimit>,
        rawglobalfilters: Vec<RawGlobalFilterSection>,
        rawacls: Vec<AclProfile>,
        rawcontentfilterprofiles: Vec<RawContentFilterProfile>,
        rawcontentfiltergroups: Vec<RawContentFilterGroup>,
        container_name: Option<String>,
        rawflows: Vec<RawFlowEntry>,
    ) -> Config {
        let mut default: Option<HostMap> = None;
        let mut securitypolicies: Vec<Matching<HostMap>> = Vec::new();

        let limits = Limit::resolve(logs, rawlimits);
        let content_filter_groups = ContentFilterGroup::resolve(rawcontentfiltergroups);
        let content_filter_profiles =
            ContentFilterProfile::resolve(logs, rawcontentfilterprofiles, &content_filter_groups);
        let acls = rawacls.into_iter().map(|a| (a.id.clone(), a)).collect();

        // build the entries while looking for the default entry
        for rawmap in rawmaps {
            let (entries, default_entry) =
                Config::resolve_security_policies(logs, rawmap.map, &limits, &acls, &content_filter_profiles);
            if default_entry.is_none() {
                logs.warning(format!(
                    "HostMap entry '{}', id '{}' does not have a default entry",
                    rawmap.name, rawmap.id
                ));
            }
            let mapname = rawmap.name.clone();
            let hostmap = HostMap {
                id: rawmap.id,
                name: rawmap.name,
                entries,
                default: default_entry,
            };
            if rawmap.match_ == "__default__" {
                if default.is_some() {
                    logs.error(format!(
                        "HostMap entry '{}', id '{}' has several default entries",
                        hostmap.name, hostmap.id
                    ));
                }
                default = Some(hostmap);
            } else {
                match Regex::new(&rawmap.match_) {
                    Err(rr) => logs.error(format!("Invalid regex {} in entry {}: {}", &rawmap.match_, mapname, rr)),
                    Ok(matcher) => securitypolicies.push(Matching {
                        matcher,
                        inner: hostmap,
                    }),
                }
            }
        }

        // order by decreasing matcher length, so that more specific rules are matched first
        securitypolicies.sort_by(|a, b| b.matcher.as_str().len().cmp(&a.matcher.as_str().len()));

        let globalfilters = GlobalFilterSection::resolve(logs, rawglobalfilters);

        let flows = flow_resolve(logs, rawflows);

        Config {
            securitypolicies,
            globalfilters,
            default,
            last_mod,
            container_name,
            flows,
            content_filter_profiles,
            content_filter_groups,
        }
    }

    fn load_config_file<A: serde::de::DeserializeOwned>(logs: &mut Logs, base: &Path, fname: &str) -> Vec<A> {
        let mut path = base.to_path_buf();
        path.push(fname);
        let fullpath = path.to_str().unwrap_or(fname).to_string();
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(rr) => {
                logs.error(format!("when loading {}: {}", fullpath, rr));
                return Vec::new();
            }
        };
        let values: Vec<serde_json::Value> = match serde_json::from_reader(std::io::BufReader::new(file)) {
            Ok(vs) => vs,
            Err(rr) => {
                // if it is not a json array, abort early and do not resolve anything
                logs.error(format!("when parsing {}: {}", fullpath, rr));
                return Vec::new();
            }
        };
        let mut out = Vec::new();
        for value in values {
            // for each entry, try to resolve it as a raw configuration value, failing otherwise
            match serde_json::from_value(value) {
                Err(rr) => logs.error(format!("when resolving entry from {}: {}", fullpath, rr)),
                Ok(v) => out.push(v),
            }
        }
        out
    }

    pub fn reload(&self, logs: &mut Logs, basepath: &str) -> Option<(Config, ContentFilterRules)> {
        let last_mod = std::fs::metadata(basepath)
            .and_then(|x| x.modified())
            .unwrap_or_else(|rr| {
                logs.error(format!("Could not get last modified time for {}: {}", basepath, rr));
                SystemTime::now()
            });
        if self.last_mod == last_mod {
            return None;
        }

        logs.debug("Loading new configuration - CFGLOAD");
        let mut bjson = PathBuf::from(basepath);
        bjson.push("json");

        let securitypolicy = Config::load_config_file(logs, &bjson, "securitypolicy.json");
        let globalfilters = Config::load_config_file(logs, &bjson, "globalfilter-lists.json");
        let limits = Config::load_config_file(logs, &bjson, "limits.json");
        let acls = Config::load_config_file(logs, &bjson, "acl-profiles.json");
        let contentfilterprofiles = Config::load_config_file(logs, &bjson, "contentfilter-profiles.json");
        let contentfiltergroups = Config::load_config_file(logs, &bjson, "contentfilter-groups.json");
        let contentfilterrules = Config::load_config_file(logs, &bjson, "contentfilter-rules.json");
        let flows = Config::load_config_file(logs, &bjson, "flow-control.json");

        let container_name = std::fs::read_to_string("/etc/hostname")
            .ok()
            .map(|s| s.trim().to_string());

        let config = Config::resolve(
            logs,
            last_mod,
            securitypolicy,
            limits,
            globalfilters,
            acls,
            contentfilterprofiles,
            contentfiltergroups,
            container_name,
            flows,
        );
        let hsdb = resolve_rules(contentfilterrules, &config.content_filter_groups).unwrap_or_else(|rr| {
            logs.error(rr);
            ContentFilterRules::empty()
        });
        Some((config, hsdb))
    }

    pub fn empty() -> Config {
        Config {
            securitypolicies: Vec::new(),
            globalfilters: Vec::new(),
            last_mod: SystemTime::UNIX_EPOCH,
            default: None,
            container_name: None,
            flows: HashMap::new(),
            content_filter_profiles: HashMap::new(),
            content_filter_groups: HashMap::new(),
        }
    }
}

pub fn init_config() -> (bool, Vec<String>) {
    let mut logs = Logs::default();
    with_config_default_path(&mut logs, |_, _| {});
    let is_ok = logs.logs.is_empty();
    (is_ok, logs.to_stringvec())
}
