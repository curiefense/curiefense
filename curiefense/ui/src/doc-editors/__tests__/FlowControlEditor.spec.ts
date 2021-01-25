import FlowControlEditor from '@/doc-editors/FlowControlEditor'
import {beforeEach, describe, expect, test, jest} from '@jest/globals'
import {mount} from '@vue/test-utils'
import LimitOption from '@/components/LimitOption'
import ResponseAction from '@/components/ResponseAction'
import Vue from 'vue'
import TagAutocompleteInput from '@/components/TagAutocompleteInput'

jest.mock('axios')
import axios from 'axios'

describe('FlowControlEditor.vue', () => {
    let docs
    let wrapper
    beforeEach(() => {
        docs = [
            {
                'exclude': ['devops', 'internal'],
                'include': ['china'],
                'name': 'flow control',
                'key': [
                    {
                        'attrs': 'ip'
                    }
                ],
                'sequence': [
                    {
                        'method': 'GET',
                        'uri': '/login',
                        'cookies': {
                            'foo': 'bar'
                        },
                        'headers': {
                            'host': 'www.example.com'
                        },
                        'args': {}
                    },
                    {
                        'method': 'POST',
                        'uri': '/login',
                        'cookies': {
                            'foo': 'bar'
                        },
                        'headers': {
                            'host': 'www.example.com',
                            'test': 'one'
                        },
                        'args': {}
                    }
                ],
                'active': true,
                'notes': 'New Flow Control Notes and Remarks',
                'action': {
                    'type': 'default',
                    'params': {}
                },
                'ttl': 60,
                'id': 'c03dabe4b9ca'
            }
        ]
        const onUpdate = (doc) => {
            wrapper.setProps({selectedDoc: doc})
        }
        wrapper = mount(FlowControlEditor, {
            propsData: {
                selectedDoc: docs[0]
            },
            listeners: {
                update: onUpdate
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

        test('should have correct active mode', () => {
            expect(wrapper.find('.document-active').element.checked).toEqual(docs[0].active)
        })

        test('should have correct ttl in input', () => {
            expect(wrapper.find('.document-ttl').element.value).toEqual(docs[0].ttl.toString())
        })

        test('should have limit option component with correct data', () => {
            const wantedType = Object.keys(docs[0].key[0])[0]
            const wantedValue = Object.values(docs[0].key[0])[0]
            const limitOptionComponent = wrapper.findAllComponents(LimitOption).at(0)
            const actualType = limitOptionComponent.vm.option.type
            const actualValue = limitOptionComponent.vm.option.key
            expect(actualType).toEqual(wantedType)
            expect(actualValue).toEqual(wantedValue)
        })

        test('should have response action component with correct data', () => {
            const ResponseActionComponent = wrapper.findComponent(ResponseAction)
            expect(ResponseActionComponent.vm.objectWithAction.action).toEqual(docs[0].action)
        })

        test('should have correct notes in input', () => {
            expect(wrapper.find('.document-notes').element.value).toEqual(docs[0].notes.toString())
        })

        test('should have correct tags in include filter', () => {
            const includeFilterColumn = wrapper.find('.include-filter-column')
            const tagCells = includeFilterColumn.findAll('.tag-cell')
            expect(tagCells.at(0).text()).toEqual(docs[0].include[0])
            expect(tagCells.length).toEqual(1)
        })

        test('should have correct tags in exclude filter', () => {
            const excludeFilterColumn = wrapper.find('.exclude-filter-column')
            const tagCells = excludeFilterColumn.findAll('.tag-cell')
            expect(tagCells.at(0).text()).toEqual(docs[0].exclude[0])
            expect(tagCells.at(1).text()).toEqual(docs[0].exclude[1])
            expect(tagCells.length).toEqual(2)
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
                        'internal'
                    ],
                },
            }
            axios.get.mockImplementation((path) => {
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

        test('should show a warning when there are duplicate tags', async () => {
            const newTag = 'test-tag'
            const newIncludeEntryButton = wrapper.findAll('.add-new-filter-entry-button').at(0)
            // add first
            newIncludeEntryButton.trigger('click')
            await Vue.nextTick()
            const firstTagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
            firstTagAutocompleteInput.vm.$emit('tag-submitted', newTag)
            await Vue.nextTick()
            // add second
            newIncludeEntryButton.trigger('click')
            await Vue.nextTick()
            const secondTagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
            secondTagAutocompleteInput.vm.$emit('tag-submitted', newTag)
            await Vue.nextTick()
            // check
            const tagsWithWarning = wrapper.findAll('.has-text-danger')
            expect(tagsWithWarning.length).toEqual(2)
        })

        test('should add tag to correct filter when tag selected - include filter', async () => {
            const newIncludeEntryButton = wrapper.findAll('.add-new-filter-entry-button').at(0)
            newIncludeEntryButton.trigger('click')
            await Vue.nextTick()
            const newTag = 'test-tag'
            const tagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
            tagAutocompleteInput.vm.$emit('tag-submitted', newTag)
            await Vue.nextTick()
            const includeFilterColumn = wrapper.find('.include-filter-column')
            const tagCells = includeFilterColumn.findAll('.tag-cell')
            expect(tagCells.length).toEqual(2)
            expect(tagCells.at(1).text()).toContain(newTag)
            expect(wrapper.vm.selectedDoc.include.includes(newTag)).toBeTruthy()
        })

        test('should add tag to correct filter when tag selected - exclude filter', async () => {
            const newExcludeEntryButton = wrapper.findAll('.add-new-filter-entry-button').at(1)
            newExcludeEntryButton.trigger('click')
            await Vue.nextTick()
            const newTag = 'test-tag'
            const tagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
            tagAutocompleteInput.vm.$emit('tag-submitted', newTag)
            await Vue.nextTick()
            const excludeFilterColumn = wrapper.find('.exclude-filter-column')
            const tagCells = excludeFilterColumn.findAll('.tag-cell')
            expect(tagCells.length).toEqual(3)
            expect(tagCells.at(2).text()).toContain(newTag)
            expect(wrapper.vm.selectedDoc.exclude.includes(newTag)).toBeTruthy()
        })

        test('should not add tag when tag selected is 2 or less characters long', async () => {
            const newIncludeEntryButton = wrapper.find('.add-new-filter-entry-button')
            newIncludeEntryButton.trigger('click')
            await Vue.nextTick()
            const newTag = 't'
            const tagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
            tagAutocompleteInput.vm.$emit('tag-submitted', newTag)
            await Vue.nextTick()
            expect(wrapper.vm.selectedDoc.include.includes(newTag)).toBeFalsy()
        })

        test('should remove tag from correct filter when tag removed', async () => {
            const removeIncludeEntryButton = wrapper.find('.remove-filter-entry-button')
            removeIncludeEntryButton.trigger('click')
            await Vue.nextTick()
            expect(wrapper.vm.selectedDoc.include.length).toEqual(0)
        })

        test('should hide tag input when tag selection cancelled', async () => {
            const newIncludeEntryButton = wrapper.find('.add-new-filter-entry-button')
            newIncludeEntryButton.trigger('click')
            await Vue.nextTick()
            wrapper.vm.cancelAddNewTag()
            await Vue.nextTick()
            const tagAutocompleteInput = wrapper.findComponent(TagAutocompleteInput)
            await Vue.nextTick()
            expect(tagAutocompleteInput.element).toBeUndefined()
        })
    })

    describe('sequence list', () => {
        describe('add section button', () => {
            test('should add section', async () => {
                const addSectionButton = wrapper.find('.new-sequence-button')
                addSectionButton.trigger('click')
                await Vue.nextTick()
                const tables = wrapper.findAll('.sequence-entries-table')
                expect(tables.length).toEqual(3)
            })

            test('should add section with default data', async () => {
                // empty sections list
                let removeSectionButton = wrapper.find('.remove-section-button')
                removeSectionButton.trigger('click')
                await Vue.nextTick()
                removeSectionButton = wrapper.find('.remove-section-button')
                removeSectionButton.trigger('click')
                await Vue.nextTick()
                // create new section
                const addSectionButton = wrapper.find('.new-sequence-button')
                addSectionButton.trigger('click')
                await Vue.nextTick()
                // check data
                const tables = wrapper.findAll('.sequence-entries-table')
                const methodEntryInput = wrapper.find('.method-entry-input')
                const uriEntryInput = wrapper.find('.uri-entry-input')
                const hostEntryInput = wrapper.find('.host-entry-input')
                expect(tables.length).toEqual(1)
                expect(methodEntryInput.element.value).toContain('GET')
                expect(uriEntryInput.element.value).toContain('/')
                expect(hostEntryInput.element.value).toContain('www.example.com')
            })

            test('should get header host value from first section', async () => {
                const wantedHostValue = 'api.example.com'
                const hostEntryInput = wrapper.findAll('.host-entry-input').at(0)
                hostEntryInput.element.value = wantedHostValue
                hostEntryInput.trigger('input')
                await Vue.nextTick()
                const addSectionButton = wrapper.find('.new-sequence-button')
                addSectionButton.trigger('click')
                await Vue.nextTick()
                const newHostEntryInput = wrapper.findAll('.host-entry-input').at(2)
                expect(newHostEntryInput.element.value).toContain(wantedHostValue)
            })
        })

        describe('remove section button', () => {
            test('should remove section', async () => {
                const removeSectionButton = wrapper.find('.remove-section-button')
                removeSectionButton.trigger('click')
                await Vue.nextTick()
                const tables = wrapper.findAll('.sequence-entries-table')
                expect(tables.length).toEqual(1)
            })

            test('should not have the option to remove section if no sections exist', async () => {
                // remove all sections
                while (wrapper.vm.selectedDoc.sequence.length > 0) {
                    const removeSectionButton = wrapper.find('.remove-section-button')
                    removeSectionButton.trigger('click')
                    await Vue.nextTick()
                }
                // check
                const removeSectionButton = wrapper.find('.remove-section-button')
                expect(removeSectionButton.element).toBeUndefined()
            })
        })

        describe('add entry button', () => {
            test('should open new entry row', async () => {
                const table = wrapper.findAll('.sequence-entries-table').at(0)
                const addEntryButton = table.find('.add-entry-button')
                addEntryButton.trigger('click')
                await Vue.nextTick()
                const newEntryRow = table.find('.new-entry-row')
                expect(newEntryRow.element).toBeDefined()
            })

            test('should add new entries from input when confirm button is clicked', async () => {
                const table = wrapper.findAll('.sequence-entries-table').at(0)
                const addEntryButton = table.find('.add-entry-button')
                addEntryButton.trigger('click')
                await Vue.nextTick()
                const newEntryRow = table.find('.new-entry-row')
                const typeSelection = newEntryRow.find('.new-entry-type-selection')
                typeSelection.trigger('click')
                const options = typeSelection.findAll('option')
                options.at(0).element.selected = true
                typeSelection.trigger('change')
                await Vue.nextTick()
                const newEntryName = newEntryRow.find('.new-entry-name-input')
                newEntryName.setValue('something')
                const newEntryValue = newEntryRow.find('.new-entry-value-input')
                newEntryValue.setValue('right')
                const confirmAddEntryButton = table.find('.confirm-add-entry-button')
                confirmAddEntryButton.trigger('click')
                await Vue.nextTick()
                const entriesRows = wrapper.findAll('.sequence-entries-table').at(0).findAll('.sequence-entry-row')
                expect(entriesRows.length).toEqual(5)
                expect(entriesRows.at(3).text()).toContain('Header')
                expect(entriesRows.at(3).text()).toContain('something')
                expect(entriesRows.at(3).text()).toContain('right')
            })

            test('should be able to add header as a new entry', async () => {
                const table = wrapper.findAll('.sequence-entries-table').at(0)
                const addEntryButton = table.find('.add-entry-button')
                addEntryButton.trigger('click')
                await Vue.nextTick()
                const newEntryRow = table.find('.new-entry-row')
                const typeSelection = newEntryRow.find('.new-entry-type-selection')
                typeSelection.trigger('click')
                const options = typeSelection.findAll('option')
                options.at(0).element.selected = true
                typeSelection.trigger('change')
                await Vue.nextTick()
                const newEntryName = newEntryRow.find('.new-entry-name-input')
                newEntryName.setValue('something')
                const newEntryValue = newEntryRow.find('.new-entry-value-input')
                newEntryValue.setValue('right')
                const confirmAddEntryButton = table.find('.confirm-add-entry-button')
                confirmAddEntryButton.trigger('click')
                await Vue.nextTick()
                const entriesRows = wrapper.findAll('.sequence-entries-table').at(0).findAll('.sequence-entry-row')
                expect(entriesRows.length).toEqual(5)
                expect(entriesRows.at(3).text()).toContain('Header')
                expect(entriesRows.at(3).text()).toContain('something')
                expect(entriesRows.at(3).text()).toContain('right')
            })

            test('should be able to add argument as a new entry', async () => {
                const table = wrapper.findAll('.sequence-entries-table').at(0)
                const addEntryButton = table.find('.add-entry-button')
                addEntryButton.trigger('click')
                await Vue.nextTick()
                const newEntryRow = table.find('.new-entry-row')
                const typeSelection = newEntryRow.find('.new-entry-type-selection')
                typeSelection.trigger('click')
                const options = typeSelection.findAll('option')
                options.at(1).element.selected = true
                typeSelection.trigger('change')
                await Vue.nextTick()
                const newEntryName = newEntryRow.find('.new-entry-name-input')
                newEntryName.setValue('something')
                const newEntryValue = newEntryRow.find('.new-entry-value-input')
                newEntryValue.setValue('right')
                const confirmAddEntryButton = table.find('.confirm-add-entry-button')
                confirmAddEntryButton.trigger('click')
                await Vue.nextTick()
                const entriesRows = wrapper.findAll('.sequence-entries-table').at(0).findAll('.sequence-entry-row')
                expect(entriesRows.length).toEqual(5)
                expect(entriesRows.at(3).text()).toContain('Argument')
                expect(entriesRows.at(3).text()).toContain('something')
                expect(entriesRows.at(3).text()).toContain('right')
            })

            test('should be able to add cookie as a new entry', async () => {
                const table = wrapper.findAll('.sequence-entries-table').at(0)
                const addEntryButton = table.find('.add-entry-button')
                addEntryButton.trigger('click')
                await Vue.nextTick()
                const newEntryRow = table.find('.new-entry-row')
                const typeSelection = newEntryRow.find('.new-entry-type-selection')
                typeSelection.trigger('click')
                const options = typeSelection.findAll('option')
                options.at(2).element.selected = true
                typeSelection.trigger('change')
                await Vue.nextTick()
                const newEntryName = newEntryRow.find('.new-entry-name-input')
                newEntryName.setValue('something')
                const newEntryValue = newEntryRow.find('.new-entry-value-input')
                newEntryValue.setValue('right')
                const confirmAddEntryButton = table.find('.confirm-add-entry-button')
                confirmAddEntryButton.trigger('click')
                await Vue.nextTick()
                const entriesRows = wrapper.findAll('.sequence-entries-table').at(0).findAll('.sequence-entry-row')
                expect(entriesRows.length).toEqual(5)
                expect(entriesRows.at(4).text()).toContain('Cookie')
                expect(entriesRows.at(4).text()).toContain('something')
                expect(entriesRows.at(4).text()).toContain('right')
            })

            test('should not add new header if name is host', async () => {
                const table = wrapper.findAll('.sequence-entries-table').at(0)
                const addEntryButton = table.find('.add-entry-button')
                addEntryButton.trigger('click')
                await Vue.nextTick()
                const newEntryRow = table.find('.new-entry-row')
                const typeSelection = newEntryRow.find('.new-entry-type-selection')
                typeSelection.trigger('click')
                const options = typeSelection.findAll('option')
                options.at(0).element.selected = true
                typeSelection.trigger('change')
                await Vue.nextTick()
                const newEntryName = newEntryRow.find('.new-entry-name-input')
                newEntryName.setValue('host')
                const newEntryValue = newEntryRow.find('.new-entry-value-input')
                newEntryValue.setValue('somevalue')
                const confirmAddEntryButton = table.find('.confirm-add-entry-button')
                confirmAddEntryButton.trigger('click')
                await Vue.nextTick()
                const entriesRows = wrapper.findAll('.sequence-entries-table').at(0).findAll('.sequence-entry-row')
                expect(entriesRows.length).toEqual(4)
            })

            test('should not add new entries from input when confirm button is clicked if does not have name', async () => {
                const table = wrapper.findAll('.sequence-entries-table').at(0)
                const addEntryButton = table.find('.add-entry-button')
                addEntryButton.trigger('click')
                await Vue.nextTick()
                const newEntryRow = table.find('.new-entry-row')
                const typeSelection = newEntryRow.find('.new-entry-type-selection')
                typeSelection.trigger('click')
                const options = typeSelection.findAll('option')
                options.at(0).element.selected = true
                typeSelection.trigger('change')
                await Vue.nextTick()
                const newEntryValue = newEntryRow.find('.new-entry-value-input')
                newEntryValue.setValue('right')
                const confirmAddEntryButton = table.find('.confirm-add-entry-button')
                confirmAddEntryButton.trigger('click')
                await Vue.nextTick()
                const entriesRows = wrapper.findAll('.sequence-entries-table').at(0).findAll('.sequence-entry-row')
                expect(entriesRows.length).toEqual(4)
            })

            test('should not add new entries from input when confirm button is clicked if does not have value', async () => {
                const table = wrapper.findAll('.sequence-entries-table').at(0)
                const addEntryButton = table.find('.add-entry-button')
                addEntryButton.trigger('click')
                await Vue.nextTick()
                const newEntryRow = table.find('.new-entry-row')
                const typeSelection = newEntryRow.find('.new-entry-type-selection')
                typeSelection.trigger('click')
                const options = typeSelection.findAll('option')
                options.at(0).element.selected = true
                typeSelection.trigger('change')
                await Vue.nextTick()
                const newEntryName = newEntryRow.find('.new-entry-name-input')
                newEntryName.setValue('something')
                const confirmAddEntryButton = table.find('.confirm-add-entry-button')
                confirmAddEntryButton.trigger('click')
                await Vue.nextTick()
                const entriesRows = wrapper.findAll('.sequence-entries-table').at(0).findAll('.sequence-entry-row')
                expect(entriesRows.length).toEqual(4)
            })
        })

        describe('remove entry button', () => {
            test('should remove entry', async () => {
                const table = wrapper.findAll('.sequence-entries-table').at(0)
                const removeEntryButton = table.find('.remove-entry-button')
                removeEntryButton.trigger('click')
                await Vue.nextTick()
                const entriesRows = wrapper.findAll('.sequence-entries-table').at(0).findAll('.sequence-entry-row')
                expect(entriesRows.length).toEqual(3)
            })
        })
    })
})
