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
              <tr>
                <td>Min Risk Level</td>
                <td>
                  <input required
                         class="input is-small min-headers-risk-input"
                         type="number"
                         @change="emitDocUpdate"
                         title="Min headers risk"
                         v-model.number="localDoc.min_headers_risk"/>
                </td>
                <td>
                  <input required
                         class="input is-small min-cookies-risk-input"
                         type="number"
                         @change="emitDocUpdate"
                         title="Min cookies risk"
                         v-model.number="localDoc.min_cookies_risk"/>
                </td>
                <td>
                  <input required
                         class="input is-small min-args-risk-input"
                         type="number"
                         @change="emitDocUpdate"
                         title="Min arguments risk"
                         v-model.number="localDoc.min_args_risk"/>
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
                        <th class="has-text-centered width-30pct">Parameter</th>
                        <th class="has-text-centered width-25pct">Matching Value</th>
                        <th class="has-text-centered width-5pct">Restrict?</th>
                        <th class="has-text-centered width-5pct">Mask?</th>
                        <th class="has-text-centered width-30pct">Ignore Content Filter</th>
                        <th class="has-text-centered width-5pct">
                          <a v-show="newContentFilterLine !== tab"
                             class="has-text-grey-dark is-small new-parameter-button"
                             title="Add new parameter"
                             tabindex="0"
                             @click="openAddNewParameter(tab)"
                             @keypress.space.prevent
                             @keypress.space="openAddNewParameter(tab)"
                             @keypress.enter="openAddNewParameter(tab)">
                            <span class="icon is-small"><i class="fas fa-plus"></i></span>
                          </a>
                          <a v-show="newContentFilterLine === tab"
                             class="has-text-grey-dark is-small cancel-new-parameter"
                             title="Cancel adding new parameter"
                             tabindex="0"
                             @click="cancelNewParemeter"
                             @keypress.space.prevent
                             @keypress.space="cancelNewParemeter"
                             @keypress.enter="cancelNewParemeter">
                            <span class="icon is-small"><i class="fas fa-minus"></i></span>
                          </a>
                        </th>
                      </tr>
                      </thead>
                      <tbody>
                      <tr v-if="newContentFilterLine === tab"
                          class="has-background-warning-light new-parameter-row">
                        <td class="px-0 py-0 width-30pct">
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
                                             title="Key" />
                                    </div>
                                  </div>
                                </div>
                              </td>
                            </tr>
                          </table>
                        </td>
                        <td class="width-25pct">
                          <p class="control has-icons-left">
                            <input required
                                   class="input is-small new-entry-reg"
                                   type="text"
                                   v-model="newEntry.reg"
                                   placeholder="Value"
                                   title="Value regex" />
                            <span class="icon is-small is-left has-text-grey">
                              <i class="fas fa-code"></i>
                            </span>
                          </p>
                        </td>
                        <td class="has-text-centered width-5pct">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="new-entry-restrict"
                                   v-model="newEntry.restrict"/>
                          </label>
                        </td>
                        <td class="has-text-centered width-5pct">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="new-entry-mask"
                                   v-model="newEntry.mask"/>
                          </label>
                        </td>
                        <td class="width-30pct">
                          <autocomplete-input
                            input-type="textarea"
                            :suggestions="entryExclusionsSuggestions(newEntry)"
                            :clear-input-after-selection="false"
                            :auto-focus="false"
                            class="new-entry-exclusions"
                            selection-type="multiple"
                            :title="autocompleteTitle"
                            @value-submitted="updateEntryExclusions(newEntry, $event)" />
                        </td>
                        <td class="has-text-centered width-5pct">
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
                        <td class="width-30pct">
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
                        <td class="width-25pct">
                          <p class="control has-icons-left">
                            <input required
                                   class="input is-small entry-reg"
                                   type="text"
                                   @change="emitDocUpdate"
                                   v-model="entry.reg"
                                   placeholder="Value"
                                   title="Value regex" />
                            <span class="icon is-small is-left has-text-grey">
                              <i class="fas fa-code"></i>
                            </span>
                          </p>
                        </td>
                        <td class="has-text-centered width-5pct">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="entry-restrict"
                                   @change="emitDocUpdate"
                                   v-model="entry.restrict"/>
                          </label>
                        </td>
                        <td class="has-text-centered width-5pct">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="entry-mask"
                                   @change="emitDocUpdate"
                                   v-model="entry.mask"/>
                          </label>
                        </td>
                        <td class="width-30pct">
                          <autocomplete-input
                            input-type="textarea"
                            :suggestions="entryExclusionsSuggestions(entry)"
                            :clear-input-after-selection="false"
                            :initial-value="unpackExclusions(entry.exclusions)"
                            :auto-focus="false"
                            class="entry-exclusions"
                            selection-type="multiple"
                            :title="autocompleteTitle"
                            @value-submitted="updateEntryExclusions(entry, $event)" />
                        </td>
                        <td class="has-text-centered width-5pct">
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
                        <td class="width-30pct">
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
                        <td class="width-25pct">
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
                        <td class="has-text-centered width-5pct">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="entry-restrict"
                                   @change="emitDocUpdate"
                                   v-model="entry.restrict" />
                          </label>
                        </td>
                        <td class="has-text-centered width-5pct">
                          <label class="checkbox">
                            <input type="checkbox"
                                   class="entry-mask"
                                   @change="emitDocUpdate"
                                   v-model="entry.mask" />
                          </label>
                        </td>
                        <td class="width-30pct">
                          <autocomplete-input
                            input-type="textarea"
                            :suggestions="entryExclusionsSuggestions(entry)"
                            :clear-input-after-selection="false"
                            :initial-value="unpackExclusions(entry.exclusions)"
                            :auto-focus="false"
                            class="entry-exclusions"
                            selection-type="multiple"
                            :title="autocompleteTitle"
                            @value-submitted="updateEntryExclusions(entry, $event)" />
                        </td>
                        <td class="has-text-centered width-5pct">
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
import {
  ArgsCookiesHeadersType, NamesRegexType, ContentFilterEntryMatch, ContentFilterProfile, ContentFilterIgnoreType,
  ContentFilterRuleGroup, ContentFilterRule,
} from '@/types'
import AutocompleteInput, {AutocompleteSuggestion} from '@/components/AutocompleteInput.vue'
import RequestsUtils from '@/assets/RequestsUtils'

export default Vue.extend({
  name: 'ContentFilterEditor',
  components: {AutocompleteInput},
  props: {
    selectedDoc: Object,
    selectedBranch: String,
    apiPath: String,
  },

  data() {
    const defaultNewEntry: ContentFilterEntryMatch = {
      type: 'names',
      key: '',
      reg: '',
      restrict: false,
      mask: false,
      exclusions: {},
    }
    return {
      tab: 'args' as ArgsCookiesHeadersType,
      newContentFilterLine: null as ArgsCookiesHeadersType,
      newEntry: defaultNewEntry,
      titles: DatasetsUtils.titles,
      defaultNewEntry: defaultNewEntry,
      contentFilterSuggestions: [] as AutocompleteSuggestion[],
      contentFilter: {
        group: [] as ContentFilterRuleGroup[],
        rule: [] as ContentFilterRule[],
      },
      autocompleteTitle: 'Rule or group name',
      groupSuffix: '(Group)',
    }
  },

  computed: {
    localDoc(): ContentFilterProfile {
      return _.cloneDeep(this.selectedDoc)
    },
  },

  methods: {
    emitDocUpdate() {
      this.$emit('update:selectedDoc', this.localDoc)
    },

    openAddNewParameter(tab: ArgsCookiesHeadersType) {
      this.newContentFilterLine = tab
      this.newEntry = {...this.defaultNewEntry}
    },

    cancelNewParemeter() {
      this.newContentFilterLine = null
      this.newEntry = {...this.defaultNewEntry}
    },

    addNewParameter() {
      const newEntry = _.cloneDeep(this.newEntry)
      this.newEntry = null
      this.newContentFilterLine = null
      const type: NamesRegexType = newEntry.type
      delete newEntry.type
      this.localDoc[this.tab][type].unshift(newEntry)
      this.emitDocUpdate()
    },

    updateEntryExclusions(entry: ContentFilterEntryMatch, exclusions: string) {
      const result: ContentFilterEntryMatch['exclusions'] = {}
      const exclusionsArray: string[] = exclusions.trim().split('\n')
      exclusionsArray.forEach((ex) => {
        const exclusionType = ex.endsWith(this.groupSuffix) ? 'group' : 'rule'
        const exId = this.getContentFilterId(exclusionType, ex)
        if (exId) {
          result[exId] = exclusionType
        }
      })
      entry.exclusions = result
      this.emitDocUpdate()
    },

    entryExclusionsSuggestions({exclusions}: ContentFilterEntryMatch) {
      return _.filter(this.contentFilterSuggestions, ({value}) => {
        const exclusionType = value.endsWith(this.groupSuffix) ? 'group' : 'rule'
        const exId = this.getContentFilterId(exclusionType, value)
        return !exclusions?.[exId]
      })
    },

    unpackExclusions(exclusions: ContentFilterEntryMatch['exclusions']) {
      const result: string[] = []
      Object.keys(exclusions).forEach(
        (exId) => {
          const exclusionType = exclusions[exId]
          const name = (this.contentFilter[exclusionType] as (ContentFilterRule | ContentFilterRuleGroup)[])?.find(
            ({id}) => id === exId,
          )?.name
          if (name) {
            result.push(exclusionType === 'group' ? `${name} ${this.groupSuffix}` : name)
          }
        },
      )
      return result.join('\n')
    },

    getContentFilterId(exclusionType: ContentFilterIgnoreType, exclusions: string) {
      return (this.contentFilter[exclusionType] as (ContentFilterRule | ContentFilterRuleGroup)[]).find(
        ({name}) => exclusions.includes(name),
      )?.id
    },

    genRowKey(tab: string, type: string, idx: number) {
      return `${tab}-${type}-${idx}`
    },

    deleteEntryRow(tab: ArgsCookiesHeadersType, type: NamesRegexType, index: number) {
      this.localDoc[tab][type].splice(index, 1)
      this.emitDocUpdate()
    },

    async loadContentFilterRuleIDs() {
      const [cfRules, cfGroups] = await Promise.all([
        RequestsUtils.sendRequest({
          methodName: 'GET',
          url: `configs/${this.selectedBranch}/d/contentfilterrules/`,
          config: {headers: {'x-fields': 'id, name'}},
        }),
        RequestsUtils.sendRequest({
          methodName: 'GET',
          url: `configs/${this.selectedBranch}/d/contentfiltergroups/`,
          config: {headers: {'x-fields': 'id, name'}},
        }),
      ])
      this.contentFilter = {
        rule: cfRules.data,
        group: cfGroups.data,
      }
      this.contentFilterSuggestions = [
        ..._.sortBy(_.map(this.contentFilter.rule, ({name}) => ({value: name}))),
        ..._.sortBy(_.map(this.contentFilter.group, ({name}) => ({value: `${name} ${this.groupSuffix}`}))),
      ]
    },
  },

  created() {
    this.loadContentFilterRuleIDs()
  },
})
</script>

<style>
  .dropdown .dropdown-menu {
    width: auto;
  }
</style>
