<template>

  <div class="dropdown"
       :class="{'is-active': suggestionsVisible}">
    <div class="dropdown-trigger">
      <input v-model="autocompleteValue"
             type="text"
             class="autocomplete-input input is-small"
             aria-haspopup="true"
             aria-controls="dropdown-menu"
             @keydown.enter="selectValue"
             @keydown.space="selectValue"
             @keydown.down='focusNextSuggestion'
             @keydown.up='focusPreviousSuggestion'
             @keydown.esc='closeDropdown'
             @input="openDropdown(); valueChanged()"
             ref="autocompleteInput"/>
    </div>
    <div class="dropdown-menu"
         id="dropdown-menu"
         role="menu">
      <div class="dropdown-content">
        <a v-for="(suggestion, index) in matches"
           :class="{'is-active': isSuggestionFocused(index)}"
           @click="suggestionClick(index)"
           :key="index"
           class="dropdown-item">
          <span v-if="suggestion.prefix" v-html="suggestion.prefix"></span>
          {{ suggestion.value }}
        </a>
      </div>
    </div>
  </div>

</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({
  name: 'AutocompleteInput',

  props: {
    initialValue: {
      type: String,
      default: ''
    },
    suggestions: {
      type: Array,
      default: () => []
    },
    clearInputAfterSelection: Boolean,
    autoFocus: Boolean,
    selectionType: {
      type: String,
      validator(val) {
        if (!val) {
          return false
        }
        return ['single', 'multiple'].includes(val.toLowerCase())
      },
      default: 'single'
    }
  },

  watch: {
    initialValue: function (newVal) {
      if (this.autocompleteValue !== newVal) {
        this.autocompleteValue = newVal
        this.closeDropdown()
      }
    }
  },

  mounted() {
    ['keyup', 'keydown', 'keypress', 'focus', 'blur'].map(event => {
      this.$refs.autocompleteInput.addEventListener(event, $event => this.$emit(event, $event))
    })
    if (this.autoFocus) {
      this.$refs.autocompleteInput.focus()
    }
  },

  data() {
    return {
      autocompleteValue: this.initialValue,
      open: false,
      focusedSuggestionIndex: -1,
    }
  },

  computed: {

    // Filtering the suggestions based on the input
    matches() {
      return this.suggestions?.filter((str) => {
        return str.value.toLowerCase().includes(this.currentValue.toLowerCase())
      })
    },

    suggestionsVisible() {
      return this.currentValue !== '' && this.matches?.length !== 0 && this.open
    },

    currentValue: {
      get: function () {
        let currentValue
        if (this?.selectionType.toLowerCase() === 'multiple') {
          const values = this.autocompleteValue.split(' ')
          currentValue = values[values.length - 1].trim()
        } else {
          currentValue = this.autocompleteValue.trim()
        }
        return currentValue
      },
      set: function (currentValue) {
        if (this.selectionType.toLowerCase() === 'multiple') {
          const values = this.autocompleteValue.split(' ')
          values[values.length - 1] = currentValue
          this.autocompleteValue = values.join(' ')
        } else {
          this.autocompleteValue = currentValue.trim()
        }
      }
    },

  },

  methods: {

    openDropdown() {
      this.open = true
    },

    valueChanged() {
      this.$emit('value-changed', this.autocompleteValue)
    },

    valueSubmitted() {
      this.$emit('value-submitted', this.autocompleteValue)
    },

    closeDropdown() {
      this.open = false
    },

    suggestionClick(index) {
      this.focusedSuggestionIndex = index
      this.selectValue()
    },

    async selectValue() {
      if (this.focusedSuggestionIndex !== -1) {
        this.currentValue = this.matches[this.focusedSuggestionIndex].value
      }
      this.valueSubmitted()
      this.valueChanged()
      this.focusedSuggestionIndex = -1
      this.$refs.autocompleteInput.focus()
      this.open = false
      if (this.clearInputAfterSelection) {
        this.autocompleteValue = ''
      }
    },

    focusPreviousSuggestion() {
      if (this.focusedSuggestionIndex > -1)
        this.focusedSuggestionIndex--
    },

    focusNextSuggestion() {
      if (this.focusedSuggestionIndex < this.matches.length - 1)
        this.focusedSuggestionIndex++
    },

    isSuggestionFocused(index) {
      return index === this.focusedSuggestionIndex
    },

  }
})
</script>

<style scoped>
.dropdown, .dropdown-trigger, .dropdown-menu {
  width: 100%
}
</style>
