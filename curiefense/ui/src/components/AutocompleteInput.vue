<template>

  <div class="dropdown"
       :class="{'is-active': suggestionsVisible}">
    <div class="dropdown-trigger">
      <input v-model="autocompleteValue"
             title="Value"
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
import Vue, {PropType, VueConstructor} from 'vue'

export type AutocompleteSuggestion = {
  prefix: string
  value: string
}

export type AutocompleteInputEvents = 'keyup' | 'keydown' | 'keypress' | 'focus' | 'blur'

export default (Vue as VueConstructor<Vue & {
  $refs: {
    autocompleteInput: InstanceType<typeof HTMLInputElement>
  }
}>).extend({
  name: 'AutocompleteInput',

  props: {
    initialValue: {
      type: String,
      default: '',
    },
    suggestions: {
      type: Array as PropType<AutocompleteSuggestion[]>,
      default: (): AutocompleteSuggestion[] => [],
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
      default: 'single',
    },
  },

  watch: {
    initialValue: function(newVal) {
      if (this.autocompleteValue !== newVal) {
        this.autocompleteValue = newVal
        this.closeDropdown()
      }
    },
  },

  mounted() {
    const events: AutocompleteInputEvents[] = ['keyup', 'keydown', 'keypress', 'focus', 'blur']
    events.map((event) => {
      this.$refs.autocompleteInput.addEventListener(event,
          ($event: Event): void => {
            this.$emit(event, $event)
          })
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
    matches(): AutocompleteSuggestion[] {
      return this.suggestions?.filter((suggestion: AutocompleteSuggestion) => {
        return suggestion.value.toLowerCase().includes(this.currentValue.toLowerCase())
      })
    },

    suggestionsVisible(): boolean {
      return this.currentValue !== '' && this.matches?.length !== 0 && this.open
    },

    currentValue: {
      get: function(): string {
        let currentValue
        if (this?.selectionType.toLowerCase() === 'multiple') {
          const values = this.autocompleteValue.split(' ')
          currentValue = values[values.length - 1].trim()
        } else {
          currentValue = this.autocompleteValue.trim()
        }
        return currentValue
      },
      set: function(currentValue: string) {
        if (this.selectionType.toLowerCase() === 'multiple') {
          const values = this.autocompleteValue.split(' ')
          values[values.length - 1] = currentValue;
          this.autocompleteValue = values.join(' ')
        } else {
          this.autocompleteValue = (currentValue as any).trim()
        }
      },
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

    closeDropdown(): void {
      this.open = false
    },

    suggestionClick(index: number) {
      this.focusedSuggestionIndex = index
      this.selectValue()
    },

    async selectValue() {
      if (this.focusedSuggestionIndex !== -1) {
        this.currentValue = this.matches[this.focusedSuggestionIndex].value
      }
      this.valueSubmitted()
      this.valueChanged()
      this.focusedSuggestionIndex = -1;
      this.$refs.autocompleteInput.focus()
      this.open = false
      if (this.clearInputAfterSelection) {
        this.autocompleteValue = ''
      }
    },

    focusPreviousSuggestion() {
      if (this.focusedSuggestionIndex > -1) {
        this.focusedSuggestionIndex--
      }
    },

    focusNextSuggestion() {
      if (this.focusedSuggestionIndex < this.matches.length - 1) {
        this.focusedSuggestionIndex++
      }
    },

    isSuggestionFocused(index: number) {
      return index === this.focusedSuggestionIndex
    },

  },
})
</script>

<style scoped>
.dropdown, .dropdown-trigger, .dropdown-menu {
  width: 100%
}
</style>
