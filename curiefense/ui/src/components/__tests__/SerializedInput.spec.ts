import SerializedInput from '@/components/SerializedInput.vue'
import {beforeEach, describe, expect, test} from '@jest/globals'
import {mount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'

describe('SerializedInput.vue', () => {
  let wrapper: Wrapper<Vue>
  let value: any
  let placeholder: string
  let getFunction: Function
  let setFunction: Function
  beforeEach(() => {
    value = '4'
    placeholder = 'half the written value'
    getFunction = (value: any) => {
      return (value * 2).toString()
    }
    setFunction = (value: any) => {
      return (value / 2).toString()
    }
    wrapper = mount(SerializedInput, {
      propsData: {
        value: value,
        placeholder: placeholder,
        getFunction: getFunction,
        setFunction: setFunction,
      },
    })
  })

  test('should have correct provided placeholder for the input', () => {
    const input = wrapper.find('.serialized-input')
    expect((input.element as HTMLInputElement).placeholder).toEqual(placeholder)
  })

  test('should format value correctly from model to view', () => {
    const wantedValue = getFunction(value)
    const input = wrapper.find('.serialized-input')
    expect((input.element as HTMLInputElement).value).toEqual(wantedValue)
  })

  test('should format value correctly from model to view when value changes', async () => {
    const newValue = '8'
    const wantedValue = getFunction(newValue)
    wrapper.setProps({value: newValue})
    await Vue.nextTick()
    const input = wrapper.find('.serialized-input')
    expect((input.element as HTMLInputElement).value).toEqual(wantedValue)
  })

  test('should format and emit value correctly from view to model on blur', async () => {
    const wantedValue = value
    const input = wrapper.find('.serialized-input')
    input.trigger('blur')
    await Vue.nextTick()
    expect(wrapper.emitted('update:value')).toBeTruthy()
    expect(wrapper.emitted('update:value')[0]).toEqual([wantedValue])
  })

  test('should format and emit value correctly from view to model on change', async () => {
    const wantedValue = value
    const input = wrapper.find('.serialized-input')
    input.trigger('change')
    await Vue.nextTick()
    expect(wrapper.emitted('update:value')).toBeTruthy()
    expect(wrapper.emitted('update:value')[0]).toEqual([wantedValue])
  })
})
