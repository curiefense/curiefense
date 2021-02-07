<template>
  <section>
    <div class="card">
      <div class="card-content">
        <div class="content">
          <div class="columns columns-divided">
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
                         placeholder="New rate limit rule name"
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
                         placeholder="New rate limit rule name"
                         @change="emitDocUpdate"
                         v-model="localDoc.limit">
                </div>
              </div>
              <div class="field">
                <label class="label is-small">
                  TTL
                </label>
                <div class="control seconds-suffix">
                  <input class="input is-small document-ttl"
                         type="text"
                         placeholder="New rate limit rule name"
                         @change="emitDocUpdate"
                         v-model="localDoc.ttl">
                </div>
              </div>
            </div>
            <div class="column is-8">
              <div class="group-key mb-3">
                <limit-option v-for="(option, index) in localDoc.key"
                              :label="index === 0 ? 'Count by' : ' '"
                              show-remove
                              @remove="removeKey(index)"
                              @change="updateKeyOption"
                              :removable="localDoc.key.length > 1"
                              :index="index"
                              :ignore-attributes="['tags']"
                              :option="generateOption(option)"
                              :key="getOptionTextKey(option, index)"/>
                <a title="Add new option rule"
                   class="is-text is-small is-size-7 ml-3 add-key-button"
                   @click="addKey()">
                  New entry
                </a>
                <p class="has-text-danger is-size-7 ml-3 mt-3 key-invalid"
                   v-if="!keysAreValid">
                  Count-by entries must be unique
                </p>
              </div>
              <div class="group-event mb-3">
                <limit-option use-default-self
                              label="Event"
                              :option.sync="eventOption"
                              :key="eventOption.type + localDoc.id"
                              :ignore-attributes="['tags']"
                              @change="updateEvent"/>
              </div>
              <div class="field">
                <response-action :action.sync="localDoc.action"
                                 @update:action="emitDocUpdate"/>
              </div>
              <div>
                <hr>
                <div class="columns mb-3">
                  <div class="column has-text-danger is-size-7">
                    <p v-if="!includesAreValid"
                       class="include-invalid">
                      Include rule keys must be unique
                    </p>
                    <p v-if="!excludesAreValid"
                       class="exclude-invalid">
                      Exclude rule keys must be unique
                    </p>
                  </div>
                  <div class="column is-narrow">
                    <a title="Add new option rule"
                       class="is-text is-small is-size-7 new-include-exclude-button"
                       @click="newIncludeOrExcludeEntry.visible = !newIncludeOrExcludeEntry.visible">
                      {{ newIncludeOrExcludeEntry.visible ? 'Cancel' : 'New entry' }}
                    </a>
                  </div>
                </div>
                <div>
                  <div v-if="newIncludeOrExcludeEntry.visible"
                       class="new-include-exclude-row has-background-warning-light">
                    <div class="columns">
                      <div class="column is-2">
                        <div class="control select is-small">
                          <select class="include-exclude-select"
                                  v-model="newIncludeOrExcludeEntry.include">
                            <option :value="true">Include</option>
                            <option :value="false">Exclude</option>
                          </select>
                        </div>
                      </div>
                      <div class="column">
                        <div class="control select is-small is-fullwidth">
                          <select class="type-select"
                                  v-model="newIncludeOrExcludeEntry.type">
                            <option value="attrs">Attribute</option>
                            <option value="args">Argument</option>
                            <option value="cookies">Cookie</option>
                            <option value="headers">Header</option>
                          </select>
                        </div>
                      </div>
                      <div class="column">
                        <div v-if="newIncludeOrExcludeEntry.type === 'attrs'"
                             class="control select is-small is-fullwidth">
                          <select class="key-select"
                                  v-model="newIncludeOrExcludeEntry.key">
                            <option value="ip">IP Address</option>
                            <option value="asn">Provider</option>
                            <option value="uri">URI</option>
                            <option value="path">Path</option>
                            <option value="tags">Tags</option>
                            <option value="query">Query</option>
                            <option value="method">Method</option>
                            <option value="company">Company</option>
                            <option value="country">Country</option>
                            <option value="authority">Authority</option>
                          </select>
                        </div>
                        <div v-else class="control">
                          <input v-model="newIncludeOrExcludeEntry.key"
                                 type="text"
                                 class="input is-small key-input">
                        </div>
                      </div>
                      <div class="column">
                        <div class="control has-icons-left">
                          <tag-autocomplete-input v-show="newIncludeOrExcludeEntry.key === 'tags'"
                                                  :initial-tag="newIncludeOrExcludeEntry.value"
                                                  :selection-type="'multiple'"
                                                  @tag-changed="newIncludeOrExcludeEntry.value = $event">
                          </tag-autocomplete-input>
                          <input v-show="newIncludeOrExcludeEntry.key !== 'tags'"
                                 v-model="newIncludeOrExcludeEntry.value"
                                 type="text"
                                 class="input is-small value-input">
                          <span class="icon is-small is-left has-text-grey-light">
                      <i class="fa fa-code"></i>
                    </span>
                        </div>
                      </div>
                      <div class="column is-narrow">
                        <button title="Add new entry"
                                class="button is-light is-small add-button"
                                @click="addIncludeOrExclude">
                          <span class="icon is-small"><i class="fas fa-plus"></i></span>
                        </button>
                      </div>
                    </div>
                  </div>
                  <div v-if="!includes.length && !excludes.length && !newIncludeOrExcludeEntry.visible">
                    <p class="is-size-7 has-text-centered has-text-grey no-include-exclude-message">
                      To limit this rule coverage add <a @click="newIncludeOrExcludeEntry.visible = true">new entry</a>
                    </p>
                  </div>
                  <div class="group-include-exclude">
                    <limit-option v-for="(option, index) in includes"
                                  @change="updateIncludeOption"
                                  @remove="removeIncludeOrExclude(index, true)"
                                  :index="index"
                                  :option="option"
                                  :key="`${option.type}_${option.key}_${index}_inc`"
                                  label="Include"
                                  use-value
                                  show-remove
                                  removable/>
                    <limit-option v-for="(option, index) in excludes"
                                  @change="updateExcludeOption"
                                  @remove="removeIncludeOrExclude(index, false)"
                                  :index="index"
                                  :option="option"
                                  :key="`${option.type}_${option.key}_${index}_exc`"
                                  label="Exclude"
                                  use-value
                                  show-remove
                                  removable/>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script lang="ts">
import _ from 'lodash'
import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import ResponseAction from '@/components/ResponseAction.vue'
import LimitOption, {OptionObject} from '@/components/LimitOption.vue'
import TagAutocompleteInput from '@/components/TagAutocompleteInput.vue'
import Vue from 'vue'
import {LimitOptionType, LimitRuleType, RateLimit} from '@/types'

export default Vue.extend({
  name: 'RateLimits',
  props: {
    selectedDoc: Object,
    apiPath: String,
  },
  components: {
    ResponseAction,
    LimitOption,
    TagAutocompleteInput,
  },
  data() {
    return {
      includes: [],
      excludes: [],
      includesAreValid: true,
      excludesAreValid: true,
      keysAreValid: true,
      newIncludeOrExcludeEntry: {
        visible: false,
        include: true,
        type: 'attrs',
        key: 'ip',
        value: '',
      },
    }
  },
  computed: {
    localDoc(): RateLimit {
      return JSON.parse(JSON.stringify(this.selectedDoc))
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

    addIncludeOrExclude() {
      const arr = this.newIncludeOrExcludeEntry.include ? this.includes : this.excludes
      const {
        type,
        key,
        value,
      } = this.newIncludeOrExcludeEntry
      arr.push({type, key, value})
      this.checkIncludeOrExcludeValidity(this.newIncludeOrExcludeEntry.include)
      this.newIncludeOrExcludeEntry.type = 'attrs'
      this.newIncludeOrExcludeEntry.key = 'ip'
      this.newIncludeOrExcludeEntry.value = ''
      this.newIncludeOrExcludeEntry.visible = false
    },

    updateIncludeOrExcludeOption(option: OptionObject, index: number, include: boolean) {
      const arr = include ? this.includes : this.excludes
      arr.splice(index, 1, option)
      this.checkIncludeOrExcludeValidity(include)
    },

    removeIncludeOrExclude(index: number, include: boolean) {
      const options = (include ? this.includes : this.excludes)
      options.splice(index, 1)
      this.checkIncludeOrExcludeValidity(include)
    },

    checkIncludeOrExcludeValidity(include: boolean) {
      const docKey = include ? 'includesAreValid' : 'excludesAreValid'
      const arr = include ? this.includes : this.excludes
      const keysToCheck = _.countBy(arr, (item) => `${item.type}_${item.key}`)
      this[docKey] = true
      for (const key of Object.keys(keysToCheck)) {
        if (keysToCheck[key] > 1) {
          this[docKey] = false
          break
        }
      }
      return this[docKey]
    },

    updateIncludeOption(option: OptionObject, index: number) {
      this.updateIncludeOrExcludeOption(option, index, true)
    },

    updateExcludeOption(option: OptionObject, index: number) {
      this.updateIncludeOrExcludeOption(option, index, false)
    },

    convertIncludesOrExcludes(obj: RateLimit['include'] | RateLimit['exclude']) {
      if (!obj) {
        return []
      }
      return Object.keys(obj).reduce((acc, type) => {
        const options = Object.keys(obj[type as LimitRuleType]).map((key) => ({
          type,
          key,
          value: obj[type as LimitRuleType][key],
        }))
        return [...acc, ...options]
      }, [])
    },

    updateEvent(option: OptionObject) {
      this.eventOption = {[option.type]: option.key}
    },

    normalizeIncludesOrExcludes(value: OptionObject[], include: boolean) {
      // converting includes/excludes from component arrays to selectedDoc objects
      const includeOrExcludeKey = include ? 'include' : 'exclude'
      const LimitRulesTypes = DatasetsUtils.LimitRulesTypes
      if (!this.localDoc[includeOrExcludeKey]) {
        this.$set(this.localDoc, includeOrExcludeKey, {})
      }
      Object.keys(LimitRulesTypes).forEach((t) => {
        this.$set(this.localDoc[includeOrExcludeKey], t, {})
      })
      value.forEach((el: OptionObject) => {
        this.$set(this.localDoc[includeOrExcludeKey][el.type], el.key, el.value)
      })
      if (!_.isEqual(this.localDoc, this.selectedDoc)) {
        this.emitDocUpdate()
      }
    },
  },
  mounted() {
    this.checkKeysValidity()
    this.checkIncludeOrExcludeValidity(true)
    this.checkIncludeOrExcludeValidity(false)
  },
  watch: {
    selectedDoc: {
      handler: function(newValue) {
        this.includes = this.convertIncludesOrExcludes(newValue.include)
        this.excludes = this.convertIncludesOrExcludes(newValue.exclude)
        this.$forceUpdate()
      },
      immediate: true,
      deep: true,
    },
    includes(newValue) {
      this.normalizeIncludesOrExcludes(newValue, true)
    },
    excludes(newValue) {
      this.normalizeIncludesOrExcludes(newValue, false)
    },
  },
})
</script>

<style scoped lang="scss">

@import 'src/assets/styles/main.scss';
@import 'node_modules/bulma/sass/helpers/typography.sass';

.form-label {
  padding-top: 0.25rem;
}

table.is-borderless td, table.is-borderless th {
  border: 0;
}

table.inner-table td, table.inner-table th {
  border: 0;
  padding-left: 0;
  padding-right: 0;
}

.seconds-suffix {
  input {
    padding-right: 60px;
  }
}

.seconds-suffix::after {
  @extend .is-size-7;
  @extend .suffix;
  content: 'seconds';
}

</style>
