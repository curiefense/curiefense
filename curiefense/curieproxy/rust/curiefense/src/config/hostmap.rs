use crate::config::contentfilter::ContentFilterProfile;
use crate::config::limit::Limit;
use crate::config::raw::AclProfile;
use crate::config::utils::Matching;

/// the default entry is statically encoded so that it is certain it exists
#[derive(Debug, Clone)]
pub struct HostMap {
    pub id: String,
    pub name: String,
    pub entries: Vec<Matching<SecurityPolicy>>,
    pub default: Option<SecurityPolicy>,
}

/// a map entry, with links to the acl and content filter profiles
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub name: String,
    pub acl_active: bool,
    pub acl_profile: AclProfile,
    pub content_filter_active: bool,
    pub content_filter_profile: ContentFilterProfile,
    pub limits: Vec<Limit>,
}
