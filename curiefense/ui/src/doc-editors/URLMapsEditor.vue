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
                  <span class="has-text-grey is-pulled-right document-id" title="Document id">
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
                         class="input is-small"
                         placeholder="(api|service).company.(io|com)"
                         @change="emitDocUpdate"
                         v-model="localDoc.match"
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
            <table class="table">
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
              <tbody v-for="(mapEntry, idx) in localDoc.map" :key="idx">
              <tr @click="changeSelectedMapEntry(idx)"
                  class="has-row-clickable"
                  :class=" mapEntryIndex === idx ? 'has-background-light borderless' : ''">
                <td class="is-size-7 width-50px has-text-right has-text-grey-light">
                  {{ idx + 1 }}
                </td>
                <td class="is-size-7">
                  {{ mapEntry.name }}
                </td>
                <td class="is-size-7 width-360px ellipsis"
                    colspan="2"
                    :title="mapEntry.match">
                  {{ mapEntry.match }}
                </td>
                <td class="is-size-7 "
                    :class=" mapEntry.waf_active ? 'has-text-success' : 'has-text-danger' "
                    :title=" mapEntry.waf_active ? 'Active mode' : 'Learning mode' ">
                  {{ wafProfileName(mapEntry.waf_profile) ? wafProfileName(mapEntry.waf_profile)[1] : '' }}
                </td>
                <td class="is-size-7 has-text-success"
                    :class=" mapEntry.acl_active ? 'has-text-success' : 'has-text-danger' "
                    :title=" mapEntry.acl_active ? 'Active mode' : 'Learning mode' ">
                  {{ aclProfileName(mapEntry.acl_profile) ? aclProfileName(mapEntry.acl_profile)[1] : '' }}
                </td>
                <td class="is-size-7"
                    v-if="mapEntry.limit_ids">
                  {{ mapEntry.limit_ids.length }}
                </td>
                <td class="is-size-7"
                    :rowspan="mapEntryIndex === idx ? '2' : '1'">
                  <a class="has-text-grey"
                     title="more details">
                    {{ mapEntryIndex === idx ? 'close' : 'expand' }}
                  </a>
                </td>
              </tr>
              <tr v-if="mapEntryIndex === idx"
                  :class=" mapEntryIndex === idx ? 'has-background-light borderless' : ''"
                  class="expanded">
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
                                <input class="input is-small"
                                       @change="emitDocUpdate"
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
                                <input class="input is-small" type="text"
                                       @change="emitDocUpdate"
                                       title="Match regex"
                                       placeholder="matching domain(s) regex"
                                       required
                                       :disabled="mapEntry.match === '__default__'"
                                       :readonly="mapEntry.match === '__default__'"
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
                              <table class="table is-hoverable is-narrow is-fullwidth">
                                <thead>
                                <tr>
                                  <th class="is-size-7"></th>
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
                                             limitRuleNames.length > mapEntry.limit_ids.length"
                                       class="has-text-grey-dark is-small"
                                       title="Add new"
                                       tabindex="0"
                                       @click="limitNewEntryModeMapEntryId = idx"
                                       @keypress.space.prevent
                                       @keypress.space="limitNewEntryModeMapEntryId = idx"
                                       @keypress.enter="limitNewEntryModeMapEntryId = idx">
                                      <span class="icon is-small"><i class="fas fa-plus"></i></span>
                                    </a>
                                  </th>
                                </tr>
                                </thead>
                                <tbody>
                                <tr v-for="(limitId, idx) in mapEntry.limit_ids"
                                    :key="limitId"
                                    :class="{ 'highlighted': rateLimitAnalyzed ?
                                              rateLimitAnalyzed.id === limitId : false }">
                                  <td class="is-size-7">
                                    <a class="has-text-grey-dark is-small"
                                       title="Analyze recommended rate limit values"
                                       @click="calcRateLimitRecommendation(mapEntry, limitDetails(limitId))">
                                      <span class="icon is-small"><i class="fas fa-chart-line"></i></span>
                                    </a>
                                  </td>
                                  <td class="is-size-7" v-if="limitDetails(limitId)">
                                    {{ limitDetails(limitId).name }}
                                  </td>
                                  <td class="is-size-7" v-if="limitDetails(limitId)">
                                    {{ limitDetails(limitId).description }}
                                  </td>
                                  <td class="is-size-7" v-if="limitDetails(limitId)">
                                    {{ limitDetails(limitId).limit }}
                                  </td>
                                  <td class="is-size-7" v-if="limitDetails(limitId)">
                                    {{ limitDetails(limitId).ttl }}
                                  </td>
                                  <td class="has-text-centered is-size-7 width-60px">
                                    <a class="is-small has-text-grey" title="Remove entry"
                                       tabindex="0"
                                       @click="removeLimitEntry(mapEntry, idx)"
                                       @keypress.space.prevent
                                       @keypress.space="removeLimitEntry(mapEntry, idx)"
                                       @keypress.enter="removeLimitEntry(mapEntry, idx)">
                                      remove
                                    </a>
                                  </td>
                                </tr>
                                <tr v-if="limitNewEntryMode(idx)">
                                  <td colspan="4">
                                    <div class="control is-expanded">
                                      <div class="select is-small is-size-7 is-fullwidth">
                                        <select class="select is-small"
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
                                    <a class="is-small has-text-grey" title="Add this entry"
                                       tabindex="0"
                                       @click="addLimitEntry(mapEntry, limitMapEntryId)"
                                       @keypress.space.prevent
                                       @keypress.space="addLimitEntry(mapEntry, limitMapEntryId)"
                                       @keypress.enter="addLimitEntry(mapEntry, limitMapEntryId)">
                                      add
                                    </a>
                                  </td>
                                </tr>
                                <tr v-if="mapEntry.limit_ids && mapEntry.limit_ids.length === 0 ">
                                  <td colspan="5">
                                    <p class="is-size-7 has-text-grey has-text-centered">
                                      To attach an existing rule, click
                                      <a title="Add New"
                                         @click="limitNewEntryModeMapEntryId = idx">here</a>.
                                      <br/>
                                      To create a new rate-limit rule, click <a @click="referToRateLimit">here</a>.
                                    </p>
                                  </td>
                                </tr>
                                </tbody>
                              </table>
                            </div>
                            <div class="content is-size-7 has-text-grey">
                              <span v-if="rateLimitRecommendationStatus === 'info' || !rateLimitAnalyzed">
                                Click on the icon next to a rate limit rule to analyze it
                                and calculate recommended threshold.
                              </span>
                              <span v-if="rateLimitRecommendationStatus === 'recommend' && rateLimitAnalyzed">
                                Recommended rate limit threshold based on data from the last seven days is
                                <b>{{ rateLimitRecommendation }}</b>.
                                <span v-if="rateLimitAnalyzed.limit === rateLimitRecommendation">
                                  <br/>
                                  You are currently using the recommended threshold for this rule.
                                </span>
                                <span v-else>
                                  Click
                                  <a title="Apply Recommended Rate Limit" @click="applyRateLimitRecommendation">here</a>
                                  to apply.
                                  <br/>
                                  <span v-if="isRateLimitReferencedElsewhere(rateLimitAnalyzed.id, mapEntryAnalyzed)">
                                    Please notice! There are other URL map entries using this rate limit rule,
                                    this action will create a copy of the rate limit rule.
                                  </span>
                                  <span v-else>
                                    Please notice! This action will modify the rate limit rule itself.
                                  </span>
                                </span>
                              </span>
                              <span v-if="rateLimitRecommendationStatus === 'empty'">
                                Server returned no data while trying to calculate recommended threshold.
                              </span>
                              <span v-if="rateLimitRecommendationStatus === 'error'"
                                    class="has-text-danger has-background-danger-light">
                                Encountered an error while trying to analyze recommended rate limit,
                                please try again later.
                              </span>
                              <span v-if="rateLimitRecommendationStatus === 'loading'">
                                <button class="button is-outlined is-text is-small is-loading"
                                        v-if="rateLimitRecommendationLoading">Loading</button>
                              </span>
                            </div>
                          </div>
                          <div class="column is-4">
                            <div class="field">
                              <label class="label is-small">WAF Policy</label>
                              <div class="control is-expanded">
                                <div class="select is-fullwidth is-small">
                                  <select v-model="mapEntry.waf_profile"
                                          @change="emitDocUpdate"
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
                                       v-model="mapEntry.acl_active">
                                Active Mode
                              </label>
                            </div>
                            <hr/>
                            <div class="field">
                              <button title="Create a new profile based on this one"
                                      class="button is-small is-pulled-left is-light"
                                      @click="addNewProfile(mapEntry, idx)">
                                <span class="icon"><i class="fas fa-code-branch"></i></span>
                                <span>
                                Fork profile
                              </span>
                              </button>
                              <button title="Delete this profile"
                                      class="button is-small is-pulled-right is-danger is-light"
                                      @click="removeMapEntry(idx)"
                                      v-if="mapEntry.name !== 'default'">
                                delete
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
import {ACLPolicy, LimitRuleType, RateLimit, URLMap, URLMapEntryMatch, WAFPolicy} from '@/types'
import {AxiosResponse} from 'axios'

export default (Vue as VueConstructor<Vue & {
  $refs: {
    profileName: HTMLInputElement[]
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
      wafProfileNames: [] as [string, string][],
      aclProfileNames: [] as [string, string][],
      limitRuleNames: [] as RateLimit[],

      limitNewEntryModeMapEntryId: null,
      limitMapEntryId: null,

      upstreams: [],

      rateLimitRecommendation: null,
      rateLimitAnalyzed: null,
      mapEntryAnalyzed: null,
      rateLimitRecommendationStatus: 'info', // info | recommend | error | loading | empty
      rateLimitRecommendationLoading: false,
    }
  },

  computed: {
    localDoc(): URLMap {
      return _.cloneDeep(this.selectedDoc)
    },
  },

  methods: {
    emitDocUpdate(): void {
      this.$emit('update:selectedDoc', this.localDoc)
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

    newLimitRules(limitIds: string[]): RateLimit[] {
      return _.filter(this.limitRuleNames, (rule) => {
        return _.indexOf(limitIds, rule.id) === -1
      })
    },

    limitDetails(limitId: string): RateLimit {
      return _.find(this.limitRuleNames, (rule) => {
        return rule.id === limitId
      })
    },

    limitNewEntryMode(id: number): boolean {
      return this.limitNewEntryModeMapEntryId === id
    },

    addNewProfile(map: URLMapEntryMatch, idx: number) {
      const mapEntry = _.cloneDeep(map)
      mapEntry.name = 'New Security Profile'
      mapEntry.match = '/new/path/to/match/profile'

      this.localDoc.map.splice(idx, 0, mapEntry)
      this.emitDocUpdate()
      const element = this.$refs.profileName[0]
      // Pushing the select action to the end of queue in order for the new profile to be rendered beforehand
      setImmediate(() => {
        element.select()
        element.focus()
      })
    },

    changeSelectedMapEntry(index: number) {
      this.mapEntryIndex = (this.mapEntryIndex === index ? -1 : index)
      this.clearRateLimitRecommendation()
    },

    formatRateLimitAnalysisData(obj: RateLimit['include'] | RateLimit['exclude']) {
      const mappedData = _.flatMapDeep(Object.keys(obj), (key: LimitRuleType) => {
        return _.map(Object.keys(obj[key]), (innerKey) => {
          if (innerKey === 'tags') {
            return [`'${key}'`, `'${innerKey}'`, `'${obj[key][innerKey]}' = '1'`]
          } else {
            return [`'${key}'`, `'${innerKey}' = '${obj[key][innerKey]}'`]
          }
        })
      })
      return _.remove(mappedData, (item) => {
        return item
      })
    },

    clearRateLimitRecommendation() {
      this.rateLimitRecommendationStatus = 'info'
      this.rateLimitAnalyzed = null
      this.mapEntryAnalyzed = null
    },

    calcRateLimitRecommendation(mapEntry: URLMapEntryMatch, rateLimit: RateLimit) {
      this.rateLimitRecommendationStatus = 'loading'
      this.rateLimitAnalyzed = rateLimit
      this.mapEntryAnalyzed = mapEntry
      const formattedIncludeData = this.formatRateLimitAnalysisData(rateLimit.include)
      const formattedExcludeData = this.formatRateLimitAnalysisData(rateLimit.exclude)
      const formattedKeyData = _.map(Object.values(rateLimit.key), (key) => {
        const innerKey = Object.keys(key)[0]
        return [`'${innerKey}'`, `'${key[innerKey]}'`]
      })
      const data = {
        action: 'rate-limit-recommendation',
        parameters: {
          urlmap: this.localDoc.id,
          mapentry: mapEntry.name,
          timeframe: parseInt(rateLimit.ttl),
          include: formattedIncludeData,
          exclude: formattedExcludeData,
          key: formattedKeyData,
        },
      }
      RequestsUtils.sendLogsRequest('POST', 'analyze/', data).then((response: AxiosResponse) => {
        if (!response.data || !response.data[0] || response.data[0].length === 0) {
          this.rateLimitRecommendationStatus = 'empty'
        } else {
          this.rateLimitRecommendationStatus = 'recommend'
          this.rateLimitRecommendation = `${response.data[0].splice(-1)}`
        }
      }).catch(() => {
        this.rateLimitRecommendationStatus = 'error'
      })
    },

    applyRateLimitRecommendation() {
      const recommendedRateLimit = _.cloneDeep(this.rateLimitAnalyzed)
      recommendedRateLimit.limit = this.rateLimitRecommendation
      if (this.isRateLimitReferencedElsewhere(this.rateLimitAnalyzed.id, this.mapEntryAnalyzed)) {
        // ID is referenced, copy rate limit
        recommendedRateLimit.name = 'copy of ' + recommendedRateLimit.name
        recommendedRateLimit.id = DatasetsUtils.generateUUID2()
        RequestsUtils.sendRequest('POST',
            `configs/${this.selectedBranch}/d/ratelimits/e/${recommendedRateLimit.id}`).then(() => {
          _.remove(this.mapEntryAnalyzed.limit_ids, (id) => {
            return id === this.rateLimitAnalyzed.id
          })
          this.mapEntryAnalyzed.limit_ids.push(recommendedRateLimit.id)
        })
        this.clearRateLimitRecommendation()
      } else {
        // ID is not referenced, edit rate limit
        RequestsUtils.sendRequest('PUT', `configs/${this.selectedBranch}/d/ratelimits/e/${recommendedRateLimit.id}`)
        this.clearRateLimitRecommendation()
      }
    },

    isRateLimitReferencedElsewhere(rateLimitID: string, mapEntry: URLMapEntryMatch) {
      let referencedIDs = _.reduce(this.docs, (referencedIDs, doc) => {
        referencedIDs.push(_.reduce((doc as URLMap).map, (entryReferencedIDs, entry: URLMapEntryMatch) => {
          if (entry !== mapEntry) {
            entryReferencedIDs.push(entry.limit_ids)
          }
          return entryReferencedIDs
        }, []))
        return referencedIDs
      }, [])
      referencedIDs = _.uniq(_.flattenDeep(referencedIDs))
      return referencedIDs.includes(rateLimitID)
    },

    referToRateLimit() {
      this.$emit('switch-doc-type', 'limits')
    },

    wafacllimitProfileNames() {
      const branch = this.selectedBranch

      RequestsUtils.sendRequest('GET',
          `configs/${branch}/d/wafpolicies/`).then((response: AxiosResponse<WAFPolicy[]>) => {
        this.wafProfileNames = _.sortBy(_.map(response.data, (entity) => {
          return [entity.id, entity.name]
        }), (e) => {
          return e[1]
        })
      })

      RequestsUtils.sendRequest('GET',
          `configs/${branch}/d/aclpolicies/`).then((response: AxiosResponse<ACLPolicy[]>) => {
        this.aclProfileNames = _.sortBy(_.map(response.data, (entity) => {
          return [entity.id, entity.name]
        }), (e) => {
          return e[1]
        })
      })

      RequestsUtils.sendRequest('GET',
          `configs/${branch}/d/ratelimits/`).then((response: AxiosResponse<RateLimit[]>) => {
        this.limitRuleNames = response.data
      })
    },

    addLimitEntry(mapEntry: URLMapEntryMatch, id: string) {
      mapEntry.limit_ids.push(id)
      this.limitNewEntryModeMapEntryId = null
      this.emitDocUpdate()
    },

    removeLimitEntry(mapEntry: URLMapEntryMatch, index: number) {
      mapEntry.limit_ids.splice(index, 1)
      this.emitDocUpdate()
    },

    removeMapEntry(index: number) {
      this.localDoc.map.splice(index, 1)
      this.emitDocUpdate()
    },
  },

  watch: {
    selectedDoc: {
      handler: function() {
        this.wafacllimitProfileNames()
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
  cursor: pointer;
  padding-top: 8px;
}

.expanded > td {
  padding-bottom: 20px;
}

.highlighted {
  background: #fafafa;
}

tr:last-child > td {
  border-bottom-width: 1px;
}

.borderless:last-child > td {
  border-bottom-width: 0;
}

</style>
