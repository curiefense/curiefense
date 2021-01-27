<template>
  <div class="card">
    <div class="card-content">
      <div class="media">
        <div class="media-content">
          <div class="columns">
            <div class="column">
              <div class="field is-grouped">
                <div class="control">
                  <div class="select is-small">
                    <select v-model="selectedBranch" @change="switchBranch()" class="branch-selection">
                      <option v-for="name in branchNames" :key="name" :value="name">{{ name }}</option>
                    </select>
                  </div>
                </div>
                <div class="control">
                  <span class="icon is-small is-vcentered">
                    <svg :width="24"
                         :height="24"
                         :viewBox="'0 0 24 24'">
                      <path :d="mdiSourceBranchPath"/>
                    </svg>
                  </span>
                  <span class="is-size-7 git-branches">{{ branches }} branches</span>
                </div>
                <div class="control">
                  <span class="icon is-small is-vcentered">
                    <svg :width="24"
                         :height="24"
                         :viewBox="'0 0 24 24'">
                      <path :d="mdiSourceCommitPath"/>
                    </svg>
                  </span>
                  <span class="is-size-7 git-commits">{{ commits }} commits</span>
                </div>
              </div>
            </div>

            <div class="column">
              <div class="field is-grouped is-pulled-right">

                <p class="control">
                  <span class="field has-addons">
                    <span class="control">
                      <a class="button is-small fork-branch-toggle"
                         @click="toggleBranchFork()">
                        <span class="icon is-small">
                          <i class="fas fa-code-branch"></i>
                        </span>
                      </a>
                    </span>
                    <span class="control is-expanded"
                          v-if="forkBranchInputOpen">
                      <input class="input is-small fork-branch-input"
                             @input="validateInput($event, isSelectedBranchForkNameValid)"
                             placeholder="Forked Branch Name"
                             v-model="forkBranchName"
                             type="text">
                    </span>
                    <span class="control" v-if="forkBranchInputOpen">
                      <a class="button is-danger is-small fork-branch-cancel" @click="toggleBranchFork">
                        <span class="icon is-small">
                          <i class="fas fa-times"></i>
                        </span>
                      </a>
                    </span>
                    <span class="control" v-if="forkBranchInputOpen">
                      <a class="button is-primary is-small fork-branch-confirm"
                         @click="forkBranch"
                         :disabled="!isSelectedBranchForkNameValid">
                        <span class="icon is-small">
                          <i class="fas fa-check"></i>
                        </span>
                      </a>
                    </span>
                  </span>
                </p>

                <p class="control">
                  <button class="button is-small download-branch-button"
                          :class="{'is-loading': isDownloadLoading}"
                          @click="downloadBranch($event)"
                          title="Download Branch">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <span class="field has-addons">
                    <span class="control">
                      <a class="button is-small has-text-danger delete-branch-toggle"
                         @click="toggleBranchDelete()">
                        <span class="icon is-small">
                          <i class="fas fa-trash"></i>
                        </span>
                      </a>
                    </span>
                    <span class="control is-expanded"
                          v-if="deleteBranchInputOpen">
                      <input class="input is-small delete-branch-input"
                             placeholder="Confirm Branch Name"
                             v-model="deleteBranchName"
                             type="text">
                    </span>
                    <span class="control" v-if="deleteBranchInputOpen">
                      <a class="button is-danger is-small delete-branch-cancel"
                         @click="toggleBranchDelete">
                        <span class="icon is-small">
                          <i class="fas fa-times"></i>
                        </span>
                      </a>
                    </span>
                    <span class="control" v-if="deleteBranchInputOpen">
                      <a class="button is-primary is-small delete-branch-confirm"
                         @click="deleteBranch"
                         :disabled="!isSelectedBranchDeleteNameValid">
                        <span class="icon is-small">
                          <i class="fas fa-check"></i>
                        </span>
                      </a>
                    </span>
                  </span>
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="content">
        <hr/>
        <git-history :gitLog.sync="gitLog"
                     :apiPath.sync="gitAPIPath"
                     @restore-version="restoreGitVersion">
        </git-history>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import DatasetsUtils from '@/assets/DatasetsUtils'
import GitHistory from '@/components/GitHistory'
import Utils from '@/assets/Utils'
import RequestsUtils from '@/assets/RequestsUtils'
import {mdiSourceBranch, mdiSourceCommit} from '@mdi/js'
import Vue from 'vue'

export default Vue.extend({

  name: 'VersionControl',
  props: {},
  components: {
    GitHistory
  },

  data() {
    return {
      mdiSourceBranchPath: mdiSourceBranch,
      mdiSourceCommitPath: mdiSourceCommit,

      configs: [],
      selectedBranch: null,
      selectedBranchData: null,
      isDownloadLoading: false,

      gitLog: [],
      commits: 0,
      branches: 0,

      forkBranchName: '',
      forkBranchInputOpen: false,
      deleteBranchName: '',
      deleteBranchInputOpen: false,

      apiRoot: DatasetsUtils.ConfAPIRoot,
      apiVersion: DatasetsUtils.ConfAPIVersion,
    }
  },

  computed: {

    gitAPIPath(): string {
      return `${this.apiRoot}/${this.apiVersion}/configs/${this.selectedBranch}/v/`
    },

    branchNames(): string[] {
      return this.ld.sortBy(this.ld.map(this.configs, 'id'))
    },

    isSelectedBranchForkNameValid(): boolean {
      const newName = this.forkBranchName?.trim()
      const isBranchNameEmpty = newName === ''
      const isBranchNameContainsSpaces = newName.includes(' ')
      const isBranchNameDuplicate = this.branchNames.includes(newName)
      return !isBranchNameEmpty && !isBranchNameDuplicate && !isBranchNameContainsSpaces
    },

    isSelectedBranchDeleteNameValid(): boolean {
      const newName = this.deleteBranchName?.trim()
      return newName === this.selectedBranch
    },

  },

  methods: {

    resetGitLog(): string {
      this.gitLog = []
    },

    validateInput(event, validator) {
      Utils.validateInput(event, validator)
    },

    toggleBranchFork() {
      this.forkBranchInputOpen = !this.forkBranchInputOpen
      if (!this.forkBranchInputOpen) {
        this.forkBranchName = ''
      }
    },

    toggleBranchDelete() {
      this.deleteBranchInputOpen = !this.deleteBranchInputOpen
      if (!this.deleteBranchInputOpen) {
        this.deleteBranchName = ''
      }
    },

    async loadConfigs(active_branch) {
      // store configs
      const response = await RequestsUtils.sendRequest('GET', 'configs/')
      let configs = response.data
      this.configs = configs
      if (!active_branch) {
        // pick first branch name as selected if not given active branch
        this.selectedBranch = this.branchNames[0]
      } else {
        this.selectedBranch = this.branchNames.find((branch) => {
          return branch === active_branch
        })
      }
      // counters
      this.commits = this.ld.sum(this.ld.map(this.ld.map(configs, 'logs'), (logs) => {
        return this.ld.size(logs)
      }))
      this.branches = this.ld.size(configs)
      console.log('config counters', this.branches, this.commits)
    },

    async loadSelectedBranchData() {
      this.isDownloadLoading = true
      this.selectedBranchData = (await RequestsUtils.sendRequest('GET', `configs/${this.selectedBranch}/`)).data
      this.isDownloadLoading = false
    },

    async switchBranch() {
      this.resetGitLog()
      this.forkBranchInputOpen = false
      this.deleteBranchInputOpen = false
      await this.loadSelectedBranchData()
      await this.loadGitLog()
    },

    async loadGitLog() {
      const config = this.selectedBranch
      const url_trail = `configs/${config}/v/`
      return RequestsUtils.sendRequest('GET', url_trail).then((response) => {
        this.gitLog = response.data
        return response
      })
    },

    async restoreGitVersion(gitVersion) {
      this.resetGitLog()
      const branch = this.selectedBranch
      const version_id = gitVersion.version
      const url_trail = `configs/${branch}/v/${version_id}/`

      await RequestsUtils.sendRequest('PUT', `${url_trail}revert/`, null, null, `Branch [${branch}] restored to version [${version_id}]!`, `Failed restoring branch [${branch}] to version [${version_id}]!`)
      this.loadGitLog()
    },

    deleteBranch() {
      if (!this.isSelectedBranchDeleteNameValid) {
        return
      }
      RequestsUtils.sendRequest('DELETE', `configs/${this.selectedBranch}/`, null, null,
          `Branch [${this.selectedBranch}] deleted successfully!`,
          `Failed deleting branch [${this.selectedBranch}]!`)
          .then(() => {
            this.loadConfigs()
            this.toggleBranchDelete()
          })
    },

    forkBranch() {
      if (!this.isSelectedBranchForkNameValid) {
        return
      }
      let newBranchName = this.forkBranchName
      RequestsUtils.sendRequest('POST', `configs/${this.selectedBranch}/clone/${newBranchName}/`,
          {
            'id': 'string',
            'description': 'string'
          }, null,
          `Branch [${this.selectedBranch}] forked to [${this.forkBranchName}] successfully!`,
          `Failed forking branch [${this.selectedBranch}] to [${this.forkBranchName}]!`)
          .then(() => {
            this.loadConfigs(newBranchName)
            this.toggleBranchFork()
          })
    },

    downloadBranch() {
      if (!this.isDownloadLoading) {
        Utils.downloadFile(this.selectedBranch, 'json', this.selectedBranchData)
      }
    },

  },

  mounted() {
  },

  async created() {
    await this.loadConfigs()
    await this.loadSelectedBranchData()
    this.loadGitLog()
  }

})
</script>
