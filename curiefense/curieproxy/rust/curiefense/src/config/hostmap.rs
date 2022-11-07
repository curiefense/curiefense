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
    pub tags: Vec<String>,
    pub acl_active: bool,
    pub acl_profile: AclProfile,
    pub content_filter_active: bool,
    pub content_filter_profile: ContentFilterProfile,
    pub limits: Vec<Limit>,
    pub session: Vec<RequestSelector>,
    pub session_ids: Vec<RequestSelector>,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            policy: PolicyId {
                id: "polid".to_string(),
                name: "policy name".to_string(),
            },
            entry: PolicyId {
                id: "entryid".to_string(),
                name: "entry name".to_string(),
            },
            tags: Vec::new(),
            acl_active: false,
            acl_profile: AclProfile::default(),
            content_filter_active: false,
            content_filter_profile: ContentFilterProfile::default_from_seed("CHANGEME"),
            limits: Vec::new(),
            session: Vec::new(),
            session_ids: Vec::new(),
        }
    }
}

impl SecurityPolicy {
    pub fn empty() -> Self {
        let mut out = Self {
            policy: PolicyId {
                id: "polid".to_string(),
                name: "policy name".to_string(),
            },
            entry: PolicyId {
                id: "entryid".to_string(),
                name: "entry name".to_string(),
            },
            tags: Vec::new(),
            acl_active: false,
            acl_profile: AclProfile::default(),
            content_filter_active: false,
            content_filter_profile: ContentFilterProfile::default_from_seed("CHANGEME"),
            limits: Vec::new(),
            session: Vec::new(),
            session_ids: Vec::new(),
        };
        out.content_filter_profile.content_type = Vec::new();
        out.content_filter_profile.decoding = Vec::new();
        out
    }
}
