<template>
  <section>
    <div class="card">
      <div class="card-content">
        <div class="media">
          <div class="media-content">
            <div class="columns">
              <div class="column is-5">
                <p class="subtitle is-6 has-text-grey" title="Document ID">{{ selectedDoc.id }}</p>
              </div>
              <div class="column"></div>
            </div>
          </div>
        </div>
        <div class="content">
          <div class="columns">
            <div class="column is-4 col-pr col-25">
              <div class="field">
                <label class="label is-small">Name</label>
                <div class="control">
                  <input class="input is-small" type="text" placeholder="Document name" v-model="selectedDoc.name">
                </div>
              </div>
            </div>
            <div class="column is-4 col-pr col-45">
              <div class="field">
                <label class="label is-small">Description</label>
                <div class="control">
                  <input class="input is-small" type="text" placeholder="New rate limit rule name" v-model="selectedDoc.description">
                </div>
              </div>
            </div>
            <div class="column is-2 col-pr col-15">
              <div class="field">
                <label class="label is-small">Threshold</label>
                <div class="control">
                  <input class="input is-small" type="text" placeholder="New rate limit rule name" v-model="selectedDoc.limit">
                </div>
              </div>
            </div>
            <div class="column is-2 col-15">
              <div class="field">
                <label class="label is-small">TTL</label>
                <div class="control">
                  <input class="input is-small" type="text" placeholder="New rate limit rule name" v-model="selectedDoc.ttl">
                </div>
              </div>
            </div>
          </div>
          <hr />
          <div class="columns">
            <div class="column is-6">
              <div class="group-key">
                <div class="columns">
                  <div class="column is-3">
                    <label class="label is-small has-text-left form-label">Count by</label>
                  </div>
                  <div class="column">
                    <div class="group-key">
                      <limit-option
                        v-for="(option, idx) in selectedDoc.key"
                        show-remove
                        @remove="removeKey(idx)"
                        @change="updateKeyOption"
                        :removable="selectedDoc.key.length > 1"
                        :index="idx"
                        :option="generateOption(option)"
                        :key="getOptionTextKey(option, idx)"
                      />
                      <a
                        title="Add new option rule"
                        class="is-text is-small is-size-7 ml-4"
                        @click="addKey()"
                      >
                        New entry
                      </a>
                      <br>
                      <p class="has-text-danger pl-3 mt-3 is-size-7" v-if="!keysAreValid">
                        Count-by entries must be unique
                      </p>
                    </div>
                  </div>
                </div>
              </div>
              <div class="group-event">
                <div class="columns" style="margin-bottom: .75rem">
                  <div class="column is-3">
                    <label class="label is-small has-text-left form-label">Event</label>
                  </div>
                  <div class="column is-9">
                    <div class="group-event">
                      <limit-option
                        use-default-self
                        :option.sync="eventOption"
                        :key="eventOption.type + selectedDoc.id"
                        @change="updateEvent"
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div class="column ml-4" style="padding: 0">
              <div class="card">
                <limit-action
                  :action.sync="selectedDoc.action"
                />
              </div>
            </div>
          </div>
        </div>
        <hr />
        <div class="columns mb-0">
          <div class="column">
            <p class="has-text-danger is-size-7" v-if="!includesAreValid" style="padding-left: 33.3%">
              Include rule keys must be unique
            </p>
            <p class="has-text-danger is-size-7" v-if="!excludesAreValid" style="padding-left: 33.3%">
              Exclude rule keys must be unique
            </p>
          </div>
          <div class="column">
            <a
              title="Add new option rule"
              class="is-text is-small is-size-7 ml-4 is-pulled-right"
              @click="newIncludeOrExcludeEntry.visible = !newIncludeOrExcludeEntry.visible"
            >
              {{ newIncludeOrExcludeEntry.visible ? 'Cancel' : 'New entry'}}
            </a>
          </div>
        </div>
        <div class="columns is-gapless" style="padding: 0.75rem; margin-bottom: .75rem">
          <div class="column">
            <div v-if="newIncludeOrExcludeEntry.visible" class="new-entry-row" style="background-color: rgb(255, 251, 235)">
              <div class="columns">
                <div class="column has-text-right">
                  <div class="control select is-small">
                    <select v-model="newIncludeOrExcludeEntry.include">
                      <option :value="true">Include</option>
                      <option :value="false">Exclude</option>
                    </select>
                  </div>
                </div>
                <div class="column">
                  <div class="control select is-small is-fullwidth">
                    <select v-model="newIncludeOrExcludeEntry.type">
                      <option value="attrs">Attribute</option>
                      <option value="args">Argument</option>
                      <option value="cookies">Cookie</option>
                      <option value="headers">Header</option>
                    </select>
                  </div>
                </div>
                <div class="column">
                  <div v-if="newIncludeOrExcludeEntry.type === 'attrs'" class="control select is-small is-fullwidth">
                    <select v-model="newIncludeOrExcludeEntry.key">
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
                    <input v-model="newIncludeOrExcludeEntry.key" type="text" class="input is-small">
                  </div>
                </div>
                <div class="column">
                  <div class="control has-icons-left">
                    <tag-autocomplete-input v-show="newIncludeOrExcludeEntry.key === 'tags'"
                                            :initialTag="newIncludeOrExcludeEntry.value"
                                            :selectionType="'multiple'"
                                            @tagChanged="newIncludeOrExcludeEntry.value = $event">
                    </tag-autocomplete-input>
                    <input v-show="newIncludeOrExcludeEntry.key !== 'tags'"
                           v-model="newIncludeOrExcludeEntry.value" type="text" class="input is-small">
                    <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
                  </div>
                </div>
                <div class="column is-1">
                  <button title="Add new entry" class="button is-light is-small" @click="addIncludeOrExclude">
                    <span class="icon is-small"><i class="fas fa-plus"></i></span>
                  </button>
                </div>
              </div>
            </div>
            <div v-if="!includes.length && !excludes.length && !newIncludeOrExcludeEntry.visible">
              <p class="is-size-7 has-text-centered has-text-grey">
                To limit this rule coverage add <a @click="newIncludeOrExcludeEntry.visible = true">new entry</a>
              </p>
            </div>
            <div class="group-include">
              <limit-option
                v-for="(option, idx) in includes"
                @change="updateIncludeOption"
                @remove="removeIncludeOrExclude(idx, true)"
                :index="idx"
                :option="option"
                :key="`${option.type}_${option.key}_${idx}_inc`"
                label="Include"
                use-value
                show-remove
                removable
              />
              <limit-option
                v-for="(option, idx) in excludes"
                @change="updateExcludeOption"
                @remove="removeIncludeOrExclude(idx, false)"
                :index="idx"
                :option="option"
                :key="`${option.type}_${option.key}_${idx}_exc`"
                label="Exclude"
                use-value
                show-remove
                removable
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script>
import LimitAction from '@/components/LimitAction.vue'
import LimitOption from '@/components/LimitOption.vue'
import TagAutocompleteInput from '@/components/TagAutocompleteInput'

export default {
  name: 'RateLimits',
  props: {
    selectedDoc: Object,
    apiPath: String
  },
  components: {
    LimitAction,
    LimitOption,
    TagAutocompleteInput
  },
  data() {
    return {
      addInclude: true,
      includes: this.convertIncludesOrExcludes(this.selectedDoc.include),
      excludes: this.convertIncludesOrExcludes(this.selectedDoc.exclude),
      includesAreValid: true,
      excludesAreValid: true,
      keysAreValid: true,
      newIncludeOrExcludeEntry: {
        visible: false,
        include: true,
        type: 'attrs',
        key: 'ip',
        value: ''
      }
    }
  },
  computed: {
    isFormValid() {
      return this.includesAreValid && this.excludesAreValid && this.keysAreValid
    },
    eventOption: {
      get() {
        return this.generateOption(this.selectedDoc.pairwith)
      },
      set(value) {
        this.$set(this.selectedDoc, 'pairwith', value)
      }
    }
  },
  methods: {
    getOptionTextKey(option, idx) {
      const [type] = Object.keys(option)
      return `${this.selectedDoc.id}_${type}_${idx}`
    },
    generateOption(data, optionType = null) {
      if (!data) {
        return {}
      }
      const [firstObjectKey] = Object.keys(data)
      const type = optionType ? optionType : firstObjectKey
      const key = optionType ? firstObjectKey : (data[firstObjectKey] || null)
      const value = optionType ? data[firstObjectKey] : null
      return { type, key, value }
    },
    addKey() {
      this.selectedDoc.key.push({ attrs: 'ip' })
      this.checkKeysValidity()
    },
    removeKey(idx) {
      if (this.selectedDoc.key.length > 1) {
        this.selectedDoc.key.splice(idx, 1)
      }
      this.checkKeysValidity()
    },
    updateKeyOption(option, index = 0) {
      this.selectedDoc.key.splice(index, 1, {
        [option.type]: option.key
      })
      this.checkKeysValidity()
    },
    checkKeysValidity() {
      const keysToCheck = this.ld.countBy(this.selectedDoc.key, item => {
        const key = Object.keys(item)[0]
        return `${key}_${item[key]}`
      })
      this.keysAreValid = true
      for (const key of Object.keys(keysToCheck)) {
        if (keysToCheck[key] > 1) {
          this.keysAreValid = false
          break
        }
      }
      return this.keysAreValid
    },
    addIncludeOrExclude() {
      const arr = this.newIncludeOrExcludeEntry.include ? this.includes : this.excludes
      const {
        type = 'attrs',
        key = 'ip',
        value = ''
      } = this.newIncludeOrExcludeEntry
      arr.push({ type, key, value })
      this.checkIncludeOrExcludeValidity(this.newIncludeOrExcludeEntry.include)
      this.newIncludeOrExcludeEntry.type = 'attrs'
      this.newIncludeOrExcludeEntry.key = 'ip'
      this.newIncludeOrExcludeEntry.value = ''
      this.newIncludeOrExcludeEntry.visible = false
    },
    updateIncludeOrExcludeOption(option, index = 0, include = true) {
      const arr = include ? this.includes : this.excludes
      arr.splice(index, 1, option)
      this.checkIncludeOrExcludeValidity(include)
    },
    removeIncludeOrExclude(index = 0, include = true) {
      const options = (include ? this.includes : this.excludes)
      options.splice(index, 1)
      this.checkIncludeOrExcludeValidity(include)
    },
    checkIncludeOrExcludeValidity(include = true) {
      const docKey = include ? 'includesAreValid' : 'excludesAreValid'
      const arr = include ? this.includes : this.excludes
      const keysToCheck = this.ld.countBy(arr, item => `${item.type}_${item.key}`)
      this[docKey] = true
      for (const key of Object.keys(keysToCheck)) {
        if (keysToCheck[key] > 1) {
          this[docKey] = false
          break
        }
      }
      return this[docKey]
    },
    updateIncludeOption(option, index) {
      this.updateIncludeOrExcludeOption(option, index, true)
    },
    updateExcludeOption(option, index) {
      this.updateIncludeOrExcludeOption(option, index, false)
    },
    convertIncludesOrExcludes(obj) {
      if (!obj) {
        return []
      }
      return Object.keys(obj).reduce((acc, type) => {
        const options = Object.keys(obj[type]).map(key => ({
          type,
          key,
          value: obj[type][key]
        }))
        return [...acc, ...options]
      }, [])
    },
    updateEvent(option) {
      this.eventOption = { [option.type]: option.key }
    },
    normalizeDocAction() {
      // adding necessary fields to selectedDoc.action field
      if (!this.selectedDoc.action) {
        this.$set(this.selectedDoc, 'action', {})
      }
      if (!this.selectedDoc.action.params) {
        this.$set(this.selectedDoc.action, 'params', {})
      }
      if (!this.selectedDoc.action.params.action) {
        this.$set(this.selectedDoc.action.params, 'action', { type: 'default', params: {} })
      }
    },
    normalizeIncludesOrExcludes(value, include = true) {
      // converting includes/excludes from component arrays to selectedDoc objects
      const includeOrExcludeKey = include ? 'include' : 'exclude'
      const { LimitRulesTypes } = this.$root.dsutils
      if (!this.selectedDoc[includeOrExcludeKey]) {
        this.$set(this.selectedDoc, includeOrExcludeKey, {})
      }
      Object.keys(LimitRulesTypes).forEach(t => {
        this.$set(this.selectedDoc[includeOrExcludeKey], t, {})
      })
      value.forEach(el => {
        this.$set(this.selectedDoc[includeOrExcludeKey][el.type], el.key, el.value)
      })
    }
  },
  mounted() {
    this.normalizeDocAction()
    this.checkKeysValidity()
    this.checkIncludeOrExcludeValidity(true)
    this.checkIncludeOrExcludeValidity(false)
  },
  watch: {
    selectedDoc(newValue) {
      this.includes = this.convertIncludesOrExcludes(newValue.include)
      this.excludes = this.convertIncludesOrExcludes(newValue.exclude)
      this.normalizeDocAction()
      this.$forceUpdate()
    },
    includes(newValue) {
      this.normalizeIncludesOrExcludes(newValue, true)
    },
    excludes(newValue) {
      this.normalizeIncludesOrExcludes(newValue, false)
    }
  }
}
</script>

<style scoped>

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

</style>
