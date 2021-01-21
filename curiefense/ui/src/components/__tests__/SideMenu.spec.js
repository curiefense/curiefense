import SideMenu from '@/components/SideMenu'
import {describe, test, expect, beforeEach} from '@jest/globals'
import {mount} from '@vue/test-utils'

describe('SideMenu.vue', () => {
    let wrapper
    beforeEach(() => {
        const $route = {
            path: '/config'
        }
        wrapper = mount(SideMenu, {
            mocks: {
                $route
            },
            stubs: ['router-link', 'router-view']
        })
    })

    test('should render all menu labels', () => {
        const wantedMenuLabels = ['Settings', 'Analytics', 'Git']

        const actualMenuLabels = wrapper.findAll('.menu-label')
        for (let i = 0; i < wantedMenuLabels.length; i++) {
            expect(actualMenuLabels.at(i).text()).toEqual(wantedMenuLabels[i])
        }
    })

    function menuItemShouldContainWantedSectionItems(menuItemName, wantedSectionItems) {
        const menuItem = wrapper
            .findAll('.menu-item')
            .filter(item => item.text().includes(menuItemName))
        const sectionItems = menuItem.at(0).findAll('.section-item')
        for (let i = 0; i < wantedSectionItems.length; i++) {
            expect(sectionItems.at(i).text()).toContain(wantedSectionItems[i].title)
            if (wantedSectionItems[i].external) {
                expect(sectionItems.at(i).html()).toContain(`href="${wantedSectionItems[i].path}"`)
            } else {
                expect(sectionItems.at(i).html()).toContain(`to="${wantedSectionItems[i].path}"`)
            }
        }
    }

    test('should render all Configuration menu items', () => {
        const wantedInternalMenuItems = [
            {path: '/config', title: 'Policies & Rules'},
            {path: '/db', title: 'System DB'},
            {path: '/publish', title: 'Publish Changes'},
            {path: `${location.protocol}//${location.hostname}:30000/api/v1/`, title: 'API', external: true},
        ]

        menuItemShouldContainWantedSectionItems('Settings', wantedInternalMenuItems)
    })

    test('should render all Analytics menu items', () => {
        const wantedMenuItems = [
            {path: `${location.protocol}//${location.hostname}:5601/app/discover`, title: 'Access Log (ELK)', external: true},
            {path: `${location.protocol}//${location.hostname}:30300/`, title: 'Grafana', external: true}
        ]

        menuItemShouldContainWantedSectionItems('Analytics', wantedMenuItems)
    })

    test('should render all Git menu items', () => {
        const wantedMenuItems = [
            {path: '/versioncontrol', title: 'Version Control'},
        ]

        menuItemShouldContainWantedSectionItems('Git', wantedMenuItems)
    })

    test('should render all Docs menu items', () => {
        const wantedMenuItems = [
            {path: 'https://docs.curiefense.io/', title: 'Curiebook', external: true},
        ]

        menuItemShouldContainWantedSectionItems('Docs', wantedMenuItems)
    })
})
