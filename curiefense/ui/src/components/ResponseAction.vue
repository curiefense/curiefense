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
        <div class="control select is-fullwidth is-small" v-if="localAction">
          <select v-model="localAction.type"
                  title="Action type"
                  @change="emitActionUpdate">
            <option v-for="(value, id) in options"
                    :value="id"
                    :key="id">
              {{ value.title }}
            </option>
          </select>
        </div>
      </div>
      <div v-if="labelDisplayedInline && isSingleInputColumn"
           class="column is-2 pt-0">
      </div>
      <div
          v-if="localAction && (localAction.type === 'response' || localAction.type === 'redirect' ||
                localAction.type === 'ban' || localAction.type === 'request_header')"
          class="column"
          :class="{'is-5': labelDisplayedInline && !isSingleInputColumn,
                    'is-6': !labelDisplayedInline && !isSingleInputColumn,
                    'is-10 pt-0': labelDisplayedInline && isSingleInputColumn,
                    'is-12 pt-0': !labelDisplayedInline && isSingleInputColumn,
                    'pr-0': wideColumns && !isSingleInputColumn}">
        <p class="control is-fullwidth">
          <input
              v-if="localAction && (localAction.type === 'response' || localAction.type === 'redirect')"
              class="input is-small"
              type="text"
              v-model="localAction.params.status"
              @change="emitActionUpdate"
              title="Status code"
              placeholder="Status code">
          <input
              v-if="localAction && localAction.type === 'ban'"
              class="input is-small"
              type="text"
              v-model="localAction.params.ttl"
              @change="emitActionUpdate"
              title="Duration"
              placeholder="Duration">
          <input
              v-if="localAction && localAction.type === 'request_header'"
              class="input is-small"
              type="text"
              v-model="localAction.params.headers"
              @change="emitActionUpdate"
              title="Header"
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
      <template v-if="localAction && (localAction.type === 'response' || localAction.type === 'redirect')">
        <div v-if="labelDisplayedInline"
             class="column is-2 pt-0">
        </div>
        <div class="column pt-0"
             :class="{'is-10': labelDisplayedInline, 'is-12': !labelDisplayedInline}">
          <div v-if="localAction.type === 'response'"
               class="control is-fullwidth">
              <textarea v-model="localAction.params.content"
                        @change="emitActionUpdate"
                        class="textarea is-small"
                        rows="2"
                        title="Response body"
                        placeholder="Response body">
              </textarea>
          </div>
          <div v-if="localAction.type === 'redirect'"
               class="control is-fullwidth">
            <p>
              <input class="input is-small"
                     type="text"
                     v-model="localAction.params.location"
                     @change="emitActionUpdate"
                     title="Location"
                     placeholder="Location">
            </p>
          </div>
        </div>
      </template>
    </div>
    <div class="content" v-if="localAction && localAction.type === 'ban' && localAction.params.action">
      <response-action :action.sync="localAction.params.action"
                       :label-separated-line="labelSeparatedLine"
                       :wide-columns="wideColumns"
                       :is-single-input-column="isSingleInputColumn"
                       :ignore="['ban']"
                       @update:action="emitActionUpdate"
                       label="Ban action"/>
    </div>
  </div>
</template>

<script lang="ts">
import _ from 'lodash'
import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import Vue, {PropType} from 'vue'
import {ResponseActionType} from '@/types'

export default Vue.extend({
  name: 'ResponseAction',
  props: {
    action: Object as PropType<ResponseActionType>,
    label: {
      type: String,
      default: 'Action',
    },
    ignore: Array,
    labelSeparatedLine: Boolean,
    wideColumns: Boolean,
    isSingleInputColumn: Boolean,
  },

  data() {
    return {
      options: _.pickBy({...DatasetsUtils.ResponseActions}, (value, key) => {
        return !this.ignore || !this.ignore.includes(key)
      }),
    }
  },
  computed: {
    localAction(): ResponseActionType {
      return JSON.parse(JSON.stringify(this.action || {}))
    },

    labelDisplayedInline(): boolean {
      return !this.labelSeparatedLine && !!this.label
    },
  },
  methods: {
    emitActionUpdate() {
      this.$emit('update:action', this.localAction)
    },

    normalizeAction() {
      // adding necessary fields to action field
      const normalizedAction: ResponseActionType = {
        ...{
          params: {
            action: {
              type: 'default',
              params: {},
            },
          },
        },
        ...this.localAction,
      }
      this.localAction.type = normalizedAction.type
      this.localAction.params = normalizedAction.params
      if (!_.isEqual(this.localAction, this.action)) {
        this.emitActionUpdate()
      }
    },
  },
  watch: {
    action: {
      handler: function() {
        this.normalizeAction()
      },
      immediate: true,
      deep: true,
    },
  },
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
