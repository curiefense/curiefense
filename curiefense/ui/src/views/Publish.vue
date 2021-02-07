<template>
  <div class="card">
    <div class="card-content">
      <div class="media">
        <div class="media-content">
          <div class="columns">
            <div class="column is-4">
              <div class="field is-grouped">
                <div class="control">
                  <div class="select is-small">
                    <select v-model="selectedBranchName"
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
                  <span class="is-size-7">Version: {{ selectedCommit }}</span>
                </div>
                <div class="control">
                  <span class="is-size-7">Buckets: {{ selectedBucketNames.length }}</span>
                </div>
                <p class="control">
                  <button
                      class="button is-small"
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
                  v-for="commit in commitLines"
                  :key="commit.version"
                  :class="getVersionRowClass(commit.version)">
                <td class="is-size-7">
                  {{ commit.date }} {{ commit.version }}
                  <br/>
                  {{ commit.message }}
                  <br/>
                  <strong>{{ commit.author }}</strong> <i>{{ commit.email }}</i>
                </td>
              </tr>
              <tr v-if="!expanded && gitLog.length > init_max_rows">
                <td>
                  <a class="has-text-grey" @click="expanded = true">View More</a>
                </td>
              </tr>
              <tr v-if="expanded && gitLog.length > init_max_rows">
                <td>
                  <a class="has-text-grey" @click="expanded = false">View Less</a>
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
                  :class="getBucketRowClass(bucket.name)"
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
                    Publish to bucket is done with success!
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
import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import RequestsUtils from '@/assets/RequestsUtils.ts'
import {mdiBucket} from '@mdi/js'
import Vue from 'vue'
import {Branch, Commit} from '@/types'
import {AxiosResponse} from 'axios'

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
      commits: 0,
      branches: 0,
      selectedBranchName: null,
      // db/system info
      publishInfo: {buckets: [], branch_buckets: []},
      // reent commit or user clicks
      selectedCommit: null,
      // branch's buckets by default + plus user clicks
      selectedBucketNames: [],
      // buckets which are within an ongoing publish operation
      publishedBuckets: [],
      apiRoot: DatasetsUtils.ConfAPIRoot,
      apiVersion: DatasetsUtils.ConfAPIVersion,
      titles: DatasetsUtils.Titles,
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

    gitAPIPath(): string {
      return `${this.apiRoot}/${this.apiVersion}/configs/v/`
    },

    branchNames(): string[] {
      return _.sortBy(_.map(this.configs, 'id'))
    },

    selectedBranch(): Branch {
      if (!this.selectedBranchName) {
        return {} as Branch
      }

      const idx = _.findIndex(this.configs, (conf) => {
        return conf.id === this.selectedBranchName
      })

      if (idx > -1) {
        return this.configs[idx]
      }

      return this.configs[0]
    },

  },

  methods: {
    selectCommit(commit: Commit) {
      this.selectedCommit = commit.version
      this.publishMode = false
      console.log('publish info buckets', this.publishInfo.buckets)
    },

    getBucketRowClass(bucketName: string) {
      const classNames = []
      if (!this.publishMode) {
        if (this.bucketWithinList(bucketName)) {
          classNames.push('has-background-warning-light')
        }
        return classNames.join(' ')
      }
    },

    getVersionRowClass(version: string) {
      const classNames = []
      if (version === this.selectedCommit) {
        classNames.push('has-background-warning-light')
        classNames.push('marked')
      }
      return classNames.join(' ')
    },

    switchBranch() {
      this.publishMode = false
      this.setGitLog()
      this.setDefaultBuckets()
      console.log('publish info buckets', this.publishInfo.buckets)
    },

    setDefaultBuckets() {
      this.selectedBucketNames = []
      if (this.publishInfo.branch_buckets.length > 0) {
        const bucketList = _.find(this.publishInfo.branch_buckets, (list) => {
          return list.name === this.selectedBranchName
        })
        if (bucketList) {
          this.selectedBucketNames = _.cloneDeep(bucketList.buckets)
        }
      }

      console.log('publish info', this.publishInfo)
    },

    setGitLog() {
      if (this.selectedBranch) {
        this.gitLog = this.selectedBranch.logs
        this.selectedCommit = this.gitLog[0].version
      } else {
        this.gitLog = []
      }
    },

    loadPublishInfo() {
      RequestsUtils.sendRequest('GET', `db/system/k/publishinfo/`).then((response: AxiosResponse) => {
        this.publishInfo = response.data
        this.setDefaultBuckets()
      })
    },

    bucketWithinList(name: string) {
      return _.indexOf(this.selectedBucketNames, name) > -1
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
      RequestsUtils.sendRequest('GET', 'configs/').then((response: AxiosResponse<Branch[]>) => {
        const configs = response.data
        this.configs = configs
        // pick first branch name as selected
        this.selectedBranchName = this.branchNames[0]

        // counters
        this.commits = _.sum(_.map(_.map(configs, 'logs'), (logs) => {
          return _.size(logs)
        }))
        this.branches = _.size(configs)
        this.switchBranch()
      })
    },

    publish(event: Event) {
      this.publishMode = true
      let node = event.target as HTMLElement
      while (node.nodeName !== 'BUTTON') {
        node = node.parentNode as HTMLElement
      }
      node.classList.add('is-loading')
      this.publishedBuckets = _.cloneDeep(_.filter(this.publishInfo.buckets, (bucket) => {
        return _.indexOf(this.selectedBucketNames, bucket.name) > -1
      }))

      RequestsUtils.sendRequest('PUT',
          `/tools/publish/${this.selectedBranchName}/v/${this.selectedCommit}/`,
          this.buckets,
          null,
          `Published successfully!`,
          `Failed publishing!`,
      ).then((response: AxiosResponse) => {
        this.parsePublishResults(response.data, node)
      }).catch((error: Error) => {
        console.error(error)
        node.classList.remove('is-loading')
      })
    },

    parsePublishResults(data: any, node: HTMLElement) {
      node.classList.remove('is-loading')
      _.each(data.status, (response) => {
        console.log('response', response)
        console.log('published buckets', this.publishedBuckets)

        const index = _.findIndex(this.publishedBuckets, (entry) => {
          return entry.name === response.name
        })
        if (index > -1) {
          this.publishedBuckets[index].publishStatus = response
        }
      })

      const tempList = _.cloneDeep(this.publishedBuckets)
      this.publishedBuckets = []
      this.publishedBuckets = tempList
    },

  },

  created() {
    this.loadConfigs()
    this.loadPublishInfo()
  },

})
</script>

<style scoped>
tr[haserror='true'] {
  color: red;
}

.marked {
  font-weight: 400
}
</style>
