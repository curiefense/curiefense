<template>
  <div class="response-actions">
    <label v-if="labelSeparatedLine && label"
           class="label is-small is-size-7 has-text-left form-label">
      {{ label }}
    </label>
    <div class="columns mb-0 is-multiline"
         :class="{'wide-columns': wideColumns}">
      <!--
      Available options:
      1) label, no single input column
      2-5-5
      2) no label, no single input column
      6-6
      3) label, single input column
      2-10
      2-10
      4) no label, single input column
      12
      12
      -->
      <div v-if="labelDisplayedInline"
           class="column is-2">
        <label class="label is-small has-text-left form-label">{{ label }}</label>
      </div>
      <div class="column"
           :class="{'is-5': labelDisplayedInline && !isSingleInputColumn,
                    'is-6': !labelDisplayedInline && !isSingleInputColumn,
                    'is-10': labelDisplayedInline && isSingleInputColumn,
                    'is-12': !labelDisplayedInline && isSingleInputColumn,
                    'pl-0': wideColumns && !labelDisplayedInline && !isSingleInputColumn}">
        <div class="control select is-fullwidth is-small" v-if="action">
          <select v-model="action.type">
            <option v-for="(value, id) in options" :value="id" :key="id">{{ value.title }}</option>
          </select>
        </div>
      </div>
      <div v-if="labelDisplayedInline && isSingleInputColumn"
           class="column is-2 pt-0">
      </div>
      <div
          v-if="action && (action.type === 'response' || action.type === 'redirect' || action.type === 'ban' || action.type === 'request_header')"
          class="column"
          :class="{'is-5': labelDisplayedInline && !isSingleInputColumn,
                    'is-6': !labelDisplayedInline && !isSingleInputColumn,
                    'is-10 pt-0': labelDisplayedInline && isSingleInputColumn,
                    'is-12 pt-0': !labelDisplayedInline && isSingleInputColumn,
                    'pr-0': wideColumns && !isSingleInputColumn}">
        <p class="control is-fullwidth">
          <input
              v-if="action && (action.type === 'response' || action.type === 'redirect')"
              class="input is-small"
              type="text"
              v-model="action.params.status"
              placeholder="Status code">
          <input
              v-if="action && action.type === 'ban'"
              class="input is-small"
              type="text"
              v-model="action.params.ttl"
              placeholder="Duration">
          <input
              v-if="action && action.type === 'request_header'"
              class="input is-small"
              type="text"
              v-model="action.params.headers"
              placeholder="Header">
        </p>
      </div>
      <!--
      Available options:
      1) label
      2-10
      2) no label
      12
      -->
      <template v-if="action && (action.type === 'response' || action.type === 'redirect')">
        <div v-if="labelDisplayedInline"
             class="column is-2 pt-0">
        </div>
        <div class="column pt-0"
             :class="{'is-10': labelDisplayedInline, 'is-12': !labelDisplayedInline}">
          <div v-if="action.type === 'response'"
               class="control is-fullwidth">
              <textarea
                  v-model="action.params.content"
                  class="textarea is-small"
                  rows="2"
                  placeholder="Response body">
              </textarea>
          </div>
          <div v-if="action.type === 'redirect'"
               class="control is-fullwidth">
            <p>
              <input
                  class="input is-small"
                  type="text"
                  v-model="action.params.location"
                  placeholder="Location">
            </p>
          </div>
        </div>
      </template>
    </div>
    <div class="content" v-if="action && action.type === 'ban' && action.params.action">
      <response-action :object-with-action.sync="action.params"
                       :label-separated-line="labelSeparatedLine"
                       :wide-columns="wideColumns"
                       :is-single-input-column="isSingleInputColumn"
                       :ignore="['ban']"
                       label="Ban action"/>
    </div>
  </div>
</template>

<script lang="ts">
import DatasetsUtils from '@/assets/DatasetsUtils'
import Vue from 'vue'

export default Vue.extend({
  name: 'ResponseAction',
  props: {
    objectWithAction: Object,
    label: {
      type: String,
      default: 'Action'
    },
    ignore: Array,
    labelSeparatedLine: Boolean,
    wideColumns: Boolean,
    isSingleInputColumn: Boolean,
  },

  data() {
    return {
      options: this.ld.pickBy({...DatasetsUtils.ResponseActions}, (value, key) => {
        return !this.ignore || !this.ignore.includes(key)
      })
    }
  },
  computed: {
    labelDisplayedInline() {
      return !this.labelSeparatedLine && this.label
    },
    action: {
      get() {
        return this.objectWithAction?.action
      },
      set(newAction) {
        this.$set(this.objectWithAction, 'action', newAction)
      }
    },
  },
  mounted() {
    this.normalizeAction()
  },
  methods: {
    normalizeAction() {
      // adding necessary fields to action field
      this.action = {
        ...{
          params: {
            action: {
              type: 'default',
              params: {}
            }
          }
        },
        ...this.action
      }
    },
  },
  watch: {
    objectWithAction() {
      this.normalizeAction()
    }
  }
})
</script>
<style scoped lang="scss">

.response-actions .column.additional {
  padding-top: 0;
}

.wide-columns {
  // 12 (full width)
  & .column.is-12 {
    padding-left: 0;
    padding-right: 0;
  }

  // 2-10 (full width)
  & .column.is-2 {
    padding-left: 0;
  }

  & .column.is-10 {
    padding-right: 0;
  }
}

</style>
