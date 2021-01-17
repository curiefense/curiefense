import RateLimitsEditor from '@/doc-editors/RateLimitsEditor'
import {beforeEach, describe, expect, test} from '@jest/globals'
import {mount, shallowMount} from '@vue/test-utils'
import LimitOption from '@/components/LimitOption'
import ResponseAction from '@/components/ResponseAction'
import Vue from 'vue'
import FlowControlEditor from '@/doc-editors/FlowControlEditor'

describe('RateLimitsEditor.vue', () => {
    let docs
    let wrapper
    beforeEach(() => {
        docs = [{
            'id': 'f971e92459e2',
            'name': 'Rate Limit Example Rule 5/60',
            'description': '5 requests per minute',
            'ttl': '60',
            'limit': '5',
            'action': {'type': 'default', 'params': {'action': {'type': 'default', 'params': {}}}},
            'include': {headers: {}, cookies: {}, args: {}, attrs: {ip: '10.0.0.1', path: 'localhost'}},
            'exclude': {headers: {}, cookies: {}, args: {foo: 'bar'}, attrs: {}},
            'key': [{'attrs': 'ip'}],
            'pairwith': {'self': 'self'}
        }]
        wrapper = shallowMount(RateLimitsEditor, {
            propsData: {
                selectedDoc: docs[0]
            }
        })
    })
    describe('form data', () => {
        test('should have correct ID displayed', () => {
            expect(wrapper.find('.document-id').text()).toEqual(docs[0].id)
        })

        test('should have correct name in input', () => {
            expect(wrapper.find('.document-name').element.value).toEqual(docs[0].name)
        })

        test('should have correct description in input', () => {
            expect(wrapper.find('.document-description').element.value).toEqual(docs[0].description)
        })

        test('should have correct threshold in input', () => {
            expect(wrapper.find('.document-limit').element.value).toEqual(docs[0].limit)
        })

        test('should have correct TTL in input', () => {
            expect(wrapper.find('.document-ttl').element.value).toEqual(docs[0].ttl)
        })

        test('should have limit option component with correct count-by data', () => {
            const wantedType = Object.keys(docs[0].key[0])[0]
            const wantedValue = Object.values(docs[0].key[0])[0]
            const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(0)
            const actualType = limitOptionComponent.vm.option.type
            const actualValue = limitOptionComponent.vm.option.key
            expect(actualType).toEqual(wantedType)
            expect(actualValue).toEqual(wantedValue)
        })

        test('should have limit option component with correct event data', () => {
            const wantedType = Object.keys(docs[0].pairwith)[0]
            const wantedValue = Object.values(docs[0].pairwith)[0]
            const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(1)
            const actualType = limitOptionComponent.vm.option.type
            const actualValue = limitOptionComponent.vm.option.key
            expect(actualType).toEqual(wantedType)
            expect(actualValue).toEqual(wantedValue)
        })

        test('should have response action component with correct data', () => {
            const ResponseActionComponent = wrapper.findComponent(ResponseAction)
            expect(ResponseActionComponent.vm.objectWithAction.action).toEqual(docs[0].action)
        })

        test('should have correct include data in table', () => {
            const includeOption1 = wrapper.findAllComponents(LimitOption).at(2)
            const wantedOption1Key = Object.keys(docs[0].include.attrs)[0]
            const wantedOption1Value = Object.values(docs[0].include.attrs)[0]
            expect(includeOption1.props('option')).toEqual({"key": wantedOption1Key, "type": "attrs", "value": wantedOption1Value})
            const includeOption2 = wrapper.findAllComponents(LimitOption).at(3)
            const wantedOption2Key = Object.keys(docs[0].include.attrs)[1]
            const wantedOption2Value = Object.values(docs[0].include.attrs)[1]
            expect(includeOption2.props('option')).toEqual({"key": wantedOption2Key, "type": "attrs", "value": wantedOption2Value})
        })

        test('should have correct exclude data in table', () => {
            const excludeOption1 = wrapper.findAllComponents(LimitOption).at(4)
            const wantedOption1Key = Object.keys(docs[0].exclude.args)[0]
            const wantedOption1Value = Object.values(docs[0].exclude.args)[0]
            expect(excludeOption1.props('option')).toEqual({"key": wantedOption1Key, "type": "args", "value": wantedOption1Value})
        })
    })

    describe('count by key', () => {
        test('should add key when button is clicked', async () => {
            const addKeyButton = wrapper.find('.add-key-button')
            addKeyButton.trigger('click')
            await Vue.nextTick()
            const wantedType = 'attrs'
            const wantedValue = 'ip'
            const actualType = Object.keys(wrapper.vm.selectedDoc.key[1])[0]
            const actualValue = Object.values(wrapper.vm.selectedDoc.key[1])[0]
            expect(wrapper.vm.selectedDoc.key.length).toEqual(2)
            expect(actualType).toEqual(wantedType)
            expect(actualValue).toEqual(wantedValue)
        })

        test('should handle key with no value', async () => {
            docs[0].key = [{'headers': null}]
            wrapper = mount(FlowControlEditor, {
                propsData: {
                    selectedDoc: docs[0],
                }
            })
            const wantedType = 'headers'
            const wantedValue = null
            const actualType = Object.keys(wrapper.vm.selectedDoc.key[0])[0]
            const actualValue = Object.values(wrapper.vm.selectedDoc.key[0])[0]
            expect(actualType).toEqual(wantedType)
            expect(actualValue).toEqual(wantedValue)
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
            expect(wrapper.vm.selectedDoc.key.length).toEqual(1)
        })

        test('should not be able to remove key when only one key exists', async () => {
            const limitOptionsComponent = wrapper.findComponent(LimitOption)
            limitOptionsComponent.vm.$emit('remove', 1)
            await Vue.nextTick()
            expect(wrapper.vm.selectedDoc.key.length).toEqual(1)
        })

        test('should update key when change event occurs', async () => {
            const newOption = {
                type: 'self',
                key: 'self'
            }
            const wantedResult = {
                self: 'self'
            }
            const limitOptionsComponent = wrapper.findComponent(LimitOption)
            limitOptionsComponent.vm.$emit('change', newOption, 0)
            await Vue.nextTick()
            expect(wrapper.vm.selectedDoc.key[0]).toEqual(wantedResult)
        })
    })

    describe('event', () => {
        test('should handle key with no value', async () => {
            docs[0].pairwith = {'self': null}
            wrapper = mount(FlowControlEditor, {
                propsData: {
                    selectedDoc: docs[0],
                }
            })
            const wantedType = 'self'
            const wantedValue = null
            const actualType = Object.keys(wrapper.vm.selectedDoc.pairwith)[0]
            const actualValue = Object.values(wrapper.vm.selectedDoc.pairwith)[0]
            expect(actualType).toEqual(wantedType)
            expect(actualValue).toEqual(wantedValue)
        })

        test('should update key when change event occurs', async () => {
            const newOption = {
                type: 'self',
                key: 'self'
            }
            const wantedResult = {
                self: 'self'
            }
            const limitOptionsComponent = wrapper.findAllComponents(LimitOption).at(1)
            limitOptionsComponent.vm.$emit('change', newOption, 0)
            await Vue.nextTick()
            expect(wrapper.vm.selectedDoc.pairwith).toEqual(wantedResult)
        })
    })
})
