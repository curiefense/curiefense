<template>
  <div class="card">
    <div class="card-content">
      <div class="media">
        <div class="media-content">
          <div class="columns">
            <div class="column">
              <div class="field is-grouped">
                <div class="control">
                  <div class="select is-small">
                    <select class="database-selection"
                            v-model="selectedDB"
                            @change="switchDB">
                      <option v-for="db in databases"
                              :key="db"
                              :value="db">
                        {{ db }}
                      </option>
                    </select>
                  </div>
                </div>

                <p class="control">
                  <button class="button is-small fork-database-button"
                          :class="{'is-loading': isForkDBLoading}"
                          @click="forkDB"
                          title="Duplicate Database">
                    <span class="icon is-small">
                      <i class="fas fa-clone"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <a class="button is-small download-database-button"
                     @click="downloadDB"
                     title="Download Database">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </a>
                </p>

                <p class="control">
                  <button class="button is-small new-database-button"
                          :class="{'is-loading': isNewDBLoading}"
                          @click="addNewDB()"
                          title="Add New Database">
                    <span class="icon is-small">
                      <i class="fas fa-plus"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small has-text-danger delete-database-button"
                          :class="{'is-loading': isDeleteDBLoading}"
                          @click="deleteDB()"
                          title="Delete Database"
                          :disabled="selectedDB === defaultDBName || databases.length <= 1">
                    <span class="icon is-small">
                      <i class="fas fa-trash"></i>
                    </span>
                  </button>
                </p>
              </div>
            </div>
            <div class="column">
              <div class="field is-grouped is-pulled-right">
                <div class="control">
                  <div class="select is-small">
                    <select class="key-selection"
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
                          title="Duplicate Key">
                    <span class="icon is-small">
                      <i class="fas fa-clone"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <a class="button is-small download-key-button"
                     @click="downloadKey"
                     title="Download Key">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </a>
                </p>

                <p class="control">
                  <button class="button is-small new-key-button"
                          :class="{'is-loading': isNewKeyLoading}"
                          @click="addNewKey()"
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
                          :disabled="(selectedDB === defaultDBName && selectedKey === defaultKeyName) || keys.length <= 1">
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
           v-if="selectedDB && selectedKey">
        <div class="card">
          <div class="card-content">
            <div class="content">
              <div class="field">
                <label class="label">Database</label>
                <div class="control">
                  <input class="input is-small is-fullwidth database-name-input"
                         @input="validateInput($event, isSelectedDBNewNameValid)"
                         type="text"
                         placeholder="Database name"
                         v-model="dbNameInput"
                         :disabled="selectedDB === defaultDBName">
                </div>
              </div>
            </div>

            <div class="content">
              <div class="field">
                <label class="label">Key</label>
                <div class="control">
                  <input class="input is-small is-fullwidth key-name-input"
                         @input="validateInput($event, isSelectedKeyNewNameValid)"
                         type="text"
                         placeholder="Key name"
                         v-model="keyNameInput"
                         :disabled="selectedDB === defaultDBName && selectedKey === defaultKeyName">
                </div>
              </div>
            </div>

            <div class="content">
              <div class="field">
                <label class="label">Document</label>
                <div class="control">

                  <div v-if="isJsonEditor"
                       id="editor">
                  </div>
                  <textarea
                      v-else
                      @input="validateInput($event, isNewDocumentValid)"
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
        <git-history :gitLog.sync="gitLog"
                     :apiPath.sync="gitAPIPath"
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
            <span v-if="!selectedDB">
              Missing database. To create a new one, click
              <a title="Add New"
                 @click="addNewDB()">
                here
              </a>
            </span>
            <span v-if="selectedDB && !selectedKey">
              Missing key. To create a new one, click
              <a title="Add New"
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

<script>

import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import Utils from '@/assets/Utils.ts'
import GitHistory from '@/components/GitHistory'
import RequestsUtils from '@/assets/RequestsUtils'
import JSONEditor from 'jsoneditor'

export default {

  name: 'DBEditor',
  props: {},
  components: {
    GitHistory
  },
  data() {
    return {
      databases: [],
      selectedDB: null,
      dbNameInput: '',
      defaultDBName: 'system',

      // Loading indicators
      loadingDocCounter: 0,
      isForkDBLoading: false,
      isNewDBLoading: false,
      isDeleteDBLoading: false,
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

      selectedDBData: null,
      document: null,

      gitLog: [],

      apiRoot: DatasetsUtils.ConfAPIRoot,
      apiVersion: DatasetsUtils.ConfAPIVersion,
    }
  },
  computed: {

    gitAPIPath() {
      return `${this.apiRoot}/${this.apiVersion}/db/${this.selectedDB}/k/${this.selectedKey}/v/`
    },

    isFormValid() {
      return this.isSelectedDBNewNameValid && this.isSelectedKeyNewNameValid && this.isNewDocumentValid
    },

    isSelectedDBNewNameValid() {
      const newName = this.dbNameInput?.trim()
      const isDBNameEmpty = newName === ''
      const isDBNameDuplicate = this.databases.includes(newName) ? this.selectedDB !== newName : false
      return !isDBNameEmpty && !isDBNameDuplicate
    },

    isSelectedKeyNewNameValid() {
      const newName = this.keyNameInput?.trim()
      const isKeyNameEmpty = newName === ''
      const isKeyNameDuplicate = this.keys.includes(newName) ? this.selectedKey !== newName : false
      return !isKeyNameEmpty && !isKeyNameDuplicate
    },

    isNewDocumentValid() {
      try {
        JSON.parse(this.document)
      } catch {
        return false
      }
      return true
    },

  },

  methods: {

    validateInput(event, validator) {
      Utils.validateInput(event, validator)
    },

    async loadDBs() {
      this.setLoadingDocStatus(true)
      await RequestsUtils.sendRequest('GET', 'db/').then((response) => {
        this.databases = response.data
        console.log('Databases: ', this.databases)
        this.loadFirstDB()
      })
      this.setLoadingDocStatus(false)
    },

    loadFirstDB() {
      const db = this.databases[0]
      if (db) {
        this.loadDB(db)
      } else {
        console.log(`failed loading database, none are present!`)
      }
    },

    async loadDB(db) {
      this.setLoadingDocStatus(true)
      this.selectedDB = db
      this.dbNameInput = this.selectedDB
      this.selectedDBData = (await RequestsUtils.sendRequest('GET', `db/${this.selectedDB}/`)).data
      this.initDBKeys(this.selectedDB)
      this.setLoadingDocStatus(false)
    },

    saveDB(db, data) {
      if (!data) {
        data = {key: {}}
      }

      return RequestsUtils.sendRequest('PUT', `db/${db}/`, data, null, `Database [${db}] saved!`, `Failed saving database [${db}]!`)
    },

    switchDB() {
      this.loadDB(this.selectedDB)
    },

    async deleteDB(db = this.selectedDB, disableAnnouncementMessages) {
      this.isDeleteDBLoading = true
      const db_index = this.ld.findIndex(this.databases, (database) => {
        return database === db
      })
      this.databases.splice(db_index, 1)
      let successMessage
      let failureMessage
      if (!disableAnnouncementMessages) {
        successMessage = `Database [${db}] deleted!`
        failureMessage = `Failed deleting database [${db}]!`
      }
      await RequestsUtils.sendRequest('DELETE', `db/${db}/`, null, null, successMessage, failureMessage)
      if (!this.databases.includes(this.selectedDB)) {
        this.loadFirstDB()
      }
      this.isDeleteDBLoading = false
    },

    async addNewDB(new_db, data) {
      this.isNewDBLoading = true
      if (!new_db) {
        new_db = Utils.generateUniqueEntityName('new database', this.databases)
      }
      await this.saveDB(new_db, data).then(() => {
        this.loadDB(new_db)
        this.databases.unshift(new_db)
      })
      this.isNewDBLoading = false
    },

    async forkDB() {
      this.isForkDBLoading = true
      const new_db = Utils.generateUniqueEntityName(this.selectedDB, this.databases, true)
      await this.addNewDB(new_db, this.selectedDBData)
      this.isForkDBLoading = false
    },

    downloadDB() {
      Utils.downloadFile(this.selectedDB, 'json', this.selectedDBData)
    },

    initDBKeys() {
      this.keys = Object.keys(this.selectedDBData)
      this.loadKey(this.keys[0])
    },

    loadKey(key) {
      this.selectedKey = key
      this.keyNameInput = this.selectedKey
      this.document = JSON.stringify(this.selectedDBData[key])
      this.editor?.set(this.selectedDBData[key])
      this.loadGitLog()
    },

    saveKey(db, key, doc) {
      const parsedDoc = JSON.parse(doc)

      return RequestsUtils.sendRequest('PUT', `db/${db}/k/${key}/`, parsedDoc,  null,`Key [${key}] in database [${db}] saved!`, `Failed saving key [${key}] in database [${db}]!`)
    },

    switchKey() {
      this.loadKey(this.selectedKey)
    },

    async deleteKey(key, disableAnnouncementMessages) {
      this.isDeleteKeyLoading = true
      const db = this.selectedDB
      if (!key) {
        key = this.selectedKey
      }
      const key_index = this.ld.findIndex(this.keys, (k) => {
        return k === key
      })
      this.keys.splice(key_index, 1)
      let successMessage
      let failureMessage
      if (!disableAnnouncementMessages) {
        successMessage = `Key [${key}] in database [${db}] deleted!`
        failureMessage = `Failed deleting key [${key}] in database [${db}]!`
      }
      await RequestsUtils.sendRequest('DELETE', `db/${db}/k/${key}/`, null, null, successMessage, failureMessage)
      if (!this.keys.includes(this.selectedKey)) {
        this.loadKey(this.keys[0])
      }
      this.isDeleteKeyLoading = false
    },

    async addNewKey(new_key, new_document) {
      this.isNewKeyLoading = true
      if (!new_key) {
        new_key = Utils.generateUniqueEntityName('new key', this.keys)
      }
      if (!new_document) {
        new_document = '{}'
      }
      await this.saveKey(this.selectedDB, new_key, new_document).then(() => {
        this.selectedDBData[new_key] = JSON.parse(new_document)
        this.loadKey(new_key)
        this.keys.unshift(new_key)
      })
      this.isNewKeyLoading = false
    },

    async forkKey() {
      this.isForkKeyLoading = true
      const new_key = Utils.generateUniqueEntityName(this.selectedKey, this.keys, true)
      const new_document = this.ld.cloneDeep(this.document)
      await this.addNewKey(new_key, new_document)
      this.isForkKeyLoading = false
    },

    downloadKey() {
      Utils.downloadFile(this.selectedKey, 'json', JSON.parse(this.document))
    },

    async saveChanges() {
      this.isSaveDocLoading = true
      // If DB name changed -> Save the data under the new name and remove the old database
      if (this.selectedDB !== this.dbNameInput) {
        const old_db = this.selectedDB
        const old_data_response = await RequestsUtils.sendRequest('GET', `db/${old_db}/`)
        const old_data = old_data_response.data
        await this.addNewDB(this.dbNameInput, old_data)
        this.deleteDB(old_db, true)
      }
      // If key name changed -> Save the data under the new name and remove the old key from the database
      if (this.selectedKey !== this.keyNameInput) {
        const old_key = this.selectedKey
        await this.addNewKey(this.keyNameInput, this.document)
        this.deleteKey(old_key, true)
      } else {
        await this.saveKey(this.selectedDB, this.selectedKey, this.document)
        this.selectedDBData[this.selectedKey] = JSON.parse(this.document)
      }
      await this.loadGitLog()
      this.isSaveDocLoading = false
    },

    async loadGitLog() {
      const url_trail = `db/${this.selectedDB}/k/${this.selectedKey}/v/`
      const response = await RequestsUtils.sendRequest('GET', url_trail)
      this.gitLog = response.data
    },

    async restoreGitVersion(gitVersion) {
      const db = this.selectedDB
      const selectedKey = this.selectedKey
      const version_id = gitVersion.version
      const url_trail = `${db}/v/${version_id}/`

      await RequestsUtils.sendRequest('PUT', `db/${url_trail}revert/`, null, null, `Database [${db}] restored to version [${version_id}]!`, `Failed restoring database [${db}] to version [${version_id}]!`)
      await this.loadDB(db)
      // load last loaded key if still exists
      const oldSelectedKey = this.keys.find((key) => {
        return key === selectedKey
      })
      if (oldSelectedKey) {
        this.loadKey(oldSelectedKey)
      }
      this.loadGitLog()
    },

    // Collect every request to display a loading indicator, the loading indicator will be displayed as long as at least one request is still active (counter > 0)
    setLoadingDocStatus(isLoading) {
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
          const container = document.getElementById('editor')
          this.editor = new JSONEditor(container,{
            modes: ['code', 'tree'],
            onChange: () => {
              try {
                this.document = JSON.stringify(this.editor.get())
              } catch (err) {
                // editor.get will throw an error when attempting to get an invalid json
              }
            }
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
    this.loadDBs()
  }

}

</script>
<style scoped lang="scss">

@import 'node_modules/bulma/sass/utilities/_all.sass';
@import 'node_modules/bulma/sass/helpers/color.sass';

#editor {
  width: 100%;
  height: 500px;
}

//Design for the json editor
::v-deep .jsoneditor-menu {
  @extend .has-background-grey-light;

  & > button, & > .jsoneditor-modes > button {
    float: none;
  }

  & .jsoneditor-contextmenu .jsoneditor-menu {
    @extend .has-background-white;

    & > li > button.jsoneditor-type-modes {

    }

    & > li > button.jsoneditor-selected {
      @extend .has-background-grey;
    }
  }
}
::v-deep .jsoneditor-contextmenu {
  z-index: 5;
}

</style>
