use std::sync::Arc;

use crate::config::contentfilter::ContentFilterProfile;
use crate::config::limit::Limit;
use crate::config::matchers::Matching;
use crate::config::raw::AclProfile;

use super::matchers::RequestSelector;

/// the default entry is statically encoded so that it is certain it exists
#[derive(Debug, Clone)]
pub struct HostMap {
    pub name: String,
    pub entries: Vec<Matching<Arc<SecurityPolicy>>>,
    pub default: Option<Arc<SecurityPolicy>>,
}

#[derive(Debug)]
pub struct PolicyId {
    pub id: String,
    pub name: String,
}

/// a map entry, with links to the acl and content filter profiles
#[derive(Debug)]
pub struct SecurityPolicy {
    pub policy: PolicyId,
    pub entry: PolicyId,
    pub acl_active: bool,
    pub acl_profile: AclProfile,
    pub content_filter_active: bool,
    pub content_filter_profile: ContentFilterProfile,
    pub limits: Vec<Limit>,
    pub session: Vec<RequestSelector>,
}
