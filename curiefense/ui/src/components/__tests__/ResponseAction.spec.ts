import ResponseAction from '@/components/ResponseAction'
import {describe, test, expect} from '@jest/globals'
import {mount} from '@vue/test-utils'

describe('ResponseAction.vue', () => {
    test('should exist - STUB', () => {
        const wrapper = mount(ResponseAction, {
            propsData: {
                objectWithAction: {
                    action: {}
                }
            }
        })
        expect(wrapper).toBeTruthy()
    })
})
