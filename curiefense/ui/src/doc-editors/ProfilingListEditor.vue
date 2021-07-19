<template>
  <div class="card">
    <div class="card-content">
      <div class="content">
        <div class="columns columns-divided">
          <div class="column is-3">
            <div class="field">
              <label class="label is-small">
                Name
                <span class="has-text-grey is-pulled-right document-id"
                      title="Rule id">
                  {{ localDoc.id }}
                </span>
              </label>
              <div class="control">
                <input class="input is-small document-name"
                       title="List name"
                       placeholder="List name"
                       @change="emitDocUpdate"
                       v-model="localDoc.name"
                       :readonly="readonly"/>
              </div>
              <p class="subtitle is-7 has-text-grey sections-entries-display">
                {{ sectionsEntriesDisplay }}
              </p>
            </div>
            <div class="field">
              <label class="checkbox is-size-7">
                <input type="checkbox"
                       class="document-active"
                       :readonly="readonly"
                       :disabled="readonly"
                       @change="emitDocUpdate"
                       v-model="localDoc.active">
                Active
              </label>
            </div>
            <div class="field">
              <div class="control"
                   v-if="editable">
                <label class="label is-small">Sections Relation</label>
                <div class="tags has-addons mb-0 document-sections-relation"
                     tabindex="0"
                     @keypress.space.prevent
                     @keypress.space="toggleRuleRelation()"
                     @keypress.enter="toggleRuleRelation()">
                  <span class="tag pointer mb-0"
                        :class="localDoc.rule.relation === 'AND' ? 'is-info xis-light is-selected' : ''"
                        @click="setRuleRelation('AND')">
                    AND
                  </span>
                  <span class="tag pointer mb-0"
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
                 class="is-small has-text-grey is-size-7 is-pulled-right update-now-button"
                 tabindex="0"
                 @click="fetchList"
                 @keypress.space.prevent
                 @keypress.space="fetchList"
                 @keypress.enter="fetchList">
                update now
              </a>
              <label class="label is-small">Source</label>
              <div class="control">
                <input class="input is-small document-source"
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
                               is-single-input-column/>
            </div>
            <div class="field">
              <label class="label is-small">Notes</label>
              <div class="control">
                <textarea class="is-small textarea document-notes"
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
                  <button class="button is-small has-text-danger-dark remove-all-sections-button"
                          title="Remove all sections"
                          @click="removeAllSections">
                    Clear all sections
                  </button>
                </div>
              </div>
              <p class="help"
                 :title="fullFormattedModifiedDate">
                updated @ {{ formattedModifiedDate }}
              </p>
            </div>

          </div>
          <div class="column is-9">
            <entries-relation-list :rule.sync="localDoc.rule"
                                   :editable="editable"
                                   @update:rule="emitDocUpdate"
                                   @invalid="emitFormInvalid">
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
import DateTimeUtils from '@/assets/DateTimeUtils'

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
    docs: Array,
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
        if (this.localDoc.tags && this.localDoc.tags.length > 0) {
          return this.localDoc.tags.join(' ')
        }
        return ''
      },
      set: function(tags: string): void {
        this.localDoc.tags = tags.length > 0 ? _.map(tags.split(' '), (tag) => {
          return tag.trim()
        }) : []
        this.emitDocUpdate()
      },
    },

    localDoc(): TagRule {
      return _.cloneDeep(this.selectedDoc)
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

    formattedModifiedDate(): string {
      return DateTimeUtils.isoToNowCuriefenseFormat(this.localDoc?.mdate)
    },

    fullFormattedModifiedDate(): string {
      return DateTimeUtils.isoToNowFullCuriefenseFormat(this.localDoc?.mdate)
    },
  },

  methods: {

    emitDocUpdate() {
      this.$emit('update:selectedDoc', this.localDoc)
    },

    emitFormInvalid( isFormInvalid: boolean ) {
      this.$emit('form-invalid', isFormInvalid)
    },

    setRuleRelation(relation: Relation) {
      this.localDoc.rule.relation = relation
      this.emitDocUpdate()
    },

    toggleRuleRelation(): void {
      this.localDoc.rule.relation === 'AND' ? this.setRuleRelation('OR') : this.setRuleRelation('AND')
    },

    removeAllSections() {
      this.localDoc.rule.sections.splice(0, this.localDoc.rule.sections.length)
      this.emitDocUpdate()
    },

    tryMatch(data: string, regex: RegExp, type: Category): TagRuleSectionEntry[] {
      let matches
      const entries = []
      matches = regex.exec(data)
      while (matches) {
        const entry: TagRuleSectionEntry = [type, matches[1], null]
        if (matches.length > 2 && matches.slice(-1)[0]) {
          entry[2] = (matches.slice(-1)[0]).slice(1, 128)
        }
        entries.push(entry)
        matches = regex.exec(data)
      }
      return entries
    },

    fetchList() {
      const lineMatchingIP =
          /^((((\d{1,3})\.){3}\d{1,3}(\/\d{1,2})?)|([0-9a-f]+:+){1,8}([0-9a-f]+)?(\/\d{1,3})?)((\s+)?([#;?].+))?/gm
      const lineMatchingASN = /(as\d{3,6})((\s+)?([#;?].+))?/gmi
      const singleIP = /^((((\d{1,3})\.){3}\d{1,3}(\/\d{1,2})?)|([0-9a-f]+:+){1,8}([0-9a-f]+)?(\/\d{1,3})?)$/
      const singleASN = /(as\d{3,6})/i
      // try every node / element of String type with the regex.
      const objectParser = (data: any, store: TagRuleSectionEntry[]) => {
        _.each(data, (item) => {
          if (_.isArray(item) && (item.length === 2 || item.length === 3)) {
            if (_.isString(item[0]) && (item[0].toLowerCase() === 'ip' || item[0].toLowerCase() === 'asn') &&
                _.isString(item[1]) && (singleIP.test(item[1]) || singleASN.test(item[1]))) {
              const annotation = (item[2] && _.isString(item[2])) ? item[2] : null
              store.push([item[0].toLowerCase() as Category, item[1], annotation])
            } else {
              objectParser(item, store)
            }
          } else if (_.isObject(item)) {
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
      RequestsUtils.sendRequest({methodName: 'GET', url: `tools/fetch?url=${url}`}).then((response: AxiosResponse) => {
        const data = response.data
        let entries: TagRuleSectionEntry[]
        entries = this.tryMatch(data, lineMatchingIP, 'ip')
        if (entries.length === 0) {
          entries = this.tryMatch(data, lineMatchingASN, 'asn')
        }
        if (entries.length === 0) {
          objectParser(data, entries)
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
  },
})
</script>

<style scoped lang="scss">
.pointer {
  cursor: pointer;
}

.tags {
  display: inline-block;
  line-height: 0;
}

</style>
