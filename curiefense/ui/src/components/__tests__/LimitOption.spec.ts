// @ts-ignore
import LimitOption, {OptionObject} from '@/components/LimitOption.vue'
import {beforeEach, describe, expect, test} from '@jest/globals'
import {mount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'

describe('LimitOption.vue', () => {
  let option: OptionObject
  let wrapper: Wrapper<Vue>
  beforeEach(async () => {
    option = {
      type: 'self',
    }
    wrapper = mount(LimitOption, {
      propsData: {
        option: option,
        useDefaultSelf: true,
        useValue: true,
      },
    })
    await Vue.nextTick()
  })

  test('should render dropdown correctly with all possible types as options', () => {
    const selection = wrapper.find('.option-type-selection')
    const options = selection.findAll('option')
    expect(options.at(0).text()).toEqual('HTTP request')
    expect(options.at(1).text()).toEqual('Header')
    expect(options.at(2).text()).toEqual('Cookie')
    expect(options.at(3).text()).toEqual('Argument')
    expect(options.at(4).text()).toEqual('Attribute')
  })

  test('should render dropdown correctly without self as possible type as option', async () => {
    wrapper.setProps({useDefaultSelf: false})
    await Vue.nextTick()
    const selection = wrapper.find('.option-type-selection')
    const options = selection.findAll('option')
    expect(options.at(0).text()).toEqual('Header')
    expect(options.at(1).text()).toEqual('Cookie')
    expect(options.at(2).text()).toEqual('Argument')
    expect(options.at(3).text()).toEqual('Attribute')
  })

  describe('label prop', () => {
    test('should not render label if no label provided', () => {
      wrapper = mount(LimitOption, {
        propsData: {
          option: option,
        },
      })
      const label = wrapper.find('.form-label')
      expect(label.exists()).toBeFalsy()
    })

    test('should render label', () => {
      const wantedLabel = 'Test'
      wrapper = mount(LimitOption, {
        propsData: {
          option: option,
          label: wantedLabel,
        },
      })
      const label = wrapper.find('.form-label')
      expect(label.text()).toEqual(wantedLabel)
    })
  })

  describe('showRemove prop', () => {
    test('should show button if showRemove prop is true', async () => {
      wrapper = mount(LimitOption, {
        propsData: {
          option: option,
          showRemove: true,
        },
      })
      await Vue.nextTick()
      const button = wrapper.find('.remove-option-button')
      expect(button.exists()).toBeTruthy()
    })

    test('should not show button if showRemove prop is false', async () => {
      wrapper = mount(LimitOption, {
        propsData: {
          option: option,
          showRemove: false,
        },
      })
      await Vue.nextTick()
      const button = wrapper.find('.remove-option-button')
      expect(button.exists()).toBeFalsy()
    })

    test('should not show button if showRemove prop does not exist', async () => {
      wrapper = mount(LimitOption, {
        propsData: {
          option: option,
        },
      })
      await Vue.nextTick()
      const button = wrapper.find('.remove-option-button')
      expect(button.exists()).toBeFalsy()
    })
  })

  describe('ignoreAttributes prop', () => {
    test('should render dropdown correctly without ignored action types (tags, method)', async () => {
      option = {
        type: 'attrs',
      }
      wrapper = mount(LimitOption, {
        propsData: {
          option: option,
          ignoreAttributes: ['tags', 'method'],
        },
      })
      await Vue.nextTick()
      const selection = wrapper.find('.option-attribute-selection')
      const options = selection.findAll('option')
      expect(options.at(0).text()).toEqual('IP Address')
      expect(options.at(1).text()).toEqual('Provider')
      expect(options.at(2).text()).toEqual('URI')
      expect(options.at(3).text()).toEqual('Path')
      expect(options.at(4).text()).toEqual('Query')
      expect(options.at(5).text()).toEqual('Company')
      expect(options.at(6).text()).toEqual('Country')
      expect(options.at(7).text()).toEqual('Authority')
    })

    test('should render dropdown correctly without ignored action types (ip, uri, company)', async () => {
      option = {
        type: 'attrs',
      }
      wrapper = mount(LimitOption, {
        propsData: {
          option: option,
          ignoreAttributes: ['ip', 'uri', 'country'],
        },
      })
      await Vue.nextTick()
      const selection = wrapper.find('.option-attribute-selection')
      const options = selection.findAll('option')
      expect(options.at(0).text()).toEqual('Provider')
      expect(options.at(1).text()).toEqual('Path')
      expect(options.at(2).text()).toEqual('Tag')
      expect(options.at(3).text()).toEqual('Query')
      expect(options.at(4).text()).toEqual('Method')
      expect(options.at(5).text()).toEqual('Company')
      expect(options.at(6).text()).toEqual('Authority')
    })

    test('should render dropdown correctly with all types if ignore is empty array', async () => {
      option = {
        type: 'attrs',
      }
      wrapper = mount(LimitOption, {
        propsData: {
          option: option,
          ignoreAttributes: [],
        },
      })
      await Vue.nextTick()
      const selection = wrapper.find('.option-attribute-selection')
      const options = selection.findAll('option')
      expect(options.at(0).text()).toEqual('IP Address')
      expect(options.at(1).text()).toEqual('Provider')
      expect(options.at(2).text()).toEqual('URI')
      expect(options.at(3).text()).toEqual('Path')
      expect(options.at(4).text()).toEqual('Tag')
      expect(options.at(5).text()).toEqual('Query')
      expect(options.at(6).text()).toEqual('Method')
      expect(options.at(7).text()).toEqual('Company')
      expect(options.at(8).text()).toEqual('Country')
      expect(options.at(9).text()).toEqual('Authority')
    })
  })

  describe('emit changes', () => {
    describe('type dropdown', () => {
      test('should emit new option when type selected from dropdown - self', async () => {
        const wantedEmit = {
          type: 'self',
          key: 'self',
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        // set to not self so we would be able to change to default
        options.at(1).setSelected()
        options.at(0).setSelected()
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[0]).toEqual([wantedEmit])
      })

      test('should emit new option when type selected from dropdown - headers', async () => {
        const wantedEmit = {
          type: 'headers',
          key: '',
          value: undefined as string,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(1).setSelected()
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[0]).toEqual([wantedEmit])
      })

      test('should emit new option when type selected from dropdown - cookies', async () => {
        const wantedEmit = {
          type: 'cookies',
          key: '',
          value: undefined as string,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(2).setSelected()
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[0]).toEqual([wantedEmit])
      })

      test('should emit new option when type selected from dropdown - args', async () => {
        const wantedEmit = {
          type: 'args',
          key: '',
          value: undefined as string,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(3).setSelected()
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[0]).toEqual([wantedEmit])
      })

      test('should emit new option when type selected from dropdown - attrs', async () => {
        const wantedEmit = {
          type: 'attrs',
          key: '',
          value: undefined as string,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(4).setSelected()
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[0]).toEqual([wantedEmit])
      })
    })

    describe('key changes', () => {
      test('should emit new option when key input changes - headers', async () => {
        const wantedKeyValue = 'foo'
        const wantedEmit = {
          type: 'headers',
          key: wantedKeyValue,
          oldKey: '',
          value: undefined as string,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(1).setSelected()
        await Vue.nextTick()
        const input = wrapper.find('.option-key-input')
        input.setValue(wantedKeyValue)
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[1]).toEqual([wantedEmit])
      })

      test('should emit new option when key selected from dropdown - cookies', async () => {
        const wantedKeyValue = 'foo'
        const wantedEmit = {
          type: 'cookies',
          key: wantedKeyValue,
          oldKey: '',
          value: undefined as string,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(2).setSelected()
        await Vue.nextTick()
        const input = wrapper.find('.option-key-input')
        input.setValue(wantedKeyValue)
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[1]).toEqual([wantedEmit])
      })

      test('should emit new option when key input changes - args', async () => {
        const wantedKeyValue = 'foo'
        const wantedEmit = {
          type: 'args',
          key: wantedKeyValue,
          oldKey: '',
          value: undefined as string,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(3).setSelected()
        await Vue.nextTick()
        const input = wrapper.find('.option-key-input')
        input.setValue(wantedKeyValue)
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[1]).toEqual([wantedEmit])
      })

      test('should emit new option when key selected from dropdown - attrs', async () => {
        const wantedEmit = {
          type: 'attrs',
          key: 'path',
          oldKey: '',
          value: undefined as string,
        }
        const typeSelection = wrapper.find('.option-type-selection')
        const typeOptions = typeSelection.findAll('option')
        typeOptions.at(4).setSelected()
        await Vue.nextTick()
        const keySelection = wrapper.find('.option-attribute-selection')
        const keyOptions = keySelection.findAll('option')
        keyOptions.at(3).setSelected()
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[1]).toEqual([wantedEmit])
      })
    })

    describe('value changes', () => {
      test('should emit new option when value input changes - headers', async () => {
        const wantedValue = 'bar'
        const wantedEmit = {
          type: 'headers',
          key: '',
          value: wantedValue,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(1).setSelected()
        await Vue.nextTick()
        const input = wrapper.find('.option-value-input')
        input.setValue(wantedValue)
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[1]).toEqual([wantedEmit])
      })

      test('should emit new option when value selected from dropdown - cookies', async () => {
        const wantedValue = 'bar'
        const wantedEmit = {
          type: 'cookies',
          key: '',
          value: wantedValue,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(2).setSelected()
        await Vue.nextTick()
        const input = wrapper.find('.option-value-input')
        input.setValue(wantedValue)
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[1]).toEqual([wantedEmit])
      })

      test('should emit new option when value input changes - args', async () => {
        const wantedValue = 'bar'
        const wantedEmit = {
          type: 'args',
          key: '',
          value: wantedValue,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(3).setSelected()
        await Vue.nextTick()
        const input = wrapper.find('.option-value-input')
        input.setValue(wantedValue)
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[1]).toEqual([wantedEmit])
      })

      test('should emit new option when value input changes - attrs', async () => {
        const wantedValue = 'bar'
        const wantedEmit = {
          type: 'attrs',
          key: '',
          value: wantedValue,
        }
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(4).setSelected()
        await Vue.nextTick()
        const input = wrapper.find('.option-value-input')
        input.setValue(wantedValue)
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[1]).toEqual([wantedEmit])
      })
    })

    describe('no option prop', () => {
      beforeEach(async () => {
        wrapper = mount(LimitOption, {
          propsData: {
            option: undefined,
            useValue: true,
          },
        })
        await Vue.nextTick()
        const selection = wrapper.find('.option-type-selection')
        const options = selection.findAll('option')
        options.at(2).setSelected()
        await wrapper.vm.$forceUpdate()
      })

      test('should emit type change correctly', () => {
        const wantedEmit = {
          type: 'args',
          key: '',
          value: undefined as string,
        }
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[0]).toEqual([wantedEmit])
      })

      test('should emit key change correctly', async () => {
        const wantedKeyValue = 'foo'
        const wantedEmit = {
          type: 'args',
          key: wantedKeyValue,
          oldKey: '',
          value: undefined as string,
        }
        const input = wrapper.find('.option-key-input')
        input.setValue(wantedKeyValue)
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[1]).toEqual([wantedEmit])
      })

      test('should emit value change correctly', async () => {
        const wantedValue = 'bar'
        const wantedEmit = {
          type: 'args',
          key: '',
          value: wantedValue,
        }
        const input = wrapper.find('.option-value-input')
        input.setValue(wantedValue)
        await Vue.nextTick()
        expect(wrapper.emitted('change')).toBeTruthy()
        expect(wrapper.emitted('change')[1]).toEqual([wantedEmit])
      })
    })

    describe('remove', () => {
      test('should emit remove correctly', async () => {
        wrapper = mount(LimitOption, {
          propsData: {
            option: option,
            showRemove: true,
            removable: true,
          },
        })
        await Vue.nextTick()
        const button = wrapper.find('.remove-option-button')
        button.trigger('click')
        await Vue.nextTick()
        expect(wrapper.emitted('remove')).toBeTruthy()
      })

      test('should not emit key change if removable prop is false', async () => {
        wrapper = mount(LimitOption, {
          propsData: {
            option: option,
            showRemove: true,
            removable: false,
          },
        })
        await Vue.nextTick()
        const button = wrapper.find('.remove-option-button')
        button.trigger('click')
        await Vue.nextTick()
        expect(wrapper.emitted('remove')).toBeFalsy()
      })
    })
  })
})
