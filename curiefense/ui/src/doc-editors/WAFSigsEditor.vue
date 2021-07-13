<template>
  <div>
    <div class="card">
      <div class="card-content">
        <div class="media">
          <div class="media-content">
            <div class="columns">
              <div class="column is-4">
                <div class="field">
                  <label class="label is-small">
                    Name
                    <span class="has-text-grey is-pulled-right document-id" title="Document id">
                    {{ selectedDoc.id }}
                  </span>
                  </label>
                  <div class="control">
                    <input class="input is-small document-name"
                           title="Document name"
                           placeholder="Document name"
                           v-model="localDoc.name"
                           @change="emitDocUpdate()"/>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="content px-3 py-3 mx-0 my-0">
          <div class="field">
            <p class="control has-icons-left">
              <input class="input is-small document-operand"
                     type="text"
                     title="Match"
                     placeholder="matching domain(s) regex"
                     v-model="localDoc.operand"
                     @change="emitDocUpdate()"
                     required>
              <span class="icon is-small is-left has-text-grey">
                <i class="fas fa-code"></i>
              </span>
            </p>
          </div>
          <div class="field is-grouped is-grouped-multiline">
            <div class="control">
              <div class="tags has-addons">
                <span class="tag">severity</span>
                <span class="tag has-text-info document-severity">{{ localDoc.severity }}</span>
              </div>
            </div>

            <div class="control">
              <div class="tags has-addons">
                <span class="tag">certainty</span>
                <span class="tag has-text-info document-certainty">{{ localDoc.certainity }}</span>
              </div>
            </div>

            <div class="control">
              <div class="tags has-addons">
                <span class="tag">category</span>
                <span class="tag has-text-info document-category">{{ localDoc.category }}</span>
              </div>
            </div>

            <div class="control">
              <div class="tags has-addons">
                <span class="tag">subcategory</span>
                <span class="tag has-text-info document-subcategory">{{ localDoc.subcategory }}</span>
              </div>
            </div>

          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import {WAFRule} from '@/types'
import _ from 'lodash'

export default Vue.extend({
  name: 'WAFSigsEditor',
  props: {
    selectedDoc: Object,
  },
  computed: {
    localDoc(): WAFRule {
      return _.cloneDeep(this.selectedDoc)
    },
  },

  methods: {
    emitDocUpdate() {
      this.$emit('update:selectedDoc', this.localDoc)
    },
  },
})
</script>
