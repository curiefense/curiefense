import TagAutocompleteInput from '@/components/TagAutocompleteInput.vue'
import AutocompleteInput from '@/components/AutocompleteInput.vue'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {mount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'
import axios from 'axios'
import {TagsNamespaceValue} from '@/types'

jest.mock('axios')

describe('TagAutocompleteInput.vue', () => {
  let wrapper: Wrapper<Vue>
  let tagsData: {
    data: TagsNamespaceValue,
  }
  beforeEach(async () => {
    tagsData = {
      data: {
        legitimate: [
          'internal',
          'devops',
          'allowlist',
        ],
        malicious: [
          'malware',
          'blocklist',
        ],
        neutral: [
          'all',
        ],
      },
    }
    jest.spyOn(axios, 'get').mockImplementation(() => Promise.resolve(tagsData))
    jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
    jest.spyOn(axios, 'post').mockImplementation(() => Promise.resolve())
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        autoFocus: true,
        clearInputAfterSelection: false,
      },
    })
    await Vue.nextTick()
  })
  afterEach(() => {
    jest.clearAllMocks()
  })

  test('should send correct tags to AutocompleteInput ordered alphabetically regardless of group', async () => {
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    const suggestionProp = autocompleteInput.props('suggestions')
    expect(suggestionProp.length).toEqual(6)
    expect(suggestionProp[0].value).toEqual('all')
    expect(suggestionProp[1].value).toEqual('allowlist')
    expect(suggestionProp[2].value).toEqual('blocklist')
    expect(suggestionProp[3].value).toEqual('devops')
    expect(suggestionProp[4].value).toEqual('internal')
    expect(suggestionProp[5].value).toEqual('malware')
  })

  test('should send request to create new DB if missing on component creation', async (done) => {
    jest.spyOn(axios, 'get').mockImplementation(() => Promise.reject(new Error()))
    jest.spyOn(axios, 'post').mockImplementation((path) => {
      expect(path).toEqual('/conf/api/v2/db/system/')
      done()
      return Promise.resolve()
    })
    wrapper = mount(TagAutocompleteInput, {})
    await Vue.nextTick()
  })

  test('should not send request to create new DB if exists on component creation', async () => {
    const spy = jest.spyOn(axios, 'post')
    wrapper = mount(TagAutocompleteInput, {})
    await Vue.nextTick()
    expect(spy).not.toHaveBeenCalledWith('db/system/')
  })

  test('should send request to create new key in DB if missing on component creation', async (done) => {
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/db/system/') {
        return Promise.resolve({data: {}})
      }
      return Promise.reject(new Error())
    })
    jest.spyOn(axios, 'put').mockImplementationOnce((path) => {
      expect(path).toEqual('/conf/api/v2/db/system/k/tags/')
      done()
      return Promise.resolve()
    })
    wrapper = mount(TagAutocompleteInput, {})
    await Vue.nextTick()
  })

  test('should not send request to create new key in DB exists on component creation', async () => {
    const spy = jest.spyOn(axios, 'put')
    wrapper = mount(TagAutocompleteInput, {})
    await Vue.nextTick()
    expect(spy).not.toHaveBeenCalledWith('db/system/k/tags')
  })

  test('should send request to add tag neutral list in DB' +
    ' if unknown tag selected - selectionType single, tags added after db loaded', async (done) => {
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        selectionType: 'single',
      },
    })
    // Twice so the DB data will be fully loaded
    await Vue.nextTick()
    await Vue.nextTick()
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    const newTagName = 'tag-of-doom'
    jest.spyOn(axios, 'put').mockImplementationOnce((path, data: TagsNamespaceValue) => {
      expect(data.neutral).toContain(newTagName)
      done()
      return Promise.resolve()
    })
    autocompleteInput.vm.$emit('value-submitted', newTagName)
    await Vue.nextTick()
  })

  test('should send request to add tag neutral list in DB' +
    ' if unknown tag selected - selectionType multiple, tags added after db loaded', async (done) => {
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        selectionType: 'multiple',
      },
    })
    // Twice so that the DB data will be fully loaded
    await Vue.nextTick()
    await Vue.nextTick()
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    const tag1 = 'tag-1'
    const tag2 = 'tag-2'
    const tag3 = 'tag-3'
    jest.spyOn(axios, 'put').mockImplementationOnce((path, data: TagsNamespaceValue) => {
      expect(data.neutral).not.toContain(tag1)
      expect(data.neutral).not.toContain(tag2)
      expect(data.neutral).toContain(tag3)
      done()
      return Promise.resolve()
    })
    autocompleteInput.vm.$emit('value-submitted', `${tag1} ${tag2} ${tag3}`)
    await Vue.nextTick()
  })

  test('should send request to add tag neutral list in DB' +
    ' if unknown tag selected - selectionType single, tags added before db loaded', async (done) => {
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        selectionType: 'single',
      },
    })
    const newTagName = 'tag-of-doom'
    wrapper.setData({tagsAddedWhileSuggestionsLoading: [newTagName]})
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    jest.spyOn(axios, 'put').mockImplementationOnce((path, data: TagsNamespaceValue) => {
      expect(data.neutral).toContain(newTagName)
      done()
      return Promise.resolve()
    })
    autocompleteInput.vm.$emit('value-submitted', newTagName)
    await Vue.nextTick()
    await Vue.nextTick()
  })

  test('should send request to add tag neutral list in DB' +
    ' if unknown tag selected - selectionType multiple, tags added before db loaded', async (done) => {
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        selectionType: 'multiple',
      },
    })
    const tag1 = 'tag-1'
    const tag2 = 'tag-2'
    const tag3 = 'tag-3'
    wrapper.setData({tagsAddedWhileSuggestionsLoading: [tag2, tag3]})
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    jest.spyOn(axios, 'put').mockImplementationOnce((path, data: TagsNamespaceValue) => {
      expect(data.neutral).not.toContain(tag1)
      expect(data.neutral).toContain(tag2)
      expect(data.neutral).toContain(tag3)
      done()
      return Promise.resolve()
    })
    autocompleteInput.vm.$emit('value-submitted', `${tag1} ${tag2} ${tag3}`)
    await Vue.nextTick()
    await Vue.nextTick()
  })

  test('should not send request to add tag list in DB' +
    ' if unknown tag selected before db loaded but exists in legitimate tags list in db', async () => {
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        selectionType: 'single',
      },
    })
    const newTagName = 'internal'
    wrapper.setData({tagsAddedWhileSuggestionsLoading: [newTagName]})
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    const spy = jest.spyOn(axios, 'put')
    autocompleteInput.vm.$emit('value-submitted', newTagName)
    await Vue.nextTick()
    await Vue.nextTick()
    expect(spy).not.toHaveBeenCalled()
  })

  test('should not send request to add tag list in DB' +
    ' if unknown tag selected before db loaded but exists in malicious tags list in db', async () => {
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        selectionType: 'single',
      },
    })
    const newTagName = 'malware'
    wrapper.setData({tagsAddedWhileSuggestionsLoading: [newTagName]})
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    const spy = jest.spyOn(axios, 'put')
    autocompleteInput.vm.$emit('value-submitted', newTagName)
    await Vue.nextTick()
    await Vue.nextTick()
    expect(spy).not.toHaveBeenCalled()
  })

  test('should not send request to add tag list in DB' +
    ' if unknown tag selected before db loaded but exists in neutral tags list in db', async () => {
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        selectionType: 'single',
      },
    })
    const newTagName = 'all'
    wrapper.setData({tagsAddedWhileSuggestionsLoading: [newTagName]})
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    const spy = jest.spyOn(axios, 'put')
    autocompleteInput.vm.$emit('value-submitted', newTagName)
    await Vue.nextTick()
    await Vue.nextTick()
    expect(spy).not.toHaveBeenCalled()
  })

  test('should send request to add tag list in DB' +
    ' if unknown tag selected before db loaded and legitimate list does not exist', async (done) => {
    tagsData = {
      data: {
        malicious: [
          'malware',
          'blocklist',
        ],
        neutral: [
          'all',
        ],
      },
    }
    jest.spyOn(axios, 'get').mockImplementation(() => Promise.resolve(tagsData))
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        selectionType: 'single',
      },
    })
    const newTagName = 'tag-2300'
    wrapper.setData({tagsAddedWhileSuggestionsLoading: [newTagName]})
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    jest.spyOn(axios, 'put').mockImplementationOnce((path, data: TagsNamespaceValue) => {
      expect(data.neutral).toContain(newTagName)
      done()
      return Promise.resolve()
    })
    autocompleteInput.vm.$emit('value-submitted', newTagName)
    await Vue.nextTick()
    await Vue.nextTick()
  })

  test('should send request to add tag list in DB' +
    ' if unknown tag selected before db loaded and malicious list does not exist', async (done) => {
    tagsData = {
      data: {
        legitimate: [
          'internal',
          'allowlist',
        ],
        neutral: [
          'all',
        ],
      },
    }
    jest.spyOn(axios, 'get').mockImplementation(() => Promise.resolve(tagsData))
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        selectionType: 'single',
      },
    })
    const newTagName = 'tag-2300'
    wrapper.setData({tagsAddedWhileSuggestionsLoading: [newTagName]})
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    jest.spyOn(axios, 'put').mockImplementationOnce((path, data: TagsNamespaceValue) => {
      expect(data.neutral).toContain(newTagName)
      done()
      return Promise.resolve()
    })
    autocompleteInput.vm.$emit('value-submitted', newTagName)
    await Vue.nextTick()
    await Vue.nextTick()
  })

  test('should send request to add tag list in DB' +
    ' if unknown tag selected before db loaded and neutral list does not exist', async (done) => {
    tagsData = {
      data: {
        legitimate: [
          'internal',
          'allowlist',
        ],
        malicious: [
          'malware',
          'blocklist',
        ],
      },
    }
    jest.spyOn(axios, 'get').mockImplementation(() => Promise.resolve(tagsData))
    wrapper = mount(TagAutocompleteInput, {
      propsData: {
        selectionType: 'single',
      },
    })
    const newTagName = 'tag-2300'
    wrapper.setData({tagsAddedWhileSuggestionsLoading: [newTagName]})
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    jest.spyOn(axios, 'put').mockImplementationOnce((path, data: TagsNamespaceValue) => {
      expect(data.neutral).toContain(newTagName)
      done()
      return Promise.resolve()
    })
    autocompleteInput.vm.$emit('value-submitted', newTagName)
    await Vue.nextTick()
    await Vue.nextTick()
  })

  test('should not send request to add tag neutral list in DB if known tag selected', async () => {
    const spy = jest.spyOn(axios, 'put')
    const autocompleteInput = wrapper.findComponent(AutocompleteInput)
    const newTagName = 'internal'
    autocompleteInput.vm.$emit('value-submitted', newTagName)
    await Vue.nextTick()
    expect(spy).not.toHaveBeenCalledWith('db/system/k/tags/')
  })

  test('watcher should follow initialTag value', async () => {
    expect((wrapper.vm as any).initialTag).toBeFalsy()
    expect((wrapper.vm as any).tag).toEqual((wrapper.vm as any).initialTag)
    const initialTagValue = 'test'
    wrapper.setProps({initialTag: initialTagValue})
    await Vue.nextTick()
    expect((wrapper.vm as any).tag).toEqual(initialTagValue)
  })

  describe('tags group prefix', () => {
    beforeEach(async () => {
      tagsData = {
        data: {
          legitimate: [
            '000 - legitimate',
          ],
          malicious: [
            '111 - malicious',
          ],
          neutral: [
            '222 - neutral',
          ],
        },
      }
      jest.spyOn(axios, 'get').mockImplementation(() => Promise.resolve(tagsData))
      wrapper = mount(TagAutocompleteInput)
      await Vue.nextTick()
    })

    test('should add correct prefix to tags based on their group - legitimate', async () => {
      const tagsSuggestions = (wrapper.vm as any).tagsSuggestions
      const titleString = 'title="legitimate"'
      const classesString = 'class="dot legitimate"'
      expect(tagsSuggestions[0].prefix).toContain(titleString)
      expect(tagsSuggestions[0].prefix).toContain(classesString)
    })

    test('should add correct prefix to tags based on their group - malicious', async () => {
      const tagsSuggestions = (wrapper.vm as any).tagsSuggestions
      const titleString = 'title="malicious"'
      const classesString = 'class="dot malicious"'
      expect(tagsSuggestions[1].prefix).toContain(titleString)
      expect(tagsSuggestions[1].prefix).toContain(classesString)
    })

    test('should add correct prefix to tags based on their group - neutral', async () => {
      const tagsSuggestions = (wrapper.vm as any).tagsSuggestions
      const titleString = 'title="neutral"'
      const classesString = 'class="dot neutral"'
      expect(tagsSuggestions[2].prefix).toContain(titleString)
      expect(tagsSuggestions[2].prefix).toContain(classesString)
    })
  })

  describe('props propagation and event bubbling', () => {
    const propInitialTag = 'test'
    const propClearInputAfterSelection = true
    const propAutoFocus = true
    const propSelectionType = 'multiple'
    beforeEach(async () => {
      wrapper = mount(TagAutocompleteInput, {
        propsData: {
          initialTag: propInitialTag,
          clearInputAfterSelection: propClearInputAfterSelection,
          autoFocus: propAutoFocus,
          selectionType: propSelectionType,
        },
      })
      await Vue.nextTick()
    })

    test('should propagate initialTag correctly to AutocompleteInput', async () => {
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      const prop = autocompleteInput.props('initialValue')
      expect(prop).toEqual(propInitialTag)
    })

    test('should propagate clearInputAfterSelection correctly to AutocompleteInput', async () => {
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      const prop = autocompleteInput.props('clearInputAfterSelection')
      expect(prop).toEqual(propClearInputAfterSelection)
    })

    test('should propagate autoFocus correctly to AutocompleteInput', async () => {
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      const prop = autocompleteInput.props('autoFocus')
      expect(prop).toEqual(propAutoFocus)
    })

    test('should propagate selectionType correctly to AutocompleteInput', async () => {
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      const prop = autocompleteInput.props('selectionType')
      expect(prop).toEqual(propSelectionType)
    })

    test('should bubble value-changed event as tag-changed', async () => {
      const emitValue = 'some-value'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('value-changed', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('tag-changed')).toBeTruthy()
      expect(wrapper.emitted('tag-changed')[0]).toEqual([emitValue])
    })

    test('should bubble value-submitted event as tag-submitted', async () => {
      const emitValue = 'some-value'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('value-submitted', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('tag-submitted')).toBeTruthy()
      expect(wrapper.emitted('tag-submitted')[0]).toEqual([emitValue])
    })

    test('should ignore multiple spaces and trim when emitting tag-changed', async () => {
      const emitValue = 'some-value                    some-other-value    '
      const wantedValue = 'some-value some-other-value'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('value-changed', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('tag-changed')).toBeTruthy()
      expect(wrapper.emitted('tag-changed')[0]).toEqual([wantedValue])
    })

    test('should ignore multiple spaces and trim when emitting tag-submitted', async () => {
      const emitValue = 'some-value                    some-other-value    '
      const wantedValue = 'some-value some-other-value'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('value-submitted', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('tag-submitted')).toBeTruthy()
      expect(wrapper.emitted('tag-submitted')[0]).toEqual([wantedValue])
    })

    test('should ignore tabs and trim when emitting tag-changed', async () => {
      const emitValue = 'some-value             some-other-value        '
      const wantedValue = 'some-value some-other-value'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('value-changed', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('tag-changed')).toBeTruthy()
      expect(wrapper.emitted('tag-changed')[0]).toEqual([wantedValue])
    })

    test('should ignore tabs and trim when emitting tag-submitted', async () => {
      const emitValue = 'some-value             some-other-value        '
      const wantedValue = 'some-value some-other-value'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('value-submitted', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('tag-submitted')).toBeTruthy()
      expect(wrapper.emitted('tag-submitted')[0]).toEqual([wantedValue])
    })

    test('should ignore new lines and trim when emitting tag-changed', async () => {
      const emitValue = 'some-value \n some-other-value \n'
      const wantedValue = 'some-value some-other-value'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('value-changed', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('tag-changed')).toBeTruthy()
      expect(wrapper.emitted('tag-changed')[0]).toEqual([wantedValue])
    })

    test('should ignore new lines and trim when emitting tag-submitted', async () => {
      const emitValue = 'some-value \n some-other-value \n'
      const wantedValue = 'some-value some-other-value'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('value-submitted', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('tag-submitted')).toBeTruthy()
      expect(wrapper.emitted('tag-submitted')[0]).toEqual([wantedValue])
    })

    test('should bubble keyup event', async () => {
      const emitValue = 'enter'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('keyup', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('keyup')).toBeTruthy()
      expect(wrapper.emitted('keyup')[0]).toEqual([emitValue])
    })

    test('should bubble keydown event', async () => {
      const emitValue = 'enter'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('keydown', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('keydown')).toBeTruthy()
      expect(wrapper.emitted('keydown')[0]).toEqual([emitValue])
    })

    test('should bubble keypress event', async () => {
      const emitValue = 'enter'
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('keypress', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('keypress')).toBeTruthy()
      expect(wrapper.emitted('keypress')[0]).toEqual([emitValue])
    })

    test('should bubble focus event', async () => {
      const emitValue = new Event('focus')
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('focus', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('focus')).toBeTruthy()
      expect(wrapper.emitted('focus')[0]).toEqual([emitValue])
    })

    test('should bubble blur event', async () => {
      const emitValue = new Event('blur')
      const autocompleteInput = wrapper.findComponent(AutocompleteInput)
      autocompleteInput.vm.$emit('blur', emitValue)
      await Vue.nextTick()
      expect(wrapper.emitted('blur')).toBeTruthy()
      expect(wrapper.emitted('blur')[0]).toEqual([emitValue])
    })
  })

  describe('selection type prop validator', () => {
    let validator: Function
    beforeEach(() => {
      validator = (wrapper.vm as any).$options.props.selectionType.validator
    })

    test('should return true for `single` type`', () => {
      const isValid = validator('single')
      expect(isValid).toEqual(true)
    })

    test('should return true for `multiple` type`', () => {
      const isValid = validator('multiple')
      expect(isValid).toEqual(true)
    })

    test('should return true for type regardless of casing`', () => {
      const isValid = validator('MuLtIpLe')
      expect(isValid).toEqual(true)
    })

    test('should return false for type not `single` or `multiple`', () => {
      const type = 'unknown value'
      const isValid = validator(type)
      expect(isValid).toEqual(false)
    })

    test('should return false for undefined type', () => {
      const type: string = undefined
      const isValid = validator(type)
      expect(isValid).toEqual(false)
    })
  })
})
