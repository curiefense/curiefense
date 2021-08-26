<template>
  <div class="card">
    <div class="card-content">
      <div class="content">
        <div class="columns columns-divided">
          <div class="column is-5">
            <div class="field">
              <label class="label is-small">
                Name
                <span class="has-text-grey is-pulled-right document-id" title="Flow control id">
                    {{ localDoc.id }}
                  </span>
              </label>
              <div class="control">
                <input class="input is-small document-name"
                       title="Flow control name"
                       placeholder="Flow control name"
                       @change="emitDocUpdate"
                       v-model="localDoc.name"/>
              </div>
            </div>
            <div class="field">
              <label class="checkbox is-size-7">
                <input type="checkbox"
                       class="document-active"
                       @change="emitDocUpdate"
                       v-model="localDoc.active">
                Active
              </label>
            </div>
            <div class="field">
              <label class="label is-small has-text-left form-label">
                TTL
              </label>
              <div class="control suffix seconds-suffix">
                <input class="input is-small document-ttl"
                       type="number"
                       title="Flow control duration"
                       placeholder="Flow control duration"
                       @change="emitDocUpdate"
                       v-model.number="localDoc.ttl">
              </div>
            </div>
            <div class="field">
              <limit-option v-for="(option, index) in localDoc.key"
                            label-separated-line
                            :label="index === 0 ? 'Count by' : ''"
                            show-remove
                            @remove="removeKey(index)"
                            @change="updateKeyOption($event, index)"
                            :removable="localDoc.key.length > 1"
                            :index="index"
                            :option="generateOption(option)"
                            :key="getOptionTextKey(option, index)"/>
              <a title="Add new option rule"
                 class="is-text is-small is-size-7 ml-3 add-key-button"
                 tabindex="0"
                 @click="addKey()"
                 @keypress.space.prevent
                 @keypress.space="addKey()"
                 @keypress.enter="addKey()">
                New entry
              </a>
              <p class="has-text-danger is-size-7 ml-3 mt-3 key-invalid"
                 v-if="!keysAreValid">
                Count-by entries must be unique
              </p>
            </div>
            <div class="field">
              <response-action :action.sync="localDoc.action"
                               @update:action="emitDocUpdate"
                               label-separated-line/>
            </div>
            <div class="field">
              <label class="label is-small">Notes</label>
              <div class="control">
                <textarea class="is-small textarea document-notes"
                          title="Notes"
                          @change="emitDocUpdate"
                          v-model="localDoc.notes"
                          rows="2">
                </textarea>
              </div>
            </div>
            <div class="columns filter-columns">
              <div class="column is-6 filter-column"
                   v-for="filter in filters"
                   :key="filter"
                   :class="filter + '-filter-column'">
                <p class="title is-7">
                  {{ titles[filter] }}
                </p>
                <hr class="bar"
                    :class="`bar-${filter}`"/>
                <table class="table is-narrow is-fullwidth">
                  <tbody>
                  <tr v-for="(tag, tagIndex) in localDoc[filter]"
                      :key="tagIndex">
                    <td class="tag-cell"
                        :class=" duplicateTags[tag] ? 'has-text-danger' : '' ">
                      {{ tag }}
                    </td>
                    <td class="is-size-7 width-20px">
                      <a title="remove entry"
                         class="is-small has-text-grey remove-filter-entry-button"
                         tabindex="0"
                         @click="removeTag(filter, tagIndex)"
                         @keypress.space.prevent
                         @keypress.space="removeTag(filter, tagIndex)"
                         @keypress.enter="removeTag(filter, tagIndex)">
                        &ndash;
                      </a>
                    </td>
                  </tr>
                  <tr>
                    <td>
                      <tag-autocomplete-input v-if="addNewTagColName === filter"
                                              ref="tagAutocompleteInput"
                                              :clear-input-after-selection="true"
                                              :selection-type="'single'"
                                              :auto-focus="true"
                                              @keydown.esc="cancelAddNewTag"
                                              @tag-submitted="addNewTag(filter, $event)">
                      </tag-autocomplete-input>
                    </td>
                    <td class="is-size-7 width-20px">
                      <a title="add new entry"
                         class="is-size-7 width-20px is-small has-text-grey add-new-filter-entry-button"
                         tabindex="0"
                         @click="openTagInput(filter)"
                         @keypress.space.prevent
                         @keypress.space="openTagInput(filter)"
                         @keypress.enter="openTagInput(filter)">
                        +
                      </a>
                    </td>
                  </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>
          <div class="column is-7">
            <div class="sequence-wrapper"
                 ref="sequences">
              <template v-for="(sequenceItem, sequenceIndex) in localDoc.sequence">
                <div :key="sequenceIndex"
                     class="sequence mb-3"
                     :class="{hover: hoverSequenceIndex === sequenceIndex}"
                     :data-index="sequenceIndex"
                     draggable="true"
                     v-if="sequenceItem">
                  <div class="dragging-icon pl-2 pt-1"
                      @mouseover="setHoverSequenceIndex(sequenceIndex)"
                      @mouseleave="setHoverSequenceIndex()">
                    <i class="fas fa-arrows-alt" />
                  </div>
                  <div class="sequence-entries">
                    <table class="table is-narrow is-size-7 sequence-entries-table">
                      <tbody>
                      <tr class="sequence-entry-row method-entry-row">
                        <td class="is-size-7 width-50px sequence-entries-relation"></td>
                        <td class="width-80px is-vcentered">
                          Method
                        </td>
                        <td colspan="2">
                          <div class="control is-fullwidth">
                            <input class="input is-small method-entry-input"
                                  title="Method"
                                  v-model="sequenceItem.method"
                                  @input="emitDocUpdate"/>
                          </div>
                        </td>
                        <td class="width-80px" />
                      </tr>
                      <tr class="sequence-entry-row host-entry-row">
                        <td class="is-size-7 width-50px has-text-centered is-vcentered has-text-grey-light
                                  has-text-weight-medium sequence-entries-relation">
                          AND
                        </td>
                        <td class="width-80px is-vcentered">
                          Host
                        </td>
                        <td colspan="2">
                          <div class="control is-fullwidth">
                            <input class="input is-small host-entry-input"
                                  title="Host"
                                  v-model="sequenceItem.headers.host"
                                  @input="emitDocUpdate"/>
                          </div>
                        </td>
                        <td class="width-80px" />
                      </tr>
                      <tr class="sequence-entry-row uri-entry-row">
                        <td class="is-size-7 width-50px has-text-centered is-vcentered has-text-grey-light
                                  has-text-weight-medium sequence-entries-relation">
                          AND
                        </td>
                        <td class="width-80px is-vcentered">
                          Path
                        </td>
                        <td colspan="2">
                          <div class="control is-fullwidth">
                            <input class="input is-small uri-entry-input"
                                  title="Path"
                                  v-model="sequenceItem.uri"
                                  @input="emitDocUpdate"/>
                          </div>
                        </td>
                        <td class="width-80px" />
                      </tr>
                      <tr v-for="(sequenceEntry, sequenceEntryIndex) in sequenceItemEntries(sequenceIndex)"
                          :key="sequenceEntryIndex"
                          class="sequence-entry-row">
                        <td class="is-size-7 width-50px has-text-centered is-vcentered has-text-grey-light
                                  has-text-weight-medium sequence-entries-relation">
                          AND
                        </td>
                        <td class="width-80px is-vcentered">
                          {{ listEntryTypes[sequenceEntry[0]].title }}
                        </td>
                        <td class="width-100px">
                          {{ sequenceEntry[1][0] }}
                        </td>
                        <td>
                          {{ sequenceEntry[1][1] }}
                        </td>
                        <td class="width-80px">
                          <a class="is-small has-text-grey remove-entry-button"
                            title="Remove sequence entry"
                            tabindex="0"
                            @click="removeSequenceItemEntry(
                                sequenceIndex, sequenceEntry[0], sequenceEntry[1][0])"
                            @keypress.space.prevent
                            @keypress.space="removeSequenceItemEntry(
                                sequenceIndex, sequenceEntry[0], sequenceEntry[1][0])"
                            @keypress.enter="removeSequenceItemEntry(
                                sequenceIndex, sequenceEntry[0], sequenceEntry[1][0])">
                            remove
                          </a>
                        </td>
                      </tr>
                      <tr v-if="newEntrySectionIndex !== sequenceIndex">
                        <td>
                          <a class="is-size-7 has-text-grey-lighter add-button add-entry-button"
                            title="add new row"
                            tabindex="0"
                            @click="setNewEntryIndex(sequenceIndex)"
                            @keypress.space.prevent
                            @keypress.space="setNewEntryIndex(sequenceIndex)"
                            @keypress.enter="setNewEntryIndex(sequenceIndex)">
                            <i class="fas fa-plus" />
                          </a>
                          &nbsp;&middot;&nbsp;
                          <a class="is-size-7 has-text-grey-lighter remove-button remove-section-button"
                            title="remove entire section"
                            tabindex="0"
                            @click="removeSequenceItem(sequenceIndex)"
                            @keypress.space.prevent
                            @keypress.space="removeSequenceItem(sequenceIndex)"
                            @keypress.enter="removeSequenceItem(sequenceIndex)">
                            <i class="fas fa-trash" />
                          </a>
                        </td>
                        <td colspan="4" />
                      </tr>
                      <tr v-if="newEntrySectionIndex === sequenceIndex" class="new-entry-row">
                        <td class="is-size-7" colspan="2">
                          <div class="select is-small is-fullwidth">
                            <select v-model="newEntryType"
                                    title="New entry type"
                                    class="select new-entry-type-selection">
                              <option v-for="(entryType, category) in listEntryTypes"
                                      :key="category"
                                      :value="category">
                                {{ entryType.title }}
                              </option>
                            </select>
                          </div>
                        </td>
                        <td class="is-size-7 width-100px">
                          <div class="control has-icons-left is-fullwidth new-entry-name">
                            <input class="input is-small new-entry-name-input"
                                  title="Name"
                                  placeholder="Name"
                                  v-model="newEntryItem.name"/>
                            <span class="icon is-small is-left has-text-grey-light">
                              <i class="fa fa-code" />
                            </span>
                          </div>
                        </td>
                        <td class="is-size-7">
                          <div class="control has-icons-left is-fullwidth new-entry-value">
                            <input class="input is-small new-entry-value-input"
                                  title="Value"
                                  placeholder="Value"
                                  v-model="newEntryItem.value"/>
                            <span class="icon is-small is-left has-text-grey-light">
                              <i class="fa fa-code" />
                            </span>
                          </div>
                        </td>
                        <td class="is-size-7 width-80px">
                          <a class="is-size-7 has-text-grey add-button confirm-add-entry-button"
                            title="add new row"
                            tabindex="0"
                            @click="addSequenceItemEntry(sequenceIndex)"
                            @keypress.space.prevent
                            @keypress.space="addSequenceItemEntry(sequenceIndex)"
                            @keypress.enter="addSequenceItemEntry(sequenceIndex)">
                            <i class="fas fa-check" /> Add
                          </a>
                          <br/>
                          <a class="is-size-7 has-text-grey remove-button"
                            title="cancel add new row"
                            tabindex="0"
                            @click="setNewEntryIndex(-1)"
                            @keypress.space.prevent
                            @keypress.space="setNewEntryIndex(-1)"
                            @keypress.enter="setNewEntryIndex(-1)">
                            <i class="fas fa-times" /> Cancel
                          </a>
                        </td>
                      </tr>
                      </tbody>
                    </table>
                  </div>
                </div>
                <div v-if="localDoc.sequence.length > 1 && sequenceIndex !== localDoc.sequence.length - 1"
                     class="control is-expanded relation-wrapper"
                     :key="`${sequenceIndex}-then`">
                  <span class="tag is-small is-relative">
                    THEN
                  </span>
                </div>
              </template>
              <button class="button is-small new-sequence-button"
                      @click="addSequenceItem()">
                Create new sequence section
              </button>
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
import ResponseAction from '@/components/ResponseAction.vue'
import LimitOption, {OptionObject} from '@/components/LimitOption.vue'
import TagAutocompleteInput from '@/components/TagAutocompleteInput.vue'
import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import Vue from 'vue'
import {
  ArgsCookiesHeadersType, FlowControl, IncludeExcludeType, LimitOptionType, LimitRuleType, DraggableEvent,
} from '@/types'
import {Dictionary} from 'vue-router/types/router'
const {Draggable} = require('@shopify/draggable')

export default Vue.extend({
  name: 'FlowControl',

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
      filters: ['include', 'exclude'] as IncludeExcludeType[],
      addNewTagColName: null,
      titles: DatasetsUtils.titles,
      defaultSequenceItem: {...DatasetsUtils.defaultFlowControlSequenceItem},
      listEntryTypes: {
        'headers': {title: 'Header', pair: true},
        'args': {title: 'Argument', pair: true},
        'cookies': {title: 'Cookie', pair: true},
      },
      keysAreValid: true,
      newEntrySectionIndex: -1,
      newEntryType: 'args' as ArgsCookiesHeadersType,
      newEntryItem: {
        name: '',
        value: '',
      },
      hoverSequenceIndex: undefined,
      dropTarget: undefined as HTMLElement,
    }
  },

  computed: {
    localDoc(): FlowControl {
      return _.cloneDeep(this.selectedDoc)
    },

    duplicateTags(): Dictionary<string> {
      const doc = this.localDoc
      const allTags = _.concat(doc['include'], doc['exclude'])
      const dupTags = _.filter(allTags, (val, i, iteratee) => _.includes(iteratee, val, i + 1))
      return _.fromPairs(_.zip(dupTags, dupTags))
    },
  },

  methods: {
    emitDocUpdate() {
      this.$emit('update:selectedDoc', this.localDoc)
    },

    getOptionTextKey(option: LimitOptionType, index: number) {
      const [type] = Object.keys(option)
      return `${this.localDoc.id}_${type}_${index}`
    },

    generateOption(data: LimitOptionType): OptionObject {
      const [firstObjectKey] = Object.keys(data)
      const type = firstObjectKey as LimitRuleType
      const key = (data[firstObjectKey] || null)
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

    // Sequence

    setNewEntryIndex(index: number) {
      this.newEntryItem = {
        name: '',
        value: '',
      }
      this.newEntryType = 'args'
      this.newEntrySectionIndex = index
    },

    sequenceItemEntries(sequenceIndex: number) {
      const sequenceItem = this.localDoc.sequence[sequenceIndex]
      const headersEntries = Object.entries(sequenceItem.headers)
      const cookiesEntries = Object.entries(sequenceItem.cookies)
      const argsEntries = Object.entries(sequenceItem.args)
      const mergedEntries = []
      for (let i = 0; i < headersEntries.length; i++) {
        if (headersEntries[i][0] !== 'host') {
          mergedEntries.push(['headers', headersEntries[i]])
        }
      }
      for (let i = 0; i < argsEntries.length; i++) {
        mergedEntries.push(['args', argsEntries[i]])
      }
      for (let i = 0; i < cookiesEntries.length; i++) {
        mergedEntries.push(['cookies', cookiesEntries[i]])
      }
      return mergedEntries
    },

    addSequenceItem() {
      if (this.localDoc.sequence.length > 0) {
        const firstSequenceItem = this.localDoc.sequence[0]
        this.defaultSequenceItem.headers.host = firstSequenceItem.headers.host
      }
      this.localDoc.sequence.push(this.defaultSequenceItem)
      this.emitDocUpdate()
    },

    removeSequenceItem(sequenceIndex: number) {
      this.localDoc.sequence.splice(sequenceIndex, 1)
      this.emitDocUpdate()
    },

    addSequenceItemEntry(sequenceIndex: number) {
      const sequenceItem = this.localDoc.sequence[sequenceIndex]
      const newEntryName = this.newEntryItem.name.trim().toLowerCase()
      const newEntryValue = this.newEntryItem.value.trim().toLowerCase()
      if (newEntryName && newEntryValue &&
          !Object.prototype.hasOwnProperty.call(sequenceItem[this.newEntryType], newEntryName)) {
        sequenceItem[this.newEntryType][newEntryName] = newEntryValue
      }
      this.setNewEntryIndex(-1)
      this.emitDocUpdate()
    },

    removeSequenceItemEntry(sequenceIndex: number, type: ArgsCookiesHeadersType, key: any) {
      const sequenceItem = this.localDoc.sequence[sequenceIndex]
      delete sequenceItem[type][key]
      this.emitDocUpdate()
    },

    // Tags filters

    addNewTag(section: IncludeExcludeType, entry: string) {
      if (entry && entry.length > 2) {
        this.localDoc[section].push(entry)
        this.emitDocUpdate()
      }
    },

    openTagInput(section: IncludeExcludeType) {
      this.addNewTagColName = section
    },

    cancelAddNewTag() {
      this.addNewTagColName = null
    },

    removeTag(section: IncludeExcludeType, index: number) {
      this.localDoc[section].splice(index, 1)
      this.addNewTagColName = null
      this.emitDocUpdate()
    },

    onDragStart(evt: DraggableEvent) {
      const target = evt.originalEvent.target as HTMLElement
      if (!target.closest('div').classList.contains('dragging-icon')) {
        evt.cancel()
      }
    },

    setHoverSequenceIndex(index?: number) {
      this.hoverSequenceIndex = index
    },

    setDropTarget({data}: DraggableEvent) {
      this.dropTarget = data.over
    },

    swapSequenceItems({data}: DraggableEvent) {
      const {sequence} = this.localDoc
      const oldIndex = parseInt(data.source?.getAttribute('data-index'))
      const newIndex = parseInt(this.dropTarget?.getAttribute('data-index'))
      const oldElement = sequence[oldIndex]
      sequence.splice(oldIndex, 1)
      sequence.splice(newIndex, 0, oldElement)
      this.setHoverSequenceIndex(newIndex)
      this.emitDocUpdate()
    },
  },

  mounted() {
    new Draggable(this.$refs.sequences, {
      draggable: '.sequence',
      classes: {'body:dragging': 'is-dragging'},
      mirror: {constrainDimensions: true}, // keeps original sizes of draggable element while dragging
    }).on(
      'drag:start', this.onDragStart,
    ).on(
      'drag:over', this.setDropTarget,
    ).on(
      'drag:stop', this.swapSequenceItems,
    )
  },
})
</script>

<style scoped lang="scss">

.bar {
  margin: 1rem 0 0.5rem;
}

.sequence.hover,
.sequence.hover:active {
  background: #fff;
  box-shadow: 0 3px 10px rgb(0 0 0 / 0.2);

  table {
    background: #fff;
  }
}

.dragging-icon {
  cursor: grab;
  display: inline-block;
  opacity: 0.7;
  width: calc(1rem + 14px);
}

.dragging-icon:active {
  cursor: grab;
}

.sequence-wrapper.draggable-container--is-dragging .sequence {
  $placetomove-color: #f9f9fa;
  $placetodrop-color: #edeef0;
  $selected-border-color: #014ac6;

  &.draggable-source--is-dragging {
    border: 1px dashed $selected-border-color;
    box-shadow: none;
    opacity: 0.3;
  }

  &.draggable-source--is-dragging .dragging-icon {
    opacity: 0;
  }

  &:not(.draggable-source--is-dragging) {
    background: $placetomove-color;
    border-color: $selected-border-color;
  }

  &:not(.draggable-source--is-dragging) .sequence-entries-table {
    background: $placetomove-color;
  }

  &.draggable--over:not(.draggable-source--is-dragging) {
    background-color: $placetodrop-color;
    box-shadow: 0 0 0 1px #fff, 0 0 0 2px $selected-border-color;
    outline: none;
  }

  &.draggable--over:not(.draggable-source--is-dragging) table {
    background-color: $placetodrop-color;
  }
}

.is-dragging * {
  cursor: grabbing;
}

.sequence-entries {
  margin-bottom: 0.75rem;
}

.sequence-entries-relation {
  margin-bottom: 1rem;
}

.sequence-entries-table .select,
.sequence-entries-table select {
  width: 100%;
}

.relation-wrapper {
  margin-bottom: 1rem;
  text-align: center;
}

.relation-wrapper::before {
  background: hsl(0, 0%, 0%);
  border-top: 1px solid hsl(0, 0%, 0%);
  content: '';
  left: 0;
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 100%;
}

.filter-columns {
  margin-top: 20px;
}

.seconds-suffix {
  input {
    padding-right: 60px;
  }
}

::v-deep .tag-input {
  font-size: 0.58rem;
}

</style>
