import ResponseAction from '@/components/ResponseAction.vue'
import {beforeEach, describe, expect, test} from '@jest/globals'
import {mount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'
import {ResponseActionType} from '@/types'

describe('ResponseAction.vue', () => {
  let action: ResponseActionType
  let wrapper: Wrapper<Vue>
  beforeEach(() => {
    action = {
      type: 'default',
    }
    wrapper = mount(ResponseAction, {
      propsData: {
        action: action,
      },
    })
  })

  test('should render dropdown correctly with all possible types as options', () => {
    const selection = wrapper.find('.action-type-selection')
    const options = selection.findAll('option')
    expect(options.at(0).text()).toEqual('503 Service Unavailable')
    expect(options.at(1).text()).toEqual('Challenge')
    expect(options.at(2).text()).toEqual('Tag Only')
    expect(options.at(3).text()).toEqual('Response')
    expect(options.at(4).text()).toEqual('Redirect')
    expect(options.at(5).text()).toEqual('Ban')
    expect(options.at(6).text()).toEqual('Header')
  })

  describe('label prop', () => {
    test('should render default label if no label provided', () => {
      wrapper = mount(ResponseAction, {
        propsData: {
          action: action,
        },
      })
      const label = wrapper.find('.form-label')
      expect(label.text()).toEqual('Action')
    })

    test('should render label', () => {
      const wantedLabel = 'Test'
      wrapper = mount(ResponseAction, {
        propsData: {
          action: action,
          label: wantedLabel,
        },
      })
      const label = wrapper.find('.form-label')
      expect(label.text()).toEqual(wantedLabel)
    })
  })

  describe('ignore prop', () => {
    test('should render dropdown correctly without ignored action types (default, ban)', () => {
      wrapper = mount(ResponseAction, {
        propsData: {
          action: action,
          ignore: ['default', 'ban'],
        },
      })
      const selection = wrapper.find('.action-type-selection')
      const options = selection.findAll('option')
      expect(options.at(0).text()).toEqual('Challenge')
      expect(options.at(1).text()).toEqual('Tag Only')
      expect(options.at(2).text()).toEqual('Response')
      expect(options.at(3).text()).toEqual('Redirect')
      expect(options.at(4).text()).toEqual('Header')
    })

    test('should render dropdown correctly without ignored action types (monitor, redirect, header)', () => {
      wrapper = mount(ResponseAction, {
        propsData: {
          action: action,
          ignore: ['monitor', 'redirect', 'request_header'],
        },
      })
      const selection = wrapper.find('.action-type-selection')
      const options = selection.findAll('option')
      expect(options.at(0).text()).toEqual('503 Service Unavailable')
      expect(options.at(1).text()).toEqual('Challenge')
      expect(options.at(2).text()).toEqual('Response')
      expect(options.at(3).text()).toEqual('Ban')
    })

    test('should render dropdown correctly with all types if ignore is empty array', () => {
      wrapper = mount(ResponseAction, {
        propsData: {
          action: action,
          ignore: [],
        },
      })
      const selection = wrapper.find('.action-type-selection')
      const options = selection.findAll('option')
      expect(options.at(0).text()).toEqual('503 Service Unavailable')
      expect(options.at(1).text()).toEqual('Challenge')
      expect(options.at(2).text()).toEqual('Tag Only')
      expect(options.at(3).text()).toEqual('Response')
      expect(options.at(4).text()).toEqual('Redirect')
      expect(options.at(5).text()).toEqual('Ban')
      expect(options.at(6).text()).toEqual('Header')
    })
  })

  describe('emit changes', () => {
    describe('type dropdown', () => {
      test('should emit new action when selected from dropdown - default', () => {
        const wantedEmit = {
          type: 'default',
        }
        const selection = wrapper.find('.action-type-selection')
        const options = selection.findAll('option')
        // set to not default so we would be able to change to default
        options.at(1).setSelected()
        options.at(0).setSelected()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when selected from dropdown - challenge', () => {
        const wantedEmit = {
          type: 'challenge',
        }
        const selection = wrapper.find('.action-type-selection')
        const options = selection.findAll('option')
        options.at(1).setSelected()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when selected from dropdown - monitor', () => {
        const wantedEmit = {
          type: 'monitor',
        }
        const selection = wrapper.find('.action-type-selection')
        const options = selection.findAll('option')
        options.at(2).setSelected()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when selected from dropdown - response', () => {
        const wantedEmit = {
          type: 'response',
          params: {
            status: '',
            content: '',
          },
        }
        const selection = wrapper.find('.action-type-selection')
        const options = selection.findAll('option')
        options.at(3).setSelected()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when selected from dropdown - redirect', () => {
        const wantedEmit = {
          type: 'redirect',
          params: {
            status: '',
            location: '',
          },
        }
        const selection = wrapper.find('.action-type-selection')
        const options = selection.findAll('option')
        options.at(4).setSelected()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when selected from dropdown - ban', () => {
        const wantedEmit = {
          type: 'ban',
          params: {
            duration: '',
            action: {
              type: 'default',
            },
          },
        }
        const selection = wrapper.find('.action-type-selection')
        const options = selection.findAll('option')
        options.at(5).setSelected()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when selected from dropdown - header', () => {
        const wantedEmit = {
          type: 'request_header',
          params: {
            headers: '',
          },
        }
        const selection = wrapper.find('.action-type-selection')
        const options = selection.findAll('option')
        options.at(6).setSelected()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should clear status when switching from redirect to response', async () => {
        const wantedEmit = {
          type: 'response',
          params: {
            status: '',
            content: '',
          },
        }
        action = {
          type: 'redirect',
          params: {
            status: '',
            location: '',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const statusInput = wrapper.find('.action-status')
        statusInput.setValue('301')
        statusInput.trigger('change')
        await Vue.nextTick()
        const selection = wrapper.find('.action-type-selection')
        const options = selection.findAll('option')
        options.at(3).setSelected()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should clear status when switching from response to redirect', async () => {
        const wantedEmit = {
          type: 'redirect',
          params: {
            status: '',
            location: '',
          },
        }
        action = {
          type: 'response',
          params: {
            status: '',
            content: '',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const statusInput = wrapper.find('.action-status')
        statusInput.setValue('301')
        statusInput.trigger('change')
        await Vue.nextTick()
        const selection = wrapper.find('.action-type-selection')
        const options = selection.findAll('option')
        options.at(4).setSelected()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })
    })

    describe('input change', () => {
      test('should emit new action when input changes - response status', async () => {
        const wantedEmit = {
          type: 'response',
          params: {
            status: '500',
            content: '',
          },
        }
        action = {
          type: 'response',
          params: {
            status: '',
            content: '',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const statusInput = wrapper.find('.action-status')
        statusInput.setValue(wantedEmit.params.status)
        statusInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - response content', async () => {
        const wantedEmit = {
          type: 'response',
          params: {
            status: '',
            content: '{"foo": "bar"}',
          },
        }
        action = {
          type: 'response',
          params: {
            status: '',
            content: '',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const contentInput = wrapper.find('.action-content')
        contentInput.setValue(wantedEmit.params.content)
        contentInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - redirect status', async () => {
        const wantedEmit = {
          type: 'redirect',
          params: {
            status: '500',
            location: '',
          },
        }
        action = {
          type: 'redirect',
          params: {
            status: '',
            location: '',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const statusInput = wrapper.find('.action-status')
        statusInput.setValue(wantedEmit.params.status)
        statusInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - redirect location', async () => {
        const wantedEmit = {
          type: 'redirect',
          params: {
            status: '',
            location: 'www.example.com',
          },
        }
        action = {
          type: 'redirect',
          params: {
            status: '',
            location: '',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const locationInput = wrapper.find('.action-location')
        locationInput.setValue(wantedEmit.params.location)
        locationInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - ban duration', async () => {
        const wantedEmit = {
          type: 'ban',
          params: {
            duration: '1800',
            action: {
              type: 'default',
            },
          },
        }
        action = {
          type: 'ban',
          params: {
            duration: '',
            action: {
              type: 'default',
            },
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const durationInput = wrapper.find('.action-duration')
        durationInput.setValue(wantedEmit.params.duration)
        durationInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - ban action', async () => {
        const wantedEmit = {
          type: 'ban',
          params: {
            duration: '',
            action: {
              type: 'monitor',
            },
          },
        }
        action = {
          type: 'ban',
          params: {
            duration: '',
            action: {
              type: 'default',
            },
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const responseActionComponent = wrapper.findAllComponents(ResponseAction).at(1)
        responseActionComponent.vm.$emit('update:action', wantedEmit.params.action)
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - header headers', async () => {
        const wantedEmit = {
          type: 'request_header',
          params: {
            headers: 'foo',
          },
        }
        action = {
          type: 'request_header',
          params: {
            headers: '',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const headersInput = wrapper.find('.action-headers')
        headersInput.setValue(wantedEmit.params.headers)
        headersInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })
    })

    describe('input change with predefined data', () => {
      test('should emit new action when input changes - response status', async () => {
        const wantedEmit = {
          type: 'response',
          params: {
            status: '500',
            content: '{"foo": "bar"}',
          },
        }
        action = {
          type: 'response',
          params: {
            status: '400',
            content: '{"foo": "bar"}',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const statusInput = wrapper.find('.action-status')
        statusInput.setValue(wantedEmit.params.status)
        statusInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - response content', async () => {
        const wantedEmit = {
          type: 'response',
          params: {
            status: '500',
            content: '{"foo": "bar"}',
          },
        }
        action = {
          type: 'response',
          params: {
            status: '500',
            content: '{"foo2": "bar2"}',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const contentInput = wrapper.find('.action-content')
        contentInput.setValue(wantedEmit.params.content)
        contentInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - redirect status', async () => {
        const wantedEmit = {
          type: 'redirect',
          params: {
            status: '500',
            location: 'www.example.com',
          },
        }
        action = {
          type: 'redirect',
          params: {
            status: '400',
            location: 'www.example.com',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const statusInput = wrapper.find('.action-status')
        statusInput.setValue(wantedEmit.params.status)
        statusInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - redirect location', async () => {
        const wantedEmit = {
          type: 'redirect',
          params: {
            status: '500',
            location: 'www.example.com',
          },
        }
        action = {
          type: 'redirect',
          params: {
            status: '500',
            location: 'example.com',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const locationInput = wrapper.find('.action-location')
        locationInput.setValue(wantedEmit.params.location)
        locationInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - ban duration', async () => {
        const wantedEmit = {
          type: 'ban',
          params: {
            duration: '1800',
            action: {
              type: 'challenge',
            },
          },
        }
        action = {
          type: 'ban',
          params: {
            duration: '600',
            action: {
              type: 'challenge',
            },
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const durationInput = wrapper.find('.action-duration')
        durationInput.setValue(wantedEmit.params.duration)
        durationInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - ban action', async () => {
        const wantedEmit = {
          type: 'ban',
          params: {
            duration: '1800',
            action: {
              type: 'monitor',
            },
          },
        }
        action = {
          type: 'ban',
          params: {
            duration: '1800',
            action: {
              type: 'default',
            },
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const responseActionComponent = wrapper.findAllComponents(ResponseAction).at(1)
        responseActionComponent.vm.$emit('update:action', wantedEmit.params.action)
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when input changes - header headers', async () => {
        const wantedEmit = {
          type: 'request_header',
          params: {
            headers: 'foo',
          },
        }
        action = {
          type: 'request_header',
          params: {
            headers: 'foo2',
          },
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        const headersInput = wrapper.find('.action-headers')
        headersInput.setValue(wantedEmit.params.headers)
        headersInput.trigger('change')
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })
    })

    describe('normalize prop', () => {
      test('should emit default action when given incomplete undefined action prop', async () => {
        const wantedEmit = {
          type: 'default',
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: undefined,
          },
        })
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when given incomplete action prop - response', async () => {
        const wantedEmit = {
          type: 'response',
          params: {
            status: '',
            content: '',
          },
        }
        action = {
          type: 'response',
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when given incomplete action prop - redirect', async () => {
        const wantedEmit = {
          type: 'redirect',
          params: {
            status: '',
            location: '',
          },
        }
        action = {
          type: 'redirect',
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when given incomplete action prop - ban', async () => {
        const wantedEmit = {
          type: 'ban',
          params: {
            duration: '',
            action: {
              type: 'default',
            },
          },
        }
        action = {
          type: 'ban',
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })

      test('should emit new action when given incomplete action prop - headers', async () => {
        const wantedEmit = {
          type: 'request_header',
          params: {
            headers: '',
          },
        }
        action = {
          type: 'request_header',
        }
        wrapper = mount(ResponseAction, {
          propsData: {
            action: action,
          },
        })
        await Vue.nextTick()
        expect(wrapper.emitted('update:action')).toBeTruthy()
        expect(wrapper.emitted('update:action')[0]).toEqual([wantedEmit])
      })
    })
  })
})
