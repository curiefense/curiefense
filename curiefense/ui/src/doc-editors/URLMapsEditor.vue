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
                    {{ selectedDoc.id }}
                  </span>
                </label>
                <div class="control">
                  <input class="input is-small document-name"
                         placeholder="Document name"
                         v-model="selectedDoc.name"/>
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
                         v-model="selectedDoc.match"
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
                <th class="is-size-7" title="Rate Limit">RL</th>
                <th></th>
              </tr>
              </thead>
              <tbody v-for="(map_entry, idx) in selectedDoc.map" :key="idx">
              <tr @click="changeSelectedMapEntry(idx)"
                  class="has-row-clickable"
                  :class=" map_entry_index === idx ? 'has-background-light borderless' : ''">
                <td class="is-size-7 width-50px has-text-right has-text-grey-light">
                  {{ idx + 1 }}
                </td>
                <td class="is-size-7">
                  {{ map_entry.name }}
                </td>
                <td class="is-size-7 width-360px ellipsis"
                    colspan="2"
                    :title="map_entry.match">
                  {{ map_entry.match }}
                </td>
                <td class="is-size-7 "
                    :class=" map_entry.waf_active ? 'has-text-success' : 'has-text-danger' "
                    :title=" map_entry.waf_active ? 'Active mode' : 'Learning mode' ">
                  {{ waf_profile_name(map_entry.waf_profile) ? waf_profile_name(map_entry.waf_profile)[1] : '' }}
                </td>
                <td class="is-size-7 has-text-success"
                    :class=" map_entry.acl_active ? 'has-text-success' : 'has-text-danger' "
                    :title=" map_entry.acl_active ? 'Active mode' : 'Learning mode' ">
                  {{ acl_profile_name(map_entry.acl_profile) ? acl_profile_name(map_entry.acl_profile)[1] : '' }}
                </td>
                <td class="is-size-7"
                    v-if="map_entry.limit_ids">
                  {{ map_entry.limit_ids.length }}
                </td>
                <td class="is-size-7"
                    :rowspan="map_entry_index === idx ? '2' : '1'">
                  <a class="has-text-grey"
                     title="more details">
                    {{ map_entry_index === idx ? 'close' : 'expand' }}
                  </a>
                </td>
              </tr>
              <tr v-if="map_entry_index === idx"
                  :class=" map_entry_index === idx ? 'has-background-light borderless' : ''"
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
                                <input class="input is-small" type="text" ref="profile_name"
                                       v-model="map_entry.name" required>
                              </div>
                            </div>
                            <div class="field">
                              <label class="label is-small">
                                Match
                              </label>
                              <div class="control">
                                <input class="input is-small" type="text"
                                       placeholder="matching domain(s) regex"
                                       required
                                       :disabled="map_entry.match === '__default__'"
                                       :readonly="map_entry.match === '__default__'"
                                       v-model="map_entry.match">
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
                                    <a v-if="limitRuleNames && map_entry.limit_ids && limitRuleNames.length > map_entry.limit_ids.length"
                                       class="has-text-grey-dark is-small" title="Add New"
                                       @click="limit_new_entry_mode_map_entry_id = map_entry.key">
                                      <span class="icon is-small"><i class="fas fa-plus"></i></span>
                                    </a>
                                  </th>
                                </tr>
                                </thead>
                                <tbody>
                                <tr v-for="(limit_id, idx) in map_entry.limit_ids"
                                    :key="limit_id"
                                    :class="{ 'highlighted': rateLimitAnalyzed ? rateLimitAnalyzed.id === limit_id : false }">
                                  <td class="is-size-7">
                                    <a class="has-text-grey-dark is-small" title="Analyze Recommended Rate Limit Values"
                                       @click="calcRateLimitRecommendation(map_entry, limit_details(limit_id))">
                                      <span class="icon is-small"><i class="fas fa-chart-line"></i></span>
                                    </a>
                                  </td>
                                  <td class="is-size-7" v-if="limit_details(limit_id)">
                                    {{ limit_details(limit_id).name }}
                                  </td>
                                  <td class="is-size-7" v-if="limit_details(limit_id)">
                                    {{ limit_details(limit_id).description }}
                                  </td>
                                  <td class="is-size-7" v-if="limit_details(limit_id)">
                                    {{ limit_details(limit_id).limit }}
                                  </td>
                                  <td class="is-size-7" v-if="limit_details(limit_id)">
                                    {{ limit_details(limit_id).ttl }}
                                  </td>
                                  <td class="has-text-centered is-size-7 width-60px">
                                    <a class="is-small has-text-grey" title="remove entry"
                                       @click="map_entry.limit_ids.splice(idx,1)">
                                      remove
                                    </a>
                                  </td>
                                </tr>
                                <tr v-if="limit_new_entry_mode(map_entry.key)">
                                  <td colspan="4">
                                    <div class="control is-expanded">
                                      <div class="select is-small is-size-7 is-fullwidth">
                                        <select class="select is-small"
                                                v-model="limit_new_entry_id">
                                          <option v-for="rule in newLimitRules(map_entry.limit_ids)" :key="rule.id"
                                                  :value="rule.id">{{ rule.name + ' ' + rule.description }}
                                          </option>
                                        </select>
                                      </div>
                                    </div>
                                  </td>
                                  <td class="has-text-centered is-size-7 width-60px">
                                    <a class="is-small has-text-grey" title="Add this entry"
                                       @click="map_entry.limit_ids.push(limit_new_entry_id); limit_new_entry_mode_map_entry_id = null">
                                      add
                                    </a>
                                  </td>
                                </tr>
                                <tr v-if="map_entry.limit_ids && map_entry.limit_ids.length === 0 ">
                                  <td colspan="5">
                                    <p class="is-size-7 has-text-grey has-text-centered">
                                      To attach an existing rule, click <a title="Add New"
                                                                           @click="limit_new_entry_mode_map_entry_id = map_entry.key">here</a>.
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
                                Click on the icon next to a rate limit rule to analyze it and calculate recommended threshold.
                              </span>
                              <span v-if="rateLimitRecommendationStatus === 'recommend' && rateLimitAnalyzed">
                                Recommended rate limit threshold based on data from the last seven days is <b>{{
                                  rateLimitRecommendation
                                }}</b>.
                                <span v-if="rateLimitAnalyzed.limit === rateLimitRecommendation">
                                  <br/>
                                  You are currently using the recommended threshold for this rule.
                                </span>
                                <span v-else>
                                  Click <a title="Apply Recommended Rate Limit" @click="applyRateLimitRecommendation">here</a> to apply.
                                  <br/>
                                  <span v-if="isRateLimitReferencedElsewhere(rateLimitAnalyzed.id, mapEntryAnalyzed)">
                                    Please notice! There are other URL map entries using this rate limit rule, this action will create a copy of the rate limit rule.
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
                                Encountered an error while trying to analyze recommended rate limit, please try again later.
                              </span>
                              <span v-if="rateLimitRecommendationStatus === 'loading'">
                                <button class="button is-outlined is-text is-small is-loading"
                                        v-if="rateLimitRecommendationLoading">Loading</button>
                              </span>
                            </div>
                          </div>
                          <div class="column is-4">
                            <div class="field">
                              <label class="label is-small">WAF Profile</label>
                              <div class="control is-expanded">
                                <div class="select is-fullwidth is-small">
                                  <select v-model="map_entry.waf_profile">
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
                                       v-model="map_entry.waf_active">
                                Active Mode
                              </label>
                            </div>
                            <hr/>
                            <div class="field">
                              <label class="label is-small">
                                ACL Profile
                              </label>
                              <div class="control is-expanded">
                                <div class="select is-fullwidth is-small">
                                  <select v-model="map_entry.acl_profile">
                                    <option v-for="acl in aclProfileNames" :value="acl[0]" :key="acl[0]">
                                      {{ acl[1] }}
                                    </option>
                                  </select>
                                </div>
                              </div>
                            </div>
                            <div class="field">
                              <label class="checkbox is-size-7">
                                <input type="checkbox" v-model="map_entry.acl_active">
                                Active Mode
                              </label>
                            </div>
                            <hr/>
                            <div class="field">
                              <button title="Create new profile based on this one"
                                      class="button is-small is-pulled-left is-light"
                                      @click="addNewProfile(map_entry, idx)">
                                <span class="icon"><i class="fas fa-code-branch"></i></span>
                                <span>
                                Fork profile
                              </span>
                              </button>
                              <button title="Delete this profile"
                                      class="button is-small is-pulled-right is-danger is-light"
                                      @click="selectedDoc.map.splice(idx, 1)"
                                      v-if="map_entry.name !== 'default'">
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
<script>

import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import RequestsUtils from '@/assets/RequestsUtils'

export default {
  name: 'URLMapsEditor',

  props: {
    selectedDoc: Object,
    selectedBranch: String,
    docs: Array,
    apiPath: String
  },


  data() {
    return {
      map_entry_index: -1,

      // for URLMap drop downs
      wafProfileNames: [],
      aclProfileNames: [],
      limitRuleNames: [],

      limit_new_entry_mode_map_entry_id: null,
      limit_new_entry_id: null,

      upstreams: [],

      rateLimitRecommendation: null,
      rateLimitAnalyzed: null,
      mapEntryAnalyzed: null,
      rateLimitRecommendationStatus: 'info', // info | recommend | error | loading | empty
      rateLimitRecommendationLoading: false,
    }
  },

  methods: {

    acl_profile_name(id) {
      return this.ld.find(this.aclProfileNames, (profile) => {
        return profile[0] === id
      })
    },
    waf_profile_name(id) {
      return this.ld.find(this.wafProfileNames, (profile) => {
        return profile[0] === id
      })
    },

    newLimitRules(limit_ids) {
      return this.ld.filter(this.limitRuleNames, (rule) => {
        return this.ld.indexOf(limit_ids, rule.id) === -1
      })
    },

    limit_details(limit_id) {
      return this.ld.find(this.limitRuleNames, (rule) => {
        return rule.id === limit_id
      })
    },

    limit_new_entry_mode(id) {
      return this.limit_new_entry_mode_map_entry_id === id
    },

    addNewProfile(map, idx) {
      let new_entry = this.ld.cloneDeep(map)
      new_entry.name = 'New Security Profile'
      new_entry.match = '/new/path/to/match/profile'
      new_entry.isnew = true

      this.selectedDoc.map.splice(idx, 0, new_entry)
      let element = this.$refs.profile_name[0]
      element.focus()
      // Pushing the select action to the end of queue in order for the new profile to be rendered beforehand
      setTimeout(() => {
        element.select()
      }, 0)
    },

    changeSelectedMapEntry(index) {
      this.map_entry_index = (this.map_entry_index === index ? -1 : index)
      this.clearRateLimitRecommendation()
    },

    formatRateLimitAnalysisData(obj) {
      const mappedData = this.ld.flatMap(Object.keys(obj), (key) => {
        return this.ld.map(Object.keys(obj[key]), (innerKey) => {
          if (innerKey === 'tags') {
            return [`'${key}'`, `'${innerKey}'`, `'${obj[key][innerKey]}' = '1'`]
          } else {
            return [`'${key}'`, `'${innerKey}' = '${obj[key][innerKey]}'`]
          }
        })
      })
      return this.ld.remove(mappedData, (item) => {
        return item !== ''
      })
    },

    clearRateLimitRecommendation() {
      this.rateLimitRecommendationStatus = 'info'
      this.rateLimitAnalyzed = null
      this.mapEntryAnalyzed = null
    },

    calcRateLimitRecommendation(mapEntry, rateLimit) {
      this.rateLimitRecommendationStatus = 'loading'
      this.rateLimitAnalyzed = rateLimit
      this.mapEntryAnalyzed = mapEntry
      const formattedIncludeData = this.formatRateLimitAnalysisData(rateLimit.include)
      const formattedExcludeData = this.formatRateLimitAnalysisData(rateLimit.exclude)
      const formattedKeyData = this.ld.map(Object.values(rateLimit.key), (key) => {
        const innerKey = Object.keys(key)[0]
        return [`'${innerKey}'`, `'${key[innerKey]}'`]
      })
      RequestsUtils.sendLogsRequest(
          'POST',
          'analyze/',
          {
            action: 'rate-limit-recommendation',
            parameters: {
              urlmap: this.selectedDoc.id,
              mapentry: mapEntry.name,
              timeframe: parseInt(rateLimit.ttl),
              include: formattedIncludeData,
              exclude: formattedExcludeData,
              key: formattedKeyData,
            }
          }
      )
          .then(response => {
            if (!response.data || !response.data[0] || response.data[0].length === 0) {
              this.rateLimitRecommendationStatus = 'empty'
            } else {
              this.rateLimitRecommendationStatus = 'recommend'
              this.rateLimitRecommendation = `${response.data[0].splice(-1)}`
            }
          })
          .catch(() => {
            this.rateLimitRecommendationStatus = 'error'
          })
    },

    applyRateLimitRecommendation() {
      const recommendedRateLimit = this.ld.cloneDeep(this.rateLimitAnalyzed)
      recommendedRateLimit.limit = this.rateLimitRecommendation
      if (this.isRateLimitReferencedElsewhere(this.rateLimitAnalyzed.id, this.mapEntryAnalyzed)) {
        // ID is referenced, copy rate limit
        recommendedRateLimit.name = 'copy of ' + recommendedRateLimit.name
        recommendedRateLimit.id = DatasetsUtils.UUID2()
        RequestsUtils.sendRequest('POST', `configs/${this.selectedBranch}/d/ratelimits/e/${recommendedRateLimit.id}`)
            .then(() => {
              this.ld.remove(this.mapEntryAnalyzed.limit_ids, (id) => {
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

    isRateLimitReferencedElsewhere(rateLimitID, mapEntry) {
      let referencedIDs = this.ld.reduce(this.docs, (referencedIDs, doc) => {
        referencedIDs.push(this.ld.reduce(doc.map, (entryReferencedIDs, entry) => {
          if (entry !== mapEntry) {
            entryReferencedIDs.push(entry.limit_ids)
          }
          return entryReferencedIDs
        }, []))
        return referencedIDs
      }, [])
      referencedIDs = this.ld.uniq(this.ld.flattenDeep(referencedIDs))
      return referencedIDs.includes(rateLimitID)
    },

    referToRateLimit() {
      this.$emit('switch-doc-type', 'limits')
    },

    wafacllimitProfileNames() {
      let branch = this.selectedBranch

      RequestsUtils.sendRequest('GET', `configs/${branch}/d/wafpolicies/`).then((response) => {
        this.wafProfileNames = this.ld.sortBy(this.ld.map(response.data, (entity) => {
          return [entity.id, entity.name]
        }), (e) => {
          return e[1]
        })
      })

      RequestsUtils.sendRequest('GET', `configs/${branch}/d/aclpolicies/`).then((response) => {
        this.aclProfileNames = this.ld.sortBy(this.ld.map(response.data, (entity) => {
          return [entity.id, entity.name]
        }), (e) => {
          return e[1]
        })
      })

      RequestsUtils.sendRequest('GET', `configs/${branch}/d/ratelimits/`).then((response) => {
        this.limitRuleNames = response.data
      })
    },

  },

  watch: {
    selectedDoc() {
      this.wafacllimitProfileNames()
    }
  },

  mounted() {
    this.wafacllimitProfileNames()
  }
}
</script>
<style type="text/css" scoped>

.content table tbody tr:last-child td {
  border-bottom-width: 1px;
}

tr.has-row-clickable > td {
  cursor: pointer
}

tr.borderless > td {
  padding-top: 8px;
  cursor: pointer;
  border-bottom-width: 0;
}

.content table tbody tr.borderless:last-child td {
  border-bottom-width: 0;
}

tr.expanded > td {
  padding-bottom: 20px;
}

tr.expanded > td {
  padding-bottom: 20px;
}

.highlighted {
  background: #fafafa;
}

</style>
