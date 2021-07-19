import DocumentSearch from '@/views/DocumentSearch.vue'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {shallowMount, Wrapper} from '@vue/test-utils'
import axios from 'axios'
import Vue from 'vue'
import {ACLProfile, BasicDocument, Branch, FlowControlPolicy, RateLimit, GlobalFilter, SecurityPolicy, ContentFilterProfile} from '@/types'

jest.useFakeTimers()
jest.mock('axios')

describe('DocumentSearch.vue', () => {
  let wrapper: Wrapper<Vue>
  let mockRouter
  let gitData: Branch[]
  let aclDocs: ACLProfile[]
  let profilingListDocs: GlobalFilter[]
  let securityPoliciesDocs: SecurityPolicy[]
  let flowControlPolicyDocs: FlowControlPolicy[]
  let rateLimitDocs: RateLimit[]
  let contentFilterDocs: ContentFilterProfile[]
  beforeEach((done) => {
    gitData = [
      {
        'id': 'master',
        'description': 'Update entry [__default__] of document [aclprofiles]',
        'date': '2020-11-10T15:49:17+02:00',
        'logs': [
          {
            'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
            'date': '2020-11-10T15:49:17+02:00',
            'parents': [
              'fc47a6cd9d7f254dd97875a04b87165cc484e075',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': 'fc47a6cd9d7f254dd97875a04b87165cc484e075',
            'date': '2020-11-10T15:48:35+02:00',
            'parents': [
              '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
            'date': '2020-11-10T15:48:31+02:00',
            'parents': [
              '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
            'date': '2020-11-10T15:48:22+02:00',
            'parents': [
              '878b47deeddac94625fe7c759786f2df885ec541',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '878b47deeddac94625fe7c759786f2df885ec541',
            'date': '2020-11-10T15:48:05+02:00',
            'parents': [
              '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
            'date': '2020-11-10T15:47:59+02:00',
            'parents': [
              '1662043d2a18d6ad2c9c94d6f826593ff5506354',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
            'date': '2020-11-08T21:31:41+01:00',
            'parents': [
              '16379cdf39501574b4a2f5a227b82a4454884b84',
            ],
            'message': 'Create config [master]\n',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '16379cdf39501574b4a2f5a227b82a4454884b84',
            'date': '2020-08-27T16:19:06+00:00',
            'parents': [
              'a34f979217215060861b58b3f270e82580c20efb',
            ],
            'message': 'Initial empty config',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': 'a34f979217215060861b58b3f270e82580c20efb',
            'date': '2020-08-27T16:19:06+00:00',
            'parents': [],
            'message': 'Initial empty content',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
        ],
        'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
      },
      {
        'id': 'zzz_branch',
        'description': 'Initial empty content',
        'date': '2020-08-27T16:19:06+00:00',
        'logs': [
          {
            'version': 'a34f979217215060861b58b3f270e82580c20efb',
            'date': '2020-08-27T16:19:06+00:00',
            'parents': [],
            'message': 'Initial empty content',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
        ],
        'version': 'a34f979217215060861b58b3f270e82580c20efb',
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
    profilingListDocs = [
      {
        'id': 'xlbp148c',
        'name': 'API Discovery',
        'source': 'self-managed',
        'mdate': '2020-05-23T00:04:41',
        'description': 'Default Tag API Requests',
        'active': true,
        'tags': ['api'],
        'action': {
          'type': 'monitor',
          'params': {},
        },
        'rule': {
          'relation': 'OR',
          'sections': [
            {
              'relation': 'OR', 'entries': [
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
          ],
        },
      }, {
        'id': '07656fbe',
        'name': 'devop internal demo',
        'source': 'self-managed',
        'mdate': '2020-05-23T00:04:41',
        'description': 'this is my own list',
        'active': false,
        'tags': ['internal', 'devops'],
        'action': {
          'type': 'monitor',
          'params': {},
        },
        'rule': {
          'relation': 'OR',
          'sections': [
            {'relation': 'OR', 'entries': [['ip', '1.1.1.1', null]]},
            {'relation': 'OR', 'entries': [['ip', '2.2.2.2', null]]},
            {'relation': 'OR', 'entries': [['headers', ['headerrr', 'valueeee'], 'anooo']]}],
        },
      },
    ]
    securityPoliciesDocs = [
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
            'content_filter_profile': '__default__',
            'content_filter_active': false,
            'limit_ids': ['f971e92459e2'],
          },
          {
            'name': 'name',
            'match': '/foo',
            'acl_profile': '__default__',
            'acl_active': false,
            'content_filter_profile': '__default__',
            'content_filter_active': false,
            'limit_ids': ['f971e92459e2'],
          },
          {
            'name': 'name',
            'match': '/foo',
            'acl_profile': '__default__',
            'acl_active': false,
            'content_filter_profile': '__default__',
            'content_filter_active': false,
            'limit_ids': ['f971e92459e2'],
          },
        ],
      },
    ]
    flowControlPolicyDocs = [
      {
        'active': true,
        'description': '',
        'exclude': [],
        'include': ['all'],
        'name': 'flow control policy',
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
        'timeframe': 60,
        'id': 'c03dabe4b9ca',
      },
    ]
    rateLimitDocs = [
      {
        'id': 'f971e92459e2',
        'name': 'Rate Limit Example Rule 5/60',
        'description': '5 requests per minute',
        'timeframe': '60',
        'thresholds': [
          {
            'limit': '5',
            'action': {'type': 'default'},
          },
        ],
        'include': ['badpeople'],
        'exclude': ['goodpeople'],
        'key': [{'attrs': 'ip'}],
        'pairwith': {'self': 'self'},
      },
    ]
    contentFilterDocs = [
      {
        'id': '01b2abccc275',
        'name': 'default contentfilter',
        'ignore_alphanum': true,
        'max_header_length': 1024,
        'max_cookie_length': 1024,
        'max_arg_length': 1024,
        'max_headers_count': 42,
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
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/configs/') {
        return Promise.resolve({data: gitData})
      }
      const branch = (wrapper.vm as any).selectedBranch
      if (path === `/conf/api/v2/configs/${branch}/d/aclprofiles/`) {
        return Promise.resolve({data: aclDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/globalfilters/`) {
        return Promise.resolve({data: profilingListDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/securitypolicies/`) {
        return Promise.resolve({data: securityPoliciesDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/flowcontrol/`) {
        return Promise.resolve({data: flowControlPolicyDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/ratelimits/`) {
        return Promise.resolve({data: rateLimitDocs})
      }
      if (path === `/conf/api/v2/configs/${branch}/d/contentfilterprofiles/`) {
        return Promise.resolve({data: contentFilterDocs})
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

  function isItemInFilteredDocs(item: BasicDocument, doctype: string) {
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
      expect((branchSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
      done()
    })
  })

  test('should display all documents on startup', () => {
    expect(isItemInFilteredDocs(aclDocs[0], 'aclprofiles')).toBeTruthy()
    expect(isItemInFilteredDocs(aclDocs[1], 'aclprofiles')).toBeTruthy()
    expect(isItemInFilteredDocs(profilingListDocs[0], 'globalfilters')).toBeTruthy()
    expect(isItemInFilteredDocs(profilingListDocs[1], 'globalfilters')).toBeTruthy()
    expect(isItemInFilteredDocs(securityPoliciesDocs[0], 'securitypolicies')).toBeTruthy()
    expect(isItemInFilteredDocs(flowControlPolicyDocs[0], 'flowcontrol')).toBeTruthy()
    expect(isItemInFilteredDocs(contentFilterDocs[0], 'contentfilterprofiles')).toBeTruthy()
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
      if (path === '/conf/api/v2/configs/') {
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

  test('should not display duplicated values in connections even if connected twice', async () => {
    const wantedIDsACL = ['5828321c37e0', '__default__']
    const wantedIDsContentFilter = ['__default__']
    const wantedIDsRateLimit = ['f971e92459e2']
    // switch filter type to security policy
    const searchTypeSelection = wrapper.find('.search-type-selection')
    searchTypeSelection.trigger('click')
    const options = searchTypeSelection.findAll('option')
    options.at(1).setSelected()
    await Vue.nextTick()
    const searchInput = wrapper.find('.search-input')
    // Using a partial name instead of 'security policy'
    // The search is executing a RegEx which means 'securitypolicies' would not
    // match 'security policies'
    searchInput.setValue('security pol')
    searchInput.trigger('input')
    await Vue.nextTick()

    // Get connections cell
    const connectionsCell = wrapper.find('.doc-connections-cell')
    const doc = (wrapper.vm as any).filteredDocs[0]

    // check that security policy exists without duplicated connections
    expect(isItemInFilteredDocs(securityPoliciesDocs[0], 'securitypolicies')).toBeTruthy()
    expect(connectionsCell.text()).toContain(`ACL Profiles:${wantedIDsACL.join('')}`)
    expect(connectionsCell.text()).toContain(`Content Filter Profiles:${wantedIDsContentFilter.join('')}`)
    expect(connectionsCell.text()).toContain(`Rate Limits:${wantedIDsRateLimit.join('')}`)
    expect(doc.connectedACL).toEqual(wantedIDsACL)
    expect(doc.connectedContentFilter).toEqual(wantedIDsContentFilter)
    expect(doc.connectedRateLimits).toEqual(wantedIDsRateLimit)
  })

  describe('filters', () => {
    test('should filter correctly if filter changed after input', async () => {
      // switch filter type
      let searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      let options = searchTypeSelection.findAll('option')
      options.at(1).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input')
      searchInput.setValue('default')
      searchInput.trigger('input')
      await Vue.nextTick()

      // switch filter type
      searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      options = searchTypeSelection.findAll('option')
      options.at(0).setSelected()
      await Vue.nextTick()

      expect(isItemInFilteredDocs(aclDocs[0], 'aclprofiles')).toBeTruthy()
      expect(isItemInFilteredDocs(aclDocs[1], 'aclprofiles')).toBeTruthy()
      expect(isItemInFilteredDocs(profilingListDocs[0], 'globalfilters')).toBeTruthy()
      expect(isItemInFilteredDocs(securityPoliciesDocs[0], 'securitypolicies')).toBeTruthy()
      expect(isItemInFilteredDocs(contentFilterDocs[0], 'contentfilterprofiles')).toBeTruthy()
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

      const searchInput = wrapper.find('.search-input')
      searchInput.setValue('default')
      searchInput.trigger('input')
      await Vue.nextTick()

      expect(isItemInFilteredDocs(aclDocs[0], 'aclprofiles')).toBeTruthy()
      expect(isItemInFilteredDocs(aclDocs[1], 'aclprofiles')).toBeTruthy()
      expect(isItemInFilteredDocs(profilingListDocs[0], 'globalfilters')).toBeTruthy()
      expect(isItemInFilteredDocs(securityPoliciesDocs[0], 'securitypolicies')).toBeTruthy()
      expect(isItemInFilteredDocs(contentFilterDocs[0], 'contentfilterprofiles')).toBeTruthy()
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

      const searchInput = wrapper.find('.search-input')
      searchInput.setValue('flow')
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(flowControlPolicyDocs[0], 'flowcontrol')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(1)
    })

    test('should filter correctly with filter id', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(2).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input')
      searchInput.setValue('c03dabe4b9ca')
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(flowControlPolicyDocs[0], 'flowcontrol')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(1)
    })

    test('should filter correctly with filter name', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(3).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input')
      searchInput.setValue('default entry')
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(securityPoliciesDocs[0], 'securitypolicies')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(1)
    })

    test('should filter correctly with filter description', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(4).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input')
      searchInput.setValue('default')
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(profilingListDocs[0], 'globalfilters')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(1)
    })

    test('should filter correctly with filter tags', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(5).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input')
      searchInput.setValue('china')
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(aclDocs[0], 'aclprofiles')).toBeTruthy()
      expect(numberOfFilteredDocs()).toEqual(1)
    })

    test('should filter correctly with filter connections', async () => {
      // switch filter type
      const searchTypeSelection = wrapper.find('.search-type-selection')
      searchTypeSelection.trigger('click')
      const options = searchTypeSelection.findAll('option')
      options.at(6).setSelected()
      await Vue.nextTick()

      const searchInput = wrapper.find('.search-input')
      searchInput.setValue('default')
      searchInput.trigger('input')
      await Vue.nextTick()
      expect(isItemInFilteredDocs(aclDocs[1], 'aclprofiles')).toBeTruthy()
      expect(isItemInFilteredDocs(securityPoliciesDocs[0], 'securitypolicies')).toBeTruthy()
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
      expect(mockRouter.push).toHaveBeenCalledWith('/config/master/securitypolicies/__default__')
    })
  })
})
