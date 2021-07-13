use crate::config::hostmap::{HostMap, UrlMap};
use crate::config::Config;
use crate::logs::Logs;
use crate::utils::RequestInfo;

/// finds the urlmap matching a given request, based on the configuration
/// there are cases where default values do not exist (even though the UI should prevent that)
///
/// note that the url is matched using the url-decoded path!
///
/// returns the matching url map, along with the id of the selected host map
pub fn match_urlmap<'a>(ri: &RequestInfo, cfg: &'a Config, logs: &mut Logs) -> Option<(String, &'a UrlMap)> {
    // find the first matching hostmap, or use the default, if it exists
    let hostmap: &HostMap = cfg
        .urlmaps
        .iter()
        .find(|e| e.matcher.is_match(&ri.rinfo.host))
        .map(|m| &m.inner)
        .or_else(|| cfg.default.as_ref())?;
    logs.debug(format!("Selected hostmap {}", hostmap.name));
    // find the first matching urlmap, or use the default, if it exists
    let urlmap: &UrlMap = match hostmap
        .entries
        .iter()
        .find(|e| e.matcher.is_match(&ri.rinfo.qinfo.qpath))
        .map(|m| &m.inner)
        .or_else(|| hostmap.default.as_ref())
    {
        None => {
            logs.debug("This hostname has no default entry!");
            return None;
        }
        Some(x) => x,
    };
    logs.debug(format!("Selected hostmap entry {}", urlmap.name));
    Some((hostmap.name.clone(), urlmap))
}
