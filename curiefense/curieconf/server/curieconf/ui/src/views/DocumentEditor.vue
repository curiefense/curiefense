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
                    <select v-model="selectedBranch" @change="switchBranch">
                      <option v-for="name in branchNames" :key="name" :value="name">{{ name }}</option>
                    </select>
                  </div>
                </div>
                <div class="control">
                  <div class="select is-small">
                    <select v-model="selectedDocType" @change="switchDocType()">
                      <option v-for="(component, propertyName) in componentsMap" :key="propertyName" :value="propertyName">{{ component.title }}</option>
                    </select>
                  </div>
                </div>
                <div class="control">
                  <span class="icon is-small">
                    <i class="mdi mdi-dark mdi-source-branch"></i>
                  </span>
                  <span class="is-size-7  ">{{ branches }} branches</span>
                </div>
                <div class="control">
                  <span class="icon is-small">
                    <i class="mdi mdi-dark mdi-source-commit"></i>
                  </span>
                  <span class="is-size-7  ">{{ commits }} commits</span>
                </div>
              </div>
            </div>

            <div class="column">
              <div class="field is-grouped is-pulled-right">
                <div class="control">
                  <div class="select is-small">
                    <select v-model="selectedDocID" @change="loadGitLog()">
                      <option v-for="pair in docIdNames" :key="pair[0]" :value="pair[0]">{{ pair[1] }}</option>
                    </select>
                  </div>
                </div>

                <p class="control"
                   v-if="selectedDocType !== 'wafsigs'">
                  <button class="button is-small"
                          @click="forkDoc"
                          title="Duplicate Document"
                          :disabled="!selectedDoc.name">
                    <span class="icon is-small">
                      <i class="fas fa-clone"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <a class="button is-small"
                     @click="downloadDoc"
                     title="Download Document x">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>

                  </a>
                </p>

                <p class="control"
                   v-if="selectedDocType !== 'wafsigs'">
                  <button class="button is-small"
                          @click="addNewDoc()"
                          title="Add New Document">
                    <span class="icon is-small">
                      <i class="fas fa-plus"></i>
                    </span>
                  </button>
                </p>

                <p class="control"
                   v-if="selectedDocType !== 'wafsigs'">
                  <button class="button is-small"
                          @click="saveChanges()"
                          title="Save changes">
                    <span class="icon is-small">
                      <i class="fas fa-save"></i>
                    </span>
                  </button>
                </p>

                <p class="control"
                   v-if="selectedDocType !== 'wafsigs'">
                  <button class="button is-small has-text-danger"
                          @click="deleteDoc"
                          title="Delete Document"
                          :disabled="selectedDoc.id === '__default__' || isDocReferenced || docs.length <= 1">
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
        <hr />
        <component
          :is="currentEditorComponent.component"
          :selectedDoc.sync="selectedDoc"
          :docs.sync="docs"
          :additioanl_props="currentEditorComponent.props"
          :apiPath="documentAPIPath"
          @switchDocType="switchDocType"
          @updateDoc="saveChanges"
          ref="currentComponent"
          ></component>
        <hr />
        <GitHistory v-if="selectedDocID"
          :gitLog.sync="gitLog"
          :gitlogLoading.sync="gitlogLoading"
          :apiPath.sync="gitAPIPath"
          @restoreVersion="restoreGitVersion"></GitHistory>
      </div>
    </div>
  </div>
</template>

<script>

import axios from 'axios'
import DatasetsUtils from '@/assets/DatasetsUtils.js'
import * as bulmaToast from 'bulma-toast'

import ACLEditor from '@/doc-editors/ACLEditor.vue'
import WAFEditor from '@/doc-editors/WAFEditor.vue'
import WAFSigsEditor from '@/doc-editors/WAFSigsEditor.vue'
import URLMapsEditor from '@/doc-editors/URLMapsEditor.vue'
import RateLimitsEditor from '@/doc-editors/RateLimitsEditor.vue'
import ProfilingListEditor from '@/doc-editors/ProfilingListEditor.vue'
import GitHistory from '@/components/GitHistory.vue'

export default {

  name: 'DocumentEditor',
  props: {},
  components: {
    GitHistory
  },
  data() {
    return {
      configs: [],

      // for URLMap drop downs
      wafProfileNames: [],
      aclProfileNames: [],
      limitRuleNames: [],

      // To prevent deletion of docs referenced by URLmaps
      referencedIDsACL: [],
      referencedIDsWAF: [],
      referencedIDsLimits: [],

      // ...
      selectedBranch: null,
      branchDocTypes: null,
      selectedDocType: null,

      docs: [],
      docIdNames: [],
      selectedDocID: null,

      gitlogLoading: false,
      gitLog: [],
      commits: 0,
      branches:0,

      componentsMap: {
        "aclprofiles": { component: ACLEditor, props: [], title: "ACL Profiles" },
        "profilinglists": { component: ProfilingListEditor, title: "Profiling Lists" },
        "limits": { component: RateLimitsEditor, title: "Rate Limits" },
        "urlmaps": { component: URLMapsEditor, props: [
            this.wafProfileNames,
            this.aclProfileNames,
            this.limitRuleNames
          ], title: "URL Maps" },
        "wafprofiles": { component: WAFEditor, props: [], title: "WAF Profiles" },
        "wafsigs": { component: WAFSigsEditor, props: [], title: "WAF Signatures" },
      },

      apiRoot: DatasetsUtils.ConfAPIRoot,
      apiVersion: DatasetsUtils.ConfAPIVersion,

    }
  },
  computed: {

    documentAPIPath() {
      return `${this.apiRoot}/${this.apiVersion}/configs/${this.selectedBranch }/d/${this.selectedDocType}/e/${this.selectedDocID}/`
    },

    gitAPIPath () {
      return `${this.apiRoot}/${this.apiVersion}/configs/${this.selectedBranch }/d/${this.selectedDocType}/e/${this.selectedDocID}/v/`
    },

    branchNames() {
      return this.ld.sortBy(this.ld.map(this.configs, "id"))
    },

    currentEditorComponent() {
      if (this.selectedDocType) {
        return this.componentsMap[this.selectedDocType]
      } else {
        return Object.values(this.componentsMap)[0]
      }
    },

    selectedDoc: {
      get () {
        return this.docs[this.selectedDocIndex] || {}
      },
      set (newDoc) {
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

    docNames() {
      return this.ld.map(this.docs, "name")
    },

    isDocReferenced() {
      if (this.selectedDocType === 'aclprofiles') {
        return this.referencedIDsACL.includes(this.selectedDocID)
      }
      if (this.selectedDocType === 'wafprofiles') {
        return this.referencedIDsWAF.includes(this.selectedDocID)
      }
      if (this.selectedDocType === 'limits') {
        return this.referencedIDsLimits.includes(this.selectedDocID)
      }
      return false
    }

  },

  methods: {

    resetGitLog() {
      this.gitLog = []
    },

    newDoc() {
      let factory = DatasetsUtils.NewDocEntryFactory[this.selectedDocType]
      return factory && factory()
    },

    async loadConfigs(counter_only) {
      // store configs
      const response = await this.ax()
      let configs = response.data
      if (!counter_only) {
        console.log("this.configs", configs)
        this.configs = configs
        // pick first branch name as selected
        this.selectedBranch = this.branchNames[0]
        // get branch doument types
        this.initDocTypes()
      }
      // counters
      this.commits = this.ld.sum(this.ld.map(this.ld.map(configs, "logs"), (logs)=> { return this.ld.size(logs)}))
      this.branches = this.ld.size(configs)
      console.log("config counters", this.branches, this.commits )
    },

    initDocTypes() {
      let doctype = this.selectedDocType = Object.keys(this.componentsMap)[0]
      this.loadDocs(doctype)
      this.wafacllimitProfileNames()
    },

    updateDocIdNames() {
      this.docIdNames = this.ld.sortBy(this.ld.map(this.docs, (doc) => { return [doc.id, doc.name]}),
          (entry) => { return entry[1] })
    },

    loadDocs(doctype) {
      let branch = this.selectedBranch
      this.ax(axios.get, `${branch}/d/${doctype}/`).then( (response) => {
        this.docs = response.data
        this.updateDocIdNames()
        if (this.docIdNames && this.docIdNames.length && this.docIdNames[0].length) {
          this.selectedDocID = this.docIdNames[0][0]
        }
        this.loadGitLog()
      })
    },

    loadGitLog(interaction) {
      this.gitlogLoading = true;

      let self = this,
          config = this.selectedBranch,
          document_ = this.selectedDocType,
          entry = this.selectedDocID,
          url_trail = `${config}/d/${document_}/e/${entry}/v/`;

      if (config && document_ && entry) {
        this.ax(axios.get, url_trail).then( (response) => {
          this.gitLog = response.data
          self.gitlogLoading = false;
          if (interaction) {
            this.loadConfigs(true)
          }
        })
      }

    },

    switchBranch() {
      this.resetGitLog()
      this.initDocTypes()
      this.loadReferencedDocsIDs()
    },

    switchDocType(docType) {
      if (!docType) {
        docType = this.selectedDocType
      } else {
        this.selectedDocType = docType
      }
      this.docs = []
      this.selectedDocID = null
      this.resetGitLog()
      this.loadDocs(docType)
    },

    downloadDoc() {
      let element = event.target;
      while (element.nodeName !== "A")
        element = element.parentNode

      let dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(this.docs));
      element.setAttribute("href", dataStr);
      element.setAttribute("download", this.selectedDocType + ".json");
    },

    forkDoc() {
      let new_doc  = this.ld.cloneDeep(this.selectedDoc)
      new_doc.name = "copy of " + new_doc.name
      new_doc.id = DatasetsUtils.UUID2()
      this.addNewDoc(new_doc)
    },

    addNewDoc(new_doc) {
      if (!new_doc){
        new_doc = this.newDoc()
      }
      this.resetGitLog()
      this.docs.unshift(new_doc)
      this.selectedDocID = new_doc.id
      this.saveChanges(axios.post)
    },

    saveChanges(axios_method, doc, doctype, entry_id) {
      if (!axios_method) {
        axios_method = axios.put
      }
      if (!doc) {
        doc = this.selectedDoc
      }
      this.preax(axios_method, doc, doctype, entry_id)
          .then(() => {
            this.updateDocIdNames()
            this.wafacllimitProfileNames()
            this.loadGitLog(true)
            this.successToast('Changes saved!')
            // If the saved doc was a urlmap, refresh the referenced IDs lists
            if (this.selectedDocType === 'urlmaps') {
              this.loadReferencedDocsIDs()
            }
          })
          .catch(() => {
            this.failToast('Failed while saving changes!')
          })
    },

    deleteDoc() {
      this.docs.splice(this.selectedDocIndex, 1)
      this.preax(axios.delete)
          .then(() => {
            this.updateDocIdNames()
            this.wafacllimitProfileNames()
            this.successToast('Document deleted!')
          })
          .catch(() => {
            this.failToast('Failed while deleting document!')
          })
      this.selectedDocID = this.docs[0].id
      this.resetGitLog()
    },

    wafacllimitProfileNames() {
      let branch = this.selectedBranch

      this.ax(axios.get, `${branch}/d/wafprofiles/`).then( (response) => {
        this.wafProfileNames = this.ld.sortBy(this.ld.map(response.data, (entity) => {
          return [entity.id, entity.name]
        }), (e) => { return e[1] })
        this.componentsMap['urlmaps'].props[0] = this.wafProfileNames
      })

      this.ax(axios.get, `${branch}/d/aclprofiles/`).then( (response) => {
        this.aclProfileNames = this.ld.sortBy(this.ld.map(response.data, (entity) => {
          return [entity.id, entity.name]
        }), (e) => { return e[1] })
        this.componentsMap['urlmaps'].props[1] = this.aclProfileNames
      })

      this.ax(axios.get, `${branch}/d/limits/`).then( (response) => {
        this.limitRuleNames = response.data
        this.componentsMap['urlmaps'].props[2] = this.limitRuleNames
      })
    },

    async loadReferencedDocsIDs() {
      const response = await this.ax(axios.get, `${this.selectedBranch}/d/urlmaps/`)
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
      const version_id = gitVersion.version
      const url_trail = `${branch}/d/${doctype}/v/${version_id}/`

      await this.ax(axios.put, `${url_trail}revert/`)
      const response = await this.ax(axios.get, url_trail)
      this.docs = response.data
      this.updateDocIdNames()
      this.loadGitLog()
    },

    preax(axios_method, data, doctype, entry_id) {
      const branch = this.selectedBranch
      if (!doctype) {
        doctype = this.selectedDocType
      }
      if (!entry_id) {
        entry_id = this.selectedDocID
      }
      let url_trail = `${branch}/d/${doctype}/e/`
      const entity_trail = `${entry_id}/`

      if (axios_method !== axios.post)
        url_trail += entity_trail

      return this.ax(axios_method, url_trail, data)
    },

    ax( axios_method, urlTail, data) {
      if (!axios_method) { axios_method = axios.get; }
      if (!urlTail) { urlTail = ""; }

      let apiroot = this.apiRoot,
          apiversion = this.apiVersion,
          apiurl = `${apiroot}/${apiversion}/configs/${urlTail}`;

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

  async created() {
    await this.loadConfigs()
    this.loadReferencedDocsIDs()
  }

}

</script>
