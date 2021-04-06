import Publish from '@/views/Publish.vue'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {mount, Wrapper} from '@vue/test-utils'
import axios from 'axios'
import Vue from 'vue'
import {Branch} from '@/types'
import * as bulmaToast from 'bulma-toast'
import {Options} from 'bulma-toast'

jest.mock('axios')

describe('Publish.vue', () => {
  let wrapper: Wrapper<Vue>
  let gitData: Branch[]
  let publishInfoData: any
  beforeEach(async () => {
    gitData = [
      {
        'id': 'devops',
        'description': 'Initial empty content',
        'date': '2020-08-27T16:19:06+00:00',
        'logs': [
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
        'version': '1662043d2a18d6ad2c9c94d6f826593ff5506354',
      },
      {
        'id': 'master',
        'description': 'Update entry [__default__] of document [aclpolicies]',
        'date': '2020-11-10T15:49:17+02:00',
        'logs': [
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
    ]
    publishInfoData = {
      'buckets': [
        {'name': 'devops', 'url': 's3://curiefense-test01/devops'},
        {'name': 'prod', 'url': 's3://curiefense-test01/prod'},
      ],
      'branch_buckets': [{'name': 'master', 'buckets': ['prod']}, {'name': 'devops', 'buckets': ['devops']}],
    }
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v1/configs/') {
        return Promise.resolve({data: gitData})
      }
      if (path === `/conf/api/v1/db/system/k/publishinfo/`) {
        return Promise.resolve({data: publishInfoData})
      }
      return Promise.resolve({data: {}})
    })
    wrapper = mount(Publish)
    await Vue.nextTick()
  })
  afterEach(() => {
    jest.clearAllMocks()
  })

  test('should be able to switch branches through dropdown', (done) => {
    const branchSelection = wrapper.find('.branch-selection')
    branchSelection.trigger('click')
    const options = branchSelection.findAll('option')
    options.at(1).setSelected()
    // allow all requests to finish
    setImmediate(() => {
      expect((branchSelection.element as HTMLSelectElement).selectedIndex).toEqual(1)
      done()
    })
  })

  test('should display no version and 0 buckets if no logs are present', async () => {
    gitData[0].logs = []
    wrapper = mount(Publish)
    await Vue.nextTick()
    const versionDisplay = wrapper.find('.version-display')
    expect(versionDisplay.text()).toEqual(`Version:`)
    const bucketsDisplay = wrapper.find('.buckets-display')
    expect(bucketsDisplay.text()).toEqual('Buckets: 0')
  })

  test('should display correct latest version on startup', () => {
    const versionDisplay = wrapper.find('.version-display')
    expect(versionDisplay.text()).toEqual(`Version: ${gitData[0].version}`)
  })

  test('should display correct amount of buckets on startup', () => {
    const bucketsDisplay = wrapper.find('.buckets-display')
    expect(bucketsDisplay.text()).toEqual('Buckets: 1')
  })

  test('should display correct amount of buckets without counting unavailable buckets', async (done) => {
    (wrapper.vm as any).publishInfo = {
      'buckets': [
        {'name': 'prod', 'url': 's3://curiefense-test01/prod'},
        {'name': 'devops', 'url': 's3://curiefense-test01/devops'},
      ],
      'branch_buckets': [{'name': 'master', 'buckets': ['prod', 'fake']}, {'name': 'devops', 'buckets': ['devops']}],
    }
    await Vue.nextTick()
    const branchSelection = wrapper.find('.branch-selection')
    branchSelection.trigger('click')
    const options = branchSelection.findAll('option')
    options.at(1).setSelected()
    // allow all requests to finish
    setImmediate(() => {
      const gitBranches = wrapper.find('.buckets-display')
      expect(gitBranches.text()).toEqual('Buckets: 1')
      done()
    })
  })

  describe('commits table display', () => {
    test('should display all buckets', async () => {
      const bucketRows = wrapper.findAll('.bucket-row')
      expect(bucketRows.length).toEqual(2)
      const bucketRow0 = bucketRows.at(0)
      expect(bucketRow0.text()).toContain(publishInfoData.buckets[0].name)
      const bucketRow1 = bucketRows.at(1)
      expect(bucketRow1.text()).toContain(publishInfoData.buckets[1].name)
    })

    test('should have correct buckets pre-selected', async () => {
      const bucketRows = wrapper.findAll('.bucket-row')
      const bucketRow0 = bucketRows.at(0)
      expect((bucketRow0.element as HTMLElement).classList).toContain('has-background-warning-light')
      const bucketRow1 = bucketRows.at(1)
      expect((bucketRow1.element as HTMLElement).classList).not.toContain('has-background-warning-light')
    })

    test('should have correct no there are buckets matches in config', async () => {
      publishInfoData = {
        'buckets': [
          {'name': 'prod', 'url': 's3://curiefense-test01/prod'},
          {'name': 'devops', 'url': 's3://curiefense-test01/devops'},
        ],
        'branch_buckets': [{'name': 'master'}, {'name': 'devops'}],
      }
      wrapper = mount(Publish)
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      const bucketRows = wrapper.findAll('.bucket-row')
      const bucketRow0 = bucketRows.at(0)
      expect((bucketRow0.element as HTMLElement).classList).not.toContain('has-background-warning-light')
      const bucketRow1 = bucketRows.at(1)
      expect((bucketRow1.element as HTMLElement).classList).not.toContain('has-background-warning-light')
    })

    test('should have correct no branch_bucket data matches the current branch', async () => {
      publishInfoData = {
        'buckets': [
          {'name': 'prod', 'url': 's3://curiefense-test01/prod'},
          {'name': 'devops', 'url': 's3://curiefense-test01/devops'},
        ],
        'branch_buckets': [{'name': 'master1'}, {'name': 'devops1'}],
      }
      wrapper = mount(Publish)
      await Vue.nextTick()
      await wrapper.vm.$forceUpdate()
      const bucketRows = wrapper.findAll('.bucket-row')
      const bucketRow0 = bucketRows.at(0)
      expect((bucketRow0.element as HTMLElement).classList).not.toContain('has-background-warning-light')
      const bucketRow1 = bucketRows.at(1)
      expect((bucketRow1.element as HTMLElement).classList).not.toContain('has-background-warning-light')
    })
  })

  describe('commits table display', () => {
    test('should display all commits if less than maximum number (default: 5)', () => {
      const commitRows = wrapper.findAll('.commit-row')
      expect(commitRows.length).toEqual(3)
    })

    test('should not display view more button if less commits than maximum number (default: 5)', () => {
      const viewMoreButton = wrapper.find('.view-more-button')
      expect(viewMoreButton.exists()).toBeFalsy()
    })

    test('should display maximum number of commits if more are present (default: 5)', (done) => {
      const branchSelection = wrapper.find('.branch-selection')
      branchSelection.trigger('click')
      const options = branchSelection.findAll('option')
      options.at(1).setSelected()
      // allow all requests to finish
      setImmediate(() => {
        const commitRows = wrapper.findAll('.commit-row')
        expect(commitRows.length).toEqual(5)
        done()
      })
    })

    test('should display all commit if view more button is clicked', (done) => {
      const branchSelection = wrapper.find('.branch-selection')
      branchSelection.trigger('click')
      const options = branchSelection.findAll('option')
      options.at(1).setSelected()
      // allow all requests to finish
      setImmediate(async () => {
        const viewMoreButton = wrapper.find('.view-more-button')
        viewMoreButton.trigger('click')
        await Vue.nextTick()
        const commitRows = wrapper.findAll('.commit-row')
        expect(commitRows.length).toEqual(9)
        done()
      })
    })

    test('should display maximum number of commits if view less button is clicked', (done) => {
      const branchSelection = wrapper.find('.branch-selection')
      branchSelection.trigger('click')
      const options = branchSelection.findAll('option')
      options.at(1).setSelected()
      // allow all requests to finish
      setImmediate(async () => {
        const viewMoreButton = wrapper.find('.view-more-button')
        viewMoreButton.trigger('click')
        await Vue.nextTick()
        const viewLessButton = wrapper.find('.view-less-button')
        viewLessButton.trigger('click')
        await Vue.nextTick()
        const commitRows = wrapper.findAll('.commit-row')
        expect(commitRows.length).toEqual(5)
        done()
      })
    })
  })

  describe('publish button', () => {
    let publishButton: Wrapper<Vue>
    let putSpy: any
    beforeEach(() => {
      jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve({data: {}}))
      publishButton = wrapper.find('.publish-button')
      putSpy = jest.spyOn(axios, 'put')
    })

    test('should publish with preset data if not changed', async () => {
      const wantedPath = `/conf/api/v1/tools/publish/devops/v/${gitData[0].version}/`
      const wantedData = [publishInfoData.buckets[0]]
      publishButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(wantedPath, wantedData)
    })

    test('should publish with new selected version', async () => {
      const wantedPath = `/conf/api/v1/tools/publish/devops/v/${gitData[0].logs[1].version}/`
      const wantedData = [publishInfoData.buckets[0]]
      const commitRow = wrapper.findAll('.commit-row').at(1)
      commitRow.trigger('click')
      await Vue.nextTick()
      publishButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(wantedPath, wantedData)
    })

    test('should publish with both the pre-selected the new selected bucket', async () => {
      const wantedPath = `/conf/api/v1/tools/publish/devops/v/${gitData[0].version}/`
      const wantedData = publishInfoData.buckets
      const bucketRow = wrapper.findAll('.bucket-row').at(1)
      bucketRow.trigger('click')
      await Vue.nextTick()
      publishButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(wantedPath, wantedData)
    })

    test('should publish without de-selected bucket', async () => {
      const wantedPath = `/conf/api/v1/tools/publish/devops/v/${gitData[0].version}/`
      const wantedData = [publishInfoData.buckets[1]]
      const newBucketRow = wrapper.findAll('.bucket-row').at(1)
      newBucketRow.trigger('click')
      await Vue.nextTick()
      const preselectedBucketRow = wrapper.findAll('.bucket-row').at(0)
      preselectedBucketRow.trigger('click')
      await Vue.nextTick()
      publishButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(wantedPath, wantedData)
    })

    test('should not publish without a selected bucket', async () => {
      const preselectedBucketRow = wrapper.findAll('.bucket-row').at(0)
      preselectedBucketRow.trigger('click')
      await Vue.nextTick()
      publishButton.trigger('click')
      await Vue.nextTick()
      expect((publishButton.element as HTMLButtonElement).disabled).toBeTruthy()
      expect(putSpy).not.toHaveBeenCalled()
    })
  })

  describe('publish request succeeded with okay status true', () => {
    let publishButton: Wrapper<Vue>
    let response: any
    let successMessage: string
    let successMessageClass: string
    let toastOutput: Options[]
    beforeEach(async () => {
      publishButton = wrapper.find('.publish-button')
      response = {
        'ok': true,
        'status': [
          {
            'message': 'Success',
            'name': 'devops',
            'ok': true,
          },
          {
            'message': 'this does not exist',
            'name': 'prod',
            'ok': false,
          },
        ],
      }
      jest.spyOn(axios, 'put').mockImplementation(() => {
        return Promise.resolve({data: response})
      })
      successMessage =
        `Branch "${publishInfoData.buckets[0].name}" was published with version "${gitData[0].logs[0].version}".`
      successMessageClass = 'is-success'
      toastOutput = []
      jest.spyOn(bulmaToast, 'toast').mockImplementation((output: Options) => {
        toastOutput.push(output)
      })
      publishButton.trigger('click')
      await Vue.nextTick()
    })

    test('should only contain buckets which were in the publish request', async () => {
      const bucketRows = wrapper.findAll('.bucket-row')
      const bucketRow = bucketRows.at(0)
      expect(bucketRows.length).toEqual(1)
      expect(bucketRow.text()).toContain(publishInfoData.buckets[0].name)
    })

    test('should disable the loading indicator for the publish button', async () => {
      expect((publishButton.element as HTMLButtonElement).classList).not.toContain('is-loading')
    })

    test('should display success text', async () => {
      const successText = 'Publish to this bucket has been done successfully!'
      const bucketRows = wrapper.findAll('.bucket-row')
      const bucketRow = bucketRows.at(0)
      expect(bucketRow.text()).toContain(successText)
    })

    test('should display toast with a success message', async () => {
      expect(toastOutput[0].message).toContain(successMessage)
      expect(toastOutput[0].type).toContain(successMessageClass)
    })
  })

  describe('publish request succeeded with okay status false', () => {
    let publishButton: Wrapper<Vue>
    let response: any
    let failureMessage: string
    let failureMessageClass: string
    let toastOutput: Options[]
    beforeEach(async () => {
      publishButton = wrapper.find('.publish-button')
      response = {
        'ok': false,
        'status': [
          {
            'message': 'Exception(\'Did not find any credential to access s3://curiefense-test01/prod\')',
            'name': 'devops',
            'ok': false,
          },
          {
            'message': 'this does not exist',
            'name': 'prod',
            'ok': false,
          },
        ],
      }
      jest.spyOn(axios, 'put').mockImplementation(() => {
        return Promise.resolve({data: response})
      })
      // eslint-disable-next-line max-len
      failureMessage = `Failed while attempting to publish branch "${publishInfoData.buckets[0].name}" version "${gitData[0].logs[0].version}".`
      failureMessageClass = 'is-danger'
      toastOutput = []
      jest.spyOn(bulmaToast, 'toast').mockImplementation((output: Options) => {
        toastOutput.push(output)
      })
      publishButton.trigger('click')
      await Vue.nextTick()
    })

    test('should only contain buckets which were in the publish request', async () => {
      const bucketRows = wrapper.findAll('.bucket-row')
      const bucketRow = bucketRows.at(0)
      expect(bucketRows.length).toEqual(1)
      expect(bucketRow.text()).toContain(publishInfoData.buckets[0].name)
    })

    test('should disable the loading indicator for the publish button', async () => {
      expect((publishButton.element as HTMLButtonElement).classList).not.toContain('is-loading')
    })

    test('should display failure text', async () => {
      const failureText = 'Error publishing to this bucket'
      const bucketRows = wrapper.findAll('.bucket-row')
      const bucketRow = bucketRows.at(0)
      expect(bucketRow.text()).toContain(failureText)
    })

    test('should display failure text from response', async () => {
      const bucketRows = wrapper.findAll('.bucket-row')
      const bucketRow = bucketRows.at(0)
      expect(bucketRow.text()).toContain(response.status[0].message)
    })

    test('should display toast with a failure message', async () => {
      expect(toastOutput[0].message).toContain(failureMessage)
      expect(toastOutput[0].type).toContain(failureMessageClass)
    })
  })

  describe('publish request failed', () => {
    let publishButton: Wrapper<Vue>
    let failureMessage: string
    let failureMessageClass: string
    let toastOutput: Options[]
    let originalError: any
    beforeEach(async () => {
      publishButton = wrapper.find('.publish-button')
      jest.spyOn(axios, 'put').mockImplementation(() => {
        return Promise.reject(new Error())
      })
      originalError = console.error
      let consoleOutput: string[] = []
      const mockedError = (output: string) => consoleOutput.push(output)
      consoleOutput = []
      console.error = mockedError
      // eslint-disable-next-line max-len
      failureMessage = `Failed while attempting to publish branch "${publishInfoData.buckets[0].name}" version "${gitData[0].logs[0].version}".`
      failureMessageClass = 'is-danger'
      toastOutput = []
      jest.spyOn(bulmaToast, 'toast').mockImplementation((output: Options) => {
        toastOutput.push(output)
      })
      publishButton.trigger('click')
      await Vue.nextTick()
    })
    afterEach(() => {
      console.error = originalError
    })

    test('should only contain buckets which were in the publish request', async () => {
      const bucketRows = wrapper.findAll('.bucket-row')
      const bucketRow = bucketRows.at(0)
      expect(bucketRows.length).toEqual(1)
      expect(bucketRow.text()).toContain(publishInfoData.buckets[0].name)
    })

    test('should disable the loading indicator for the publish button', async () => {
      expect((publishButton.element as HTMLButtonElement).classList).not.toContain('is-loading')
    })

    test('should display toast with a failure message', async () => {
      expect(toastOutput[0].message).toContain(failureMessage)
      expect(toastOutput[0].type).toContain(failureMessageClass)
    })
  })
})
