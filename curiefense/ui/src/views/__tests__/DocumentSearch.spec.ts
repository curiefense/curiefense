import DocumentSearch from '@/views/DocumentSearch.vue'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {shallowMount, Wrapper} from '@vue/test-utils'
import axios from 'axios'
import Vue from 'vue'

jest.useFakeTimers()
jest.mock('axios')

describe('DocumentSearch.vue', () => {
  let wrapper: Wrapper<Vue>
  let mockRouter
  let gitData: any[]
  let aclDocs: any[]
  let profilingListDocs: any[]
  let urlMapsDocs: any[]
  let flowControlDocs: any[]
  let rateLimitDocs: any[]
  let wafDocs: any[]
  beforeEach((done) => {
    gitData = [
      {
        id: 'master',
      },
      {
        id: 'zzz_branch',
      },
    ]
    aclDocs = [
      {
        'id': '__default__',
        'name': 'default-acl',
        'allow': [],
        'allow_bot': [
          'google',
        ],
        'deny_bot': [],
        'bypass': [
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
        'bypass': [
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
    profilingListDocs = [
      {
        'id': 'xlbp148c',
        'name': 'API Discovery',
        'source': 'self-managed',
        'mdate': '2020-05-23T00:04:41',
        'notes': 'Default Tag API Requests',
        'active': true,
        'entries_relation': 'OR',
        'tags': [
          'api',
        ],
        'entries': [
          [
            'headers',
            [
              'content-type',
              '.*/(json|xml)',
            ],
            'content type',
          ],
          [
            'headers',
            [
              'host',
              '.?ap[ip]\\.',
            ],
            'app or api in domain name',
          ],
          [
            'method',
            '(POST|PUT|DELETE|PATCH)',
            'Methods',
          ],
          [
            'path',
            '/api/',
            'api path',
          ],
          [
            'uri',
            '/.+\\.json',
            'URI JSON extention',
          ],
        ],
      },
      {
        'id': '07656fbe',
        'name': 'devop internal demo',
        'source': 'self-managed',
        'mdate': '2020-05-23T00:04:41',
        'notes': 'this is my own list',
        'active': false,
        'entries_relation': 'OR',
        'tags': [
          'internal',
          'devops',
        ],
        'entries': [
          [
            'ip',
            '12.34.56.78/32',
            'testers',
          ],
          [
            'ip',
            '98.76.54.0/24',
            'monitoring',
          ],
          [
            'ip',
            '!5.4.3.2/32',
            'old monitoring',
          ],
          [
            'path',
            '/test/app/status',
            'monitoring path',
          ],
        ],
      },
    ]
    urlMapsDocs = [
      {
        'id': '__default__',
        'name': 'default entry',
        'match': '__default__',
        'map': [
          {
            'name': 'default',
            'match': '/',
            'acl_profile': '5828321c37e0',
            'acl_active': false,
            'waf_profile': '__default__',
            'waf_active': false,
            'limit_ids': ['f971e92459e2'],
          },
        ],
      },
    ]
    flowControlDocs = [
      {
        'exclude': [],
        'include': ['all'],
        'name': 'flow control',
        'key': [
          {'headers': 'something'},
        ],
        'sequence': [
          {
            'method': 'GET',
            'uri': '/login',
            'cookies': {
              'foo': 'bar',
            },
            'headers': {},
            'args': {},
          },
          {
            'method': 'POST',
            'uri': '/login',
            'cookies': {
              'foo': 'bar',
            },
            'headers': {
              'test': 'one',
            },
            'args': {},
          },
        ],
        'action': {
          'type': 'default',
          'params': {},
        },
        'ttl': 60,
        'id': 'c03dabe4b9ca',
      },
    ]
    rateLimitDocs = [
      {
        'id': 'f971e92459e2',
        'name': 'Rate Limit Example Rule 5/60',
        'description': '5 requests per minute',
        'ttl': '60',
        'limit': '5',
        'action': {'type': 'default'},
        'include': {'headers': {}, 'cookies': {}, 'args': {}, 'attrs': {}},
        'exclude': {'headers': {}, 'cookies': {}, 'args': {}, 'attrs': {}},
        'key': [{'attrs': 'ip'}],
        'pairwith': {'self': 'self'},
      },
    ]
    wafDocs = [
      {
        'id': '01b2abccc275',
        'name': 'default waf',
        'ignore_alphanum': true,
        'max_header_length': 1024,
        'max_cookie_length': 1024,
        'max_arg_length': 1024,
        'max_headers_count': 42,
        'max_cookies_count': 42,
        'max_args_count': 512,
        'args': {'names': [], 'regex': []},
        'headers': {'names': [], 'regex': []},
        'cookies': {'names': [], 'regex': []},
      },
    ]
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v1/configs/') {
        return Promise.resolve({data: gitData})
      }
      const branch = (wrapper.vm as any).selectedBranch
      if (path === `/conf/api/v1/configs/${branch}/d/aclpolicies/`) {
        return Promise.resolve({data: aclDocs})
      }
      if (path === `/conf/api/v1/configs/${branch}/d/tagrules/`) {
        return Promise.resolve({data: profilingListDocs})
      }
      if (path === `/conf/api/v1/configs/${branch}/d/urlmaps/`) {
        return Promise.resolve({data: urlMapsDocs})
      }
      if (path === `/conf/api/v1/configs/${branch}/d/flowcontrol/`) {
        return Promise.resolve({data: flowControlDocs})
      }
      if (path === `/conf/api/v1/configs/${branch}/d/ratelimits/`) {
        return Promise.resolve({data: rateLimitDocs})
      }
      if (path === `/conf/api/v1/configs/${branch}/d/wafpolicies/`) {
        return Promise.resolve({data: wafDocs})
      }
      return Promise.resolve({data: []})
    })
    mockRouter = {
      push: jest.fn(),
    }
    wrapper = shallowMount(DocumentSearch, {
      mocks: {
        $router: mockRouter,
      },
    })
    // allow all requests to finish
    setImmediate(() => {
      done()
    })
  })
  afterEach(() => {
    jest.clearAllMocks()
  })

  function isItemInFilteredDocs(item: any, doctype: string) {
    const isInModel = (wrapper.vm as any).filteredDocs.some((doc: any) => {
      return doc.id === item.id && doc.docType === doctype
    })
    const isInView = wrapper.findAll('.doc-id-cell').filter((w) => {
      return w.text().includes(item.id)
    }).length > 0
    return isInModel && isInView
  }

  function numberOfFilteredDocs() {
    return wrapper.findAll('.result-row').length
  }

  test('should be able to switch branches through dropdown', (done) => {
    const branchSelection = wrapper.find('.branch-selection')
    branchSelection.trigger('click')
    const options = branchSelection.findAll('option')
    options.at(1).setSelected()
    // allow all requests to finish
    setImmediate(() => {
      expect((branchSelection.element as any).selectedIndex).toEqual(1)
      done()
    })
  })

  test('should display all documents on startup', () => {
    expect(isItemInFilteredDocs(aclDocs[0], 'aclpolicies')).toBeTruthy()
    expect(isItemInFilteredDocs(aclDocs[1], 'aclpolicies')).toBeTruthy()
    expect(isItemInFilteredDocs(profilingListDocs[0], 'tagrules')).toBeTruthy()
    expect(isItemInFilteredDocs(profilingListDocs[1], 'tagrules')).toBeTruthy()
    expect(isItemInFilteredDocs(urlMapsDocs[0], 'urlmaps')).toBeTruthy()
    expect(isItemInFilteredDocs(flowControlDocs[0], 'flowcontrol')).toBeTruthy()
    expect(isItemInFilteredDocs(wafDocs[0], 'wafpolicies')).toBeTruthy()
    expect(isItemInFilteredDocs(rateLimitDocs[0], 'ratelimits')).toBeTruthy()
    expect(numberOfFilteredDocs()).toEqual(8)
  })

  test('should log message when receiving no configs from the server', (done) => {
    const originalLog = console.log
    let consoleOutput = [] as string[]
    const mockedLog = (output: string) => consoleOutput.push(output)
    consoleOutput = []
    console.log = mockedLog
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v1/configs/') {
        return Promise.reject(new Error())
      }
      return Promise.resolve({data: {}})
    })
    wrapper = shallowMount(DocumentSearch)
    // allow all requests to finish
    setImmediate(() => {
      expect(consoleOutput).toContain(`Error while attempting to get configs`)
      console.log = originalLog
      done()
    })
  })

  describe('filters', () => {
    test('should filter correctly if filter changed after input', async () => {
      // switch filter type
      let searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      let options = searchTypeSelection.findAll('option')
      options.at(1).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input');
      (searchInput.element as any).value = 'default'
      searchInput.trigger('input')
      await Vue.nextTick()

      // switch filter type
      searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      options = searchTypeSelection.findAll('option')
      options.at(0).setSelected()
      await Vue.nextTick()

      expect(isItemInFilteredDocs(aclDocs[0], 'aclpolicies')).toBeTruthy()
      expect(isItemInFilteredDocs(aclDocs[1], 'aclpolicies')).toBeTruthy()
      expect(isItemInFilteredDocs(profilingListDocs[0], 'tagrules')).toBeTruthy()
      expect(isItemInFilteredDocs(urlMapsDocs[0], 'urlmaps')).toBeTruthy()
      expect(isItemInFilteredDocs(wafDocs[0], 'wafpolicies')).toBeTruthy()
      expect(isItemInFilteredDocs(rateLimitDocs[0], 'ratelimits')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(6)
    })

    test('should filter correctly with filter all', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(0).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input');
      (searchInput.element as any).value = 'default'
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(aclDocs[0], 'aclpolicies')).toBeTruthy()
      expect(isItemInFilteredDocs(aclDocs[1], 'aclpolicies')).toBeTruthy()
      expect(isItemInFilteredDocs(profilingListDocs[0], 'tagrules')).toBeTruthy()
      expect(isItemInFilteredDocs(urlMapsDocs[0], 'urlmaps')).toBeTruthy()
      expect(isItemInFilteredDocs(wafDocs[0], 'wafpolicies')).toBeTruthy()
      expect(isItemInFilteredDocs(rateLimitDocs[0], 'ratelimits')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(6)
    })

    test('should filter correctly with filter document type', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(1).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input');
      (searchInput.element as any).value = 'flow'
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(flowControlDocs[0], 'flowcontrol')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(1)
    })

    test('should filter correctly with filter id', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(2).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input');
      (searchInput.element as any).value = 'c03dabe4b9ca'
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(flowControlDocs[0], 'flowcontrol')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(1)
    })

    test('should filter correctly with filter name', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(3).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input');
      (searchInput.element as any).value = 'default entry'
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(urlMapsDocs[0], 'urlmaps')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(1)
    })

    test('should filter correctly with filter description', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(4).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input');
      (searchInput.element as any).value = 'default'
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(profilingListDocs[0], 'tagrules')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(1)
    })

    test('should filter correctly with filter tags', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(5).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input');
      (searchInput.element as any).value = 'china'
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(aclDocs[0], 'aclpolicies')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(1)
    })

    test('should filter correctly with filter connections', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(6).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input');
      (searchInput.element as any).value = 'default'
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(aclDocs[1], 'aclpolicies')).toBeTruthy()
      expect(isItemInFilteredDocs(urlMapsDocs[0], 'urlmaps')).toBeTruthy()
      expect(isItemInFilteredDocs(rateLimitDocs[0], 'ratelimits')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(3)
    })
  })

  describe('go to link', () => {
    test('should not render go to link button when not hovering over a row', () => {
      expect(wrapper.findAll('.go-to-link-button').length).toEqual(0)
    })

    test('should render a single go to link button when hovering over a row', async () => {
      // 0 is the table header, 1 is our first data
      const firstDataRow = wrapper.findAll('tr').at(1)
      await firstDataRow.trigger('mouseover')
      expect(firstDataRow.findAll('.go-to-link-button').length).toEqual(1)
    })

    test('should stop rendering the go to link button when no longer hovering over a row', async () => {
      // 0 is the table header, 1 is our first data
      const firstDataRow = wrapper.findAll('tr').at(1)
      await firstDataRow.trigger('mouseover')
      await firstDataRow.trigger('mouseleave')
      expect(firstDataRow.findAll('.go-to-link-button').length).toEqual(0)
    })

    test('should change route when go to link button is clicked', async () => {
      // 0 is the table header, 1 is our first data
      const firstDataRow = wrapper.findAll('tr').at(1)
      await firstDataRow.trigger('mouseover')
      const goToLinkButton = firstDataRow.find('.go-to-link-button')
      await goToLinkButton.trigger('click')
      expect(mockRouter.push).toHaveBeenCalledTimes(1)
      expect(mockRouter.push).toHaveBeenCalledWith('/config/master/urlmaps/__default__')
    })
  })
})
