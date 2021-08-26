/* eslint-disable */

declare module CuriefenseClient {

  type GenericObject = { [key: string]: any }

  type TagsNamespaceValue = {
    neutral?: string[]
    malicious?: string[]
    legitimate?: string[]
  }

  // Document types helpers - START

  type WAFEntryMatch = {
    key: string
    reg: string
    restrict: boolean
    mask: boolean
    type: NamesRegexType
    exclusions: { [key: string]: number }
  }

  type URLMapEntryMatch = {
    match: string
    name: string
    acl_profile: string
    waf_profile: string
    acl_active: boolean
    waf_active: boolean
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

  type ResponseActionType = {
    type: 'default' | 'challenge' | 'monitor' | 'response' | 'redirect' | 'ban' | 'request_header'
    params?: {
      status?: string
      ttl?: string
      headers?: string
      content?: string
      location?: string
      action?: ResponseActionType
    }
  }

  type ACLProfileFilter = 'allow' | 'allow_bot' | 'deny_bot' | 'bypass' | 'force_deny' | 'deny'

  type IncludeExcludeType = 'include' | 'exclude'

  type Relation = 'OR' | 'AND'

  type Category = 'path' | 'query' | 'uri' | 'method' | 'ip' | 'asn' | 'country' | 'headers' | 'args' | 'cookies'

  type ArgsCookiesHeadersType = 'headers' | 'args' | 'cookies'

  type LimitRuleType = 'headers' | 'args' | 'cookies' | 'attrs' | 'self'

  type NamesRegexType = 'names' | 'regex'

  type Document = BasicDocument & (ACLProfile | FlowControlPolicy | GlobalFilter | RateLimit | URLMap | WAFPolicy | WAFRule)

  type DocumentType = 'aclprofiles' | 'flowcontrol' | 'globalfilters' | 'ratelimits' | 'urlmaps' | 'wafpolicies' | 'wafrules'

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
    bypass: string[]
    force_deny: string[]
    deny: string[]
  }

  type WAFPolicy = {
    id: string
    name: string
    ignore_alphanum: boolean
    max_header_length: number
    max_cookie_length: number
    max_arg_length: number
    max_headers_count: number
    max_cookies_count: number
    max_args_count: number
    args: {
      names: WAFEntryMatch[]
      regex: WAFEntryMatch[]
    }
    headers: {
      names: WAFEntryMatch[]
      regex: WAFEntryMatch[]
    }
    cookies: {
      names: WAFEntryMatch[]
      regex: WAFEntryMatch[]
    }
  }

  type GlobalFilter = {
    id: string
    name: string
    source: string
    mdate: string // ISO string
    notes: string
    active: boolean
    tags: string[]
    action: ResponseActionType
    rule: {
      relation: Relation
      sections: GlobalFilterSection[]
    }
  }

  type URLMap = {
    id: string
    name: string
    match: string
    map: URLMapEntryMatch[]
  }

  type RateLimit = {
    id: string
    name: string
    description: string
    limit: string
    key: LimitOptionType[]
    ttl: string
    action: ResponseActionType
    exclude: string[]
    include: string[]
    pairwith: LimitOptionType
  }

  type FlowControlPolicy = {
    id: string
    name: string
    ttl: number
    active: boolean
    notes: string
    key: LimitOptionType[]
    action: ResponseActionType
    exclude: string[]
    include: string[]
    sequence: {
      args: GenericObject
      cookies: GenericObject
      headers: GenericObject
      method: string
      uri: string
    }[]
  }

  type WAFRule = {
    id: string
    name: string
    category?: string
    certainity?: number
    msg?: string
    operand: string
    severity?: number
    subcategory?: string
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