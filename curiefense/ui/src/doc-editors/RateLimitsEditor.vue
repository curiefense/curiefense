<template>
  <section>
    <div class="card">
      <div class="card-content">
        <div class="content">
          <div class="columns columns-divided">
            <div class="column is-5">
              <div class="field">
                <label class="label is-small">
                  Name
                  <span class="has-text-grey is-pulled-right document-id"
                        title="Document id">
                    {{ localDoc.id }}
                  </span>
                </label>
                <div class="control">
                  <input class="input is-small document-name"
                         title="Document name"
                         placeholder="Document name"
                         @change="emitDocUpdate"
                         v-model="localDoc.name"/>
                </div>
              </div>
              <div class="field">
                <label class="label is-small">
                  Description
                </label>
                <div class="control">
                  <input class="input is-small document-description"
                         type="text"
                         title="Rate limit rule description"
                         placeholder="Rate limit rule description"
                         @change="emitDocUpdate"
                         v-model="localDoc.description">
                </div>
              </div>
              <div class="field">
                <label class="label is-small">
                  Threshold
                </label>
                <div class="control">
                  <input class="input is-small document-limit"
                         type="text"
                         title="Rate limit threshold"
                         placeholder="Rate limit threshold"
                         @change="emitDocUpdate"
                         v-model="localDoc.limit">
                </div>
              </div>
              <div class="field">
                <label class="label is-small">
                  TTL
                </label>
                <div class="control suffix seconds-suffix">
                  <input class="input is-small document-ttl"
                         type="text"
                         title="Rate limit duration"
                         placeholder="Rate limit duration"
                         @change="emitDocUpdate"
                         v-model="localDoc.ttl">
                </div>
              </div>
              <div class="group-key mb-3">
                <limit-option v-for="(option, index) in localDoc.key"
                              label-separated-line
                              :label="index === 0 ? 'Count by' : ' '"
                              show-remove
                              @remove="removeKey(index)"
                              @change="updateKeyOption($event, index)"
                              :removable="localDoc.key.length > 1"
                              :ignore-attributes="['tags']"
                              :option="generateOption(option)"
                              :key="getOptionTextKey(option, index)"/>
                <a title="Add new option rule"
                   class="is-text is-small is-size-7 ml-3 add-key-button"
                   tabindex="0"
                   @click="addKey()"
                   @keypress.space.prevent
                   @keypress.space="addKey()"
                   @keypress.enter="addKey()">
                  New entry
                </a>
                <p class="has-text-danger is-size-7 ml-3 mt-3 key-invalid"
                   v-if="!keysAreValid">
                  Count-by entries must be unique
                </p>
              </div>
              <div class="group-event mb-3">
                <limit-option use-default-self
                              label-separated-line
                              label="Event"
                              :option.sync="eventOption"
                              :key="eventOption.type + localDoc.id"
                              :ignore-attributes="['tags']"
                              @change="updateEvent"/>
              </div>
              <div class="field">
                <response-action :action.sync="localDoc.action"
                                 label-separated-line
                                 @update:action="emitDocUpdate"/>
              </div>
            </div>
            <div class="column is-7">
              <div class="columns">
                <div class="column is-6 filter-column"
                     v-for="filter in filters"
                     :key="filter"
                     :class="filter + '-filter-column'">
                  <p class="title is-7">
                    {{ titles[filter] }}
                  </p>
                  <hr class="bar"
                      :class="`bar-${filter}`"/>
                  <table class="table is-narrow is-fullwidth">
                    <tbody>
                    <tr v-for="(tag, tagIndex) in localDoc[filter]"
                        :key="tagIndex">
                      <td class="tag-cell"
                          :class=" duplicateTags[tag] ? 'has-text-danger' : '' ">
                        {{ tag }}
                      </td>
                      <td class="is-size-7 width-20px">
                        <a title="Remove entry"
                           class="is-small has-text-grey remove-filter-entry-button"
                           tabindex="0"
                           @click="removeTag(filter, tagIndex)"
                           @keypress.space.prevent
                           @keypress.space="removeTag(filter, tagIndex)"
                           @keypress.enter="removeTag(filter, tagIndex)">
                          &ndash;
                        </a>
                      </td>
                    </tr>
                    <tr>
                      <td>
                        <tag-autocomplete-input v-if="addNewTagColName === filter"
                                                ref="tagAutocompleteInput"
                                                :clear-input-after-selection="true"
                                                :selection-type="'single'"
                                                :auto-focus="true"
                                                @keydown.esc="cancelAddNewTag"
                                                @tag-submitted="addNewTag(filter, $event)">
                        </tag-autocomplete-input>
                      </td>
                      <td class="is-size-7 width-20px">
                        <a title="add new entry"
                           class="is-size-7 width-20px is-small has-text-grey add-new-filter-entry-button"
                           tabindex="0"
                           @click="openTagInput(filter)"
                           @keypress.space.prevent
                           @keypress.space="openTagInput(filter)"
                           @keypress.enter="openTagInput(filter)">
                          +
                        </a>
                      </td>
                    </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
          </div>
          <div>
            <div class="has-text-left has-text-weight-bold pb-3">Connections to URL Maps</div>
            <table class="table connected-url-maps-table">
              <thead>
              <tr>
                <th class="is-size-7 width-200px">Name</th>
                <th class="is-size-7 width-120px">ID</th>
                <th class="is-size-7 width-300px">Domain Match</th>
                <th class="is-size-7 width-300px">Entry Match</th>
                <th class="is-size-7 width-80px has-text-centered">
                  <a v-if="!newURLMapConnectionOpened"
                     class="has-text-grey-dark is-small new-connection-button"
                     title="Add new connection"
                     tabindex="0"
                     @click="openNewURLMapConnection"
                     @keypress.space.prevent
                     @keypress.space="openNewURLMapConnection"
                     @keypress.enter="openNewURLMapConnection">
                    <span class="icon is-small"><i class="fas fa-plus"></i></span>
                  </a>
                  <a v-else
                     class="has-text-grey-dark is-small new-connection-button"
                     title="Cancel adding new connection"
                     tabindex="0"
                     @click="closeNewURLMapConnection"
                     @keypress.space.prevent
                     @keypress.space="closeNewURLMapConnection"
                     @keypress.enter="closeNewURLMapConnection">
                    <span class="icon is-small"><i class="fas fa-minus"></i></span>
                  </a>
                </th>
              </tr>
              </thead>
              <tbody>
              <tr v-if="newURLMapConnectionOpened"
                  class="has-background-warning-light new-connection-row">
                <template v-if="newURLMapConnections.length > 0">
                  <td>
                    <div class="select is-small">
                      <select v-model="newURLMapConnectionData.map"
                              @change="newURLMapConnectionData.entryIndex = 0"
                              class="new-connection-map"
                              title="Type">
                        <option v-for="map in newURLMapConnections" :key="map.id" :value="map">
                          {{ map.name }}
                        </option>
                      </select>
                    </div>
                  </td>
                  <td>
                    {{ newURLMapConnectionData.map.id }}
                  </td>
                  <td>
                    {{ newURLMapConnectionData.map.match }}
                  </td>
                  <td>
                    <div class="select is-small">
                      <select v-model="newURLMapConnectionData.entryIndex"
                              class="new-connection-entry-index"
                              title="Type">
                        <option v-for="(mapEntry, index) in newURLMapConnectionEntries"
                                :key="mapEntry.match"
                                :value="index">
                          {{ mapEntry.match }}
                        </option>
                      </select>
                    </div>
                  </td>
                  <td class="has-text-centered">
                    <button title="Add new connection"
                            class="button is-light is-small add-new-connection"
                            @click="addNewURLMapConnection">
                      <span class="icon is-small"><i class="fas fa-plus fa-xs"></i></span>
                    </button>
                  </td>
                </template>
                <template v-else>
                  <td colspan="5">
                    All URL Maps entries are currently connected to this Rate Limit
                  </td>
                </template>
              </tr>
              <tr v-for="(connection, index) in connectedURLMapsEntries" :key="index">
                <td class="is-size-7 is-vcentered py-3 width-200px connected-entry-row"
                    :title="connection[0]">
                  <a title="Add new"
                     class="url-map-referral-button"
                     @click="referToURLMap(connection.id)">
                    {{ connection.name }}
                  </a>
                </td>
                <td class="is-size-7 is-vcentered py-3 width-120px"
                    :title="connection.id">
                  {{ connection.id }}
                </td>
                <td class="is-size-7 is-vcentered py-3 width-300px"
                    :title="connection.domainMatch">
                  {{ connection.domainMatch }}
                </td>
                <td class="is-size-7 is-vcentered py-3 width-300px"
                    :title="connection.entryMatch">
                  {{ connection.entryMatch }}
                </td>
                <td class="is-size-7 is-vcentered width-80px height-50px">
                    <span v-show="currentEntryDeleteIndex !== index">
                    <a tabindex="0"
                       title="Remove connection to the URL Map"
                       class="is-small has-text-grey remove-connection-button"
                       @click="setEntryDeleteIndex(index)"
                       @keypress.space.prevent
                       @keypress.space="setEntryDeleteIndex(index)"
                       @keypress.enter="setEntryDeleteIndex(index)">
                      Remove
                    </a>
                    </span>
                  <span v-show="currentEntryDeleteIndex === index">
                      <a class="is-size-7 has-text-grey add-button confirm-remove-connection-button"
                         title="Confirm"
                         tabindex="0"
                         @click="removeURLMapConnection(connection.id, connection.entryMatch)"
                         @keypress.space.prevent
                         @keypress.space="removeURLMapConnection(connection.id, connection.entryMatch)"
                         @keypress.enter="removeURLMapConnection(connection.id, connection.entryMatch)">
                      <i class="fas fa-check"></i> Confirm
                    </a>
                    <br/>
                    <a class="is-size-7 has-text-grey cancel-remove-connection-button"
                       title="Cancel"
                       tabindex="0"
                       @click="setEntryDeleteIndex(-1)"
                       @keypress.space.prevent
                       @keypress.space="setEntryDeleteIndex(-1)"
                       @keypress.enter="setEntryDeleteIndex(-1)">
                      <i class="fas fa-times"></i> Cancel
                    </a>
                    </span>
                </td>
              </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script lang="ts">
import _ from 'lodash'
import ResponseAction from '@/components/ResponseAction.vue'
import LimitOption, {OptionObject} from '@/components/LimitOption.vue'
import TagAutocompleteInput from '@/components/TagAutocompleteInput.vue'
import Vue from 'vue'
import {IncludeExcludeType, LimitOptionType, LimitRuleType, RateLimit, URLMap, URLMapEntryMatch} from '@/types'
import {Dictionary} from 'vue-router/types/router'
import DatasetsUtils from '@/assets/DatasetsUtils'
import RequestsUtils from '@/assets/RequestsUtils'
import {AxiosResponse} from 'axios'

export default Vue.extend({
  name: 'RateLimits',
  props: {
    selectedDoc: Object,
    selectedBranch: String,
    apiPath: String,
  },
  components: {
    ResponseAction,
    LimitOption,
    TagAutocompleteInput,
  },
  data() {
    return {
      filters: ['include', 'exclude'] as IncludeExcludeType[],
      addNewTagColName: null,
      titles: DatasetsUtils.titles,
      urlMaps: [] as URLMap[],
      currentEntryDeleteIndex: -1,
      newURLMapConnectionData: {
        map: null,
        entryIndex: 0,
      } as {
        map: URLMap,
        entryIndex: number,
      },
      newURLMapConnectionOpened: false,
      connectedURLMapsEntries: [],
      keysAreValid: true,
    }
  },
  computed: {
    localDoc(): RateLimit {
      return _.cloneDeep(this.selectedDoc)
    },

    duplicateTags(): Dictionary<string> {
      const doc = this.localDoc
      const allTags = _.concat(doc['include'], doc['exclude'])
      const dupTags = _.filter(allTags, (val, i, iteratee) => _.includes(iteratee, val, i + 1))
      return _.fromPairs(_.zip(dupTags, dupTags))
    },

    eventOption: {
      get: function(): LimitOptionType {
        return this.generateOption(this.localDoc.pairwith)
      },
      set: function(value: RateLimit['pairwith']): void {
        this.localDoc.pairwith = value
        this.emitDocUpdate()
      },
    },

    newURLMapConnections(): URLMap[] {
      return this.urlMaps.filter((urlMap) => {
        return !urlMap.map.every((urlMapEntry) => {
          return urlMapEntry.limit_ids.includes(this.localDoc.id)
        })
      })
    },

    newURLMapConnectionEntries(): URLMapEntryMatch[] {
      const urlMap = this.newURLMapConnections.find((urlMap) => {
        return urlMap.id === this.newURLMapConnectionData.map?.id
      })
      return urlMap?.map?.filter((urlMapEntry) => {
        return !urlMapEntry.limit_ids.includes(this.localDoc.id)
      })
    },
  },
  methods: {
    emitDocUpdate() {
      this.$emit('update:selectedDoc', this.localDoc)
    },

    getOptionTextKey(option: LimitOptionType, index: number) {
      if (!option) {
        return ''
      }
      const [type] = Object.keys(option)
      return `${this.localDoc.id}_${type}_${index}`
    },

    generateOption(data: LimitOptionType): OptionObject {
      if (!data) {
        return {}
      }
      const [firstObjectKey] = Object.keys(data)
      const type = firstObjectKey as LimitRuleType
      const key = data[firstObjectKey]
      return {type, key, value: null}
    },

    addKey() {
      this.localDoc.key.push({attrs: 'ip'})
      this.emitDocUpdate()
      this.checkKeysValidity()
    },

    removeKey(index: number) {
      if (this.localDoc.key.length > 1) {
        this.localDoc.key.splice(index, 1)
      }
      this.emitDocUpdate()
      this.checkKeysValidity()
    },

    updateKeyOption(option: OptionObject, index: number) {
      this.localDoc.key.splice(index, 1, {
        [option.type]: option.key,
      })
      this.emitDocUpdate()
      this.checkKeysValidity()
    },

    checkKeysValidity() {
      const keysToCheck = _.countBy(this.localDoc.key, (item) => {
        if (!item) {
          return ''
        }
        const key = Object.keys(item)[0]
        return `${key}_${item[key]}`
      })
      this.keysAreValid = true
      for (const key of Object.keys(keysToCheck)) {
        if (keysToCheck[key] > 1 || keysToCheck[''] > 0) {
          this.keysAreValid = false
          break
        }
      }
      return this.keysAreValid
    },

    updateEvent(option: OptionObject) {
      this.eventOption = {[option.type]: option.key}
    },

    getConnectedURLMapsEntries() {
      this.connectedURLMapsEntries = _.sortBy(_.flatMap(_.filter(this.urlMaps, (urlMap) => {
        return _.some(urlMap.map, (mapEntry: URLMapEntryMatch) => {
          return mapEntry.limit_ids.includes(this.localDoc.id)
        })
      }), (urlMap) => {
        return _.compact(_.map(urlMap.map, (mapEntry) => {
          if (mapEntry.limit_ids.includes(this.localDoc.id)) {
            return {
              name: urlMap.name,
              id: urlMap.id,
              domainMatch: urlMap.match,
              entryMatch: mapEntry.match,
            }
          } else {
            return null
          }
        }))
      }))
    },

    openNewURLMapConnection() {
      this.newURLMapConnectionOpened = true
      this.newURLMapConnectionData.map = this.newURLMapConnections.length > 0 ? this.newURLMapConnections[0] : null
      this.newURLMapConnectionData.entryIndex = 0
    },

    closeNewURLMapConnection() {
      this.newURLMapConnectionOpened = false
    },

    addNewURLMapConnection() {
      const id = this.newURLMapConnectionData.map?.id
      const entryMatch = this.newURLMapConnectionEntries[this.newURLMapConnectionData.entryIndex].match
      const methodName = 'PUT'
      const selectedDocType = 'urlmaps'
      const urlTrail = `configs/${this.selectedBranch}/t/${selectedDocType}/e/${id}/`
      const doc = _.find(this.urlMaps, (urlMap) => {
        return urlMap.id === id
      })
      const mapEntry = _.find(doc.map, (mapEntry) => {
        return mapEntry.match === entryMatch
      })
      mapEntry.limit_ids.push(this.localDoc.id)
      this.closeNewURLMapConnection()
      const docTypeText = this.titles[selectedDocType + '-singular']
      const successMessage = `The connection to the ${docTypeText} was added.`
      const failureMessage = `Failed while attempting to add the connection to the ${docTypeText}.`
      RequestsUtils.sendRequest({methodName, url: urlTrail, data: doc, successMessage, failureMessage}).then(() => {
        this.getConnectedURLMapsEntries()
      })
    },

    removeURLMapConnection(id: URLMap['id'], entryMatch: URLMapEntryMatch['match']) {
      const methodName = 'PUT'
      const selectedDocType = 'urlmaps'
      const urlTrail = `configs/${this.selectedBranch}/t/${selectedDocType}/e/${id}/`
      const doc = _.find(this.urlMaps, (urlMap) => {
        return urlMap.id === id
      })
      const mapEntry = _.find(doc.map, (mapEntry) => {
        return mapEntry.match === entryMatch
      })
      const limitIdIndex = _.findIndex(mapEntry.limit_ids, (rateLimitID) => {
        return rateLimitID === this.localDoc.id
      })
      mapEntry.limit_ids.splice(limitIdIndex, 1)
      const docTypeText = this.titles[selectedDocType + '-singular']
      const successMessage = `The connection to the ${docTypeText} was removed.`
      const failureMessage = `Failed while attempting to remove the connection to the ${docTypeText}.`
      RequestsUtils.sendRequest({methodName, url: urlTrail, data: doc, successMessage, failureMessage}).then(() => {
        this.setEntryDeleteIndex(-1)
        this.getConnectedURLMapsEntries()
      })
    },

    setEntryDeleteIndex(index: number) {
      this.closeNewURLMapConnection()
      this.currentEntryDeleteIndex = index
    },

    loadURLMaps() {
      RequestsUtils.sendRequest({
        methodName: 'GET',
        url: `configs/${this.selectedBranch}/t/urlmaps/`,
      }).then((response: AxiosResponse<URLMap[]>) => {
        this.urlMaps = _.sortBy(response.data)
        this.getConnectedURLMapsEntries()
        this.newURLMapConnectionData.map = this.newURLMapConnections.length > 0 ? this.newURLMapConnections[0] : null
      })
    },

    addNewTag(section: IncludeExcludeType, entry: string) {
      if (entry && entry.length > 2) {
        this.localDoc[section].push(entry)
        this.emitDocUpdate()
      }
    },

    openTagInput(section: IncludeExcludeType) {
      this.addNewTagColName = section
    },

    cancelAddNewTag() {
      this.addNewTagColName = null
    },

    removeTag(section: IncludeExcludeType, index: number) {
      this.localDoc[section].splice(index, 1)
      this.addNewTagColName = null
      this.emitDocUpdate()
    },

    referToURLMap(id: string) {
      this.$router.push(`/config/${this.selectedBranch}/urlmaps/${id}`)
    },
  },
  created() {
    this.loadURLMaps()
  },
  mounted() {
    this.checkKeysValidity()
  },
  watch: {
    selectedDoc: {
      handler: function() {
        this.getConnectedURLMapsEntries()
        this.$forceUpdate()
      },
      immediate: true,
      deep: true,
    },
  },
})
</script>

<style scoped lang="scss">

.form-label {
  padding-top: 0.25rem;
}

.bar {
  margin: 1rem 0 0.5rem;
}

.seconds-suffix {
  input {
    padding-right: 60px;
  }
}

</style>
