<template>
  <div class="limit-options">
    <label v-if="labelSeparatedLine && label"
           class="label is-small is-size-7 has-text-left form-label">
      {{ label }}
    </label>
    <div class="columns mb-0">
      <div v-if="!labelSeparatedLine && label"
           class="column is-2">
        <label class="label is-small is-size-7 form-label">
          {{ label }}
        </label>
      </div>
      <div class="column">
        <div class="control select is-small is-fullwidth">
          <select v-model="selectedType"
                  class="option-type-selection"
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
                 class="input is-small option-key-input">
          <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
        </div>
        <div class="control select is-small is-fullwidth" v-if="selectedType === 'attrs'">
          <div class="select is-fullwidth">
            <select v-model="selectedKey"
                    class="option-attribute-selection"
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
                 class="input is-small option-value-input">
          <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
        </div>
      </div>
      <div class="column is-narrow"
           v-if="!!showRemove">
        <button
            :class="['button', 'is-light', 'is-small', 'remove-icon', 'is-small',
                    removable ? 'has-text-grey' : 'has-text-grey-light is-disabled']"
            :disabled="!removable"
            class="remove-option-button"
            title="Click to remove"
            @click="$emit('remove')">
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
import {LimitRuleType} from '@/types'

export type OptionObject = {
  type?: LimitRuleType
  key?: string
  value?: string
  oldKey?: string
}

export const limitAttributes = {
  'ip': 'IP Address',
  'asn': 'Provider',
  'uri': 'URI',
  'path': 'Path',
  'tags': 'Tag',
  'query': 'Query',
  'method': 'Method',
  'company': 'Company',
  'country': 'Country',
  'authority': 'Authority',
}

export default Vue.extend({
  name: 'LimitOption',
  props: {
    label: {
      type: String,
      default: '',
    },
    option: {
      type: Object as PropType<OptionObject>,
      default: (): OptionObject => {
        return {
          type: 'attrs' as OptionObject['type'],
          key: '',
        } as OptionObject
      },
    },
    removable: {
      type: Boolean,
      default: false,
    },
    showRemove: {
      type: Boolean,
      default: false,
    },
    useDefaultSelf: {
      type: Boolean,
      default: false,
    },
    useValue: {
      type: Boolean,
      default: false,
    },
    labelSeparatedLine: {
      type: Boolean,
      default: false,
    },
    ignoreAttributes: {
      type: Array,
      default: () => {
        return [] as string[]
      },
    },
  },
  data() {
    const limitOptionsTypes = DatasetsUtils.limitOptionsTypes
    const optionsData: { [key: string]: OptionObject } = {
      self: {
        type: 'self',
        key: 'self',
      },
    }
    const attributes = _.pickBy(limitAttributes, (value, key) => {
      return !this.ignoreAttributes || !this.ignoreAttributes.includes(key)
    })
    Object.keys(limitOptionsTypes).forEach((ruleType) => {
      const {type, key = '', value} = this.option
      optionsData[ruleType] = {type, key, value}
    })
    return {
      optionsData,
      options: limitOptionsTypes,
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
      get: function(): LimitRuleType {
        return this.selectedOption.type
      },
      set: function(value: LimitRuleType): void {
        this.type = value
        this.selectedOption.type = value
      },
    },
    selectedOption(): OptionObject {
      return this.optionsData[this.type]
    },
  },
  updated() {
    this.$emit('change', {...this.selectedOption})
  },
  methods: {
    isCategoryArgsCookiesHeaders(limitRuleType: LimitRuleType) {
      return (new RegExp('(args|cookies|headers)')).test(limitRuleType)
    },
  },
})
</script>
