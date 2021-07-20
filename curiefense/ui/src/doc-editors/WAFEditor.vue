<template>
  <div>
    <div class="card">
      <div class="card-content">
        <div class="media">
          <div class="media-content">
            <div class="columns">
              <div class="column is-4">
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
              </div>
            </div>
          </div>
        </div>
        <div class="tile is-ancestor px-3 py-3 mx-0 my-0">
          <div class="tile is-9">
            <table class="table is-fullwidth">
              <thead>
              <tr>
                <th></th>
                <th class="has-text-centered">Headers</th>
                <th class="has-text-centered">Cookies</th>
                <th class="has-text-centered">Arguments</th>
              </tr>
              </thead>
              <tbody>
              <tr>
                <td>Max Length</td>
                <td>
                  <input required
                         class="input is-small max-header-length-input"
                         type="number"
                         @change="emitDocUpdate"
                         title="Max header length"
                         v-model.number="localDoc.max_header_length"/>
                </td>
                <td>
                  <input required
                         class="input is-small max-cookie-length-input"
                         type="number"
                         @change="emitDocUpdate"
                         title="Max cookie length"
                         v-model.number="localDoc.max_cookie_length"/>
                </td>
                <td>
                  <input required
                         class="input is-small max-arg-length-input"
                         type="number"
                         @change="emitDocUpdate"
                         title="Max argument length"
                         v-model.number="localDoc.max_arg_length"/>
                </td>
              </tr>
              <tr>
                <td>Max Count</td>
                <td>
                  <input required
                         class="input is-small max-headers-count-input"
                         type="number"
                         @change="emitDocUpdate"
                         title="Max headers count"
                         v-model.number="localDoc.max_headers_count"/>
                </td>
                <td>
                  <input required
                         class="input is-small max-cookies-count-input"
                         type="number"
                         @change="emitDocUpdate"
                         title="Max cookies count"
                         v-model.number="localDoc.max_cookies_count"/>
                </td>
                <td>
                  <input required
                         class="input is-small max-args-count-input"
                         type="number"
                         @change="emitDocUpdate"
                         title="Max arguments count"
                         v-model.number="localDoc.max_args_count"/>
                </td>
              </tr>
              </tbody>
            </table>
          </div>
          <div class="tile is-3">
            <div class="card">
              <div class="card-content">
                <label class="checkbox">
                  <input type="checkbox"
                         class="ignore-alphanumeric-input"
                         @change="emitDocUpdate"
                         v-model="localDoc.ignore_alphanum"/>
                  Ignore Alphanumeric input
                </label>
                <p class="help">When checked, arguments, headers or cookies, which contain only alpha numeric
                  characters, will be ignored.</p>
              </div>
            </div>
          </div>
        </div>
        <div class="tile is-ancestor px-3 py-3 mx-0 my-0">
          <div class="tile is-parent">
            <div class="tile is-12">
              <table class="table is-fullwidth">
                <tr>
                  <td>
                    <div class="tabs is-centered">
                      <ul>
                        <li :class=" tab === 'headers' ? 'is-active' : '' "
                            class="headers-tab">
                          <a tabindex="0"
                             @click='tab="headers"'
                             @keypress.space.prevent
                             @keypress.space='tab="headers"'
                             @keypress.enter='tab="headers"'>
                            Headers
                          </a>
                        </li>
                        <li :class=" tab === 'cookies' ? 'is-active' : '' "
                            class="cookies-tab">
                          <a tabindex="0"
                             @click='tab="cookies"'
                             @keypress.space.prevent
                             @keypress.space='tab="cookies"'
                             @keypress.enter='tab="cookies"'>
                            Cookies
                          </a>
                        </li>
                        <li :class=" tab === 'args' ? 'is-active' : '' "
                            class="args-tab">
                          <a tabindex="0"
                             @click='tab="args"'
                             @keypress.space.prevent
                             @keypress.space='tab="args"'
                             @keypress.enter='tab="args"'>
                            Arguments
                          </a>
                        </li>
                      </ul>
                    </div>
                  </td>
                </tr>
                <tr>
                  <td>
                    <table class="table is-fullwidth is-hoverable"
                           v-if="localDoc && localDoc[tab]">
                      <thead>
                      <tr>
                        <th class="has-text-centered">Parameter</th>
                        <th class="has-text-centered">Matching Value</th>
                        <th class="has-text-centered">Restrict?</th>
                        <th class="has-text-centered">Mask?</th>
                        <th class="has-text-centered">Exclude WAF Rule</th>
                        <th class="has-text-centered">
                          <a v-show="newWAFLine !== tab"
                             class="has-text-grey-dark is-small new-parameter-button"
                             title="Add new parameter"
                             tabindex="0"
                             @click="openAddNewParameter(tab)"
                             @keypress.space.prevent
                             @keypress.space="openAddNewParameter(tab)"
                             @keypress.enter="openAddNewParameter(tab)">
                            <span class="icon is-small"><i class="fas fa-plus"></i></span>
                          </a>
                          <a v-show="newWAFLine === tab"
                             class="has-text-grey-dark is-small" title="Cancel adding new parameter"
                             tabindex="0"
                             @click="newWAFLine = null"
                             @keypress.space.prevent
                             @keypress.space="newWAFLine = null"
                             @keypress.enter="newWAFLine = null">
                            <span class="icon is-small"><i class="fas fa-minus"></i></span>
                          </a>
                        </th>
                      </tr>
                      </thead>
                      <tbody>
                      <tr v-if="newWAFLine === tab"
                          class="has-background-warning-light new-parameter-row">
                        <td class="px-0 py-0">
                          <table class="table is-fullwidth has-background-warning-light">
                            <tr>
                              <td class="is-fullwidth">
                                <div class="field">
                                  <div class="control">
                                    <div class="select is-small">
                                      <select v-model="newEntry.type"
                                              class="new-entry-type"
                                              title="Type">
                                        <option value="names">
                                          {{ titles.names }}
                                        </option>
                                        <option value="regex">
                                          {{ titles.regex }}
                                        </option>
                                      </select>
                                    </div>
                                  </div>
                                </div>
                              </td>
                              <td>
                                <div class="field">
                                  <div class="control">
                                    <div>
                                      <input required
                                             class="input is-small new-entry-key"
                                             type="text"
                                             v-model="newEntry.key"
                                             placeholder="Key"
                                             title="Key"/>
                                    </div>
                                  </div>
                                </div>
                              </td>
                            </tr>
                          </table>
                        </td>
                        <td>
                          <p class="control has-icons-left">
                            <input required
                                   class="input is-small new-entry-reg"
                                   type="text"
                                   v-model="newEntry.reg"
                                   placeholder="Value"
                                   title="Value regex"/>
                            <span class="icon is-small is-left has-text-grey">
                                  <i class="fas fa-code"></i>
                                </span>
                          </p>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="new-entry-restrict"
                                   v-model="newEntry.restrict"/>
                          </label>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="new-entry-mask"
                                   v-model="newEntry.mask"/>
                          </label>
                        </td>
                        <td>
                          <autocomplete-input
                              :suggestions="entryExclusionsSuggestions(newEntry)"
                              :clear-input-after-selection="false"
                              :auto-focus="false"
                              class="new-entry-exclusions"
                              selection-type="multiple"
                              title="Space separated rule IDs"
                              @value-submitted="updateEntryExclusions(newEntry, $event)"/>
                        </td>
                        <td class="has-text-centered">
                          <button title="Add new parameter"
                                  class="button is-light is-small confirm-add-new-parameter"
                                  @click="addNewParameter">
                            <span class="icon is-small"><i class="fas fa-plus fa-xs"></i></span>
                          </button>
                        </td>

                      </tr>
                      <tr v-for="(entry, idx) in localDoc[tab].names"
                          class="entry-row"
                          :key="genRowKey(tab, 'names', idx)">
                        <td>
                          <div class="field">
                            <p class="control has-icons-left">
                              <input required
                                     class="input is-small entry-key"
                                     type="text"
                                     @change="emitDocUpdate"
                                     v-model="entry.key"
                                     placeholder="Key"
                                     title="Key name"/>
                              <span class="icon is-small is-left has-text-grey">
                                <i class="fas fa-font"></i>
                              </span>
                            </p>
                          </div>
                        </td>
                        <td>
                          <p class="control has-icons-left">
                            <input required
                                   class="input is-small entry-reg"
                                   type="text"
                                   @change="emitDocUpdate"
                                   v-model="entry.reg"
                                   placeholder="Value"
                                   title="Value regex"/>
                            <span class="icon is-small is-left has-text-grey">
                              <i class="fas fa-code"></i>
                            </span>
                          </p>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="entry-restrict"
                                   @change="emitDocUpdate"
                                   v-model="entry.restrict"/>
                          </label>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="entry-mask"
                                   @change="emitDocUpdate"
                                   v-model="entry.mask"/>
                          </label>
                        </td>
                        <td>
                          <autocomplete-input
                              :suggestions="entryExclusionsSuggestions(entry)"
                              :clear-input-after-selection="false"
                              :initial-value="unpackExclusions(entry.exclusions)"
                              :auto-focus="false"
                              class="entry-exclusions"
                              selection-type="multiple"
                              title="Space separated rule IDs"
                              @value-submitted="updateEntryExclusions(entry, $event)"/>
                        </td>
                        <td class="has-text-centered">
                          <button title="Delete entry"
                                  :data-curie="genRowKey(tab, 'names', idx)"
                                  @click="deleteEntryRow(tab, 'names', idx)"
                                  class="button is-light is-small remove-entry-button">
                              <span class="icon is-small">
                                <i class="fas fa-trash fa-xs"></i>
                              </span>
                          </button>
                        </td>
                      </tr>
                      <tr v-for="(entry, idx) in localDoc[tab].regex"
                          class="entry-row"
                          :key="genRowKey(tab, 'regex', idx)">
                        <td>
                          <div class="field">
                            <p class="control has-icons-left">
                              <input required
                                     class="input is-small entry-key"
                                     type="text"
                                     @change="emitDocUpdate"
                                     v-model="entry.key"
                                     placeholder="Key"
                                     title="Key regex"/>
                              <span class="icon is-small is-left has-text-grey">
                                <i class="fas fa-code"></i>
                              </span>
                            </p>
                          </div>
                        </td>
                        <td>
                          <p class="control has-icons-left">
                            <input required
                                   class="input is-small entry-reg"
                                   type="text"
                                   @change="emitDocUpdate"
                                   v-model="entry.reg"
                                   placeholder="Value"
                                   title="Value regex"/>
                            <span class="icon is-small is-left has-text-grey">
                                  <i class="fas fa-code"></i>
                            </span>
                          </p>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="entry-restrict"
                                   @change="emitDocUpdate"
                                   v-model="entry.restrict"/>
                          </label>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="entry-mask"
                                   @change="emitDocUpdate"
                                   v-model="entry.mask"/>
                          </label>
                        </td>
                        <td>
                          <autocomplete-input
                              :suggestions="entryExclusionsSuggestions(entry)"
                              :clear-input-after-selection="false"
                              :initial-value="unpackExclusions(entry.exclusions)"
                              :auto-focus="false"
                              class="entry-exclusions"
                              selection-type="multiple"
                              title="Space separated rule IDs"
                              @value-submitted="updateEntryExclusions(entry, $event)"/>
                        </td>
                        <td class="has-text-centered">
                          <button title="Delete entry"
                                  :data-curie="genRowKey(tab, 'regex', idx)"
                                  @click="deleteEntryRow(tab, 'regex', idx)"
                                  class="button is-light is-small remove-entry-button">
                              <span class="icon is-small">
                                <i class="fas fa-trash fa-xs"></i>
                              </span>
                          </button>
                        </td>
                      </tr>
                      </tbody>
                    </table>
                  </td>
                </tr>
              </table>

            </div>
          </div>
        </div>
        <span class="is-family-monospace has-text-grey-lighter">{{ apiPath }}</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import _ from 'lodash'
import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import Vue from 'vue'
import {ArgsCookiesHeadersType, NamesRegexType, WAFEntryMatch, WAFPolicy, WAFRule} from '@/types'
import AutocompleteInput, {AutocompleteSuggestion} from '@/components/AutocompleteInput.vue'
import RequestsUtils from '@/assets/RequestsUtils'
import {AxiosResponse} from 'axios'

export default Vue.extend({
  name: 'WAFEditor',
  components: {AutocompleteInput},
  props: {
    selectedDoc: Object,
    selectedBranch: String,
    apiPath: String,
  },

  data() {
    const defaultNewEntry: WAFEntryMatch = {
      type: 'names',
      key: '',
      reg: '',
      restrict: false,
      mask: false,
      exclusions: null,
    }
    return {
      tab: 'args' as ArgsCookiesHeadersType,
      newWAFLine: null as ArgsCookiesHeadersType,
      newEntry: defaultNewEntry,
      titles: DatasetsUtils.titles,
      defaultNewEntry: defaultNewEntry,
      wafRuleIDsSuggestions: [] as AutocompleteSuggestion[],
    }
  },

  computed: {
    localDoc(): WAFPolicy {
      return _.cloneDeep(this.selectedDoc)
    },
  },

  methods: {
    emitDocUpdate() {
      this.$emit('update:selectedDoc', this.localDoc)
    },

    openAddNewParameter(tab: ArgsCookiesHeadersType) {
      this.newWAFLine = tab
      this.newEntry = {...this.defaultNewEntry}
    },

    addNewParameter() {
      const newEntry = _.cloneDeep(this.newEntry)
      this.newEntry = this.newWAFLine = null
      const type: NamesRegexType = newEntry.type
      delete newEntry.type
      this.localDoc[this.tab][type].unshift(newEntry)
      this.emitDocUpdate()
    },

    updateEntryExclusions(entry: WAFEntryMatch, exclusions: string) {
      entry.exclusions = this.packExclusions(exclusions)
      this.emitDocUpdate()
    },

    entryExclusionsSuggestions(entry: WAFEntryMatch) {
      return _.filter(this.wafRuleIDsSuggestions, ((suggestion) => {
        return !_.keys(entry.exclusions).includes(suggestion.value)
      }))
    },

    packExclusions(exclusions: string) {
      const ret = {}
      if (_.size(exclusions) === 0 || !exclusions) {
        return ret
      }

      return _.fromPairs(_.map(exclusions.split(' '), (ex) => {
        return [ex.trim(), 1]
      }))
    },

    unpackExclusions(exclusions: WAFEntryMatch['exclusions']) {
      return _.keys(exclusions).join(' ')
    },

    genRowKey(tab: string, type: string, idx: number) {
      return `${tab}-${type}-${idx}`
    },

    deleteEntryRow(tab: ArgsCookiesHeadersType, type: NamesRegexType, index: number) {
      this.localDoc[tab][type].splice(index, 1)
      this.emitDocUpdate()
    },

    loadWAFRuleIDs() {
      const branch = this.selectedBranch

      RequestsUtils.sendRequest({
        methodName: 'GET',
        url: `configs/${branch}/d/wafrules/`,
        config: {headers: {'x-fields': 'id'}},
      }).then((response: AxiosResponse<WAFRule[]>) => {
        this.wafRuleIDsSuggestions = _.sortBy(_.map(response.data, (entity) => {
          return {
            value: entity.id,
          }
        }))
      })
    },
  },
  watch: {
    selectedDoc: {
      handler: function(val, oldVal) {
        if (!val || !oldVal || val.id !== oldVal.id) {
          this.loadWAFRuleIDs()
        }
      },
      immediate: true,
      deep: true,
    },
  },
})
</script>
