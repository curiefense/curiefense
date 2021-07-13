<template>
  <div class="card">
    <div class="card-content">
      <div class="media">
        <div class="media-content">
          <div class="columns">
            <div class="column">
              <div class="field is-grouped">
                <div class="control" v-if="databases.length">
                  <div class="select is-small">
                    <select class="database-selection"
                            title="Switch database"
                            v-model="selectedDatabase"
                            @change="switchDatabase">
                      <option v-for="database in databases"
                              :key="database"
                              :value="database">
                        {{ database }}
                      </option>
                    </select>
                  </div>
                </div>

                <p class="control">
                  <button class="button is-small fork-database-button"
                          :class="{'is-loading': isForkDatabaseLoading}"
                          @click="forkDatabase"
                          :disabled="!selectedDatabase"
                          title="Duplicate database">
                    <span class="icon is-small">
                      <i class="fas fa-clone"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small download-database-button"
                     @click="downloadDatabase"
                     :disabled="!selectedDatabase"
                     title="Download database">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small new-database-button"
                          :class="{'is-loading': isNewDatabaseLoading}"
                          @click="addNewDatabase()"
                          title="Add new database">
                    <span class="icon is-small">
                      <i class="fas fa-plus"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small has-text-danger delete-database-button"
                          :class="{'is-loading': isDeleteDatabaseLoading}"
                          @click="deleteDatabase()"
                          title="Delete database"
                          :disabled="selectedDatabase === defaultDatabaseName || databases.length <= 1">
                    <span class="icon is-small">
                      <i class="fas fa-trash"></i>
                    </span>
                  </button>
                </p>
              </div>
            </div>
            <div class="column">
              <div class="field is-grouped is-pulled-right">
                <div class="control" v-if="keys.length">
                  <div class="select is-small">
                    <select class="key-selection"
                            title="Switch key"
                            v-model="selectedKey"
                            @change="switchKey">
                      <option v-for="key in keys"
                              :key="key"
                              :value="key">
                        {{ key }}
                      </option>
                    </select>
                  </div>
                </div>

                <p class="control">
                  <button class="button is-small fork-key-button"
                          :class="{'is-loading': isForkKeyLoading}"
                          @click="forkKey"
                          :disabled="!selectedDatabase"
                          title="Duplicate Key">
                    <span class="icon is-small">
                      <i class="fas fa-clone"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small download-key-button"
                     @click="downloadKey"
                     :disabled="!document"
                     title="Download Key">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small new-key-button"
                          :class="{'is-loading': isNewKeyLoading}"
                          @click="addNewKey()"
                          :disabled="!selectedDatabase"
                          title="Add New Key">
                    <span class="icon is-small">
                      <i class="fas fa-plus"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small save-button"
                          :class="{'is-loading': isSaveDocLoading}"
                          @click="saveChanges"
                          title="Save changes"
                          :disabled="!isFormValid">
                    <span class="icon is-small">
                      <i class="fas fa-save"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small has-text-danger delete-key-button"
                          :class="{'is-loading': isDeleteKeyLoading}"
                          @click="deleteKey()"
                          title="Delete Key"
                          :disabled="(selectedDatabase === defaultDatabaseName && selectedKey === defaultKeyName)
                                     || keys.length <= 1">
                    <span class="icon is-small">
                      <i class="fas fa-trash"></i>
                    </span>
                  </button>
                </p>

              </div>
            </div>
          </div>
        </div>
      </div>

      <hr/>

      <div class="content"
           v-if="selectedDatabase && selectedKey">
        <div class="card">
          <div class="card-content">
            <div class="content">
              <div class="field">
                <label class="label">Database</label>
                <div class="control">
                  <input class="input is-small is-fullwidth database-name-input"
                         title="Database name"
                         @input="validateInput($event, isSelectedDatabaseNewNameValid)"
                         type="text"
                         placeholder="Database name"
                         v-model="databaseNameInput"
                         :disabled="selectedDatabase === defaultDatabaseName">
                </div>
              </div>
            </div>

            <div class="content">
              <div class="field">
                <label class="label">Key</label>
                <div class="control">
                  <input class="input is-small is-fullwidth key-name-input"
                         title="Key name"
                         @input="validateInput($event, isSelectedKeyNewNameValid)"
                         type="text"
                         placeholder="Key name"
                         v-model="keyNameInput"
                         :disabled="selectedDatabase === defaultDatabaseName && selectedKey === defaultKeyName">
                </div>
              </div>
            </div>

            <div class="content">
              <div class="field">
                <label class="label">Document</label>
                <div class="control">

                  <div v-if="isJsonEditor"
                       class="editor">
                  </div>
                  <textarea
                      v-else
                      @input="validateInput($event, isNewDocumentValid)"
                      title="Document"
                      rows="20"
                      class="is-family-monospace textarea document-input"
                      v-model="document">
                  </textarea>
                </div>
              </div>
            </div>
          </div>
        </div>
        <hr/>
        <git-history :gitLog="gitLog"
                     :apiPath="gitAPIPath"
                     :loading="loadingGitlog"
                     @restore-version="restoreGitVersion"></git-history>
      </div>

      <div class="content no-data-wrapper"
           v-else>
        <div v-if="loadingDocCounter > 0">
          <button class="button is-outlined is-text is-small is-loading document-loading">
            Loading
          </button>
        </div>
        <div v-else
             class="no-data-message">
          No data found!
          <div>
            <!--display correct message by priority (Database -> Key)-->
            <span v-if="!selectedDatabase">
              Missing database. To create a new one, click
              <a title="Add new"
                 @click="addNewDatabase()">
                here
              </a>
            </span>
            <span v-if="selectedDatabase && !selectedKey">
              Missing key. To create a new one, click
              <a title="Add new"
                 @click="addNewKey()">
                here
              </a>
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import _ from 'lodash'
import RequestsUtils from '@/assets/RequestsUtils.ts'
import Utils from '@/assets/Utils.ts'
import GitHistory from '@/components/GitHistory.vue'
import JSONEditor from 'jsoneditor'
import Vue from 'vue'
import {Commit, GenericObject} from '@/types'
import {AxiosResponse} from 'axios'

export default Vue.extend({

  name: 'DBEditor',
  props: {},
  components: {
    GitHistory,
  },
  data() {
    return {
      databases: [],
      selectedDatabase: null,
      databaseNameInput: '',
      defaultDatabaseName: 'system',

      // Loading indicators
      loadingDocCounter: 0,
      isForkDatabaseLoading: false,
      isNewDatabaseLoading: false,
      isDeleteDatabaseLoading: false,
      isForkKeyLoading: false,
      isNewKeyLoading: false,
      isDeleteKeyLoading: false,
      isSaveDocLoading: false,

      // json editor
      editor: null,
      isJsonEditor: true,

      keys: [],
      selectedKey: null,
      keyNameInput: '',
      defaultKeyName: 'publishinfo',

      selectedDatabaseData: {} as GenericObject,
      document: null,

      gitLog: [] as Commit[],
      loadingGitlog: false,

      apiRoot: RequestsUtils.confAPIRoot,
      apiVersion: RequestsUtils.confAPIVersion,
    }
  },
  computed: {

    gitAPIPath(): string {
      return `${this.apiRoot}/${this.apiVersion}/db/${this.selectedDatabase}/k/${this.selectedKey}/v/`
    },

    isFormValid(): boolean {
      return this.isSelectedDatabaseNewNameValid && this.isSelectedKeyNewNameValid && this.isNewDocumentValid
    },

    isSelectedDatabaseNewNameValid(): boolean {
      const newName = this.databaseNameInput?.trim()
      const isDatabaseNameEmpty = newName === ''
      const isDatabaseNameDuplicate = this.databases.includes(newName) ? this.selectedDatabase !== newName : false
      return !isDatabaseNameEmpty && !isDatabaseNameDuplicate
    },

    isSelectedKeyNewNameValid(): boolean {
      const newName = this.keyNameInput?.trim()
      const isKeyNameEmpty = newName === ''
      const isKeyNameDuplicate = this.keys.includes(newName) ? this.selectedKey !== newName : false
      return !isKeyNameEmpty && !isKeyNameDuplicate
    },

    isNewDocumentValid(): boolean {
      try {
        JSON.parse(this.document)
      } catch {
        return false
      }
      return true
    },
  },

  methods: {

    validateInput(event: Event, validator: Function | boolean) {
      Utils.validateInput(event, validator)
    },

    async loadDatabases() {
      this.setLoadingDocStatus(true)
      await RequestsUtils.sendRequest({methodName: 'GET', url: 'db/'}).then((response: AxiosResponse<string[]>) => {
        this.databases = response?.data || []
        console.log('Databases: ', this.databases)
        this.loadFirstDatabase()
      })
      this.setLoadingDocStatus(false)
    },

    loadFirstDatabase() {
      const database = this.databases?.[0]
      if (database) {
        this.loadDatabase(database)
      } else {
        console.log(`failed loading database, none are present!`)
      }
    },

    async loadDatabase(database: string) {
      this.setLoadingDocStatus(true)
      this.selectedDatabase = database
      this.databaseNameInput = this.selectedDatabase
      const response = await RequestsUtils.sendRequest({methodName: 'GET', url: `db/${this.selectedDatabase}/`})
      this.selectedDatabaseData = response?.data || {}
      this.initDatabaseKeys()
      this.setLoadingDocStatus(false)
    },

    switchDatabase() {
      this.loadDatabase(this.selectedDatabase)
      Utils.toast(`Switched to database "${this.selectedDatabase}".`, 'is-info')
    },

    async deleteDatabase(database?: string, disableAnnouncementMessages?: boolean) {
      this.isDeleteDatabaseLoading = true
      if (!database) {
        database = this.selectedDatabase
      }
      const databaseIndex = _.findIndex(this.databases, (db) => {
        return db === database
      })
      this.databases.splice(databaseIndex, 1)
      let successMessage
      let failureMessage
      if (!disableAnnouncementMessages) {
        successMessage = `Database "${database}" was deleted.`
        failureMessage = `Failed while attempting to delete database "${database}".`
      }
      await RequestsUtils.sendRequest({methodName: 'DELETE', url: `db/${database}/`, successMessage, failureMessage})
      if (!this.databases.includes(this.selectedDatabase)) {
        this.loadFirstDatabase()
      }
      this.isDeleteDatabaseLoading = false
    },

    async addNewDatabase(newDatabase?: string, data?: { [key: string]: any }) {
      this.isNewDatabaseLoading = true
      if (!newDatabase) {
        newDatabase = Utils.generateUniqueEntityName('new database', this.databases)
      }
      if (!data) {
        data = {key: {}}
      }
      await RequestsUtils.sendRequest({
        methodName: 'PUT',
        url: `db/${newDatabase}/`,
        data,
        successMessage: `Database "${newDatabase}" was saved`,
        failureMessage: `Failed while attempting to create the new database.`,
      }).then(() => {
        this.loadDatabase(newDatabase)
        this.databases.unshift(newDatabase)
      })
      this.isNewDatabaseLoading = false
    },

    async forkDatabase() {
      this.isForkDatabaseLoading = true
      const newDatabase = Utils.generateUniqueEntityName(this.selectedDatabase, this.databases, true)
      await this.addNewDatabase(newDatabase, this.selectedDatabaseData)
      this.isForkDatabaseLoading = false
    },

    downloadDatabase() {
      Utils.downloadFile(this.selectedDatabase, 'json', this.selectedDatabaseData)
    },

    initDatabaseKeys() {
      this.keys = Object.keys(this.selectedDatabaseData)
      this.loadKey(this.keys[0])
    },

    loadKey(key: string) {
      this.selectedKey = key
      this.keyNameInput = this.selectedKey
      this.document = JSON.stringify(this.selectedDatabaseData[key])
      this.editor?.set(this.selectedDatabaseData[key])
      this.loadGitLog()
    },

    saveKey(database: string, key: string, doc: string) {
      const parsedDoc = JSON.parse(doc)
      return RequestsUtils.sendRequest({
        methodName: 'PUT',
        url: `db/${database}/k/${key}/`,
        data: parsedDoc,
        successMessage: `Key "${key}" in database "${database}" was saved.`,
        failureMessage: `Failed while attempting to save key "${key}" in database "${database}".`,
      })
    },

    switchKey() {
      this.loadKey(this.selectedKey)
      Utils.toast(`Switched to key "${this.selectedKey}".`, 'is-info')
    },

    async deleteKey(key?: string, disableAnnouncementMessages?: boolean) {
      this.isDeleteKeyLoading = true
      const database = this.selectedDatabase
      if (!key) {
        key = this.selectedKey
      }
      const keyIndex = _.findIndex(this.keys, (k) => {
        return k === key
      })
      this.keys.splice(keyIndex, 1)
      let successMessage
      let failureMessage
      if (!disableAnnouncementMessages) {
        successMessage = `Key "${key}" in database "${database}" was deleted.`
        failureMessage = `Failed while attempting to delete key "${key}" in database "${database}".`
      }
      await RequestsUtils.sendRequest({
        methodName: 'DELETE',
        url: `db/${database}/k/${key}/`,
        successMessage,
        failureMessage,
      })
      if (!this.keys.includes(this.selectedKey)) {
        this.loadKey(this.keys[0])
      }
      this.isDeleteKeyLoading = false
    },

    async addNewKey(newKey?: string, newDocument?: string) {
      if (!this.selectedDatabase) {
        return
      }
      this.isNewKeyLoading = true
      if (!newKey) {
        newKey = Utils.generateUniqueEntityName('new key', this.keys)
      }
      if (!newDocument) {
        newDocument = '{}'
      }
      await this.saveKey(this.selectedDatabase, newKey, newDocument).then(() => {
        this.selectedDatabaseData[newKey] = JSON.parse(newDocument)
        this.loadKey(newKey)
        this.keys.unshift(newKey)
      })
      this.isNewKeyLoading = false
    },

    async forkKey() {
      this.isForkKeyLoading = true
      const newKey = Utils.generateUniqueEntityName(this.selectedKey, this.keys, true)
      const newDocument = _.cloneDeep(this.document)
      await this.addNewKey(newKey, newDocument)
      this.isForkKeyLoading = false
    },

    downloadKey() {
      if (!this.document) {
        return
      }
      Utils.downloadFile(this.selectedKey, 'json', JSON.parse(this.document))
    },

    async saveChanges() {
      this.isSaveDocLoading = true
      if (this.selectedDatabase === this.databaseNameInput && this.selectedKey === this.keyNameInput) {
        // If database name and key name did not change - save normally
        await this.saveKey(this.selectedDatabase, this.selectedKey, this.document)
        this.selectedDatabaseData[this.selectedKey] = JSON.parse(this.document)
      } else if (this.selectedDatabase !== this.databaseNameInput) {
        // If database name changed -> Save the data under the new name and remove the old database
        const oldDatabase = this.selectedDatabase
        const oldDataResponse = await RequestsUtils.sendRequest({methodName: 'GET', url: `db/${oldDatabase}/`})
        const data = oldDataResponse.data
        const oldKey = this.selectedKey
        delete data[oldKey]
        data[this.keyNameInput] = JSON.parse(this.document)
        await this.addNewDatabase(this.databaseNameInput, data)
        await this.deleteDatabase(oldDatabase, true)
      } else {
        // If key name changed -> Save the data under the new name and remove the old key from the database
        const oldKey = this.selectedKey
        await this.addNewKey(this.keyNameInput, this.document)
        await this.deleteKey(oldKey, true)
      }
      await this.loadGitLog()
      this.isSaveDocLoading = false
    },

    async loadGitLog() {
      this.loadingGitlog = true
      const url = `db/${this.selectedDatabase}/k/${this.selectedKey}/v/`
      const response = await RequestsUtils.sendRequest({methodName: 'GET', url})
      this.gitLog = response?.data
      this.loadingGitlog = false
    },

    async restoreGitVersion(gitVersion: Commit) {
      const database = this.selectedDatabase
      const selectedKey = this.selectedKey
      const versionId = gitVersion.version
      const urlTrail = `${database}/v/${versionId}/`
      await RequestsUtils.sendRequest({
        methodName: 'PUT',
        url: `db/${urlTrail}revert/`,
        successMessage: `Database [${database}] restored to version [${versionId}]!`,
        failureMessage: `Failed restoring database [${database}] to version [${versionId}]!`,
      })
      await this.loadDatabase(database)
      // load last loaded key if still exists
      const oldSelectedKey = this.keys.find((key) => {
        return key === selectedKey
      })
      if (oldSelectedKey) {
        this.loadKey(oldSelectedKey)
      }
      await this.loadGitLog()
    },

    // Collect every request to display a loading indicator
    // The loading indicator will be displayed as long as at least one request is still active (counter > 0)
    setLoadingDocStatus(isLoading: boolean) {
      if (isLoading) {
        this.loadingDocCounter++
      } else {
        this.loadingDocCounter--
      }
    },

    loadJsonEditor() {
      let failedCounter = 0
      const editorLoaderInterval = setInterval(() => {
        try {
          const container = document.getElementsByClassName('editor')[0] as HTMLElement
          this.editor = new JSONEditor(container, {
            modes: ['code', 'tree'],
            onChange: () => {
              try {
                this.document = JSON.stringify(this.editor.get())
              } catch (err) {
                // editor.get will throw an error when attempting to get an invalid json
              }
            },
          }, JSON.parse(this.document))
          this.isJsonEditor = true
          console.log('Successfully loaded json editor')
          clearInterval(editorLoaderInterval)
        } catch (err) {
          failedCounter++
          console.log('Could not load json editor, trying again')
        }
        if (failedCounter > 20) {
          // If cannot load the editor after 2 seconds, display a normal textarea instead
          console.log('Could not load json editor for over 2 seconds, using a simplified editor instead')
          this.isJsonEditor = false
          clearInterval(editorLoaderInterval)
        }
      }, 100)
    },
  },

  mounted() {
    this.loadJsonEditor()
  },

  created() {
    this.loadDatabases()
  },

})
</script>
<style scoped lang="scss">

@import 'node_modules/bulma/sass/utilities/initial-variables.sass';
@import 'node_modules/bulma/sass/utilities/functions.sass';
@import 'node_modules/bulma/sass/utilities/derived-variables.sass';
@import 'node_modules/bulma/sass/helpers/color.sass';

.editor {
  height: 500px;
  width: 100%;
}

//Design for the json editor
::v-deep .jsoneditor-menu {
  @extend .has-background-grey-light;

  button {
    float: none;
  }

  .jsoneditor-menu {
    @extend .has-background-white;
  }

  .jsoneditor-selected {
    @extend .has-background-grey;
  }
}

::v-deep .jsoneditor-contextmenu {
  z-index: 5;
}

</style>
