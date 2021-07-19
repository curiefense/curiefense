import VersionControl from '@/views/VersionControl.vue'
import GitHistory from '@/components/GitHistory.vue'
import Utils from '@/assets/Utils'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {mount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'
import axios from 'axios'
import {Branch} from '@/types'

jest.mock('axios')

describe('VersionControl.vue', () => {
  let wrapper: Wrapper<Vue>
  let gitData: Branch[]
  beforeEach(() => {
    gitData = [
      {
        'id': 'master',
        'description': 'Update entry [__default__] of document [aclprofiles]',
        'date': '2020-11-10T15:49:17+02:00',
        'logs': [
          {
            'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
            'date': '2020-11-10T15:49:17+02:00',
            'parents': [
              'fc47a6cd9d7f254dd97875a04b87165cc484e075',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': 'fc47a6cd9d7f254dd97875a04b87165cc484e075',
            'date': '2020-11-10T15:48:35+02:00',
            'parents': [
              '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '5aba4a5b9d6faea1896ee8965c7aa651f76af63c',
            'date': '2020-11-10T15:48:31+02:00',
            'parents': [
              '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '277c5d7bd0e2eb4b9d2944f7eefdfadf37ba8581',
            'date': '2020-11-10T15:48:22+02:00',
            'parents': [
              '878b47deeddac94625fe7c759786f2df885ec541',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '878b47deeddac94625fe7c759786f2df885ec541',
            'date': '2020-11-10T15:48:05+02:00',
            'parents': [
              '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': '93c180513fe7edeaf1c0ca69a67aa2a11374da4f',
            'date': '2020-11-10T15:47:59+02:00',
            'parents': [
              '1662043d2a18d6ad2c9c94d6f826593ff5506354',
            ],
            'message': 'Update entry [__default__] of document [aclprofiles]',
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
          {
            'version': '16379cdf39501574b4a2f5a227b82a4454884b84',
            'date': '2020-08-27T16:19:06+00:00',
            'parents': [
              'a34f979217215060861b58b3f270e82580c20efb',
            ],
            'message': 'Initial empty config',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
          {
            'version': 'a34f979217215060861b58b3f270e82580c20efb',
            'date': '2020-08-27T16:19:06+00:00',
            'parents': [],
            'message': 'Initial empty content',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
        ],
        'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
      },
      {
        'id': 'zzz_branch',
        'description': 'Initial empty content',
        'date': '2020-08-27T16:19:06+00:00',
        'logs': [
          {
            'version': 'a34f979217215060861b58b3f270e82580c20efb',
            'date': '2020-08-27T16:19:06+00:00',
            'parents': [],
            'message': 'Initial empty content',
            'email': 'curiefense@reblaze.com',
            'author': 'Curiefense API',
          },
        ],
        'version': 'a34f979217215060861b58b3f270e82580c20efb',
      },
    ]
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/configs/') {
        return Promise.resolve({data: gitData})
      }
      if (path === '/conf/api/v2/configs/master/') {
        return Promise.resolve({data: gitData[0]})
      }
      if (path === '/conf/api/v2/configs/zzz_branch/') {
        return Promise.resolve({data: gitData[1]})
      }
      if (path === '/conf/api/v2/configs/master/v/') {
        return Promise.resolve({data: gitData[0].logs})
      }
      if (path === '/conf/api/v2/configs/zzz_branch/v/') {
        return Promise.resolve({data: gitData[1].logs})
      }
      return Promise.resolve({data: []})
    })
    wrapper = mount(VersionControl)
  })
  afterEach(() => {
    jest.clearAllMocks()
  })

  test('should have a git history component', () => {
    const gitHistory = wrapper.findComponent(GitHistory)
    expect(gitHistory).toBeTruthy()
  })

  test('should display correct zero amount of branches', (done) => {
    gitData = []
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/configs/') {
        return Promise.resolve({data: gitData})
      }
      return Promise.resolve({data: []})
    })
    wrapper = mount(VersionControl)
    // allow all requests to finish
    setImmediate(() => {
      const gitBranches = wrapper.find('.git-branches')
      expect(gitBranches.text()).toEqual('0 branches')
      done()
    })
  })

  test('should display correct zero amount of commits', (done) => {
    gitData = []
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/configs/') {
        return Promise.resolve({data: gitData})
      }
      return Promise.resolve({data: []})
    })
    wrapper = mount(VersionControl)
    // allow all requests to finish
    setImmediate(() => {
      const gitCommits = wrapper.find('.git-commits')
      expect(gitCommits.text()).toEqual('0 commits')
      done()
    })
  })

  test('should display correct singular amount of branches', (done) => {
    gitData = [
      {
        'id': 'master',
        'description': 'Update entry [__default__] of document [aclprofiles]',
        'date': '2020-11-10T15:49:17+02:00',
        'logs': [{
          'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
          'date': '2020-11-10T15:49:17+02:00',
          'parents': [
            'fc47a6cd9d7f254dd97875a04b87165cc484e075',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        }],
        'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
      },
    ]
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/configs/') {
        return Promise.resolve({data: gitData})
      }
      if (path === '/conf/api/v2/configs/master/') {
        return Promise.resolve({data: gitData[0]})
      }
      if (path === '/conf/api/v2/configs/master/v/') {
        return Promise.resolve({data: gitData[0].logs})
      }
      return Promise.resolve({data: []})
    })
    wrapper = mount(VersionControl)
    // allow all requests to finish
    setImmediate(() => {
      const gitBranches = wrapper.find('.git-branches')
      expect(gitBranches.text()).toEqual('1 branch')
      done()
    })
  })

  test('should display correct singular amount of commits', (done) => {
    gitData = [
      {
        'id': 'master',
        'description': 'Update entry [__default__] of document [aclprofiles]',
        'date': '2020-11-10T15:49:17+02:00',
        'logs': [{
          'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
          'date': '2020-11-10T15:49:17+02:00',
          'parents': [
            'fc47a6cd9d7f254dd97875a04b87165cc484e075',
          ],
          'message': 'Update entry [__default__] of document [aclprofiles]',
          'email': 'curiefense@reblaze.com',
          'author': 'Curiefense API',
        }],
        'version': '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
      },
    ]
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/configs/') {
        return Promise.resolve({data: gitData})
      }
      if (path === '/conf/api/v2/configs/master/') {
        return Promise.resolve({data: gitData[0]})
      }
      if (path === '/conf/api/v2/configs/master/v/') {
        return Promise.resolve({data: gitData[0].logs})
      }
      return Promise.resolve({data: []})
    })
    wrapper = mount(VersionControl)
    // allow all requests to finish
    setImmediate(() => {
      const gitCommits = wrapper.find('.git-commits')
      expect(gitCommits.text()).toEqual('1 commit')
      done()
    })
  })

  test('should display correct plural amount of branches', () => {
    const gitBranches = wrapper.find('.git-branches')
    expect(gitBranches.text()).toEqual('2 branches')
  })

  test('should display correct plural amount of commits', () => {
    const gitCommits = wrapper.find('.git-commits')
    expect(gitCommits.text()).toEqual('10 commits')
  })

  test('should be able to switch branches through dropdown', (done) => {
    const branchSelection = wrapper.find('.branch-selection')
    branchSelection.trigger('click')
    const options = branchSelection.findAll('option')
    options.at(1).setSelected()
    // allow all requests to finish
    setImmediate(() => {
      expect((wrapper.vm as any).selectedBranch).toEqual(gitData[1].id)
      done()
    })
  })

  test('should have correct git log displayed after switching branches', async (done) => {
    const branchSelection = wrapper.find('.branch-selection')
    branchSelection.trigger('click')
    const options = branchSelection.findAll('option')
    options.at(1).setSelected()
    // allow all requests to finish
    setImmediate(() => {
      const gitHistory = wrapper.findComponent(GitHistory)
      expect(gitHistory.props('gitLog')).toEqual(gitData[1].logs)
      done()
    })
  })

  test('should have fork branch input be hidden on init', async () => {
    const forkBranchNameInput = wrapper.find('.fork-branch-input')
    expect(forkBranchNameInput.element).toBeUndefined()
  })

  test('should have delete branch input be hidden on init', async () => {
    const deleteBranchNameInput = wrapper.find('.delete-branch-input')
    expect(deleteBranchNameInput.element).toBeUndefined()
  })

  test('should send API request to restore to the correct version', async () => {
    const wantedVersion = {
      version: '7dd9580c00bef1049ee9a531afb13db9ef3ee956',
    }
    const putSpy = jest.spyOn(axios, 'put')
    putSpy.mockImplementation(() => Promise.resolve())
    const gitHistory = wrapper.findComponent(GitHistory)
    gitHistory.vm.$emit('restore-version', wantedVersion)
    await Vue.nextTick()
    expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/configs/master/v/${wantedVersion.version}/revert/`)
  })

  test('should attempt to download branch when download button is clicked', async () => {
    const wantedFileName = 'master'
    const wantedFileType = 'json'
    const wantedFileData = gitData[0]
    const downloadFileSpy = jest.spyOn(Utils, 'downloadFile')
    // force update because downloadFile is mocked after it is read to to be used as event handler
    await wrapper.vm.$forceUpdate()
    await Vue.nextTick()
    const downloadBranchButton = wrapper.find('.download-branch-button')
    downloadBranchButton.trigger('click')
    await Vue.nextTick()
    expect(downloadFileSpy).toHaveBeenCalledWith(wantedFileName, wantedFileType, wantedFileData)
  })

  test('should not attempt to download branch when download button is clicked while loading is true', async () => {
    const downloadFileSpy = jest.spyOn(Utils, 'downloadFile')
    // force update because downloadFile is mocked after it is read to to be used as event handler
    await wrapper.vm.$forceUpdate()
    await Vue.nextTick()
    wrapper.setData({isDownloadLoading: true})
    const downloadBranchButton = wrapper.find('.download-branch-button')
    downloadBranchButton.trigger('click')
    await Vue.nextTick()
    expect(downloadFileSpy).not.toHaveBeenCalled()
  })

  describe('fork branch', () => {
    let postSpy: any
    let originalError: any
    beforeEach(async () => {
      originalError = console.error
      let consoleOutput: string[] = []
      const mockedError = (output: string) => consoleOutput.push(output)
      consoleOutput = []
      console.error = mockedError
      postSpy = jest.spyOn(axios, 'post').mockImplementation(() => Promise.resolve({data: {}}))
      const forkBranchIcon = wrapper.find('.fork-branch-toggle')
      forkBranchIcon.trigger('click')
      await Vue.nextTick()
    })
    afterEach(() => {
      console.error = originalError
      jest.clearAllMocks()
    })

    test('should be visible if toggled on', async () => {
      const forkBranchNameInput = wrapper.find('.fork-branch-input')
      expect(forkBranchNameInput.element).toBeDefined()
    })

    test('should be hidden if toggled off', async () => {
      const forkBranchIcon = wrapper.find('.fork-branch-toggle')
      forkBranchIcon.trigger('click')
      await Vue.nextTick()
      const forkBranchNameInput = wrapper.find('.fork-branch-input')
      expect(forkBranchNameInput.element).toBeUndefined()
    })

    test('should be able to fork if name does not exist and does not have spaces', async () => {
      const newBranchName = 'new_branch'
      const forkBranchNameInput = wrapper.find('.fork-branch-input')
      const forkBranchSaveButton = wrapper.find('.fork-branch-confirm')
      forkBranchNameInput.setValue(newBranchName)
      await Vue.nextTick()
      forkBranchSaveButton.trigger('click')
      await Vue.nextTick()
      expect(postSpy).toHaveBeenCalledWith(`/conf/api/v2/configs/master/clone/${newBranchName}/`, {
        'description': 'string',
        'id': 'string',
      })
    })

    test('should not be able to fork if name is empty', async () => {
      const forkBranchNameInput = wrapper.find('.fork-branch-input')
      const forkBranchSaveButton = wrapper.find('.fork-branch-confirm')
      forkBranchNameInput.setValue('')
      await Vue.nextTick()
      forkBranchSaveButton.trigger('click')
      await Vue.nextTick()
      expect(postSpy).not.toHaveBeenCalled()
    })

    test('should not be able to fork if name already exists', async () => {
      const forkBranchNameInput = wrapper.find('.fork-branch-input')
      const forkBranchSaveButton = wrapper.find('.fork-branch-confirm')
      forkBranchNameInput.setValue('zzz_branch')
      await Vue.nextTick()
      forkBranchSaveButton.trigger('click')
      await Vue.nextTick()
      expect(postSpy).not.toHaveBeenCalled()
    })

    test('should not be able to fork if name contains spaces', async () => {
      const forkBranchNameInput = wrapper.find('.fork-branch-input')
      const forkBranchSaveButton = wrapper.find('.fork-branch-confirm')
      forkBranchNameInput.setValue('a new branch name')
      await Vue.nextTick()
      forkBranchSaveButton.trigger('click')
      await Vue.nextTick()
      expect(postSpy).not.toHaveBeenCalled()
    })

    test('should be hidden if forked successfully', async () => {
      const newBranchName = 'new_branch'
      let forkBranchNameInput = wrapper.find('.fork-branch-input')
      const forkBranchSaveButton = wrapper.find('.fork-branch-confirm')
      forkBranchNameInput.setValue(newBranchName)
      await Vue.nextTick()
      forkBranchSaveButton.trigger('click')
      // process click
      await Vue.nextTick()
      // process API (fake) return
      await Vue.nextTick()
      forkBranchNameInput = wrapper.find('.fork-branch-input')
      expect(forkBranchNameInput.element).toBeUndefined()
    })

    test('should be visible if fork failed', async () => {
      postSpy.mockImplementation(() => Promise.reject(new Error()))
      const newBranchName = 'new_branch'
      let forkBranchNameInput = wrapper.find('.fork-branch-input')
      const forkBranchSaveButton = wrapper.find('.fork-branch-confirm')
      forkBranchNameInput.setValue(newBranchName)
      await Vue.nextTick()
      forkBranchSaveButton.trigger('click')
      await Vue.nextTick()
      forkBranchNameInput = wrapper.find('.fork-branch-input')
      expect(forkBranchNameInput.element).toBeDefined()
    })
  })

  describe('delete branch', () => {
    let deleteSpy: any
    let originalError: any
    beforeEach(async () => {
      originalError = console.error
      let consoleOutput: string[] = []
      const mockedError = (output: string) => consoleOutput.push(output)
      consoleOutput = []
      console.error = mockedError
      deleteSpy = jest.spyOn(axios, 'delete').mockImplementation(() => Promise.resolve())
      const deleteBranchIcon = wrapper.find('.delete-branch-toggle')
      deleteBranchIcon.trigger('click')
      await Vue.nextTick()
    })
    afterEach(() => {
      console.error = originalError
      jest.clearAllMocks()
    })

    test('should be visible if toggled on', async () => {
      const deleteBranchNameInput = wrapper.find('.delete-branch-input')
      expect(deleteBranchNameInput.element).toBeDefined()
    })

    test('should be hidden if toggled off', async () => {
      const deleteBranchIcon = wrapper.find('.delete-branch-toggle')
      deleteBranchIcon.trigger('click')
      await Vue.nextTick()
      const deleteBranchNameInput = wrapper.find('.delete-branch-input')
      expect(deleteBranchNameInput.element).toBeUndefined()
    })

    test('should be able to delete if name matches current branch name', async () => {
      const currentBranchName = (wrapper.vm as any).selectedBranch
      const deleteBranchNameInput = wrapper.find('.delete-branch-input')
      const deleteBranchSaveButton = wrapper.find('.delete-branch-confirm')
      deleteBranchNameInput.setValue(currentBranchName)
      await Vue.nextTick()
      deleteBranchSaveButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).toHaveBeenCalledWith(`/conf/api/v2/configs/${currentBranchName}/`)
    })

    test('should not be able to delete if name is empty', async () => {
      const deleteBranchNameInput = wrapper.find('.delete-branch-input')
      const deleteBranchSaveButton = wrapper.find('.delete-branch-confirm')
      deleteBranchNameInput.setValue('')
      await Vue.nextTick()
      deleteBranchSaveButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).not.toHaveBeenCalled()
    })

    test('should not be able to delete if name does not match current branch name', async () => {
      const deleteBranchNameInput = wrapper.find('.delete-branch-input')
      const deleteBranchSaveButton = wrapper.find('.delete-branch-confirm')
      deleteBranchNameInput.setValue('new_branch')
      await Vue.nextTick()
      deleteBranchSaveButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).not.toHaveBeenCalled()
    })

    test('should be hidden if deleted successfully', async () => {
      const currentBranchName = (wrapper.vm as any).selectedBranch
      let deleteBranchNameInput = wrapper.find('.delete-branch-input')
      const deleteBranchSaveButton = wrapper.find('.delete-branch-confirm')
      deleteBranchNameInput.setValue(currentBranchName)
      await Vue.nextTick()
      deleteBranchSaveButton.trigger('click')
      // process click
      await Vue.nextTick()
      // process API (fake) return
      await Vue.nextTick()
      deleteBranchNameInput = wrapper.find('.delete-branch-input')
      expect(deleteBranchNameInput.element).toBeUndefined()
    })

    test('should be visible if delete failed', async () => {
      deleteSpy.mockImplementation(() => Promise.reject(new Error()))
      const currentBranchName = (wrapper.vm as any).selectedBranch
      let deleteBranchNameInput = wrapper.find('.delete-branch-input')
      const deleteBranchSaveButton = wrapper.find('.delete-branch-confirm')
      deleteBranchNameInput.setValue(currentBranchName)
      await Vue.nextTick()
      deleteBranchSaveButton.trigger('click')
      await Vue.nextTick()
      deleteBranchNameInput = wrapper.find('.delete-branch-input')
      expect(deleteBranchNameInput.element).toBeDefined()
    })
  })
})
