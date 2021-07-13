import SideMenu from '@/components/SideMenu.vue'
import {beforeEach, describe, expect, jest, test} from '@jest/globals'
import {mount, Wrapper} from '@vue/test-utils'
import axios from 'axios'
import Vue from 'vue'

jest.mock('axios')

describe('SideMenu.vue', () => {
  let wrapper: Wrapper<Vue>
  let $route: any
  let swaggerURL: string
  let kibanaURL: string
  let grafanaURL: string
  let prometheusURL: string
  beforeEach(() => {
    $route = {
      path: '/config',
    }
    swaggerURL = 'https://10.0.0.1:30000/api/v1/'
    kibanaURL = 'https://10.0.0.1:5601/app/discover/'
    grafanaURL = 'https://10.0.0.1:30300/'
    prometheusURL = 'https://10.0.0.1:9090/'
    const dbData = {
      links: {
        kibana_url: kibanaURL,
        grafana_url: grafanaURL,
        swagger_url: swaggerURL,
        prometheus_url: prometheusURL,
      },
    }
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === `/conf/api/v1/db/system/`) {
        return Promise.resolve({data: dbData})
      }
      return Promise.resolve({data: {}})
    })
    wrapper = mount(SideMenu, {
      mocks: {
        $route,
      },
      stubs: ['router-link', 'router-view'],
    })
  })

  test('should render all menu labels', () => {
    const wantedMenuLabels = ['settings', 'analytics', 'git', 'docs']

    const actualMenuLabels = wrapper.findAll('.menu-label')
    for (let i = 0; i < wantedMenuLabels.length; i++) {
      expect(actualMenuLabels.at(i).text()).toEqual(wantedMenuLabels[i])
    }
  })

  function menuItemShouldContainWantedSectionItems(menuItemName: string, wantedSectionItems: any[]) {
    const menuItem = wrapper.findAll('.menu-item').filter((item) => item.text().includes(menuItemName))
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

  test('should render all Configuration menu items when db key does not exist', () => {
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === `/conf/api/v1/db/system/`) {
        return Promise.resolve({data: {}})
      }
      return Promise.resolve({data: {}})
    })
    wrapper = mount(SideMenu, {
      mocks: {
        $route,
      },
      stubs: ['router-link', 'router-view'],
    })
    const wantedInternalMenuItems = [
      {path: '/config', title: 'Policies & Rules'},
      {path: '/db', title: 'Databases'},
      {path: '/publish', title: 'Publish Changes'},
      {path: `${location.protocol}//${location.hostname}:30000/api/v1/`, title: 'API', external: true},
    ]

    menuItemShouldContainWantedSectionItems('settings', wantedInternalMenuItems)
  })

  test('should render all Configuration menu items when db key exists', () => {
    const wantedInternalMenuItems = [
      {path: '/config', title: 'Policies & Rules'},
      {path: '/db', title: 'Databases'},
      {path: '/publish', title: 'Publish Changes'},
      {path: swaggerURL, title: 'API', external: true},
    ]

    menuItemShouldContainWantedSectionItems('settings', wantedInternalMenuItems)
  })

  test('should render all Analytics menu items when db key does not exist', async () => {
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === `/conf/api/v1/db/system/`) {
        return Promise.resolve({data: {}})
      }
      return Promise.resolve({data: {}})
    })
    wrapper = mount(SideMenu, {
      mocks: {
        $route,
      },
      stubs: ['router-link', 'router-view'],
    })
    const wantedMenuItems = [
      {
        path: `${location.protocol}//${location.hostname}:5601/app/discover`,
        title: 'Kibana',
        external: true,
      },
      {
        path: `${location.protocol}//${location.hostname}:30300/`,
        title: 'Grafana',
        external: true,
      },
      {
        path: `${location.protocol}//${location.hostname}:9090/`,
        title: 'Prometheus',
        external: true,
      },
    ]
    await Vue.nextTick()
    await Vue.nextTick()
    menuItemShouldContainWantedSectionItems('analytics', wantedMenuItems)
  })

  test('should render all Analytics menu items when db key exists', async () => {
    wrapper = mount(SideMenu, {
      mocks: {
        $route,
      },
      stubs: ['router-link', 'router-view'],
    })
    await Vue.nextTick()
    await Vue.nextTick()
    const wantedMenuItems = [
      {
        path: kibanaURL,
        title: 'Kibana',
        external: true,
      },
      {
        path: grafanaURL,
        title: 'Grafana',
        external: true,
      },
      {
        path: prometheusURL,
        title: 'Prometheus',
        external: true,
      },
    ]

    menuItemShouldContainWantedSectionItems('analytics', wantedMenuItems)
  })

  test('should render all Git menu items', () => {
    const wantedMenuItems = [
      {path: '/versioncontrol', title: 'Version Control'},
    ]

    menuItemShouldContainWantedSectionItems('git', wantedMenuItems)
  })

  test('should render all Docs menu items', () => {
    const wantedMenuItems = [
      {path: 'https://docs.curiefense.io/', title: 'Curiebook', external: true},
    ]

    menuItemShouldContainWantedSectionItems('docs', wantedMenuItems)
  })
})
