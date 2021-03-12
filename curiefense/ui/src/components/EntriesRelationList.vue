<template>
  <div>
    <div class="tile is-ancestor">
      <div class="tile is-vertical">
        <div class="tile">
          <div class="tile is-parent is-vertical">
            <div class="tile is-child box is-primary section"
                 v-for="(section, sectionIndex) in localRule.sections" :key="sectionIndex">
              <div class="has-text-centered relation-selection-wrapper"
                   v-if="localRule.sections.length > 1 && sectionIndex > 0">
                  <span class="tag has-text-weight-semibold">
                    {{ localRule.relation }}
                  </span>
              </div>
              <table class="table is-narrow entries-table mb-0">
                <tbody>
                <tr v-for="(entry,entryIndex) in sectionsCurrentPage[sectionIndex]" :key="entryIndex" class="entry-row">
                  <td class="is-size-7 width-50px has-text-centered has-text-weight-medium">
                      <span
                          v-if="((entryIndex + 1) + ((sectionsCurrentPageIndex[sectionIndex] - 1) * rowsPerPage)) !== 1"
                          class="is-small pointer section-relation-toggle"
                          @click="toggleSectionRelation(section)">
                        {{ section.relation }}
                      </span>
                  </td>
                  <td class="is-size-7 entry-category has-text-weight-medium width-100px">
                    {{ listEntryTypes[entry[0]].title }}
                  </td>
                  <td :title="dualCell(entry[1])" class="is-size-7 entry-value width-250px ellipsis"><span
                      v-html="dualCell(entry[1])"></span></td>
                  <td :title="entry[2]" class="is-size-7 entry-annotation width-250px ellipsis">
                    {{ entry[2] ? entry[2].substr(0, 60) : '' }}
                  </td>
                  <td class="is-size-7 width-80px">
                    <a v-if="editable"
                       tabindex="0"
                       class="is-small has-text-grey remove-entry-button" title="remove entry"
                       @click="removeEntry(section, sectionIndex, entryIndex)"
                       @keypress.space.prevent
                       @keypress.space="removeEntry(section, sectionIndex, entryIndex)"
                       @keypress.enter="removeEntry(section, sectionIndex, entryIndex)">
                      remove
                    </a>
                  </td>
                </tr>
                <tr v-if="newEntrySectionIndex !== sectionIndex && editable">
                  <td>
                    <a class="is-size-7 has-text-grey-lighter add-button add-entry-button"
                       title="add new row"
                       tabindex="0"
                       @click="setNewEntryIndex(sectionIndex)"
                       @keypress.space.prevent
                       @keypress.space="setNewEntryIndex(sectionIndex)"
                       @keypress.enter="setNewEntryIndex(sectionIndex)">
                      <i class="fas fa-plus"></i>
                    </a>
                    &nbsp;&middot;&nbsp;
                    <a class="is-size-7 has-text-grey-lighter remove-button remove-section-button"
                       title="remove entire section"
                       tabindex="0"
                       @click="removeSection(sectionIndex)"
                       @keypress.space.prevent
                       @keypress.space="removeSection(sectionIndex)"
                       @keypress.enter="removeSection(sectionIndex)">
                      <i class="fas fa-trash"></i>
                    </a>
                  </td>
                  <td colspan="4">
                  </td>

                </tr>
                <tr v-if="newEntrySectionIndex === sectionIndex && editable" class="new-entry-row">
                  <td class="is-size-7" colspan="2">
                    <div class="select is-small is-fullwidth">
                      <select v-model="newEntryCategory"
                              title="New entry category"
                              class="select new-entry-type-selection">
                        <option v-for="(entryType, category) in listEntryTypes" :key="category" :value="category">
                          {{ entryType.title }}
                        </option>
                      </select>
                    </div>
                  </td>
                  <td class="is-size-7 width-250px">
                    <div v-if="isCategoryArgsCookiesHeaders(newEntryCategory)"
                         class="control has-icons-left is-fullwidth new-entry-name">
                      <input class="input is-small new-entry-name-input"
                             title="Name"
                             placeholder="Name"
                             v-model="newEntryItem.firstAttr"/>
                      <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
                    </div>
                    <textarea v-else
                              title="Entries"
                              v-model="newEntryItem.firstAttr"
                              placeholder="One entry per line, use '#' for annotation"
                              class="textarea is-small is-fullwidth new-entry-textarea"
                              rows="3">
                    </textarea>
                  </td>
                  <td class="is-size-7 width-250px">
                    <div class="control has-icons-left is-fullwidth new-entry-value-annotation">
                      <input class="input is-small new-entry-value-annotation-input"
                             :placeholder="isCategoryArgsCookiesHeaders(newEntryCategory) ? 'Value' : 'Annotation'"
                             v-model="newEntryItem.secondAttr"/>
                      <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
                    </div>
                  </td>
                  <td class="is-size-7 width-80px">
                    <a class="is-size-7 has-text-grey add-button confirm-add-entry-button"
                       title="add new row"
                       tabindex="0"
                       @click="addEntry(section)"
                       @keypress.space.prevent
                       @keypress.space="addEntry(section)"
                       @keypress.enter="addEntry(section)">
                      <i class="fas fa-check"></i> Add
                    </a>
                    <br/>
                    <a class="is-size-7 has-text-grey remove-button"
                       title="cancel add new row"
                       tabindex="0"
                       @click="setNewEntryIndex(-1)"
                       @keypress.space.prevent
                       @keypress.space="setNewEntryIndex(-1)"
                       @keypress.enter="setNewEntryIndex(-1)">
                      <i class="fas fa-times"></i> Cancel
                    </a>
                  </td>
                </tr>

                <tr v-if="totalPages(section) > 1">
                  <td colspan="5">
                    <nav aria-label="pagination" class="pagination is-small" role="navigation">
                      <a :disabled="sectionsCurrentPageIndex[sectionIndex] === 1"
                         class="is-pulled-left pagination-previous"
                         tabindex="0"
                         @click="navigate(section, sectionIndex, sectionsCurrentPageIndex[sectionIndex] - 1)"
                         @keypress.space.prevent
                         @keypress.space="navigate(section, sectionIndex, sectionsCurrentPageIndex[sectionIndex] - 1)"
                         @keypress.enter="navigate(section, sectionIndex, sectionsCurrentPageIndex[sectionIndex] - 1)">
                        Previous page
                      </a>
                      <a :disabled="sectionsCurrentPageIndex[sectionIndex] === totalPages(section)"
                         class="is-pulled-right pagination-next"
                         tabindex="0"
                         @click="navigate(section, sectionIndex, sectionsCurrentPageIndex[sectionIndex] + 1)"
                         @keypress.space.prevent
                         @keypress.space="navigate(section, sectionIndex, sectionsCurrentPageIndex[sectionIndex] + 1)"
                         @keypress.enter="navigate(section, sectionIndex, sectionsCurrentPageIndex[sectionIndex] + 1)">
                        Next page
                      </a>
                    </nav>
                  </td>
                </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div v-if="editable" class="field is-grouped is-pulled-left">
      <div class="control">
        <button class="button is-small add-section-button"
                title="Add new section"
                @click="addSection">
          Create new section
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import _ from 'lodash'
import Vue, {PropType} from 'vue'
import {Category, Relation, TagRule, TagRuleSection, TagRuleSectionEntry} from '@/types'

export default Vue.extend({
  name: 'EntriesRelationList',

  props: {
    rule: {
      type: Object as PropType<TagRule['rule']>,
      default: () => {
        return {
          relation: 'OR',
          sections: [] as TagRuleSection[],
        }
      },
      validator(value) {
        if (!value || !value.relation || !value.sections) {
          return false
        }
        const isRelationValid = ['OR', 'AND'].includes(value.relation.toUpperCase())
        const isListInvalid = value.sections.find((section: TagRuleSection) => {
          const isSectionRelationInvalid = !(['OR', 'AND'].includes(value.relation.toUpperCase()))
          const isSectionsEntriesInvalid = !section.entries || !section.entries.find ||
              section.entries.find((entry: TagRuleSectionEntry) => {
                return (!entry || !entry.length || entry.length < 2 || entry.length > 3)
              })
          return isSectionRelationInvalid || isSectionsEntriesInvalid
        })
        return isRelationValid && !isListInvalid
      },
    },
    editable: Boolean,
  },

  data() {
    return {
      // rowsPerPage: 20, >> computed property now
      sectionsCurrentPageIndex: [],
      listEntryTypes: {
        'path': {'title': 'Path', 'pair': false},
        'query': {'title': 'Query', 'pair': false},
        'uri': {'title': 'URI', 'pair': false},
        'method': {'title': 'Method', 'pair': false},
        'ip': {'title': 'IP Address', 'pair': false},
        'asn': {'title': 'ASN', 'pair': false},
        'country': {'title': 'Country', 'pair': false},
        'headers': {'title': 'Header', 'pair': true},
        'args': {'title': 'Argument', 'pair': true},
        'cookies': {'title': 'Cookie', 'pair': true},
      },
      newEntrySectionIndex: -1,
      // newEntryCategory - start with most common category - IP
      newEntryCategory: 'ip' as Category,
      // For headers, args, cookies:
      //   firstAttr = name
      //   secondAttr = value
      // For every other entry type:
      //   firstAttr = list of values (with possible annotation)
      //   secondAttr = single annotation
      newEntryItem: {
        firstAttr: '',
        secondAttr: '',
      },
    }
  },

  computed: {

    rowsPerPage(): number {
      // no pagination for multiple sections
      return this.localRule.sections.length > 1 ? 1000 * 1000 : 20
    },

    sectionsCurrentPage(): TagRuleSectionEntry[][] {
      const pages = []
      for (let i = 0; i < this.localRule.sections.length; i++) {
        const section = this.localRule.sections[i]
        if (this.sectionTotalEntries(section) !== 0) {
          pages[i] = _.slice(section.entries,
              (this.sectionsCurrentPageIndex[i] - 1) * this.rowsPerPage,
              this.rowsPerPage * this.sectionsCurrentPageIndex[i])
        }
      }
      return pages
    },

    localRule(): TagRule['rule'] {
      return _.cloneDeep(this.rule) as TagRule['rule']
    },
  },

  watch: {
    rule: {
      handler: function() {
        this.sectionsCurrentPageIndex = []
        for (let i = 0; i < this.localRule.sections.length; i++) {
          const section = this.localRule.sections[i]
          Vue.set(this.sectionsCurrentPageIndex, i, 1)
          if (this.sectionContainsSameCategoryItems(section)) {
            section.relation = 'OR'
          }
        }
      },
      immediate: true,
      deep: true,
    },
  },

  methods: {
    isCategoryArgsCookiesHeaders(category: Category) {
      return (new RegExp('(args|cookies|headers)')).test(category)
    },

    emitRuleUpdate() {
      this.$emit('update:rule', this.localRule)
    },

    sectionContainsSameCategoryItems(section: TagRuleSection) {
      const countedCategories = _.countBy(section.entries, (entry) => {
        return this.listEntryTypes[entry[0]].title
      })
      const categoriesKeys = Object.keys(this.listEntryTypes)
      for (let i = 0; i < categoriesKeys.length; i++) {
        const categoryKey = categoriesKeys[i] as Category
        const category = this.listEntryTypes[categoryKey]
        if (this.isCategoryArgsCookiesHeaders(categoryKey)) {
          break
        }
        if (countedCategories[category.title] > 1) {
          return true
        }
      }
      return false
    },

    toggleSectionRelation(section: TagRuleSection) {
      if (this.sectionContainsSameCategoryItems(section)) {
        return
      }
      section.relation = (section.relation === 'AND') ? 'OR' : 'AND'
      this.emitRuleUpdate()
    },

    setNewEntryIndex(index: number) {
      this.newEntryItem = {
        firstAttr: '',
        secondAttr: '',
      }
      this.newEntryCategory = 'ip'
      this.newEntrySectionIndex = index
    },

    totalPages(section: TagRuleSection) {
      return Math.ceil(this.sectionTotalEntries(section) / this.rowsPerPage)
    },

    sectionTotalEntries(section: TagRuleSection) {
      return section?.entries?.length || 0
    },

    dualCell(cell: TagRuleSectionEntry[1]) {
      if (_.isArray(cell)) {
        return `${cell[0]}: ${cell[1]}`
      } else {
        return cell
      }
    },

    navigate(section: TagRuleSection, sectionIndex: number, pageNum: number) {
      if (pageNum >= 1 && pageNum <= this.totalPages(section)) {
        Vue.set(this.sectionsCurrentPageIndex, sectionIndex, pageNum)
      }
    },

    addSection() {
      const newSection = {
        relation: 'OR' as Relation,
        entries: [] as TagRuleSectionEntry[],
      }
      this.localRule.sections.push(newSection)
      // Vue.set(this.sectionsCurrentPageIndex, this.localRule.sections.length - 1, 1)
      this.setNewEntryIndex(this.localRule.sections.length - 1)
      this.emitRuleUpdate()
    },

    removeSection(sectionIndex: number) {
      this.localRule.sections.splice(sectionIndex, 1)
      this.sectionsCurrentPageIndex.splice(sectionIndex, 1)
      this.emitRuleUpdate()
    },

    addEntry(section: TagRuleSection) {
      // args cookies or headers
      if (this.isCategoryArgsCookiesHeaders(this.newEntryCategory)) {
        const newEntryName = this.newEntryItem.firstAttr.trim().toLowerCase()
        const newEntryValue = this.newEntryItem.secondAttr.trim().toLowerCase()
        if (newEntryName && newEntryValue) {
          section.entries.push([this.newEntryCategory, [newEntryName, newEntryValue]])
        }
      } else { // every other entry type
        const generalAnnotation = this.newEntryItem.secondAttr.trim()
        _.each(this.newEntryItem.firstAttr.split('\n'), (line) => {
          let [entry, annotation] = line.trim().split('#')
          entry = entry.trim()
          annotation = annotation ? annotation.trim() : generalAnnotation
          section.entries.push([this.newEntryCategory, entry, annotation])
        })
      }
      // change relation to 'OR' if needed
      if (this.sectionContainsSameCategoryItems(section)) {
        section.relation = 'OR'
      }
      this.setNewEntryIndex(-1)
      this.emitRuleUpdate()
    },

    removeEntry(section: TagRuleSection, sectionIndex: number, entryIndex: number) {
      const pointer = ((this.sectionsCurrentPageIndex[sectionIndex] - 1) * this.rowsPerPage) + entryIndex
      section.entries.splice(pointer, 1)
      if (section.entries.length === 0) {
        this.removeSection(sectionIndex)
      }
      this.emitRuleUpdate()
    },
  },
})
</script>
<style scoped lang="scss">

.pointer {
  cursor: pointer;
}

.section {
  padding: initial;
}

.relation-selection-wrapper {
  margin-top: -1.5rem;
}

.sections-wrapper {
  max-height: 1000px;
  overflow-y: auto;
}

.section-relation-toggle {
  cursor: pointer;
}
</style>
