<template>
  <div>
    <div class="card">
      <div class="card-content">
        <div class="media">
          <div class="media-content">
            <div class="columns mb-0">
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
                <div class="field">
                  <label class="label is-small">Notes</label>
                  <div class="control">
                    <textarea class="is-small textarea document-notes"
                              title="Notes"
                              v-model="localDoc.notes"
                              @change="emitDocUpdate()"
                              rows="2">
                    </textarea>
                  </div>
                </div>
                <div class="field">
                  <label class="label is-small">Category</label>
                  <div class="control">
                    <input class="input is-small document-category"
                           title="Category"
                           placeholder="Category"
                           v-model="localDoc.category"
                           @change="emitDocUpdate()"/>
                  </div>
                </div>
                <div class="field">
                  <label class="label is-small">Subcategory</label>
                  <div class="control">
                    <input class="input is-small document-subcategory"
                           title="Subcategory"
                           placeholder="Subcategory"
                           v-model="localDoc.subcategory"
                           @change="emitDocUpdate()"/>
                  </div>
                </div>
                <div class="field">
                  <label class="label is-small">Risk Level</label>
                  <div class="control select is-small">
                    <select v-model="localDoc.risk"
                            @change="emitDocUpdate"
                            class="risk-level-selection"
                            title="Risk level">
                      <option v-for="(riskLevel, index) in riskLevels"
                              :value="riskLevel"
                              :key="index">
                        {{ riskLevel }}
                      </option>
                    </select>
                  </div>
                </div>
              </div>
            </div>
            <div class="field">
              <label class="label is-small">Match</label>
              <div class="control has-icons-left">
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
import {ContentFilterRule} from '@/types'
import _ from 'lodash'

export default Vue.extend({
  name: 'ContentFilterRulesEditor',
  props: {
    selectedDoc: Object,
  },
  computed: {
    localDoc(): ContentFilterRule {
      return _.cloneDeep(this.selectedDoc)
    },
  },
  data() {
    return {
      riskLevels: [1, 2, 3, 4, 5],
    }
  },
  methods: {
    emitDocUpdate() {
      this.$emit('update:selectedDoc', this.localDoc)
    },
  },
})
</script>
