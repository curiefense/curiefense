pub mod contentfilter;
pub mod flow;
pub mod globalfilter;
pub mod hostmap;
pub mod limit;
pub mod raw;
pub mod utils;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::SystemTime;

use crate::config::limit::Limit;
use crate::logs::Logs;
use contentfilter::{resolve_rules, ContentFilterProfile, ContentFilterRules};
use flow::{flow_resolve, FlowElement, SequenceKey};
use globalfilter::GlobalFilterSection;
use hostmap::{HostMap, SecurityPolicy};
use raw::{AclProfile, RawFlowEntry, RawGlobalFilterSection, RawHostMap, RawLimit, RawSecurityPolicy};
use utils::Matching;

use self::raw::RawManifest;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::empty());
    pub static ref HSDB: RwLock<HashMap<String, ContentFilterRules>> = RwLock::new(HashMap::new());
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
            logs.error(|| rr.to_string());
            return None;
        }
    };
    let r = f(logs, &newconfig);
    match CONFIG.write() {
        Ok(mut w) => *w = newconfig,
        Err(rr) => logs.error(|| rr.to_string()),
    };
    match HSDB.write() {
        Ok(mut dbw) => *dbw = newhsdb,
        Err(rr) => logs.error(|| rr.to_string()),
    };
    Some(r)
}

pub fn with_config_default_path<R, F>(logs: &mut Logs, f: F) -> Option<R>
where
    F: FnOnce(&mut Logs, &Config) -> R,
{
    with_config("/cf-config/current/config", logs, f)
}

#[derive(Debug, Clone)]
pub struct Config {
    pub revision: String,
    pub securitypolicies: Vec<Matching<HostMap>>,
    pub globalfilters: Vec<GlobalFilterSection>,
    pub default: Option<HostMap>,
    pub last_mod: SystemTime,
    pub container_name: Option<String>,
    pub flows: HashMap<SequenceKey, Vec<FlowElement>>,
    pub content_filter_profiles: HashMap<String, ContentFilterProfile>,
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
                    logs.warning(|| format!("Unknown ACL profile {}", &rawmap.acl_profile));
                    AclProfile::default()
                }
            };
            let content_filter_profile: ContentFilterProfile =
                match contentfilterprofiles.get(&rawmap.content_filter_profile) {
                    Some(p) => p.clone(),
                    None => {
                        logs.error(|| format!("Unknown Content Filter profile {}", &rawmap.content_filter_profile));
                        continue;
                    }
                };
            let mut olimits: Vec<Limit> = Vec::new();
            for lid in rawmap.limit_ids {
                match from_map(limits, &lid) {
                    Ok(lm) => olimits.push(lm),
                    Err(rr) => logs.error(format!("When resolving limits in rawmap {}, {}", rawmap.name, rr).as_str()),
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
                match Matching::from_str(&rawmap.match_, securitypolicy) {
                    Err(rr) => {
                        logs.warning(format!("Invalid regex {} in entry {}: {}", &rawmap.match_, &mapname, rr).as_str())
                    }
                    Ok(matcher) => entries.push(matcher),
                }
            }
        }
        entries.sort_by_key(|x: &Matching<SecurityPolicy>| usize::MAX - x.matcher_len());
        (entries, default)
    }

    fn resolve(
        logs: &mut Logs,
        revision: String,
        last_mod: SystemTime,
        rawmaps: Vec<RawHostMap>,
        rawlimits: Vec<RawLimit>,
        rawglobalfilters: Vec<RawGlobalFilterSection>,
        rawacls: Vec<AclProfile>,
        content_filter_profiles: HashMap<String, ContentFilterProfile>,
        container_name: Option<String>,
        rawflows: Vec<RawFlowEntry>,
    ) -> Config {
        let mut default: Option<HostMap> = None;
        let mut securitypolicies: Vec<Matching<HostMap>> = Vec::new();

        let limits = Limit::resolve(logs, rawlimits);
        let acls = rawacls.into_iter().map(|a| (a.id.clone(), a)).collect();

        // build the entries while looking for the default entry
        for rawmap in rawmaps {
            let (entries, default_entry) =
                Config::resolve_security_policies(logs, rawmap.map, &limits, &acls, &content_filter_profiles);
            if default_entry.is_none() {
                logs.warning(
                    format!(
                        "HostMap entry '{}', id '{}' does not have a default entry",
                        &rawmap.name, &rawmap.id
                    )
                    .as_str(),
                );
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
                    logs.error(|| {
                        format!(
                            "HostMap entry '{}', id '{}' has several default entries",
                            hostmap.name, hostmap.id
                        )
                    });
                }
                default = Some(hostmap);
            } else {
                match Matching::from_str(&rawmap.match_, hostmap) {
                    Err(rr) => {
                        logs.error(format!("Invalid regex {} in entry {}: {}", &rawmap.match_, mapname, rr).as_str())
                    }
                    Ok(matcher) => securitypolicies.push(matcher),
                }
            }
        }

        // order by decreasing matcher length, so that more specific rules are matched first
        securitypolicies.sort_by_key(|b| std::cmp::Reverse(b.matcher_len()));

        let globalfilters = GlobalFilterSection::resolve(logs, rawglobalfilters);

        let flows = flow_resolve(logs, rawflows);

        Config {
            revision,
            securitypolicies,
            globalfilters,
            default,
            last_mod,
            container_name,
            flows,
            content_filter_profiles,
        }
    }

    fn load_config_file<A: serde::de::DeserializeOwned>(logs: &mut Logs, base: &Path, fname: &str) -> Vec<A> {
        let mut path = base.to_path_buf();
        path.push(fname);
        let fullpath = path.to_str().unwrap_or(fname).to_string();
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(rr) => {
                logs.error(|| format!("when loading {}: {}", fullpath, rr));
                return Vec::new();
            }
        };
        let values: Vec<serde_json::Value> = match serde_json::from_reader(std::io::BufReader::new(file)) {
            Ok(vs) => vs,
            Err(rr) => {
                // if it is not a json array, abort early and do not resolve anything
                logs.error(|| format!("when parsing {}: {}", fullpath, rr));
                return Vec::new();
            }
        };
        let mut out = Vec::new();
        for value in values {
            // for each entry, try to resolve it as a raw configuration value, failing otherwise
            match serde_json::from_value(value) {
                Err(rr) => logs.error(|| format!("when resolving entry from {}: {}", fullpath, rr)),
                Ok(v) => out.push(v),
            }
        }
        out
    }

    pub fn load(
        logs: &mut Logs,
        basepath: &str,
        last_mod: SystemTime,
    ) -> (Config, HashMap<String, ContentFilterRules>) {
        logs.debug("Loading new configuration - CFGLOAD");
        let mut bjson = PathBuf::from(basepath);
        bjson.push("json");

        let mmanifest: Result<RawManifest, String> = PathBuf::from(basepath)
            .parent()
            .ok_or_else(|| "could not get parent directory?".to_string())
            .and_then(|x| {
                let mut pth = x.to_owned();
                pth.push("manifest.json");
                std::fs::File::open(pth).map_err(|rr| rr.to_string())
            })
            .and_then(|file| serde_json::from_reader(file).map_err(|rr| rr.to_string()));

        let revision = match mmanifest {
            Err(rr) => {
                logs.error(move || format!("When loading manifest.json: {}", rr));
                "unknown".to_string()
            }
            Ok(manifest) => manifest.meta.version,
        };

        let securitypolicy = Config::load_config_file(logs, &bjson, "securitypolicy.json");
        let globalfilters = Config::load_config_file(logs, &bjson, "globalfilter-lists.json");
        let limits = Config::load_config_file(logs, &bjson, "limits.json");
        let acls = Config::load_config_file(logs, &bjson, "acl-profiles.json");
        let rawcontentfilterprofiles = Config::load_config_file(logs, &bjson, "contentfilter-profiles.json");
        let contentfilterrules = Config::load_config_file(logs, &bjson, "contentfilter-rules.json");
        let contentfiltergroups = Config::load_config_file(logs, &bjson, "contentfilter-groups.json");
        let flows = Config::load_config_file(logs, &bjson, "flow-control.json");

        let container_name = std::fs::read_to_string("/etc/hostname")
            .ok()
            .map(|s| s.trim().to_string());

        let content_filter_profiles = ContentFilterProfile::resolve(logs, rawcontentfilterprofiles);

        let hsdb = resolve_rules(logs, &content_filter_profiles, contentfilterrules, contentfiltergroups);

        let config = Config::resolve(
            logs,
            revision,
            last_mod,
            securitypolicy,
            limits,
            globalfilters,
            acls,
            content_filter_profiles,
            container_name,
            flows,
        );

        (config, hsdb)
    }

    pub fn reload(&self, logs: &mut Logs, basepath: &str) -> Option<(Config, HashMap<String, ContentFilterRules>)> {
        let last_mod = std::fs::metadata(basepath)
            .and_then(|x| x.modified())
            .unwrap_or_else(|rr| {
                logs.error(|| format!("Could not get last modified time for {}: {}", basepath, rr));
                SystemTime::now()
            });
        if self.last_mod == last_mod {
            return None;
        }

        Some(Config::load(logs, basepath, last_mod))
    }

    pub fn empty() -> Config {
        Config {
            revision: "dummy".to_string(),
            securitypolicies: Vec::new(),
            globalfilters: Vec::new(),
            last_mod: SystemTime::UNIX_EPOCH,
            default: None,
            container_name: None,
            flows: HashMap::new(),
            content_filter_profiles: HashMap::new(),
        }
    }
}

pub fn init_config() -> (bool, Vec<String>) {
    let mut logs = Logs::default();
    with_config_default_path(&mut logs, |_, _| {});
    let is_ok = logs.logs.is_empty();
    (is_ok, logs.to_stringvec())
}
