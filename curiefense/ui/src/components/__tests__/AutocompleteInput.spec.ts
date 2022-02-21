import AutocompleteInput from '@/components/AutocompleteInput.vue'
import {beforeEach, describe, expect, jest, test} from '@jest/globals'
import {mount, Wrapper, WrapperArray} from '@vue/test-utils'
import Vue from 'vue'
import axios from 'axios'
import * as bulmaToast from 'bulma-toast'
import {Options} from 'bulma-toast'

jest.mock('axios')

describe('AutocompleteInput.vue', () => {
  let wrapper: Wrapper<Vue>
  let suggestions: any[]
  beforeEach(() => {
    suggestions = [
      {
        prefix: '<span>prefix html</span>',
        value: 'another-value',
      },
      {
        value: 'devops',
      },
      {
        value: 'internal',
      },
      {
        prefix: 'prefix string',
        value: 'test-value-1',
      },
      {
        value: 'test-value-2',
      },
      {
        value: 'united-states',
      },
    ]
    wrapper = mount(AutocompleteInput, {
      propsData: {
        suggestions: suggestions,
        autoFocus: true,
        clearInputAfterSelection: false,
      },
    })
  })

  test('should have dropdown hidden on init', () => {
    expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeFalsy()
  })

  test('should have dropdown hidden after typing in input if empty', async () => {
    jest.spyOn(axios, 'get').mockImplementation(() => Promise.resolve({data: {}}))
    wrapper = mount(AutocompleteInput)
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    await Vue.nextTick()
    expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeFalsy()
  })

  test('should have dropdown displayed after typing in input', async () => {
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    await Vue.nextTick()
    expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeTruthy()
  })

  test('should emit changed value when input changes', async () => {
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    await Vue.nextTick()
    expect(wrapper.emitted('value-changed')).toBeTruthy()
    expect(wrapper.emitted('value-changed')[0]).toEqual(['value'])
  })

  test('should show correct filtered values in dropdown ordered alphabetically', async () => {
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    await Vue.nextTick()
    const dropdownItems = wrapper.findAll('.dropdown-item')
    expect(dropdownItems.length).toEqual(3)
    expect(dropdownItems.at(0).text()).toContain('another-value')
    expect(dropdownItems.at(1).text()).toContain('test-value-1')
    expect(dropdownItems.at(2).text()).toContain('test-value-2')
  })

  test('should show correct prefixes in dropdown', async () => {
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    await Vue.nextTick()
    const dropdownItems = wrapper.findAll('.dropdown-item')
    expect(dropdownItems.at(0).html()).toContain('<span>prefix html</span>')
    expect(dropdownItems.at(1).html()).toContain('prefix string')
  })

  test('should show correct filtered values in dropdown ordered alphabetically regardless of casing', async () => {
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    await Vue.nextTick()
    const dropdownItems = wrapper.findAll('.dropdown-item')
    expect(dropdownItems.length).toEqual(3)
    expect(dropdownItems.at(0).text()).toContain('another-value')
    expect(dropdownItems.at(1).text()).toContain('test-value-1')
    expect(dropdownItems.at(2).text()).toContain('test-value-2')
  })

  test('should re-assign the input when prop changes', async () => {
    const newValue = 'another'
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    wrapper.setProps({initialValue: newValue})
    await Vue.nextTick()
    expect((input.element as HTMLInputElement).value).toEqual(newValue)
  })

  test('should have dropdown hidden when prop changes', async () => {
    const newValue = 'another'
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    wrapper.setProps({initialValue: newValue})
    await Vue.nextTick()
    expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeFalsy()
  })

  test('should have dropdown hidden when prop changes to the same value', async () => {
    const value = 'value'
    const input = wrapper.find('.autocomplete-input')
    input.setValue(value)
    input.trigger('input')
    wrapper.setProps({initialValue: value})
    await Vue.nextTick()
    expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeTruthy()
  })

  test('should clear autocomplete input when selected', async () => {
    wrapper.setProps({clearInputAfterSelection: true})
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    wrapper.setData({focusedSuggestionIndex: 2})
    input.trigger('keydown.enter')
    await Vue.nextTick()
    expect((input.element as HTMLInputElement).value).toEqual('')
  })

  test('should emit selected value on input blur event', (done) => {
    wrapper.setProps({clearInputAfterSelection: true})
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    wrapper.setData({focusedSuggestionIndex: 2})
    input.trigger('blur')
    setTimeout(() => {
      expect(wrapper.emitted('value-submitted')).toBeTruthy()
      expect(wrapper.emitted('value-submitted')[0]).toEqual(['test-value-2'])
      done()
    }, 0)
  })

  test('should emit selected value on input blur event with the correct clicked suggestion', (done) => {
    wrapper.setProps({clearInputAfterSelection: true})
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    const dropdownItems = wrapper.findAll('.dropdown-item')
    input.trigger('blur')
    dropdownItems.at(1).trigger('mousedown')
    setTimeout(() => {
      expect(wrapper.emitted('value-submitted')).toBeTruthy()
      expect(wrapper.emitted('value-submitted')[0]).toEqual(['test-value-1'])
      done()
    }, 0)
  })

  test('should not emit selected value on input blur event if destroyed before finishing', async (done) => {
    wrapper.setProps({clearInputAfterSelection: true})
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    input.trigger('blur')
    wrapper.destroy()
    await Vue.nextTick()
    setTimeout(() => {
      expect(wrapper.emitted('value-submitted')).toBeFalsy()
      done()
    }, 0)
  })

  test('should auto focus on the autocomplete input after suggestion clicked' +
    ' if autoFocus prop is true', async (done) => {
    const elem = document.createElement('div')
    if (document.body) {
      document.body.appendChild(elem)
    }
    wrapper = mount(AutocompleteInput, {
      propsData: {
        suggestions: suggestions,
        autoFocus: true,
        clearInputAfterSelection: false,
      },
      attachTo: elem,
    })
    await Vue.nextTick()
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    const dropdownItems = wrapper.findAll('.dropdown-item')
    dropdownItems.at(1).trigger('mousedown')
    await Vue.nextTick()
    setTimeout(() => {
      expect(input.element).toBe(document.activeElement)
      done()
    }, 0)
  })

  test('should not auto focus on the autocomplete input after suggestion clicked' +
    ' if autoFocus prop is false', async (done) => {
    const elem = document.createElement('div')
    if (document.body) {
      document.body.appendChild(elem)
    }
    wrapper = mount(AutocompleteInput, {
      propsData: {
        suggestions: suggestions,
        autoFocus: false,
        clearInputAfterSelection: false,
      },
      attachTo: elem,
    })
    await Vue.nextTick()
    const input = wrapper.find('.autocomplete-input')
    input.setValue('value')
    input.trigger('input')
    const dropdownItems = wrapper.findAll('.dropdown-item')
    dropdownItems.at(1).trigger('mousedown')
    await Vue.nextTick()
    setTimeout(() => {
      expect(input.element).not.toBe(document.activeElement)
      done()
    }, 0)
  })

  describe('keyboard control', () => {
    let input: Wrapper<Vue>
    let dropdownItems: WrapperArray<Vue>
    beforeEach(async () => {
      input = wrapper.find('.autocomplete-input')
      input.setValue('value')
      input.trigger('input')
      dropdownItems = wrapper.findAll('.dropdown-item')
    })

    test('should focus on next item when down arrow is pressed', async () => {
      input.trigger('keydown.down')
      await Vue.nextTick()
      expect(dropdownItems.at(0).element.classList.contains('is-active')).toBeTruthy()
    })

    test('should focus on previous item when up arrow is pressed', async () => {
      wrapper.setData({focusedSuggestionIndex: 2})
      input.trigger('keydown.up')
      await Vue.nextTick()
      expect(dropdownItems.at(1).element.classList.contains('is-active')).toBeTruthy()
    })

    test('should not focus on next item when down arrow is pressed if focused on last element', async () => {
      wrapper.setData({focusedSuggestionIndex: 2})
      input.trigger('keydown.down')
      await Vue.nextTick()
      expect(dropdownItems.at(2).element.classList.contains('is-active')).toBeTruthy()
    })

    test('should not focus on any item when up arrow is pressed if focused on input', async () => {
      wrapper.setData({focusedSuggestionIndex: -1})
      input.trigger('keydown.up')
      await Vue.nextTick()
      expect(dropdownItems.at(0).element.classList.contains('is-active')).toBeFalsy()
      expect(dropdownItems.at(1).element.classList.contains('is-active')).toBeFalsy()
      expect(dropdownItems.at(2).element.classList.contains('is-active')).toBeFalsy()
    })

    test('should select focused suggestion when enter is pressed', async () => {
      wrapper.setData({focusedSuggestionIndex: 2})
      input.trigger('keydown.enter')
      await Vue.nextTick()
      expect((input.element as HTMLInputElement).value).toEqual('test-value-2')
    })

    test('should select input value when enter is pressed and there is no focused suggestion', async () => {
      wrapper.setData({focusedSuggestionIndex: -1})
      input.setValue('test-value-1')
      input.trigger('input')
      input.trigger('keydown.enter')
      await Vue.nextTick()
      expect((input.element as HTMLInputElement).value).toEqual('test-value-1')
    })

    test('should emit selected value when enter is pressed', async () => {
      wrapper.setData({focusedSuggestionIndex: 2})
      input.trigger('keydown.enter')
      expect(wrapper.emitted('value-submitted')).toBeTruthy()
      expect(wrapper.emitted('value-submitted')[0]).toEqual(['test-value-2'])
    })

    test('should select focused suggestion when space is pressed', async () => {
      wrapper.setData({focusedSuggestionIndex: 2})
      input.trigger('keydown.space')
      await Vue.nextTick()
      expect((input.element as HTMLInputElement).value).toEqual('test-value-2')
    })

    test('should select input value when space is pressed and there is no focused suggestion', async () => {
      wrapper.setData({focusedSuggestionIndex: -1})
      input.setValue('test-value-1')
      input.trigger('input')
      input.trigger('keydown.space')
      await Vue.nextTick()
      expect((input.element as HTMLInputElement).value).toEqual('test-value-1')
    })

    test('should select input value when input is longer than the minimumValueLength prop', async () => {
      wrapper.setProps({minimumValueLength: 3})
      await Vue.nextTick()
      input.setValue('test-value-1')
      input.trigger('input')
      input.trigger('keydown.enter')
      await Vue.nextTick()
      expect(wrapper.emitted('value-submitted')).toBeTruthy()
    })

    test('should not select input value when input is empty', async () => {
      wrapper.setProps({minimumValueLength: 3})
      await Vue.nextTick()
      input.setValue('')
      input.trigger('input')
      input.trigger('keydown.enter')
      await Vue.nextTick()
      expect(wrapper.emitted('value-submitted')).toBeFalsy()
    })

    test('should not select input value when input is shorter than the minimumValueLength prop', async () => {
      wrapper.setProps({minimumValueLength: 3})
      await Vue.nextTick()
      input.setValue('t')
      input.trigger('input')
      input.trigger('keydown.enter')
      await Vue.nextTick()
      expect(wrapper.emitted('value-submitted')).toBeFalsy()
    })

    test('should not display a failure toast after selecting input value when input is empty', async () => {
      const toastOutput: Options[] = []
      jest.spyOn(bulmaToast, 'toast').mockImplementation((output: Options) => {
        toastOutput.push(output)
      })
      wrapper.setProps({minimumValueLength: 3})
      await Vue.nextTick()
      input.setValue('')
      input.trigger('input')
      input.trigger('keydown.enter')
      await Vue.nextTick()
      expect(toastOutput.length).toEqual(0)
      jest.clearAllMocks()
    })

    test('should display a failure toast after selecting' +
      'input value when input is shorter than the minimumValueLength prop', async () => {
      const minLength = 3
      const selectedValue = 't'
      const failureMessage = `Selected value "${selectedValue}" is invalid!\n` +
        `Values must be at least ${minLength} characters long.`
      const failureMessageClass = 'is-danger'
      const toastOutput: Options[] = []
      jest.spyOn(bulmaToast, 'toast').mockImplementation((output: Options) => {
        toastOutput.push(output)
      })
      wrapper.setProps({minimumValueLength: minLength})
      await Vue.nextTick()
      input.setValue(selectedValue)
      input.trigger('input')
      input.trigger('keydown.enter')
      await Vue.nextTick()
      expect(toastOutput[0].message).toContain(failureMessage)
      expect(toastOutput[0].type).toContain(failureMessageClass)
      jest.clearAllMocks()
    })

    test('should emit selected value when space is pressed', async () => {
      wrapper.setData({focusedSuggestionIndex: 2})
      input.trigger('keydown.space')
      expect(wrapper.emitted('value-submitted')).toBeTruthy()
      expect(wrapper.emitted('value-submitted')[0]).toEqual(['test-value-2'])
    })

    test('should emit filtered value on space pressed', async () => {
      wrapper.setProps({
        filterFunction: (tag: string) => tag.replace(/[^\w: ]|_/g, '-').toLowerCase(),
      })
      await Vue.nextTick();
      (input.element as HTMLInputElement).value = 'test:CHECK-CASE_01'
      input.trigger('input')
      await Vue.nextTick()
      input.trigger('keydown.space')
      expect(wrapper.emitted('value-submitted')).toBeTruthy()
      expect(wrapper.emitted('value-submitted')[0]).toEqual(['test:check-case-01'])
    })

    test('should select suggestion when clicked', async () => {
      dropdownItems.at(1).trigger('mousedown')
      await Vue.nextTick()
      expect((input.element as HTMLInputElement).value).toEqual('test-value-1')
    })

    test('should emit selected value when clicked', async () => {
      dropdownItems.at(1).trigger('mousedown')
      expect(wrapper.emitted('value-submitted')).toBeTruthy()
      expect(wrapper.emitted('value-submitted')[0]).toEqual(['test-value-1'])
    })

    test('should have dropdown hidden when esc is pressed', async () => {
      input.trigger('keydown.esc')
      await Vue.nextTick()
      expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeFalsy()
    })
  })

  describe('multiple values selection', () => {
    let input: Wrapper<Vue>
    beforeEach(async () => {
      wrapper = mount(AutocompleteInput, {
        propsData: {
          suggestions: suggestions,
          autoFocus: true,
          selectionType: 'multiple',
        },
      })
      input = wrapper.find('.autocomplete-input')
      input.setValue('devops value')
      input.trigger('input')
    })

    test('should filter suggestion based on last word in input', async () => {
      const dropdownItems = wrapper.findAll('.dropdown-item')
      expect(dropdownItems.length).toEqual(3)
      expect(dropdownItems.at(0).text()).toContain('another-value')
      expect(dropdownItems.at(1).text()).toContain('test-value-1')
      expect(dropdownItems.at(2).text()).toContain('test-value-2')
    })

    test('should only change last word in input when selecting value with enter', async () => {
      wrapper.setData({focusedSuggestionIndex: 2})
      input.trigger('keydown.enter')
      await Vue.nextTick()
      expect((input.element as HTMLInputElement).value).toEqual('devops test-value-2')
    })

    test('should only change last word in input when selecting value with space', async () => {
      wrapper.setData({focusedSuggestionIndex: 2})
      input.trigger('keydown.space')
      await Vue.nextTick()
      expect((input.element as HTMLInputElement).value).toEqual('devops test-value-2')
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

  describe('AutocompleteInput.vue Type=Textarea', () => {
    let initialValue: string
    let initialValueDisplayed: string
    beforeEach(() => {
      initialValue = ' 1002\n1003\n 1004 '
      initialValueDisplayed = '• 1002\n• 1003\n• 1004'
      suggestions = [
        {
          value: '1000',
        },
        {
          value: '1001',
        },
        {
          value: 'test-value-1',
        },
        {
          value: 'test-value-2',
        },
        {
          value: 'united-states',
        },
      ]
      wrapper = mount(AutocompleteInput, {
        propsData: {
          suggestions,
          initialValue,
          inputType: 'textarea',
        },
      })
    })

    test('should show value splitted into lines decorated with discs', async () => {
      expect((wrapper.vm as any).autocompleteValue).toEqual(initialValue)
      expect((wrapper.find('textarea') as any).element.value).toEqual(initialValueDisplayed)
      wrapper.setProps({initialValue: ' first \n second\n third '})
      await Vue.nextTick()
      expect((wrapper.find('textarea') as any).element.value).toEqual('• first\n• second\n• third')
    })

    test('should add a new line after selected value when suggestion clicked', async () => {
      const textarea = wrapper.find('.autocomplete-input')
      textarea.setValue('value')
      textarea.trigger('input')
      await Vue.nextTick()
      const dropdownItems = wrapper.findAll('.dropdown-item')
      dropdownItems.at(0).trigger('mousedown')
      await Vue.nextTick()
      const wantedValue = 'test-value-1'
      expect((wrapper.vm as any).autocompleteValue).toEqual(`${wantedValue}\n`)
    })

    test('should move cursor to end when textarea clicked', async () => {
      const textarea = wrapper.find('.autocomplete-input')
      textarea.trigger('mousedown')
      await Vue.nextTick()
      expect((wrapper.vm as any).$refs.autocompleteInput.selectionStart).toEqual(initialValueDisplayed.length)
    })

    test('should not add more than 1 new line when enter pressed', async () => {
      const textarea = wrapper.find('.autocomplete-input')
      textarea.trigger('keydown.enter')
      await Vue.nextTick()
      expect(wrapper.emitted('value-submitted')).toBeTruthy()
      const event = {key: 'Enter', preventDefault: jest.fn()}
      const spy = jest.spyOn(event, 'preventDefault')
      textarea.setValue('1000\n')
      textarea.trigger('keydown.enter', event)
      await Vue.nextTick()
      expect(spy).toHaveBeenCalled()
      textarea.setValue('')
      textarea.trigger('keydown.enter', event)
      await Vue.nextTick()
      expect(spy).toHaveBeenCalled()
    })

    test('should add new line on focus', async () => {
      const textarea = wrapper.find('.autocomplete-input')
      textarea.trigger('focus')
      await Vue.nextTick()
      let {autocompleteValue} = wrapper.vm as any
      expect(autocompleteValue[autocompleteValue.length - 1]).toEqual('\n')
      textarea.trigger('blur')
      textarea.setValue('')
      textarea.trigger('focus')
      await Vue.nextTick()
      autocompleteValue = (wrapper.vm as any).autocompleteValue
      expect(autocompleteValue[autocompleteValue.length - 1]).not.toEqual('\n')
    })

    test('should delete whole last line when delete pressed', async () => {
      const textarea = wrapper.find('.autocomplete-input')
      textarea.trigger('keydown.delete')
      await Vue.nextTick()
      expect(wrapper.emitted('value-submitted')).toBeTruthy()
      expect(wrapper.emitted('value-submitted')[0]).toEqual(['1002\n1003'])
    })
  })
})
