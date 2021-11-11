<template>
  <div class="card">
    <div class="card-content">
      <div class="content">
        <div class="columns columns-divided">
          <div class="column is-4">
            <div class="field">
              <label class="label is-small">
                Name
                <span class="has-text-grey is-pulled-right document-id"
                      title="Group id">
                  {{ localDoc.id }}
                </span>
              </label>
              <div class="control">
                <input class="input is-small document-name"
                       title="Group name"
                       placeholder="Group name"
                       @change="emitDocUpdate"
                       v-model="localDoc.name" />
              </div>
            </div>
            <div class="field">
              <label class="label is-small">Description</label>
              <div class="control">
                <textarea class="is-small textarea document-description"
                          title="Description"
                          @change="emitDocUpdate"
                          v-model="localDoc.description" />
              </div>
            </div>
          </div>
          <div class="column is-8">
            <div class="tile is-ancestor">
              <div class="tile is-vertical">
                <div class="tile">
                  <div class="tile is-parent is-vertical">
                    <table class="table is-narrow entries-table mb-0">
                      <tr
                        v-for="(ruleId, ruleIndex) in contentFilterRulesPage"
                        :key="ruleIndex"
                        class="entry-row"
                      >
                        <td class="is-size-7 has-text-weight-medium width-80pct">
                          <a :href="`/config/${selectedBranch}/contentfilterrules/${ruleId}`"
                             target="_blank">
                          {{ getRuleName(ruleId) }}
                          </a>
                        </td>
                        <td class="is-size-7 width-20pct has-text-right">
                          <a tabindex="0"
                            class="is-small has-text-grey remove-rule-button"
                            title="remove rule"
                            @click="removeRule(ruleIndex)">
                              remove
                          </a>
                        </td>
                      </tr>
                      <tr v-if="addRuleMode" class="new-rule-row">
                        <td class="is-size-7 width-80pct">
                          <div class="select is-small is-fullwidth">
                            <select
                              v-model="newRule"
                              title="New rule"
                              class="select new-rule-selection"
                            >
                              <option v-for="({id, name}) in localContentFilterRules"
                                      :key="id"
                                      :value="id">
                                {{ name }}
                              </option>
                            </select>
                          </div>
                        </td>
                        <td class="is-size-7 width-20pct has-text-right">
                          <a class="is-size-7 has-text-grey add-button confirm-add-rule-button"
                            title="add new rule"
                            tabindex="0"
                            @click="addRule">
                              <i class="fas fa-check" /> Add
                          </a>
                          <br/>
                          <a class="is-size-7 has-text-grey remove-button cancel-rule-button"
                            title="cancel add new row"
                            tabindex="0"
                            @click="closeAddRuleMode">
                              <i class="fas fa-times" /> Cancel
                          </a>
                        </td>
                      </tr>
                      <tr v-if="localContentFilterRules.length && !addRuleMode">
                        <td colspan="2">
                          <a class="is-size-7 has-text-grey-lighter add-button add-rule-button"
                            title="add rule"
                            tabindex="0"
                            @click="addRuleMode = true">
                              <i class="fas fa-plus" />
                          </a>
                        </td>
                      </tr>
                      <tr v-if="totalPages > 1">
                        <td colspan="2">
                          <nav aria-label="pagination"
                               class="pagination is-small"
                               role="navigation">
                            <a :disabled="currentPage === 1"
                              class="is-pulled-left pagination-previous"
                              tabindex="0"
                              @click="navigate(currentPage - 1)">
                                Previous page
                            </a>
                            <a :disabled="currentPage === totalPages"
                              class="is-pulled-right pagination-next"
                              tabindex="0"
                              @click="navigate(currentPage + 1)">
                                Next page
                            </a>
                          </nav>
                        </td>
                      </tr>
                    </table>
                  </div>
                </div>
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
import {ContentFilterRuleGroup, ContentFilterRule} from '@/types'
import _ from 'lodash'
import {AxiosResponse} from 'axios'
import RequestsUtils from '@/assets/RequestsUtils'

export default Vue.extend({
  name: 'ContentFilterRuleGroupEditor',
  props: {
    selectedDoc: Object,
    selectedBranch: String,
  },
  data() {
    return {
      addRuleMode: false,
      newRule: '',
      contentFilterRules: [] as ContentFilterRule[],
      rowsPerPage: 20,
      currentPage: 1,
    }
  },
  computed: {
    localDoc(): ContentFilterRuleGroup {
      return _.cloneDeep(this.selectedDoc)
    },
    localContentFilterRules(): ContentFilterRule[] {
      return this.contentFilterRules.filter(({id}) => !this.localDoc.content_filter_ids?.includes(id))
    },
    totalPages(): number {
      return Math.ceil(this.localDoc.content_filter_ids?.length / this.rowsPerPage)
    },
    contentFilterRulesPage(): ContentFilterRuleGroup['content_filter_ids'] {
      const {localDoc, currentPage, rowsPerPage} = this
      const firstIndex = (currentPage-1) * rowsPerPage
      const lastIndex = firstIndex + rowsPerPage
      return localDoc.content_filter_ids?.slice(firstIndex, lastIndex)
    },
  },
  methods: {
    emitDocUpdate() {
      this.$emit('update:selectedDoc', this.localDoc)
    },
    getRuleName(ruleId: ContentFilterRule['id']) {
      return this.contentFilterRules.find(({id}) => ruleId === id)?.name
    },
    removeRule(ruleIndex: number) {
      this.localDoc.content_filter_ids.splice(ruleIndex, 1)
      this.emitDocUpdate()
    },
    addRule() {
      if (this.newRule) {
        this.localDoc.content_filter_ids.push(this.newRule)
        this.emitDocUpdate()
        this.closeAddRuleMode()
        this.$nextTick(() => this.navigate(this.totalPages))
      }
    },
    closeAddRuleMode() {
      this.addRuleMode = false
      this.newRule = ''
    },
    navigate(pageNum: number) {
      if (pageNum && pageNum <= this.totalPages) {
        this.currentPage = pageNum
      }
    },
  },
  async mounted() {
    const response: AxiosResponse = await RequestsUtils.sendRequest({
      methodName: 'GET',
      url: `configs/${this.selectedBranch}/d/contentfilterrules/`,
    })
    this.contentFilterRules = response?.data || []
  },
})
</script>
