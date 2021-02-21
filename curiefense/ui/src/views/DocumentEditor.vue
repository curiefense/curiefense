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
                    <select v-model="selectedBranch"
                            title="Switch branch"
                            @change="switchBranch()"
                            class="branch-selection">
                      <option v-for="name in branchNames"
                              :key="name"
                              :value="name">
                        {{ name }}
                      </option>
                    </select>
                  </div>
                </div>
                <div class="control">
                  <div class="select is-small">
                    <select v-model="selectedDocType"
                            title="Switch document type"
                            @change="switchDocType()"
                            class="doc-type-selection">
                      <option v-for="(component, propertyName) in componentsMap"
                              :key="propertyName"
                              :value="propertyName">
                        {{ component.title }}
                      </option>
                    </select>
                  </div>
                </div>
                <p class="control">
                  <button class="button is-small download-doc-button"
                          :class="{'is-loading': isDownloadLoading}"
                          @click="downloadDoc"
                          title="Download document">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </button>
                </p>
                <div class="control">
                  <span class="icon is-small is-vcentered">
                    <svg :width="24"
                         :height="24"
                         :viewBox="'0 0 24 24'">
                      <path :d="mdiSourceBranchPath"/>
                    </svg>
                  </span>
                  <span class="is-size-7 git-branches">
                    {{ branches }} branch<span v-if="branches !== 1">es</span>
                  </span>
                </div>
                <div class="control">
                  <span class="icon is-small is-vcentered">
                    <svg :width="24"
                         :height="24"
                         :viewBox="'0 0 24 24'">
                      <path :d="mdiSourceCommitPath"/>
                    </svg>
                  </span>
                  <span class="is-size-7 git-commits">
                    {{ commits }} commit<span v-if="commits !== 1">s</span>
                  </span>
                </div>
              </div>
            </div>

            <div class="column">
              <div class="field is-grouped is-pulled-right">
                <div class="control">
                  <div class="select is-small">
                    <select v-model="selectedDocID"
                            title="Switch document ID"
                            @change="switchDocID()"
                            class="doc-selection">
                      <option v-for="pair in docIdNames"
                              :key="pair[0]"
                              :value="pair[0]">
                        {{ pair[1] }}
                      </option>
                    </select>
                  </div>
                </div>

                <p class="control"
                   v-if="selectedDocType !== 'wafrules'">
                  <button class="button is-small fork-document-button"
                          :class="{'is-loading': isForkLoading}"
                          @click="forkDoc"
                          title="Duplicate document"
                          :disabled="!selectedDoc">
                    <span class="icon is-small">
                      <i class="fas fa-clone"></i>
                    </span>
                  </button>
                </p>

                <p class="control"
                   v-if="selectedDocType !== 'wafrules'">
                  <button class="button is-small new-document-button"
                          :class="{'is-loading': isNewLoading}"
                          @click="addNewDoc()"
                          title="Add new document">
                    <span class="icon is-small">
                      <i class="fas fa-plus"></i>
                    </span>
                  </button>
                </p>

                <p class="control"
                   v-if="selectedDocType !== 'wafrules'">
                  <button class="button is-small save-document-button"
                          :class="{'is-loading': isSaveLoading}"
                          @click="saveChanges()"
                          title="Save changes">
                    <span class="icon is-small">
                      <i class="fas fa-save"></i>
                    </span>
                  </button>
                </p>

                <p class="control"
                   v-if="selectedDocType !== 'wafrules'">
                  <button class="button is-small has-text-danger delete-document-button"
                          :class="{'is-loading': isDeleteLoading}"
                          @click="deleteDoc"
                          title="Delete document"
                          :disabled="selectedDocNotDeletable">
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

      <div class="content document-editor-wrapper"
           v-if="!loadingDocCounter && selectedBranch && selectedDocType && selectedDoc">
        <component
            :is="componentsMap[selectedDocType].component"
            :selectedBranch.sync="selectedBranch"
            :selectedDoc.sync="selectedDoc"
            :docs.sync="docs"
            :apiPath="documentAPIPath"
            @switch-doc-type="switchDocType"
            ref="currentComponent">
        </component>
        <hr/>
        <git-history v-if="selectedDocID"
                     :gitLog="gitLog"
                     :apiPath="gitAPIPath"
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
            <!--display correct message by priority (Branch -> Document type -> Document)-->
            <span v-if="!branchNames.includes(selectedBranch)">
              Missing branch. To be redirected to Version Control page where you will be able to create a new one, click
              <a title="Add new"
                 @click="referToVersionControl()">
                here
              </a>
            </span>
            <span v-else-if="!Object.keys(componentsMap).includes(selectedDocType)">
              Missing document type. Please select one from the dropdown above
            </span>
            <span v-else-if="!docIdNames.find((docIdName) => docIdName.includes(selectedDoc))">
              Missing document. To create a new one, click
              <a title="Add new"
                 @click="addNewDoc()">
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
import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import RequestsUtils, {MethodNames} from '@/assets/RequestsUtils.ts'
import Utils from '@/assets/Utils.ts'
import ACLEditor from '@/doc-editors/ACLEditor.vue'
import WAFEditor from '@/doc-editors/WAFEditor.vue'
import WAFSigsEditor from '@/doc-editors/WAFSigsEditor.vue'
import URLMapsEditor from '@/doc-editors/URLMapsEditor.vue'
import RateLimitsEditor from '@/doc-editors/RateLimitsEditor.vue'
import ProfilingListEditor from '@/doc-editors/ProfilingListEditor.vue'
import FlowControlEditor from '@/doc-editors/FlowControlEditor.vue'
import GitHistory from '@/components/GitHistory.vue'
import {mdiSourceBranch, mdiSourceCommit} from '@mdi/js'
import Vue from 'vue'
import {BasicDocument, Commit, Document, DocumentType} from '@/types'
import axios, {AxiosResponse} from 'axios'

export default Vue.extend({

  name: 'DocumentEditor',
  props: {},
  components: {
    GitHistory,
  },
  watch: {
    $route: {
      handler: async function() {
        this.setLoadingDocStatus(true)
        await this.setSelectedDataFromRouteParams()
        this.setLoadingDocStatus(false)
      },
    },
  },
  data() {
    return {
      configs: [],
      mdiSourceBranchPath: mdiSourceBranch,
      mdiSourceCommitPath: mdiSourceCommit,

      // Loading indicators
      loadingDocCounter: 0,
      isForkLoading: false,
      isNewLoading: false,
      isSaveLoading: false,
      isDeleteLoading: false,

      // To prevent deletion of docs referenced by URL maps
      referencedIDsACL: [],
      referencedIDsWAF: [],
      referencedIDsLimits: [],

      selectedBranch: null,
      selectedDocType: null as DocumentType,

      docs: [],
      docIdNames: [],
      selectedDocID: null,
      cancelSource: axios.CancelToken.source(),
      isDownloadLoading: false,

      gitLog: [],
      commits: 0,
      branches: 0,

      componentsMap: {
        'aclpolicies': {component: ACLEditor, title: 'ACL Policies'},
        'flowcontrol': {component: FlowControlEditor, title: 'Flow Control'},
        'tagrules': {component: ProfilingListEditor, title: 'Tag Rules'},
        'ratelimits': {component: RateLimitsEditor, title: 'Rate Limits'},
        'urlmaps': {component: URLMapsEditor, title: 'URL Maps'},
        'wafpolicies': {component: WAFEditor, title: 'WAF Policies'},
        'wafrules': {component: WAFSigsEditor, title: 'WAF Rules'},
      },

      apiRoot: RequestsUtils.confAPIRoot,
      apiVersion: RequestsUtils.confAPIVersion,
    }
  },
  computed: {

    documentAPIPath(): string {
      const apiPrefix = `${this.apiRoot}/${this.apiVersion}`
      return `${apiPrefix}/configs/${this.selectedBranch}/d/${this.selectedDocType}/e/${this.selectedDocID}/`
    },

    gitAPIPath(): string {
      const apiPrefix = `${this.apiRoot}/${this.apiVersion}`
      return `${apiPrefix}/configs/${this.selectedBranch}/d/${this.selectedDocType}/e/${this.selectedDocID}/v/`
    },

    branchNames(): string[] {
      return _.sortBy(_.map(this.configs, 'id'))
    },

    selectedDoc: {
      get(): Document {
        return this.docs[this.selectedDocIndex]
      },
      set(newDoc): void {
        this.$set(this.docs, this.selectedDocIndex, newDoc)
      },
    },

    selectedDocNotDeletable(): boolean {
      return !this.selectedDoc ||
          (this.selectedDoc as BasicDocument).id === '__default__' ||
          this.isDocReferenced ||
          this.docs.length <= 1
    },

    selectedDocIndex(): number {
      if (this.selectedDocID) {
        return _.findIndex(this.docs, (doc) => {
          return doc.id === this.selectedDocID
        })
      }
      return 0
    },

    isDocReferenced(): boolean {
      if (this.selectedDocType === 'aclpolicies') {
        return this.referencedIDsACL.includes(this.selectedDocID)
      }
      if (this.selectedDocType === 'wafpolicies') {
        return this.referencedIDsWAF.includes(this.selectedDocID)
      }
      if (this.selectedDocType === 'ratelimits') {
        return this.referencedIDsLimits.includes(this.selectedDocID)
      }
      return false
    },

  },

  methods: {

    goToRoute() {
      const currentRoute = `/config/${this.selectedBranch}/${this.selectedDocType}/${this.selectedDocID}`
      if (this.$route.path !== currentRoute) {
        console.log('Switching document, new document path: ' + currentRoute)
        this.$router.push(currentRoute)
      }
    },

    async setSelectedDataFromRouteParams() {
      this.setLoadingDocStatus(true)
      this.selectedBranch = this.$route.params.branch || this.branchNames[0]
      const prevDocType = this.selectedDocType
      this.selectedDocType = (this.$route.params.doc_type || Object.keys(this.componentsMap)[0]) as DocumentType
      if (!prevDocType || prevDocType !== this.selectedDocType) {
        await this.loadDocs(this.selectedDocType)
      }
      this.selectedDocID = this.$route.params.doc_id || this.docIdNames[0][0]
      await this.loadSelectedDocData()
      this.addMissingDefaultsToDoc()
      this.setLoadingDocStatus(false)
      this.goToRoute()
    },

    resetGitLog() {
      this.gitLog = []
    },

    newDoc(): Document {
      const factory = DatasetsUtils.newDocEntryFactory[this.selectedDocType]
      return factory && factory()
    },

    async loadConfigs(counterOnly?: boolean) {
      // store configs
      let configs
      try {
        const response = await RequestsUtils.sendRequest('GET', 'configs/')
        configs = response.data
      } catch (err) {
        console.log('Error while attempting to get configs')
        console.log(err)
      }
      if (!counterOnly) {
        console.log('loaded configs: ', configs)
        this.configs = configs
      }
      // counters
      this.commits = _.sum(_.map(_.map(configs, 'logs'), (logs) => {
        return _.size(logs)
      }))
      this.branches = _.size(configs)
      console.log('config counters', this.branches, this.commits)
    },

    async initDocTypes() {
      const doctype = this.selectedDocType = Object.keys(this.componentsMap)[0] as DocumentType
      await this.loadDocs(doctype)
    },

    updateDocIdNames() {
      this.docIdNames = _.sortBy(_.map(this.docs, (doc) => [doc.id, doc.name]), (entry) => entry[1])
    },

    async loadSelectedDocData() {
      this.setLoadingDocStatus(true)
      // check if the selected doc only has id and name, if it does, attempt to load the rest of the document data
      if (this.selectedDoc && Object.keys(this.selectedDoc).length === 2) {
        this.selectedDoc = (await RequestsUtils.sendRequest('GET',
            `configs/${this.selectedBranch}/d/${this.selectedDocType}/e/${this.selectedDocID}/`)).data
      }
      this.setLoadingDocStatus(false)
    },

    async loadDocs(doctype: DocumentType) {
      this.isDownloadLoading = true
      const branch = this.selectedBranch
      try {
        const response = await RequestsUtils.sendRequest('GET',
            `configs/${branch}/d/${doctype}/`, {headers: {'x-fields': 'id, name'}})
        this.docs = response.data
        // After we load the basic data (id and name) we can async load the full data
        this.cancelSource.cancel(`Operation cancelled and restarted for a new document type ${doctype}`)
        RequestsUtils.sendRequest('GET',
            `configs/${branch}/d/${doctype}/`,
            null,
            {cancelToken: this.cancelSource.token}).then((response: AxiosResponse) => {
          this.docs = response.data
          this.isDownloadLoading = false
        })
      } catch (err) {
        console.log('Error while attempting to load documents')
        console.log(err)
        this.docs = []
      }
      this.updateDocIdNames()
      if (this.docIdNames && this.docIdNames.length && this.docIdNames[0].length) {
        this.selectedDocID = this.docIdNames[0][0]
        await this.loadSelectedDocData()
        this.addMissingDefaultsToDoc()
      }
      this.loadGitLog()
    },

    loadGitLog(interaction?: boolean) {
      const config = this.selectedBranch
      const document = this.selectedDocType
      const entry = this.selectedDocID
      const urlTrail = `configs/${config}/d/${document}/e/${entry}/v/`

      if (config && document && entry) {
        RequestsUtils.sendRequest('GET', urlTrail).then((response: AxiosResponse<Commit[]>) => {
          this.gitLog = response.data
          if (interaction) {
            this.loadConfigs(true)
          }
        })
      }
    },

    async switchBranch(branch?: string) {
      this.setLoadingDocStatus(true)
      if (branch) {
        this.selectedBranch = branch
      }
      this.resetGitLog()
      await this.initDocTypes()
      await this.loadReferencedDocsIDs()
      this.goToRoute()
      this.setLoadingDocStatus(false)
    },

    async switchDocType(docType?: DocumentType) {
      this.setLoadingDocStatus(true)
      if (!docType) {
        docType = this.selectedDocType
      } else {
        this.selectedDocType = docType
      }
      this.docs = []
      this.selectedDocID = null
      this.resetGitLog()
      await this.loadDocs(docType)
      this.goToRoute()
      this.setLoadingDocStatus(false)
    },

    async switchDocID(docID?: string) {
      this.setLoadingDocStatus(true)
      if (docID) {
        this.selectedDocID = docID
        await this.loadSelectedDocData()
        this.addMissingDefaultsToDoc()
      }
      this.loadGitLog()
      this.goToRoute()
      this.setLoadingDocStatus(false)
    },

    downloadDoc() {
      if (!this.isDownloadLoading) {
        Utils.downloadFile(this.selectedDocType, 'json', this.docs)
      }
    },

    async forkDoc() {
      this.setLoadingDocStatus(true)
      this.isForkLoading = true
      const docToAdd = _.cloneDeep(this.selectedDoc) as Document
      docToAdd.name = 'copy of ' + docToAdd.name
      docToAdd.id = DatasetsUtils.generateUUID2()
      await this.addNewDoc(docToAdd)
      this.isForkLoading = false
      this.setLoadingDocStatus(false)
    },

    async addNewDoc(docToAdd?: Document) {
      this.setLoadingDocStatus(true)
      this.isNewLoading = true
      if (!docToAdd) {
        docToAdd = this.newDoc()
      }
      this.resetGitLog()
      this.docs.unshift(docToAdd)
      this.selectedDocID = docToAdd.id
      await this.saveChanges('POST')
      this.goToRoute()
      this.isNewLoading = false
      this.setLoadingDocStatus(false)
    },

    async saveChanges(methodName?: MethodNames) {
      this.isSaveLoading = true
      if (!methodName) {
        methodName = 'PUT'
      }
      let urlTrail = `configs/${this.selectedBranch}/d/${this.selectedDocType}/e/`
      if (methodName !== 'POST') {
        urlTrail += `${this.selectedDocID}/`
      }
      const doc = this.selectedDoc

      await RequestsUtils.sendRequest(methodName, urlTrail, doc, null,
          'Changes saved!', 'Failed while saving changes!',
      ).then(() => {
        this.updateDocIdNames()
        this.loadGitLog(true)
        // If the saved doc was a url map, refresh the referenced IDs lists
        if (this.selectedDocType === 'urlmaps') {
          this.loadReferencedDocsIDs()
        }
      })
      this.isSaveLoading = false
    },

    async deleteDoc() {
      this.setLoadingDocStatus(true)
      this.isDeleteLoading = true
      this.docs.splice(this.selectedDocIndex, 1)
      await RequestsUtils.sendRequest('DELETE',
          `configs/${this.selectedBranch}/d/${this.selectedDocType}/e/${this.selectedDocID}/`,
          null, null, 'Document deleted!', 'Failed while deleting document!',
      ).then(() => {
        this.updateDocIdNames()
        this.loadGitLog(true)
      })
      this.selectedDocID = this.docs[0].id
      await this.loadSelectedDocData()
      this.addMissingDefaultsToDoc()
      this.resetGitLog()
      this.goToRoute()
      this.isDeleteLoading = false
      this.setLoadingDocStatus(false)
    },

    async loadReferencedDocsIDs() {
      const response = await RequestsUtils.sendRequest('GET', `configs/${this.selectedBranch}/d/urlmaps/`)
      const docs = response.data
      const referencedACL: string[] = []
      const referencedWAF: string[] = []
      const referencedLimit: string[] = []
      _.forEach(docs, (doc) => {
        _.forEach(doc.map, (mapEntry) => {
          referencedACL.push(mapEntry['acl_profile'])
          referencedWAF.push(mapEntry['waf_profile'])
          referencedLimit.push(mapEntry['limit_ids'])
        })
      })
      this.referencedIDsACL = _.uniq(referencedACL)
      this.referencedIDsWAF = _.uniq(referencedWAF)
      this.referencedIDsLimits = _.uniq(_.flatten(referencedLimit))
    },

    async restoreGitVersion(gitVersion: Commit) {
      const branch = this.selectedBranch
      const doctype: DocumentType = this.selectedDocType
      const docTitle = this.componentsMap[doctype].title
      const versionId = gitVersion.version
      const urlTrail = `configs/${branch}/d/${doctype}/v/${versionId}/`

      await RequestsUtils.sendRequest('PUT',
          `${urlTrail}revert/`,
          null,
          null,
          `Document [${docTitle}] restored to version [${versionId}]!`,
          `Failed restoring document [${docTitle}] to version [${versionId}]!`)
      await this.loadDocs(this.selectedDocType)
    },

    addMissingDefaultsToDoc() {
      if (!this.selectedDoc) {
        return
      }
      this.selectedDoc = {...this.newDoc(), ...this.selectedDoc as {}}
    },

    referToVersionControl() {
      this.$router.push('/versioncontrol')
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
  },

  async created() {
    this.setLoadingDocStatus(true)
    await this.loadConfigs()
    this.setSelectedDataFromRouteParams()
    this.loadReferencedDocsIDs()
    this.setLoadingDocStatus(false)
  },

})
</script>
<style scoped lang="scss">

.no-data-wrapper {
  /* Magic number! Delayed the display of loading indicator as to not display it in short loads */
  animation: delayedDisplay 300ms;
  /* Magic number! The page looks empty without content */
  min-height: 50vh;
}

@keyframes delayedDisplay {
  0% {
    opacity: 0;
  }

  50% {
    opacity: 0;
  }

  51% {
    opacity: 1;
  }

  100% {
    opacity: 1;
  }
}

</style>
