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
                    <select v-model="selectedBranch" @change="switchBranch()">
                      <option v-for="name in branchNames" :key="name" :value="name">{{ name }}</option>
                    </select>
                  </div>
                </div>
                <div class="control">
                  <span class="icon is-small">
                    <i class="mdi mdi-dark mdi-source-branch"></i>
                  </span>
                  <span class="is-size-7  ">{{ branches }} branches</span>
                </div>
                <div class="control">
                  <span class="icon is-small">
                    <i class="mdi mdi-dark mdi-source-commit"></i>
                  </span>
                  <span class="is-size-7  ">{{ commits }} commits</span>
                </div>
              </div>
            </div>

            <div class="column">
              <div class="field is-grouped is-pulled-right">

                <p class="control">
                  <a class="button is-small"
                     :class="{'is-loading': isDownloadLoading}"
                     @click="downloadBranch($event)"
                     title="Download Branch x">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </a>
                </p>

                <p class="control">
                  <span class="field has-addons">
                    <span class="control">
                      <a class="button is-small"
                         @click="toggleBranchFork()">
                        <span class="icon is-small">
                          <i class="fas fa-code-branch"></i>
                        </span>
                      </a>
                    </span>
                    <span class="control is-expanded"
                          v-if="forkBranchInputOpen">
                      <input class="input is-small"
                             @input="validateInput($event, isSelectedBranchForkNameValid)"
                             placeholder="Forked Branch Name"
                             v-model="forkBranchName"
                             type="text">
                    </span>
                    <span class="control" v-if="forkBranchInputOpen">
                      <a class="button is-danger is-small" @click="toggleBranchFork">
                        <span class="icon is-small">
                          <i class="fas fa-times"></i>
                        </span>
                      </a>
                    </span>
                    <span class="control" v-if="forkBranchInputOpen">
                      <a class="button is-primary is-small"
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
                  <span class="field has-addons">
                    <span class="control">
                      <a class="button is-small has-text-danger"
                         @click="toggleBranchDelete()">
                        <span class="icon is-small">
                          <i class="fas fa-trash"></i>
                        </span>
                      </a>
                    </span>
                    <span class="control is-expanded"
                          v-if="deleteBranchInputOpen">
                      <input class="input is-small"
                             placeholder="Confirm Branch Name"
                             v-model="deleteBranchName"
                             type="text">
                    </span>
                    <span class="control" v-if="deleteBranchInputOpen">
                      <a class="button is-danger is-small" @click="toggleBranchDelete">
                        <span class="icon is-small">
                          <i class="fas fa-times"></i>
                        </span>
                      </a>
                    </span>
                    <span class="control" v-if="deleteBranchInputOpen">
                      <a class="button is-primary is-small"
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
        <GitHistory :gitLog.sync="gitLog"
                    :gitlogLoading.sync="gitLogLoading"
                    :apiPath.sync="gitAPIPath"
                    @restoreVersion="restoreGitVersion">
        </GitHistory>
      </div>
    </div>
  </div>
</template>
<script>

import axios from "axios";
import DatasetsUtils from "@/assets/DatasetsUtils";
import GitHistory from "@/components/GitHistory";
import Utils from "@/assets/Utils";
import * as bulmaToast from "bulma-toast";

export default {

  name: 'VersionControl',
  props: {},
  components: {
    GitHistory
  },

  data() {
    return {
      configs: [],
      selectedBranch: null,
      selectedBranchData: null,
      isDownloadLoading: false,

      gitLogLoading: false,
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

    gitAPIPath() {
      return `${this.apiRoot}/${this.apiVersion}/configs/${this.selectedBranch}/v/`
    },

    branchNames() {
      return this.ld.sortBy(this.ld.map(this.configs, "id"))
    },

    isSelectedBranchForkNameValid() {
      const newName = this.forkBranchName?.trim()
      const isBranchNameEmpty = newName === ''
      const isBranchNameContainsSpaces = newName.includes(' ')
      const isBranchNameDuplicate = this.branchNames.includes(newName)
      return !isBranchNameEmpty && !isBranchNameDuplicate && !isBranchNameContainsSpaces
    },

    isSelectedBranchDeleteNameValid() {
      const newName = this.deleteBranchName?.trim()
      return newName === this.selectedBranch
    },

  },

  methods: {

    resetGitLog() {
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
      const response = await this.ax()
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
      this.commits = this.ld.sum(this.ld.map(this.ld.map(configs, "logs"), (logs) => {
        return this.ld.size(logs)
      }))
      this.branches = this.ld.size(configs)
      console.log("config counters", this.branches, this.commits)
    },

    async loadSelectedBranchData() {
      this.isDownloadLoading = true
      this.selectedBranchData = (await this.ax(axios.get, `${this.selectedBranch}/`)).data
      this.isDownloadLoading = false
    },

    async switchBranch() {
      this.resetGitLog()
      this.forkBranchInputOpen = false
      this.deleteBranchInputOpen = false
      await this.loadSelectedBranchData()
      this.loadGitLog()
    },

    loadGitLog() {
      this.gitlogLoading = true;

      let self = this,
          config = this.selectedBranch,
          url_trail = `${config}/v/`;

      if (config) {
        this.ax(axios.get, url_trail).then((response) => {
          this.gitLog = response.data
          self.gitlogLoading = false;
        })
      }

    },

    async restoreGitVersion(gitVersion) {
      this.resetGitLog()
      const branch = this.selectedBranch
      const version_id = gitVersion.version
      const url_trail = `${branch}/v/${version_id}/`

      await this.ax(axios.put, `${url_trail}revert/`)
      this.loadGitLog()
    },

    async deleteBranch() {
      if (!this.isSelectedBranchDeleteNameValid) {
        return;
      }
      const isDeleted = await this.ax(axios.delete, `${this.selectedBranch}/`)
          .then(() => {
            this.successToast(`Branch [${this.selectedBranch}] deleted successfully!`)
            return true
          })
          .catch(() => {
            this.failToast(`Failed deleting branch [${this.selectedBranch}]!`)
            return false
          })
      if (isDeleted) {
        this.loadConfigs()
      }
      this.toggleBranchDelete()
    },

    async forkBranch() {
      if (!this.isSelectedBranchForkNameValid) {
        return;
      }
      let newBranchName = this.forkBranchName
      const isSaved = await this.ax(axios.post, `${this.selectedBranch}/clone/${newBranchName}/`,
          {
            "id": "string",
            "description": "string"
          })
          .then(() => {
            this.successToast(`Branch [${this.selectedBranch}] forked to [${this.forkBranchName}] successfully!`)
            return true
          })
          .catch(() => {
            this.failToast(`Failed forking branch [${this.selectedBranch}] to [${this.forkBranchName}]!`)
            return false
          })
      if (isSaved) {
        this.loadConfigs(newBranchName)
      }
      this.toggleBranchFork()
    },

    downloadBranch(event) {
      if (this.isDownloadLoading) {
        return
      }
      let element = event.target;
      while (element.nodeName !== "A")
        element = element.parentNode
      let dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(this.selectedBranchData));
      element.setAttribute("href", dataStr);
      element.setAttribute("download", this.selectedBranch + ".json");
    },

    ax(axios_method, urlTail, data) {
      if (!axios_method) {
        axios_method = axios.get;
      }
      if (!urlTail) {
        urlTail = "";
      }

      let apiroot = this.apiRoot,
          apiversion = this.apiVersion,
          apiurl = `${apiroot}/${apiversion}/configs/${urlTail}`;

      if (axios_method) {
        console.log("apiurl", apiurl)
        if (data) {
          return axios_method(apiurl, data)
        }

        return axios_method(apiurl)
      }
    },

    toast(message, type) {
      bulmaToast.toast(
          {
            message: message,
            type: `is-light ${type}`,
            position: 'bottom-left',
            closeOnClick: true,
            pauseOnHover: true,
            duration: 3000,
            opacity: 0.8,
          }
      )
    },

    successToast(message) {
      this.toast(message, 'is-success')
    },

    failToast(message) {
      this.toast(message, 'is-danger')
    },

  },

  mounted() {
  },

  async created() {
    await this.loadConfigs()
    await this.loadSelectedBranchData()
    this.loadGitLog()
  }

}

</script>
