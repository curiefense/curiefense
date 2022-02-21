/* eslint-disable */
import {httpRequestMethods} from './const'
declare module CuriefenseClient {

  type GenericObject = { [key: string]: any }

  type TagsNamespaceValue = {
    neutral?: string[]
    malicious?: string[]
    legitimate?: string[]
  }

  // Document types helpers - START

  type ContentFilterIgnoreType = 'rule' | 'group'

  type ContentFilterEntryMatch = {
    key: string
    reg: string
    restrict: boolean
    mask: boolean
    type: NamesRegexType
    exclusions: { [key: string]: ContentFilterIgnoreType }
  }

  type SecurityPolicyEntryMatch = {
    match: string
    name: string
    acl_profile: string
    content_filter_profile: string
    acl_active: boolean
    content_filter_active: boolean
    limit_ids: string[]
  }

  type GlobalFilterSectionEntry = [Category, string | string[], string?]

  type GlobalFilterSection = {
    entries: GlobalFilterSectionEntry[]
    relation: Relation
  }

  type LimitOptionType = {
    [key: string]: string
  }

  type ThresholdActionPair = {
    limit: string
    action: ResponseActionType
  }

  type ResponseActionType = {
    type: 'default' | 'challenge' | 'monitor' | 'response' | 'redirect' | 'ban' | 'request_header'
    params?: {
      status?: string
      duration?: string
      headers?: string
      content?: string
      location?: string
      action?: ResponseActionType
    }
  }

  type ACLProfileFilter = 'allow' | 'allow_bot' | 'deny_bot' | 'passthrough' | 'force_deny' | 'deny'

  type IncludeExcludeType = 'include' | 'exclude'

  type Relation = 'OR' | 'AND'

  type Category = 'path' | 'query' | 'uri' | 'method' | 'ip' | 'asn' | 'country' | 'headers' | 'args' | 'cookies'

  type ArgsCookiesHeadersType = 'headers' | 'args' | 'cookies'

  type LimitRuleType = 'headers' | 'args' | 'cookies' | 'attrs' | 'self'

  type NamesRegexType = 'names' | 'regex'

  type Document = BasicDocument & (ACLProfile | FlowControlPolicy | GlobalFilter | RateLimit | SecurityPolicy | ContentFilterProfile | ContentFilterRule)

  type DocumentType = 'aclprofiles' | 'flowcontrol' | 'globalfilters' | 'ratelimits' | 'securitypolicies' | 'contentfilterprofiles' | 'contentfilterrules' | 'contentfiltergroups'

  // Document types helpers - END

  // Document types - START

  type BasicDocument = {
    id: string
    name: string
  }

  type ACLProfile = {
    id: string
    name: string
    allow: string[]
    allow_bot: string[]
    deny_bot: string[]
    passthrough: string[]
    force_deny: string[]
    deny: string[]
  }

  type ContentFilterProfile = {
    id: string
    name: string
    ignore_alphanum: boolean
    max_header_length: number
    max_cookie_length: number
    max_arg_length: number
    max_headers_count: number
    max_cookies_count: number
    max_args_count: number
    min_headers_risk: number,
    min_cookies_risk: number,
    min_args_risk: number,
    args: {
      names: ContentFilterEntryMatch[]
      regex: ContentFilterEntryMatch[]
    }
    headers: {
      names: ContentFilterEntryMatch[]
      regex: ContentFilterEntryMatch[]
    }
    cookies: {
      names: ContentFilterEntryMatch[]
      regex: ContentFilterEntryMatch[]
    }
  }

  type GlobalFilter = {
    id: string
    name: string
    source: string
    mdate: string // ISO string
    description: string
    active: boolean
    tags: string[]
    action: ResponseActionType
    rule: {
      relation: Relation
      sections: GlobalFilterSection[]
    }
  }

  type SecurityPolicy = {
    id: string
    name: string
    match: string
    map: SecurityPolicyEntryMatch[]
  }

  type RateLimit = {
    id: string
    name: string
    description: string
    thresholds: ThresholdActionPair[]
    key: LimitOptionType[]
    timeframe: string
    exclude: string[]
    include: string[]
    pairwith: LimitOptionType
  }

  type HttpRequestMethods = typeof httpRequestMethods[number]

  type FlowControlPolicy = {
    id: string
    name: string
    timeframe: number
    active: boolean
    description: string
    key: LimitOptionType[]
    action: ResponseActionType
    exclude: string[]
    include: string[]
    sequence: {
      args: GenericObject
      cookies: GenericObject
      headers: GenericObject
      method: HttpRequestMethods
      uri: string
    }[]
  }

  type ContentFilterRule = {
    id: string
    name: string
    operand: string
    notes: string
    risk: number
    msg: string
    category: string
    subcategory: string
  }

  type ContentFilterRuleGroup = {
    id: string
    name: string
    description?: string
    content_filter_rule_ids: ContentFilterRule['id'][]
  }

  // Document types - END

  // Git - START

  type Branch = {
    id: string
    description: string
    date: string // ISO string
    logs: Commit[]
    version: string
  }

  type Commit = {
    version: string
    date: string // ISO string
    parents: string[]
    message: string
    email: string
    author: string
  }

  // Git - END

}
export = CuriefenseClient
