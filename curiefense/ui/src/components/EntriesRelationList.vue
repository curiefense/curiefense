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
                <tr
                  v-for="(entry,entryIndex) in sectionsCurrentPage[sectionIndex]"
                  :key="entryIndex"
                  class="entry-row"
                  :class="{'has-text-danger': isEntryDuplicate( sectionIndex, entry )}"
                >
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
                  <td :title="dualCell(entry[1])" class="is-size-7 entry-value width-250px ellipsis">
                    <span v-html="dualCell(entry[1])"></span>
                  </td>
                  <td :title="entry[2]" class="is-size-7 entry-annotation width-250px ellipsis">
                    {{ entry[2] ? entry[2].substr(0, 60) : '' }}
                  </td>
                  <td class="is-size-7 width-80px">
                    <a v-if="editable"
                       tabindex="0"
                       class="is-small has-text-grey remove-entry-button"
                       :data-section="sectionIndex"
                       :data-entry="entryIndex"
                       title="remove entry"
                       @click="removeEntry(section, sectionIndex, entryIndex)"
                       @keypress.space.prevent
                       @keypress.space="removeEntry(section, sectionIndex, entryIndex)"
                       @keypress.enter="removeEntry(section, sectionIndex, entryIndex)">
                      remove
                    </a>
                  </td>
                </tr>

                <tr v-if="newEntrySectionIndex !== sectionIndex && editable">
                  <td colspan="5">
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
                </tr>

                <tr v-if="newEntrySectionIndex === sectionIndex && editable" class="new-entry-row">
                  <td class="is-size-7" colspan="2">
                    <div class="select is-small is-fullwidth">
                      <select
                        v-model="newEntryCategory"
                        @change="clearFields"
                        title="New entry category"
                        class="select new-entry-type-selection"
                      >
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
                             :class="{ 'is-danger': isErrorField( `${newEntryCategory}${sectionIndex}` )}"
                             title="Name"
                             placeholder="Name"
                             @input="validateRegex( `${newEntryCategory}${sectionIndex}`, $event.target.value )"
                             v-model="newEntryItem.firstAttr"/>
                      <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
                    </div>
                    <textarea v-else
                              title="Entries"
                              v-model="newEntryItem.firstAttr"
                              @input="validateValue( sectionIndex, $event.target.value )"
                              placeholder="One entry per line, use '#' for annotation"
                              class="textarea is-small is-fullwidth new-entry-textarea"
                              :class="{ 'is-danger': isErrorField( `${newEntryCategory}${sectionIndex}` )}"
                              rows="3">
                    </textarea>
                    <div class="help is-danger" v-if="invalidIPs.length">
                      <div class="mr-2">Please check the following:</div>
                      <div v-for="(err,errIndex) in invalidIPs" :key="errIndex">
                        {{ err }}
                      </div>
                    </div>
                  </td>
                  <td class="is-size-7 width-250px">
                    <div class="control has-icons-left is-fullwidth new-entry-value-annotation">
                      <input
                        class="input is-small new-entry-value-annotation-input"
                        :class="{'is-danger': errorSecondAttr( sectionIndex )}"
                        :placeholder="isCategoryArgsCookiesHeaders( newEntryCategory ) ? 'Value' : 'Annotation'"
                        v-model="newEntryItem.secondAttr"
                        @input="onChangeSecondAttr( sectionIndex, $event.target.value )"
                      />
                      <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
                    </div>
                  </td>
                  <td class="is-size-7 width-80px">
                    <a class="is-size-7 has-text-grey add-button confirm-add-entry-button"
                       title="add new row"
                       tabindex="0"
                       @click="addEntry(section,sectionIndex)"
                       @keypress.space.prevent
                       @keypress.space="addEntry(section,sectionIndex)"
                       @keypress.enter="addEntry(section,sectionIndex)">
                      <i class="fas fa-check"></i> Add
                    </a>
                    <br/>
                    <a class="is-size-7 has-text-grey remove-button cancel-entry-button"
                       title="cancel add new row"
                       tabindex="0"
                       @click="cancelEntry(sectionIndex)"
                       @keypress.space.prevent
                       @keypress.space="cancelEntry(sectionIndex)"
                       @keypress.enter="cancelEntry(sectionIndex)">
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
    <div
      v-if="editable && newEntrySectionIndex === -1"
      class="field is-grouped is-pulled-left"
    >
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
import Utils from '@/assets/Utils.ts'
import {Category, Relation, GlobalFilter, GlobalFilterSection, GlobalFilterSectionEntry} from '@/types'

export default Vue.extend({
  name: 'EntriesRelationList',

  props: {
    rule: {
      type: Object as PropType<GlobalFilter['rule']>,
      default: () => {
        return {
          relation: 'OR',
          sections: [] as GlobalFilterSection[],
        }
      },
      validator(value) {
        if (!value || !value.relation || !value.sections) {
          return false
        }
        const isRelationValid = ['OR', 'AND'].includes(value.relation.toUpperCase())
        const isListInvalid = value.sections.find((section: GlobalFilterSection) => {
          const isSectionRelationInvalid = !(['OR', 'AND'].includes(value.relation.toUpperCase()))
          const isSectionsEntriesInvalid = !section.entries || !section.entries.find ||
              section.entries.find((entry: GlobalFilterSectionEntry) => {
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
      duplicatedEntries: [],
      invalidIPs: [],
      entriesErrors: [],
    }
  },

  computed: {

    rowsPerPage(): number {
      // no pagination for multiple sections
      return this.localRule.sections.length > 1 ? 1000 * 1000 : 20
    },

    sectionsCurrentPage(): GlobalFilterSectionEntry[][] {
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

    localRule(): GlobalFilter['rule'] {
      return _.cloneDeep(this.rule) as GlobalFilter['rule']
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
    entriesErrors: {
      handler( value ) {
        this.$emit('invalid', !!value.length)
      },
    },
  },

  methods: {

    isCategoryArgsCookiesHeaders(category: Category) {
      return (new RegExp('(args|cookies|headers)')).test(category)
    },

    emitRuleUpdate() {
      this.$emit('update:rule', this.localRule)
    },

    sectionContainsSameCategoryItems(section: GlobalFilterSection) {
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

    toggleSectionRelation(section: GlobalFilterSection) {
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

    totalPages(section: GlobalFilterSection) {
      return Math.ceil(this.sectionTotalEntries(section) / this.rowsPerPage)
    },

    sectionTotalEntries(section: GlobalFilterSection) {
      return section?.entries?.length || 0
    },

    dualCell(cell: GlobalFilterSectionEntry[1]) {
      if (_.isArray(cell)) {
        return `${cell[0]}: ${cell[1]}`
      } else {
        return cell
      }
    },

    navigate(section: GlobalFilterSection, sectionIndex: number, pageNum: number) {
      if (pageNum >= 1 && pageNum <= this.totalPages(section)) {
        Vue.set(this.sectionsCurrentPageIndex, sectionIndex, pageNum)
      }
    },

    addSection() {
      const newSection = {
        relation: 'OR' as Relation,
        entries: [] as GlobalFilterSectionEntry[],
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

    addEntry(section: GlobalFilterSection, sectionIndex: number) {
      if ( this.isErrorField( `${this.newEntryCategory}${sectionIndex}` ) || !this.newEntryItem.firstAttr.trim() ) {
        return
      }
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
      this.$nextTick( this.validateDuplicates )
    },

    removeEntry(section: GlobalFilterSection, sectionIndex: number, entryIndex: number) {
      const pointer = ((this.sectionsCurrentPageIndex[sectionIndex] - 1) * this.rowsPerPage) + entryIndex
      section.entries.splice(pointer, 1)
      if ( !section.entries.length ) {
        this.removeSection(sectionIndex)
      }
      this.emitRuleUpdate()
      this.$nextTick( this.validateDuplicates )
    },

    cancelEntry( sectionIndex: number ) {
      this.setNewEntryIndex(-1)
      this.invalidIPs = []
      this.clearError( `${this.newEntryCategory}${sectionIndex}` )
      if ( !this.localRule.sections[sectionIndex].entries.length ) {
        this.removeSection(sectionIndex)
      }
    },

    clearFields() {
      this.newEntryItem = {
        firstAttr: '',
        secondAttr: '',
      }
      this.clearError()
      this.invalidIPs = []
    },

    clearError( field: string='' ) {
      this.entriesErrors = field ? this.entriesErrors.filter( (err: string) => err !== field ) : []
    },

    isErrorField( field: string ) {
      return this.entriesErrors.includes( field )
    },

    addError( field: string ) {
      if ( !this.isErrorField( field )) {
        this.entriesErrors.push( field )
      }
    },

    validateDuplicates() {
      this.duplicatedEntries = []
      this.rule.sections.forEach(
        ({entries}, sectionIndex: number ) => entries.map(
          ({0: category, 1: value}) => {
            const isDuplicate = entries.filter(({0: eCat, 1: eVal}) => eCat === category && _.isEqual(eVal, value))?.length > 1
            if ( isDuplicate && !this.isEntryDuplicate( sectionIndex, [category, value])) {
              this.duplicatedEntries.push( [sectionIndex, category, value] )
            }
            return !isDuplicate
          },
        ).every((entry) => entry ),
      )
      if ( this.duplicatedEntries.length ) {
        const duplicatesMsg = this.duplicatedEntries.reduce(
          ( prev: string, [section, category, value]: GlobalFilterSectionEntry ) => {
            const sectionMsg = this.rule.sections.length > 1 ? `Section ${section+1}: ` : ''
            return `${prev}<br />${sectionMsg}${this.listEntryTypes[category as Category]?.title} = ${this.dualCell( value )}`
          },
          '',
        )
        this.addError( 'duplicate' )
        Utils.toast( `There are duplicate entries in the list:${duplicatesMsg}`, 'is-danger' )
      } else {
        this.clearError( 'duplicate' )
      }
    },

    isEntryDuplicate( sectionIndex: number, [currentCategory, currentValue]: GlobalFilterSectionEntry ) {
      return this.duplicatedEntries.findIndex(
        ([section, category, value]) => section === sectionIndex && category === currentCategory && _.isEqual(value, currentValue),
      ) > -1
    },

    validateValue( sectionIndex: number, value: string ) {
      const {validateNotEmpty, newEntryCategory, validateIp, validateRegex} = this
      let validator: Function = validateNotEmpty
      if (['path', 'query', 'uri'].includes( newEntryCategory )) {
        validator = validateRegex
      } else if ( newEntryCategory === 'ip' ) {
        validator = validateIp
      }
      validator( `${newEntryCategory}${sectionIndex}`, value )
    },

    validateIp( id: string, value: string ) {
      const ipPattern = /((^\s*((([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]))\s*(:([0-9]|[1-8][0-9]|9[0-9]|[1-8][0-9]{2}|9[0-8][0-9]|99[0-9]|[1-8][0-9]{3}|9[0-8][0-9]{2}|99[0-8][0-9]|999[0-9]|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])|(\/[0-9]|\/[1-2][0-9]|\/[1-3][0-2]))?(\s?(#([a-zA-Z0-9$@$!%*?&#^-_. +:"'/\\;,-=]+)?)?)?$)|(^\s*((([0-9A-Fa-f]{1,4}:){7}([0-9A-Fa-f]{1,4}|:))|(([0-9A-Fa-f]{1,4}:){6}(:[0-9A-Fa-f]{1,4}|((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){5}(((:[0-9A-Fa-f]{1,4}){1,2})|:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){4}(((:[0-9A-Fa-f]{1,4}){1,3})|((:[0-9A-Fa-f]{1,4})?:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){3}(((:[0-9A-Fa-f]{1,4}){1,4})|((:[0-9A-Fa-f]{1,4}){0,2}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){2}(((:[0-9A-Fa-f]{1,4}){1,5})|((:[0-9A-Fa-f]{1,4}){0,3}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){1}(((:[0-9A-Fa-f]{1,4}){1,6})|((:[0-9A-Fa-f]{1,4}){0,4}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(:(((:[0-9A-Fa-f]{1,4}){1,7})|((:[0-9A-Fa-f]{1,4}){0,5}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:)))(%.+)?\s*(:([0-9]|[1-8][0-9]|9[0-9]|[1-8][0-9]{2}|9[0-8][0-9]|99[0-9]|[1-8][0-9]{3}|9[0-8][0-9]{2}|99[0-8][0-9]|999[0-9]|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])|(\/[0-9]|\/[1-2][0-9]|\/[1-3][0-2]))?(\s?(#([a-zA-Z0-9$@$!%*?&#^-_. +:"'/\\;,-=]+)?)?)?$))/
      const ipList = value.split('\n').filter( (ip) => ip )
      this.invalidIPs = []
      ipList.forEach(( line, index ) => {
        if ( !this.validate( line, ipPattern, id )) {
          this.invalidIPs.push( ipList.length > 1 ? `(line ${index+1}) ${line}` : line )
        }
      })
      if ( this.invalidIPs.length ) {
        this.addError( id )
      }
    },

    validateRegex( id: string, value: string ) {
      // TODO: Fix regex test for rust standards and re-apply this
      // const val = value.trim().replaceAll(/\(\?[a-z]{1,3}\)/g, '') // remove unsupported in js mode modifiers
      // try {
      //   this.clearError(id)
      //   new RegExp(val)
      // } catch {
      //   this.validate( val, /^[\w-]+$/, id )
      // }
    },

    validateNotEmpty( id: string, value: string ) {
      const val = value.trim()
      this.clearError( id )
      this.validate( val, null, id )
    },

    validate( value: string, pattern: RegExp, name: string ) {
      const isValid = pattern ? ( new RegExp( pattern )).test( value ) : value.length
      if ( !isValid && !this.isErrorField( name )) {
        this.addError( name )
      } else if ( isValid ) {
        this.clearError( name )
      }
      return isValid
    },

    errorSecondAttr( sectionIndex: number ) {
      const {isCategoryArgsCookiesHeaders, newEntryCategory, isErrorField} = this
      return isCategoryArgsCookiesHeaders(newEntryCategory) && isErrorField(`${newEntryCategory}${sectionIndex}-secondAttr`)
    },

    onChangeSecondAttr( sectionIndex: number, value: string ) {
      const {isCategoryArgsCookiesHeaders, newEntryCategory, validateRegex} = this
      if ( isCategoryArgsCookiesHeaders( newEntryCategory )) {
        validateRegex( `${newEntryCategory}${sectionIndex}-secondAttr`, value )
      }
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
