<template>
  <div class="response-actions">
    <label v-if="labelSeparatedLine && label"
           class="label is-small is-size-7 has-text-left form-label">
      {{ label }}
    </label>
    <div class="columns mb-0 is-multiline">
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
                    'is-12': !labelDisplayedInline && isSingleInputColumn}">
        <div class="control select is-fullwidth is-small action-type-selection" v-if="localAction">
          <select v-model="localAction.type"
                  title="Action type"
                  @change="changeActionType()">
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
                    'is-12 pt-0': !labelDisplayedInline && isSingleInputColumn}">
        <p class="control is-fullwidth">
          <input
              v-if="localAction && (localAction.type === 'response' || localAction.type === 'redirect')"
              class="input is-small action-status"
              type="text"
              v-model="localAction.params.status"
              @change="emitActionUpdate"
              title="Status code"
              placeholder="Status code">
          <span v-if="localAction && localAction.type === 'ban'"
                class="suffix seconds-suffix">
            <input
                class="input is-small action-duration"
                type="text"
                v-model="localAction.params.duration"
                @change="emitActionUpdate"
                title="Duration"
                placeholder="Duration">
          </span>
          <input
              v-if="localAction && localAction.type === 'request_header'"
              class="input is-small action-headers"
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
                        class="textarea is-small action-content"
                        rows="2"
                        title="Response body"
                        placeholder="Response body">
              </textarea>
          </div>
          <div v-if="localAction.type === 'redirect'"
               class="control is-fullwidth">
            <p>
              <input class="input is-small action-location"
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
                       :is-single-input-column="isSingleInputColumn"
                       :ignore="['ban']"
                       @update:action="emitActionUpdate"
                       label="Ban action"/>
    </div>
  </div>
</template>

<script lang="ts">
import _ from 'lodash'
import Vue, {PropType} from 'vue'
import {ResponseActionType} from '@/types'

export const responseActions = {
  'default': {'title': '503 Service Unavailable'},
  'challenge': {'title': 'Challenge'},
  'monitor': {'title': 'Tag Only'},
  'response': {'title': 'Response', 'params': {'status': '', 'content': ''}},
  'redirect': {'title': 'Redirect', 'params': {'status': '30[12378]', 'location': 'https?://.+'}},
  'ban': {'title': 'Ban', 'params': {'duration': '[0-9]+', 'action': {'type': 'default', 'params': {}}}},
  'request_header': {'title': 'Header', 'params': {'headers': ''}},
}

export default Vue.extend({
  name: 'ResponseAction',
  props: {
    action: Object as PropType<ResponseActionType>,
    label: {
      type: String,
      default: 'Action',
    },
    ignore: {
      type: Array as PropType<ResponseActionType['type'][]>,
      default: (): ResponseActionType['type'][] => {
        return []
      },
    },
    labelSeparatedLine: {
      type: Boolean,
      default: false,
    },
    isSingleInputColumn: {
      type: Boolean,
      default: false,
    },
  },

  data() {
    return {
      options: _.pickBy({...responseActions}, (value, key) => {
        return !this.ignore || !this.ignore.includes(key as ResponseActionType['type'])
      }),
    }
  },
  computed: {
    localAction(): ResponseActionType {
      return _.cloneDeep(this.action)
    },

    labelDisplayedInline(): boolean {
      return !this.labelSeparatedLine && !!this.label
    },
  },
  methods: {
    emitActionUpdate() {
      this.$emit('update:action', this.localAction)
    },

    changeActionType() {
      delete this.localAction.params
      this.normalizeActionParams()
    },

    normalizeActionParams() {
      const oldParams = _.cloneDeep(this.localAction.params) || {}
      delete this.localAction.params
      if (this.localAction.type !== 'default' &&
          this.localAction.type !== 'challenge' &&
          this.localAction.type !== 'monitor') {
        this.localAction.params = {}
      }
      if (this.localAction.type === 'response') {
        this.localAction.params.status = oldParams.status ? oldParams.status : ''
        this.localAction.params.content = oldParams.content ? oldParams.content : ''
      }
      if (this.localAction.type === 'redirect') {
        this.localAction.params.status = oldParams.status ? oldParams.status : ''
        this.localAction.params.location = oldParams.location ? oldParams.location : ''
      }
      if (this.localAction.type === 'ban') {
        this.localAction.params.duration = oldParams.duration ? oldParams.duration : ''
        this.localAction.params.action = oldParams.action ? oldParams.action : {
          type: 'default',
        }
      }
      if (this.localAction.type === 'request_header') {
        this.localAction.params.headers = oldParams.headers ? oldParams.headers : ''
      }
      if (!_.isEqual(this.localAction, this.action)) {
        this.emitActionUpdate()
      }
    },
  },
  watch: {
    action: {
      handler: function(value) {
        if (!value) {
          this.$emit('update:action', {
            type: 'default',
          })
          return
        }
        // adding necessary fields to action params field
        this.normalizeActionParams()
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


</style>
