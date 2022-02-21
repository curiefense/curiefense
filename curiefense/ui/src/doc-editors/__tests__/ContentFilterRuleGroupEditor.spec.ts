import ContentFilterRuleGroupEditor from '@/doc-editors/ContentFilterRuleGroupEditor.vue'
import {beforeEach, describe, expect, jest, test} from '@jest/globals'
import {shallowMount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'
import {ContentFilterRuleGroup, ContentFilterRule} from '@/types'
import axios from 'axios'

jest.mock('axios')

describe('ContentFilterRuleGroupEditor.vue', () => {
  let rules: ContentFilterRule[]
  let wrapper: Wrapper<Vue>
  let selectedDoc: ContentFilterRuleGroup
  const ALL_RULES_NUMBER = 30
  const DOC_RULES_NUMBER = 25
  const docExample = {
    'id': '100000',
    'name': '100000',
    'msg': 'SQLi Attempt (Conditional Operator Detected)',
    'operand': '\\s(and|or)\\s+\\d+\\s+.*between\\s.*\\d+\\s+and\\s+\\d+.*',
    'severity': 5,
    'certainity': 5,
    'category': 'sqli',
    'subcategory': 'statement injection',
  }
  const selectedBranch = 'master'
  beforeEach(() => {
    rules = [...Array(ALL_RULES_NUMBER)].map((_d, index) => ({
      ...docExample,
      id: `${parseInt(docExample['id']) + index}`,
      name: `${parseInt(docExample['id']) + index}`,
      notes: '',
      risk: 1,
    }))
    selectedDoc = {
      id: '1',
      name: 'test group',
      description: 'the testing',
      content_filter_rule_ids: rules.slice(0, DOC_RULES_NUMBER).map(({id}: ContentFilterRule) => id),
    }
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === `/conf/api/v2/configs/${selectedBranch}/d/contentfilterrules/`) {
        return Promise.resolve({data: rules})
      }
      return Promise.resolve({data: []})
    })
    const onUpdate = (selectedDoc: ContentFilterRuleGroup) => {
      wrapper.setProps({selectedDoc})
    }
    wrapper = shallowMount(ContentFilterRuleGroupEditor, {
      propsData: {
        selectedDoc,
        selectedBranch,
      },
      listeners: {
        'update:selectedDoc': onUpdate,
      },
    })
  })

  test('should have an entries list table rendered', () => {
    const tables = wrapper.findAll('.entries-table')
    expect(tables.length).toEqual(1)
    expect(tables.at(0).findAll('.entry-row').length).toEqual((wrapper.vm as any).rowsPerPage)
  })

  test('should have only unattached rules in the list of rules to add to the group', async () => {
    const table = wrapper.find('.entries-table')
    const addBtn = table.find('.add-rule-button')
    addBtn.trigger('click')
    await Vue.nextTick()
    const rulesSelect = table.find('.new-rule-selection')
    expect(rulesSelect.findAll('option').length).toEqual(ALL_RULES_NUMBER - DOC_RULES_NUMBER)
  })

  test('should emit doc update when adding rule', async () => {
    const table = wrapper.find('.entries-table')
    const addBtn = table.find('.add-rule-button')
    addBtn.trigger('click')
    await Vue.nextTick()
    const rulesSelectOptions = table.find('.new-rule-selection').findAll('option')
    rulesSelectOptions.at(0).setSelected()
    await Vue.nextTick()
    table.find('.confirm-add-rule-button').trigger('click')
    await Vue.nextTick()
    expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
    expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([{
      ...selectedDoc,
      content_filter_rule_ids: [
        ...selectedDoc.content_filter_rule_ids,
        rules[DOC_RULES_NUMBER].id,
      ],
    }])
  })

  test('should not call adding function if no rule selected', async () => {
    const table = wrapper.find('.entries-table')
    const addBtn = table.find('.add-rule-button')
    addBtn.trigger('click')
    await Vue.nextTick()
    table.find('.confirm-add-rule-button').trigger('click')
    await Vue.nextTick()
    expect(wrapper.emitted('update:selectedDoc')).toBeFalsy()
  })

  test('should emit doc update when deleting rule', async () => {
    const table = wrapper.find('.entries-table')
    const removeBtns = table.findAll('.remove-rule-button')
    const indexToDelete = 10
    removeBtns.at(indexToDelete).trigger('click')
    await Vue.nextTick()
    expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
    expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([{
      ...selectedDoc,
      content_filter_rule_ids: selectedDoc.content_filter_rule_ids.filter((id, index) => index !== indexToDelete),
    }])
  })

  test('should not navigate by paginator if current page is out of range (1, totalPages)', async () => {
    const vm = wrapper.vm as any
    const {currentPage} = vm
    vm.navigate(0)
    await Vue.nextTick()
    expect(vm.currentPage).toEqual(currentPage)
    vm.navigate(1000)
    await Vue.nextTick()
    expect(vm.currentPage).toEqual(currentPage)
    vm.navigate(2)
    await Vue.nextTick()
    expect(vm.currentPage).toEqual(2)
  })
})
