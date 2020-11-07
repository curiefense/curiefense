<template>

  <div class="dropdown"
       :class="{'is-active': suggestionsVisible}">
    <div class="dropdown-trigger">
      <input v-model="tag"
             type="text"
             class="tag-input input is-small"
             aria-haspopup="true"
             aria-controls="dropdown-menu"
             @keyup.enter="selectTag"
             @keyup.down='focusPreviousSuggestion'
             @keyup.up='focusNextSuggestion'
             @keyup.esc='closeDropdown'
             @input="openDropdown(); tagChanged()"
             ref="tagInput"/>
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
          {{ suggestion }}
        </a>
      </div>
    </div>
  </div>

</template>

<script>

import DatasetsUtils from "@/assets/DatasetsUtils";
import axios from "axios";

export default {
  name: 'TagAutocompleteInput',

  props: {
    initialTag: {
      type: String,
      default: ''
    },
    clearInputAfterSelection: Boolean,
    autoFocus: Boolean,
    selectionType: {
      type: String,
      validator(val) {
        return ['single', 'multiple'].includes(val)
      }
    }
  },

  watch: {
    initialTag: function (newVal) {
      if (this.tag !== newVal) {
        this.tag = newVal
        this.closeDropdown()
      }
    }
  },

  mounted() {
    ['keyup', 'keydown', 'keypress', 'focus', 'blur'].map(event => {
      this.$refs.tagInput.addEventListener(event, $event => this.$emit(event, $event));
    });
    if (this.autoFocus) {
      this.$refs.tagInput.focus()
    }
  },

  data() {
    return {
      tag: this.initialTag,
      open: false,
      tagsSuggestions: [],
      focusedSuggestionIndex: -1,
      db: 'system',
      key: 'autocomplete',

      apiRoot: DatasetsUtils.ConfAPIRoot,
      apiVersion: DatasetsUtils.ConfAPIVersion,
    }
  },

  computed: {

    // Filtering the tags based on the input
    matches() {
      return this.tagsSuggestions?.filter((str) => {
        return str.includes(this.currentTag)
      })
    },

    suggestionsVisible() {
      return this.currentTag !== '' && this.matches?.length !== 0 && this.open
    },

    currentTag: {
      get: function () {
        let currentTag
        if (this?.selectionType === 'multiple') {
          const tags = this.tag.split(' ')
          currentTag = tags[tags.length - 1]
        } else {
          currentTag = this.tag.trim()
        }
        return currentTag
      },
      set: function (currentTag) {
        if (this.selectionType === 'multiple') {
          const tags = this.tag.split(' ')
          tags[tags.length - 1] = currentTag
          this.tag = tags.join(' ');
        } else {
          this.tag = currentTag.trim()
        }
      }
    },

  },

  methods: {

    loadAutocompleteSuggestions() {
      this.ax(axios.get, `${this.db}/k/${this.key}/`)
          .then(response => {
            this.tagsSuggestions = response.data?.tags || []
            this.tagsSuggestions.sort()
          })
          .catch(() => {
            this.createAutocompleteDBKey()
          })
    },

    openDropdown() {
      this.open = true
    },

    tagChanged() {
      this.$emit('tagChanged', this.tag)
    },

    tagSubmitted() {
      this.$emit('tagSubmitted', this.tag)
    },

    closeDropdown() {
      this.open = false
    },

    suggestionClick(index) {
      this.focusedSuggestionIndex = index
      this.selectTag()
    },

    selectTag() {
      if (this.focusedSuggestionIndex !== -1) {
        this.currentTag = this.matches[this.focusedSuggestionIndex]
      } else if (!this.tagsSuggestions.includes(this.currentTag)) {
        this.addUnknownTagToDB(this.currentTag)
      }
      this.tagSubmitted()
      this.tagChanged()
      this.focusedSuggestionIndex = -1
      this.$refs.tagInput.focus()
      this.open = false
      if (this.clearInputAfterSelection) {
        this.tag = ''
      }
    },

    focusNextSuggestion() {
      if (this.focusedSuggestionIndex > -1)
        this.focusedSuggestionIndex--
    },

    focusPreviousSuggestion() {
      if (this.focusedSuggestionIndex < this.matches.length - 1)
        this.focusedSuggestionIndex++
    },

    isSuggestionFocused(index) {
      return index === this.focusedSuggestionIndex
    },

    createAutocompleteDBKey() {
      // if database doesn't exist, create it
      this.ax(axios.get, `${this.db}/`)
          .catch(() => {
            this.ax(axios.post, `${this.db}`, {})
          })
      // if key doesn't exist, create it
      this.ax(axios.get, `${this.db}/k/${this.key}/`)
          .catch(() => {
            this.ax(axios.put, `${this.db}/k/${this.key}/`, {})
          })
    },

    async addUnknownTagToDB(tag) {
      const response = await this.ax(axios.get, `${this.db}/k/${this.key}/`)
      const document = {...{tags: []}, ...response.data}
      document.tags.push(tag)
      return this.ax(axios.put, `${this.db}/k/${this.key}/`, document)
          .then(() => {
            console.log(`saved key [${this.key}] to database [${this.db}]`)
          })
          .catch(() => {
            console.log(`failed saving key [${this.key}] to database [${this.db}]`)
          })
    },

    ax(axios_method, urlTail, data) {
      if (!axios_method) {
        axios_method = axios.get
      }
      if (!urlTail) {
        urlTail = ""
      }

      const apiroot = this.apiRoot
      const apiversion = this.apiVersion
      const apiurl = `${apiroot}/${apiversion}/db/${urlTail}`

      if (axios_method) {
        console.log("apiurl", apiurl)
        if (data) {
          return axios_method(apiurl, data)
        }
        return axios_method(apiurl)
      }
    },

  },

  created() {
    this.loadAutocompleteSuggestions()
  }
}
</script>

<style scoped>
.dropdown, .dropdown-trigger, .dropdown-menu {
  width: 100%
}
</style>
