<template>
  <div class="limit-actions">
    <div class="columns is-gapless" style="margin-bottom: .75rem">
      <div class="column is-3">
        <label class="label is-small has-text-left form-label">{{ title }}</label>
      </div>
      <div class="column is-9">
        <div class="columns" style="height: 55%">
          <div class="column">
            <div class="control select is-fullwidth is-small" v-if="action">
              <select v-model="action.type">
                <option v-for="(value, id) in options" :value="id" :key="id">{{ value.title }}</option>
              </select>
            </div>
          </div>
          <div class="column">
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
        </div>
        <div class="columns">
          <div class="column is-12 additional" v-if="action && action.type === 'response'">
            <div class="control is-fullwidth">
              <textarea
                v-model="action.params.content"
                class="textarea is-small"
                rows="2"
                placeholder="Response body">
              </textarea>
            </div>
          </div>
          <div class="column is-12 additional" v-if="action && action.type === 'redirect'">
            <p class="control is-fullwidth" style="margin-top: 3.5%">
              <input
                class="input is-small"
                type="text"
                v-model="action.params.location"
                placeholder="Location">
            </p>
          </div>
        </div>
      </div>
    </div>
    <div class="content" v-if="action && action.type === 'ban' && action.params.action">
      <limit-action
        :action.sync="action.params.action"
        caption="Ban action"
        :ignore="['ban']"
      />
    </div>
  </div>
</template>

<script>
export default {
  name: "LimitAction",
  props: {
    action: Object,
    caption: String,
    ignore: Array
  },

  data() {
    const { LimitActions } = this.$root.dsutils
    const availableActions = {}
    Object.keys(LimitActions).forEach(actionType => {
      availableActions[actionType] = {
        type: actionType,
        params: LimitActions[actionType].params
          ? { ...LimitActions[actionType].params }
          : {}
      }
    })
    const options = this.ld.pickBy({ ...LimitActions }, (value, key) => {
      return !this.ignore || !this.ignore.includes(key)
    })
    return {
      options,
      selectedAction: this.action,
      availableActions,
      title: this.caption || 'Action',
      ignoreActions: this.ignore || []
    }
  },
  methods: {
    sendUpdate() {
      this.$emit('change', { ...this.selectedAction }, this.index)
    }
  }
}
</script>
<style scoped>

  .limit-actions {
    padding: 0.75rem;
  }

  .limit-actions .column.additional {
    padding-top: 0;
  }

</style>
