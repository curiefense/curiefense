<template>
  <div class="card">
    <div class="card-content">
      <div class="media">
        <div class="media-content">
          <div class="columns">
            <div class="column">
              <div class="field is-grouped">
                <div class="control" v-if="branchNames.length">
                  <div class="select is-small">
                    <select v-model="selectedBranch"
                            title="Switch Branch"
                            @change="switchBranch()"
                            class="branch-selection">
                      <option v-for="name in branchNames"
                              :key="name"
                              :value="name">
                        {{ name }}
                      </option>
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
                  <span class="is-size-7 git-branches">
                    {{ branches }} branch<span v-if="branches !== 1">es</span>
                  </span>
                </div>
                <div class="control">
                  <span class="icon is-small is-vcentered">
                    <svg :width="24"
                         :height="24"
                         :viewBox="'0 0 24 24'">
                      <path :d="mdiSourceCommitPath"/>
                    </svg>
                  </span>
                  <span class="is-size-7 git-commits">
                    {{ commits }} commit<span v-if="commits !== 1">s</span>
                  </span>
                </div>
              </div>
            </div>

            <div class="column" v-if="branches">
              <div class="field is-grouped is-pulled-right">

                <p class="control">
                  <span class="field has-addons">
                    <span class="control">
                      <button class="button is-small fork-branch-toggle"
                              @click="toggleBranchFork()">
                        <span class="icon is-small">
                          <i class="fas fa-code-branch"></i>
                        </span>
                      </button>
                    </span>
                    <span class="control is-expanded"
                          v-if="forkBranchInputOpen">
                      <input class="input is-small fork-branch-input"
                             title="Forked branch name"
                             @input="validateInput($event, isSelectedBranchForkNameValid)"
                             placeholder="Forked Branch Name"
                             v-model="forkBranchName"
                             type="text">
                    </span>
                    <span class="control" v-if="forkBranchInputOpen">
                      <button class="button is-danger is-small fork-branch-cancel"
                              @click="toggleBranchFork">
                        <span class="icon is-small">
                          <i class="fas fa-times"></i>
                        </span>
                      </button>
                    </span>
                    <span class="control" v-if="forkBranchInputOpen">
                      <button class="button is-primary is-small fork-branch-confirm"
                              @click="forkBranch"
                              :disabled="!isSelectedBranchForkNameValid">
                        <span class="icon is-small">
                          <i class="fas fa-check"></i>
                        </span>
                      </button>
                    </span>
                  </span>
                </p>

                <p class="control">
                  <button class="button is-small download-branch-button"
                          :class="{'is-loading': isDownloadLoading}"
                          @click="downloadBranch()"
                          title="Download branch">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </button>
                </p>

                <p class="control">
                  <span class="field has-addons">
                    <span class="control">
                      <button class="button is-small has-text-danger delete-branch-toggle"
                              @click="toggleBranchDelete()">
                        <span class="icon is-small">
                          <i class="fas fa-trash"></i>
                        </span>
                      </button>
                    </span>
                    <span class="control is-expanded"
                          v-if="deleteBranchInputOpen">
                      <input class="input is-small delete-branch-input"
                             title="Confirm branch name"
                             placeholder="Confirm Branch Name"
                             v-model="deleteBranchName"
                             type="text">
                    </span>
                    <span class="control" v-if="deleteBranchInputOpen">
                      <button class="button is-danger is-small delete-branch-cancel"
                              @click="toggleBranchDelete">
                        <span class="icon is-small">
                          <i class="fas fa-times"></i>
                        </span>
                      </button>
                    </span>
                    <span class="control" v-if="deleteBranchInputOpen">
                      <button class="button is-primary is-small delete-branch-confirm"
                              @click="deleteBranch"
                              :disabled="!isSelectedBranchDeleteNameValid">
                        <span class="icon is-small">
                          <i class="fas fa-check"></i>
                        </span>
                      </button>
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
        <git-history :gitLog="gitLog"
                     :apiPath="gitAPIPath"
                     :loading="loadingGitlog"
                     @restore-version="restoreGitVersion">
        </git-history>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import _ from 'lodash'
import RequestsUtils from '@/assets/RequestsUtils.ts'
import Utils from '@/assets/Utils.ts'
import GitHistory from '@/components/GitHistory.vue'
import {mdiSourceBranch, mdiSourceCommit} from '@mdi/js'
import Vue from 'vue'
import {AxiosResponse} from 'axios'
import {Commit} from '@/types'

export default Vue.extend({

  name: 'VersionControl',
  props: {},
  components: {
    GitHistory,
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

      apiRoot: RequestsUtils.confAPIRoot,
      apiVersion: RequestsUtils.confAPIVersion,

      loadingGitlog: false,
    }
  },

  computed: {

    gitAPIPath(): string {
      return `${this.apiRoot}/${this.apiVersion}/configs/${this.selectedBranch}/v/`
    },

    branchNames(): string[] {
      return _.sortBy(_.map(this.configs, 'id'))
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

    resetGitLog(): void {
      this.gitLog = []
    },

    validateInput(event: Event, validator: Function | boolean) {
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

    async loadConfigs(activeBranch?: string) {
      // store configs
      const response = await RequestsUtils.sendRequest({methodName: 'GET', url: 'configs/'})
      const configs = response?.data
      this.configs = configs
      if (!activeBranch) {
        // pick first branch name as selected if not given active branch
        this.selectedBranch = this.branchNames[0]
      } else {
        this.selectedBranch = this.branchNames.find((branch) => {
          return branch === activeBranch
        })
      }
      // counters
      this.commits = _.sum(_.map(_.map(configs, 'logs'), (logs) => {
        return _.size(logs)
      }))
      this.branches = _.size(configs)
      console.log('config counters', this.branches, this.commits)
    },

    async loadSelectedBranchData() {
      this.isDownloadLoading = true
      this.selectedBranchData = (await RequestsUtils.sendRequest({
        methodName: 'GET',
        url: `configs/${this.selectedBranch}/`,
      }))?.data
      this.isDownloadLoading = false
    },

    async switchBranch() {
      this.resetGitLog()
      Utils.toast(`Switched to branch "${this.selectedBranch}".`, 'is-info')
      this.forkBranchInputOpen = false
      this.deleteBranchInputOpen = false
      await this.loadSelectedBranchData()
      await this.loadGitLog()
    },

    async loadGitLog() {
      this.loadingGitlog = true
      const config = this.selectedBranch
      const url = `configs/${config}/v/`
      return RequestsUtils.sendRequest({
        methodName: 'GET',
        url,
        failureMessage: 'Failed while attempting to load Git log',
      }).then((response: AxiosResponse<Commit[]>) => {
        this.gitLog = response?.data
        this.loadingGitlog = false
        return response
      })
    },

    async restoreGitVersion(gitVersion: Commit) {
      this.resetGitLog()
      const branch = this.selectedBranch
      const versionId = gitVersion.version
      const urlTrail = `configs/${branch}/v/${versionId}/`

      await RequestsUtils.sendRequest({
        methodName: 'PUT',
        url: `${urlTrail}revert/`,
        successMessage: `Branch "${branch}" was restored to version "${versionId}".`,
        failureMessage: `Failed while attempting to restore branch "${branch}" to version "${versionId}".`,
      })
      await this.loadGitLog()
    },

    deleteBranch() {
      RequestsUtils.sendRequest({
        methodName: 'DELETE',
        url: `configs/${this.selectedBranch}/`,
        successMessage: `Branch ${this.selectedBranch} was deleted.`,
        failureMessage: `Failed while attempting to delete branch "${this.selectedBranch}".`,
      }).then(() => {
        this.loadConfigs()
        this.toggleBranchDelete()
      })
    },

    forkBranch() {
      const newBranchName = this.forkBranchName
      RequestsUtils.sendRequest({
        methodName: 'POST',
        url: `configs/${this.selectedBranch}/clone/${newBranchName}/`,
        data: {
          'id': 'string',
          'description': 'string',
        },
        successMessage: `Branch "${this.selectedBranch}" was forked to "${this.forkBranchName}".`,
        failureMessage: `Failed while attempting to fork branch "${this.selectedBranch}" to "${this.forkBranchName}".`,
      }).then(() => {
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
  },

})
</script>
<style scoped lang="scss">
</style>
