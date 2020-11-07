<template>
  <div class="limit-options">
    <div class="columns mb-0">
      <div class="column has-text-right" v-if="label">
        <label class="is-small is-size-7 has-text-weight-bold">{{ label }}</label>
      </div>
      <div class="column">
        <div class="control select is-small is-fullwidth">
          <select v-model="selectedType">
            <option v-if="useDefaultSelf" value="self">HTTP request</option>
            <option v-for="(value, id) in options" :selected="value === selectedType" :value="id" :key="id">{{ value }}</option>
          </select>
        </div>
      </div>
      <div class="column" v-if="selectedType !== 'self'">
        <div v-if="selectedType === 'headers'" :class="{control: true, 'is-fullwidth': true, 'has-icons-left': !useValue}">
          <input type="text" v-model="selectedKey" class="input is-small">
          <span v-if="!useValue" class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
        </div>
        <div v-if="selectedType === 'cookies'" :class="{control: true, 'is-fullwidth': true, 'has-icons-left': !useValue}">
          <input type="text" v-model="selectedKey" class="input is-small">
          <span v-if="!useValue" class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
        </div>
        <div v-if="selectedType === 'args'" :class="{control: true, 'is-fullwidth': true, 'has-icons-left': !useValue}">
          <input type="text" v-model="selectedKey" class="input is-small">
          <span v-if="!useValue" class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
        </div>
        <div class="control select is-small is-fullwidth" v-if="selectedType === 'attrs'">
          <div class="select is-fullwidth">
            <select v-model="selectedKey">
              <option v-for="(value, id) in attributes" :value="id" :key="id">{{ value }}</option>
            </select>
          </div>
        </div>
      </div>
      <div class="column" v-if="useValue">
        <div class="control has-icons-left is-fullwidth">
          <input type="text" v-model="selectedValue" class="input is-small">
          <span class="icon is-small is-left has-text-grey-light"><i class="fa fa-code"></i></span>
        </div>
      </div>
      <div class="column is-1">
        <button
          :class="['button', 'is-light', 'is-small', 'remove-icon', 'is-small', removable ? 'has-text-grey' : 'has-text-grey-light', removable ? '' : 'is-disabled']"
          :disabled="!removable"
          title="click to remove"
          @click="$emit('remove', index)"
          v-if="!!showRemove"
        >
          <span class="icon is-small"><i class="fas fa-trash fa-xs"></i></span>
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "LimitOption",
  props: {
    label: String,
    option: Object,
    index: Number,
    removable: Boolean,
    showRemove: Boolean,
    useDefaultSelf: Boolean,
    useValue: Boolean,
    disabledOptions: Array
  },
  data() {
    const { LimitRulesTypes, LimitAttributes } = this.$root.dsutils
    const optionsData = {
      self: {
        type: 'self',
        key: 'self'
      }
    }
    Object.keys(LimitRulesTypes).forEach(ruleType => {
      const { type, key = '', value } = this.option || {}
      optionsData[ruleType] = { type, key, value }
    })
    return {
      optionsData,
      options: LimitRulesTypes,
      attributes: LimitAttributes,
      type: this.option.type || 'attrs'
    }
  },
  computed: {
    selectedValue: {
      get() {
        return this.selectedOption.value
      },
      set(value) {
        this.selectedOption.value = value
      },
    },
    selectedKey: {
      get() {
        return this.selectedOption.key
      },
      set(value) {
        this.selectedOption.oldKey = this.selectedOption.key
        this.selectedOption.key = value
      },
    },
    selectedType: {
      get() {
        return this.selectedOption.type
      },
      set(value) {
        this.type = value
        this.selectedOption.type = value
      },
    },
    selectedOption: {
      get() {
        return this.optionsData[this.type]
      },
      set(value) {
        this.optionsData[this.type] = value
      }
    }
  },
  updated() {
    this.$emit('change', { ...this.selectedOption }, this.index)
  }
}
</script>
