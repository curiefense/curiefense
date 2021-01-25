import MasterComponent from '@/views/MasterComponent'
import {describe, test, expect, beforeEach} from '@jest/globals'
import {shallowMount} from '@vue/test-utils'

describe('MasterComponent.vue', () => {
    let wrapper
    beforeEach(() => {
        const $route = {
            path: '/config'
        }
        wrapper = shallowMount(MasterComponent, {
            mocks: {
                $route
            },
            stubs: ['router-link', 'router-view']
        })
    })

    test('should render side menu component', () => {
        const component = wrapper.find('side-menu')
        expect(component).toBeTruthy()
    })

    test('should render header component', () => {
        const component = wrapper.find('header-main')
        expect(component).toBeTruthy()
    })
})
