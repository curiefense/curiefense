import DatasetsUtils from '@/assets/DatasetsUtils'
import {describe, expect, test} from '@jest/globals'

describe('RequestsUtils.ts', () => {
  const regexUUID = /[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}/
  const regexUUID2 = /[0-9a-fA-F]{12}/
  const {generateUUID, generateUUID2, newDocEntryFactory} = DatasetsUtils

  describe('generateUUID function', () => {
    test('should generate random UUID', async () => {
      const actualUUID = generateUUID()
      expect(regexUUID.test(actualUUID)).toBeTruthy()
    })
  })

  describe('generateUUID2 function', () => {
    test('should generate random UUID and return last 12 digits of it', async () => {
      const actualUUID = generateUUID2()
      expect(regexUUID2.test(actualUUID)).toBeTruthy()
    })
  })

  describe('newDocEntryFactory', () => {
    test('should generate a new ACL Profile', async () => {
      const document = newDocEntryFactory.aclprofiles()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New ACL Profile')
      expect(document['allow']).toEqual([])
      expect(document['allow_bot']).toEqual([])
      expect(document['deny_bot']).toEqual([])
      expect(document['passthrough']).toEqual([])
      expect(document['force_deny']).toEqual([])
      expect(document['deny']).toEqual([])
    })

    test('should generate a new Content Filter Profile', async () => {
      const document = newDocEntryFactory.contentfilterprofiles()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New Content Filter Profile')
      expect(document['ignore_alphanum']).toEqual(true)
      expect(document['max_header_length']).toEqual(1024)
      expect(document['max_cookie_length']).toEqual(1024)
      expect(document['max_arg_length']).toEqual(1024)
      expect(document['max_headers_count']).toEqual(42)
      expect(document['max_cookies_count']).toEqual(42)
      expect(document['max_args_count']).toEqual(512)
      expect(document['min_headers_risk']).toEqual(1)
      expect(document['min_cookies_risk']).toEqual(1)
      expect(document['min_args_risk']).toEqual(1)
      expect(document['args']['names']).toEqual([])
      expect(document['args']['regex']).toEqual([])
      expect(document['headers']['names']).toEqual([])
      expect(document['headers']['regex']).toEqual([])
      expect(document['cookies']['names']).toEqual([])
      expect(document['cookies']['regex']).toEqual([])
    })

    test('should generate a new Global Filter', async () => {
      // eslint-disable-next-line max-len
      const regexISODate = /(\d{4}-[01]\d-[0-3]\dT[0-2]\d:[0-5]\d:[0-5]\d\.\d+)|(\d{4}-[01]\d-[0-3]\dT[0-2]\d:[0-5]\d:[0-5]\d)|(\d{4}-[01]\d-[0-3]\dT[0-2]\d:[0-5]\d)/
      const document = newDocEntryFactory.globalfilters()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New Global Filter')
      expect(document['source']).toEqual('self-managed')
      expect(typeof document['mdate'] === 'string').toBeTruthy()
      expect(regexISODate.test(document['mdate'])).toBeTruthy()
      expect(document['description']).toEqual('New List Description and Remarks')
      expect(document['active']).toEqual(true)
      expect(document['tags']).toEqual([])
      expect(document['action']).toEqual({'type': 'monitor'})
      expect(document['rule']['relation']).toEqual('OR')
      expect(document['rule']['sections']).toEqual([])
    })

    test('should generate a new Security Policy', async () => {
      const document = newDocEntryFactory.securitypolicies()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New Security Policy')
      expect(document['match']).toEqual(`${document['id']}.example.com`)
      expect(document['map'][0]['match']).toEqual('/')
      expect(document['map'][0]['name']).toEqual('default')
      expect(document['map'][0]['acl_profile']).toEqual('__default__')
      expect(document['map'][0]['content_filter_profile']).toEqual('__default__')
      expect(document['map'][0]['acl_active']).toEqual(true)
      expect(document['map'][0]['content_filter_active']).toEqual(true)
      expect(document['map'][0]['limit_ids']).toEqual([])
    })

    test('should generate a new Rate Limit', async () => {
      const document = newDocEntryFactory.ratelimits()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New Rate Limit Rule')
      expect(document['description']).toEqual('New Rate Limit Rule')
      expect(document['thresholds'][0]['limit']).toEqual('3')
      expect(document['timeframe']).toEqual('180')
      expect(document['key']).toEqual([{'attrs': 'ip'}])
      expect(document['thresholds'][0]['action']).toEqual({'type': 'default'})
      expect(document['pairwith']).toEqual({'self': 'self'})
      expect(document['exclude']).toEqual(['allowlist'])
      expect(document['include']).toEqual(['blocklist'])
    })

    test('should generate a new Flow Control', async () => {
      const {defaultFlowControlSequenceItem} = DatasetsUtils
      const document = newDocEntryFactory.flowcontrol()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New Flow Control Policy')
      expect(document['timeframe']).toEqual(60)
      expect(document['active']).toEqual(true)
      expect(document['description']).toEqual('New Flow Control Policy Description and Remarks')
      expect(document['key']).toEqual([{'attrs': 'ip'}])
      expect(document['action']).toEqual({'type': 'default'})
      expect(document['exclude']).toEqual([])
      expect(document['include']).toEqual(['all'])
      expect(document['sequence']).toEqual([
        {...defaultFlowControlSequenceItem},
        {
          ...defaultFlowControlSequenceItem,
          method: 'POST',
        },
      ])
    })

    test('should generate a new Content Filter Rule', async () => {
      const document = newDocEntryFactory.contentfilterrules()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New Content Filter Rule')
      expect(document['risk']).toEqual(1)
      expect(document['msg']).toEqual('')
      expect(document['notes']).toEqual('')
      expect(document['operand']).toEqual('')
      expect(document['category']).toEqual('')
      expect(document['subcategory']).toEqual('')
    })

    test('should generate a new Content Filter Group', async () => {
      const {id, name, description, content_filter_rule_ids: contentFilterIds} = newDocEntryFactory.contentfiltergroups()
      expect(regexUUID2.test(id)).toBeTruthy()
      expect(name).toEqual('New Content Filter Rule Group')
      expect(description).toEqual('')
      expect(contentFilterIds).toEqual([])
    })
  })
})
