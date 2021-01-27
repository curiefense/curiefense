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
                    <select v-model="selectedBranchName" @change="switchBranch">
                      <option v-for="name in branchNames" :key="name" :value="name">{{ name }}</option>
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
              <tr
                  @click="selectCommit(commit)"
                  v-for="commit in commitLines"
                  :key="commit.version"
                  :class="vesionRowClass(commit.version)">
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
                  :class="bucketRowCLass(bucket.name)"
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

import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import RequestsUtils from '@/assets/RequestsUtils'
import {mdiBucket} from '@mdi/js'
import Vue from 'vue'

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
      publishinfo: {buckets: [], branch_buckets: []},

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
    buckets() {

      if (!this.publishMode) {
        return this.publishinfo.buckets
      }
      return this.publishedBuckets

    },

    commitLines() {
      if (this.expanded)
        return this.gitLog

      return this.gitLog.slice(0, this.init_max_rows)
    },

    gitAPIPath() {
      return `${this.apiRoot}/${this.apiVersion}/configs/v/`
    },

    branchNames() {
      return this.ld.sortBy(this.ld.map(this.configs, 'id'))
    },

    selectedBranch() {
      if (!this.selectedBranchName)
        return {}

      let idx = this.ld.findIndex(this.configs, (conf) => {
        return conf.id === this.selectedBranchName
      })

      if (idx > -1)
        return this.configs[idx]

      return this.configs[0]
    }

  },

  methods: {
    selectCommit(commit) {
      this.selectedCommit = commit.version
      this.publishMode = false
      console.log('this.publishinfo.buckets', this.publishinfo.buckets)
    },

    bucketRowCLass(bucketname) {
      let classNames = []
      if (!this.publishMode) {
        if (this.bucketWithinList(bucketname)) {
          classNames.push('has-background-warning-light')
        }
        return classNames.join(' ')
      }
    },

    vesionRowClass(version) {
      let classNames = []
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
      console.log('this.publishinfo.buckets', this.publishinfo.buckets)
    },

    setDefaultBuckets() {
      this.selectedBucketNames = []
      if (this.publishinfo.branch_buckets.length > 0) {
        let bucketList = this.ld.find(this.publishinfo.branch_buckets, (list) => {
          return list.name === this.selectedBranchName
        })
        if (bucketList)
          this.selectedBucketNames = this.ld.cloneDeep(bucketList.buckets)
      }

      console.log('this.publishinfo', this.publishinfo)
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
      RequestsUtils.sendRequest('GET', `db/system/k/publishinfo/`)
          .then((response) => {
            this.publishinfo = response.data
            this.setDefaultBuckets()
          })
    },

    bucketWithinList(name) {
      return this.ld.indexOf(this.selectedBucketNames, name) > -1
    },

    bucketNameClicked(name) {
      let idx = this.ld.indexOf(this.selectedBucketNames, name)
      if (idx > -1) {
        this.selectedBucketNames.splice(idx, 1)
      } else {
        this.selectedBucketNames.push(name)
      }
    },

    loadConfigs() {
      // store configs
      RequestsUtils.sendRequest('GET', 'configs/').then((response) => {
        let configs = response.data
        this.configs = configs
        // pick first branch name as selected
        this.selectedBranchName = this.branchNames[0]

        // counters
        this.commits = this.ld.sum(this.ld.map(this.ld.map(configs, 'logs'), (logs) => {
          return this.ld.size(logs)
        }))
        this.branches = this.ld.size(configs)
        this.switchBranch()

      })
    },

    publish(event) {
      this.publishMode = true

      let node = event.target

      while (node.nodeName !== 'BUTTON')
        node = node.parentNode

      node.classList.add('is-loading')

      this.publishedBuckets = this.ld.cloneDeep(this.ld.filter(this.publishinfo.buckets, (bucket) => {
        return this.ld.indexOf(this.selectedBucketNames, bucket.name) > -1
      }))

      RequestsUtils.sendRequest('PUT', `/tools/publish/${this.selectedBranchName}/v/${this.selectedCommit}/`, this.buckets, null, `Published successfully!`, `Failed publishing!`)
          .then((response) => {
            this.parsePublishResults(response.data, node)
          })
          .catch((error) => {
            console.error(error)
            node.classList.remove('is-loading')
          })
    },

    parsePublishResults(data, node) {
      node.classList.remove('is-loading')
      this.ld.each(data.status, (response) => {
        console.log('response', response)
        console.log('this.publishedBuckets', this.publishedBuckets)

        let idx = this.ld.findIndex(this.publishedBuckets, (entry) => {
          return entry.name === response.name
        })
        if (idx > -1)
          this.publishedBuckets[idx].publishStatus = response

      })

      let tempList = this.ld.cloneDeep(this.publishedBuckets)
      this.publishedBuckets = []
      this.publishedBuckets = tempList
    },

  },

  created() {
    this.loadConfigs()
    this.loadPublishInfo()
  }

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
