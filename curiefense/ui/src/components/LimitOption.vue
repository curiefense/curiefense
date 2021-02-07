<template>
  <div class="limit-options">
    <label v-if="labelSeparatedLine && label"
           class="label is-small is-size-7 has-text-left">
      {{ label }}
    </label>
    <div class="columns mb-0"
         :class="{'wide-columns': wideColumns}">
      <div v-if="!labelSeparatedLine && label"
           class="column is-2">
        <label class="label is-small is-size-7">
          {{ label }}
        </label>
      </div>
      <div class="column">
        <div class="control select is-small is-fullwidth">
          <select v-model="selectedType"
                  title="Type">
            <option v-if="useDefaultSelf" value="self">HTTP request</option>
            <option v-for="(value, id) in options" :selected="value === selectedType" :value="id" :key="id">
              {{ value }}
            </option>
          </select>
        </div>
      </div>
      <div class="column" v-if="selectedType !== 'self'">
        <div v-if="isCategoryArgsCookiesHeaders(selectedType)"
             :class="{control: true, 'is-fullwidth': true}"
             class="has-icons-left">
          <input type="text"
                 title="Key"
                 v-model="selectedKey"
                 class="input is-small">
          <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
        </div>
        <div class="control select is-small is-fullwidth" v-if="selectedType === 'attrs'">
          <div class="select is-fullwidth">
            <select v-model="selectedKey"
                    title="Key">
              <option v-for="(value, id) in attributes" :value="id" :key="id">{{ value }}</option>
            </select>
          </div>
        </div>
      </div>
      <div class="column" v-if="useValue">
        <div class="control has-icons-left is-fullwidth">
          <input type="text"
                 title="Value"
                 v-model="selectedValue"
                 class="input is-small">
          <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
        </div>
      </div>
      <div class="column is-narrow"
           v-if="!!showRemove">
        <button
            :class="['button', 'is-light', 'is-small', 'remove-icon', 'is-small',
                    removable ? 'has-text-grey' : 'has-text-grey-light is-disabled']"
            :disabled="!removable"
            title="click to remove"
            @click="$emit('remove', index)">
          <span class="icon is-small"><i class="fas fa-trash fa-xs"></i></span>
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import _ from 'lodash'
import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import Vue, {PropType} from 'vue'
import {Category, LimitRuleType} from '@/types'

export type OptionObject = {
  type?: LimitRuleType
  key?: string
  value?: string
  oldKey?: string
}

export default Vue.extend({
  name: 'LimitOption',
  props: {
    label: String,
    option: Object as PropType<OptionObject>,
    index: Number,
    removable: Boolean,
    showRemove: Boolean,
    useDefaultSelf: Boolean,
    useValue: Boolean,
    labelSeparatedLine: Boolean,
    wideColumns: Boolean,
    disabledOptions: Array,
    ignoreAttributes: Array,
  },
  data() {
    const {LimitRulesTypes, LimitAttributes} = DatasetsUtils
    const optionsData: { [key: string]: OptionObject } = {
      self: {
        type: 'self',
        key: 'self',
      },
    }
    const attributes = _.pickBy(LimitAttributes, (value, key) => {
      return !this.ignoreAttributes || !this.ignoreAttributes.includes(key)
    })
    Object.keys(LimitRulesTypes).forEach((ruleType) => {
      const {type, key = '', value} = this.option || {}
      optionsData[ruleType] = {type, key, value}
    })
    return {
      optionsData,
      options: LimitRulesTypes,
      attributes: attributes,
      type: this.option?.type || 'attrs',
    }
  },
  computed: {
    selectedValue: {
      get: function(): string {
        return this.selectedOption.value
      },
      set: function(value: string): void {
        this.selectedOption.value = value
      },
    },
    selectedKey: {
      get: function(): string {
        return this.selectedOption.key
      },
      set: function(value: string): void {
        this.selectedOption.oldKey = this.selectedOption.key
        this.selectedOption.key = value
      },
    },
    selectedType: {
      get: function(): string {
        return this.selectedOption.type
      },
      set: function(value: LimitRuleType): void {
        this.type = value
        this.selectedOption.type = value
      },
    },
    selectedOption: {
      get: function(): OptionObject {
        return this.optionsData[this.type]
      },
      set: function(value: OptionObject): void {
        this.optionsData[this.type] = value
      },
    },
  },
  updated() {
    this.$emit('change', {...this.selectedOption}, this.index)
  },
  methods: {
    isCategoryArgsCookiesHeaders(category: Category) {
      return (new RegExp('(args|cookies|headers)')).test(category)
    },
  },
})
</script>
<style scoped lang="scss">
.wide-columns {
  & .column:first-child {
    padding-left: 0
  }

  & .column:last-child {
    padding-right: 0
  }
}
</style>
