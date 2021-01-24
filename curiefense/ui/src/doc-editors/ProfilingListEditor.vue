<template>
  <div class="card">
    <div class="card-content">
      <div class="content">
        <div class="columns columns-divided">
          <div class="column is-3">
            <div class="field">
              <label class="label is-small">Name
                <span class="has-text-grey is-pulled-right" title="Rule Id">{{ selectedDoc.id }}</span>
              </label>
              <div class="control">
                <input class="input is-small"
                       placeholder="list name"
                       v-model="selectedDoc.name"
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
                       v-model="selectedDoc.active">
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
              <a v-if="selectedDoc && selectedDoc.source && selectedDoc.source.indexOf('http') === 0"
                 class="is-small has-text-grey is-size-7 is-pulled-right"
                 @click="fetchList">
                update now
              </a>
              <label class="label is-small">Source</label>
              <div class="control">
                <input class="input is-small" v-model="selectedDoc.source"
                       :readonly="readonly"/>
              </div>
            </div>
            <div class="field">
              <response-action :object-with-action.sync="selectedDoc"
                               ignore="ban"
                               label-separated-line
                               wide-columns
                               is-single-input-column/>
            </div>
            <div class="field">
              <label class="label is-small">Notes</label>
              <div class="control">
                <textarea class="is-small textarea" v-model="selectedDoc.notes" rows="2"
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
              <p class="help">updated @ {{ selectedDoc.mdate }}</p>
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

<script>

import _ from 'lodash'
import ResponseAction from '@/components/ResponseAction'
import TagAutocompleteInput from '@/components/TagAutocompleteInput'
import RequestsUtils from '@/assets/RequestsUtils'
import EntriesRelationList from '@/components/EntriesRelationList'

export default {
  name: 'ProfilingListEditor',

  components: {
    ResponseAction,
    EntriesRelationList,
    TagAutocompleteInput
  },

  props: {
    selectedDoc: Object,
    apiPath: String
  },

  computed: {
    sectionsEntriesDisplay() {
      let sectionsCounter = (this.localDoc?.rule?.sections?.length !== 1) ? 'sections' : 'section'
      let entriesCounter = (this.localDocTotalEntries !== 1) ? 'entries' : 'entry'
      return `${this.localDoc?.rule?.sections?.length} ${sectionsCounter}\t|\t${this.localDocTotalEntries} ${entriesCounter}`
    },
    readonly() {
      return this.selectedDoc.source === 'reblaze-managed'
    },

    editable() {
      return this.selectedDoc.source === 'self-managed'
    },

    selectedDocTags: {
      get: function () {
        if (this.selectedDoc.tags)
          return this.selectedDoc.tags.join(' ')
        return ''
      },
      set: _.debounce(function (tags) {
        this.selectedDoc.tags = this.ld.map(tags.split(' '), (tag) => {
          return tag.trim()
        })
      }, 500)
    },

    localDoc() {
      return JSON.parse(JSON.stringify(this.selectedDoc))
    },

    localDocTotalEntries() {
      let totalEntries = 0
      if (this.localDoc?.rule?.sections?.length) {
        totalEntries = this.ld.sumBy(this.localDoc.rule.sections, (section) => {
          return section.entries?.length
        })
      }
      return totalEntries
    }

  },

  methods: {

    emitDocUpdate() {
      this.$emit('update', this.localDoc)
    },

    setRuleRelation(relation) {
      if (relation)
        this.localDoc.rule.relation = relation
      else // toggle
        this.localDoc.rule.relation = (this.localDoc.rule.relation === 'AND') ? 'OR' : 'AND'

      this.emitDocUpdate()
    },

    removeAllSections() {
      this.localDoc.rule.sections.splice(0, this.localDoc.rule.sections.length)
      this.emitDocUpdate()
    },

    tryMatch(data, regex, type) {
      let matches, entries = []
      matches = regex.exec(data)
      while (matches) {
        let entry = [type, matches[1], null]
        if (matches.length > 2) {
          entry[2] = (matches.slice(-1)[0] || '').slice(0, 128)
        }
        entries.push(entry)
        matches = regex.exec(data)
      }
      return entries
    },

    fetchList() {
      const line_matching_ip = /^((((\d{1,3})\.){3}\d{1,3}(\/\d{1,2}))|([0-9a-f]+:+){1,8}([0-9a-f]+)?(\/\d{1,3})?)\s+([#;/].+)/gm,
          line_matching_asn = /(as\d{3,6})((\s+)?([#;/?].+))?/gmi,
          single_ip = /^((((\d{1,3})\.){3}\d{1,3}(\/\d{1,2}))|([0-9a-f]+:+){1,8}([0-9a-f]+)?(\/\d{1,3})?)$/,
          single_asn = /(as\d{3,6})/i
      // try every node / element of String type with the regex.
      let object_parser = (data, store) => {
        this.ld.each(data,
            (item) => {
              if (this.ld.isArray(item) || this.ld.isObject(item)) {
                object_parser(item, store)
              }
              if (this.ld.isString(item)) {
                if (single_ip.test(item)) {
                  store.push(['ip', item, null])
                } else if (single_asn.test(item)) {
                  store.push(['asn', item, null])
                }
              }
            })
      }
      let url = this.selectedDoc.source
      RequestsUtils.sendRequest('GET', `tools/fetch?url=${url}`).then(
          (response) => {
            let data = response.data,
                entries = []
            entries = this.tryMatch(data, line_matching_ip, 'ip')
            if (entries.length === 0) {
              entries = this.tryMatch(data, line_matching_asn, 'asn')
            }
            if (entries.length === 0) {
              try {
                object_parser(data, entries)
              } catch (e) {
                console.log(e)
              }
            }
            if (entries.length > 0) {
              let new_section = {
                relation: 'OR',
                entries: entries
              }
              this.selectedDoc.rule = {
                'sections': [
                  new_section
                ],
                'relation': 'OR'
              }

              this.selectedDoc.mdate = (new Date).toISOString()
            }

          })
    },

    updateRule(rule) {
      this.localDoc.rule = rule
      this.emitDocUpdate()
    },
  }

}
</script>

<style scoped>
.pointer {
  cursor: pointer;
}

.rule-relation-toggle {
  cursor: pointer;
}
</style>
