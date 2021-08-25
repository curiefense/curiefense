import DatasetsUtils from '../../assets/DatasetsUtils'
import {describe, expect, test} from '@jest/globals'

describe('RequestsUtils.ts', () => {
  const regexUUID = /[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}/
  const regexUUID2 = /[0-9a-fA-F]{12}/

  describe('generateUUID function', () => {
    test('should generate random UUID', async () => {
      const actualUUID = DatasetsUtils.generateUUID()
      expect(regexUUID.test(actualUUID)).toBeTruthy()
    })
  })

  describe('generateUUID2 function', () => {
    test('should generate random UUID and return last 12 digits of it', async () => {
      const actualUUID = DatasetsUtils.generateUUID2()
      expect(regexUUID2.test(actualUUID)).toBeTruthy()
    })
  })

  describe('newDocEntryFactory', () => {
    test('should generate a new ACL Policy', async () => {
      const document = DatasetsUtils.newDocEntryFactory.aclpolicies()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New ACL Policy')
      expect(document['allow']).toEqual([])
      expect(document['allow_bot']).toEqual([])
      expect(document['deny_bot']).toEqual([])
      expect(document['bypass']).toEqual([])
      expect(document['force_deny']).toEqual([])
      expect(document['deny']).toEqual([])
    })

    test('should generate a new WAF Policy', async () => {
      const document = DatasetsUtils.newDocEntryFactory.wafpolicies()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New WAF Policy')
      expect(document['ignore_alphanum']).toEqual(true)
      expect(document['max_header_length']).toEqual(1024)
      expect(document['max_cookie_length']).toEqual(1024)
      expect(document['max_arg_length']).toEqual(1024)
      expect(document['max_headers_count']).toEqual(42)
      expect(document['max_cookies_count']).toEqual(42)
      expect(document['max_args_count']).toEqual(512)
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
      const document = DatasetsUtils.newDocEntryFactory.globalfilters()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New Global Filter')
      expect(document['source']).toEqual('self-managed')
      expect(typeof document['mdate'] === 'string').toBeTruthy()
      expect(regexISODate.test(document['mdate'])).toBeTruthy()
      expect(document['notes']).toEqual('New List Notes and Remarks')
      expect(document['active']).toEqual(true)
      expect(document['tags']).toEqual([])
      expect(document['action']).toEqual({'type': 'monitor'})
      expect(document['rule']['relation']).toEqual('OR')
      expect(document['rule']['sections']).toEqual([])
    })

    test('should generate a new URL Map', async () => {
      const document = DatasetsUtils.newDocEntryFactory.urlmaps()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New URL Map')
      expect(document['match']).toEqual(`${document['id']}.example.com`)
      expect(document['map'][0]['match']).toEqual('/')
      expect(document['map'][0]['name']).toEqual('default')
      expect(document['map'][0]['acl_profile']).toEqual('__default__')
      expect(document['map'][0]['waf_profile']).toEqual('__default__')
      expect(document['map'][0]['acl_active']).toEqual(true)
      expect(document['map'][0]['waf_active']).toEqual(true)
      expect(document['map'][0]['limit_ids']).toEqual([])
    })

    test('should generate a new Rate Limit', async () => {
      const document = DatasetsUtils.newDocEntryFactory.ratelimits()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New Rate Limit Rule')
      expect(document['description']).toEqual('New Rate Limit Rule')
      expect(document['limit']).toEqual('3')
      expect(document['ttl']).toEqual('180')
      expect(document['key']).toEqual([{'attrs': 'ip'}])
      expect(document['action']).toEqual({'type': 'default'})
      expect(document['pairwith']).toEqual({'self': 'self'})
      expect(document['exclude']).toEqual(['allowlist'])
      expect(document['include']).toEqual(['blocklist'])
    })

    test('should generate a new Rate Limit', async () => {
      const document = DatasetsUtils.newDocEntryFactory.flowcontrol()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New Flow Control Policy')
      expect(document['ttl']).toEqual(60)
      expect(document['active']).toEqual(true)
      expect(document['notes']).toEqual('New Flow Control Policy Notes and Remarks')
      expect(document['key']).toEqual([{'attrs': 'ip'}])
      expect(document['action']).toEqual({'type': 'default'})
      expect(document['exclude']).toEqual([])
      expect(document['include']).toEqual(['all'])
      expect(document['sequence']).toEqual([])
    })

    test('should generate a new WAF Rule', async () => {
      const document = DatasetsUtils.newDocEntryFactory.wafrules()
      expect(regexUUID2.test(document['id'])).toBeTruthy()
      expect(document['name']).toEqual('New WAF Rule')
      expect(document['operand']).toEqual('')
    })
  })
})
