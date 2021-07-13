import WAFSigsEditor from '@/doc-editors/WAFSigsEditor.vue'
import {beforeEach, describe, expect, test} from '@jest/globals'
import {shallowMount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'
import {WAFRule} from '@/types'

describe('WAFSigsEditor.vue', () => {
  let docs: WAFRule[]
  let wrapper: Wrapper<Vue>
  beforeEach(async () => {
    docs = [{
      'id': '100000',
      'name': '100000',
      'msg': 'SQLi Attempt (Conditional Operator Detected)',
      'operand': '\\s(and|or)\\s+\\d+\\s+.*between\\s.*\\d+\\s+and\\s+\\d+.*',
      'severity': 5,
      'certainity': 5,
      'category': 'sqli',
      'subcategory': 'statement injection',
    }]
    wrapper = shallowMount(WAFSigsEditor, {
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

    test('should have correct severity displayed', () => {
      expect(wrapper.find('.document-severity').text()).toEqual(docs[0].severity.toString())
    })

    test('should have correct certainty displayed', () => {
      expect(wrapper.find('.document-certainty').text()).toEqual(docs[0].certainity.toString())
    })

    test('should have correct category displayed', () => {
      expect(wrapper.find('.document-category').text()).toEqual(docs[0].category)
    })

    test('should have correct subcategory displayed', () => {
      expect(wrapper.find('.document-subcategory').text()).toEqual(docs[0].subcategory)
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

  test('should emit doc update when operand input changes', async () => {
    const wantedOperand = '\\s(and|or)\\s+["\']\\w+["\']\\s+.*between\\s.*["\']\\w+["\']\\s+and\\s+["\']\\w+.*'
    const wantedEmit = JSON.parse(JSON.stringify(docs[0]))
    wantedEmit.operand = wantedOperand
    const element = wrapper.find('.document-operand');
    element.setValue(wantedOperand)
    element.trigger('change')
    await Vue.nextTick()
    expect(wrapper.emitted('update:selectedDoc')).toBeTruthy()
    expect(wrapper.emitted('update:selectedDoc')[0]).toEqual([wantedEmit])
  })
})
