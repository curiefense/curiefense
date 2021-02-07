<template>
  <div>
    <input required
           type="text"
           class="serialized-input input is-small"
           v-model="formattedValue"
           @blur="updateValue($event.target.value)"
           :title="placeholder"
           :placeholder="placeholder"/>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({
  name: 'SerializedInput',
  props: {
    value: null,
    placeholder: String,
    getFunction: Function,
    setFunction: Function,
  },
  data() {
    return {
      formattedValue: '',
    }
  },
  watch: {
    value: function() {
      this.formatValue()
    },
  },
  mounted: function() {
    this.formatValue()
  },
  methods: {
    updateValue: function(newValue: string) {
      const result = this.setFunction(newValue)
      this.$emit('blur', result)
    },
    formatValue: function() {
      this.formattedValue = this.getFunction(this.value)
    },
  },
})
</script>
