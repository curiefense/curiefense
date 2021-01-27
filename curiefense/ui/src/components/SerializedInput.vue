<template>
  <div>
    <input required
           type="text"
           class="serialized-input input is-small"
           v-model="formattedValue"
           @blur="updateValue($event.target.value)"
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
    setFunction: Function
  },
  data() {
    return {
      formattedValue: ''
    }
  },
  watch: {
    value: function () {
      this.formatValue()
    }
  },
  mounted: function () {
    this.formatValue()
  },
  methods: {
    updateValue: function (newValue) {
      if (typeof this.setFunction === 'function') {
        const result = this.setFunction(newValue)
        this.$emit('blur', result)
      } else {
        console.log('SerializedInput setFunction prop provided is not a function!')
      }
    },
    formatValue: function () {
      if (typeof this.getFunction === 'function') {
        this.formattedValue = this.getFunction(this.value)
      } else {
        console.log('SerializedInput getFunction prop provided is not a function!')
      }
    }
  }
})
</script>
