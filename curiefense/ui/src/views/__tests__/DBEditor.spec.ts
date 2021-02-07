import DBEditor from '@/views/DBEditor.vue'
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

describe('DBEditor.vue', () => {
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
      'message': 'Setting key [publishinfo] in database [system]',
      'version': 'b104d3dd17f790b75c4e067c44bb06b914902d78',
      'parents': ['ff59eb0e6d230c077dfa503c9f2d4aacec1b72ab'],
    }, {
      'author': 'Curiefense API',
      'email': 'curiefense@reblaze.com',
      'date': '2020-08-27T16:19:58+00:00',
      'message': 'Added database [system]',
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
      if (path === '/conf/api/v1/db/') {
        return Promise.resolve({data: ['system', 'databaseCopy', 'anotherDB']})
      }
      const db = (wrapper.vm as any).selectedDatabase
      const key = (wrapper.vm as any).selectedKey
      if (path === `/conf/api/v1/db/new database/`) {
        return Promise.resolve({data: {key: {}}})
      }
      if (path === `/conf/api/v1/db/${db}/`) {
        return Promise.resolve({data: dbData})
      }
      if (path === `/conf/api/v1/db/${db}/k/${key}/v/`) {
        return Promise.resolve({data: dbKeyLogs})
      }
      return Promise.resolve({data: {}})
    })
    wrapper = mount(DBEditor)
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
      if (path === '/conf/api/v1/db/') {
        return Promise.resolve({data: []})
      }
      return Promise.resolve({data: {}})
    })
    wrapper = mount(DBEditor)
    // allow all requests to finish
    setImmediate(() => {
      expect(consoleOutput).toContain(`failed loading database, none are present!`)
      console.log = originalLog
      done()
    })
  })

  test('should be able to switch databases through dropdown', (done) => {
    const wantedValue = 'databaseCopy'
    const databaseSelection = wrapper.find('.database-selection')
    databaseSelection.trigger('click')
    const options = databaseSelection.findAll('option')
    options.at(1).setSelected()
    // allow all requests to finish
    setImmediate(() => {
      expect((wrapper.vm as any).selectedDatabase).toEqual(wantedValue)
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
    const gitHistory = wrapper.findComponent(GitHistory);
    gitHistory.vm.$emit('restore-version', wantedVersion)
    await Vue.nextTick()
    expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/db/system/v/${wantedVersion.version}/revert/`)
  })

  test('should load last loaded key if still exists after restoring version', (done) => {
    const restoredVersion = {
      version: 'b104d3dd17f790b75c4e067c44bb06b914902d78',
    }
    const wantedKey = 'publishinfo';
    (wrapper.vm as any).selectedKey = wantedKey
    jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
    const gitHistory = wrapper.findComponent(GitHistory);
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
    const wantedKey = 'publishinfo';
    (wrapper.vm as any).selectedKey = 'somekey'
    jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
    const gitHistory = wrapper.findComponent(GitHistory);
    gitHistory.vm.$emit('restore-version', restoredVersion)
    // allow all requests to finish
    setImmediate(() => {
      expect((wrapper.vm as any).selectedKey).toEqual(wantedKey)
      done()
    })
  })

  test('should attempt to download database when download button is clicked', async () => {
    const wantedFileName = 'system'
    const wantedFileType = 'json'
    const wantedFileData = dbData
    const downloadFileSpy = jest.spyOn(Utils, 'downloadFile')
    // force update because downloadFile is mocked after it is read to to be used as event handler
    await (wrapper.vm as any).$forceUpdate()
    await Vue.nextTick()
    const downloadDatabaseButton = wrapper.find('.download-database-button')
    downloadDatabaseButton.trigger('click')
    await Vue.nextTick()
    expect(downloadFileSpy).toHaveBeenCalledWith(wantedFileName, wantedFileType, wantedFileData)
  })

  test('should attempt to download key when download button is clicked', async () => {
    const wantedFileName = 'publishinfo'
    const wantedFileType = 'json'
    const wantedFileData = publishInfoData
    const downloadFileSpy = jest.spyOn(Utils, 'downloadFile')
    // force update because downloadFile is mocked after it is read to be used as event handler
    await (wrapper.vm as any).$forceUpdate()
    await Vue.nextTick()
    const downloadKeyButton = wrapper.find('.download-key-button')
    downloadKeyButton.trigger('click')
    await Vue.nextTick()
    expect(downloadFileSpy).toHaveBeenCalledWith(wantedFileName, wantedFileType, wantedFileData)
  })

  describe('database action buttons', () => {
    test('should be able to fork database', async () => {
      const dbData = (wrapper.vm as any).selectedDatabaseData
      const putSpy = jest.spyOn(axios, 'put')
      putSpy.mockImplementation(() => Promise.resolve())
      const forkDatabaseButton = wrapper.find('.fork-database-button')
      forkDatabaseButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/db/copy of system/`, dbData)
    })

    test('should be able to add a new database', async () => {
      const newDatabase = {
        key: {},
      }
      const putSpy = jest.spyOn(axios, 'put')
      putSpy.mockImplementation(() => Promise.resolve())
      const newDatabaseButton = wrapper.find('.new-database-button')
      newDatabaseButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/db/new database/`, newDatabase)
    })

    test('should be able to delete a database', (done) => {
      jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      // create new database so we can delete it
      const newDatabaseButton = wrapper.find('.new-database-button')
      newDatabaseButton.trigger('click')
      setImmediate(async () => {
        const databaseName = (wrapper.vm as any).selectedDatabase
        const deleteDatabaseButton = wrapper.find('.delete-database-button')
        deleteDatabaseButton.trigger('click')
        await Vue.nextTick()
        expect(deleteSpy).toHaveBeenCalledWith(`/conf/api/v1/db/${databaseName}/`)
        done()
      })
    })

    test('should not be able to delete the `system` database', async () => {
      const deleteSpy = jest.spyOn(axios, 'delete')
      deleteSpy.mockImplementation(() => Promise.resolve())
      const deleteDatabaseButton = wrapper.find('.delete-database-button')
      deleteDatabaseButton.trigger('click')
      await Vue.nextTick()
      expect(deleteSpy).not.toHaveBeenCalled()
    })
  })

  describe('key action buttons', () => {
    test('should be able to fork key', async () => {
      const doc = JSON.parse((wrapper.vm as any).document || '{}')
      const putSpy = jest.spyOn(axios, 'put')
      putSpy.mockImplementation(() => Promise.resolve())
      const forkKeyButton = wrapper.find('.fork-key-button')
      forkKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/db/system/k/copy of publishinfo/`, doc)
    })

    test('should be able to add a new key', async () => {
      const newKey = {}
      const putSpy = jest.spyOn(axios, 'put')
      putSpy.mockImplementation(() => Promise.resolve())
      const newKeyButton = wrapper.find('.new-key-button')
      newKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/db/system/k/new key/`, newKey)
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
        expect(deleteSpy).toHaveBeenCalledWith(`/conf/api/v1/db/system/k/${keyName}/`)
        done()
      })
    })

    test('should not be able to delete a `publishinfo` key under `system` database', async () => {
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
      // create a new database for empty environment to test changes on
      putSpy.mockImplementation(() => Promise.resolve())
      const newDatabaseButton = wrapper.find('.new-database-button')
      newDatabaseButton.trigger('click')
      // allow all requests to finish
      setImmediate(() => {
        jest.clearAllMocks()
        putSpy = jest.spyOn(axios, 'put')
        done()
      })
    })

    test('should be able to save database changes even if database name changes', async (done) => {
      const databaseNameInput = wrapper.find('.database-name-input')
      const key = 'key_name'
      const doc = {
        buckets: {},
        foo: 'bar',
      }
      const wantedResult = {
        [key]: doc,
      }
      // @ts-ignore
      databaseNameInput.element.value = 'newDB'
      databaseNameInput.trigger('input')
      await Vue.nextTick()
      const keyNameInput = wrapper.find('.key-name-input')
      // @ts-ignore
      keyNameInput.element.value = key
      keyNameInput.trigger('input')
      await Vue.nextTick()
      // @ts-ignore
      wrapper.vm.document = JSON.stringify(doc)
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      // allow all requests to finish
      setImmediate(() => {
        expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/db/newDB/`, wantedResult)
        done()
      })
    })

    test('should be able to save key changes even if key name changes', async () => {
      const keyNameInput = wrapper.find('.key-name-input');
      (keyNameInput.element as any).value = 'key_name'
      keyNameInput.trigger('input')
      await Vue.nextTick()
      const doc = {
        buckets: {},
        foo: 'bar',
      };
      (wrapper.vm as any).document = JSON.stringify(doc)
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/db/new database/k/key_name/`, doc)
    })

    test('should be able to save key changes', async () => {
      const doc = {
        buckets: {},
        foo: 'bar',
      };
      (wrapper.vm as any).document = JSON.stringify(doc)
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/db/new database/k/key/`, doc)
    })

    test('should use correct values when saving key changes when using json editor', (done) => {
      // setTimeout to allow the editor to be fully loaded before we interact with it
      setTimeout(async () => {
        const doc = {
          buckets: {},
          foo: 'bar',
        };
        (wrapper.vm as any).editor.set(doc)
        await Vue.nextTick()
        const saveKeyButton = wrapper.find('.save-button')
        saveKeyButton.trigger('click')
        await Vue.nextTick()
        expect(putSpy).toHaveBeenCalledWith(`/conf/api/v1/db/new database/k/key/`, doc)
        done()
      }, 300)
    })

    test('should not be able to save key changes if document is an invalid json when not using json editor', async () => {
      (wrapper.vm as any).editor = null;
      (wrapper.vm as any).isJsonEditor = false
      await Vue.nextTick()
      const doc = '{'
      const documentInput = wrapper.find('.document-input');
      (documentInput.element as any).value = doc
      documentInput.trigger('input')
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).not.toHaveBeenCalled()
    })

    test('should not render normal text area if json editor has been loaded', (done) => {
      // setTimeout to allow the editor to be fully loaded before we interact with it
      setTimeout(async () => {
        const documentInput = wrapper.find('.document-input')
        expect(documentInput.element).toBeUndefined()
        done()
      }, 300)
    })

    test('should default to normal text area when json editor cannot be loaded after 2 seconds', (done) => {
      // @ts-ignore
      JSONEditor.mockImplementation(() => {
        throw new Error('ouchie')
      })
      wrapper = mount(DBEditor)
      // setTimeout to allow the editor to be fully loaded before we interact with it
      setTimeout(async () => {
        const documentInput = wrapper.find('.document-input')
        expect(documentInput.element).toBeDefined()
        done()
      }, 2300)
    })

    test('should not be able to save key changes if database name is empty', async () => {
      const databaseNameInput = wrapper.find('.database-name-input');
      (databaseNameInput.element as any).value = ''
      databaseNameInput.trigger('input')
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).not.toHaveBeenCalled()
    })

    test('should not be able to save key changes if database name is duplicate of another database', async () => {
      const databaseNameInput = wrapper.find('.database-name-input');
      (databaseNameInput.element as any).value = 'databaseCopy'
      databaseNameInput.trigger('input')
      await Vue.nextTick()
      const saveKeyButton = wrapper.find('.save-button')
      saveKeyButton.trigger('click')
      await Vue.nextTick()
      expect(putSpy).not.toHaveBeenCalled()
    })

    test('should not be able to save key changes if key name is empty', async () => {
      const keyNameInput = wrapper.find('.key-name-input');
      (keyNameInput.element as any).value = ''
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
      const keyNameInput = wrapper.find('.key-name-input');
      (keyNameInput.element as any).value = 'key'
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
    test('should display correct message when there is no database list data', (done) => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v1/db/') {
          return Promise.resolve({data: []})
        }
        return Promise.resolve({data: {}})
      })
      wrapper = mount(DBEditor)
      // allow all requests to finish
      setImmediate(() => {
        const noDataMessage = wrapper.find('.no-data-message')
        expect(noDataMessage.element).toBeDefined()
        expect(noDataMessage.text().toLowerCase()).toContain('no data found!')
        expect(noDataMessage.text().toLowerCase()).toContain('missing database.')
        done()
      })
    })

    test('should display correct message when there is no key data', (done) => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v1/db/') {
          return Promise.resolve({data: ['system', 'databaseCopy', 'anotherDB']})
        }
        const db = (wrapper.vm as any).selectedDatabase
        if (path === `/conf/api/v1/db/${db}/`) {
          return Promise.resolve({data: {}})
        }
        return Promise.resolve({
          data: {},
        })
      })
      wrapper = mount(DBEditor)
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
    test('should display loading indicator when databases list not loaded', async () => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v1/db/') {
          return new Promise(() => {
          })
        }
        return Promise.resolve({data: []})
      })
      wrapper = mount(DBEditor)
      await Vue.nextTick()
      const docLoadingIndicator = wrapper.find('.document-loading')
      expect(docLoadingIndicator.element).toBeDefined()
    })

    test('should display loading indicator when database not loaded', async () => {
      jest.spyOn(axios, 'get').mockImplementation((path) => {
        if (path === '/conf/api/v1/db/') {
          return Promise.resolve({data: ['system', 'databaseCopy', 'anotherDB']})
        }
        const db = (wrapper.vm as any).selectedDatabase
        if (path === `/conf/api/v1/db/${db}/`) {
          return new Promise(() => {
          })
        }
        return Promise.resolve({data: {}})
      })
      wrapper = mount(DBEditor)
      await Vue.nextTick()
      const docLoadingIndicator = wrapper.find('.document-loading')
      expect(docLoadingIndicator.element).toBeDefined()
    })

    test('should display loading indicator when saving document changes', async () => {
      jest.spyOn(axios, 'put').mockImplementation(() => new Promise(() => {
      }))
      const saveDocumentButton = wrapper.find('.save-button')
      saveDocumentButton.trigger('click')
      await Vue.nextTick()
      expect(saveDocumentButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when forking database', async () => {
      jest.spyOn(axios, 'post').mockImplementation(() => new Promise(() => {
      }))
      const forkDatabaseButton = wrapper.find('.fork-database-button')
      forkDatabaseButton.trigger('click')
      await Vue.nextTick()
      expect(forkDatabaseButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when adding a new database', async () => {
      jest.spyOn(axios, 'post').mockImplementation(() => new Promise(() => {
      }))
      const newDatabaseButton = wrapper.find('.new-database-button')
      newDatabaseButton.trigger('click')
      await Vue.nextTick()
      expect(newDatabaseButton.element.classList).toContain('is-loading')
    })

    test('should display loading indicator when deleting a database', (done) => {
      jest.spyOn(axios, 'put').mockImplementation(() => Promise.resolve())
      jest.spyOn(axios, 'delete').mockImplementation(() => new Promise(() => {
      }))
      // create new database so we can delete it
      const newDatabaseButton = wrapper.find('.new-database-button')
      newDatabaseButton.trigger('click')
      setImmediate(async () => {
        const deleteDatabaseButton = wrapper.find('.delete-database-button')
        deleteDatabaseButton.trigger('click')
        await Vue.nextTick()
        expect(deleteDatabaseButton.element.classList).toContain('is-loading')
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
      // create new database so we can delete it
      const newDatabaseButton = wrapper.find('.new-key-button')
      newDatabaseButton.trigger('click')
      setImmediate(async () => {
        const deleteDatabaseButton = wrapper.find('.delete-key-button')
        deleteDatabaseButton.trigger('click')
        await Vue.nextTick()
        expect(deleteDatabaseButton.element.classList).toContain('is-loading')
        done()
      })
    })
  })
})
