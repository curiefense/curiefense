<template>

  <div class="dropdown"
       :class="{'is-active': suggestionsVisible}">
    <div class="dropdown-trigger">
      <input v-model="autocompleteValue"
             :title="title"
             :placeholder="title"
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
             @blur="inputBlurred"
             ref="autocompleteInput"/>
    </div>
    <div class="dropdown-menu"
         id="dropdown-menu"
         role="menu">
      <div class="dropdown-content">
        <a v-for="(suggestion, index) in matches"
           :class="{'is-active': isSuggestionFocused(index)}"
           @mousedown="suggestionClick(index)"
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
import Utils from '@/assets/Utils.ts'
import Vue, {PropType, VueConstructor} from 'vue'

export type AutocompleteSuggestion = {
  prefix?: string
  value: string
}

export type AutocompleteInputEvents = 'keyup' | 'keydown' | 'keypress' | 'focus' | 'blur'

export default (Vue as VueConstructor<Vue & {
  $refs: {
    autocompleteInput: HTMLInputElement
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
    // Minimum characters length allowed for the value
    minimumValueLength: {
      type: Number,
      default: 0,
    },
    title: {
      type: String,
      default: 'Value',
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
      inputBlurredTimeout: null,
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
          values[values.length - 1] = currentValue
          this.autocompleteValue = values.join(' ')
        } else {
          this.autocompleteValue = currentValue.trim()
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
      this.clearInputBlurredTimeout()
      this.focusedSuggestionIndex = index
      this.selectValue(!this.autoFocus)
      if (this.autoFocus) {
        // Putting the focus at the end of the queue so the suggestion focus event would finish beforehand
        setImmediate(() => {
          this.$refs.autocompleteInput.focus()
        })
      }
    },

    async selectValue(skipFocus?: boolean) {
      if (this.focusedSuggestionIndex !== -1) {
        this.currentValue = this.matches[this.focusedSuggestionIndex].value
      }
      if (this.currentValue.length < this.minimumValueLength) {
        if (!this.currentValue.length) {
          return
        }
        Utils.toast(
            `Selected tag "${this.currentValue}" is invalid!\n` +
            `Tags must be at least ${this.minimumValueLength} characters long.`,
            'is-danger',
        )
        this.currentValue = ''
      } else {
        this.valueSubmitted()
        this.valueChanged()
      }
      this.focusedSuggestionIndex = -1
      if (!skipFocus) {
        this.$refs.autocompleteInput.focus()
      }
      this.closeDropdown()
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

    inputBlurred() {
      // We would like to cancel and skip the selection if one of the following occoured:
      // * The blur is due to a suggestion click
      // * The component is destroyed before we finish selecting
      this.inputBlurredTimeout = setTimeout(() => {
        this.selectValue(true)
      }, 0)
    },

    clearInputBlurredTimeout() {
      clearTimeout(this.inputBlurredTimeout)
    },

  },

  destroyed() {
    this.clearInputBlurredTimeout()
  },
})
</script>

<style scoped lang="scss">
.dropdown,
.dropdown-trigger,
.dropdown-menu {
  width: 100%;
}
</style>
