import {
  ACLProfile,
  ContentFilterProfile,
  ContentFilterRule,
  ContentFilterRuleGroup,
  FlowControlPolicy,
  GlobalFilter,
  HttpRequestMethods,
  RateLimit,
  SecurityPolicy,
} from '@/types'

const titles: { [key: string]: string } = {
  'admin': 'Admin',
  'allow': 'Allow',
  'allow_bot': 'Allow Bot',
  'args': 'Arguments',
  'attrs': 'Attributes',
  'audit-log': 'Audit Log',
  'passthrough': 'Passthrough',
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
  'contentfilterprofiles': 'Content Filter Profiles',
  'contentfilterprofiles-singular': 'Content Filter Profile',
  'contentfilterrules': 'Content Filter Rules',
  'contentfilterrules-singular': 'Content Filter Rule',
  'globalfilters': 'Global Filters',
  'globalfilters-singular': 'Global Filter',
  'flowcontrol': 'Flow Control Policies',
  'flowcontrol-singular': 'Flow Control Policy',
  'contentfiltergroups': 'Content Filter Rules Groups',
  'contentfiltergroups-singular': 'Content Filter Rules Group',
  'active': 'Active',
  'report': 'Report',
  'ignore': 'Ignore',
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

const defaultFlowControlSequenceItem = {
  'method': 'GET' as HttpRequestMethods,
  'uri': '/',
  'cookies': {},
  'headers': {
    'host': 'www.example.com',
  },
  'args': {},
}

const newDocEntryFactory: { [key: string]: Function } = {
  aclprofiles(): ACLProfile {
    return {
      'id': generateUUID2(),
      'name': 'New ACL Profile',
      'allow': [],
      'allow_bot': [],
      'deny_bot': [],
      'passthrough': [],
      'force_deny': [],
      'deny': [],
    }
  },

  contentfilterprofiles(): ContentFilterProfile {
    return {
      'id': generateUUID2(),
      'name': 'New Content Filter Profile',
      'ignore_alphanum': true,
      'headers': {
        'names': [],
        'regex': [],
        'max_count': 42,
        'max_length': 1024,
      },
      'cookies': {
        'names': [],
        'regex': [],
        'max_count': 42,
        'max_length': 1024,
      },
      'args': {
        'names': [],
        'regex': [],
        'max_count': 512,
        'max_length': 1024,
      },
      'path': {
        'names': [],
        'regex': [],
        'max_count': 42,
        'max_length': 1024,
      },
      'decoding': {
        'base64': true,
        'dual': true,
        'html': false,
        'unicode': false,
      },
      'masking_seed': 'CHANGEME',
      'content_type': [],
      'active': ['cf-rule-risk:5', 'cf-rule-risk:4', 'cf-rule-risk:3', 'cf-rule-subcategory:libinjection-xss'],
      'report': [],
      'ignore': [],
    }
  },

  globalfilters(): GlobalFilter {
    return {
      'id': generateUUID2(),
      'name': 'New Global Filter',
      'source': 'self-managed',
      'mdate': '',
      'description': 'New Global Filter Description and Remarks',
      'active': false,
      'tags': ['trusted'],
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
          'content_filter_profile': '__default__',
          'acl_active': false,
          'content_filter_active': false,
          'limit_ids': [],
        },
      ],
    }
  },

  ratelimits(): RateLimit {
    return {
      'id': generateUUID2(),
      'name': 'New Rate Limit Rule',
      'description': 'New Rate Limit Rule Description and Remarks',
      'timeframe': '60',
      'thresholds': [
        {
          'limit': '5',
          'action': {'type': 'default'},
        },
      ],
      'include': ['all'],
      'exclude': [],
      'key': [
        {
          'attrs': 'ip',
        },
      ],
      'pairwith': {
        'self': 'self',
      },
    }
  },

  flowcontrol(): FlowControlPolicy {
    return {
      'id': generateUUID2(),
      'name': 'New Flow Control Policy',
      'description': 'New Flow Control Policy Description and Remarks',
      'active': true,
      'include': ['all'],
      'exclude': [],
      'timeframe': 60,
      'action': {
        'type': 'default',
      },
      'key': [
        {
          'attrs': 'ip',
        },
      ],
      'sequence': [
        {...defaultFlowControlSequenceItem},
        {
          ...defaultFlowControlSequenceItem,
          method: 'POST' as HttpRequestMethods,
        },
      ],
    }
  },

  contentfilterrules(): ContentFilterRule {
    return {
      'id': generateUUID2(),
      'name': 'New Content Filter Rule',
      'description': 'New Content Filter Rule Description and Remarks',
      'msg': '',
      'operand': '',
      'risk': 1,
      'category': '',
      'subcategory': '',
      'tags': [],
    }
  },

  contentfiltergroups(): ContentFilterRuleGroup {
    return {
      id: generateUUID2(),
      name: 'New Content Filter Rule Group',
      description: '',
      content_filter_rule_ids: [],
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
  defaultFlowControlSequenceItem,
}
