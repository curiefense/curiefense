<template>
  <div class="card">
    <div class="card-content">
      <div class="media">
        <div class="media-content">
          <div class="columns">
            <div class="column">
              <div class="field is-grouped">
                <input class="input is-small is-fullwidth"
                       type="text"
                       placeholder="Document name"
                       v-model="selectedDoc.name">
              </div>
              <p class="subtitle is-6 has-text-grey"
                 title="Document ID">
                {{ selectedDoc.id }}
              </p>
            </div>
            <div class="column"></div>
            <div class="column">
            </div>
          </div>
        </div>
      </div>

      <div class="content">
        <hr/>
        <div class="columns">
          <div class="column is-2" v-for="operation in operations" :key="operation">
            <p class="title is-7 is-uppercase x-has-text-centered">{{ titles[operation] }}</p>
            <hr :style="barStyle[operation]"/>
            <table class="table is-narrow is-fullwidth ">
              <tbody>
              <tr v-for="(tag, idx) in selectedDoc[operation]" :key="idx">
                <td :class=" duplicateTags[tag] ? 'has-text-danger' : '' "
                    :style=" allPrior(operation) ? 'text-decoration: line-through; color: lightgray' : '' "
                    :title=" allPrior(operation) ? '[all] is set in a higher priorirty section' : '' ">
                  {{ tag }}
                </td>
                <td class="is-size-7 is-18-px">
                  <a title="remove entry"
                     class="is-small has-text-grey"
                     @click="removeTag(operation, idx)">
                    &ndash;
                  </a>
                </td>
              </tr>
              <tr>
                <td>
                  <tag-autocomplete-input v-if="addNewColName === operation"
                                          :clearInputAfterSelection="true"
                                          :selectionType="'single'"
                                          :autoFocus="true"
                                          @keyup.esc="cancelAddNewTag"
                                          @tagSubmitted="addNewEntry(operation, $event)">
                  </tag-autocomplete-input>
                </td>
                <td class="is-size-7 is-18-px">
                  <a title="add new entry"
                     class="is-size-7 is-18-px is-small has-text-grey"
                     @click="openTagInput(operation)">
                    +
                  </a>
                </td>
              </tr>
              </tbody>
            </table>
          </div>
        </div>
        <span class="is-family-monospace has-text-grey-lighter">{{ apiPath }}</span>
      </div>
    </div>
  </div>
</template>

<script>

import DatasetsUtils from '@/assets/DatasetsUtils.js'
import TagAutocompleteInput from '@/components/TagAutocompleteInput'

export default {
  name: 'ACLEditor',

  components: {
    TagAutocompleteInput
  },

  props: {
    selectedDoc: Object,
    apiPath: String
  },

  data() {
    return {
      operations: ["force_deny", "bypass", "allow_bot", "deny_bot", "allow", "deny"],
      barStyle: {
        "force_deny": "background-color: hsl(348, 100%, 61%); margin: 1rem 0 0.5rem 0;",
        "deny_bot": "background-color: hsl(348, 100%, 61%); margin: 1rem 0 0.5rem 0;",
        "deny": "background-color: hsl(348, 100%, 61%); margin: 1rem 0 0.5rem 0;",
        "bypass": "background-color: hsl(141,  71%, 48%); margin: 1rem 0 0.5rem 0;",
        "allow": "background-color: hsl(204,  86%, 53%); margin: 1rem 0 0.5rem 0;",
        "allow_bot": "background-color: hsl(204,  86%, 53%); margin: 1rem 0 0.5rem 0;"
      },
      titles: DatasetsUtils.Titles,
      addNewColName: null
    }
  },
  computed: {

    duplicateTags() {
      let doc = this.selectedDoc;
      let allTags = this.ld.concat(doc["force_deny"], doc["bypass"], doc["allow_bot"], doc["deny_bot"], doc["allow"], doc["deny"]);
      let dupTags = this.ld.filter(allTags, (val, i, iteratee) => this.ld.includes(iteratee, val, i + 1));
      return this.ld.fromPairs(this.ld.zip(dupTags, dupTags))
    },


  },
  methods: {
    // returns true if tag "all" is set in a higher priority section
    allPrior(self) {
      // top priority, skip
      if (self === "force_deny") {
        return false
      }

      let selfIdx = this.ld.indexOf(this.operations, self),
          doc = this.selectedDoc,
          operations = this.operations;

      for (let idx = 0; idx < selfIdx; idx++) {
        if (this.ld.indexOf(doc[operations[idx]], "all") > -1) {
          if (idx === 3)
            return false
          if (idx === 2)
            return selfIdx === 3;
          return true;
        }
      }
    },

    addNewEntry(section, entry) {
      if (entry && entry.length > 2) {
        this.selectedDoc[section].push(entry)
        this.$emit('update:selectedDoc', this.selectedDoc)
      }
    },

    openTagInput(section) {
      this.addNewColName = section
    },

    cancelAddNewTag() {
      this.addNewColName = null
    },

    removeTag(section, idx) {
      this.selectedDoc[section].splice(idx, 1)
      this.addNewColName = null
      this.$emit('update:selectedDoc', this.selectedDoc)
    }

  },

}


</script>

<style scoped>
.is-18-px {
  min-width: 18px;
  max-width: 18px;
  width: 18px;
}

/deep/ .tag-input {
  font-size: 0.58rem
}
</style>
