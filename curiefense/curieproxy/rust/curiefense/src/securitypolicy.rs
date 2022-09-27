use std::sync::Arc;

use crate::config::hostmap::{HostMap, SecurityPolicy};
use crate::config::Config;
use crate::logs::Logs;

/// finds the securitypolicy matching a given request, based on the configuration
/// there are cases where default values do not exist (even though the UI should prevent that)
///
/// note that the url is matched using the url-decoded path!
///
/// returns the matching security policy, along with the name and id of the selected host map
pub fn match_securitypolicy<'a>(
    host: &str,
    path: &str,
    cfg: &'a Config,
    logs: &mut Logs,
) -> Option<Arc<SecurityPolicy>> {
    // find the first matching hostmap, or use the default, if it exists
    let hostmap: &HostMap = cfg
        .securitypolicies
        .iter()
        .find(|e| e.matches(host))
        .map(|m| &m.inner)
        .or(cfg.default.as_ref())?;
    logs.debug(|| format!("Selected hostmap {}", hostmap.name));
    // find the first matching securitypolicy, or use the default, if it exists
    let securitypolicy: Arc<SecurityPolicy> = match hostmap
        .entries
        .iter()
        .find(|e| e.matches(path))
        .map(|m| &m.inner)
        .or(hostmap.default.as_ref())
    {
        None => {
            logs.debug("This hostname has no default entry!");
            return None;
        }
        Some(x) => x.clone(),
    };
    logs.debug(|| format!("Selected hostmap entry {}", securitypolicy.entry.id));
    Some(securitypolicy)
}
