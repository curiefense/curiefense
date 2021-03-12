import WAFEditor from '@/doc-editors/WAFEditor.vue'
import {beforeEach, describe, expect, test} from '@jest/globals'
import {shallowMount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'
import {ArgsCookiesHeadersType, NamesRegexType, WAFPolicy} from '@/types'
import SerializedInput from '@/components/SerializedInput.vue'

describe('WAFEditor.vue', () => {
  let docs: WAFPolicy[]
  let wrapper: Wrapper<Vue>
  beforeEach(() => {
    docs = [{
      'id': '__default__',
      'name': 'default waf',
      'ignore_alphanum': true,
      'max_header_length': 1024,
      'max_cookie_length': 2048,
      'max_arg_length': 1536,
      'max_headers_count': 36,
      'max_cookies_count': 42,
      'max_args_count': 512,
      'args': {'names': [], 'regex': []},
      'headers': {'names': [], 'regex': []},
      'cookies': {'names': [], 'regex': []},
    }]
    wrapper = shallowMount(WAFEditor, {
      propsData: {
        selectedDoc: docs[0],
      },
    })
  })

  describe('form data', () => {
    test('should have correct ID displayed', () => {
      expect(wrapper.find('.document-id').text()).toEqual(docs[0].id)
    })

    test('should have correct name in input', () => {
      const element = wrapper.find('.document-name').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].name)
    })

    test('should have correct max header length in input', () => {
      const element = wrapper.find('.max-header-length-input').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].max_header_length.toString())
    })

    test('should have correct max cookie length in input', () => {
      const element = wrapper.find('.max-cookie-length-input').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].max_cookie_length.toString())
    })

    test('should have correct max arg length in input', () => {
      const element = wrapper.find('.max-arg-length-input').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].max_arg_length.toString())
    })

    test('should have correct max headers count in input', () => {
      const element = wrapper.find('.max-headers-count-input').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].max_headers_count.toString())
    })

    test('should have correct max cookies count in input', () => {
      const element = wrapper.find('.max-cookies-count-input').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].max_cookies_count.toString())
    })

    test('should have correct max args count in input', () => {
      const element = wrapper.find('.max-args-count-input').element as HTMLInputElement
      expect(element.value).toEqual(docs[0].max_args_count.toString())
    })

    test('should have correct ignore alphanumeric boolean in checkbox input', () => {
      const element = wrapper.find('.ignore-alphanumeric-input').element as HTMLInputElement
      expect(element.checked).toEqual(docs[0].ignore_alphanum)
    })
  })

  test('should unpack exclusions correctly from model for view', async () => {
    const unpackedExclusions = '100040 100041'
    const packedExclusions = {
      '100040': 1,
      '100041': 1,
    }
    const actualUnpackedExclusions = (wrapper.vm as any).unpackExclusions(packedExclusions)
    expect(actualUnpackedExclusions).toEqual(unpackedExclusions)
  })

  test('should pack exclusions correctly from view for model', async () => {
    const unpackedExclusions = '100040 100041'
    const packedExclusions = {
      '100040': 1,
      '100041': 1,
    }
    const actualPackedExclusions = (wrapper.vm as any).packExclusions(unpackedExclusions)
    expect(actualPackedExclusions).toEqual(packedExclusions)
  })

  test('should unpack empty exclusions correctly from model for view', async () => {
    const unpackedExclusions = ''
    const packedExclusions = {}
    const actualUnpackedExclusions = (wrapper.vm as any).unpackExclusions(packedExclusions)
    expect(actualUnpackedExclusions).toEqual(unpackedExclusions)
  })

  test('should pack empty exclusions correctly from view for model', async () => {
    const unpackedExclusions = ''
    const packedExclusions = {}
    const actualPackedExclusions = (wrapper.vm as any).packExclusions(unpackedExclusions)
    expect(actualPackedExclusions).toEqual(packedExclusions)
  })

  buildTabDescribe('headers')
  buildTabDescribe('cookies')
  buildTabDescribe('args')

  function buildTabDescribe(tab: ArgsCookiesHeadersType) {
    describe(`tab ${tab}`, () => {
      beforeEach(async () => {
        // select tab
        const tabElement = wrapper.find(`.${tab}-tab`)
        tabElement.trigger('click')
        await Vue.nextTick()
      })

      test('should open new parameter row when button is clicked', async () => {
        const button = wrapper.find('.new-parameter-button')
        button.trigger('click')
        await Vue.nextTick()
        const newRow = wrapper.find('.new-parameter-row')
        expect(newRow.element).toBeDefined()
      })

      buildNamesRegexDescribe('names', 0)
      buildNamesRegexDescribe('regex', 1)

      function buildNamesRegexDescribe(type: NamesRegexType, typeIndex: number) {
        describe(`type ${type}`, () => {
          let newRow: Wrapper<Vue>

          beforeEach(async () => {
            const button = wrapper.find('.new-parameter-button')
            button.trigger('click')
            await Vue.nextTick()
            newRow = wrapper.find('.new-parameter-row')
            const typeSelection = newRow.find('.new-entry-type')
            const options = typeSelection.findAll('option')
            options.at(typeIndex).setSelected()
            await Vue.nextTick()
          })

          test('should add name key when creating new parameter', async () => {
            const wantedValue = 'foo'
            const input = newRow.find('.new-entry-key')
            input.setValue(wantedValue)
            await Vue.nextTick()
            const confirmButton = newRow.find('.confirm-add-new-parameter')
            confirmButton.trigger('click')
            await Vue.nextTick()
            const actualValue = (wrapper.find('.entry-key').element as HTMLInputElement).value
            expect(actualValue).toEqual(wantedValue)
          })

          test('should add value when creating new parameter', async () => {
            const wantedValue = 'bar'
            const input = newRow.find('.new-entry-reg')
            input.setValue(wantedValue)
            await Vue.nextTick()
            const confirmButton = newRow.find('.confirm-add-new-parameter')
            confirmButton.trigger('click')
            await Vue.nextTick()
            const actualValue = (wrapper.find('.entry-reg').element as HTMLInputElement).value
            expect(actualValue).toEqual(wantedValue)
          })

          test('should add restrict when creating new parameter', async () => {
            const input = newRow.find('.new-entry-restrict')
            input.setChecked(true)
            await Vue.nextTick()
            const confirmButton = newRow.find('.confirm-add-new-parameter')
            confirmButton.trigger('click')
            await Vue.nextTick()
            const actualValue = (wrapper.find('.entry-restrict').element as HTMLInputElement).checked
            expect(actualValue).toEqual(true)
          })

          test('should add mask when creating new parameter', async () => {
            const input = newRow.find('.new-entry-mask')
            input.setChecked(true)
            await Vue.nextTick()
            const confirmButton = newRow.find('.confirm-add-new-parameter')
            confirmButton.trigger('click')
            await Vue.nextTick()
            const actualValue = (wrapper.find('.entry-mask').element as HTMLInputElement).checked
            expect(actualValue).toEqual(true)
          })

          test('should add exclusions when creating new parameter', async () => {
            const wantedValue = {
              '100040': 1,
              '100041': 1,
            }
            const serializedInput = wrapper.findComponent(SerializedInput)
            serializedInput.vm.$emit('update:value', wantedValue)
            await Vue.nextTick()
            const confirmButton = newRow.find('.confirm-add-new-parameter')
            confirmButton.trigger('click')
            await Vue.nextTick()
            const actualValue = wrapper.findComponent(SerializedInput).props('value')
            expect(actualValue).toEqual(wantedValue)
          })

          test('should remove parameter when remove button is clicked', async () => {
            const confirmButton = newRow.find('.confirm-add-new-parameter')
            confirmButton.trigger('click')
            await Vue.nextTick()
            const removeButton = wrapper.find('.remove-entry-button')
            removeButton.trigger('click')
            await Vue.nextTick()
            await wrapper.vm.$forceUpdate()
            const rows = wrapper.findAll('.entry-row')
            expect(rows.length).toEqual(0)
          })
        })
      }
    })
  }
})
