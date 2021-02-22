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
                    <span class="has-text-grey is-pulled-right document-id" title="Document id">
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
                        <li :class=" tab === 'headers' ? 'is-active' : '' ">
                          <a tabindex="0"
                             @click='tab="headers"'
                             @keypress.space.prevent
                             @keypress.space='tab="headers"'
                             @keypress.enter='tab="headers"'>
                            Headers
                          </a>
                        </li>
                        <li :class=" tab === 'cookies' ? 'is-active' : '' ">
                          <a tabindex="0"
                             @click='tab="cookies"'
                             @keypress.space.prevent
                             @keypress.space='tab="cookies"'
                             @keypress.enter='tab="cookies"'>
                            Cookies
                          </a>
                        </li>
                        <li :class=" tab === 'args' ? 'is-active' : '' ">
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
                             class="has-text-grey-dark is-small" title="Add new parameter"
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
                          class="has-background-warning-light">
                        <td class="px-0 py-0">
                          <table class="table is-fullwidth has-background-warning-light">
                            <tr>
                              <td class="is-fullwidth">
                                <div class="field">
                                  <div class="control ">
                                    <div class="select is-small">
                                      <select v-model="newEntry.type"
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
                                      <input required class="input is-small"
                                             type="text"
                                             v-model="newEntry.key"
                                             :title="titles.names"/>
                                    </div>
                                  </div>
                                </div>
                              </td>
                            </tr>
                          </table>
                        </td>
                        <td>
                          <p class="control has-icons-left">
                            <input required class="input is-small"
                                   type="text"
                                   v-model="newEntry.reg"
                                   :title="titles.regex"/>
                            <span class="icon is-small is-left has-text-grey">
                                  <i class="fas fa-code"></i>
                                </span>
                          </p>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox" v-model="newEntry.restrict"/>
                          </label>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox" v-model="newEntry.mask"/>
                          </label>
                        </td>
                        <td>
                          <serialized-input placeholder="Space separated rule IDs"
                                            :value="newEntry.exclusions"
                                            :get-function="unpackExclusions"
                                            :set-function="packExclusions"
                                            @blur="newEntry.exclusions = $event">
                          </serialized-input>
                        </td>
                        <td class="has-text-centered">
                          <button title="Add new parameter" class="button is-light is-small" @click="addNewParameter">
                            <span class="icon is-small"><i class="fas fa-plus fa-xs"></i></span>
                          </button>
                        </td>

                      </tr>
                      <tr v-for="(entry, idx) in localDoc[tab].names" :key="genRowKey(tab, 'names', idx)">
                        <td>
                          <div class="field">
                            <p class="control has-icons-left">
                              <input required class="input is-small"
                                     type="text"
                                     @change="emitDocUpdate"
                                     v-model="entry.key"
                                     :title="titles.names"/>
                              <span class="icon is-small is-left has-text-grey">
                                <i class="fas fa-font"></i>
                              </span>
                            </p>
                          </div>
                        <td>
                          <p class="control has-icons-left">
                            <input required class="input is-small" type="text" v-model="entry.reg"
                                   :title="titles.regex"/>
                            <span class="icon is-small is-left has-text-grey">
                              <i class="fas fa-code"></i>
                            </span>
                          </p>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox" :checked="entry.restrict"/>
                          </label>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox" v-model="entry.mask"/>
                          </label>
                        </td>
                        <td>
                          <serialized-input placeholder="Space separated rule IDs"
                                            :value="entry.exclusions"
                                            :get-function="unpackExclusions"
                                            :set-function="packExclusions"
                                            @blur="entry.exclusions = $event">
                          </serialized-input>
                        </td>
                        <td class="has-text-centered">
                          <button title="Delete entry"
                                  :data-curie="genRowKey(tab, 'names', idx)"
                                  @click="deleteWAFRow"
                                  class="button is-light is-small">
                              <span class="icon is-small">
                                <i class="fas fa-trash fa-xs"></i>
                              </span>
                          </button>
                        </td>
                      </tr>
                      <tr v-for="(entry, idx) in localDoc[tab].regex" :key="genRowKey(tab, 'regex', idx)">
                        <td>
                          <div class="field">
                            <p class="control has-icons-left">
                              <input required class="input is-small"
                                     type="text"
                                     @change="emitDocUpdate"
                                     v-model="entry.key"
                                     :title="titles.regex"/>
                              <span class="icon is-small is-left has-text-grey">
                                  <i class="fas fa-code"></i>
                              </span>
                            </p>
                          </div>
                        </td>
                        <td>
                          <p class="control has-icons-left">
                            <input required class="input is-small" type="text" v-model="entry.reg"
                                   :title="titles.regex"/>
                            <span class="icon is-small is-left has-text-grey">
                                  <i class="fas fa-code"></i>
                            </span>
                          </p>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox" :checked="entry.restrict"/>
                          </label>
                        </td>
                        <td class="has-text-centered">
                          <label class="checkbox">
                            <input type="checkbox" :checked="entry.mask"/>
                          </label>
                        </td>
                        <td>
                          <serialized-input placeholder="Space separated rule IDs"
                                            :value="entry.exclusions"
                                            :get-function="unpackExclusions"
                                            :set-function="packExclusions"
                                            @blur="entry.exclusions = $event">
                          </serialized-input>
                        </td>
                        <td class="has-text-centered">
                          <button :data-curie="genRowKey(tab, 'regex', idx)"
                                  @click="deleteWAFRow"
                                  title="Delete entry" class="button is-light is-small">
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
import SerializedInput from '@/components/SerializedInput.vue'
import Vue from 'vue'
import {ArgsCookiesHeadersType, NamesRegexType, WAFEntryMatch, WAFPolicy} from '@/types'

export default Vue.extend({
  name: 'WAFEditor',
  components: {SerializedInput},
  props: {
    selectedDoc: Object,
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

    packExclusions(exclusions: string) {
      const ret = {}
      if (_.size(exclusions) === 0 || !exclusions) {
        return ret
      }

      return _.fromPairs(_.map(exclusions.split(' '), (ex) => {
        return [ex.trim(), 1]
      }))
    },

    unpackExclusions(exclusions: string[]) {
      return _.keys(exclusions).join(' ')
    },

    genRowKey(tab: string, type: string, idx: number) {
      return `${tab}-${type}-${idx}`
    },

    deleteWAFRow(event: Event) {
      let elem = event.target as HTMLElement
      let rowKey = elem.dataset.curie
      while (elem) {
        if (rowKey) {
          const [tab, type, idx] = rowKey.split('-')
          this.localDoc[tab as ArgsCookiesHeadersType][type as NamesRegexType].splice(Number(idx), 1)
          this.emitDocUpdate()
          break
        }
        elem = elem.parentElement
        rowKey = elem.dataset.curie
      }
    },
  },
})
</script>
