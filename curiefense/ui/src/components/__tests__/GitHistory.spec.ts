import GitHistory from '@/components/GitHistory.vue'
import {beforeEach, describe, expect, test} from '@jest/globals'
import {mount, Wrapper} from '@vue/test-utils'
import {Commit} from '@/types'
import Vue from 'vue'

describe('GitHistory.vue', () => {
  // Number of log items = 7
  const initialGitLog = [
    {
      'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
      'date': '2020-11-10T15:49:17+02:00',
      'parents': [
        'fc47a6cd9d7f254dd97875a04b87165cc484e075',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API',
    },
    {
      'version': 'fc47a6cd9d7f254dd97875a04b87165cc484e075',
      'date': '2020-11-10T15:48:35+02:00',
      'parents': [
        '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API',
    },
    {
      'version': '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
      'date': '2020-11-10T15:48:31+02:00',
      'parents': [
        '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API',
    },
    {
      'version': '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
      'date': '2020-11-10T15:48:22+02:00',
      'parents': [
        '878b47deeddac94625fe7c759786f2df885ec541',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API',
    },
    {
      'version': '878b47deeddac94625fe7c759786f2df885ec541',
      'date': '2020-11-10T15:48:05+02:00',
      'parents': [
        '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API',
    },
    {
      'version': '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
      'date': '2020-11-10T15:47:59+02:00',
      'parents': [
        '1662043d2a18d6ad2c9c94d6f826593ff5506354',
      ],
      'message': 'Update entry [__default__] of document [aclpolicies]',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API',
    },
    {
      'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
      'date': '2020-11-08T21:31:41+01:00',
      'parents': [
        '16379cdf39501574b4a2f5a227b82a4454884b84',
      ],
      'message': 'Create config [master]\n',
      'email': 'curiefense@reblaze.com',
      'author': 'Curiefense API',
    },
  ]
  let gitLog = initialGitLog
  const apiPath = '/conf/api/v1/configs/master/d/aclpolicies/e/__default__/v/'
  let wrapper: Wrapper<Vue>
  beforeEach(() => {
    gitLog = initialGitLog
    const newVersions = [
      'b88b46ac16348d86e38cc1ba50686cde6a9c78c1',
      '1a172f464b73d62ccbc0f545732dc527ec0125d4',
      'a6f7a5d907fa0cd2340a393e3f57624d6c5ce1cf',
      'b280dead30cb7cf9024493f4bc85d499dc3b67ed',
    ]
    const onRestoreVersion = (commit: Commit) => {
      const newGitLog = JSON.parse(JSON.stringify(gitLog))
      const newCommit = JSON.parse(JSON.stringify(commit))
      newCommit.version = newVersions.pop()
      newCommit.parents = [newGitLog[0].version]
      newCommit.message = 'updated for tests'
      newGitLog.unshift(newCommit)
      wrapper.setProps({gitLog: newGitLog})
      gitLog = newGitLog
    }
    wrapper = mount(GitHistory, {
      propsData: {
        gitLog,
        apiPath,
      },
      listeners: {
        'restore-version': onRestoreVersion,
      },
    })
  })

  describe('log table rendering', () => {
    test('should only render five rows in addition to header and footer' +
      'if the log has more than 5 rows of data', () => {
      expect(wrapper.findAll('tr').length).toEqual(7)
    })

    test('should render all rows if table expanded in addition to header and footer', async () => {
      await wrapper.setData({expanded: true})
      expect(wrapper.findAll('tr').length).toEqual(9)
    })

    test('should render footer with expand message' +
      'if table is not expanded and more than five items are present', () => {
      const lastRow = wrapper.findAll('tr').at(wrapper.findAll('tr').length - 1)
      expect(lastRow.text()).toEqual('View More')
    })

    test('should render footer with collapse message' +
      'if table is expanded and more than five items are present', async () => {
      await wrapper.setData({expanded: true})
      const lastRow = wrapper.findAll('tr').at(wrapper.findAll('tr').length - 1)
      expect(lastRow.text()).toEqual('View Less')
    })

    test('should not render footer if less than five items are present', async () => {
      const shortGitLog = gitLog.slice(0, 4)
      wrapper = mount(GitHistory, {
        propsData: {
          gitLog: shortGitLog,
          apiPath,
        },
      })
      const lastRow = wrapper.findAll('tr').at(wrapper.findAll('tr').length - 1)
      expect(lastRow.text()).not.toEqual('View More')
      expect(lastRow.text()).not.toEqual('View Less')
    })
  })

  describe('log version restoration', () => {
    test('should not render restore button when not hovering over a row', () => {
      expect(wrapper.findAll('.restore-button').length).toEqual(0)
    })

    test('should render a single restore button when hovering over a row', async () => {
      const firstDataRow = wrapper.findAll('.version-row').at(0)
      await firstDataRow.trigger('mouseover')
      expect(firstDataRow.findAll('.restore-button').length).toEqual(1)
    })

    test('should stop rendering the restore button when no longer hovering over a row', async () => {
      const firstDataRow = wrapper.findAll('.version-row').at(0)
      await firstDataRow.trigger('mouseover')
      await firstDataRow.trigger('mouseleave')
      expect(firstDataRow.findAll('.restore-button').length).toEqual(0)
    })

    test('should emit a restore-version event when restore button is clicked', async () => {
      const firstDataRow = wrapper.findAll('.version-row').at(0)
      await firstDataRow.trigger('mouseover')
      const restoreButton = firstDataRow.find('.restore-button')
      await restoreButton.trigger('click')
      expect(wrapper.emitted('restore-version')).toBeTruthy()
      expect(wrapper.emitted('restore-version')[0]).toEqual([gitLog[1]])
    })
  })

  describe('undoing log version restoration', () => {
    test('should not render undo button when no version has been restored yet', () => {
      expect(wrapper.findAll('.undo-restore-button').length).toEqual(0)
    })

    describe('after a few versions had been restored', () => {
      let timesRestored = 0
      beforeEach(async () => {
        timesRestored = 0
        let dataRow = wrapper.findAll('.version-row').at(1)
        await dataRow.trigger('mouseover')
        let restoreButton = dataRow.find('.restore-button')
        await restoreButton.trigger('click')
        timesRestored++
        await wrapper.vm.$forceUpdate()

        // 2 + timesRestored = 3 = location 2 in the original gitLog
        dataRow = wrapper.findAll('.version-row').at(2 + timesRestored)
        await dataRow.trigger('mouseover')
        restoreButton = dataRow.find('.restore-button')
        await restoreButton.trigger('click')
        timesRestored++
        await wrapper.vm.$forceUpdate()
      })

      test('should render a single undo button', async () => {
        expect(wrapper.findAll('.undo-restore-button').length).toEqual(1)
      })

      test('should stop rendering the undo version restore button when gitLog is set to an empty array', async () => {
        wrapper.setProps({gitLog: []})
        await Vue.nextTick()
        expect(wrapper.findAll('.undo-restore-button').length).toEqual(0)
      })

      test('should emit a restore-version event with correct version when undo button is clicked', async () => {
        const restoreButton = wrapper.find('.undo-restore-button')
        await restoreButton.trigger('click')
        timesRestored++
        // 0 - current version after undo (undid restoration. Moved from version at 1, to version at 2)
        // 1 - version after second restoration (restored from 5)
        // 2 - version after first restoration (restored from 4)
        // 3 - original version at the start of the tests
        // 4 - original version in gitLog[1] (restored to 2)
        // 5 - original version in gitLog[2] (restored to 1)
        expect(wrapper.emitted('restore-version')).toBeTruthy()
        expect(wrapper.emitted('restore-version')[timesRestored - 1]).toEqual([gitLog[2]])
      })

      test('should emit a restore-version events correctly when undo button is clicked multiple times', async () => {
        const restoreButton = wrapper.find('.undo-restore-button')
        await restoreButton.trigger('click')
        timesRestored++
        // 0 - current version after undo (undid restoration. Moved from version at 1, to version at 2)
        // 1 - version after second restoration (restored from 5)
        // 2 - version after first restoration (restored from 4)
        // 3 - original version at the start of the tests
        // 4 - original version in gitLog[1] (restored to 2)
        // 5 - original version in gitLog[2] (restored to 1)
        expect(wrapper.emitted('restore-version')).toBeTruthy()
        expect(wrapper.emitted('restore-version')[timesRestored - 1]).toEqual([gitLog[2]])
        await restoreButton.trigger('click')
        timesRestored++
        // 0 - current version after second undo (undid restoration. Moved from version at 3, to version at 4)
        // 1 - current version after first undo (undid restoration. Moved from version at 2, to version at 3)
        // 2 - version after second restoration (restored from 6)
        // 3 - version after first restoration (restored from 5)
        // 4 - original version at the start of the tests
        // 5 - original version in gitLog[1] (restored to 3)
        // 6 - original version in gitLog[2] (restored to 2)
        expect(wrapper.emitted('restore-version')).toBeTruthy()
        expect(wrapper.emitted('restore-version')[timesRestored - 1]).toEqual([gitLog[4]])
      })

      test('should stop rendering the undo version restore button when undid all known version restores', async () => {
        const restoreButton = wrapper.find('.undo-restore-button')
        await restoreButton.trigger('click')
        await restoreButton.trigger('click')
        expect(wrapper.findAll('.undo-restore-button').length).toEqual(0)
      })
    })
  })
})
