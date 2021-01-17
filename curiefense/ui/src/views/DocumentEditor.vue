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
                    <select v-model="selectedBranch" @change="switchBranch()" class="branch-selection">
                      <option v-for="name in branchNames" :key="name" :value="name">{{ name }}</option>
                    </select>
                  </div>
                </div>
                <div class="control">
                  <div class="select is-small">
                    <select v-model="selectedDocType" @change="switchDocType()" class="doc-type-selection">
                      <option v-for="(component, propertyName) in componentsMap" :key="propertyName"
                              :value="propertyName">{{ component.title }}
                      </option>
                    </select>
                  </div>
                </div>
                <div class="control">
                  <span class="icon is-small is-vcentered">
                    <svg :width="24"
                         :height="24"
                         :viewBox="'0 0 24 24'">
                      <path :d="mdiSourceBranchPath"/>
                    </svg>
                  </span>
                  <span class="is-size-7 git-branches">{{ branches }} branches</span>
                </div>
                <div class="control">
                  <span class="icon is-small is-vcentered">
                    <svg :width="24"
                         :height="24"
                         :viewBox="'0 0 24 24'">
                      <path :d="mdiSourceCommitPath"/>
                    </svg>
                  </span>
                  <span class="is-size-7 git-commits">{{ commits }} commits</span>
                </div>
              </div>
            </div>

            <div class="column">
              <div class="field is-grouped is-pulled-right">
                <div class="control">
                  <div class="select is-small">
                    <select v-model="selectedDocID" @change="switchDocID()" class="doc-selection">
                      <option v-for="pair in docIdNames" :key="pair[0]" :value="pair[0]">{{ pair[1] }}</option>
                    </select>
                  </div>
                </div>

                <p class="control"
                   v-if="selectedDocType !== 'wafrules'">
                  <button class="button is-small fork-document-button"
                          :class="{'is-loading': isForkLoading}"
                          @click="forkDoc"
                          title="Duplicate Document"
                          :disabled="!selectedDoc">
                    <span class="icon is-small">
                      <i class="fas fa-clone"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <a class="button is-small download-doc-button"
                     @click="downloadDoc"
                     title="Download Document">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>

                  </a>
                </p>

                <p class="control"
                   v-if="selectedDocType !== 'wafrules'">
                  <button class="button is-small new-document-button"
                          :class="{'is-loading': isNewLoading}"
                          @click="addNewDoc()"
                          title="Add New Document">
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
                          title="Delete Document"
                          :disabled="selectedDoc && (selectedDoc.id === '__default__' || isDocReferenced || docs.length <= 1)">
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
            :is="currentEditorComponent.component"
            :selectedBranch.sync="selectedBranch"
            :selectedDoc.sync="selectedDoc"
            :docs.sync="docs"
            :apiPath="documentAPIPath"
            @switch-doc-type="switchDocType"
            @update="selectedDoc = $event"
            ref="currentComponent">
        </component>
        <hr/>
        <git-history v-if="selectedDocID"
                     :gitLog.sync="gitLog"
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
            <!--display correct message by priority (Branch -> Document type -> Document)-->
            <span v-if="!branchNames.includes(selectedBranch)">
              Missing branch. To be redirected to Version Control page where you will be able to create a new one, click
              <a title="Add New"
                 @click="referToVersionControl()">
                here
              </a>
            </span>
            <span v-else-if="!Object.keys(componentsMap).includes(selectedDocType)">
              Missing document type. Please select one from the dropdown above
            </span>
            <span v-else-if="!docIdNames.find((docIdName) => docIdName.includes(selectedDoc))">
              Missing document. To create a new one, click
              <a title="Add New"
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

<script>

import DatasetsUtils from '@/assets/DatasetsUtils.js'
import Utils from '@/assets/Utils.js'
import ACLEditor from '@/doc-editors/ACLEditor.vue'
import WAFEditor from '@/doc-editors/WAFEditor.vue'
import WAFSigsEditor from '@/doc-editors/WAFSigsEditor.vue'
import URLMapsEditor from '@/doc-editors/URLMapsEditor.vue'
import RateLimitsEditor from '@/doc-editors/RateLimitsEditor.vue'
import ProfilingListEditor from '@/doc-editors/ProfilingListEditor.vue'
import FlowControlEditor from '@/doc-editors/FlowControlEditor'
import GitHistory from '@/components/GitHistory.vue'
import RequestsUtils from '@/assets/RequestsUtils'
import {mdiSourceBranch, mdiSourceCommit} from '@mdi/js'

export default {

  name: 'DocumentEditor',
  props: {},
  components: {
    GitHistory
  },
  watch: {
    $route: {
      handler: async function() {
        this.setLoadingDocStatus(true)
        await this.setSelectedDataFromRouteParams()
        this.setLoadingDocStatus(false)
      }
    }
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

      // To prevent deletion of docs referenced by URLmaps
      referencedIDsACL: [],
      referencedIDsWAF: [],
      referencedIDsLimits: [],

      selectedBranch: null,
      selectedDocType: null,

      docs: [],
      docIdNames: [],
      selectedDocID: null,

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

      apiRoot: DatasetsUtils.ConfAPIRoot,
      apiVersion: DatasetsUtils.ConfAPIVersion,
    }
  },
  computed: {

    documentAPIPath() {
      return `${this.apiRoot}/${this.apiVersion}/configs/${this.selectedBranch}/d/${this.selectedDocType}/e/${this.selectedDocID}/`
    },

    gitAPIPath() {
      return `${this.apiRoot}/${this.apiVersion}/configs/${this.selectedBranch}/d/${this.selectedDocType}/e/${this.selectedDocID}/v/`
    },

    branchNames() {
      return this.ld.sortBy(this.ld.map(this.configs, 'id'))
    },

    currentEditorComponent() {
      if (this.selectedDocType) {
        return this.componentsMap[this.selectedDocType]
      } else {
        return Object.values(this.componentsMap)[0]
      }
    },

    selectedDoc: {
      get() {
        return this.docs[this.selectedDocIndex]
      },
      set(newDoc) {
        this.$set(this.docs, this.selectedDocIndex, newDoc)
      }
    },

    selectedDocIndex() {
      if (this.selectedDocID) {
        return this.ld.findIndex(this.docs, (doc) => {
          return doc.id === this.selectedDocID
        })
      }
      return 0
    },

    isDocReferenced() {
      if (this.selectedDocType === 'aclpolicies') {
        return this.referencedIDsACL.includes(this.selectedDocID)
      }
      if (this.selectedDocType === 'wafpolicies') {
        return this.referencedIDsWAF.includes(this.selectedDocID)
      }
      if (this.selectedDocType === 'limits') {
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
      this.selectedBranch = this.$route.params.branch || this.branchNames[0]
      const prevDocType = this.selectedDocType
      console.log(this.selectedDocType)
      this.selectedDocType = this.$route.params.doc_type || Object.keys(this.componentsMap)[0]
      console.log(this.selectedDocType)
      if (!prevDocType || prevDocType !== this.selectedDocType) {
        await this.loadDocs(this.selectedDocType)
      }
      this.selectedDocID = this.$route.params.doc_id || this.docIdNames[0][0]
      this.addMissingDefaultsToDoc()
      this.goToRoute()
    },

    resetGitLog() {
      this.gitLog = []
    },

    newDoc() {
      let factory = DatasetsUtils.NewDocEntryFactory[this.selectedDocType]
      return factory && factory()
    },

    async loadConfigs(counter_only) {
      // store configs
      let configs
      try {
        const response = await RequestsUtils.sendRequest('GET', 'configs/')
        configs = response.data
      } catch (err) {
        console.log('Error while attempting to get configs')
        console.log(err)
      }
      if (!counter_only) {
        console.log('loaded configs: ', configs)
        this.configs = configs
      }
      // counters
      this.commits = this.ld.sum(this.ld.map(this.ld.map(configs, 'logs'), (logs) => {
        return this.ld.size(logs)
      }))
      this.branches = this.ld.size(configs)
      console.log('config counters', this.branches, this.commits)
    },

    async initDocTypes() {
      let doctype = this.selectedDocType = Object.keys(this.componentsMap)[0]
      await this.loadDocs(doctype)
    },

    updateDocIdNames() {
      this.docIdNames = this.ld.sortBy(this.ld.map(this.docs, (doc) => {
            return [doc.id, doc.name]
          }),
          (entry) => {
            return entry[1]
          })
    },

    async loadDocs(doctype) {
      let branch = this.selectedBranch
      try {
        const response = await RequestsUtils.sendRequest('GET', `configs/${branch}/d/${doctype}/`)
        this.docs = response.data
      } catch (err) {
        console.log('Error while attempting to load documents')
        console.log(err)
        this.docs = []
      }
      this.updateDocIdNames()
      if (this.docIdNames && this.docIdNames.length && this.docIdNames[0].length) {
        this.selectedDocID = this.docIdNames[0][0]
        this.addMissingDefaultsToDoc()
      }
      this.loadGitLog()
    },

    loadGitLog(interaction) {
      let config = this.selectedBranch,
          document_ = this.selectedDocType,
          entry = this.selectedDocID,
          url_trail = `configs/${config}/d/${document_}/e/${entry}/v/`

      if (config && document_ && entry) {
        RequestsUtils.sendRequest('GET', url_trail).then((response) => {
          this.gitLog = response.data
          if (interaction) {
            this.loadConfigs(true)
          }
        })
      }
    },

    async switchBranch(branch) {
      this.setLoadingDocStatus(true)
      if (branch) {
        this.selectedBranch = branch
      }
      this.resetGitLog()
      await this.initDocTypes()
      this.loadReferencedDocsIDs()
      this.goToRoute()
      this.setLoadingDocStatus(false)
    },

    async switchDocType(docType) {
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

    async switchDocID(docID) {
      this.setLoadingDocStatus(true)
      if (docID) {
        this.selectedDocID = docID
        this.addMissingDefaultsToDoc()
      }
      this.loadGitLog()
      this.goToRoute()
      this.setLoadingDocStatus(false)
    },

    downloadDoc() {
      Utils.downloadFile(this.selectedDocType, 'json', this.docs)
    },

    async forkDoc() {
      this.setLoadingDocStatus(true)
      this.isForkLoading = true
      let docToAdd = this.ld.cloneDeep(this.selectedDoc)
      docToAdd.name = 'copy of ' + docToAdd.name
      docToAdd.id = DatasetsUtils.UUID2()
      await this.addNewDoc(docToAdd)
      this.isForkLoading = false
      this.setLoadingDocStatus(false)
    },

    async addNewDoc(docToAdd) {
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

    async saveChanges(methodName) {
      this.isSaveLoading = true
      if (!methodName) {
        methodName = 'PUT'
      }
      let url_trail = `configs/${this.selectedBranch}/d/${this.selectedDocType}/e/`
      if (methodName !== 'POST')
        url_trail += `${this.selectedDocID}/`
      const doc = this.selectedDoc

      await RequestsUtils.sendRequest(methodName, url_trail, doc, null, 'Changes saved!', 'Failed while saving changes!')
          .then(() => {
            this.updateDocIdNames()
            this.loadGitLog(true)
            // If the saved doc was a urlmap, refresh the referenced IDs lists
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
      await RequestsUtils.sendRequest('DELETE', `configs/${this.selectedBranch}/d/${this.selectedDocType}/e/${this.selectedDocID}/`, null, null, 'Document deleted!', 'Failed while deleting document!')
          .then(() => {
            this.updateDocIdNames()
            this.loadGitLog(true)
          })
      this.selectedDocID = this.docs[0].id
      this.addMissingDefaultsToDoc()
      this.resetGitLog()
      this.goToRoute()
      this.isDeleteLoading = false
      this.setLoadingDocStatus(false)
    },

    async loadReferencedDocsIDs() {
      const response = await RequestsUtils.sendRequest('GET', `configs/${this.selectedBranch}/d/urlmaps/`)
      const docs = response.data
      const referencedACL = []
      const referencedWAF = []
      const referencedLimit = []
      this.ld.forEach(docs, (doc) => {
        this.ld.forEach(doc.map, (mapEntry) => {
          referencedACL.push(mapEntry['acl_profile'])
          referencedWAF.push(mapEntry['waf_profile'])
          referencedLimit.push(mapEntry['limit_ids'])
        })
      })
      this.referencedIDsACL = this.ld.uniq(referencedACL)
      this.referencedIDsWAF = this.ld.uniq(referencedWAF)
      this.referencedIDsLimits = this.ld.uniq(this.ld.flatten(referencedLimit))
    },

    async restoreGitVersion(gitVersion) {
      const branch = this.selectedBranch
      const doctype = this.selectedDocType
      const docTitle = this.componentsMap[doctype].title
      const version_id = gitVersion.version
      const url_trail = `configs/${branch}/d/${doctype}/v/${version_id}/`

      await RequestsUtils.sendRequest('PUT', `${url_trail}revert/`, null, null, `Document [${docTitle}] restored to version [${version_id}]!`, `Failed restoring document [${docTitle}] to version [${version_id}]!`)
      const response = await RequestsUtils.sendRequest('GET', url_trail)
      this.docs = response.data
      this.updateDocIdNames()
      this.loadGitLog()
    },

    addMissingDefaultsToDoc() {
      if (!this.selectedDoc) {
        return
      }
      this.selectedDoc = {...this.newDoc(),...this.selectedDoc}
      return this.selectedDoc
    },

    referToVersionControl() {
      this.$router.push('/versioncontrol')
    },

    // Collect every request to display a loading indicator, the loading indicator will be displayed as long as at least one request is still active (counter > 0)
    setLoadingDocStatus(isLoading) {
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
  }

}

</script>
<style scoped>
.no-data-wrapper {
  /* Magic number! The page looks empty without content */
  min-height: 50vh;
  /* Magic number! Delayed the display of loading indicator as to not display it in short loads */
  animation: delayedDisplay 300ms;
}

@keyframes delayedDisplay {
  0% {
    opacity: 0
  }
  50% {
    opacity: 0
  }
  51% {
    opacity: 1
  }
  100% {
    opacity: 1
  }
}

</style>
