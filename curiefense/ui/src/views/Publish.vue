<template>
  <div class="card">
    <div class="card-content">
      <div class="media">
        <div class="media-content">
          <div class="columns">
            <div class="column is-4">
              <div class="field is-grouped">
                <div class="control" v-if="branchNames.length">
                  <div class="select is-small">
                    <select v-model="selectedBranchName"
                            class="branch-selection"
                            title="Switch branch"
                            @change="switchBranch">
                      <option v-for="name in branchNames"
                              :key="name"
                              :value="name">
                        {{ name }}
                      </option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

            <div class="column">
              <div class="field is-grouped is-pulled-right">
                <div class="control">
                  <span class="is-size-7 version-display">Version: {{ selectedCommit }}</span>
                </div>
                <div class="control">
                  <span class="is-size-7 buckets-display">Buckets: {{ selectedBucketNames.length }}</span>
                </div>
                <p class="control">
                  <button
                      class="button is-small publish-button"
                      :class="{'is-loading': isPublishLoading}"
                      @click="publish"
                      :title="selectedBucketNames.length > 0 ? 'Publish configuration': 'Select one or more buckets'"
                      :disabled="selectedBucketNames.length === 0">
                    <span class="icon is-small">
                      <i class="fas fa-cloud-upload-alt"></i>
                    </span>
                    <span>Publish configuration</span>
                  </button>
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="content">
        <hr/>
        <div class="columns">
          <div class="column">
            <p class="title is-6 is-expanded">Version History</p>
            <table class="table" v-if="gitLog && gitLog.length > 0">
              <tbody>
              <tr @click="selectCommit(commit)"
                  class="commit-row"
                  v-for="commit in commitLines"
                  :key="commit.version"
                  :class="getVersionRowClass(commit.version)">
                <td class="is-size-7">
                  {{ formatDate(commit.date) }} {{ commit.version }}
                  <br/>
                  {{ commit.message }}
                  <br/>
                  <strong>{{ commit.author }}</strong> <i>{{ commit.email }}</i>
                </td>
              </tr>
              <tr v-if="!expanded && gitLog.length > init_max_rows">
                <td>
                  <a class="has-text-grey view-more-button"
                     @click="expanded = true">
                    View More
                  </a>
                </td>
              </tr>
              <tr v-if="expanded && gitLog.length > init_max_rows">
                <td>
                  <a class="has-text-grey view-less-button"
                     @click="expanded = false">
                    View Less
                  </a>
                </td>
              </tr>
              </tbody>
            </table>
          </div>
          <div class="column">
            <p class="title is-6 is-expanded">Target Buckets</p>
            <table class="table" v-if="gitLog && gitLog.length > 0">
              <tbody>
              <tr
                  v-for="bucket in buckets"
                  :key="bucket.name"
                  class="bucket-row"
                  :class="{'has-background-warning-light': !publishMode && selectedBucketNames.includes(bucket.name)}"
                  @click="bucketNameClicked(bucket.name)">
                <td class="is-size-7">
                  <span class="icon is-small is-vcentered">
                    <svg :width="14"
                         :height="14"
                         :viewBox="'0 0 24 24'">
                      <path :d="mdiBucketPath"/>
                    </svg>
                  </span>
                  &nbsp;
                  <span class="is-vcentered">{{ bucket.name }}</span>
                </td>
                <td class="is-size-7">
                  {{ bucket.url }}
                  <p class="has-text-danger" v-if="bucket.publishStatus && !bucket.publishStatus.ok">
                    Error publishing to this bucket: {{ bucket.publishStatus.message }}!
                  </p>
                  <p class="has-text-success" v-if="bucket.publishStatus && bucket.publishStatus.ok">
                    Publish to this bucket has been done successfully!
                  </p>
                </td>
              </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import _ from 'lodash'
import RequestsUtils from '@/assets/RequestsUtils.ts'
import {mdiBucket} from '@mdi/js'
import Vue from 'vue'
import {Branch, Commit} from '@/types'
import {AxiosResponse} from 'axios'
import Utils from '@/assets/Utils'
import DateTimeUtils from '@/assets/DateTimeUtils'

export default Vue.extend({
  name: 'Publish',
  props: {},
  components: {},
  data() {
    return {
      mdiBucketPath: mdiBucket,
      configs: [],
      gitLog: [],
      expanded: false,
      init_max_rows: 5,
      publishMode: false,
      selectedBranchName: null,
      // db/system info
      publishInfo: {buckets: [], branch_buckets: []},
      // reent commit or user clicks
      selectedCommit: null,
      // branch's buckets by default + plus user clicks
      selectedBucketNames: [],
      // buckets which are within an ongoing publish operation
      publishedBuckets: [],
      apiRoot: RequestsUtils.confAPIRoot,
      apiVersion: RequestsUtils.confAPIVersion,
      // loading indicator
      isPublishLoading: false,
    }
  },
  computed: {
    buckets(): any[] {
      if (!this.publishMode) {
        return this.publishInfo.buckets
      }
      return this.publishedBuckets
    },

    commitLines(): Commit[] {
      if (this.expanded) {
        return this.gitLog
      }
      return this.gitLog.slice(0, this.init_max_rows)
    },

    branchNames(): string[] {
      return _.sortBy(_.map(this.configs, 'id'))
    },
  },
  methods: {
    selectCommit(commit: Commit) {
      this.selectedCommit = commit.version
      this.publishMode = false
    },

    formatDate(date: string) {
      return DateTimeUtils.isoToNowFullCuriefenseFormat(date)
    },

    getVersionRowClass(version: string) {
      const classNames = []
      if (version === this.selectedCommit) {
        classNames.push('has-background-warning-light')
        classNames.push('marked')
      }
      return classNames.join(' ')
    },

    loadBranchLogs() {
      const selectedBranch = _.find(this.configs, (conf) => {
        return conf.id === this.selectedBranchName
      })
      this.gitLog = selectedBranch?.logs
      this.selectedCommit = this.gitLog?.[0]?.version || null
    },

    switchBranch() {
      this.loadBranchLogs()
      this.publishMode = false
      Utils.toast(`Switched to branch "${this.selectedBranchName}".`, 'is-info')
      this.setDefaultBuckets()
    },

    setDefaultBuckets() {
      this.selectedBucketNames = []
      if (this.publishInfo?.branch_buckets?.length) {
        const bucketList = _.find(this.publishInfo.branch_buckets, (list) => {
          return list.name === this.selectedBranchName
        })
        if (bucketList) {
          this.selectedBucketNames = _.cloneDeep(_.filter(bucketList.buckets, (bucket) => {
            return _.find(this.publishInfo.buckets, (publishInfoBucket) => {
              return publishInfoBucket.name === bucket
            })
          }))
        }
      }
    },

    loadPublishInfo() {
      RequestsUtils.sendRequest({
        methodName: 'GET',
        url: `db/system/k/publishinfo/`,
        failureMessage: 'Failed while attempting to load publish info',
      }).then((response: AxiosResponse) => {
        this.publishInfo = response?.data
        this.setDefaultBuckets()
      })
    },

    bucketNameClicked(name: string) {
      const index = _.indexOf(this.selectedBucketNames, name)
      if (index > -1) {
        this.selectedBucketNames.splice(index, 1)
      } else {
        this.selectedBucketNames.push(name)
      }
    },

    loadConfigs() {
      // store configs
      RequestsUtils.sendRequest({methodName: 'GET', url: 'configs/'}).then((response: AxiosResponse<Branch[]>) => {
        this.configs = response?.data
        // pick first branch name as selected
        this.selectedBranchName = this.branchNames[0]
        this.loadBranchLogs()
        this.setDefaultBuckets()
      })
    },

    async publish() {
      this.isPublishLoading = true
      this.publishMode = true
      this.publishedBuckets = _.cloneDeep(_.filter(this.publishInfo.buckets, (bucket) => {
        return _.indexOf(this.selectedBucketNames, bucket.name) > -1
      }))

      const failureMessage = 'Failed while attempting to publish branch ' +
          `"${this.selectedBranchName}" version "${this.selectedCommit}".`
      await RequestsUtils.sendRequest({
        methodName: 'PUT',
        url: `tools/publish/${this.selectedBranchName}/v/${this.selectedCommit}/`,
        data: this.buckets,
        failureMessage,
        onFail: () => {
          this.isPublishLoading = false
        },
      }).then((response: AxiosResponse) => {
        this.parsePublishResults(response?.data)
        this.isPublishLoading = false
      })
    },

    parsePublishResults(data: any) {
      if (data?.ok) {
        Utils.toast(
            `Branch "${this.selectedBranchName}" was published with version "${this.selectedCommit}".`,
            'is-success',
        )
      } else {
        Utils.toast(
            `Failed while attempting to publish branch "${this.selectedBranchName}" version "${this.selectedCommit}".`,
            'is-danger',
        )
      }
      _.each(data?.status, (responseStatus) => {
        const index = _.findIndex(this.publishedBuckets, (entry) => {
          return entry.name === responseStatus.name
        })
        if (index > -1) {
          this.publishedBuckets[index].publishStatus = responseStatus
        }
      })
      this.publishedBuckets = _.cloneDeep(this.publishedBuckets)
    },

  },

  created() {
    this.loadConfigs()
    this.loadPublishInfo()
  },

})
</script>

<style scoped lang="scss">
.marked {
  font-weight: 400;
}
</style>
