import EntriesRelationList from '@/components/EntriesRelationList.vue'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {mount, Wrapper, WrapperArray} from '@vue/test-utils'
import _ from 'lodash'
import Vue from 'vue'
import {GlobalFilter, GlobalFilterSection} from '@/types'

describe('EntriesRelationList.vue', () => {
  let wrapper: Wrapper<Vue>
  let ruleData: GlobalFilter['rule']
  let entryData1: GlobalFilterSection
  let entryData2: GlobalFilterSection
  beforeEach(() => {
    entryData1 = {
      relation: 'OR',
      entries: [
        [
          'uri',
          '/login',
        ],
        [
          'ip',
          '1.1.1.1',
        ],
      ],
    }
    entryData2 = {
      relation: 'AND',
      entries: [
        [
          'headers',
          ['user-agent', 'curl'],
        ],
        [
          'headers',
          ['content-type', 'application/json'],
        ],
      ],
    }
    ruleData = {
      relation: 'AND',
      sections: [
        entryData1,
        entryData2,
      ],
    }
    const onUpdate = (rule: GlobalFilter['rule']) => {
      wrapper.setProps({rule})
    }
    wrapper = mount(EntriesRelationList, {
      propsData: {
        rule: ruleData,
        editable: true,
      },
      listeners: {
        'update:rule': onUpdate,
      },
    })
  })
  afterEach(() => {
    jest.clearAllMocks()
  })

  test('should have a single table rendered for each entries list', () => {
    const tables = wrapper.findAll('.entries-table')
    expect(tables.length).toEqual(2)
  })

  test('should display correct data from prop to view', () => {
    const wantedEntryData = _.cloneDeep(entryData1.entries)
    const component = wrapper.findComponent(EntriesRelationList)
    const categories = component.findAll('.entry-category')
    const values = component.findAll('.entry-value')
    expect(categories.at(0).text().toLowerCase()).toContain(wantedEntryData[0][0].toLowerCase())
    expect(values.at(0).text().toLowerCase()).toContain((wantedEntryData[0][1] as string).toLowerCase())
    expect(categories.at(1).text().toLowerCase()).toContain(wantedEntryData[1][0].toLowerCase())
    expect(values.at(1).text().toLowerCase()).toContain((wantedEntryData[1][1] as string).toLowerCase())
    expect(ruleData.sections[0].entries).toEqual(wantedEntryData)
  })

  test('should display correct data from prop to view if data changed', async () => {
    const wantedEntryData = ['ip', '1.2.3.4']
    const newRuleData = JSON.parse(JSON.stringify(ruleData))
    newRuleData.sections[0].entries = [wantedEntryData]
    wrapper.setProps({rule: newRuleData})
    await Vue.nextTick()
    const component = wrapper.findComponent(EntriesRelationList)
    const categories = component.findAll('.entry-category')
    const values = component.findAll('.entry-value')
    expect(categories.at(0).text().toLowerCase()).toContain(wantedEntryData[0].toLowerCase())
    expect(values.at(0).text().toLowerCase()).toContain(wantedEntryData[1].toLowerCase())
  })

  test('should not break if not given a prop', () => {
    wrapper = mount(EntriesRelationList)
    const component = wrapper.findComponent(EntriesRelationList)
    expect(component).toBeTruthy()
  })

  test('should not break if data changes to invalid data', async () => {
    (entryData1 as any) = {}
    await Vue.nextTick()
    const component = wrapper.findComponent(EntriesRelationList)
    expect(component).toBeTruthy()
  })

  describe('relation switch labels', () => {
    test('should change section relation between `OR` and `AND` when clicked', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const section = component.findAll('.section').at(1)
      const sectionRelationToggle = section.find('.section-relation-toggle')
      sectionRelationToggle.trigger('click')
      await Vue.nextTick()
      expect(sectionRelationToggle.text()).toEqual('OR')
      sectionRelationToggle.trigger('click')
      await Vue.nextTick()
      expect(sectionRelationToggle.text()).toEqual('AND')
    })

    test('should not change section relation if section entries contains two entries of same category', async () => {
      const newRuleData = JSON.parse(JSON.stringify(ruleData))
      newRuleData.sections[0].entries = [
        [
          'uri',
          '/login',
        ],
        [
          'uri',
          '/account',
        ],
      ]
      wrapper.setProps({rule: newRuleData})
      await Vue.nextTick()
      const component = wrapper.findComponent(EntriesRelationList)
      const section = component.findAll('.section').at(0)
      const sectionRelationToggle = section.find('.section-relation-toggle')
      sectionRelationToggle.trigger('click')
      await Vue.nextTick()
      expect(sectionRelationToggle.text()).toEqual('OR')
    })
  })

  describe('large data pagination', () => {
    let checkedComponent: Wrapper<Vue>
    let checkedTable: Wrapper<Vue>
    beforeEach(() => {
      entryData1 = {
        relation: 'AND',
        entries: [
          ['uri', '/login0'],
          ['uri', '/login1'],
          ['uri', '/login2'],
          ['uri', '/login3'],
          ['uri', '/login4'],
          ['uri', '/login5'],
          ['uri', '/login6'],
          ['uri', '/login7'],
          ['uri', '/login8'],
          ['uri', '/login9'],
          ['uri', '/account0'],
          ['uri', '/account1'],
          ['uri', '/account2'],
          ['uri', '/account3'],
          ['uri', '/account4'],
          ['uri', '/account5'],
          ['uri', '/account6'],
          ['uri', '/account7'],
          ['uri', '/account8'],
          ['uri', '/account9'],
          ['uri', '/about0'],
          ['uri', '/about1'],
          ['uri', '/about2'],
          ['uri', '/about3'],
          ['uri', '/about4'],
          ['uri', '/about5'],
          ['uri', '/about6'],
          ['uri', '/about7'],
          ['uri', '/about8'],
          ['uri', '/about9'],
        ],
      }
      ruleData = {
        relation: 'AND',
        sections: [
          entryData1,
        ],
      }
      const onUpdate = (rule: GlobalFilter['rule']) => {
        wrapper.setProps({rule})
      }
      wrapper = mount(EntriesRelationList, {
        propsData: {
          rule: ruleData,
          editable: true,
        },
        listeners: {
          update: onUpdate,
        },
      })
      checkedComponent = wrapper.findComponent(EntriesRelationList)
      checkedTable = checkedComponent.findAll('.entries-table').at(0)
    })

    test('should show 20 entries per page', () => {
      const entryRows = checkedTable.findAll('.entry-row')
      expect(entryRows.length).toEqual(20)
    })

    test('should correctly render next page when next page button is clicked', async () => {
      const nextPageButton = checkedTable.find('.pagination-next')
      nextPageButton.trigger('click')
      await Vue.nextTick()
      checkedTable = checkedComponent.findAll('.entries-table').at(0)
      const entryRows = checkedTable.findAll('.entry-row')
      expect(entryRows.length).toEqual(10)
    })

    test('should have next page button disabled if currently in last page', async () => {
      const nextPageButton = checkedTable.find('.pagination-next')
      nextPageButton.trigger('click')
      await Vue.nextTick()
      expect(nextPageButton.attributes('disabled')).toBeTruthy()
    })

    test('should correctly render prev page when next prev button is clicked', async () => {
      const nextPageButton = checkedTable.find('.pagination-next')
      nextPageButton.trigger('click')
      await Vue.nextTick()
      const prevPageButton = checkedTable.find('.pagination-previous')
      prevPageButton.trigger('click')
      await Vue.nextTick()
      checkedTable = checkedComponent.findAll('.entries-table').at(0)
      const entryRows = checkedTable.findAll('.entry-row')
      expect(entryRows.length).toEqual(20)
    })

    test('should have prev page button disabled if currently in first page', async () => {
      const prevPageButton = checkedTable.find('.pagination-previous')
      expect(prevPageButton.attributes('disabled')).toBeTruthy()
    })
  })

  describe('rule prop validator', () => {
    let validator: Function
    beforeEach(() => {
      validator = (wrapper.vm as any).$options.props.rule.validator
    })

    test('should return true for data in the correct schema', () => {
      const isValid = validator(ruleData)
      expect(isValid).toEqual(true)
    })

    test('should return false for relation not `or` or `and`', () => {
      (ruleData as any).relation = 'unknown value'
      const isValid = validator(ruleData)
      expect(isValid).toEqual(false)
    })

    test('should return false for entries with too few arguments', () => {
      (ruleData as any).sections[0].entries[0] = ['ip']
      const isValid = validator(ruleData)
      expect(isValid).toEqual(false)
    })

    test('should return false for entries with too many arguments', () => {
      (ruleData as any).sections[0].entries[0] = ['ip', 'test', 'banana', 'apple', 'pear', 'eggplant']
      const isValid = validator(ruleData)
      expect(isValid).toEqual(false)
    })

    test('should return false for object entry which does not match the schema', () => {
      (ruleData as any).sections[0].entries[0] = {
        prop: 'value',
      }
      const isValid = validator(ruleData)
      expect(isValid).toEqual(false)
    })

    test('should return false for undefined sections', () => {
      ruleData.sections = undefined
      const isValid = validator(ruleData)
      expect(isValid).toEqual(false)
    })

    test('should return false for undefined relation', () => {
      ruleData.relation = undefined
      const isValid = validator(ruleData)
      expect(isValid).toEqual(false)
    })

    test('should return false for undefined rule', () => {
      ruleData = undefined
      const isValid = validator(ruleData)
      expect(isValid).toEqual(false)
    })
  })

  describe('add section button', () => {
    test('should add empty section', () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const addSectionButton = component.find('.add-section-button')
      addSectionButton.trigger('click')
      const tables = component.findAll('.entries-table')
      expect(tables.length).toEqual(2)
    })

    test('should not have the option to add component if not editable', () => {
      wrapper = mount(EntriesRelationList, {
        propsData: {
          rule: ruleData,
          editable: false,
        },
      })
      const component = wrapper.findComponent(EntriesRelationList)
      const addSectionButton = component.find('.add-section-button')
      expect(addSectionButton.element).toBeUndefined()
    })
  })

  describe('remove section button', () => {
    test('should remove section', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const removeSectionButton = component.find('.remove-section-button')
      removeSectionButton.trigger('click')
      await Vue.nextTick()
      const tables = wrapper.findAll('.entries-table')
      expect(tables.length).toEqual(1)
    })

    test('should not have the option to remove section if no sections exist', () => {
      ruleData.sections = []
      wrapper = mount(EntriesRelationList, {
        propsData: {
          rule: ruleData,
          editable: true,
        },
      })
      const component = wrapper.findComponent(EntriesRelationList)
      const removeSectionButton = component.find('.remove-section-button')
      expect(removeSectionButton.element).toBeUndefined()
    })

    test('should not have the option to remove component if not editable', () => {
      wrapper = mount(EntriesRelationList, {
        propsData: {
          rule: ruleData,
          editable: false,
        },
      })
      const component = wrapper.findComponent(EntriesRelationList)
      const removeSectionButton = component.find('.remove-section-button')
      expect(removeSectionButton.element).toBeUndefined()
    })
  })

  describe('add entry button', () => {
    test('should open new entry row', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      const newEntryRow = component.find('.new-entry-row')
      expect(newEntryRow.element).toBeDefined()
    })

    test('should add new entry from input when confirm button is clicked', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      const newEntryRow = component.find('.new-entry-row')
      const newEntryTextarea = newEntryRow.find('.new-entry-textarea')
      newEntryTextarea.setValue('1.2.3.4#annotation')
      const confirmAddEntryButton = component.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).rule.sections[0].entries.length).toEqual(3)
      expect((wrapper.vm as any).rule.sections[0].entries[2]).toEqual(['ip', '1.2.3.4', 'annotation'])
    })

    test('should add new entry from input with general annotation when confirm button is clicked', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      const newEntryRow = component.find('.new-entry-row')
      const newEntryTextarea = newEntryRow.find('.new-entry-textarea')
      newEntryTextarea.setValue('1.2.3.4')
      const newEntryAnnotation = newEntryRow.find('.new-entry-value-annotation-input')
      newEntryAnnotation.setValue('annot')
      const confirmAddEntryButton = component.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).rule.sections[0].entries.length).toEqual(3)
      expect((wrapper.vm as any).rule.sections[0].entries[2]).toEqual(['ip', '1.2.3.4', 'annot'])
    })

    test('should add multiple new entries from input when confirm button is clicked', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      const newEntryRow = component.find('.new-entry-row')
      const newEntryTextarea = newEntryRow.find('.new-entry-textarea')
      newEntryTextarea.setValue('1.2.3.4#annotation\n127.0.0.1#localhost')
      const confirmAddEntryButton = component.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).rule.sections[0].entries.length).toEqual(4)
      expect((wrapper.vm as any).rule.sections[0].entries[2]).toEqual(['ip', '1.2.3.4', 'annotation'])
      expect((wrapper.vm as any).rule.sections[0].entries[3]).toEqual(['ip', '127.0.0.1', 'localhost'])
    })

    test('should add multiple new entries with general' +
      'annotation from input when confirm button is clicked', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      const newEntryRow = component.find('.new-entry-row')
      const newEntryTextarea = newEntryRow.find('.new-entry-textarea')
      newEntryTextarea.setValue('1.2.3.4\n127.0.0.1#localhost')
      const newEntryAnnotation = newEntryRow.find('.new-entry-value-annotation-input')
      newEntryAnnotation.setValue('annot')
      const confirmAddEntryButton = component.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).rule.sections[0].entries.length).toEqual(4)
      expect((wrapper.vm as any).rule.sections[0].entries[2]).toEqual(['ip', '1.2.3.4', 'annot'])
      expect((wrapper.vm as any).rule.sections[0].entries[3]).toEqual(['ip', '127.0.0.1', 'localhost'])
    })

    test('should add new entries from name-value input when confirm button is clicked', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      const newEntryRow = component.find('.new-entry-row')
      const typeSelection = newEntryRow.find('.new-entry-type-selection')
      typeSelection.trigger('click')
      const options = typeSelection.findAll('option')
      options.at(7).setSelected()
      await Vue.nextTick()
      const newEntryInputName = newEntryRow.find('.new-entry-name-input')
      newEntryInputName.setValue('something')
      const newEntryInputValue = newEntryRow.find('.new-entry-value-annotation-input')
      newEntryInputValue.setValue('right')
      const confirmAddEntryButton = component.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).rule.sections[0].entries.length).toEqual(3)
      expect((wrapper.vm as any).rule.sections[0].entries[2]).toEqual(['headers', ['something', 'right']])
    })

    test('should not add new entries from multi-line' +
      'input when confirm button is clicked if has too few arguments', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      const newEntryRow = component.find('.new-entry-row')
      const typeSelection = newEntryRow.find('.new-entry-type-selection')
      typeSelection.trigger('click')
      const options = typeSelection.findAll('option')
      options.at(7).setSelected()
      await Vue.nextTick()
      const newEntryInputName = newEntryRow.find('.new-entry-name-input')
      newEntryInputName.setValue('something')
      const newEntryInputValue = newEntryRow.find('.new-entry-value-annotation-input')
      newEntryInputValue.setValue('')
      const confirmAddEntryButton = component.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).rule.sections[0].entries.length).toEqual(2)
    })

    test('should not show new entry button if not editable', () => {
      wrapper = mount(EntriesRelationList, {
        propsData: {
          rule: ruleData,
          editable: false,
        },
      })
      const component = wrapper.findComponent(EntriesRelationList)
      const addEntryButton = component.find('.add-entry-button')
      expect(addEntryButton.element).toBeUndefined()
    })

    test('should set section relation to `OR` if two items of same category added', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      // set relation to AND
      const sectionRelationToggle = component.findAll('.section-relation-toggle').at(0)
      sectionRelationToggle.trigger('click')
      await Vue.nextTick()
      expect(sectionRelationToggle.text()).toEqual('AND')
      // open new entry row
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      // add input to new entry row
      const newEntryRow = component.find('.new-entry-row')
      const newEntryTextarea = newEntryRow.find('.new-entry-textarea')
      newEntryTextarea.setValue('1.2.3.4#annotation\n1.2.3.5#wow')
      // confirm add new entry
      const confirmAddEntryButton = component.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      // check
      expect(sectionRelationToggle.text()).toEqual('OR')
    })

    test('should not set section relation to `OR` if no more than one item of same category added', async () => {
      const newRuleData = JSON.parse(JSON.stringify(ruleData))
      newRuleData.sections[0].entries = [
        [
          'uri',
          '/login',
        ],
      ]
      wrapper.setProps({rule: newRuleData})
      await Vue.nextTick()
      const component = wrapper.findComponent(EntriesRelationList)
      // check relation is AND
      const sectionRelationToggle = component.findAll('.section-relation-toggle').at(0)
      expect(sectionRelationToggle.text()).toEqual('AND')
      // open new entry row
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      // add input to new entry row
      const newEntryRow = component.find('.new-entry-row')
      const newEntryTextarea = newEntryRow.find('.new-entry-textarea')
      newEntryTextarea.setValue('1.2.3.4#annotation')
      // confirm add new entry
      const confirmAddEntryButton = component.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      // check relation is still AND
      expect(sectionRelationToggle.text()).toEqual('AND')
    })

    test('should not set section relation to `OR` if two headers added', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      // set relation to AND
      const sectionRelationToggle = component.findAll('.section-relation-toggle').at(0)
      sectionRelationToggle.trigger('click')
      await Vue.nextTick()
      expect(sectionRelationToggle.text()).toEqual('AND')
      // open new entry row
      let addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      let newEntryRow = component.find('.new-entry-row')
      // change entry type to headers
      let typeSelection = newEntryRow.find('.new-entry-type-selection')
      typeSelection.trigger('click')
      let options = typeSelection.findAll('option')
      options.at(7).setSelected()
      await Vue.nextTick()
      // add input to new entry row
      let newEntryTextarea = newEntryRow.find('.new-entry-textarea')
      newEntryTextarea.setValue('something\nright\nhere')
      // confirm add new entry
      let confirmAddEntryButton = component.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      // open new entry row - second time
      addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      newEntryRow = component.find('.new-entry-row')
      // change entry type to headers - second time
      typeSelection = newEntryRow.find('.new-entry-type-selection')
      typeSelection.trigger('click')
      options = typeSelection.findAll('option')
      options.at(7).setSelected()
      await Vue.nextTick()
      // add input to new entry row - second time
      newEntryTextarea = newEntryRow.find('.new-entry-textarea')
      newEntryTextarea.setValue('something\nright\nhere')
      // confirm add new entry - second time
      confirmAddEntryButton = component.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      // check
      expect(sectionRelationToggle.text()).toEqual('AND')
    })
  })

  describe('cancel entry button', () => {
    test('should close adding mode', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      const cancelEntryButton = component.find('.cancel-entry-button')
      cancelEntryButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).newEntrySectionIndex).toEqual(-1)
    })
    test('should remove empty section', async () => {
      wrapper = mount(EntriesRelationList, {
        propsData: {
          rule: {
            relation: 'OR',
            sections: [],
          },
          editable: true,
        },
      })
      const component = wrapper.findComponent(EntriesRelationList)
      const addSectionButton = component.find('.add-section-button')
      addSectionButton.trigger('click')
      await Vue.nextTick()
      const cancelEntryButton = component.find('.cancel-entry-button')
      cancelEntryButton.trigger('click')
      await Vue.nextTick()
      expect(component.findAll('.section').length).toEqual(0)
    })
  })

  describe('remove entry button', () => {
    test('should remove entry', async () => {
      const component = wrapper.findComponent(EntriesRelationList)
      const removeEntryButton = component.find('.remove-entry-button')
      removeEntryButton.trigger('click')
      await Vue.nextTick()
      expect((wrapper.vm as any).rule.sections[0].entries.length).toEqual(1)
    })
    test('should remove section if no entries left', async () => {
      const entryData3 = {
        relation: 'AND',
        entries: [
          [
            'headers',
            ['user-agent', 'curl'],
          ],
        ],
      }
      const rule = {
        relation: 'AND',
        sections: [
          entryData1,
          entryData3,
        ],
      }
      wrapper = mount(EntriesRelationList, {
        propsData: {
          rule,
          editable: true,
        },
      })
      const component = wrapper.findComponent(EntriesRelationList)
      const section = component.findAll('.section').at(1)
      const removeEntryButton = section.find('.remove-entry-button')
      removeEntryButton.trigger('click')
      await Vue.nextTick()
      expect(component.findAll('.section').length).toEqual(rule.sections.length - 1)
    })
  })

  describe('validators', () => {
    let newEntryTextarea: Wrapper<Vue>
    let newEntrySecondAttr: Wrapper<Vue>
    let component: Wrapper<Vue>
    let newEntryRows: WrapperArray<Vue>
    let newEntryRow: Wrapper<Vue>
    let confirmAddEntryButton: Wrapper<Vue>
    let options: WrapperArray<Vue>
    beforeEach(async () => {
      component = wrapper.findComponent(EntriesRelationList)
      // open new entry row
      const addEntryButton = component.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      // add input to new entry row
      newEntryRows = component.findAll('.new-entry-row')
      newEntryRow = newEntryRows.at(0)
      newEntryTextarea = newEntryRow.find('.new-entry-textarea')
      newEntrySecondAttr = newEntryRow.find('.new-entry-value-annotation-input')
      const typeSelection = newEntryRow.find('.new-entry-type-selection')
      typeSelection.trigger('click')
      options = typeSelection.findAll('option')
      confirmAddEntryButton = component.find('.confirm-add-entry-button')
    })
    test('should not add entry if it contains an error or empty first attr', async () => {
      newEntryTextarea.setValue('')
      // confirm add new entry
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      // check
      const entries = component.findAll('.entry-row')
      expect(entries.length).toEqual(entryData1.entries.length + entryData2.entries.length)
      expect(newEntryRows.length).toEqual(1)
    })
    test('should highlight duplicated entries', async () => {
      newEntryTextarea.setValue('1.2.3.4\n1.2.3.4\n1.2.3.4')
      // confirm add new entry
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      // check
      expect((wrapper.vm as any).duplicatedEntries.length).toEqual(1)
    })
    test('should validate duplicated entries if category is args, cookies, headers', async () => {
      // open new entry row
      const section = component.findAll('.section').at(1)
      const addEntryButton = section.find('.add-entry-button')
      addEntryButton.trigger('click')
      await Vue.nextTick()
      // change entry type to headers
      options.at(7).setSelected()
      await Vue.nextTick()
      // add input to new entry row
      newEntryRow = section.find('.new-entry-row')
      const newEntryFirstAttr = newEntryRow.find('.new-entry-name-input')
      const duplicateValues = entryData2.entries[0][1]
      newEntryFirstAttr.setValue(duplicateValues[0])
      newEntrySecondAttr.setValue(duplicateValues[1])
      // confirm add new entry
      confirmAddEntryButton = newEntryRow.find('.confirm-add-entry-button')
      confirmAddEntryButton.trigger('click')
      await Vue.nextTick()
      // check
      expect((wrapper.vm as any).duplicatedEntries.length).toEqual(1)
    })
    test('should validate value as regex if category is path, query, or uri', async () => {
      // change entry type to path
      options.at(0).setSelected()
      await Vue.nextTick()
      newEntryTextarea.setValue('\\')
      // check
      expect((wrapper.vm as any).entriesErrors.length).toEqual(1)
    })
    test('should validate value as ip if category is ip', async () => {
      // change entry type to ip
      options.at(4).setSelected()
      await Vue.nextTick()
      newEntryTextarea.setValue('1.1.1.1.')
      // check
      expect((wrapper.vm as any).entriesErrors.length).toEqual(1)
    })
    test('should validate value as non-empty if category is not ip, path, query, uri', async () => {
      // change entry type to method
      options.at(3).setSelected()
      await Vue.nextTick()
      newEntryTextarea.setValue(' ')
      // check
      expect((wrapper.vm as any).entriesErrors.length).toEqual(1)
    })
    test('should validate second attribute if category is args, cookies, headers', async () => {
      // change entry type to headers
      options.at(7).setSelected()
      await Vue.nextTick()
      newEntrySecondAttr.setValue('\\')
      // check
      expect((wrapper.vm as any).entriesErrors.length).toEqual(1)
    })
    test('should clear errors', async () => {
      // change entry type to headers
      options.at(7).setSelected()
      await Vue.nextTick()
      newEntrySecondAttr.setValue('\\')
      await Vue.nextTick()
      newEntrySecondAttr.setValue('something')
      await Vue.nextTick()
      // check
      expect((wrapper.vm as any).entriesErrors.length).toEqual(0)
    })
  })
})
