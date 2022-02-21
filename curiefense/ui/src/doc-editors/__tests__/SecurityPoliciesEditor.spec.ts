import SecurityPoliciesEditor from '@/doc-editors/SecurityPoliciesEditor.vue'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {shallowMount, Wrapper} from '@vue/test-utils'
import {ACLProfile, ContentFilterProfile, RateLimit, SecurityPolicy} from '@/types'
import axios from 'axios'
import Vue from 'vue'
import _ from 'lodash'

jest.mock('axios')

describe('SecurityPoliciesEditor.vue', () => {
  let securityPoliciesDocs: SecurityPolicy[]
  let aclDocs: ACLProfile[]
  let contentFilterDocs: ContentFilterProfile[]
  let rateLimitsDocs: RateLimit[]
  let wrapper: Wrapper<Vue>
  let mockRouter
  let axiosGetSpy: any
  beforeEach(() => {
    securityPoliciesDocs = [
      {
        'id': '__default__',
        'name': 'default entry',
        'match': '__default__',
        'map': [
          {
            'name': 'default',
            'match': '/',
            'acl_profile': '__default__',
            'acl_active': false,
            'content_filter_profile': '__default__',
            'content_filter_active': false,
            'limit_ids': ['f971e92459e2'],
          },
          {
            'name': 'entry name',
            'match': '/login',
            'acl_profile': '5828321c37e0',
            'acl_active': false,
            'content_filter_profile': '009e846e819e',
            'content_filter_active': false,
            'limit_ids': ['365757ec0689'],
          },
        ],
      },
      {
        'id': '3086b9c5b518',
        'name': 'copy of default entry',
        'match': 'www.example.com',
        'map': [
          {
            'name': 'default',
            'match': '/',
            'acl_profile': '__default__',
            'acl_active': false,
            'content_filter_profile': '__default__',
            'content_filter_active': false,
            'limit_ids': ['f971e92459e2', '365757ec0689'],
          },
          {
            'name': 'entry name',
            'match': '/login',
            'acl_profile': '5828321c37e0',
            'acl_active': false,
            'content_filter_profile': '009e846e819e',
            'content_filter_active': false,
            'limit_ids': [],
          },
        ],
      },
    ]
    aclDocs = [
      {
        'id': '__default__',
        'name': 'default acl',
        'allow': [],
        'allow_bot': [
          'google',
        ],
        'deny_bot': [],
        'passthrough': [
          'internal',
        ],
        'deny': [
          'tor',
        ],
        'force_deny': [
          'china',
        ],
      },
      {
        'id': '5828321c37e0',
        'name': 'an ACL',
        'allow': [],
        'allow_bot': [
          'google',
          'yahoo',
        ],
        'deny_bot': [],
        'passthrough': [
          'devops',
        ],
        'deny': [
          'tor',
        ],
        'force_deny': [
          'iran',
        ],
      },
    ]
    contentFilterDocs = [
      {
        'id': '__default__',
        'name': 'default contentfilter',
        'ignore_alphanum': true,
        'max_header_length': 1024,
        'max_cookie_length': 2048,
        'max_arg_length': 1536,
        'max_headers_count': 36,
        'max_cookies_count': 42,
        'max_args_count': 512,
        'min_headers_risk': 1,
        'min_cookies_risk': 1,
        'min_args_risk': 1,
        'args': {'names': [], 'regex': []},
        'headers': {'names': [], 'regex': []},
        'cookies': {'names': [], 'regex': []},
      },
      {
        'id': '009e846e819e',
        'name': 'example content filter',
        'ignore_alphanum': true,
        'max_header_length': 1024,
        'max_cookie_length': 2048,
        'max_arg_length': 1536,
        'max_headers_count': 36,
        'max_cookies_count': 42,
        'max_args_count': 512,
        'min_headers_risk': 1,
        'min_cookies_risk': 1,
        'min_args_risk': 1,
        'args': {'names': [], 'regex': []},
        'headers': {'names': [], 'regex': []},
        'cookies': {'names': [], 'regex': []},
      },
    ]
    rateLimitsDocs = [
      {
        'id': 'f971e92459e2',
        'name': 'Rate Limit Example Rule 5/60',
        'description': '5 requests per minute',
        'timeframe': '60',
        'thresholds': [
          {
            'limit': '5',
            'action': {'type': 'default', 'params': {'action': {'type': 'default', 'params': {}}}},
          },
        ],
        'include': ['badpeople'],
        'exclude': ['goodpeople'],
        'key': [{'attrs': 'ip'}],
        'pairwith': {'self': 'self'},
      },
      {
        'id': '365757ec0689',
        'name': 'Copy of Rate Limit Example Rule 5/60',
        'description': '5 requests per minute',
        'timeframe': '60',
        'thresholds': [
          {
            'limit': '5',
            'action': {'type': 'default', 'params': {'action': {'type': 'default', 'params': {}}}},
          },
        ],
        'include': ['badpeople'],
        'exclude': ['goodpeople'],
        'key': [{'attrs': 'ip'}],
        'pairwith': {'self': 'self'},
      },
    ]
    axiosGetSpy = jest.spyOn(axios, 'get').mockImplementation((path, config) => {
      if (!wrapper) {
        return Promise.resolve({data: []})
      }
      const branch = (wrapper.vm as any).selectedBranch
      if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(aclDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: aclDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/securitypolicies/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(securityPoliciesDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: securityPoliciesDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/contentfilterprofiles/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(contentFilterDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: contentFilterDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/ratelimits/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(rateLimitsDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: rateLimitsDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/ratelimits/e/f971e92459e2/`) {
        return Promise.resolve({data: rateLimitsDocs[0]})
      }
      return Promise.resolve({data: []})
    })
    mockRouter = {
      push: jest.fn(),
    }
    wrapper = shallowMount(SecurityPoliciesEditor, {
      propsData: {
        selectedDoc: securityPoliciesDocs[0],
        selectedBranch: 'master',
      },
      mocks: {
        $router: mockRouter,
      },
    })
  })
  afterEach(() => {
    jest.clearAllMocks()
  })

  test('should not send new requests to API if document data updates but document ID does not', async () => {
    // 4 requests - ACL Profiles, Content Filter Profiles, Rate Limits, Security Policies
    expect(axiosGetSpy).toHaveBeenCalledTimes(4)
    securityPoliciesDocs[0] = {
      'id': '__default__',
      'name': 'new name',
      'match': 'example.com',
      'map': [
        {
          'name': 'one',
          'match': '/one',
          'acl_profile': '5828321c37e0',
          'acl_active': false,
          'content_filter_profile': '009e846e819e',
          'content_filter_active': true,
          'limit_ids': ['365757ec0689'],
        },
        {
          'name': 'two',
          'match': '/two',
          'acl_profile': '__default__',
          'acl_active': true,
          'content_filter_profile': '__default__',
          'content_filter_active': false,
          'limit_ids': ['f971e92459e2'],
        },
      ],
    }
    wrapper.setProps({
      selectedDoc: securityPoliciesDocs[0],
    })
    await Vue.nextTick()
    expect(axiosGetSpy).toHaveBeenCalledTimes(4)
  })

  test('should send a single new request to API if document data updates with new ID', async () => {
    // 4 requests - ACL Profiles, Content Filter Profiles, Rate Limits, Security Policies
    expect(axiosGetSpy).toHaveBeenCalledTimes(4)
    wrapper.setProps({
      selectedDoc: securityPoliciesDocs[1],
    })
    await Vue.nextTick()
    expect(axiosGetSpy).toHaveBeenCalledTimes(5)
  })

  describe('form data', () => {
    test('should have correct ID displayed', () => {
      expect(wrapper.find('.document-id').text()).toEqual(securityPoliciesDocs[0].id)
    })

    test('should have correct name in input', () => {
      const element = wrapper.find('.document-name').element as HTMLInputElement
      expect(element.value).toEqual(securityPoliciesDocs[0].name)
    })

    test('should have correct domain match in input', () => {
      const element = wrapper.find('.document-domain-name').element as HTMLInputElement
      expect(element.value).toEqual(securityPoliciesDocs[0].match)
    })

    test('should have correct amount of entry rows in table', () => {
      const table = wrapper.find('.entries-table')
      const entryRows = table.findAll('.entry-row')
      expect(entryRows.length).toEqual(securityPoliciesDocs[0].map.length)
    })

    test('should have correct entry data displayed in non-expanded rows (first row)', () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      const entryName = entryRow.find('.entry-name')
      expect(entryName.text()).toEqual(securityPoliciesDocs[0].map[0].name)
      const entryMatch = entryRow.find('.entry-match')
      expect(entryMatch.text()).toEqual(securityPoliciesDocs[0].map[0].match)
      const entryContentFilter = entryRow.find('.entry-content-filter')
      expect(entryContentFilter.text()).toEqual('default contentfilter')
      const entryACL = entryRow.find('.entry-acl')
      expect(entryACL.text()).toEqual('default acl')
      const entryRateLimitCount = entryRow.find('.entry-rate-limits-count')
      expect(entryRateLimitCount.text()).toEqual(String(securityPoliciesDocs[0].map[0].limit_ids.length))
    })

    test('should have correct entry data displayed in non-expanded rows (second row)', () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(1)
      const entryName = entryRow.find('.entry-name')
      expect(entryName.text()).toEqual(securityPoliciesDocs[0].map[1].name)
      const entryMatch = entryRow.find('.entry-match')
      expect(entryMatch.text()).toEqual(securityPoliciesDocs[0].map[1].match)
      const entryContentFilter = entryRow.find('.entry-content-filter')
      expect(entryContentFilter.text()).toEqual('example content filter')
      const entryACL = entryRow.find('.entry-acl')
      expect(entryACL.text()).toEqual('an ACL')
      const entryRateLimitCount = entryRow.find('.entry-rate-limits-count')
      expect(entryRateLimitCount.text()).toEqual(String(securityPoliciesDocs[0].map[1].limit_ids.length))
    })

    test('should have correct entry data displayed in expanded row', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryName = currentEntryRow.find('.current-entry-name')
      expect((entryName.element as HTMLInputElement).value).toEqual(securityPoliciesDocs[0].map[0].name)
      const entryMatch = currentEntryRow.find('.current-entry-match')
      expect((entryMatch.element as HTMLInputElement).value).toEqual(securityPoliciesDocs[0].map[0].match)
      const entryContentFilterSelection = currentEntryRow.find('.current-entry-content-filter-selection')
      expect((entryContentFilterSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
      const entryContentFilterActive = currentEntryRow.find('.current-entry-content-filter-active')
      expect((entryContentFilterActive.element as HTMLInputElement).checked)
        .toEqual(securityPoliciesDocs[0].map[0].content_filter_active)
      const entryACLSelection = currentEntryRow.find('.current-entry-acl-selection')
      expect((entryACLSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
      const entryACLActive = currentEntryRow.find('.current-entry-acl-active')
      expect((entryACLActive.element as HTMLInputElement).checked).toEqual(securityPoliciesDocs[0].map[0].acl_active)
    })

    test('should have correct entry rate limit data displayed in expanded row', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
      const entryRateLimitsRows = entryRateLimitsTable.findAll('.rate-limit-row')
      expect(entryRateLimitsRows.length).toEqual(securityPoliciesDocs[0].map[0].limit_ids.length)
      const rateLimitName = entryRateLimitsRows.at(0).find('.rate-limit-name')
      expect(rateLimitName.text()).toEqual(rateLimitsDocs[0].name)
      const rateLimitDescription = entryRateLimitsRows.at(0).find('.rate-limit-description')
      expect(rateLimitDescription.text()).toEqual(rateLimitsDocs[0].description)
      const rateLimitTTL = entryRateLimitsRows.at(0).find('.rate-limit-timeframe')
      expect(rateLimitTTL.text()).toEqual(rateLimitsDocs[0].timeframe)
    })

    test('should not have rate limit data displayed if no corresponding rate limit exists', async () => {
      securityPoliciesDocs[1].map[0].limit_ids.push('invalid')
      wrapper.setProps({
        selectedDoc: securityPoliciesDocs[1],
      })
      await Vue.nextTick()
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
      const entryRateLimitsRows = entryRateLimitsTable.findAll('.rate-limit-row')
      expect(entryRateLimitsRows.length).toEqual(securityPoliciesDocs[1].map[0].limit_ids.length - 1)
      const rateLimitName0 = entryRateLimitsRows.at(0).find('.rate-limit-name')
      expect(rateLimitName0.text()).toEqual(rateLimitsDocs[0].name)
      const rateLimitName1 = entryRateLimitsRows.at(1).find('.rate-limit-name')
      expect(rateLimitName1.text()).toEqual(rateLimitsDocs[1].name)
    })

    test('should open new rate limit row from add button', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
      const addButton = entryRateLimitsTable.find('.rate-limit-add-button')
      addButton.trigger('click')
      await Vue.nextTick()
      const enwRateLimitRow = entryRateLimitsTable.find('.new-rate-limit-row')
      expect(enwRateLimitRow.exists()).toBeTruthy()
    })

    test('should open new rate limit row from text `here` button if list is empty', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
      const removeButton = entryRateLimitsTable.find('.rate-limit-remove-button')
      removeButton.trigger('click')
      await wrapper.vm.$forceUpdate()
      const addButton = entryRateLimitsTable.find('.rate-limit-text-add-button')
      addButton.trigger('click')
      await Vue.nextTick()
      const enwRateLimitRow = entryRateLimitsTable.find('.new-rate-limit-row')
      expect(enwRateLimitRow.exists()).toBeTruthy()
    })

    test('should have all unselected rate limits in dropdown', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
      const addButton = entryRateLimitsTable.find('.rate-limit-add-button')
      addButton.trigger('click')
      await Vue.nextTick()
      const newRateLimitSelection = entryRateLimitsTable.find('.new-rate-limit-selection')
      const options = newRateLimitSelection.findAll('option')
      expect(options.length).toEqual(rateLimitsDocs.length - securityPoliciesDocs[0].map[0].limit_ids.length)
      expect(options.at(0).text()).toEqual(`${rateLimitsDocs[1].name} ${rateLimitsDocs[1].description}`)
    })

    test('should add selected rate limit from dropdown to table', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
      const addButton = entryRateLimitsTable.find('.rate-limit-add-button')
      addButton.trigger('click')
      await Vue.nextTick()
      const newRateLimitSelection = entryRateLimitsTable.find('.new-rate-limit-selection')
      const options = newRateLimitSelection.findAll('option')
      options.at(0).setSelected()
      await Vue.nextTick()
      const confirmAddButton = entryRateLimitsTable.find('.rate-limit-confirm-add-button')
      confirmAddButton.trigger('click')
      await Vue.nextTick()
      const entryRateLimitsRows = entryRateLimitsTable.findAll('.rate-limit-row')
      expect(entryRateLimitsRows.length).toEqual(securityPoliciesDocs[0].map[0].limit_ids.length + 1)
    })

    test('should not add a rate limit if nothing is selected in dropdown', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
      const addButton = entryRateLimitsTable.find('.rate-limit-add-button')
      addButton.trigger('click')
      await Vue.nextTick()
      const confirmAddButton = entryRateLimitsTable.find('.rate-limit-confirm-add-button')
      confirmAddButton.trigger('click')
      await Vue.nextTick()
      const entryRateLimitsRows = entryRateLimitsTable.findAll('.rate-limit-row')
      expect(entryRateLimitsRows.length).toEqual(securityPoliciesDocs[0].map[0].limit_ids.length)
    })

    test('should remove selected rate limit from table', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
      const removeButton = entryRateLimitsTable.find('.rate-limit-remove-button')
      removeButton.trigger('click')
      await wrapper.vm.$forceUpdate()
      const entryRateLimitsRows = entryRateLimitsTable.findAll('.rate-limit-row')
      expect(entryRateLimitsRows.length).toEqual(securityPoliciesDocs[0].map[0].limit_ids.length - 1)
    })

    test('should change route when create new rate limit is clicked', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
      const removeButton = entryRateLimitsTable.find('.rate-limit-remove-button')
      removeButton.trigger('click')
      await wrapper.vm.$forceUpdate()
      const referralButton = entryRateLimitsTable.find('.rate-limit-referral-button')
      await referralButton.trigger('click')
      await Vue.nextTick()
      expect(mockRouter.push).toHaveBeenCalledTimes(1)
      expect(mockRouter.push).toHaveBeenCalledWith('/config/master/ratelimits')
    })
  })

  describe('form validation', () => {
    beforeEach(async () => {
      wrapper.setProps({
        selectedDoc: securityPoliciesDocs[1],
      })
      await Vue.nextTick()
    })

    test('should emit form is invalid when changing match to already existing one', async () => {
      const input = wrapper.find('.document-domain-name')
      input.setValue(securityPoliciesDocs[0].match)
      input.trigger('input')
      await Vue.nextTick()
      expect(wrapper.emitted('form-invalid')).toBeTruthy()
      expect(wrapper.emitted('form-invalid')[0]).toEqual([true])
    })

    test('should emit form is invalid when filling match with illegal characters', async () => {
      const input = wrapper.find('.document-domain-name')
      input.setValue('БЮ')
      input.trigger('input')
      await Vue.nextTick()
      expect(wrapper.emitted('form-invalid')).toBeTruthy()
      expect(wrapper.emitted('form-invalid')[0]).toEqual([true])
    })

    test('should emit form is valid when changing match to valid one', async () => {
      const input = wrapper.find('.document-domain-name')
      input.setValue(securityPoliciesDocs[0].match)
      input.trigger('input')
      await Vue.nextTick()
      // reset all events for clearer event emitting
      wrapper.emitted('form-invalid').length = 0
      input.setValue('example.com')
      input.trigger('input')
      await Vue.nextTick()
      expect(wrapper.emitted('form-invalid')).toBeTruthy()
      expect(wrapper.emitted('form-invalid')[0]).toEqual([false])
    })

    test('should emit form is valid when changing match to valid one starting with special character', async () => {
      const input = wrapper.find('.document-domain-name')
      input.setValue('(api|service).company.(io|com)')
      input.trigger('input')
      await Vue.nextTick()
      expect(wrapper.emitted('form-invalid')).toBeTruthy()
      expect(wrapper.emitted('form-invalid')[0]).toEqual([false])
    })

    test('should emit form is invalid when changing map entry match to already existing one', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryMatch = currentEntryRow.find('.current-entry-match')
      entryMatch.setValue(securityPoliciesDocs[1].map[1].match)
      entryMatch.trigger('input')
      await Vue.nextTick()
      expect(wrapper.emitted('form-invalid')).toBeTruthy()
      expect(wrapper.emitted('form-invalid')[0]).toEqual([true])
    })

    test('should emit form is invalid when filling map entry match with unacceptable characters', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryMatch = currentEntryRow.find('.current-entry-match')
      entryMatch.setValue('/א')
      entryMatch.trigger('input')
      await Vue.nextTick()
      expect(wrapper.emitted('form-invalid')).toBeTruthy()
      expect(wrapper.emitted('form-invalid')[0]).toEqual([true])
    })

    test('should emit form is valid when changing map entry match to valid one', async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      const entryMatch = currentEntryRow.find('.current-entry-match')
      entryMatch.setValue('/logout')
      entryMatch.trigger('input')
      await Vue.nextTick()
      expect(wrapper.emitted('form-invalid')).toBeTruthy()
      expect(wrapper.emitted('form-invalid')[0]).toEqual([false])
    })

    test('should revert old entry match data to be valid before switching selected entry', async () => {
      let table = wrapper.find('.entries-table')
      let entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      let currentEntryRow = table.findAll('.current-entry-row').at(0)
      let entryMatch = currentEntryRow.find('.current-entry-match')
      entryMatch.setValue('')
      entryMatch.trigger('change')
      entryMatch.trigger('input')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      currentEntryRow = table.findAll('.current-entry-row').at(0)
      entryMatch = currentEntryRow.find('.current-entry-match')
      expect((entryMatch.element as HTMLInputElement).value).toEqual('/login')
    })

    test('should revert old entry match data to be valid before closing selected entry', async () => {
      let table = wrapper.find('.entries-table')
      let entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      let currentEntryRow = table.findAll('.current-entry-row').at(0)
      let entryMatch = currentEntryRow.find('.current-entry-match')
      entryMatch.setValue('')
      entryMatch.trigger('change')
      entryMatch.trigger('input')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      currentEntryRow = table.findAll('.current-entry-row').at(0)
      entryMatch = currentEntryRow.find('.current-entry-match')
      expect((entryMatch.element as HTMLInputElement).value).toEqual('/login')
    })

    test('should not revert entry match data if valid when switching selected entry', async () => {
      const wantedMatch = '/test'
      let table = wrapper.find('.entries-table')
      let entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      let currentEntryRow = table.findAll('.current-entry-row').at(0)
      let entryMatch = currentEntryRow.find('.current-entry-match')
      entryMatch.setValue(wantedMatch)
      entryMatch.trigger('change')
      entryMatch.trigger('input')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      entryRow = table.findAll('.entry-row').at(0)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      currentEntryRow = table.findAll('.current-entry-row').at(0)
      entryMatch = currentEntryRow.find('.current-entry-match')
      expect((entryMatch.element as HTMLInputElement).value).toEqual(wantedMatch)
    })

    test('should not revert entry match data if valid when closing selected entry', async () => {
      const wantedMatch = '/test'
      let table = wrapper.find('.entries-table')
      let entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      let currentEntryRow = table.findAll('.current-entry-row').at(0)
      let entryMatch = currentEntryRow.find('.current-entry-match')
      entryMatch.setValue(wantedMatch)
      entryMatch.trigger('change')
      entryMatch.trigger('input')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      table = wrapper.find('.entries-table')
      currentEntryRow = table.findAll('.current-entry-row').at(0)
      entryMatch = currentEntryRow.find('.current-entry-match')
      expect((entryMatch.element as HTMLInputElement).value).toEqual(wantedMatch)
    })

    describe('validation after add or remove map entries', () => {
      let currentEntryRow: Wrapper<Vue>
      let forkButton: Wrapper<Vue>
      let removeButton: Wrapper<Vue>
      beforeEach(async () => {
        const table = wrapper.find('.entries-table')
        const entryRow = table.findAll('.entry-row').at(1)
        entryRow.trigger('click')
        await Vue.nextTick()
        currentEntryRow = table.findAll('.current-entry-row').at(0)
        forkButton = currentEntryRow.find('.fork-entry-button')
        removeButton = currentEntryRow.find('.remove-entry-button')
      })

      describe('fork', () => {
        test('should emit form is valid when forking an invalid entry', async () => {
          const entryMatch = currentEntryRow.find('.current-entry-match')
          entryMatch.setValue('')
          entryMatch.trigger('input')
          await Vue.nextTick()
          // reset all events for clearer event emitting
          wrapper.emitted('form-invalid').length = 0
          forkButton.trigger('click')
          await Vue.nextTick()
          expect(wrapper.emitted('form-invalid')).toBeTruthy()
          expect(wrapper.emitted('form-invalid')[0]).toEqual([false])
        })

        test('should revert when forking an invalid entry', async () => {
          let entryMatch = currentEntryRow.find('.current-entry-match')
          entryMatch.setValue('')
          entryMatch.trigger('change')
          entryMatch.trigger('input')
          await Vue.nextTick()
          forkButton.trigger('click')
          await wrapper.vm.$forceUpdate()
          let table = wrapper.find('.entries-table')
          const entryRow = table.findAll('.entry-row').at(2)
          entryRow.trigger('click')
          await Vue.nextTick()
          await wrapper.vm.$forceUpdate()
          table = wrapper.find('.entries-table')
          currentEntryRow = table.findAll('.current-entry-row').at(0)
          entryMatch = currentEntryRow.find('.current-entry-match')
          expect((entryMatch.element as HTMLInputElement).value).toEqual('/login')
        })

        test('should not revert entry match data if valid when forking selected entry', async () => {
          const wantedMatch = '/test'
          let entryMatch = currentEntryRow.find('.current-entry-match')
          entryMatch.setValue(wantedMatch)
          entryMatch.trigger('change')
          entryMatch.trigger('input')
          await Vue.nextTick()
          forkButton.trigger('click')
          await wrapper.vm.$forceUpdate()
          let table = wrapper.find('.entries-table')
          const entryRow = table.findAll('.entry-row').at(2)
          entryRow.trigger('click')
          await Vue.nextTick()
          table = wrapper.find('.entries-table')
          currentEntryRow = table.findAll('.current-entry-row').at(0)
          entryMatch = currentEntryRow.find('.current-entry-match')
          expect((entryMatch.element as HTMLInputElement).value).toEqual(wantedMatch)
        })

        test('should not revert entry match data of new entry when forking selected entry', async () => {
          const validMatch = expect.stringContaining('/new/path/to/match/profile/')
          forkButton.trigger('click')
          await wrapper.vm.$forceUpdate()
          let table = wrapper.find('.entries-table')
          let entryRow = table.findAll('.entry-row').at(1)
          entryRow.trigger('click')
          await Vue.nextTick()
          table = wrapper.find('.entries-table')
          entryRow = table.findAll('.entry-row').at(1)
          entryRow.trigger('click')
          await Vue.nextTick()
          table = wrapper.find('.entries-table')
          currentEntryRow = table.findAll('.current-entry-row').at(0)
          const entryMatch = currentEntryRow.find('.current-entry-match')
          expect((entryMatch.element as HTMLInputElement).value).toEqual(validMatch)
        })

        test('should not revert entry match data of new entry when forking selected entry', async () => {
          const validMatch = expect.stringContaining('/new/path/to/match/profile/')
          forkButton.trigger('click')
          await wrapper.vm.$forceUpdate()
          let table = wrapper.find('.entries-table')
          let entryRow = table.findAll('.entry-row').at(2)
          entryRow.trigger('click')
          await Vue.nextTick()
          table = wrapper.find('.entries-table')
          entryRow = table.findAll('.entry-row').at(1)
          entryRow.trigger('click')
          await Vue.nextTick()
          table = wrapper.find('.entries-table')
          currentEntryRow = table.findAll('.current-entry-row').at(0)
          const entryMatch = currentEntryRow.find('.current-entry-match')
          expect((entryMatch.element as HTMLInputElement).value).toEqual(validMatch)
        })
      })

      describe('remove', () => {
        test('should emit form is valid when deleting an invalid entry', async () => {
          const entryMatch = currentEntryRow.find('.current-entry-match')
          entryMatch.setValue('')
          entryMatch.trigger('input')
          await Vue.nextTick()
          // reset all events for clearer event emitting
          wrapper.emitted('form-invalid').length = 0
          removeButton.trigger('click')
          await wrapper.vm.$forceUpdate()
          expect(wrapper.emitted('form-invalid')).toBeTruthy()
          expect(wrapper.emitted('form-invalid')[0]).toEqual([false])
        })

        test('should not revert entry match data of new selected entry when deleting selected entry', async () => {
          let table = wrapper.find('.entries-table')
          let entryRow = table.findAll('.entry-row').at(1)
          entryRow.trigger('click')
          await Vue.nextTick()
          removeButton.trigger('click')
          await wrapper.vm.$forceUpdate()
          table = wrapper.find('.entries-table')
          entryRow = table.findAll('.entry-row').at(0)
          entryRow.trigger('click')
          await Vue.nextTick()
          table = wrapper.find('.entries-table')
          currentEntryRow = table.findAll('.current-entry-row').at(0)
          const entryMatch = currentEntryRow.find('.current-entry-match')
          expect((entryMatch.element as HTMLInputElement).value).toEqual(securityPoliciesDocs[0].map[0].match)
        })
      })
    })
  })

  describe('add and remove map entries', () => {
    let forkButton: Wrapper<Vue>
    let removeButton: Wrapper<Vue>
    beforeEach(async () => {
      const table = wrapper.find('.entries-table')
      const entryRow = table.findAll('.entry-row').at(1)
      entryRow.trigger('click')
      await Vue.nextTick()
      const currentEntryRow = table.findAll('.current-entry-row').at(0)
      forkButton = currentEntryRow.find('.fork-entry-button')
      removeButton = currentEntryRow.find('.remove-entry-button')
    })

    describe('fork', () => {
      test('should add another map entry after forking an entry', async () => {
        forkButton.trigger('click')
        await Vue.nextTick()
        const table = wrapper.find('.entries-table')
        const entryRows = table.findAll('.entry-row')
        expect(entryRows.length).toEqual(securityPoliciesDocs[0].map.length + 1)
      })

      test('should have correct copied data after forking an entry', async () => {
        forkButton.trigger('click')
        await Vue.nextTick()
        const table = wrapper.find('.entries-table')
        const currentEntryRow = table.findAll('.current-entry-row').at(0)
        const entryName = currentEntryRow.find('.current-entry-name')
        expect((entryName.element as HTMLInputElement).value).toEqual('New Security Profile')
        const entryMatch = currentEntryRow.find('.current-entry-match')
        const validMatch = expect.stringContaining('/new/path/to/match/profile/')
        expect((entryMatch.element as HTMLInputElement).value).toEqual(validMatch)
        const entryContentFilterSelection = currentEntryRow.find('.current-entry-content-filter-selection')
        expect((entryContentFilterSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
        const entryContentFilterActive = currentEntryRow.find('.current-entry-content-filter-active')
        expect((entryContentFilterActive.element as HTMLInputElement).checked)
          .toEqual(securityPoliciesDocs[0].map[1].content_filter_active)
        const entryACLSelection = currentEntryRow.find('.current-entry-acl-selection')
        expect((entryACLSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        const entryACLActive = currentEntryRow.find('.current-entry-acl-active')
        expect((entryACLActive.element as HTMLInputElement).checked).toEqual(securityPoliciesDocs[0].map[1].acl_active)
      })

      test('should have correct copied rate limit data after forking an entry', async () => {
        forkButton.trigger('click')
        await Vue.nextTick()
        const table = wrapper.find('.entries-table')
        const currentEntryRow = table.findAll('.current-entry-row').at(0)
        const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
        const entryRateLimitsRows = entryRateLimitsTable.findAll('.rate-limit-row')
        expect(entryRateLimitsRows.length).toEqual(securityPoliciesDocs[0].map[1].limit_ids.length)
        const rateLimitName = entryRateLimitsRows.at(0).find('.rate-limit-name')
        expect(rateLimitName.text()).toEqual(rateLimitsDocs[1].name)
        const rateLimitDescription = entryRateLimitsRows.at(0).find('.rate-limit-description')
        expect(rateLimitDescription.text()).toEqual(rateLimitsDocs[1].description)
        const rateLimitTTL = entryRateLimitsRows.at(0).find('.rate-limit-timeframe')
        expect(rateLimitTTL.text()).toEqual(rateLimitsDocs[1].timeframe)
      })

      test('should revert old match data to be valid before forking if invalid', async () => {
        let table = wrapper.find('.entries-table')
        let currentEntryRow = table.findAll('.current-entry-row').at(0)
        let entryMatch = currentEntryRow.find('.current-entry-match')
        entryMatch.setValue('')
        entryMatch.trigger('change')
        entryMatch.trigger('input')
        await Vue.nextTick()
        forkButton.trigger('click')
        await Vue.nextTick()
        table = wrapper.find('.entries-table')
        const entryRow = table.findAll('.entry-row').at(2)
        entryRow.trigger('click')
        await Vue.nextTick()
        table = wrapper.find('.entries-table')
        currentEntryRow = table.findAll('.current-entry-row').at(0)
        entryMatch = currentEntryRow.find('.current-entry-match')
        expect((entryMatch.element as HTMLInputElement).value).toEqual('/login')
      })

      test('should have correct valid reverted match in forked entry', async () => {
        let table = wrapper.find('.entries-table')
        let currentEntryRow = table.findAll('.current-entry-row').at(0)
        let entryMatch = currentEntryRow.find('.current-entry-match')
        entryMatch.setValue('')
        entryMatch.trigger('change')
        entryMatch.trigger('input')
        await Vue.nextTick()
        forkButton.trigger('click')
        await Vue.nextTick()
        table = wrapper.find('.entries-table')
        currentEntryRow = table.findAll('.current-entry-row').at(0)
        const entryName = currentEntryRow.find('.current-entry-name')
        expect((entryName.element as HTMLInputElement).value).toEqual('New Security Profile')
        entryMatch = currentEntryRow.find('.current-entry-match')
        const validMatch = expect.stringContaining('/new/path/to/match/profile/')
        expect((entryMatch.element as HTMLInputElement).value).toEqual(validMatch)
        const entryContentFilterSelection = currentEntryRow.find('.current-entry-content-filter-selection')
        expect((entryContentFilterSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
        const entryContentFilterActive = currentEntryRow.find('.current-entry-content-filter-active')
        expect((entryContentFilterActive.element as HTMLInputElement).checked)
          .toEqual(securityPoliciesDocs[0].map[1].content_filter_active)
        const entryACLSelection = currentEntryRow.find('.current-entry-acl-selection')
        expect((entryACLSelection.element as HTMLSelectElement).selectedIndex).toEqual(0)
        const entryACLActive = currentEntryRow.find('.current-entry-acl-active')
        expect((entryACLActive.element as HTMLInputElement).checked).toEqual(securityPoliciesDocs[0].map[1].acl_active)
        const entryRateLimitsTable = currentEntryRow.find('.current-entry-rate-limits-table')
        const entryRateLimitsRows = entryRateLimitsTable.findAll('.rate-limit-row')
        expect(entryRateLimitsRows.length).toEqual(securityPoliciesDocs[0].map[1].limit_ids.length)
      })
    })

    describe('remove', () => {
      test('should remove map entry after clicking remove button', async () => {
        removeButton.trigger('click')
        await wrapper.vm.$forceUpdate()
        const table = wrapper.find('.entries-table')
        const entryRows = table.findAll('.entry-row')
        expect(entryRows.length).toEqual(securityPoliciesDocs[0].map.length - 1)
      })

      test('should close map entry after clicking remove button', async () => {
        forkButton.trigger('click')
        await wrapper.vm.$forceUpdate()
        removeButton.trigger('click')
        await wrapper.vm.$forceUpdate()
        const table = wrapper.find('.entries-table')
        const currentEntryRows = table.findAll('.current-entry-row')
        expect(currentEntryRows.length).toEqual(0)
      })
    })
  })

  test('should have forked entry name input focused', async (done) => {
    const elem = document.createElement('div')
    if (document.body) {
      document.body.appendChild(elem)
    }
    wrapper = shallowMount(SecurityPoliciesEditor, {
      propsData: {
        selectedDoc: securityPoliciesDocs[0],
        selectedBranch: 'master',
      },
      attachTo: elem,
    })
    let table = wrapper.find('.entries-table')
    const entryRow = table.findAll('.entry-row').at(1)
    entryRow.trigger('click')
    await Vue.nextTick()
    let currentEntryRow = table.findAll('.current-entry-row').at(0)
    const forkButton = currentEntryRow.find('.fork-entry-button')
    forkButton.trigger('click')
    await Vue.nextTick()
    table = wrapper.find('.entries-table')
    currentEntryRow = table.findAll('.current-entry-row').at(0)
    const entryName = currentEntryRow.find('.current-entry-name')
    // allow all requests to finish
    setImmediate(() => {
      expect(entryName.element).toBe(document.activeElement)
      done()
    })
  })
})
