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
                    <select class="namespace-selection"
                            title="Switch namespace"
                            v-model="selectedNamespace"
                            @change="switchNamespace">
                      <option v-for="namespace in databases"
                              :key="namespace"
                              :value="namespace">
                        {{ namespace }}
                      </option>
                    </select>
                  </div>
                </div>

                <p class="control">
                  <button class="button is-small fork-namespace-button"
                          :class="{'is-loading': isForkNamespaceLoading}"
                          @click="forkNamespace"
                          :disabled="!selectedNamespace"
                          title="Duplicate namespace">
                    <span class="icon is-small">
                      <i class="fas fa-clone"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small download-namespace-button"
                     @click="downloadNamespace"
                     :disabled="!selectedNamespace"
                     title="Download namespace">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small new-namespace-button"
                          :class="{'is-loading': isNewNamespaceLoading}"
                          @click="addNewNamespace()"
                          title="Add new namespace">
                    <span class="icon is-small">
                      <i class="fas fa-plus"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <button class="button is-small has-text-danger delete-namespace-button"
                          :class="{'is-loading': isDeleteNamespaceLoading}"
                          @click="deleteNamespace()"
                          title="Delete namespace"
                          :disabled="selectedNamespace === defaultNamespaceName || databases.length <= 1">
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
                          :disabled="!selectedNamespace"
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
                          :disabled="!selectedNamespace"
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
                          :disabled="(selectedNamespace === defaultNamespaceName && selectedKey === defaultKeyName)
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
           v-if="selectedNamespace && selectedKey">
        <div class="card">
          <div class="card-content">
            <div class="content">
              <div class="field">
                <label class="label">Namespace</label>
                <div class="control">
                  <input class="input is-small is-fullwidth namespace-name-input"
                         title="Namespace name"
                         @input="validateInput($event, isSelectedNamespaceNewNameValid)"
                         type="text"
                         placeholder="Namespace name"
                         v-model="namespaceNameInput"
                         :disabled="selectedNamespace === defaultNamespaceName">
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
                         :disabled="selectedNamespace === defaultNamespaceName && selectedKey === defaultKeyName">
                </div>
              </div>
            </div>

            <div class="content">
              <div class="field">
                <label class="label">Value</label>
                <div class="control">

                  <div v-if="isJsonEditor"
                       class="editor">
                  </div>
                  <textarea
                      v-else
                      @input="validateInput($event, isNewValueValid)"
                      title="Document"
                      rows="20"
                      class="is-family-monospace textarea value-input"
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
          <button class="button is-outlined is-text is-small is-loading value-loading">
            Loading
          </button>
        </div>
        <div v-else
             class="no-data-message">
          No data found!
          <div>
            <!--display correct message by priority (Namespace -> Key)-->
            <span v-if="!selectedNamespace">
              Missing namespace. To create a new one, click
              <a title="Add new"
                 @click="addNewNamespace()">
                here
              </a>
            </span>
            <span v-if="selectedNamespace && !selectedKey">
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
      selectedNamespace: null,
      namespaceNameInput: '',
      defaultNamespaceName: 'system',

      // Loading indicators
      loadingDocCounter: 0,
      isForkNamespaceLoading: false,
      isNewNamespaceLoading: false,
      isDeleteNamespaceLoading: false,
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

      selectedNamespaceData: {} as GenericObject,
      document: null,

      gitLog: [] as Commit[],
      loadingGitlog: false,

      apiRoot: RequestsUtils.confAPIRoot,
      apiVersion: RequestsUtils.confAPIVersion,
    }
  },
  computed: {

    gitAPIPath(): string {
      return `${this.apiRoot}/${this.apiVersion}/db/${this.selectedNamespace}/k/${this.selectedKey}/v/`
    },

    isFormValid(): boolean {
      return this.isSelectedNamespaceNewNameValid && this.isSelectedKeyNewNameValid && this.isNewValueValid
    },

    isSelectedNamespaceNewNameValid(): boolean {
      const newName = this.namespaceNameInput?.trim()
      const isNamespaceNameEmpty = newName === ''
      const isNamespaceNameDuplicate = this.databases.includes(newName) ? this.selectedNamespace !== newName : false
      return !isNamespaceNameEmpty && !isNamespaceNameDuplicate
    },

    isSelectedKeyNewNameValid(): boolean {
      const newName = this.keyNameInput?.trim()
      const isKeyNameEmpty = newName === ''
      const isKeyNameDuplicate = this.keys.includes(newName) ? this.selectedKey !== newName : false
      return !isKeyNameEmpty && !isKeyNameDuplicate
    },

    isNewValueValid(): boolean {
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
        this.loadFirstNamespace()
      })
      this.setLoadingDocStatus(false)
    },

    loadFirstNamespace() {
      const namespace = this.databases?.[0]
      if (namespace) {
        this.loadNamespace(namespace)
      } else {
        console.log(`failed loading namespace, none are present!`)
      }
    },

    async loadNamespace(namespace: string) {
      this.setLoadingDocStatus(true)
      this.selectedNamespace = namespace
      this.namespaceNameInput = this.selectedNamespace
      const response = await RequestsUtils.sendRequest({methodName: 'GET', url: `db/${this.selectedNamespace}/`})
      this.selectedNamespaceData = response?.data || {}
      this.initNamespaceKeys()
      this.setLoadingDocStatus(false)
    },

    switchNamespace() {
      this.loadNamespace(this.selectedNamespace)
      Utils.toast(`Switched to namespace "${this.selectedNamespace}".`, 'is-info')
    },

    async deleteNamespace(namespace?: string, disableAnnouncementMessages?: boolean) {
      this.isDeleteNamespaceLoading = true
      if (!namespace) {
        namespace = this.selectedNamespace
      }
      const namespaceIndex = _.findIndex(this.databases, (db) => {
        return db === namespace
      })
      this.databases.splice(namespaceIndex, 1)
      let successMessage
      let failureMessage
      if (!disableAnnouncementMessages) {
        successMessage = `Namespace "${namespace}" was deleted.`
        failureMessage = `Failed while attempting to delete namespace "${namespace}".`
      }
      await RequestsUtils.sendRequest({methodName: 'DELETE', url: `db/${namespace}/`, successMessage, failureMessage})
      if (!this.databases.includes(this.selectedNamespace)) {
        this.loadFirstNamespace()
      }
      this.isDeleteNamespaceLoading = false
    },

    async addNewNamespace(newNamespace?: string, data?: { [key: string]: any }) {
      this.isNewNamespaceLoading = true
      if (!newNamespace) {
        newNamespace = Utils.generateUniqueEntityName('new namespace', this.databases)
      }
      if (!data) {
        data = {key: {}}
      }
      await RequestsUtils.sendRequest({
        methodName: 'PUT',
        url: `db/${newNamespace}/`,
        data,
        successMessage: `Namespace "${newNamespace}" was saved`,
        failureMessage: `Failed while attempting to create the new namespace.`,
      }).then(() => {
        this.loadNamespace(newNamespace)
        this.databases.unshift(newNamespace)
      })
      this.isNewNamespaceLoading = false
    },

    async forkNamespace() {
      this.isForkNamespaceLoading = true
      const newNamespace = Utils.generateUniqueEntityName(this.selectedNamespace, this.databases, true)
      await this.addNewNamespace(newNamespace, this.selectedNamespaceData)
      this.isForkNamespaceLoading = false
    },

    downloadNamespace() {
      Utils.downloadFile(this.selectedNamespace, 'json', this.selectedNamespaceData)
    },

    initNamespaceKeys() {
      this.keys = Object.keys(this.selectedNamespaceData)
      this.loadKey(this.keys[0])
    },

    loadKey(key: string) {
      this.selectedKey = key
      this.keyNameInput = this.selectedKey
      this.document = JSON.stringify(this.selectedNamespaceData[key])
      this.editor?.set(this.selectedNamespaceData[key])
      this.loadGitLog()
    },

    saveKey(namespace: string, key: string, doc: string) {
      const parsedDoc = JSON.parse(doc)
      return RequestsUtils.sendRequest({
        methodName: 'PUT',
        url: `db/${namespace}/k/${key}/`,
        data: parsedDoc,
        successMessage: `Key "${key}" in namespace "${namespace}" was saved.`,
        failureMessage: `Failed while attempting to save key "${key}" in namespace "${namespace}".`,
      })
    },

    switchKey() {
      this.loadKey(this.selectedKey)
      Utils.toast(`Switched to key "${this.selectedKey}".`, 'is-info')
    },

    async deleteKey(key?: string, disableAnnouncementMessages?: boolean) {
      this.isDeleteKeyLoading = true
      const namespace = this.selectedNamespace
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
        successMessage = `Key "${key}" in namespace "${namespace}" was deleted.`
        failureMessage = `Failed while attempting to delete key "${key}" in namespace "${namespace}".`
      }
      await RequestsUtils.sendRequest({
        methodName: 'DELETE',
        url: `db/${namespace}/k/${key}/`,
        successMessage,
        failureMessage,
      })
      if (!this.keys.includes(this.selectedKey)) {
        this.loadKey(this.keys[0])
      }
      this.isDeleteKeyLoading = false
    },

    async addNewKey(newKey?: string, newDocument?: string) {
      if (!this.selectedNamespace) {
        return
      }
      this.isNewKeyLoading = true
      if (!newKey) {
        newKey = Utils.generateUniqueEntityName('new key', this.keys)
      }
      if (!newDocument) {
        newDocument = '{}'
      }
      await this.saveKey(this.selectedNamespace, newKey, newDocument).then(() => {
        this.selectedNamespaceData[newKey] = JSON.parse(newDocument)
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
      if (this.selectedNamespace === this.namespaceNameInput && this.selectedKey === this.keyNameInput) {
        // If namespace name and key name did not change - save normally
        await this.saveKey(this.selectedNamespace, this.selectedKey, this.document)
        this.selectedNamespaceData[this.selectedKey] = JSON.parse(this.document)
      } else if (this.selectedNamespace !== this.namespaceNameInput) {
        // If namespace name changed -> Save the data under the new name and remove the old namespace
        const oldNamespace = this.selectedNamespace
        const oldDataResponse = await RequestsUtils.sendRequest({methodName: 'GET', url: `db/${oldNamespace}/`})
        const data = oldDataResponse.data
        const oldKey = this.selectedKey
        delete data[oldKey]
        data[this.keyNameInput] = JSON.parse(this.document)
        await this.addNewNamespace(this.namespaceNameInput, data)
        await this.deleteNamespace(oldNamespace, true)
      } else {
        // If key name changed -> Save the data under the new name and remove the old key from the namespace
        const oldKey = this.selectedKey
        await this.addNewKey(this.keyNameInput, this.document)
        await this.deleteKey(oldKey, true)
      }
      await this.loadGitLog()
      this.isSaveDocLoading = false
    },

    async loadGitLog() {
      this.loadingGitlog = true
      const url = `db/${this.selectedNamespace}/k/${this.selectedKey}/v/`
      const response = await RequestsUtils.sendRequest({methodName: 'GET', url})
      this.gitLog = response?.data
      this.loadingGitlog = false
    },

    async restoreGitVersion(gitVersion: Commit) {
      const namespace = this.selectedNamespace
      const selectedKey = this.selectedKey
      const versionId = gitVersion.version
      const urlTrail = `${namespace}/v/${versionId}/`
      await RequestsUtils.sendRequest({
        methodName: 'PUT',
        url: `db/${urlTrail}revert/`,
        successMessage: `Namespace [${namespace}] restored to version [${versionId}]!`,
        failureMessage: `Failed restoring namespace [${namespace}] to version [${versionId}]!`,
      })
      await this.loadNamespace(namespace)
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
