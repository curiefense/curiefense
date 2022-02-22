import ContentFilterRulesEditor from '@/doc-editors/ContentFilterRulesEditor.vue'
import {beforeEach, describe, expect, test} from '@jest/globals'
import {shallowMount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'
import {ContentFilterRule} from '@/types'

describe('ContentFilterRulesEditor.vue', () => {
  let docs: ContentFilterRule[]
  let wrapper: Wrapper<Vue>
  beforeEach(async () => {
    docs = [{
      'id': '100000',
      'name': '100000',
      'msg': 'SQLi Attempt (Conditional Operator Detected)',
      'operand': '\\s(and|or)\\s+\\d+\\s+.*between\\s.*\\d+\\s+and\\s+\\d+.*',
      'risk': 5,
      'notes': 'SQL injection',
      'category': 'sqli',
      'subcategory': 'statement injection',
    }]
    wrapper = shallowMount(ContentFilterRulesEditor, {
      propsData: {
        selectedDoc: docs[0],
      },
    })
    await Vue.nextTick()
  })

  describe('form data', () => {
    test('should have correct ID displayed', () => {
      expect(wrapper.find('.document-id').text()).toEqual(docs[0].id)
    })

    test('should have correct name in input', () => {
      const element = wrapper.find('.document-name').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].name)
    })

    test('should have correct operand in input', () => {
      const element = wrapper.find('.document-operand').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].operand)
    })

    test('should have correct risk displayed', () => {
      const riskSelection = wrapper.find('.risk-level-selection')
      expect((riskSelection.find('option:checked').element as HTMLOptionElement).value).toEqual(docs[0].risk.toString())
    })

    test('should have correct category displayed', () => {
      const element = wrapper.find('.document-category').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].category)
    })

    test('should have correct subcategory displayed', () => {
      const element = wrapper.find('.document-subcategory').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].subcategory)
    })
  })

  test('should emit doc update when name input changes', async () => {
    const wantedName = 'new name'
    const wantedEmit = JSON.parse(JSON.stringify(docs[0]))
    wantedEmit.name = wantedName
    const element = wrapper.find('.document-name')
    element.setValue(wantedName)
    element.trigger('change')
    await Vue.nextTick()
    expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
    expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([wantedEmit])
  })

  test('should emit doc update when notes input changes', async () => {
    const wantedNotes = 'new notes'
    const wantedEmit = JSON.parse(JSON.stringify(docs[0]))
    wantedEmit.notes = wantedNotes
    const element = wrapper.find('.document-notes')
    element.setValue(wantedNotes)
    element.trigger('change')
    await Vue.nextTick()
    expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
    expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([wantedEmit])
  })

  test('should emit doc update when category input changes', async () => {
    const wantedCategory = 'new category'
    const wantedEmit = JSON.parse(JSON.stringify(docs[0]))
    wantedEmit.category = wantedCategory
    const element = wrapper.find('.document-category')
    element.setValue(wantedCategory)
    element.trigger('change')
    await Vue.nextTick()
    expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
    expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([wantedEmit])
  })

  test('should emit doc update when subcategory input changes', async () => {
    const wantedSubcategory = 'new category'
    const wantedEmit = JSON.parse(JSON.stringify(docs[0]))
    wantedEmit.subcategory = wantedSubcategory
    const element = wrapper.find('.document-subcategory')
    element.setValue(wantedSubcategory)
    element.trigger('change')
    await Vue.nextTick()
    expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
    expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([wantedEmit])
  })

  test('should emit doc update when risk level input changes', async () => {
    const wantedRisk = 3
    const wantedEmit = JSON.parse(JSON.stringify(docs[0]))
    wantedEmit.risk = wantedRisk
    const selection = wrapper.find('.risk-level-selection')
    const options = selection.findAll('option')
    options.at(2).setSelected() // index => value: 0 => 1, 1 => 2, 2 => 3
    selection.trigger('change')
    await Vue.nextTick()
    expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
    expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([wantedEmit])
  })

  test('should emit doc update when operand input changes', async () => {
    const wantedOperand = '\\s(and|or)\\s+["\']\\w+["\']\\s+.*between\\s.*["\']\\w+["\']\\s+and\\s+["\']\\w+.*'
    const wantedEmit = JSON.parse(JSON.stringify(docs[0]))
    wantedEmit.operand = wantedOperand
    const element = wrapper.find('.document-operand')
    element.setValue(wantedOperand)
    element.trigger('change')
    await Vue.nextTick()
    expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
    expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([wantedEmit])
  })
})
