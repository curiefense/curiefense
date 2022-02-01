import RateLimitsEditor from '@/doc-editors/RateLimitsEditor.vue'
import LimitOption from '@/components/LimitOption.vue'
import ResponseAction from '@/components/ResponseAction.vue'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {mount, shallowMount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'
import {RateLimit, SecurityPolicy} from '@/types'
import axios from 'axios'
import TagAutocompleteInput from '@/components/TagAutocompleteInput.vue'
import _ from 'lodash'

jest.mock('axios')

describe('RateLimitsEditor.vue', () => {
  let rateLimitsDocs: RateLimit[]
  let securityPoliciesDocs: SecurityPolicy[]
  let mockRouter: any
  let wrapper: Wrapper<Vue>
  let enableListener: Boolean
  beforeEach(() => {
    rateLimitsDocs = [{
      'id': 'f971e92459e2',
      'name': 'Rate Limit Example Rule 5/60',
      'description': '5 requests per minute',
      'thresholds': [
        {
          'limit': '5',
          'action': {'type': 'default'},
        },
      ],
      'timeframe': '60',
      'include': ['blocklist'],
      'exclude': ['allowlist'],
      'key': [{'attrs': 'ip'}],
      'pairwith': {'self': 'self'},
    }]
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
    jest.spyOn(axios, 'get').mockImplementation((path, config) => {
      if (!wrapper) {
        return Promise.resolve({data: []})
      }
      const branch = (wrapper.vm as any).selectedBranch
      if (path === `/conf/api/v2/configs/${branch}/d/securitypolicies/`) {
        if (config && config.headers && config.headers['x-fields'] === 'id, name') {
          return Promise.resolve({data: _.map(securityPoliciesDocs, (i) => _.pick(i, 'id', 'name'))})
        }
        return Promise.resolve({data: securityPoliciesDocs})
      }
      return Promise.resolve({data: []})
    })
    mockRouter = {
      push: jest.fn(),
    }
    const onUpdate = (selectedDoc: Object) => {
      if (enableListener) {
        wrapper.setProps({selectedDoc})
      }
    }
    enableListener = false
    wrapper = mount(RateLimitsEditor, {
      propsData: {
        selectedDoc: rateLimitsDocs[0],
        selectedBranch: 'master',
      },
      mocks: {
        $router: mockRouter,
      },
      listeners: {
        'update:selectedDoc': onUpdate,
      },
    })
  })

  describe('form data', () => {
    test('should have correct ID displayed', () => {
      expect(wrapper.find('.document-id').text()).toEqual(rateLimitsDocs[0].id)
    })

    test('should have correct name in input', () => {
      const element = wrapper.find('.document-name').element as HTMLInputElement
      expect(element.value).toEqual(rateLimitsDocs[0].name)
    })

    test('should have correct description in input', () => {
      const element = wrapper.find('.document-description').element as HTMLInputElement
      expect(element.value).toEqual(rateLimitsDocs[0].description)
    })

    test('should show error when more than one ban actions exist', async () => {
      enableListener = true
      const addKeyButton = wrapper.find('.add-threshold-button')
      addKeyButton.trigger('click')
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      expect(wrapper.find('.only-one-ban').element).toBeUndefined()
      const responseActionComponents = wrapper.findAllComponents(ResponseAction)
      responseActionComponents
        .at(0).findAll('option')
        .at(5).setSelected()
      await Vue.nextTick()
      responseActionComponents
        .at(1).findAll('option')
        .at(5).setSelected()
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      expect(wrapper.find('.only-one-ban').element).toBeDefined()
    })

    test('should have correct threshold in input', () => {
      const element = wrapper.find('.document-limit').element as HTMLInputElement
      expect(element.value).toEqual(rateLimitsDocs[0].thresholds[0].limit)
    })

    test('should have correct Time Frame in input', () => {
      const element = wrapper.find('.document-timeframe').element as HTMLInputElement
      expect(element.value).toEqual(rateLimitsDocs[0].timeframe)
    })

    test('should have count-by limit option component with correct data', () => {
      const wantedType = Object.keys(rateLimitsDocs[0].key[0])[0]
      const wantedValue = Object.values(rateLimitsDocs[0].key[0])[0]
      const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(0)
      const actualType = (limitOptionComponent.vm as any).option.type
      const actualValue = (limitOptionComponent.vm as any).option.key
      expect(actualType).toEqual(wantedType)
      expect(actualValue).toEqual(wantedValue)
    })

    test('should have event limit option component with correct data', () => {
      const wantedType = Object.keys(rateLimitsDocs[0].pairwith)[0]
      const wantedValue = Object.values(rateLimitsDocs[0].pairwith)[0]
      const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(1)
      const actualType = (limitOptionComponent.vm as any).option.type
      const actualValue = (limitOptionComponent.vm as any).option.key
      expect(actualType).toEqual(wantedType)
      expect(actualValue).toEqual(wantedValue)
    })

    test('should have count-by limit option component with correct ignored attributes', () => {
      const wantedIgnoredAttributes = ['tags']
      const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(0)
      const actualIgnoredAttributes = (limitOptionComponent.vm as any).ignoreAttributes
      expect(wantedIgnoredAttributes).toEqual(actualIgnoredAttributes)
    })

    test('should have event limit option component with correct ignored attributes', () => {
      const wantedIgnoredAttributes = ['tags']
      const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(0)
      const actualIgnoredAttributes = (limitOptionComponent.vm as any).ignoreAttributes
      expect(wantedIgnoredAttributes).toEqual(actualIgnoredAttributes)
    })

    test('should have response action component with correct data', () => {
      const ResponseActionComponent = wrapper.findComponent(ResponseAction)
      expect((ResponseActionComponent.vm as any).action).toEqual(rateLimitsDocs[0].thresholds[0].action)
    })

    test('should have correct include data in table', () => {
      const includeTags = wrapper.find('.include-filter-column')
      const includeTagsCell0 = includeTags.find('.tag-cell')
      const wantedIncludeTags = rateLimitsDocs[0].include.toString()
      expect(includeTagsCell0.text()).toEqual(wantedIncludeTags)
    })

    test('should have correct exclude data in table', () => {
      const excludeTags = wrapper.find('.exclude-filter-column')
      const includeTagsCell0 = excludeTags.find('.tag-cell')
      const wantedExcludeTags = rateLimitsDocs[0].exclude.toString()
      expect(includeTagsCell0.text()).toEqual(wantedExcludeTags)
    })
  })

  describe('count by key', () => {
    test('should add key when button is clicked', async () => {
      const addKeyButton = wrapper.find('.add-key-button')
      addKeyButton.trigger('click')
      await Vue.nextTick()
      const wantedType = 'attrs'
      const wantedValue = 'ip'
      const actualType = Object.keys((wrapper.vm as any).localDoc.key[1])[0]
      const actualValue = Object.values((wrapper.vm as any).localDoc.key[1])[0]
      expect((wrapper.vm as any).localDoc.key.length).toEqual(2)
      expect(actualType).toEqual(wantedType)
      expect(actualValue).toEqual(wantedValue)
    })

    test('should handle key with no value', async () => {
      rateLimitsDocs[0].key = [{'headers': null}]
      wrapper = mount(RateLimitsEditor, {
        propsData: {
          selectedDoc: rateLimitsDocs[0],
        },
      })
      const wantedType = 'headers'
      const actualType = Object.keys((wrapper.vm as any).localDoc.key[0])[0]
      const actualValue = Object.values((wrapper.vm as any).localDoc.key[0])[0]
      expect(actualType).toEqual(wantedType)
      expect(actualValue).toEqual(null)
    })

    test('should show error when two of the same key type exist', async () => {
      const addKeyButton = wrapper.find('.add-key-button')
      addKeyButton.trigger('click')
      await Vue.nextTick()
      addKeyButton.trigger('click')
      await Vue.nextTick()
      const keyInvalidLabel = wrapper.find('.key-invalid')
      expect(keyInvalidLabel.element).toBeDefined()
    })

    test('should remove key when remove event occurs', async () => {
      const addKeyButton = wrapper.find('.add-key-button')
      addKeyButton.trigger('click')
      await Vue.nextTick()
      const limitOptionsComponent = wrapper.findComponent(LimitOption)
      limitOptionsComponent.vm.$emit('remove', 1)
      await Vue.nextTick()
      expect((wrapper.vm as any).localDoc.key.length).toEqual(1)
    })

    test('should not be able to remove key when only one key exists', async () => {
      const limitOptionsComponent = wrapper.findComponent(LimitOption)
      limitOptionsComponent.vm.$emit('remove', 1)
      await Vue.nextTick()
      expect((wrapper.vm as any).localDoc.key.length).toEqual(1)
    })

    test('should update key when change event occurs', async () => {
      const newOption = {
        type: 'self',
        key: 'self',
      }
      const wantedResult = {
        self: 'self',
      }
      const limitOptionsComponent = wrapper.findComponent(LimitOption)
      limitOptionsComponent.vm.$emit('change', newOption, 0)
      await Vue.nextTick()
      expect((wrapper.vm as any).localDoc.key[0]).toEqual(wantedResult)
    })

    test('should handle selectedDoc with undefined key value', async (done) => {
      try {
        rateLimitsDocs[0].key = [{'headers': null}, undefined]
        wrapper = mount(RateLimitsEditor, {
          propsData: {
            selectedDoc: rateLimitsDocs[0],
          },
        })
        await Vue.nextTick()
        done()
      } catch (err) {
        expect(err).not.toBeDefined()
        done()
      }
    })
  })

  describe('thresholds', () => {
    test('should add threshold when button is clicked', async () => {
      const addThresholdButton = wrapper.find('.add-threshold-button')
      addThresholdButton.trigger('click')
      await Vue.nextTick()
      const wantedLimit = ''
      const wantedAction = {type: 'default'}
      const actualLimit = (wrapper.vm as any).localDoc.thresholds[1].limit
      const actualAction = (wrapper.vm as any).localDoc.thresholds[1].action
      expect((wrapper.vm as any).localDoc.thresholds.length).toEqual(2)
      expect(actualLimit).toEqual(wantedLimit)
      expect(actualAction).toEqual(wantedAction)
    })

    test('should remove threshold when remove event occurs', async () => {
      expect((wrapper.vm as any).localDoc.thresholds.length).toEqual(1)
      const addThresholdButton = wrapper.find('.add-threshold-button')
      addThresholdButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).localDoc.thresholds.length).toEqual(2)
      const removeThresholdButton = wrapper.find('.remove-threshold-option-button')
      removeThresholdButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).localDoc.thresholds.length).toEqual(1)
    })

    test('should not be able to remove threshold when only one key exists', async () => {
      const removeThresholdButton = wrapper.find('.remove-threshold-option-button')
      removeThresholdButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).localDoc.thresholds.length).toEqual(1)
    })

    test('should update threshold when change event occurs', async () => {
      const newLimitOption = '20'
      const newActionOption = {
        new: 'value',
        params: {},
      }
      const thresholdLimitField = wrapper.find('.document-limit')
      thresholdLimitField.setValue(newLimitOption)
      thresholdLimitField.trigger('change')
      console.log((wrapper.vm as any).localDoc.thresholds)
      const thresholdActionField = wrapper.findComponent(ResponseAction)
      thresholdActionField.vm.$emit('update:action', newActionOption, 0)
      await Vue.nextTick()
      expect((wrapper.vm as any).localDoc.thresholds[0].limit).toEqual(newLimitOption)
      expect((wrapper.vm as any).localDoc.thresholds[0].action).toEqual(newActionOption)
    })
  })

  describe('event', () => {
    test('should handle key with no value', async () => {
      rateLimitsDocs[0].pairwith = {'self': null}
      wrapper = mount(RateLimitsEditor, {
        propsData: {
          selectedDoc: rateLimitsDocs[0],
        },
      })
      const wantedType = 'self'
      const actualType = Object.keys((wrapper.vm as any).localDoc.pairwith)[0]
      const actualValue = Object.values((wrapper.vm as any).localDoc.pairwith)[0]
      expect(actualType).toEqual(wantedType)
      expect(actualValue).toEqual(null)
    })

    test('should update key when change event occurs', async () => {
      const newOption = {
        type: 'self',
        key: 'self',
      }
      const wantedResult = {
        self: 'self',
      }
      const limitOptionsComponent = wrapper.findAllComponents(LimitOption).at(1)
      limitOptionsComponent.vm.$emit('change', newOption, 0)
      await Vue.nextTick()
      expect((wrapper.vm as any).localDoc.pairwith).toEqual(wantedResult)
    })

    test('should handle selectedDoc without pairwith property', async (done) => {
      try {
        delete rateLimitsDocs[0].pairwith
        wrapper = mount(RateLimitsEditor, {
          propsData: {
            selectedDoc: rateLimitsDocs[0],
          },
        })
        await Vue.nextTick()
        done()
      } catch (err) {
        expect(err).not.toBeDefined()
        done()
      }
    })
  })

  describe('tags', () => {
    beforeEach(() => {
      const tagsData = {
        data: {
          tags: [
            'united-states',
            'test-tag-1',
            'test-tag-2',
            'another-tag',
            'devops',
            'internal',
          ],
        },
      }
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === `db/master/k/autocomplete/`) {
          return Promise.resolve(tagsData)
        }
        return Promise.resolve()
      })
    })

    test('should not have any warning in the tags table when there are no duplicate tags', () => {
      const tagsWithWarning = wrapper.findAll('.has-text-danger')
      expect(tagsWithWarning.length).toEqual(0)
    })

    test('should emit doc update when adding tags', async () => {
      const newTag = 'test-tag'
      const wantedEmit = JSON.parse(JSON.stringify(rateLimitsDocs[0]))
      wantedEmit.include.push(newTag)
      const newIncludeEntryButton = wrapper.findAll('.add-new-filter-entry-button').at(0)
      // add first
      newIncludeEntryButton.trigger('click')
      await Vue.nextTick()
      const firstTagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
      firstTagAutocompleteInput.vm.$emit('tag-submitted', newTag)
      await Vue.nextTick()
      // check
      expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
      expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([wantedEmit])
    })

    test('should show a warning when there are duplicate tags', async () => {
      const duplicatedTagsDoc = JSON.parse(JSON.stringify(rateLimitsDocs[0]))
      duplicatedTagsDoc.include = ['test-tag', 'test-tag']
      wrapper.setProps({selectedDoc: duplicatedTagsDoc})
      await Vue.nextTick()
      // check
      const tagsWithWarning = wrapper.findAll('.has-text-danger')
      expect(tagsWithWarning.length).toEqual(2)
    })

    test('should not emit doc update when adding tags which is 2 or less characters long', async () => {
      const newTag = 't'
      const wantedEmit = JSON.parse(JSON.stringify(rateLimitsDocs[0]))
      wantedEmit.include.push(newTag)
      const newIncludeEntryButton = wrapper.findAll('.add-new-filter-entry-button').at(0)
      // add first
      newIncludeEntryButton.trigger('click')
      await Vue.nextTick()
      const firstTagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
      firstTagAutocompleteInput.vm.$emit('tag-submitted', newTag)
      await Vue.nextTick()
      // check
      expect(wrapper.emitted('update:selectedDoc')).toBeFalsy()
    })

    test('should remove tag from correct filter when tag removed', async () => {
      const removeIncludeEntryButton = wrapper.find('.remove-filter-entry-button')
      removeIncludeEntryButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).localDoc.include.length).toEqual(0)
    })

    test('should hide tag input when tag selection cancelled', async () => {
      const newIncludeEntryButton = wrapper.find('.add-new-filter-entry-button')
      newIncludeEntryButton.trigger('click')
      await Vue.nextTick();
      (wrapper.vm as any).cancelAddNewTag()
      await Vue.nextTick()
      const tagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
      await Vue.nextTick()
      expect(tagAutocompleteInput.element).toBeUndefined()
    })
  })

  describe('connected Security Policies', () => {
    afterEach(() => {
      jest.clearAllMocks()
    })

    test('should display all connected Security Policies', () => {
      const connectedSecurityPoliciesEntriesRows = wrapper.findAll('.connected-entry-row')
      expect(connectedSecurityPoliciesEntriesRows.length).toEqual(2)
    })

    test('should have a link to each connected Security Policy', async () => {
      const wantedRoute = `/config/${(wrapper.vm as any).selectedBranch}/securitypolicies/${securityPoliciesDocs[0].id}`
      const connectedSecurityPoliciesEntryRow = wrapper.findAll('.connected-entry-row').at(0)
      const referralButton = connectedSecurityPoliciesEntryRow.find('.security-policy-referral-button')
      await referralButton.trigger('click')
      await Vue.nextTick()
      expect(mockRouter.push).toHaveBeenCalledTimes(1)
      expect(mockRouter.push).toHaveBeenCalledWith(wantedRoute)
    })

    test('should show the new connection row when `+` button is clicked', async () => {
      const newConnectionButton = wrapper.find('.new-connection-button')
      newConnectionButton.trigger('click')
      await Vue.nextTick()
      const newConnectionRow = wrapper.find('.new-connection-row')
      expect(newConnectionRow.exists()).toBeTruthy()
    })

    test('should show an appropriate message when there are no available new connections', async () => {
      const wantedMessage = `All Security Policies entries are currently connected to this Rate Limit`
      securityPoliciesDocs[0].map[1].limit_ids.push(rateLimitsDocs[0].id)
      securityPoliciesDocs[1].map[1].limit_ids.push(rateLimitsDocs[0].id)
      wrapper = shallowMount(RateLimitsEditor, {
        propsData: {
          selectedDoc: rateLimitsDocs[0],
          selectedBranch: 'master',
        },
      })
      await Vue.nextTick()
      const newConnectionButton = wrapper.find('.new-connection-button')
      newConnectionButton.trigger('click')
      await Vue.nextTick()
      const newConnectionRow = wrapper.find('.new-connection-row')
      expect(newConnectionRow.text()).toEqual(wantedMessage)
    })

    test('should hide the new connection row when `-` button is clicked', async () => {
      let newConnectionButton = wrapper.find('.new-connection-button')
      newConnectionButton.trigger('click')
      await Vue.nextTick()
      newConnectionButton = wrapper.find('.new-connection-button')
      newConnectionButton.trigger('click')
      await Vue.nextTick()
      const newConnectionRow = wrapper.find('.new-connection-row')
      expect(newConnectionRow.exists()).toBeFalsy()
    })

    test('should send request to change Security Policy when new connection is added', async () => {
      const putSpy = jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
      const wantedUrl = `/conf/api/v2/configs/${(wrapper.vm as any).selectedBranch}/d/securitypolicies/e/${securityPoliciesDocs[1].id}/`
      const wantedDoc = JSON.parse(JSON.stringify(securityPoliciesDocs[1]))
      wantedDoc.map[1].limit_ids.push(rateLimitsDocs[0].id)
      const newConnectionButton = wrapper.find('.new-connection-button')
      newConnectionButton.trigger('click')
      await Vue.nextTick()
      const newConnectionRow = wrapper.find('.new-connection-row')
      const newConnectionMapSelection = newConnectionRow.find('.new-connection-map')
      const options = newConnectionMapSelection.findAll('option')
      options.at(1).setSelected()
      await Vue.nextTick()
      const addNewConnectionButton = wrapper.find('.add-new-connection')
      addNewConnectionButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(wantedUrl, wantedDoc)
    })

    test('should send request to change Security Policy when removing connection was confirmed', async () => {
      const putSpy = jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
      const wantedUrl = `/conf/api/v2/configs/${(wrapper.vm as any).selectedBranch}/d/securitypolicies/e/${securityPoliciesDocs[0].id}/`
      const wantedDoc = JSON.parse(JSON.stringify(securityPoliciesDocs[0]))
      wantedDoc.map[0].limit_ids = []
      const removeConnectionButton = wrapper.findAll('.remove-connection-button').at(0)
      removeConnectionButton.trigger('click')
      await Vue.nextTick()
      const confirmRemoveConnectionButton = wrapper.find('.confirm-remove-connection-button')
      confirmRemoveConnectionButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(wantedUrl, wantedDoc)
    })

    test('should not send request to change Security Policy when removing connection was cancelled', async () => {
      const putSpy = jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
      const removeConnectionButton = wrapper.findAll('.remove-connection-button').at(0)
      removeConnectionButton.trigger('click')
      await Vue.nextTick()
      const cancelRemoveConnectionButton = wrapper.find('.cancel-remove-connection-button')
      cancelRemoveConnectionButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).not.toHaveBeenCalled()
    })
  })
})
