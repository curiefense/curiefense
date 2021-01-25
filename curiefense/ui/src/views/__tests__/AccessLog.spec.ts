import AccessLog from '@/views/AccessLog'
import {describe, test, expect} from '@jest/globals'
import {mount} from '@vue/test-utils'

describe('AccessLog.vue', () => {
    test('should exist - STUB', () => {
        const wrapper = mount(AccessLog)
        expect(wrapper).toBeTruthy()
    })
})
