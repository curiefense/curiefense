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
                    {{ selectedDoc.id }}
                  </span>
                  </label>
                  <div class="control">
                    <input class="input is-small document-name"
                           placeholder="Document name"
                           v-model="selectedDoc.name"/>
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
                         v-model.number="selectedDoc.max_header_length"/>
                </td>
                <td>
                  <input required
                         class="input is-small max-cookie-length-input"
                         type="number"
                         v-model.number="selectedDoc.max_cookie_length"/>
                </td>
                <td>
                  <input required
                         class="input is-small max-arg-length-input"
                         type="number"
                         v-model.number="selectedDoc.max_arg_length"/>
                </td>
              </tr>
              <tr>
                <td>Max Count</td>
                <td>
                  <input required
                         class="input is-small max-headers-count-input"
                         type="number"
                         v-model.number="selectedDoc.max_headers_count"/>
                </td>
                <td>
                  <input required
                         class="input is-small max-cookies-count-input"
                         type="number"
                         v-model.number="selectedDoc.max_cookies_count"/>
                </td>
                <td>
                  <input required
                         class="input is-small max-args-count-input"
                         type="number"
                         v-model.number="selectedDoc.max_args_count"/>
                </td>
              </tr>
              </tbody>
            </table>
          </div>
          <div class="tile is-3">
            <div class="card">
              <div class="card-content">
                <label class="checkbox">
                  <input type="checkbox" v-model="selectedDoc.ignore_alphanum">
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
                        <li :class=" tab === 'headers' ? 'is-active' : '' "><a @click='tab="headers"'>Headers</a></li>
                        <li :class=" tab === 'cookies' ? 'is-active' : '' "><a @click='tab="cookies"'>Cookies</a></li>
                        <li :class=" tab === 'args' ? 'is-active' : '' "><a @click='tab="args"'>Arguments</a></li>
                      </ul>
                    </div>
                  </td>
                </tr>
                <tr>
                  <td>
                    <table class="table is-fullwidth is-hoverable" v-if="selectedDoc && selectedDoc[tab]">
                      <thead>
                      <tr>
                        <th class="has-text-centered">Parameter</th>
                        <th class="has-text-centered">Matching Value</th>
                        <th class="has-text-centered">Restrict?</th>
                        <th class="has-text-centered">Mask?</th>
                        <th class="has-text-centered">Exclude Sig</th>
                        <th class="has-text-centered">
                          <a v-show="newWAFLine !== tab"
                             class="has-text-grey-dark is-small" title="Add new parameter"
                             @click="newWAFLine = tab; newEntry = {...defaultNewEntry}">
                            <span class="icon is-small"><i class="fas fa-plus"></i></span>
                          </a>
                          <a v-show="newWAFLine === tab"
                             class="has-text-grey-dark is-small" title="Cancel adding new parameter"
                             @click="newWAFLine = null">
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
                                      <select v-model="newEntry.type">
                                        <option value="names">
                                          {{ dsutils.Titles.names }}
                                        </option>
                                        <option value="regex">
                                          {{ dsutils.Titles.regex }}
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
                                             :title="dsutils.Titles.names">
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
                                   :title="dsutils.Titles.regex"/>
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
                          <serialized-input :placeholder="'comma separated sig IDs'" :value="newEntry.exclusions"
                                            :get-function="unpackExclusions" :set-function="packExclusions"
                                            @blur="newEntry.exclusions = $event"></serialized-input>
                        </td>
                        <td class="has-text-centered">
                          <button title="Add new parameter" class="button is-light is-small" @click="addNewParameter">
                            <span class="icon is-small"><i class="fas fa-plus fa-xs"></i></span>
                          </button>
                        </td>

                      </tr>
                      <tr v-for="(entry, idx) in selectedDoc[tab].names" :key="gen_row_key(tab, 'names', idx)">
                        <td>
                          <div class="field">
                            <p class="control has-icons-left">
                              <input required class="input is-small" type="text" v-model="entry.key"
                                     :title="dsutils.Titles.names">
                              <span class="icon is-small is-left has-text-grey">
                                  <i class="fas fa-font"></i>
                                </span>
                            </p>
                          </div>
                        <td>
                          <p class="control has-icons-left">
                            <input required class="input is-small" type="text" v-model="entry.reg"
                                   :title="dsutils.Titles.regex"/>
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
                          <serialized-input :placeholder="'comma separated sig IDs'" :value="entry.exclusions"
                                            :get-function="unpackExclusions" :set-function="packExclusions"
                                            @blur="entry.exclusions = $event"></serialized-input>
                        </td>
                        <td class="has-text-centered">
                          <button title="Delete entry"
                                  :data-curie="gen_row_key(tab, 'names', idx)"
                                  @click="deleteWAFRow"

                                  class="button is-light is-small">
                              <span class="icon is-small"
                              ><i class="fas fa-trash fa-xs"></i></span>
                          </button>
                        </td>
                      </tr>
                      <tr v-for="(entry, idx) in selectedDoc[tab].regex" :key="gen_row_key(tab, 'regex', idx)">
                        <td>
                          <div class="field">
                            <p class="control has-icons-left">
                              <input required class="input is-small" type="text" v-model="entry.key"
                                     :title="dsutils.Titles.regex">
                              <span class="icon is-small is-left has-text-grey">
                                  <i class="fas fa-code"></i>
                                </span>
                            </p>
                          </div>
                        </td>
                        <td>
                          <p class="control has-icons-left">
                            <input required class="input is-small" type="text" v-model="entry.reg"
                                   :title="dsutils.Titles.regex"/>
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
                          <serialized-input :placeholder="'comma separated sig IDs'" :value="entry.exclusions"
                                            :get-function="unpackExclusions" :set-function="packExclusions"
                                            @blur="entry.exclusions = $event"></serialized-input>
                        </td>
                        <td class="has-text-centered">
                          <button
                              :data-curie="gen_row_key(tab, 'regex', idx)"
                              @click="deleteWAFRow"

                              title="Delete entry" class="button is-light is-small">
                              <span class="icon is-small"

                              ><i class="fas fa-trash fa-xs"></i></span>
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

<script>

import SerializedInput from '@/components/SerializedInput'

export default {
  name: 'WAFEditor',
  components: {SerializedInput},
  props: {
    selectedDoc: Object,
    apiPath: String
  },

  data() {
    return {
      tab: 'args',
      newWAFLine: null,
      newEntry: null,
      defaultNewEntry: {
        type: 'names',
        key: null,
        reg: null,
        restrict: false,
        mask: false,
        exclusions: null
      },
    }
  },
  computed: {},

  methods: {

    addNewParameter() {
      let newEntry = this.ld.cloneDeep(this.newEntry)
      this.newEntry = this.newWAFLine = null
      let type = newEntry.type
      delete newEntry.type
      this.selectedDoc[this.tab][type].unshift(newEntry)
    },

    packExclusions(exclusions) {
      let ret = {}
      if (this.ld.size(exclusions) === 0 || !exclusions) {
        return ret
      }

      return this.ld.fromPairs(this.ld.map(exclusions.split(','), (ex) => {
        return [ex.trim(), 1]
      }))

    },

    unpackExclusions(exclusions) {
      return this.ld.keys(exclusions).join(', ')
    },

    gen_row_key(tab, type, idx) {
      return `${tab}-${type}-${idx}`
    },

    deleteWAFRow(event) {
      let elem = event.target
      let row_key = elem.dataset.curie
      while (elem) {
        if (row_key) {
          let [tab, type, idx] = row_key.split('-')
          this.selectedDoc[tab][type].splice(idx, 1)
          break
        }
        elem = elem.parentElement
        row_key = elem.dataset.curie
      }
    }
  }
}
</script>
