<template>
  <div class="card">
    <div class="card-content">
      <div class="media">
        <div class="media-content">
          <div class="columns">
            <div class="column">
              <div class="field is-grouped">
                <div class="control" v-if="branchNames.length">
                  <div class="select is-small">
                    <select v-model="selectedBranch"
                            title="Switch branch"
                            @change="switchBranch"
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
                    <select v-model="selectedSearchType"
                            title="Switch search type"
                            class="search-type-selection">
                      <option v-for="(searchType, propertyName) in searchTypeMap"
                              :key="propertyName"
                              :value="propertyName">
                        {{ searchType.title }}
                      </option>
                    </select>
                  </div>
                </div>
                <div class="control has-icons-left">
                  <input class="input is-small search-input"
                         title="Search"
                         placeholder="Search"
                         v-model="searchValue"/>
                  <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-search"></i></span>
                </div>
              </div>
            </div>

            <div class="column">

            </div>
          </div>
        </div>
      </div>

      <hr/>

      <div class="content document-editor-wrapper"
           v-if="filteredDocs && filteredDocs.length > 0">
        <table class="table">
          <thead>
          <tr>
            <th class="is-size-7 width-100px">Type</th>
            <th class="is-size-7 width-150px">ID</th>
            <th class="is-size-7 width-200px">Name</th>
            <th class="is-size-7 width-200px">Description</th>
            <th class="is-size-7 width-200px">Tags</th>
            <th class="is-size-7 width-150px">Connections</th>
            <th class="is-size-7 width-50px"></th>
          </tr>
          </thead>
          <tbody>
          <tr v-for="(doc, index) in filteredDocs"
              :key="index"
              class="result-row"
              @mouseleave="mouseLeave()"
              @mouseover="mouseOver(index)">
            <td class="is-size-7 py-3 ellipsis width-100px doc-type-cell"
                :title="componentsMap[doc.docType].title"
                v-html="highlightSearchValue(componentsMap[doc.docType].title)">
            </td>
            <td class="is-size-7 py-3 ellipsis width-150px doc-id-cell"
                :title="doc.id"
                v-html="highlightSearchValue(doc.id)">
            </td>
            <td class="is-size-7 py-3 ellipsis width-200px doc-name-cell"
                :title="doc.name"
                v-html="highlightSearchValue(doc.name)">
            </td>
            <td class="is-size-7 py-3 width-200px doc-description-cell"
                :title="doc.notes || doc.description">
              <div class="vertical-scroll scrollbox-shadowed"
                   v-html="highlightSearchValue(doc.notes || doc.description)">
              </div>
            </td>
            <td class="is-size-7 py-3 width-200px doc-tags-cell"
                :title="doc.tags">
              <div class="vertical-scroll scrollbox-shadowed"
                   v-html="highlightSearchValue(doc.tags)">
              </div>
            </td>
            <td class="is-size-7 py-3 width-150px doc-connections-cell">
              <div class="vertical-scroll scrollbox-shadowed"
                   v-html="connectionsDisplayText(doc)">
              </div>
            </td>
            <td class="is-size-7 width-50px">
              <p class="control has-text-centered" v-if="rowOverIndex === index">
                <button class="button is-small go-to-link-button"
                   @click="goToDocument(doc)"
                   title="Go to document">
                    <span class="icon is-small">
                      <i class="fas fa-link"></i>
                    </span>
                </button>
              </p>
            </td>
          </tr>
          </tbody>
        </table>
      </div>

      <div class="content no-data-wrapper"
           v-else>
        <div v-if="loadingCounter > 0">
          <button class="button is-outlined is-text is-small is-loading document-loading">
            Loading
          </button>
        </div>
        <div v-else
             class="no-data-message">
          No data found!
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import _ from 'lodash'
import ACLEditor from '@/doc-editors/ACLEditor.vue'
import WAFEditor from '@/doc-editors/WAFEditor.vue'
import URLMapsEditor from '@/doc-editors/URLMapsEditor.vue'
import RateLimitsEditor from '@/doc-editors/RateLimitsEditor.vue'
import ProfilingListEditor from '@/doc-editors/ProfilingListEditor.vue'
import FlowControlEditor from '@/doc-editors/FlowControlEditor.vue'
import RequestsUtils from '@/assets/RequestsUtils.ts'
import Vue, {VueConstructor} from 'vue'
import {Document, DocumentType, URLMapEntryMatch} from '@/types'
import DatasetsUtils from '@/assets/DatasetsUtils'
import Utils from '@/assets/Utils'

type SearchDocument = Document & {
  docType: DocumentType
  notes: string
  description: string
  tags: string
  connections: string[]
  connectedACL: string[]
  connectedWAF: string[]
  connectedRateLimits: string[]
  connectedURLMaps: string[]
  map: URLMapEntryMatch[]
}

type ReferencesMap = {
  [key: string]: string[]
}

export default Vue.extend({

  name: 'DocumentSearch',
  props: {},
  components: {},
  data() {
    const titles = DatasetsUtils.titles
    // Order is important
    // We load [urlmaps] before [aclpolicies, wafpolicies, ratelimits] so we can pull all references correctly
    const componentsMap: {
      [key in DocumentType]?: {
        component: VueConstructor
        title: typeof titles[key]
        fields: string
      }
    } = {
      'urlmaps': {
        component: URLMapsEditor,
        title: titles['urlmaps'],
        fields: 'id, name, map',
      },
      'aclpolicies': {
        component: ACLEditor,
        title: titles['aclpolicies'],
        fields: 'id, name, allow, allow_bot, deny_bot, bypass, deny, force_deny',
      },
      'flowcontrol': {
        component: FlowControlEditor,
        title: titles['flowcontrol'],
        fields: 'id, name, notes, include, exclude',
      },
      'tagrules': {
        component: ProfilingListEditor,
        title: titles['tagrules'],
        fields: 'id, name, notes, tags',
      },
      'ratelimits': {
        component: RateLimitsEditor,
        title: titles['ratelimits'],
        fields: 'id, name, description',
      },
      'wafpolicies': {
        component: WAFEditor,
        title: titles['wafpolicies'],
        fields: 'id, name',
      },
    }

    const searchTypeMap: {
      [key: string]: {
        title: string
        filter: Function
      }
    } = {
      all: {
        title: 'All',
        filter: (doc: SearchDocument): boolean => {
          const isFoundDocType = searchTypeMap['doctype'].filter(doc)
          const isFoundID = searchTypeMap['id'].filter(doc)
          const isFoundName = searchTypeMap['name'].filter(doc)
          const isFoundDescription = searchTypeMap['description'].filter(doc)
          const isFoundTags = searchTypeMap['tags'].filter(doc)
          const isFoundConnections = searchTypeMap['connections'].filter(doc)
          return isFoundDocType || isFoundID || isFoundName || isFoundDescription || isFoundTags || isFoundConnections
        },
      },
      doctype: {
        title: 'Document Type',
        filter: (doc: SearchDocument): boolean => {
          return (this as any).searchValueRegex.test(componentsMap[doc.docType].title)
        },
      },
      id: {
        title: 'ID',
        filter: (doc: SearchDocument): boolean => {
          return (this as any).searchValueRegex.test(doc.id)
        },
      },
      name: {
        title: 'Name',
        filter: (doc: SearchDocument): boolean => {
          return (this as any).searchValueRegex.test(doc.name)
        },
      },
      description: {
        title: 'Description',
        filter: (doc: SearchDocument): boolean => {
          return (this as any).searchValueRegex.test(doc.notes || doc.description)
        },
      },
      tags: {
        title: 'Tags',
        filter: (doc: SearchDocument): boolean => {
          return (this as any).searchValueRegex.test(doc.tags)
        },
      },
      connections: {
        title: 'Connections',
        filter: (doc: SearchDocument): boolean => {
          return (this as any).searchValueRegex.test(doc.connections)
        },
      },
    }
    return {
      configs: [],
      docs: [],
      selectedBranch: null,
      // Starting with the first search type in searchTypeMap
      selectedSearchType: 'all',
      searchValue: '',
      rowOverIndex: null,
      loadingCounter: 0,

      searchTypeMap: searchTypeMap,

      componentsMap: componentsMap,

      // Referenced IDs of [aclpolicies, wafpolicies, ratelimits] in [urlmaps]
      referencedACL: {} as ReferencesMap,
      referencedWAF: {} as ReferencesMap,
      referencedLimit: {} as ReferencesMap,
    }
  },
  computed: {

    branchNames(): string[] {
      return _.sortBy(_.map(this.configs, 'id'))
    },

    filteredDocs(): SearchDocument[] {
      return this.docs.filter((doc) => {
        return this.searchTypeMap[this.selectedSearchType].filter(doc, this.searchValue)
      })
    },

    searchValueRegex(): RegExp {
      return new RegExp(this.searchValue, 'gi')
    },

  },

  methods: {

    async loadConfigs() {
      // store configs
      let configs
      try {
        const response = await RequestsUtils.sendRequest({methodName: 'GET', url: 'configs/'})
        configs = response.data
      } catch (err) {
        console.log('Error while attempting to get configs')
        console.log(err)
      }
      console.log('loaded configs: ', configs)
      this.configs = configs
      // pick first branch name as selected
      this.selectedBranch = this.branchNames[0]
      await this.loadDocs()
    },

    async loadDocs() {
      const docTypes = Object.keys(this.componentsMap)
      for (let i = 0; i < docTypes.length; i++) {
        const doctype: DocumentType = docTypes[i] as DocumentType
        const branch = this.selectedBranch
        try {
          const response = await RequestsUtils.sendRequest({
            methodName: 'GET',
            url: `configs/${branch}/d/${doctype}/`,
            config: {headers: {'x-fields': this.componentsMap[doctype].fields}},
          })
          for (let j = 0; j < response.data.length; j++) {
            const doc = response.data[j]
            doc.docType = doctype
            // Build tags based on document type
            if (doctype === 'aclpolicies') {
              const forceDenyTags = doc.force_deny.filter(Boolean).join(', ').toLowerCase()
              const bypassTags = doc.bypass.filter(Boolean).join(', ').toLowerCase()
              const allowBotTags = doc.allow_bot.filter(Boolean).join(', ').toLowerCase()
              const denyBotTags = doc.deny_bot.filter(Boolean).join(', ').toLowerCase()
              const allowTags = doc.allow.filter(Boolean).join(', ').toLowerCase()
              const denyTags = doc.deny.filter(Boolean).join(', ').toLowerCase()
              doc.tags = [
                forceDenyTags,
                bypassTags,
                allowBotTags,
                denyBotTags,
                allowTags,
                denyTags,
              ].filter(Boolean).join(', ')
            }
            if (doctype === 'flowcontrol') {
              const includeTags = doc.include.filter(Boolean).join(', ').toLowerCase()
              const excludeTags = doc.exclude.filter(Boolean).join(', ').toLowerCase()
              doc.tags = [includeTags, excludeTags].filter(Boolean).join(', ')
            }
            if (doctype === 'tagrules') {
              doc.tags = doc.tags.filter(Boolean).join(', ').toLowerCase()
            }
            // Build connections based on document type
            if (doctype === 'urlmaps') {
              this.buildURLMapConnections(doc)
              this.saveWafAclLimitConnections(doc)
            }
            if (doctype === 'aclpolicies') {
              this.buildWafAclLimitConnections(doc, this.referencedACL)
            }
            if (doctype === 'wafpolicies') {
              this.buildWafAclLimitConnections(doc, this.referencedWAF)
            }
            if (doctype === 'ratelimits') {
              this.buildWafAclLimitConnections(doc, this.referencedLimit)
            }
            this.docs.push(doc)
          }
        } catch (err) {
          console.log(`Error while attempting to load document: ${doctype}`)
          console.log(err)
        }
      }
    },

    buildURLMapConnections(doc: SearchDocument) {
      const connectedACL: string[] = []
      const connectedWAF: string[] = []
      const connectedRateLimits: string[] = []
      for (let i = 0; i < doc.map.length; i++) {
        const map = doc.map[i]
        if (!connectedACL.includes(map.acl_profile)) {
          connectedACL.push(map.acl_profile)
        }
        if (!connectedWAF.includes(map.waf_profile)) {
          connectedWAF.push(map.waf_profile)
        }
        for (let j = 0; j < map.limit_ids.length; j++) {
          if (!connectedRateLimits.includes(map.limit_ids[j])) {
            connectedRateLimits.push(map.limit_ids[j])
          }
        }
      }
      doc.connectedACL = connectedACL
      doc.connectedWAF = connectedWAF
      doc.connectedRateLimits = connectedRateLimits
      doc.connections = [].concat(connectedACL, connectedWAF, connectedRateLimits)
    },

    buildWafAclLimitConnections(doc: SearchDocument, referencesMap: ReferencesMap) {
      if (!referencesMap[doc.id] || referencesMap[doc.id].length === 0) {
        return
      }
      doc.connectedURLMaps = referencesMap[doc.id]
      doc.connections = referencesMap[doc.id]
    },

    saveWafAclLimitConnections(doc: SearchDocument) {
      for (let i = 0; i < doc.map.length; i++) {
        const map = doc.map[i]
        // initialize array if needed
        if (!this.referencedACL[map.acl_profile] || this.referencedACL[map.acl_profile].length === 0) {
          this.referencedACL[map.acl_profile] = []
        }
        // add map id to referenced acl
        this.referencedACL[map.acl_profile].push(doc.id)
        // initialize array if needed
        if (!this.referencedWAF[map.waf_profile] || this.referencedWAF[map.waf_profile].length === 0) {
          this.referencedWAF[map.waf_profile] = []
        }
        // add map id to referenced waf
        this.referencedWAF[map.waf_profile].push(doc.id)
        for (let j = 0; j < map.limit_ids.length; j++) {
          // initialize array if needed
          if (!this.referencedLimit[map.limit_ids[j]] || this.referencedLimit[map.limit_ids[j]].length === 0) {
            this.referencedLimit[map.limit_ids[j]] = []
          }
          // add map id to referenced rate limit
          this.referencedLimit[map.limit_ids[j]].push(doc.id)
        }
      }
    },

    highlightSearchValue(text: string) {
      if (!this.searchValue) {
        return text
      }
      return text && text.replace(this.searchValueRegex, (str: string) => {
        return `<mark>${str}</mark>`
      })
    },

    connectionsDisplayText(doc: SearchDocument) {
      let connections = ''
      if (doc.connectedACL && doc.connectedACL.length > 0) {
        const highlightedConnectedEntities = this.highlightSearchValue(doc.connectedACL.join('<br/>'))
        connections = connections.concat(
            `<b>${this.componentsMap['aclpolicies'].title}:</b><br/>${highlightedConnectedEntities}<br/>`)
      }
      if (doc.connectedWAF && doc.connectedWAF.length > 0) {
        const highlightedConnectedEntities = this.highlightSearchValue(doc.connectedWAF.join('<br/>'))
        connections = connections.concat(
            `<b>${this.componentsMap['wafpolicies'].title}:</b><br/>${highlightedConnectedEntities}<br/>`)
      }
      if (doc.connectedRateLimits && doc.connectedRateLimits.length > 0) {
        const highlightedConnectedEntities = this.highlightSearchValue(doc.connectedRateLimits.join('<br/>'))
        connections = connections.concat(
            `<b>${this.componentsMap['ratelimits'].title}:</b><br/>${highlightedConnectedEntities}<br/>`)
      }
      if (doc.connectedURLMaps && doc.connectedURLMaps.length > 0) {
        const highlightedConnectedEntities = this.highlightSearchValue(doc.connectedURLMaps.join('<br/>'))
        connections = connections.concat(
            `<b>${this.componentsMap['urlmaps'].title}:</b><br/>${highlightedConnectedEntities}<br/>`)
      }
      return connections
    },

    async switchBranch() {
      await this.loadDocs()
      Utils.toast(`Switched to branch ${this.selectedBranch}.`, 'is-info')
    },

    goToDocument(doc: SearchDocument) {
      const docRoute = `/config/${this.selectedBranch}/${doc.docType}/${doc.id}`
      this.$router.push(docRoute)
    },

    // Collect every request to display a loading indicator
    // The loading indicator will be displayed as long as at least one request is still active (counter > 0)
    setLoadingStatus(isLoading: boolean) {
      if (isLoading) {
        this.loadingCounter++
      } else {
        this.loadingCounter--
      }
    },

    mouseLeave() {
      this.rowOverIndex = null
    },

    mouseOver(index: number) {
      this.rowOverIndex = index
    },

  },

  async created() {
    this.setLoadingStatus(true)
    await this.loadConfigs()
    this.setLoadingStatus(false)
  },

})
</script>
<style scoped lang="scss">
.vertical-scroll {
  max-height: 4.5rem;
}

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
