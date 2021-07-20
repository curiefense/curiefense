<template>

  <autocomplete-input
      :suggestions="tagsSuggestions"
      :initial-value="initialTag"
      :clear-input-after-selection="clearInputAfterSelection"
      :auto-focus="autoFocus"
      :selection-type="selectionType"
      :minimum-value-length="minimumTagLength"
      :title="inputTitle"
      @value-changed="tagChanged"
      @value-submitted="tagSubmitted"
      @keyup="bubbleEvent('keyup', $event)"
      @keydown="bubbleEvent('keydown', $event)"
      @keypress="bubbleEvent('keypress', $event)"
      @focus="bubbleEvent('focus', $event)"
      @blur="bubbleEvent('blur', $event)"/>

</template>

<script lang="ts">
import _ from 'lodash'
import RequestsUtils from '@/assets/RequestsUtils.ts'
import AutocompleteInput, {AutocompleteInputEvents, AutocompleteSuggestion} from '@/components/AutocompleteInput.vue'
import Vue from 'vue'
import {AxiosResponse} from 'axios'
import {TagsDatabaseDocument} from '@/types'

export default Vue.extend({
  name: 'TagAutocompleteInput',

  components: {
    AutocompleteInput,
  },

  props: {
    initialTag: {
      type: String,
      default: '',
    },
    clearInputAfterSelection: {
      type: Boolean,
      default: false,
    },
    autoFocus: {
      type: Boolean,
      default: false,
    },
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

  data() {
    const defaultKeyData = {
      legitimate: [] as string[],
      malicious: [] as string[],
      neutral: [] as string[],
    }
    return {
      tag: this.initialTag,
      open: false,
      tagsSuggestions: [] as AutocompleteSuggestion[],
      tagsSuggestionsLoading: false,
      tagsAddedWhileSuggestionsLoading: [] as string[],
      focusedSuggestionIndex: -1,
      db: 'system',
      key: 'tags',
      defaultKeyData: defaultKeyData,
      defaultDatabaseData: {
        tags: defaultKeyData,
      },
      minimumTagLength: 3,

      apiRoot: RequestsUtils.confAPIRoot,
      apiVersion: RequestsUtils.confAPIVersion,
    }
  },

  computed: {

    currentTag(): string {
      let currentTag
      if (this?.selectionType.toLowerCase() === 'multiple') {
        const tags = this.tag.split(' ')
        currentTag = tags[tags.length - 1].trim()
      } else {
        currentTag = this.tag.trim()
      }
      return currentTag
    },

    inputTitle(): string {
      return this.selectionType.toLowerCase() === 'multiple' ? 'Space separated tags' : 'Tag'
    },

  },

  methods: {

    async loadAutocompleteSuggestions() {
      this.tagsSuggestionsLoading = true
      const response: AxiosResponse<TagsDatabaseDocument> = await RequestsUtils.sendRequest({
        methodName: 'GET',
        url: `db/${this.db}/k/${this.key}/`,
        onFail: async () => {
          // key does not exist, check if db exists
          await RequestsUtils.sendRequest({
            methodName: 'GET',
            url: `db/${this.db}/`,
            onFail: async () => {
              // db doest not exist, key does not exist -> create a db with key
              await RequestsUtils.sendRequest({
                methodName: 'POST',
                url: `db/${this.db}/`,
                data: this.defaultDatabaseData,
              })
              this.tagsSuggestionsLoading = false
            },
          })
          // db exits, key does not exist -> create a key
          await RequestsUtils.sendRequest({
            methodName: 'PUT',
            url: `db/${this.db}/k/${this.key}/`,
            data: this.defaultKeyData,
          })
          this.tagsSuggestionsLoading = false
        },
      })
      this.buildTagsSuggestionsFromData(response.data)
      this.tagsSuggestionsLoading = false
      if (this.tagsAddedWhileSuggestionsLoading.length > 0) {
        this.addUnknownTagsToDB(this.tagsAddedWhileSuggestionsLoading)
        this.tagsAddedWhileSuggestionsLoading = []
      }
    },

    buildTagsSuggestionsFromData(data: TagsDatabaseDocument) {
      data = {...this.defaultKeyData, ...data}
      const legitimateTags = data.legitimate.map((item: string) => {
        return {
          prefix: '<span class="dot legitimate" title="legitimate"></span>',
          value: item,
        }
      })
      const maliciousTags = data.malicious.map((item: string) => {
        return {
          prefix: '<span class="dot malicious" title="malicious"></span>',
          value: item,
        }
      })
      const neutralTags = data.neutral.map((item: string) => {
        return {
          prefix: '<span class="dot neutral" title="neutral"></span>',
          value: item,
        }
      })
      this.tagsSuggestions = [].concat(legitimateTags, maliciousTags, neutralTags) as AutocompleteSuggestion[]
      this.tagsSuggestions = _.sortBy(this.tagsSuggestions, 'value') as AutocompleteSuggestion[]
    },

    bubbleEvent(eventName: AutocompleteInputEvents, event: Event) {
      this.$emit(eventName, event)
    },

    tagChanged(newTag: string) {
      this.tag = newTag
      this.$emit('tag-changed', this.tag)
    },

    tagSubmitted(newTag: string) {
      this.tag = newTag
      // if submitting a tag we don't recognize -> add it to the DB
      if (!this.tagsSuggestions.find((suggestion) => {
        return suggestion.value.toLowerCase() === this.currentTag.toLowerCase()
      })) {
        this.addUnknownTagsToDB([this.currentTag])
      }
      this.$emit('tag-submitted', this.tag)
    },

    async addUnknownTagsToDB(tags: string[]) {
      // do not add tags to DB if DB hasn't loaded
      if (this.tagsSuggestionsLoading) {
        this.tagsAddedWhileSuggestionsLoading.concat(tags)
        return
      }
      // get current tags from DB
      const response = await RequestsUtils.sendRequest({methodName: 'GET', url: `db/${this.db}/k/${this.key}/`})
      const document = {...this.defaultKeyData, ...response.data}
      // add each new tag to neutral group if does not exist anywhere
      for (let i = 0; i < tags.length; i++) {
        // set both the temporary tag and the tag in array to lowercase for easier logging later
        const tag = tags[i] = tags[i].toLowerCase()
        if ((!document.legitimate || _.findIndex(document.legitimate, (dbTag) => dbTag === tag) === -1) &&
            (!document.malicious || _.findIndex(document.malicious, (dbTag) => dbTag === tag) === -1) &&
            (!document.neutral || _.findIndex(document.neutral, (dbTag) => dbTag === tag) === -1)) {
          document.neutral.push(tag)
        }
      }
      // save to DB
      await RequestsUtils.sendRequest({
        methodName: 'PUT',
        url: `db/${this.db}/k/${this.key}/`,
        data: document,
      })
      // rebuild the tags suggestion list after a successful save
      this.buildTagsSuggestionsFromData(document)
      console.log(
        `saved to database the following tags list: [${tags.join(',')}],it will now be available for autocomplete!`,
      )
    },
  },

  created() {
    this.loadAutocompleteSuggestions()
  },
})
</script>
<style scoped lang="scss">

@import 'node_modules/bulma/sass/utilities/initial-variables.sass';
@import 'node_modules/bulma/sass/utilities/functions.sass';
@import 'node_modules/bulma/sass/utilities/derived-variables.sass';
@import 'node_modules/bulma/sass/helpers/color.sass';

::v-deep .dot {
  @extend .has-background-info;
  border-radius: 50%;
  display: inline-block;
  height: 0.5rem;
  margin-left: -0.25rem;
  margin-right: 0.25rem;
  width: 0.5rem;
}

::v-deep .dot.legitimate {
  @extend .has-background-success;
}

::v-deep .dot.malicious {
  @extend .has-background-danger;
}

::v-deep .dot.neutral {
  @extend .has-background-grey-light ;
}
</style>
