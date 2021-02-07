import ResponseAction from '@/components/ResponseAction.vue'
import {describe, test, expect} from '@jest/globals'
import {mount} from '@vue/test-utils'

describe('ResponseAction.vue', () => {
  test('should exist - STUB', () => {
    const wrapper = mount(ResponseAction, {
      propsData: {
        action: {},
      },
    })
    expect(wrapper).toBeTruthy()
  })
})
