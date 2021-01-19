import RateLimitsEditor from '@/doc-editors/RateLimitsEditor'
import {beforeEach, describe, expect, test} from '@jest/globals'
import {mount, shallowMount} from '@vue/test-utils'
import LimitOption from '@/components/LimitOption'
import ResponseAction from '@/components/ResponseAction'
import Vue from 'vue'

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

        test('should have count-by limit option component with correct data', () => {
            const wantedType = Object.keys(docs[0].key[0])[0]
            const wantedValue = Object.values(docs[0].key[0])[0]
            const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(0)
            const actualType = limitOptionComponent.vm.option.type
            const actualValue = limitOptionComponent.vm.option.key
            expect(actualType).toEqual(wantedType)
            expect(actualValue).toEqual(wantedValue)
        })

        test('should have event limit option component with correct data', () => {
            const wantedType = Object.keys(docs[0].pairwith)[0]
            const wantedValue = Object.values(docs[0].pairwith)[0]
            const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(1)
            const actualType = limitOptionComponent.vm.option.type
            const actualValue = limitOptionComponent.vm.option.key
            expect(actualType).toEqual(wantedType)
            expect(actualValue).toEqual(wantedValue)
        })

        test('should have count-by limit option component with correct ignored attributes', () => {
            const wantedIgnoredAttributes = ['tags']
            const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(0)
            const actualIgnoredAttributes = limitOptionComponent.vm.ignoreAttributes
            expect(wantedIgnoredAttributes).toEqual(actualIgnoredAttributes)
        })

        test('should have event limit option component with correct ignored attributes', () => {
            const wantedIgnoredAttributes = ['tags']
            const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(0)
            const actualIgnoredAttributes = limitOptionComponent.vm.ignoreAttributes
            expect(wantedIgnoredAttributes).toEqual(actualIgnoredAttributes)
        })

        test('should have response action component with correct data', () => {
            const ResponseActionComponent = wrapper.findComponent(ResponseAction)
            expect(ResponseActionComponent.vm.objectWithAction.action).toEqual(docs[0].action)
        })

        test('should have correct include data in table', () => {
            const includeOption1 = wrapper.findAllComponents(LimitOption).at(2)
            const wantedOption1Key = Object.keys(docs[0].include.attrs)[0]
            const wantedOption1Value = Object.values(docs[0].include.attrs)[0]
            expect(includeOption1.props('option')).toEqual({
                'key': wantedOption1Key,
                'type': 'attrs',
                'value': wantedOption1Value
            })
            const includeOption2 = wrapper.findAllComponents(LimitOption).at(3)
            const wantedOption2Key = Object.keys(docs[0].include.attrs)[1]
            const wantedOption2Value = Object.values(docs[0].include.attrs)[1]
            expect(includeOption2.props('option')).toEqual({
                'key': wantedOption2Key,
                'type': 'attrs',
                'value': wantedOption2Value
            })
        })

        test('should have correct exclude data in table', () => {
            const excludeOption1 = wrapper.findAllComponents(LimitOption).at(4)
            const wantedOption1Key = Object.keys(docs[0].exclude.args)[0]
            const wantedOption1Value = Object.values(docs[0].exclude.args)[0]
            expect(excludeOption1.props('option')).toEqual({
                'key': wantedOption1Key,
                'type': 'args',
                'value': wantedOption1Value
            })
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
            wrapper = mount(RateLimitsEditor, {
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

        test('should handle selectedDoc with undefined key value', async (done) => {
            try {
                docs[0].key = [{'headers': null}, undefined]
                wrapper = mount(RateLimitsEditor, {
                    propsData: {
                        selectedDoc: docs[0],
                    }
                })
                await Vue.nextTick()
                done()
            } catch (err) {
                expect(err).not.toBeDefined()
                done()
            }
        })
    })

    describe('event', () => {
        test('should handle key with no value', async () => {
            docs[0].pairwith = {'self': null}
            wrapper = mount(RateLimitsEditor, {
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

        test('should handle selectedDoc without pairwith property', async (done) => {
            try {
                delete docs[0].pairwith
                wrapper = mount(RateLimitsEditor, {
                    propsData: {
                        selectedDoc: docs[0],
                    }
                })
                await Vue.nextTick()
                done()
            } catch (err) {
                expect(err).not.toBeDefined()
                done()
            }
        })
    })

    describe.skip('include / exclude', () => {
        const addEntry = async (includeExcludeIndex, typeIndex, keyIndex, value) => {
            // open new entry row
            const newButton = wrapper.find('.new-include-exclude-button')
            newButton.trigger('click')
            await Vue.nextTick()
            // find new entry row
            const newEntryRow = wrapper.find('.new-include-exclude-row')
            // select include or exclude
            const includeExcludeSelect = newEntryRow.find('.include-exclude-select')
            includeExcludeSelect.findAll('option').at(includeExcludeIndex).setSelected()
            await Vue.nextTick()
            // select type
            const typeSelect = newEntryRow.find('.type-select')
            typeSelect.findAll('option').at(typeIndex).setSelected()
            await Vue.nextTick()
            // select key
            const keySelect = newEntryRow.find('.key-select')
            keySelect.findAll('option').at(keyIndex).setSelected()
            await Vue.nextTick()
            // insert value input
            const valueInput = newEntryRow.find('.value-input')
            valueInput.element.value = value
            valueInput.trigger('input')
            await Vue.nextTick()
            // click add button
            const addButton = newEntryRow.find('.add-button')
            addButton.trigger('click')
            await Vue.nextTick()
        }
        beforeEach(() => {
            const emptyIncludeExclude = {headers: {}, cookies: {}, args: {}, attrs: {}}
            docs[0].include = emptyIncludeExclude
            docs[0].exclude = emptyIncludeExclude
            wrapper = shallowMount(RateLimitsEditor, {
                propsData: {
                    selectedDoc: docs[0]
                }
            })
        })

        test('should handle adding include entry to doc with no include property', async (done) => {
            try {
                delete docs[0].include
                wrapper = mount(RateLimitsEditor, {
                    propsData: {
                        selectedDoc: docs[0],
                    }
                })
                await Vue.nextTick()
                await addEntry(0, 0, 0, '10.0.0.1')
                // ignore first 2 limit options, 0 is count by, 1 is event
                const includeItem = wrapper.findAllComponents(LimitOption).at(2)
                expect(includeItem.vm.label).toEqual('Include')
                expect(includeItem.vm.option.type).toEqual('attrs')
                expect(includeItem.vm.option.key).toEqual('ip')
                expect(includeItem.vm.option.value).toEqual('10.0.0.1')
                done()
            } catch (err) {
                expect(err).not.toBeDefined()
                done()
            }
        })

        test('should handle adding exclude entry to doc with no exclude property', async (done) => {
            try {
                delete docs[0].exclude
                wrapper = mount(RateLimitsEditor, {
                    propsData: {
                        selectedDoc: docs[0],
                    }
                })
                await Vue.nextTick()
                await addEntry(1, 0, 0, '10.0.0.1')
                // ignore first 2 limit options, 0 is count by, 1 is event
                const includeItem = wrapper.findAllComponents(LimitOption).at(2)
                expect(includeItem.vm.label).toEqual('Exclude')
                expect(includeItem.vm.option.type).toEqual('attrs')
                expect(includeItem.vm.option.key).toEqual('ip')
                expect(includeItem.vm.option.value).toEqual('10.0.0.1')
                done()
            } catch (err) {
                expect(err).not.toBeDefined()
                done()
            }
        })

        test('should have correct message when no entries exist', async () => {
            const noEntriesMessage = wrapper.find('.no-include-exclude-message')
            expect(noEntriesMessage.text()).toEqual('To limit this rule coverage add new entry')
        })

        test('should have correct text in new entry button', async () => {
            const newButton = wrapper.find('.new-include-exclude-button')
            expect(newButton.text()).toEqual('New entry')
            newButton.trigger('click')
            await Vue.nextTick()
            expect(newButton.text()).toEqual('Cancel')
            newButton.trigger('click')
            await Vue.nextTick()
            expect(newButton.text()).toEqual('New entry')
        })

        test('should show new entry row when new entry button is clicked', async () => {
            const newButton = wrapper.find('.new-include-exclude-button')
            newButton.trigger('click')
            await Vue.nextTick()
            const newEntryRow = wrapper.find('.new-include-exclude-row')
            expect(newEntryRow.element).toBeDefined()
        })

        test('should change include entry when selectedDoc changes', async () => {
            const newDoc = {...docs[0]}
            newDoc.include = {headers: {}, cookies: {}, args: {}, attrs: {ip: '10.0.0.1'}}
            wrapper.setProps({selectedDoc: newDoc})
            await Vue.nextTick()
            // ignore first 2 limit options, 0 is count by, 1 is event
            const includeItem = wrapper.findAllComponents(LimitOption).at(2)
            expect(includeItem.vm.label).toEqual('Include')
            expect(includeItem.vm.option.type).toEqual('attrs')
            expect(includeItem.vm.option.key).toEqual('ip')
            expect(includeItem.vm.option.value).toEqual('10.0.0.1')
        })

        test('should change exclude entry when selectedDoc changes', async () => {
            const newDoc = {...docs[0]}
            newDoc.exclude = {headers: {}, cookies: {}, args: {}, attrs: {ip: '10.0.0.1'}}
            wrapper.setProps({selectedDoc: newDoc})
            await Vue.nextTick()
            // ignore first 2 limit options, 0 is count by, 1 is event
            const includeItem = wrapper.findAllComponents(LimitOption).at(2)
            expect(includeItem.vm.label).toEqual('Exclude')
            expect(includeItem.vm.option.type).toEqual('attrs')
            expect(includeItem.vm.option.key).toEqual('ip')
            expect(includeItem.vm.option.value).toEqual('10.0.0.1')
        })

        test('should be able to add include entry', async () => {
            await addEntry(0, 0, 0, '10.0.0.1')
            // ignore first 2 limit options, 0 is count by, 1 is event
            const includeItem = wrapper.findAllComponents(LimitOption).at(2)
            expect(includeItem.vm.label).toEqual('Include')
            expect(includeItem.vm.option.type).toEqual('attrs')
            expect(includeItem.vm.option.key).toEqual('ip')
            expect(includeItem.vm.option.value).toEqual('10.0.0.1')
        })

        test('should be able to add exclude entry', async () => {
            await addEntry(1, 0, 0, '10.0.0.1')
            // ignore first 2 limit options, 0 is count by, 1 is event
            const excludeItem = wrapper.findAllComponents(LimitOption).at(2)
            expect(excludeItem.vm.label).toEqual('Exclude')
            expect(excludeItem.vm.option.type).toEqual('attrs')
            expect(excludeItem.vm.option.key).toEqual('ip')
            expect(excludeItem.vm.option.value).toEqual('10.0.0.1')
        })

        test('should be able to remove include entry', async () => {
            await addEntry(0, 0, 0, '10.0.0.1')
            // ignore first 2 limit options, 0 is count by, 1 is event
            const includeItem = wrapper.findAllComponents(LimitOption).at(2)
            includeItem.vm.$emit('remove')
            await Vue.nextTick()
            expect(wrapper.findAllComponents(LimitOption).length).toEqual(2)
        })

        test('should be able to remove exclude entry', async () => {
            await addEntry(1, 0, 0, '10.0.0.1')
            // ignore first 2 limit options, 0 is count by, 1 is event
            const excludeItem = wrapper.findAllComponents(LimitOption).at(2)
            excludeItem.vm.$emit('remove')
            await Vue.nextTick()
            expect(wrapper.findAllComponents(LimitOption).length).toEqual(2)
        })

        test('should show duplicated include message when duplicated include entries exist', async () => {
            await addEntry(0, 0, 0, '10.0.0.1')
            await addEntry(0, 0, 0, '10.0.0.1')
            const invalidMessage = wrapper.find('.include-invalid')
            expect(invalidMessage.text()).toEqual('Include rule keys must be unique')
        })

        test('should show duplicated exclude message when duplicated exclude entries exist', async () => {
            await addEntry(1, 0, 0, '10.0.0.1')
            await addEntry(1, 0, 0, '10.0.0.1')
            const invalidMessage = wrapper.find('.exclude-invalid')
            expect(invalidMessage.text()).toEqual('Exclude rule keys must be unique')
        })

        test('should not show duplicated include message when duplicated entries issue resolved', async () => {
            await addEntry(0, 0, 0, '10.0.0.1')
            await addEntry(0, 0, 0, '10.0.0.1')
            // ignore first 2 limit options, 0 is count by, 1 is event
            const includeItem = wrapper.findAllComponents(LimitOption).at(2)
            includeItem.vm.$emit('change', {key: 'asn', oldKey: 'ip', type: 'attrs', value: '10.0.0.1'}, 0)
            await Vue.nextTick()
            const invalidMessage = wrapper.find('.include-invalid')
            expect(invalidMessage.element).not.toBeDefined()
        })

        test('should not show duplicated exclude message when duplicated entries issue resolved', async () => {
            await addEntry(1, 0, 0, '10.0.0.1')
            await addEntry(1, 0, 0, '10.0.0.1')
            // ignore first 2 limit options, 0 is count by, 1 is event
            const excludeItem = wrapper.findAllComponents(LimitOption).at(2)
            excludeItem.vm.$emit('change', {key: 'asn', oldKey: 'ip', type: 'attrs', value: '10.0.0.1'}, 0)
            await Vue.nextTick()
            const invalidMessage = wrapper.find('.exclude-invalid')
            expect(invalidMessage.element).not.toBeDefined()
        })
    })
})
