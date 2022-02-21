<template>

  <div class="dropdown"
       :class="{'is-active': suggestionsVisible}">
    <div class="dropdown-trigger">
      <textarea v-if="inputType === 'textarea'"
                :value="autocompleteTextareaValue"
                :title="title"
                :placeholder="title"
                class="autocomplete-input textarea is-small"
                @keydown.enter="selectTextareaValue"
                @keydown.space.prevent
                @keydown.down="focusNextSuggestion"
                @keydown.up="focusPreviousSuggestion"
                @keydown.esc="closeDropdown"
                @keydown.delete.prevent="onTextareaDelete"
                @input="onInput"
                @blur="inputBlurred"
                @focus="onTextareaFocus"
                @mousedown.prevent="moveCursorToEnd"
                ref="autocompleteInput" />
      <input v-else
             v-model="autocompleteValue"
             :title="title"
             :placeholder="title"
             type="text"
             class="autocomplete-input input is-small"
             aria-haspopup="true"
             aria-controls="dropdown-menu"
             @keydown.enter="selectValue"
             @keydown.space="selectValue"
             @keydown.down="focusNextSuggestion"
             @keydown.up="focusPreviousSuggestion"
             @keydown.esc="closeDropdown"
             @input="openDropdown(); valueChanged()"
             @blur="inputBlurred"
             ref="autocompleteInput" />
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
  },
  divider: string,
  autocompleteValue: string,
}>).extend({
  name: 'AutocompleteInput',

  props: {
    inputType: {
      type: String,
      default: 'input',
      validator: (val: string) => ['input', 'textarea'].includes(val),
    },
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
    filterFunction: Function,
  },

  watch: {
    initialValue(newVal) {
      const newValFiltered = this.filterFunction ? this.filterFunction(newVal) : newVal
      if (this.autocompleteValue !== newVal) {
        this.autocompleteValue = newValFiltered
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
    const {filterFunction, initialValue} = this
    return {
      autocompleteValue: filterFunction ? filterFunction(initialValue) : initialValue,
      open: false,
      focusedSuggestionIndex: -1,
      inputBlurredTimeout: null,
      divider: this.inputType === 'textarea' ? '\n' : ' ',
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
      return this.currentValue !== '' && this.matches?.length && this.open
    },

    currentValue: {
      get(): string {
        let currentValue
        if (this?.selectionType.toLowerCase() === 'multiple') {
          const values = this.autocompleteValue.split(this.divider)
          currentValue = values[values.length - 1]
        } else {
          currentValue = this.autocompleteValue
        }
        return currentValue.replace('•', '').trim()
      },
      set(currentValue: string) {
        if (this.selectionType.toLowerCase() === 'multiple') {
          const values = this.autocompleteValue.split(this.divider)
          values[values.length - 1] = currentValue
          this.autocompleteValue = values.join(this.divider)
        } else {
          this.autocompleteValue = currentValue
        }
      },
    },

    autocompleteTextareaValue() {
      return this.autocompleteValue.split(this.divider).map(
        (val: string) => {
          val = val.trim()
          return val ? `• ${val.replace('• ', '')}` : val
        },
      ).join(this.divider)
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
      if (this.filterFunction) {
        this.autocompleteValue = this.filterFunction(this.autocompleteValue)
      }
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
      if (this.inputType === 'textarea') {
        this.autocompleteValue = `${this.autocompleteValue.trim()}${this.divider}`
      }
    },

    moveCursorToEnd(event: KeyboardEvent) {
      const element = event.target as HTMLTextAreaElement
      element.focus()
      element.setSelectionRange(element.value.length, element.value.length)
    },

    selectTextareaValue(event: KeyboardEvent) {
      if (event.key === 'Enter' && this.autocompleteValue.endsWith(this.divider)) {
        event.preventDefault()
      }
      if (!(event.target as HTMLTextAreaElement).value) {
        event.preventDefault()
        return
      }
      this.selectValue()
    },

    onTextareaFocus() {
      if (this.autocompleteValue.trim()) {
        this.autocompleteValue = `${this.autocompleteValue.trim()}${this.divider}`
      }
    },

    onTextareaDelete() {
      const valueArray = this.autocompleteValue.trim().split(this.divider)
      valueArray.splice(-1)
      this.autocompleteValue = valueArray.join(this.divider)
      this.valueSubmitted()
    },

    onInput({target}: KeyboardEvent) {
      this.autocompleteValue = (target as HTMLTextAreaElement).value
      this.openDropdown()
      this.valueSubmitted()
    },

    selectValue(skipFocus?: boolean) {
      if (this.focusedSuggestionIndex !== -1) {
        this.currentValue = this.matches[this.focusedSuggestionIndex].value
      }
      if (this.currentValue.length < this.minimumValueLength) {
        if (!this.currentValue.length) {
          return
        }
        Utils.toast(
          `Selected value "${this.currentValue}" is invalid!\n` +
          `Values must be at least ${this.minimumValueLength} characters long.`,
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
