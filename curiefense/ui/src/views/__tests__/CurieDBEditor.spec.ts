import CurieDBEditor from '@/views/CurieDBEditor.vue'
import GitHistory from '@/components/GitHistory.vue'
import Utils from '@/assets/Utils'
import {afterEach, beforeEach, describe, expect, jest, test} from '@jest/globals'
import {mount, Wrapper} from '@vue/test-utils'
import Vue from 'vue'
import axios from 'axios'
import JSONEditor from 'jsoneditor'
import {Commit} from '@/types'

jest.mock('axios')
jest.mock('jsoneditor')

describe('CurieDBEditor.vue', () => {
  let wrapper: Wrapper<Vue>
  let dbData: any
  let publishInfoData: any
  let dbKeyLogs: Commit[]
  beforeEach(async () => {
    publishInfoData = {
      'buckets': [{'name': 'prod', 'url': 's3://curiefense-test01/prod'}, {
        'name': 'devops',
        'url': 's3://curiefense-test01/devops',
      }],
      'branch_buckets': [{'name': 'master', 'buckets': ['prod']}, {'name': 'devops', 'buckets': ['devops']}],
    }
    dbData = {
      'publishinfo': publishInfoData,
      'tags': {
        'neutral': [
          'china',
          'ukraine',
          'internal',
          'devops',
          'google',
          'yahoo',
          'localhost',
          'tor',
          'bad-people',
          'dev',
          'test-tag',
          'all',
          '',
          'okay',
        ],
      },
    }
    dbKeyLogs = [{
      'author': 'Curiefense API',
      'email': 'curiefense@reblaze.com',
      'date': '2020-11-10T09:41:31+02:00',
      'message': 'Setting key [publishinfo] in namespace [system]',
      'version': 'b104d3dd17f790b75c4e067c44bb06b914902d78',
      'parents': ['ff59eb0e6d230c077dfa503c9f2d4aacec1b72ab'],
    }, {
      'author': 'Curiefense API',
      'email': 'curiefense@reblaze.com',
      'date': '2020-08-27T16:19:58+00:00',
      'message': 'Added namespace [system]',
      'version': 'ff59eb0e6d230c077dfa503c9f2d4aacec1b72ab',
      'parents': ['a34f979217215060861b58b3f270e82580c20efb'],
    }]
    // @ts-ignore
    JSONEditor.mockImplementation((container, options) => {
      let value = {}
      let onChangeFunc: Function
      if (options.onChange) {
        onChangeFunc = options.onChange
      }
      return {
        set: (newValue: any) => {
          value = newValue
          if (typeof onChangeFunc === 'function') {
            onChangeFunc()
          }
        },
        get: () => {
          return value
        },
      }
    })
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/db/') {
        return Promise.resolve({data: ['system', 'namespaceCopy', 'anotherDB']})
      }
      const db = (wrapper.vm as any).selectedNamespace
      const key = (wrapper.vm as any).selectedKey
      if (path === `/conf/api/v2/db/new namespace/`) {
        return Promise.resolve({data: {key: {}}})
      }
      if (path === `/conf/api/v2/db/${db}/`) {
        return Promise.resolve({data: dbData})
      }
      if (path === `/conf/api/v2/db/${db}/k/${key}/v/`) {
        return Promise.resolve({data: dbKeyLogs})
      }
      return Promise.resolve({data: {}})
    })
    wrapper = mount(CurieDBEditor)
    await Vue.nextTick()
  })
  afterEach(() => {
    jest.clearAllMocks()
  })

  test('should have a git history component', () => {
    const gitHistory = wrapper.findComponent(GitHistory)
    expect(gitHistory).toBeTruthy()
  })

  test('should log message when receiving no databases from the server', (done) => {
    const originalLog = console.log
    let consoleOutput: string[] = []
    const mockedLog = (output: string) => consoleOutput.push(output)
    consoleOutput = []
    console.log = mockedLog
    jest.spyOn(axios, 'get').mockImplementation((path) => {
      if (path === '/conf/api/v2/db/') {
        return Promise.resolve({data: []})
      }
      return Promise.resolve({data: {}})
    })
    wrapper = mount(CurieDBEditor)
    // allow all requests to finish
    setImmediate(() => {
      expect(consoleOutput).toContain(`failed loading namespace, none are present!`)
      console.log = originalLog
      done()
    })
  })

  test('should be able to switch namespaces through dropdown', (done) => {
    const wantedValue = 'namespaceCopy'
    const namespaceSelection = wrapper.find('.namespace-selection')
    namespaceSelection.trigger('click')
    const options = namespaceSelection.findAll('option')
    options.at(1).setSelected()
    // allow all requests to finish
    setImmediate(() => {
      expect((wrapper.vm as any).selectedNamespace).toEqual(wantedValue)
      done()
    })
  })

  test('should be able to switch key through dropdown', (done) => {
    const wantedValue = Object.keys(dbData)[1]
    const keySelection = wrapper.find('.key-selection')
    keySelection.trigger('click')
    const options = keySelection.findAll('option')
    options.at(1).setSelected()
    // allow all requests to finish
    setImmediate(() => {
      expect((wrapper.vm as any).selectedKey).toEqual(wantedValue)
      done()
    })
  })

  test('should send API request to restore to the correct version', async () => {
    const wantedVersion = {
      version: 'b104d3dd17f790b75c4e067c44bb06b914902d78',
    }
    const putSpy = jest.spyOn(axios, 'put')
    putSpy.mockImplementation(() => Promise.resolve())
    const gitHistory = wrapper.findComponent(GitHistory)
    gitHistory.vm.$emit('restore-version', wantedVersion)
    await Vue.nextTick()
    expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/db/system/v/${wantedVersion.version}/revert/`)
  })

  test('should load last loaded key if still exists after restoring version', (done) => {
    const restoredVersion = {
      version: 'b104d3dd17f790b75c4e067c44bb06b914902d78',
    }
    const wantedKey = 'publishinfo'
    wrapper.setData({selectedKey: wantedKey})
    jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
    const gitHistory = wrapper.findComponent(GitHistory)
    gitHistory.vm.$emit('restore-version', restoredVersion)
    // allow all requests to finish
    setImmediate(() => {
      expect((wrapper.vm as any).selectedKey).toEqual(wantedKey)
      done()
    })
  })

  test('should load first key if key no longer exists after restoring version', (done) => {
    const restoredVersion = {
      version: 'b104d3dd17f790b75c4e067c44bb06b914902d78',
    }
    const wantedKey = 'publishinfo'
    wrapper.setData({selectedKey: 'somekey'})
    jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
    const gitHistory = wrapper.findComponent(GitHistory)
    gitHistory.vm.$emit('restore-version', restoredVersion)
    // allow all requests to finish
    setImmediate(() => {
      expect((wrapper.vm as any).selectedKey).toEqual(wantedKey)
      done()
    })
  })

  test('should attempt to download namespace when download button is clicked', async () => {
    const wantedFileName = 'system'
    const wantedFileType = 'json'
    const wantedFileData = dbData
    const downloadFileSpy = jest.spyOn(Utils, 'downloadFile').mockImplementation(() => {})
    // force update because downloadFile is mocked after it is read to to be used as event handler
    await (wrapper.vm as any).$forceUpdate()
    await Vue.nextTick()
    const downloadNamespaceButton = wrapper.find('.download-namespace-button')
    downloadNamespaceButton.trigger('click')
    await Vue.nextTick()
    expect(downloadFileSpy).toHaveBeenCalledWith(wantedFileName, wantedFileType, wantedFileData)
  })

  test('should attempt to download key when download button is clicked', async () => {
    const wantedFileName = 'publishinfo'
    const wantedFileType = 'json'
    const wantedFileData = publishInfoData
    const downloadFileSpy = jest.spyOn(Utils, 'downloadFile').mockImplementation(() => {})
    // force update because downloadFile is mocked after it is read to be used as event handler
    await (wrapper.vm as any).$forceUpdate()
    await Vue.nextTick()
    const downloadKeyButton = wrapper.find('.download-key-button')
    downloadKeyButton.trigger('click')
    await Vue.nextTick()
    expect(downloadFileSpy).toHaveBeenCalledWith(wantedFileName, wantedFileType, wantedFileData)
  })

  test('should not attempt to download key when download button is clicked if value does not exist', async () => {
    const wantedFileName = 'publishinfo'
    const wantedFileType = 'json'
    const wantedFileData = publishInfoData
    const downloadFileSpy = jest.spyOn(Utils, 'downloadFile').mockImplementation(() => {})
    wrapper.setData({selectedKeyValue: null})
    // force update because downloadFile is mocked after it is read to be used as event handler
    await (wrapper.vm as any).$forceUpdate()
    await Vue.nextTick()
    const downloadKeyButton = wrapper.find('.download-key-button')
    downloadKeyButton.trigger('click')
    await Vue.nextTick()
    expect(downloadFileSpy).not.toHaveBeenCalledWith(wantedFileName, wantedFileType, wantedFileData)
  })

  describe('namespace action buttons', () => {
    test('should be able to fork namespace', async () => {
      const dbData = (wrapper.vm as any).selectedNamespaceData
      const putSpy = jest.spyOn(axios, 'put')
      putSpy.mockImplementation(() => Promise.resolve())
      const forkNamespaceButton = wrapper.find('.fork-namespace-button')
      forkNamespaceButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/db/copy of system/`, dbData)
    })

    test('should be able to add a new namespace', async () => {
      const newNamespace = {
        key: {},
      }
      const putSpy = jest.spyOn(axios, 'put')
      putSpy.mockImplementation(() => Promise.resolve())
      const newNamespaceButton = wrapper.find('.new-namespace-button')
      newNamespaceButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/db/new namespace/`, newNamespace)
    })

    test('should be able to delete a namespace', (done) => {
      jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      // create new namespace so we can delete it
      const newNamespaceButton = wrapper.find('.new-namespace-button')
      newNamespaceButton.trigger('click')
      setImmediate(async () => {
        const namespaceName = (wrapper.vm as any).selectedNamespace
        const deleteNamespaceButton = wrapper.find('.delete-namespace-button')
        deleteNamespaceButton.trigger('click')
        await Vue.nextTick()
        expect(deleteSpy).toHaveBeenCalledWith(`/conf/api/v2/db/${namespaceName}/`)
        done()
      })
    })

    test('should not be able to delete the `system` namespace', async () => {
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      const deleteNamespaceButton = wrapper.find('.delete-namespace-button')
      deleteNamespaceButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).not.toHaveBeenCalled()
    })
  })

  describe('key action buttons', () => {
    test('should be able to fork key', async () => {
      const doc = JSON.parse((wrapper.vm as any).selectedKeyValue || '{}')
      const putSpy = jest.spyOn(axios, 'put')
      putSpy.mockImplementation(() => Promise.resolve())
      const forkKeyButton = wrapper.find('.fork-key-button')
      forkKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/db/system/k/copy of publishinfo/`, doc)
    })

    test('should be able to add a new key', async () => {
      const newKey = {}
      const putSpy = jest.spyOn(axios, 'put')
      putSpy.mockImplementation(() => Promise.resolve())
      const newKeyButton = wrapper.find('.new-key-button')
      newKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/db/system/k/new key/`, newKey)
    })

    test('should be able to delete a key', (done) => {
      jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      // create new key so we can delete it
      const newKeyButton = wrapper.find('.new-key-button')
      newKeyButton.trigger('click')
      setImmediate(async () => {
        const keyName = (wrapper.vm as any).selectedKey
        const deleteKeyButton = wrapper.find('.delete-key-button')
        deleteKeyButton.trigger('click')
        await Vue.nextTick()
        expect(deleteSpy).toHaveBeenCalledWith(`/conf/api/v2/db/system/k/${keyName}/`)
        done()
      })
    })

    test('should not be able to delete a `publishinfo` key under `system` namespace', async () => {
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      const deleteKeyButton = wrapper.find('.delete-key-button')
      deleteKeyButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).not.toHaveBeenCalled()
    })
  })

  describe('save changes button', () => {
    let putSpy: any
    beforeEach((done) => {
      putSpy = jest.spyOn(axios, 'put')
      // create a new namespace for empty environment to test changes on
      putSpy.mockImplementation(() => Promise.resolve())
      const newNamespaceButton = wrapper.find('.new-namespace-button')
      newNamespaceButton.trigger('click')
      // allow all requests to finish
      setImmediate(() => {
        jest.clearAllMocks()
        putSpy = jest.spyOn(axios, 'put')
        done()
      })
    })

    test('should be able to save namespace changes even if namespace name changes', async (done) => {
      const namespaceNameInput = wrapper.find('.namespace-name-input')
      const key = 'key_name'
      const value = {
        buckets: {},
        foo: 'bar',
      }
      const wantedResult = {
        [key]: value,
      }
      namespaceNameInput.setValue('newDB')
      namespaceNameInput.trigger('input')
      await Vue.nextTick()
      const keyNameInput = wrapper.find('.key-name-input')
      keyNameInput.setValue(key)
      keyNameInput.trigger('input')
      await Vue.nextTick()
      // @ts-ignore
      wrapper.vm.selectedKeyValue = JSON.stringify(value)
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      // allow all requests to finish
      setImmediate(() => {
        expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/db/newDB/`, wantedResult)
        done()
      })
    })

    test('should be able to save key changes even if key name changes', async () => {
      const keyNameInput = wrapper.find('.key-name-input')
      keyNameInput.setValue('key_name')
      keyNameInput.trigger('input')
      await Vue.nextTick()
      const value = {
        buckets: {},
        foo: 'bar',
      }
      wrapper.setData({selectedKeyValue: JSON.stringify(value)})
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/db/new namespace/k/key_name/`, value)
    })

    test('should be able to save key changes', async () => {
      const value = {
        buckets: {},
        foo: 'bar',
      }
      wrapper.setData({selectedKeyValue: JSON.stringify(value)})
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/db/new namespace/k/key/`, value)
    })

    test('should use correct values when saving key changes when using json editor', (done) => {
      // setTimeout to allow the editor to be fully loaded before we interact with it
      setTimeout(async () => {
        const value = {
          buckets: {},
          foo: 'bar',
        }
        wrapper.vm.$data.editor.set(value)
        await Vue.nextTick()
        const saveKeyButton = wrapper.find('.save-button')
        saveKeyButton.trigger('click')
        await Vue.nextTick()
        expect(putSpy).toHaveBeenCalledWith(`/conf/api/v2/db/new namespace/k/key/`, value)
        done()
      }, 300)
    })

    test('should not be able to save key changes' +
      'if value is an invalid json when not using json editor', async () => {
      wrapper.setData({editor: null})
      wrapper.setData({isJsonEditor: false})
      await Vue.nextTick()
      const value = '{'
      const valueInput = wrapper.find('.value-input')
      valueInput.setValue(value)
      valueInput.trigger('input')
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).not.toHaveBeenCalled()
    })

    test('should not render normal text area if json editor has been loaded', (done) => {
      // setTimeout to allow the editor to be fully loaded before we interact with it
      setTimeout(async () => {
        const valueInput = wrapper.find('.value-input')
        expect(valueInput.element).toBeUndefined()
        done()
      }, 300)
    })

    test('should default to normal text area when json editor cannot be loaded after 2 seconds', (done) => {
      // @ts-ignore
      JSONEditor.mockImplementation(() => {
        throw new Error('ouchie')
      })
      wrapper = mount(CurieDBEditor)
      // setTimeout to allow the editor to be fully loaded before we interact with it
      setTimeout(async () => {
        const valueInput = wrapper.find('.value-input')
        expect(valueInput.element).toBeDefined()
        done()
      }, 2300)
    })

    test('should not be able to save key changes if namespace name is empty', async () => {
      const namespaceNameInput = wrapper.find('.namespace-name-input')
      namespaceNameInput.setValue('')
      namespaceNameInput.trigger('input')
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).not.toHaveBeenCalled()
    })

    test('should not be able to save key changes if namespace name is duplicate of another namespace', async () => {
      const namespaceNameInput = wrapper.find('.namespace-name-input')
      namespaceNameInput.setValue('namespaceCopy')
      namespaceNameInput.trigger('input')
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).not.toHaveBeenCalled()
    })

    test('should not be able to save key changes if key name is empty', async () => {
      const keyNameInput = wrapper.find('.key-name-input')
      keyNameInput.setValue('')
      keyNameInput.trigger('input')
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).not.toHaveBeenCalled()
    })

    test('should not be able to save key changes if key name is duplicate of another key', async () => {
      // add a new key so we would have multiple keys
      const newKeyButton = wrapper.find('.new-key-button')
      newKeyButton.trigger('click')
      // click event
      await Vue.nextTick()
      // key switch
      await Vue.nextTick()
      // change key name
      const keyNameInput = wrapper.find('.key-name-input')
      keyNameInput.setValue('key')
      keyNameInput.trigger('input')
      await Vue.nextTick()
      // reset spy counter
      jest.clearAllMocks()
      putSpy = jest.spyOn(axios, 'put')
      // attempt saving duplicate named key
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).not.toHaveBeenCalled()
    })
  })

  describe('no data', () => {
    test('should display correct message when there is no namespace list data', (done) => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/db/') {
          return Promise.resolve({data: []})
        }
        return Promise.resolve({data: {}})
      })
      wrapper = mount(CurieDBEditor)
      // allow all requests to finish
      setImmediate(() => {
        const noDataMessage = wrapper.find('.no-data-message')
        expect(noDataMessage.element).toBeDefined()
        expect(noDataMessage.text().toLowerCase()).toContain('no data found!')
        expect(noDataMessage.text().toLowerCase()).toContain('missing namespace.')
        done()
      })
    })

    test('should display correct message when there is no key data', (done) => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/db/') {
          return Promise.resolve({data: ['system', 'namespaceCopy', 'anotherDB']})
        }
        const db = (wrapper.vm as any).selectedNamespace
        if (path === `/conf/api/v2/db/${db}/`) {
          return Promise.resolve({data: {}})
        }
        return Promise.resolve({
          data: {},
        })
      })
      wrapper = mount(CurieDBEditor)
      // allow all requests to finish
      setImmediate(() => {
        const noDataMessage = wrapper.find('.no-data-message')
        expect(noDataMessage.element).toBeDefined()
        expect(noDataMessage.text().toLowerCase()).toContain('no data found!')
        expect(noDataMessage.text().toLowerCase()).toContain('missing key.')
        done()
      })
    })
  })

  describe('loading indicator', () => {
    test('should display loading indicator when namespaces list not loaded', async () => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/db/') {
          return new Promise(() => {
          })
        }
        return Promise.resolve({data: []})
      })
      wrapper = mount(CurieDBEditor)
      await Vue.nextTick()
      const valueLoadingIndicator = wrapper.find('.value-loading')
      expect(valueLoadingIndicator.element).toBeDefined()
    })

    test('should display loading indicator when namespace not loaded', async () => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v2/db/') {
          return Promise.resolve({data: ['system', 'namespaceCopy', 'anotherDB']})
        }
        const db = (wrapper.vm as any).selectedNamespace
        if (path === `/conf/api/v2/db/${db}/`) {
          return new Promise(() => {
          })
        }
        return Promise.resolve({data: {}})
      })
      wrapper = mount(CurieDBEditor)
      await Vue.nextTick()
      const valueLoadingIndicator = wrapper.find('.value-loading')
      expect(valueLoadingIndicator.element).toBeDefined()
    })

    test('should display loading indicator when saving value changes', async () => {
      jest.spyOn(axios, 'put').mockImplementation(() => new Promise(() => {
      }))
      const saveDocumentButton = wrapper.find('.save-button')
      saveDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(saveDocumentButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when forking namespace', async () => {
      jest.spyOn(axios, 'post').mockImplementation(() => new Promise(() => {
      }))
      const forkNamespaceButton = wrapper.find('.fork-namespace-button')
      forkNamespaceButton.trigger('click')
      await Vue.nextTick()
      expect(forkNamespaceButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when adding a new namespace', async () => {
      jest.spyOn(axios, 'post').mockImplementation(() => new Promise(() => {
      }))
      const newNamespaceButton = wrapper.find('.new-namespace-button')
      newNamespaceButton.trigger('click')
      await Vue.nextTick()
      expect(newNamespaceButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when deleting a namespace', (done) => {
      jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
      jest.spyOn(axios, 'delete').mockImplementation(() => new Promise(() => {
      }))
      // create new namespace so we can delete it
      const newNamespaceButton = wrapper.find('.new-namespace-button')
      newNamespaceButton.trigger('click')
      setImmediate(async () => {
        const deleteNamespaceButton = wrapper.find('.delete-namespace-button')
        deleteNamespaceButton.trigger('click')
        await Vue.nextTick()
        expect(deleteNamespaceButton.element.classList).toContain('is-loading')
        done()
      })
    })

    test('should display loading indicator when forking key', async () => {
      jest.spyOn(axios, 'post').mockImplementation(() => new Promise(() => {
      }))
      const forkKeyButton = wrapper.find('.fork-key-button')
      forkKeyButton.trigger('click')
      await Vue.nextTick()
      expect(forkKeyButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when adding a new key', async () => {
      jest.spyOn(axios, 'post').mockImplementation(() => new Promise(() => {
      }))
      const newKeyButton = wrapper.find('.new-key-button')
      newKeyButton.trigger('click')
      await Vue.nextTick()
      expect(newKeyButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when deleting a key', (done) => {
      jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
      jest.spyOn(axios, 'delete').mockImplementation(() => new Promise(() => {
      }))
      // create new namespace so we can delete it
      const newNamespaceButton = wrapper.find('.new-key-button')
      newNamespaceButton.trigger('click')
      setImmediate(async () => {
        const deleteNamespaceButton = wrapper.find('.delete-key-button')
        deleteNamespaceButton.trigger('click')
        await Vue.nextTick()
        expect(deleteNamespaceButton.element.classList).toContain('is-loading')
        done()
      })
    })
  })
})
