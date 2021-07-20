<template>
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

      <div class="content">
        <hr/>
        <div class="columns">
          <div class="column is-2" v-for="operation in operations" :key="operation">
            <p class="title is-7 is-uppercase">{{ titles[operation] }}</p>
            <hr class="bar" :class="`bar-${operationClassName(operation)}`"/>
            <table class="table is-narrow is-fullwidth">
              <tbody>
              <tr v-for="(tag, idx) in localDoc[operation]" :key="idx">
                <td class="tag-cell"
                    :class=" { 'has-text-danger': duplicateTags[tag], 'tag-crossed': allPrior(operation) }"
                    :title="tagMessage(tag, operation)">
                  {{ tag }}
                </td>
                <td class="is-size-7 width-20px">
                  <a title="remove entry"
                     tabindex="0"
                     class="is-small has-text-grey remove-entry-button"
                     @click="removeTag(operation, idx)"
                     @keypress.space.prevent
                     @keypress.space="removeTag(operation, idx)"
                     @keypress.enter="removeTag(operation, idx)">
                    &ndash;
                  </a>
                </td>
              </tr>
              <tr>
                <td>
                  <tag-autocomplete-input v-if="addNewColName === operation"
                                          ref="tagAutocompleteInput"
                                          :clear-input-after-selection="true"
                                          :selection-type="'single'"
                                          :auto-focus="true"
                                          @keydown.esc="cancelAddNewTag"
                                          @tag-submitted="addNewEntry(operation, $event)">
                  </tag-autocomplete-input>
                </td>
                <td class="is-size-7 width-20px">
                  <a title="add new entry"
                     tabindex="0"
                     class="is-size-7 width-20px is-small has-text-grey add-new-entry-button"
                     @click="openTagInput(operation)"
                     @keypress.space.prevent
                     @keypress.space="openTagInput(operation)"
                     @keypress.enter="openTagInput(operation)">
                    +
                  </a>
                </td>
              </tr>
              </tbody>
            </table>
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
import TagAutocompleteInput from '@/components/TagAutocompleteInput.vue'
import Vue from 'vue'
import {Dictionary} from 'vue-router/types/router'
import {ACLPolicy, ACLPolicyFilter} from '@/types'

export default Vue.extend({
  name: 'ACLEditor',

  components: {
    TagAutocompleteInput,
  },

  props: {
    selectedDoc: Object,
    apiPath: String,
  },

  data() {
    return {
      operations: ['force_deny', 'bypass', 'allow_bot', 'deny_bot', 'allow', 'deny'] as ACLPolicyFilter[],
      titles: DatasetsUtils.titles,
      addNewColName: null,
    }
  },
  computed: {
    localDoc(): ACLPolicy {
      return _.cloneDeep(this.selectedDoc)
    },

    duplicateTags(): Dictionary<string> {
      const doc = this.localDoc
      const allTags = _.concat(doc['force_deny'], doc['bypass'],
          doc['allow_bot'], doc['deny_bot'], doc['allow'], doc['deny'])
      const dupTags = _.filter(allTags, (val, i, iteratee) => _.includes(iteratee, val, i + 1))
      const result = _.fromPairs(_.zip(dupTags, dupTags))
      this.$emit('form-invalid', !!_.size(result))
      return result
    },

  },
  methods: {
    emitDocUpdate() {
      this.$emit('update:selectedDoc', this.localDoc)
    },

    // returns true if tag "all" is set in a higher priority section
    allPrior(self: ACLPolicyFilter): boolean {
      // top priority, skip
      if (self === 'force_deny') {
        return false
      }

      const selfIdx = _.indexOf(this.operations, self)
      const doc = this.localDoc
      const operations = this.operations

      for (let idx = 0; idx < selfIdx; idx++) {
        if (_.indexOf(doc[operations[idx]], 'all') > -1) {
          if (idx === 3) {
            return false
          }
          if (idx === 2) {
            return selfIdx === 3
          }
          return true
        }
      }
    },

    addNewEntry(section: ACLPolicyFilter, entry: string) {
      this.localDoc[section].push(entry)
      this.emitDocUpdate()
    },

    openTagInput(section: ACLPolicyFilter) {
      this.addNewColName = section
    },

    cancelAddNewTag() {
      this.addNewColName = null
    },

    removeTag(section: ACLPolicyFilter, index: number) {
      this.localDoc[section].splice(index, 1)
      this.addNewColName = null
      this.emitDocUpdate()
    },

    operationClassName(operation: ACLPolicyFilter) {
      return operation && operation.replace('_', '-')
    },

    tagMessage(tag: string, operation: ACLPolicyFilter) {
      let message = ''
      if (this.allPrior(operation)) {
        message = '[all] is set in a higher priority section'
      } else if (this.duplicateTags[tag]) {
        message = `[${tag}] is duplicated`
      }
      return message
    },

  },

})
</script>

<style scoped lang="scss">

@import 'node_modules/bulma/sass/utilities/initial-variables.sass';
@import 'node_modules/bulma/sass/utilities/functions.sass';
@import 'node_modules/bulma/sass/utilities/derived-variables.sass';
@import 'node_modules/bulma/sass/helpers/color.sass';

.bar {
  margin: 1rem 0 0.5rem;
}

.bar-force-deny {
  @extend .has-background-danger;
}

.bar-deny-bot {
  @extend .has-background-danger;
}

.bar-deny {
  @extend .has-background-danger;
}

.bar-bypass {
  @extend .has-background-success;
}

.bar-allow {
  @extend .has-background-info;
}

.bar-allow-bot {
  @extend .has-background-info;
}

.tag-crossed {
  @extend .has-text-grey-lighter;
  text-decoration: line-through;
}

::v-deep .tag-input {
  font-size: 0.58rem;
}
</style>
