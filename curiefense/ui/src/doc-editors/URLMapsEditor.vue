<template>
  <div>
    <div class="card">
      <div class="card-content">
        <div class="content">
          <div class="columns">
            <div class="column is-4">
              <div class="field">
                <label class="label is-small">
                  Name
                  <span class="has-text-grey is-pulled-right document-id"
                        title="Document id">
                    {{ localDoc.id }}
                  </span>
                </label>
                <div class="control">
                  <input class="input is-small document-name"
                         title="Document name"
                         placeholder="Document name"
                         @change="emitDocUpdate"
                         v-model="localDoc.name"/>
                </div>
              </div>
              <div class="field">
                <label class="label is-small">
                  Matching Names
                </label>
                <div class="control has-icons-left">
                  <input type="text"
                         class="input is-small document-domain-name"
                         placeholder="(api|service).company.(io|com)"
                         @change="emitDocUpdate"
                         @input="validateInput($event, isSelectedDomainMatchValid)"
                         v-model="localDoc.match"
                         :disabled="localDoc.id === '__default__'"
                         :readonly="localDoc.id === '__default__'"
                         title="Enter a regex to match hosts headers (domain names)">
                  <span class="icon is-small is-left has-text-grey"><i class="fas fa-code"></i></span>
                </div>
              </div>
            </div>
          </div>
          <div class="field px-3">
            <label class="label is-small">
              Path Mapping
            </label>
            <table class="table entries-table">
              <thead>
              <tr>
                <th class="is-size-7 width-50px"></th>
                <th class="is-size-7">Name</th>
                <th class="is-size-7" colspan="2"><span>Match</span>&nbsp;<span><i
                    class="fas fa-sort-alpha-down"></i></span></th>
                <th class="is-size-7">WAF</th>
                <th class="is-size-7">ACL</th>
                <th class="is-size-7" title="Rate limit">RL</th>
                <th></th>
              </tr>
              </thead>
              <tbody v-for="(mapEntry, mapIndex) in localDoc.map" :key="mapIndex">
              <tr @click="changeSelectedMapEntry(mapIndex)"
                  class="has-row-clickable entry-row"
                  :class=" mapEntryIndex === mapIndex ? 'has-background-light borderless' : ''">
                <td class="is-size-7 width-50px has-text-right has-text-grey-light entry-index">
                  {{ mapIndex + 1 }}
                </td>
                <td class="is-size-7 entry-name">
                  {{ mapEntry.name }}
                </td>
                <td class="is-size-7 width-360px ellipsis entry-match"
                    colspan="2"
                    :title="mapEntry.match">
                  {{ mapEntry.match }}
                </td>
                <td class="is-size-7 entry-waf"
                    :class="mapEntry.waf_active ? 'has-text-success' : 'has-text-danger'"
                    :title="mapEntry.waf_active ? 'Active mode' : 'Learning mode'">
                  {{ wafProfileName(mapEntry.waf_profile) ? wafProfileName(mapEntry.waf_profile)[1] : '' }}
                </td>
                <td class="is-size-7 entry-acl"
                    :class="mapEntry.acl_active ? 'has-text-success' : 'has-text-danger'"
                    :title="mapEntry.acl_active ? 'Active mode' : 'Learning mode'">
                  {{ aclProfileName(mapEntry.acl_profile) ? aclProfileName(mapEntry.acl_profile)[1] : '' }}
                </td>
                <td class="is-size-7 entry-rate-limits-count"
                    v-if="existingRateLimitIDs(mapEntry)">
                  {{ existingRateLimitIDs(mapEntry).length }}
                </td>
                <td class="is-size-7"
                    :rowspan="mapEntryIndex === mapIndex ? '2' : '1'">
                  <a class="has-text-grey"
                     title="more details">
                    {{ mapEntryIndex === mapIndex ? 'close' : 'expand' }}
                  </a>
                </td>
              </tr>
              <tr v-if="mapEntryIndex === mapIndex"
                  :class=" mapEntryIndex === mapIndex ? 'has-background-light borderless' : ''"
                  class="expanded current-entry-row">
                <td colspan="10">
                  <div class="card">
                    <div class="card-content">
                      <div class="content">
                        <div class="columns">
                          <div class="column is-8">
                            <div class="field">
                              <label class="label is-small">
                                Name
                              </label>
                              <div class="control">
                                <input class="input is-small current-entry-name"
                                       @input="emitDocUpdate"
                                       type="text"
                                       ref="profileName"
                                       title="Name"
                                       v-model="mapEntry.name"
                                       required>
                              </div>
                            </div>
                            <div class="field">
                              <label class="label is-small">
                                Match
                              </label>
                              <div class="control has-icons-left">
                                <input class="input is-small current-entry-match"
                                       type="text"
                                       @input="emitDocUpdate();
                                               validateInput($event, isSelectedMapEntryMatchValid(mapIndex))"
                                       title="A unique matching regex value, not overlapping other URL Map definitions"
                                       placeholder="Matching domain(s) regex"
                                       required
                                       :disabled="localDoc.id === '__default__' && initialMapEntryMatch === '/'"
                                       :readonly="localDoc.id === '__default__' && initialMapEntryMatch === '/'"
                                       ref="mapEntryMatch"
                                       v-model="mapEntry.match">
                                <span class="icon is-small is-left has-text-grey">
                                  <i class="fas fa-code"></i>
                                </span>
                              </div>
                            </div>
                            <hr/>
                            <p class="title is-6 has-text-grey">
                              Rate Limit Rules
                            </p>
                            <div class="content">
                              <table class="table is-hoverable is-narrow is-fullwidth current-entry-rate-limits-table">
                                <thead>
                                <tr>
                                  <th class="is-size-7">
                                    Rule Name
                                  </th>
                                  <th class="is-size-7">
                                    Description
                                  </th>
                                  <th class="is-size-7">
                                    Threshold
                                  </th>
                                  <th class="is-size-7">
                                    Timeframe
                                  </th>
                                  <th class="has-text-centered is-size-7 width-60px">
                                    <a v-if="limitRuleNames && mapEntry.limit_ids &&
                                             limitRuleNames.length > existingRateLimitIDs(mapEntry).length"
                                       class="has-text-grey-dark is-small rate-limit-add-button"
                                       title="Add new"
                                       tabindex="0"
                                       @click="limitNewEntryModeMapEntryId = mapIndex"
                                       @keypress.space.prevent
                                       @keypress.space="limitNewEntryModeMapEntryId = mapIndex"
                                       @keypress.enter="limitNewEntryModeMapEntryId = mapIndex">
                                      <span class="icon is-small"><i class="fas fa-plus"></i></span>
                                    </a>
                                  </th>
                                </tr>
                                </thead>
                                <tbody>
                                <template v-for="(limitId, limitIndex) in mapEntry.limit_ids">
                                  <tr v-if="limitDetails(limitId)"
                                      :key="limitId"
                                      class="rate-limit-row">
                                    <td class="is-size-7 rate-limit-name"
                                        v-if="limitDetails(limitId)">
                                      {{ limitDetails(limitId).name }}
                                    </td>
                                    <td class="is-size-7 rate-limit-description"
                                        v-if="limitDetails(limitId)">
                                      {{ limitDetails(limitId).description }}
                                    </td>
                                    <td class="is-size-7 rate-limit-threshold"
                                        v-if="limitDetails(limitId)">
                                      {{ limitDetails(limitId).limit }}
                                    </td>
                                    <td class="is-size-7 rate-limit-ttl"
                                        v-if="limitDetails(limitId)">
                                      {{ limitDetails(limitId).ttl }}
                                    </td>
                                    <td class="has-text-centered is-size-7 width-60px">
                                      <a class="is-small has-text-grey rate-limit-remove-button"
                                         title="Remove entry"
                                         tabindex="0"
                                         @click="removeRateLimitFromEntry(mapEntry, limitIndex)"
                                         @keypress.space.prevent
                                         @keypress.space="removeRateLimitFromEntry(mapEntry, limitIndex)"
                                         @keypress.enter="removeRateLimitFromEntry(mapEntry, limitIndex)">
                                        remove
                                      </a>
                                    </td>
                                  </tr>
                                </template>
                                <tr v-if="limitNewEntryMode(mapIndex)"
                                    class="new-rate-limit-row">
                                  <td colspan="4">
                                    <div class="control is-expanded">
                                      <div class="select is-small is-size-7 is-fullwidth">
                                        <select class="select is-small new-rate-limit-selection"
                                                title="Rate limit ID"
                                                v-model="limitMapEntryId">
                                          <option v-for="rule in newLimitRules(mapEntry.limit_ids)" :key="rule.id"
                                                  :value="rule.id">{{ rule.name + ' ' + rule.description }}
                                          </option>
                                        </select>
                                      </div>
                                    </div>
                                  </td>
                                  <td class="has-text-centered is-size-7 width-60px">
                                    <a class="is-small has-text-grey rate-limit-confirm-add-button"
                                       title="Add this entry"
                                       tabindex="0"
                                       @click="addRateLimitToEntry(mapEntry, limitMapEntryId)"
                                       @keypress.space.prevent
                                       @keypress.space="addRateLimitToEntry(mapEntry, limitMapEntryId)"
                                       @keypress.enter="addRateLimitToEntry(mapEntry, limitMapEntryId)">
                                      add
                                    </a>
                                  </td>
                                </tr>
                                <tr v-if="mapEntry.limit_ids && !existingRateLimitIDs(mapEntry).length">
                                  <td colspan="5">
                                    <p class="is-size-7 has-text-grey has-text-centered">
                                      To attach an existing rule, click
                                      <a class="rate-limit-text-add-button"
                                         title="Add New"
                                         @click="limitNewEntryModeMapEntryId = mapIndex">here</a>.
                                      <br/>
                                      To create a new rate-limit rule, click
                                      <a class="rate-limit-referral-button"
                                         @click="referToRateLimit">here</a>.
                                    </p>
                                  </td>
                                </tr>
                                </tbody>
                              </table>
                            </div>
                          </div>
                          <div class="column is-4">
                            <div class="field">
                              <label class="label is-small">WAF Policy</label>
                              <div class="control is-expanded">
                                <div class="select is-fullwidth is-small">
                                  <select v-model="mapEntry.waf_profile"
                                          @change="emitDocUpdate"
                                          class="current-entry-waf-selection"
                                          title="WAF policy">
                                    <option v-for="waf in wafProfileNames"
                                            :value="waf[0]"
                                            :key="waf[0]">
                                      {{ waf[1] }}
                                    </option>
                                  </select>
                                </div>
                              </div>
                            </div>
                            <div class="field">
                              <label class="checkbox is-size-7">
                                <input type="checkbox"
                                       @change="emitDocUpdate"
                                       class="current-entry-waf-active"
                                       v-model="mapEntry.waf_active">
                                Active Mode
                              </label>
                            </div>
                            <hr/>
                            <div class="field">
                              <label class="label is-small">
                                ACL Policy
                              </label>
                              <div class="control is-expanded">
                                <div class="select is-fullwidth is-small">
                                  <select v-model="mapEntry.acl_profile"
                                          @change="emitDocUpdate"
                                          class="current-entry-acl-selection"
                                          title="ACL policy">
                                    <option v-for="acl in aclProfileNames" :value="acl[0]" :key="acl[0]">
                                      {{ acl[1] }}
                                    </option>
                                  </select>
                                </div>
                              </div>
                            </div>
                            <div class="field">
                              <label class="checkbox is-size-7">
                                <input type="checkbox"
                                       @change="emitDocUpdate"
                                       class="current-entry-acl-active"
                                       v-model="mapEntry.acl_active">
                                Active Mode
                              </label>
                            </div>
                            <hr/>
                            <div class="field">
                              <button title="Create a new profile based on this one"
                                      class="button is-small is-pulled-left is-light fork-entry-button"
                                      @click="addNewProfile(mapEntry, mapIndex)">
                                <span class="icon"><i class="fas fa-code-branch"></i></span>
                                <span>
                                Fork profile
                              </span>
                              </button>
                              <button title="Delete this profile"
                                      class="button is-small is-pulled-right is-danger is-light remove-entry-button"
                                      @click="removeMapEntry(mapIndex)"
                                      v-if="localDoc.id !== '__default__' || initialMapEntryMatch !== '/'">
                                Delete
                              </button>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </td>
              </tr>
              </tbody>
            </table>
          </div>
          <span class="is-family-monospace has-text-grey-lighter">{{ apiPath }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import _ from 'lodash'
import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import RequestsUtils from '@/assets/RequestsUtils.ts'
import Vue, {VueConstructor} from 'vue'
import {ACLPolicy, RateLimit, URLMap, URLMapEntryMatch, WAFPolicy} from '@/types'
import {AxiosResponse} from 'axios'
import Utils from '@/assets/Utils'

export default (Vue as VueConstructor<Vue & {
  $refs: {
    profileName: HTMLInputElement[]
    mapEntryMatch: HTMLInputElement[]
  }
}>).extend({
  name: 'URLMapsEditor',

  props: {
    selectedDoc: Object,
    selectedBranch: String,
    docs: Array,
    apiPath: String,
  },


  data() {
    return {
      mapEntryIndex: -1,

      // for URLMap drop downs
      wafProfileNames: [] as [WAFPolicy['id'], WAFPolicy['name']][],
      aclProfileNames: [] as [ACLPolicy['id'], ACLPolicy['name']][],
      limitRuleNames: [] as RateLimit[],
      domainNames: [] as URLMap['match'][],
      entriesMatchNames: [] as URLMapEntryMatch['match'][],

      limitNewEntryModeMapEntryId: null,
      limitMapEntryId: null,
      initialDocDomainMatch: '',
      initialMapEntryMatch: '',
      upstreams: [],
    }
  },

  computed: {
    localDoc(): URLMap {
      return _.cloneDeep(this.selectedDoc)
    },

    isFormInvalid(): boolean {
      const isDomainMatchValid = this.isSelectedDomainMatchValid()
      // Entries are reverted to valid state on close, so if no entry is opened they are valid
      const isCurrentEntryMatchValid = this.mapEntryIndex === -1 ||
          this.isSelectedMapEntryMatchValid(this.mapEntryIndex)
      return !isDomainMatchValid || !isCurrentEntryMatchValid
    },
  },

  methods: {
    emitDocUpdate(): void {
      this.$emit('update:selectedDoc', this.localDoc)
    },

    emitCurrentDocInvalidity(): void {
      this.$emit('form-invalid', this.isFormInvalid)
    },

    validateInput(event: Event, validator: Function | boolean) {
      const isValid = Utils.validateInput(event, validator)
      if (!isValid) {
        this.$emit('form-invalid', true)
      } else {
        this.emitCurrentDocInvalidity()
      }
    },

    isSelectedDomainMatchValid(): boolean {
      const newDomainMatch = this.localDoc.match?.trim()
      const isDomainMatchEmpty = newDomainMatch === ''
      const isDomainMatchDuplicate = this.domainNames.includes(
          newDomainMatch) ? this.initialDocDomainMatch !== newDomainMatch : false
      return !isDomainMatchEmpty && !isDomainMatchDuplicate
    },

    isSelectedMapEntryMatchValid(index: number): boolean {
      const newMapEntryMatch = this.localDoc.map[index] ? this.localDoc.map[index].match.trim() : ''
      const isMapEntryMatchEmpty = newMapEntryMatch === ''
      const isMapEntryMatchDuplicate = this.entriesMatchNames.includes(
          newMapEntryMatch) ? this.initialMapEntryMatch !== newMapEntryMatch : false
      return !isMapEntryMatchEmpty && !isMapEntryMatchDuplicate
    },

    aclProfileName(id: string): [string, string] {
      return _.find(this.aclProfileNames, (profile) => {
        return profile[0] === id
      })
    },

    wafProfileName(id: string): [string, string] {
      return _.find(this.wafProfileNames, (profile) => {
        return profile[0] === id
      })
    },

    newLimitRules(currentRateLimitIDs: string[]): RateLimit[] {
      return _.filter(this.limitRuleNames, (rule) => {
        return _.indexOf(currentRateLimitIDs, rule.id) === -1
      })
    },

    addRateLimitToEntry(mapEntry: URLMapEntryMatch, id: string) {
      if ( id ) {
        mapEntry.limit_ids.push(id)
        this.limitNewEntryModeMapEntryId = null
        this.limitMapEntryId = null
        this.emitDocUpdate()
      }
    },

    removeRateLimitFromEntry(mapEntry: URLMapEntryMatch, index: number) {
      mapEntry.limit_ids.splice(index, 1)
      this.emitDocUpdate()
    },

    limitDetails(limitId: string): RateLimit {
      return _.find(this.limitRuleNames, (rule) => {
        return rule.id === limitId
      })
    },

    limitNewEntryMode(id: number): boolean {
      return this.limitNewEntryModeMapEntryId === id
    },

    existingRateLimitIDs(mapEntry: URLMapEntryMatch): RateLimit['id'][] {
      return _.filter(mapEntry.limit_ids, (limitId) => {
        return this.limitDetails(limitId) !== undefined
      })
    },

    addNewProfile(map: URLMapEntryMatch, idx: number) {
      const mapEntry = _.cloneDeep(map)
      const randomUniqueString = DatasetsUtils.generateUUID2()
      mapEntry.name = 'New Security Profile'
      mapEntry.match = `/new/path/to/match/profile/${randomUniqueString}`

      // reverting the entry match to a stable and valid state if invalid
      if (!this.isSelectedMapEntryMatchValid(idx)) {
        this.localDoc.map[idx].match = this.initialMapEntryMatch
        Utils.clearInputValidationClasses(this.$refs.mapEntryMatch[0])
        this.emitCurrentDocInvalidity()
      }
      this.localDoc.map.splice(idx, 0, mapEntry)
      this.emitDocUpdate()
      const element = this.$refs.profileName[0] as HTMLInputElement
      this.initialMapEntryMatch = mapEntry.match
      this.entriesMatchNames = _.map(this.localDoc.map, 'match')
      // Pushing the select action to the end of queue in order for the new profile to be rendered beforehand
      setImmediate(() => {
        element.select()
        element.focus()
      })
    },

    changeSelectedMapEntry(index: number) {
      // reverting the entry match to a stable and valid state if invalid on close
      if (this.mapEntryIndex !== -1 && !this.isSelectedMapEntryMatchValid(this.mapEntryIndex)) {
        if (this.localDoc.map[this.mapEntryIndex]) {
          this.localDoc.map[this.mapEntryIndex].match = this.initialMapEntryMatch
        }
        this.mapEntryIndex = (this.mapEntryIndex === index ? -1 : index)
        Utils.clearInputValidationClasses(this.$refs.mapEntryMatch[0])
        this.emitDocUpdate()
        this.emitCurrentDocInvalidity()
      } else {
        this.mapEntryIndex = (this.mapEntryIndex === index ? -1 : index)
      }
      this.initialMapEntryMatch = this.localDoc.map[index] ? this.localDoc.map[index].match.trim() : ''
      this.entriesMatchNames = _.map(this.localDoc.map, 'match')
    },

    removeMapEntry(index: number) {
      this.localDoc.map.splice(index, 1)
      this.changeSelectedMapEntry(-1)
    },

    referToRateLimit() {
      this.$emit('form-invalid', false)
      this.$router.push(`/config/${this.selectedBranch}/ratelimits`)
    },

    wafacllimitProfileNames() {
      const branch = this.selectedBranch

      RequestsUtils.sendRequest({
        methodName: 'GET',
        url: `configs/${branch}/d/wafpolicies/`,
        config: {headers: {'x-fields': 'id, name'}},
      }).then((response: AxiosResponse<WAFPolicy[]>) => {
        this.wafProfileNames = _.sortBy(_.map(response.data, (entity) => {
          return [entity.id, entity.name]
        }), (e) => {
          return e[1]
        })
      })

      RequestsUtils.sendRequest({
        methodName: 'GET',
        url: `configs/${branch}/d/aclpolicies/`,
        config: {headers: {'x-fields': 'id, name'}},
      }).then((response: AxiosResponse<ACLPolicy[]>) => {
        this.aclProfileNames = _.sortBy(_.map(response.data, (entity) => {
          return [entity.id, entity.name]
        }), (e) => {
          return e[1]
        })
      })

      RequestsUtils.sendRequest({
        methodName: 'GET',
        url: `configs/${branch}/d/ratelimits/`,
      }).then((response: AxiosResponse<RateLimit[]>) => {
        this.limitRuleNames = response.data
      })
    },

    urlMapsDomainMatches() {
      const branch = this.selectedBranch
      RequestsUtils.sendRequest({
        methodName: 'GET',
        url: `configs/${branch}/d/urlmaps/`,
        config: {headers: {'x-fields': 'match'}},
      }).then((response: AxiosResponse<URLMap[]>) => {
        this.domainNames = _.map(response.data, 'match')
      })
    },
  },

  watch: {
    selectedDoc: {
      handler: function(val, oldVal) {
        if (!val || !oldVal || val.id !== oldVal.id) {
          this.wafacllimitProfileNames()
          this.urlMapsDomainMatches()
          this.initialDocDomainMatch = this.selectedDoc.match
        }
      },
      immediate: true,
      deep: true,
    },
  },
})
</script>
<style scoped lang="scss">

.has-row-clickable > td {
  cursor: pointer;
}

.borderless > td {
  border-bottom-width: 0;
  padding-top: 8px;
}

.expanded > td {
  padding-bottom: 20px;
}

.new-rate-limit-row > td {
  vertical-align: middle;
}

tr:last-child > td {
  border-bottom-width: 1px;
}

.borderless:last-child > td {
  border-bottom-width: 0;
}

</style>
