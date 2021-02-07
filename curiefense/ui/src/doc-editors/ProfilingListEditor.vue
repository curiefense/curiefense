<template>
  <div class="card">
    <div class="card-content">
      <div class="content">
        <div class="columns columns-divided">
          <div class="column is-3">
            <div class="field">
              <label class="label is-small">Name
                <span class="has-text-grey is-pulled-right"
                      title="Rule id">
                  {{ localDoc.id }}
                </span>
              </label>
              <div class="control">
                <input class="input is-small"
                       title="List name"
                       placeholder="List name"
                       @change="emitDocUpdate"
                       v-model="localDoc.name"
                       :readonly="readonly"/>
              </div>
              <p class="subtitle is-7 has-text-grey">
                {{ sectionsEntriesDisplay }}
              </p>
            </div>
            <div class="field">
              <label class="checkbox is-size-7">
                <input type="checkbox"
                       :readonly="readonly"
                       :disabled="readonly"
                       @change="emitDocUpdate"
                       v-model="localDoc.active">
                Active
              </label>
            </div>
            <div class="field">

              <div class="control" v-if="editable">
                <label class="label is-small">Sections Relation</label>
                <div class="tags has-addons">
                  <span class="tag pointer"
                        :class="localDoc.rule.relation === 'AND' ? 'is-info xis-light is-selected' : ''"
                        @click="setRuleRelation('AND')">
                    AND
                  </span>
                  <span class="tag pointer"
                        :class="localDoc.rule.relation === 'OR' ? 'is-info xis-light is-selected' : ''"
                        @click="setRuleRelation('OR')">
                    OR
                  </span>
                </div>
              </div>
            </div>
            <div class="field">
              <label class="label is-small">Tags</label>
              <div class="control">
                <tag-autocomplete-input :initial-tag="selectedDocTags"
                                        :selection-type="'multiple'"
                                        @tag-changed="selectedDocTags = $event">
                </tag-autocomplete-input>
              </div>
            </div>
            <div class="field">
              <a v-if="localDoc && localDoc.source && localDoc.source.indexOf('http') === 0"
                 class="is-small has-text-grey is-size-7 is-pulled-right"
                 @click="fetchList">
                update now
              </a>
              <label class="label is-small">Source</label>
              <div class="control">
                <input class="input is-small"
                       title="List source"
                       placeholder="List source"
                       @change="emitDocUpdate"
                       v-model="localDoc.source"
                       :readonly="readonly"/>
              </div>
            </div>
            <div class="field">
              <response-action :action.sync="localDoc.action"
                               :ignore="['ban']"
                               @update:action="emitDocUpdate"
                               label-separated-line
                               wide-columns
                               is-single-input-column/>
            </div>
            <div class="field">
              <label class="label is-small">Notes</label>
              <div class="control">
                <textarea class="is-small textarea"
                          title="Notes"
                          @change="emitDocUpdate"
                          v-model="localDoc.notes"
                          rows="2"
                          :readonly="readonly"></textarea>
              </div>
            </div>
            <div class="pt-6">
              <div class="field" v-if="editable">
                <div class="control is-expanded">
                  <button class="button is-small has-text-danger-dark"
                          title="Remove all sections"
                          @click="removeAllSections">Clear all sections
                  </button>
                </div>
              </div>
              <p class="help">updated @ {{ localDoc.mdate }}</p>
            </div>

          </div>
          <div class="column is-9">
            <entries-relation-list :rule="localDoc.rule"
                                   :editable="editable"
                                   @update="updateRule($event)">
            </entries-relation-list>
          </div>
        </div>
        <span class="is-family-monospace has-text-grey-lighter">{{ apiPath }}</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import _ from 'lodash'
import RequestsUtils from '@/assets/RequestsUtils.ts'
import ResponseAction from '@/components/ResponseAction.vue'
import TagAutocompleteInput from '@/components/TagAutocompleteInput.vue'
import EntriesRelationList from '@/components/EntriesRelationList.vue'
import Vue from 'vue'
import {Category, Relation, TagRule, TagRuleSection, TagRuleSectionEntry} from '@/types'
import {AxiosResponse} from 'axios'

export default Vue.extend({
  name: 'ProfilingListEditor',

  components: {
    ResponseAction,
    EntriesRelationList,
    TagAutocompleteInput,
  },

  props: {
    selectedDoc: Object,
    apiPath: String,
  },

  computed: {
    sectionsEntriesDisplay(): string {
      const sectionsCounter = (this.localDoc?.rule?.sections?.length !== 1) ? 'sections' : 'section'
      const entriesCounter = (this.localDocTotalEntries !== 1) ? 'entries' : 'entry'
      const sectionsLength = this.localDoc?.rule?.sections?.length
      return `${sectionsLength} ${sectionsCounter}\t|\t${this.localDocTotalEntries} ${entriesCounter}`
    },

    readonly(): boolean {
      return this.localDoc.source === 'reblaze-managed'
    },

    editable(): boolean {
      return this.localDoc.source === 'self-managed'
    },

    selectedDocTags: {
      get: function(): string {
        if (this.localDoc.tags) {
          return this.localDoc.tags.join(' ')
        }
        return ''
      },
      set: function(): void {
        _.debounce((tags: any) => {
          (this as any).localDoc.tags = _.map(tags.split(' '), (tag) => {
            return tag.trim()
          })
        }, 500)
      },
    },

    localDoc(): TagRule {
      return JSON.parse(JSON.stringify(this.selectedDoc))
    },

    localDocTotalEntries(): number {
      let totalEntries = 0
      if (this.localDoc?.rule?.sections?.length) {
        totalEntries = _.sumBy(this.localDoc.rule.sections, (section: TagRuleSection) => {
          return section.entries?.length
        })
      }
      return totalEntries
    },

  },

  methods: {
    emitDocUpdate() {
      this.$emit('update:selectedDoc', this.localDoc)
    },

    setRuleRelation(relation: Relation) {
      if (relation) {
        this.localDoc.rule.relation = relation
      } else {
        this.localDoc.rule.relation = (this.localDoc.rule.relation === 'AND') ? 'OR' : 'AND'
      }

      this.emitDocUpdate()
    },

    removeAllSections() {
      this.localDoc.rule.sections.splice(0, this.localDoc.rule.sections.length)
      this.emitDocUpdate()
    },

    tryMatch(data: any, regex: RegExp, type: Category): TagRuleSectionEntry[] {
      let matches
      const entries = []
      matches = regex.exec(data)
      while (matches) {
        const entry: TagRuleSectionEntry = [type, matches[1], null]
        if (matches.length > 2) {
          entry[2] = (matches.slice(-1)[0] || '').slice(0, 128)
        }
        entries.push(entry)
        matches = regex.exec(data)
      }
      return entries
    },

    fetchList() {
      const lineMatchingIP =
          /^((((\d{1,3})\.){3}\d{1,3}(\/\d{1,2}))|([0-9a-f]+:+){1,8}([0-9a-f]+)?(\/\d{1,3})?)\s+([#;/].+)/gm
      const lineMatchingASN = /(as\d{3,6})((\s+)?([#;/?].+))?/gmi
      const singleIP = /^((((\d{1,3})\.){3}\d{1,3}(\/\d{1,2}))|([0-9a-f]+:+){1,8}([0-9a-f]+)?(\/\d{1,3})?)$/
      const singleASN = /(as\d{3,6})/i
      // try every node / element of String type with the regex.
      const objectParser = (data: any, store: TagRuleSectionEntry[]) => {
        _.each(data, (item) => {
          if (_.isArray(item) || _.isObject(item)) {
            objectParser(item, store)
          }
          if (_.isString(item)) {
            if (singleIP.test(item)) {
              store.push(['ip', item, null])
            } else if (singleASN.test(item)) {
              store.push(['asn', item, null])
            }
          }
        })
      }
      const url = this.localDoc.source
      RequestsUtils.sendRequest('GET', `tools/fetch?url=${url}`).then((response: AxiosResponse) => {
        const data = response.data
        let entries: TagRuleSectionEntry[]
        entries = this.tryMatch(data, lineMatchingIP, 'ip')
        if (entries.length === 0) {
          entries = this.tryMatch(data, lineMatchingASN, 'asn')
        }
        if (entries.length === 0) {
          try {
            objectParser(data, entries)
          } catch (e) {
            console.log(e)
          }
        }
        if (entries.length > 0) {
          const newSection: TagRuleSection = {
            relation: 'OR',
            entries: entries,
          }
          this.localDoc.rule = {
            'sections': [
              newSection,
            ],
            'relation': 'OR',
          }
          this.localDoc.mdate = (new Date).toISOString()
          this.emitDocUpdate()
        }
      })
    },

    updateRule(rule: TagRule['rule']) {
      this.localDoc.rule = rule
      this.emitDocUpdate()
    },
  },

})
</script>

<style scoped>
.pointer {
  cursor: pointer;
}
</style>
