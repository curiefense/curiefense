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
                    <select v-model="selectedDB"
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
                  <button class="button is-small"
                          @click="forkDB"
                          title="Duplicate Database">
                    <span class="icon is-small">
                      <i class="fas fa-clone"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <a class="button is-small"
                     @click="downloadDB"
                     title="Download Database x">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </a>
                </p>

                <p class="control">
                  <button class="button is-small"
                          @click="addNewDB()"
                          title="Add New Database">
                    <span class="icon is-small">
                      <i class="fas fa-plus"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small has-text-danger"
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
                    <select v-model="selectedKey"
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
                  <button class="button is-small"
                          @click="forkKey"
                          title="Duplicate Key">
                    <span class="icon is-small">
                      <i class="fas fa-clone"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <a class="button is-small"
                     @click="downloadKey"
                     title="Download Key x">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </a>
                </p>

                <p class="control">
                  <button class="button is-small"
                          @click="addNewKey()"
                          title="Add New Key">
                    <span class="icon is-small">
                      <i class="fas fa-plus"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small"
                          @click="saveChanges"
                          title="Save changes"
                          :disabled="!isFormValid">
                    <span class="icon is-small">
                      <i class="fas fa-save"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small has-text-danger"
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
      <div class="content">
        <hr/>
        <div class="card">
          <div class="card-content">
            <div class="content">
              <div class="field">
                <label class="label">Database</label>
                <div class="control">
                  <input class="input is-small is-fullwidth"
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
                  <input class="input is-small is-fullwidth"
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
                  <textarea
                      @input="validateInput($event, isNewDocumentValid)"
                      rows="20"
                      class="is-family-monospace textarea"
                      v-model="document"
                  >
                  </textarea>
                </div>
              </div>
            </div>
          </div>
        </div>
        <hr/>
        <GitHistory :gitLog.sync="gitLog"
                    :gitlogLoading.sync="gitLogLoading"
                    :apiPath.sync="gitAPIPath"
                    @restoreVersion="restoreGitVersion"></GitHistory>
      </div>
    </div>
  </div>
</template>

<script>

import axios from 'axios'
import DatasetsUtils from '@/assets/DatasetsUtils.js'
import Utils from '@/assets/Utils.js'
import GitHistory from "@/components/GitHistory";
import * as bulmaToast from "bulma-toast";

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

      keys: [],
      selectedKey: null,
      keyNameInput: '',
      defaultKeyName: 'publishinfo',

      selectedDBData: null,
      document: null,

      gitLogLoading: false,
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

    loadDBs() {
      this.ax().then((response) => {
        this.databases = response.data
        console.log("Databases: ", this.databases)
        this.loadFirstDB()
      })
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
      this.selectedDB = db
      this.dbNameInput = this.selectedDB
      this.selectedDBData = (await this.ax(axios.get, `${this.selectedDB}/`)).data
      this.initDBKeys(this.selectedDB)
    },

    saveDB(db, data) {
      if (!db) {
        db = this.selectedDB
      }
      if (!data) {
        data = {key: {}}
      }

      return this.ax(axios.put, `${db}/`, data)
          .then(() => {
            this.successToast(`Database [${db}] saved!`)
            return true
          })
          .catch(() => {
            this.failToast(`Failed saving database [${db}]!`)
            return false
          })

    },

    switchDB() {
      this.loadDB(this.selectedDB)
    },

    deleteDB(db) {
      if (!db) {
        db = this.selectedDB
      }
      const db_index = this.ld.findIndex(this.databases, (database) => {
        return database === db
      })
      this.databases.splice(db_index, 1)
      this.ax(axios.delete, `${db}/`)
          .then(() => {
            this.successToast(`Database [${db}] deleted!`)
          })
          .catch(() => {
            this.failToast(`Failed deleting database [${db}]!`)
          })
      if (!this.databases.includes(this.selectedDB)) {
        this.loadFirstDB()
      }
    },

    async addNewDB(new_db, data) {
      if (!new_db) {
        new_db = Utils.generateUniqueEntityName('new database', this.databases)
      }
      const isSaved = await this.saveDB(new_db, data)
      if (isSaved) {
        this.loadDB(new_db)
        this.databases.unshift(new_db)
      }
    },

    forkDB() {
      const new_db = Utils.generateUniqueEntityName(this.selectedDB, this.databases, true)
      this.addNewDB(new_db, this.selectedDBData)
    },

    downloadDB(event) {
      let element = event.target;
      while (element.nodeName !== "A")
        element = element.parentNode

      let dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(this.selectedDBData))
      element.setAttribute("href", dataStr)
      element.setAttribute("download", this.selectedDB + ".json")
    },

    initDBKeys() {
      this.keys = Object.keys(this.selectedDBData)
      this.loadKey(this.keys[0])
    },

    loadKey(key) {
      this.selectedKey = key
      this.keyNameInput = this.selectedKey
      this.document = JSON.stringify(this.selectedDBData[key], null, 2)
      this.loadGitLog()
    },

    async saveKey(db, key, doc) {
      if (!db) {
        db = this.selectedDB
      }
      if (!key) {
        key = this.selectedKey
      }
      if (!doc) {
        doc = this.document
      }
      doc = JSON.parse(doc)

      return this.ax(axios.put, `${db}/k/${key}/`, doc)
          .then(() => {
            this.successToast(`Key [${key}] in database [${db}] saved!`)
            return true
          })
          .catch(() => {
            this.failToast(`Failed saving key [${key}] in database [${db}]!`)
            return false
          })

    },

    switchKey() {
      this.loadKey(this.selectedKey)
    },

    deleteKey(key) {
      const db = this.selectedDB
      if (!key) {
        key = this.selectedKey
      }
      const key_index = this.ld.findIndex(this.keys, (k) => {
        return k === key
      })
      this.keys.splice(key_index, 1)
      this.ax(axios.delete, `${db}/k/${key}/`)
          .then(() => {
            this.successToast(`Key [${key}] in database [${db}] deleted!`)
          })
          .catch(() => {
            this.failToast(`Failed deleting key [${key}] in database [${db}]!`)
          })
      if (!this.keys.includes(this.selectedKey)) {
        this.loadKey(this.keys[0])
      }
    },

    async addNewKey(new_key, new_document) {
      if (!new_key) {
        new_key = Utils.generateUniqueEntityName('ney key', this.keys)
      }
      if (!new_document) {
        new_document = '{}'
      }
      const isSaved = await this.saveKey(this.selectedDB, new_key, new_document)
      if (isSaved) {
        this.selectedDBData[new_key] = JSON.parse(new_document)
        this.loadKey(new_key)
        this.keys.unshift(new_key)
      }
    },

    forkKey() {
      const new_key = Utils.generateUniqueEntityName(this.selectedKey, this.keys, true)
      const new_document = this.ld.cloneDeep(this.document)
      this.addNewKey(new_key, new_document)
    },

    downloadKey(event) {
      let element = event.target;
      while (element.nodeName !== "A")
        element = element.parentNode

      let dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(this.document)
      element.setAttribute("href", dataStr)
      element.setAttribute("download", this.selectedKey + ".json")
    },

    async saveChanges() {
      // If DB name changed -> Save the data under the new name and remove the old database
      if (this.selectedDB !== this.dbNameInput) {
        const old_db = this.selectedDB
        const old_data = (await this.ax(axios.get, `${old_db}/`)).data
        await this.addNewDB(this.dbNameInput, old_data)
        this.deleteDB(old_db)
      }
      // If key name changed -> Save the data under the new name and remove the old key from the database
      if (this.selectedKey !== this.keyNameInput) {
        const old_key = this.selectedKey
        await this.addNewKey(this.keyNameInput, this.document)
        this.deleteKey(old_key)
      } else {
        await this.saveKey(this.selectedDB, this.selectedKey, this.document)
        this.selectedDBData[this.selectedKey] = JSON.parse(this.document)
      }
      await this.loadGitLog()
    },

    resetGitLog() {
      this.gitLog = []
    },

    async loadGitLog() {
      this.gitlogLoading = true;
      const url_trail = `${this.selectedDB}/k/${this.selectedKey}/v/`
      const response = await this.ax(axios.get, url_trail)
      this.gitLog = response.data
      this.gitlogLoading = false
    },

    async restoreGitVersion(gitVersion) {
      const db = this.selectedDB
      const selectedKey = this.selectedKey
      const version_id = gitVersion.version
      const url_trail = `${db}/v/${version_id}/`

      await this.ax(axios.put, `${url_trail}revert/`)
      await this.loadDB(db)
      const oldSelectedKey = this.keys.find((key) => {
        return key === selectedKey
      })
      if (oldSelectedKey) {
        this.loadKey(oldSelectedKey)
      }
      this.loadGitLog()
    },

    ax(axios_method, urlTail, data) {
      if (!axios_method) {
        axios_method = axios.get
      }
      if (!urlTail) {
        urlTail = ""
      }

      const apiroot = this.apiRoot
      const apiversion = this.apiVersion
      const apiurl = `${apiroot}/${apiversion}/db/${urlTail}`

      if (axios_method) {
        console.log("apiurl", apiurl)
        if (data) {
          return axios_method(apiurl, data)
        }
        return axios_method(apiurl)
      }
    },

    toast(message, type) {
      bulmaToast.toast(
          {
            message: message,
            type: `is-light ${type}`,
            position: 'bottom-left',
            closeOnClick: true,
            pauseOnHover: true,
            duration: 3000,
            opacity: 0.8,
          }
      )
    },

    successToast(message) {
      this.toast(message, 'is-success')
    },

    failToast(message) {
      this.toast(message, 'is-danger')
    },

  },

  created() {
    this.loadDBs()
  }

}

</script>
