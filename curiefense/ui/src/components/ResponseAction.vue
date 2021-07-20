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
      <div
        v-if="labelDisplayedInline"
        class="column is-2"
      >
        <label class="label is-small has-text-left form-label">{{ label }}</label>
      </div>
      <div class="column"
           :class="{'is-5': labelDisplayedInline && !isSingleInputColumn,
                    'is-6': !labelDisplayedInline && !isSingleInputColumn,
                    'is-10': labelDisplayedInline && isSingleInputColumn,
                    'is-12': !labelDisplayedInline && isSingleInputColumn}">
        <div class="control select is-fullwidth is-small action-type-selection">
          <select v-model="localAction.type"
                  title="Action type"
                  @change="changeType">
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
          v-if="['response', 'redirect', 'ban'].includes( localAction.type )"
          class="column"
          :class="{'is-5': labelDisplayedInline && !isSingleInputColumn,
                    'is-6': !labelDisplayedInline && !isSingleInputColumn,
                    'is-10 pt-0': labelDisplayedInline && isSingleInputColumn,
                    'is-12 pt-0': !labelDisplayedInline && isSingleInputColumn}">
        <div class="control is-fullwidth">
          <input
              v-if="['response', 'redirect'].includes( localAction.type )"
              class="input is-small action-status"
              :class="{'is-danger': fieldErrors.includes ( 'status' )}"
              type="text"
              v-model="localAction.params.status"
              @blur="fieldChange( 'status' )"
              @input="fieldChange( 'status' )"
              title="Status code"
              placeholder="Status code"
              :ref="localAction.type"
          />
          <span v-else class="suffix seconds-suffix">
            <input
              class="input is-small action-duration"
              :class="{'is-danger': fieldErrors.includes ( 'ttl' )}"
              type="text"
              v-model="localAction.params.ttl"
              @blur="fieldChange( 'ttl' )"
              @input="fieldChange( 'ttl' )"
              title="Duration"
              placeholder="Duration"
              ref="ban"
            />
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
      <template v-if="['response', 'redirect'].includes( localAction.type )">
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
            <input
              class="input is-small action-location"
              :class="{'is-danger': fieldErrors.includes ( 'location' )}"
              type="text"
              v-model="localAction.params.location"
              @blur="fieldChange( 'location' )"
              @input="fieldChange( 'location' )"
              title="Location"
              placeholder="Location"
            />
          </div>
        </div>
      </template>
    </div>
    <div
      v-if="['request_header', 'response'].includes( localAction.type )"
      class="columns"
    >
      <div
        class="column"
        :class="{
          'is-6': !labelDisplayedInline || isSingleInputColumn,
          'is-5 is-offset-2': labelDisplayedInline && !isSingleInputColumn,
        }"
      >
        <input
          v-model="tempHeader.name"
          class="input is-small action-headers"
          :class="{'is-danger': fieldErrors.includes ( 'headers-name' )}"
          type="text"
          placeholder="Header Name"
          @blur="fieldChange( 'headers-name' )"
          @input="fieldChange( 'headers-name' )"
          ref="request_header"
        />
      </div>
      <div
        class="column"
        :class="{
          'is-6': !labelDisplayedInline || isSingleInputColumn,
          'is-5': labelDisplayedInline && !isSingleInputColumn,
        }"
      >
        <input
          v-model="tempHeader.value"
          class="input is-small action-headers-value"
          :class="{'is-danger': fieldErrors.includes ( 'headers-value' )}"
          type="text"
          placeholder="Header Value"
          @blur="fieldChange( 'headers-value' )"
          @input="fieldChange( 'headers-value' )"
        />
      </div>
    </div>
    <div
      class="content"
      v-if="localAction.type === 'ban' && localAction.params.action"
    >
      <response-action
        :action.sync="localAction.params.action"
        :label-separated-line="labelSeparatedLine"
        :is-single-input-column="isSingleInputColumn"
        :ignore="['ban']"
        @update:action="emitActionUpdate"
        @update:invalid="$emit( 'update:invalid', $event )"
        label="Ban action"
      />
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
      tempHeader: {name: '', value: ''},
      fieldErrors: [],
      isValid: true,
    }
  },
  computed: {
    localAction(): ResponseActionType {
      return _.cloneDeep(this.action) || {type: ''}
    },

    labelDisplayedInline(): boolean {
      return !this.labelSeparatedLine && !!this.label
    },
  },
  methods: {
    emitActionUpdate() {
      this.$emit('update:action', this.localAction)
    },

    changeType() {
      this.fieldErrors = []
      this.isValid = true
      this.normalizeActionParams()
      this.removeFromErrors( `headers-name` )
      this.removeFromErrors( `headers-value` )
      if (['response', 'redirect', 'ban', 'request_header'].includes( this.localAction.type )) {
        this.$emit( 'update:invalid', true )
      }
      this.$nextTick( () => (this.$refs[this.localAction.type] as HTMLElement)?.focus() );
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
      if (this.action && !_.isEqual(this.localAction, this.action)) {
        this.emitActionUpdate()
      }
    },

    fieldChange( field: string ) {
      if ( ['headers-name', 'headers-value'].includes( field )) {
        this.localAction.params.headers = {}
        this.localAction.params.headers[this.tempHeader.name as string] = this.tempHeader.value
      }
      this.emitActionUpdate()
      this.validate()
    },

    validate() {
      const message: string[] = []

      for ( const [name, val] of Object.entries( this.localAction.params || {} )) {
        const value = val as string
        if ( name === 'status' ) {
          this.removeFromErrors( name )
          if ( !value.match( /^[1-5][0-9][0-9]$/ )) {
            this.fieldErrors.push( name )
            message.push(
              value === '' ? 'Status field shouldn\'t be empty' : 'Status doesn\'t match ^[1-5][0-9][0-9]$',
            )
          }
        } else if ( name === 'ttl' ) {
          const numValue = Number( value.trim() )
          this.removeFromErrors( 'ttl' )
          if ( !value || !/^([1-9][0-9]+|[0-9])?$/.test( value ) || numValue > 86400 ) {
            this.fieldErrors.push( 'ttl' )
            if ( numValue > 86400 ) {
              message.push( 'Max recommended ban time frame is 86400 seconds (24H)' )
            } else {
              message.push(
                !value.trim() ? 'Ban time frame field shouldn\'t be empty' : 'Ban time frame should be numeric',
              )
            }
          }
        } else if ( name !== 'headers' ) {
          this.removeFromErrors( name )
          const isIncorrectLocation = name === 'location' && !/^https?:\/\/.+$/.test( value )
          if ( isIncorrectLocation ) {
            this.fieldErrors.push( name )
            message.push(
              value === '' ? 'Location field shouldn\'t be empty' : 'Location doesn\'t match ^https?://.+$',
            )
          }
        } else {
          ['name', 'value'].forEach( (field: string) => {
            this.removeFromErrors( `headers-${field}` )
            if ( !/^[0-9a-zA-Z-_]+$/.test( this.tempHeader[field as 'name'|'value'])) {
              this.fieldErrors.push( `headers-${field}` )
              message.push(
                value === '' ? `Header ${field} shouldn't be empty` : `Header ${field} doesn't match ^[0-9a-zA-Z-_]+$`,
              )
            }
          })
        }
      }
      if ( message.length ) {
        // to be changed with some errors displaying mechanism
        console.error( message.join( '<br />' ), 'is-danger' )
      }
      this.isValid = !this.fieldErrors.length
    },

    removeFromErrors( val: string ) {
      const index = this.fieldErrors.indexOf( val )
      if ( index > -1 ) {
        this.fieldErrors.splice( index, 1 )
      }
    },
  },
  watch: {
    action: {
      handler( value ) {
        if (!value) {
          this.$emit('update:action', {type: 'default'})
          return
        } else {
          const {type, params} = value
          if ( ['request_header', 'response'].includes( type )) {
            const header = Object.entries( params?.headers || {} )?.[0] as string[]
            this.tempHeader.name = header?.[0] || ''
            this.tempHeader.value = header?.[1] || ''
          }
        }
        // adding necessary fields to action params field
        this.normalizeActionParams()
      },
      immediate: true,
      deep: true,
    },
    isValid: {
      handler( val ) {
        this.$emit( 'update:invalid', !val )
      },
    },
  },
})
</script>
<style scoped lang="scss">

.response-actions .column.additional {
  padding-top: 0;
}

</style>
