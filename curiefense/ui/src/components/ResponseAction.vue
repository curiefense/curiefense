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
                  @change="normalizeActionParams">
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
      <div v-if="localAction && ['response', 'redirect', 'ban'].includes( localAction.type )"
          class="column"
          :class="{'is-5': labelDisplayedInline && !isSingleInputColumn,
                    'is-6': !labelDisplayedInline && !isSingleInputColumn,
                    'is-10 pt-0': labelDisplayedInline && isSingleInputColumn,
                    'is-12 pt-0': !labelDisplayedInline && isSingleInputColumn}">
        <div class="control is-fullwidth">
          <input v-if="['response', 'redirect'].includes( localAction.type )"
              class="input is-small action-status"
              type="text"
              v-model="localAction.params.status"
              @input="fieldChange( 'status' )"
              title="Status code"
              placeholder="Status code" />
          <span v-else class="suffix seconds-suffix">
            <input class="input is-small action-duration"
              type="text"
              v-model="localAction.params.ttl"
              @input="fieldChange( 'ttl' )"
              title="Duration"
              placeholder="Duration" />
          </span>
        </div>
      </div>
      <!--
      Available options:
      1) label
      2-10
      2) no label
      12
      -->
      <template v-if="localAction && ['response', 'redirect'].includes( localAction.type )">
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
            <input class="input is-small action-location"
              type="text"
              v-model="localAction.params.location"
              @input="fieldChange( 'location' )"
              title="Location"
              placeholder="Location"
            />
          </div>
        </div>
      </template>
    </div>
    <div v-if="localAction && ['request_header', 'response'].includes( localAction.type )"
         class="columns">
      <div class="column"
        :class="{
          'is-6': !labelDisplayedInline || isSingleInputColumn,
          'is-5 is-offset-2': labelDisplayedInline && !isSingleInputColumn,
        }">
        <input v-model="tempHeader.name"
          class="input is-small action-headers"
          type="text"
          placeholder="Header Name"
          @input="fieldChange( 'headers-name' )" />
      </div>
      <div class="column"
        :class="{
          'is-6': !labelDisplayedInline || isSingleInputColumn,
          'is-5': labelDisplayedInline && !isSingleInputColumn,
        }">
        <input v-model="tempHeader.value"
          class="input is-small action-headers-value"
          type="text"
          placeholder="Header Value"
          @input="fieldChange( 'headers-value' )" />
      </div>
    </div>
    <div class="content"
      v-if="localAction && localAction.type === 'ban' && localAction.params.action">
      <response-action
        :action.sync="localAction.params.action"
        :label-separated-line="labelSeparatedLine"
        :is-single-input-column="isSingleInputColumn"
        :ignore="['ban']"
        @update:action="emitActionUpdate"
        label="Ban action" />
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
  'ban': {'title': 'Ban', 'params': {'ttl': '[0-9]+', 'action': {'type': 'default', 'params': {}}}},
  'request_header': {'title': 'Header', 'params': {'headers': {}}},
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
      tempHeader: {name: '', value: ''},
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
        this.localAction.params.headers = oldParams.headers ? oldParams.headers : {}
      }
      if (this.localAction.type === 'redirect') {
        this.localAction.params.status = oldParams.status ? oldParams.status : ''
        this.localAction.params.location = oldParams.location ? oldParams.location : ''
      }
      if (this.localAction.type === 'ban') {
        this.localAction.params.ttl = oldParams.ttl ? oldParams.ttl : ''
        this.localAction.params.action = oldParams.action ? oldParams.action : {
          type: 'default',
        }
      }
      if (this.localAction.type === 'request_header') {
        this.localAction.params.headers = oldParams.headers ? oldParams.headers : {}
      }
      if (!_.isEqual(this.localAction, this.action)) {
        this.emitActionUpdate()
      }
    },

    fieldChange( field: string ) {
      if (['headers-name', 'headers-value'].includes( field )) {
        this.localAction.params.headers = {}
        this.localAction.params.headers[this.tempHeader.name as string] = this.tempHeader.value
      }
      this.emitActionUpdate()
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
