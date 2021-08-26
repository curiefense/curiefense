import {ACLProfile, FlowControlPolicy, RateLimit, GlobalFilter, SecurityPolicy, WAFPolicy, WAFRule} from '@/types'

const titles: { [key: string]: string } = {
  'admin': 'Admin',
  'allow': 'Allow',
  'allow_bot': 'Allow Bot',
  'args': 'Arguments',
  'attrs': 'Attributes',
  'audit-log': 'Audit Log',
  'bypass': 'Bypass',
  'cookies': 'Cookies',
  'curiefense-lists': 'Curiefense Lists',
  'customsigs': 'Custom Signatures',
  'deny': 'Deny',
  'deny_bot': 'Deny Bot',
  'events-and-attacks': 'Events & Attacks',
  'external-lists': 'External Lists',
  'force_deny': 'Enforce Deny',
  'headers': 'Headers',
  'names': 'Name',
  'reg': 'Regex',
  'regex': 'Regex',
  'saml2-sso': 'SAML2 SSO',
  'top-activities': 'Top Activities',
  'traffic-overview': 'Traffic Overview',
  'update-log': 'Update log',
  'version-control': 'Version Control',
  'include': 'Include',
  'exclude': 'Exclude',
  'headers-entry': 'Header',
  'cookies-entry': 'Cookie',
  'args-entry': 'Argument',
  'attrs-entry': 'Attribute',
  'aclprofiles': 'ACL Profiles',
  'aclprofiles-singular': 'ACL Profile',
  'ratelimits': 'Rate Limits',
  'ratelimits-singular': 'Rate Limit',
  'securitypolicies': 'Security Policies',
  'securitypolicies-singular': 'Security Policy',
  'wafpolicies': 'WAF Policies',
  'wafpolicies-singular': 'WAF Policy',
  'wafrules': 'WAF Rules',
  'wafrules-singular': 'WAF Rule',
  'globalfilters': 'Global Filters',
  'globalfilters-singular': 'Global Filter',
  'flowcontrol': 'Flow Control Policies',
  'flowcontrol-singular': 'Flow Control Policy',
}

const limitOptionsTypes = {
  'headers': 'Header',
  'cookies': 'Cookie',
  'args': 'Argument',
  'attrs': 'Attribute',
}

function generateUUID(): string {
  let dt = new Date().getTime()
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    const r = (dt + Math.random() * 16) % 16 | 0
    dt = Math.floor(dt / 16)
    return (c == 'x' ? r : (r & 0x3 | 0x8)).toString(16)
  })
}

function generateUUID2(): string {
  return generateUUID().split('-')[4]
}

const newDocEntryFactory: { [key: string]: Function } = {
  aclprofiles(): ACLProfile {
    return {
      'id': generateUUID2(),
      'name': 'New ACL Profile',
      'allow': [],
      'allow_bot': [],
      'deny_bot': [],
      'bypass': [],
      'force_deny': [],
      'deny': [],
    }
  },

  wafpolicies(): WAFPolicy {
    return {
      'id': generateUUID2(),
      'name': 'New WAF Policy',
      'ignore_alphanum': true,

      'max_header_length': 1024,
      'max_cookie_length': 1024,
      'max_arg_length': 1024,

      'max_headers_count': 42,
      'max_cookies_count': 42,
      'max_args_count': 512,

      'args': {
        'names': [],
        'regex': [],
      },
      'headers': {
        'names': [],
        'regex': [],
      },
      'cookies': {
        'names': [],
        'regex': [],
      },
    }
  },

  globalfilters(): GlobalFilter {
    return {
      'id': generateUUID2(),
      'name': 'New Global Filter',
      'source': 'self-managed',
      'mdate': (new Date()).toISOString(),
      'notes': 'New List Notes and Remarks',
      'active': true,
      'tags': [],
      'action': {
        'type': 'monitor',
      },
      'rule': {
        'relation': 'OR',
        'sections': [],
      },
    }
  },

  securitypolicies(): SecurityPolicy {
    const id = generateUUID2()
    return {
      'id': id,
      'name': 'New Security Policy',
      'match': `${id}.example.com`,
      'map': [
        {
          'match': '/',
          'name': 'default',
          'acl_profile': '__default__',
          'waf_profile': '__default__',
          'acl_active': true,
          'waf_active': true,
          'limit_ids': [],
        },
      ],
    }
  },

  ratelimits(): RateLimit {
    return {
      'id': generateUUID2(),
      'description': 'New Rate Limit Rule',
      'name': 'New Rate Limit Rule',
      'limit': '3',
      'key': [
        {
          'attrs': 'ip',
        },
      ],
      'ttl': '180',
      'action': {
        'type': 'default',
      },
      'exclude': ['allowlist'],
      'include': ['blocklist'],
      'pairwith': {
        'self': 'self',
      },
    }
  },

  flowcontrol(): FlowControlPolicy {
    return {
      'id': generateUUID2(),
      'name': 'New Flow Control Policy',
      'ttl': 60,
      'active': true,
      'notes': 'New Flow Control Policy Notes and Remarks',
      'key': [
        {
          'attrs': 'ip',
        },
      ],
      'action': {
        'type': 'default',
      },
      'exclude': [],
      'include': ['all'],
      'sequence': [],
    }
  },

  wafrules(): WAFRule {
    return {
      'id': generateUUID2(),
      'name': 'New WAF Rule',
      'operand': '',
    }
  },

}

export default {
  name: 'DatasetsUtils',
  titles,
  limitOptionsTypes,
  generateUUID,
  generateUUID2,
  newDocEntryFactory,
}
