import AutocompleteInput from '@/components/AutocompleteInput'
import {beforeEach, describe, expect, jest, test} from '@jest/globals'
import {mount} from '@vue/test-utils'
import Vue from 'vue'
import axios from 'axios'

jest.mock('axios')

describe('AutocompleteInput.vue', () => {
    let wrapper
    let suggestions
    beforeEach(() => {
        suggestions = [
            {
                prefix: '<span>prefix html</span>',
                value: 'another-value'
            },
            {
                value: 'devops'
            },
            {
                value: 'internal'
            },
            {
                prefix: 'prefix string',
                value: 'test-value-1'
            },
            {
                value: 'test-value-2'
            },
            {
                value: 'united-states'
            }
        ]
        wrapper = mount(AutocompleteInput, {
            propsData: {
                suggestions: suggestions,
                autoFocus: true,
                clearInputAfterSelection: false
            }
        })
    })

    test('should have dropdown hidden on init', () => {
        expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeFalsy()
    })

    test('should have dropdown hidden after typing in input if empty', async () => {
        axios.get.mockImplementation(() => Promise.resolve({data: {}}))
        wrapper = mount(AutocompleteInput)
        const input = wrapper.find('.autocomplete-input')
        input.element.value = 'value'
        input.trigger('input')
        await Vue.nextTick()
        expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeFalsy()
    })

    test('should have dropdown displayed after typing in input', async () => {
        const input = wrapper.find('.autocomplete-input')
        input.element.value = 'value'
        input.trigger('input')
        await Vue.nextTick()
        expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeTruthy()
    })

    test('should emit changed value when input changes', async () => {
        const input = wrapper.find('.autocomplete-input')
        input.element.value = 'value'
        input.trigger('input')
        await Vue.nextTick()
        expect(wrapper.emitted('value-changed')).toBeTruthy()
        expect(wrapper.emitted('value-changed')[0]).toEqual(['value'])
    })

    test('should show correct filtered values in dropdown ordered alphabetically', async () => {
        const input = wrapper.find('.autocomplete-input')
        input.element.value = 'value'
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
        input.element.value = 'value'
        input.trigger('input')
        await Vue.nextTick()
        const dropdownItems = wrapper.findAll('.dropdown-item')
        expect(dropdownItems.at(0).html()).toContain('<span>prefix html</span>')
        expect(dropdownItems.at(1).html()).toContain('prefix string')
    })

    test('should show correct filtered values in dropdown ordered alphabetically regardless of casing', async () => {
        const input = wrapper.find('.autocomplete-input')
        input.element.value = 'value'
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
        input.element.value = 'value'
        input.trigger('input')
        wrapper.setProps({initialValue: newValue})
        await Vue.nextTick()
        expect(input.element.value).toEqual(newValue)
    })

    test('should have dropdown hidden when prop changes', async () => {
        const newValue = 'another'
        const input = wrapper.find('.autocomplete-input')
        input.element.value = 'value'
        input.trigger('input')
        wrapper.setProps({initialValue: newValue})
        await Vue.nextTick()
        expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeFalsy()
    })

    test('should have dropdown hidden when prop changes to the same value', async () => {
        const value = 'value'
        const input = wrapper.find('.autocomplete-input')
        input.element.value = value
        input.trigger('input')
        wrapper.setProps({initialValue: value})
        await Vue.nextTick()
        expect(wrapper.find('.dropdown').element.classList.contains('is-active')).toBeTruthy()
    })

    test('should clear autocomplete input when selected', async () => {
        wrapper.setProps({clearInputAfterSelection: true})
        const input = wrapper.find('.autocomplete-input')
        input.element.value = 'value'
        input.trigger('input')
        wrapper.setData({focusedSuggestionIndex: 2})
        input.trigger('keydown.enter')
        await Vue.nextTick()
        expect(input.element.value).toEqual('')
    })

    describe('keyboard control', () => {
        let input
        let dropdownItems
        beforeEach(async () => {
            input = wrapper.find('.autocomplete-input')
            input.element.value = 'value'
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
            expect(input.element.value).toEqual('test-value-2')
        })

        test('should select input value when enter is pressed and there is no focused suggestion', async () => {
            wrapper.setData({focusedSuggestionIndex: -1})
            input.element.value = 'test-value-1'
            input.trigger('input')
            input.trigger('keydown.enter')
            await Vue.nextTick()
            expect(input.element.value).toEqual('test-value-1')
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
            expect(input.element.value).toEqual('test-value-2')
        })

        test('should select input value when space is pressed and there is no focused suggestion', async () => {
            wrapper.setData({focusedSuggestionIndex: -1})
            input.element.value = 'test-value-1'
            input.trigger('input')
            input.trigger('keydown.space')
            await Vue.nextTick()
            expect(input.element.value).toEqual('test-value-1')
        })

        test('should emit selected value when space is pressed', async () => {
            wrapper.setData({focusedSuggestionIndex: 2})
            input.trigger('keydown.space')
            expect(wrapper.emitted('value-submitted')).toBeTruthy()
            expect(wrapper.emitted('value-submitted')[0]).toEqual(['test-value-2'])
        })

        test('should select suggestion when clicked', async () => {
            dropdownItems.at(1).trigger('click')
            await Vue.nextTick()
            expect(input.element.value).toEqual('test-value-1')
        })

        test('should emit selected value when clicked', async () => {
            dropdownItems.at(1).trigger('click')
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
        let input
        beforeEach(async () => {
            wrapper = mount(AutocompleteInput, {
                propsData: {
                    suggestions: suggestions,
                    autoFocus: true,
                    selectionType: 'multiple'
                }
            })
            input = wrapper.find('.autocomplete-input')
            input.element.value = 'devops value'
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
            expect(input.element.value).toEqual('devops test-value-2')
        })

        test('should only change last word in input when selecting value with space', async () => {
            wrapper.setData({focusedSuggestionIndex: 2})
            input.trigger('keydown.space')
            await Vue.nextTick()
            expect(input.element.value).toEqual('devops test-value-2')
        })
    })

    describe('selection type prop validator', () => {
        let validator
        beforeEach(() => {
            validator = AutocompleteInput.props.selectionType.validator
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
            const type = undefined
            const isValid = validator(type)
            expect(isValid).toEqual(false)
        })
    })
})
