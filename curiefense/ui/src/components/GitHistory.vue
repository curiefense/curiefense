<template>
  <section>
    <div class="card">
      <div class="card-content">
        <div class="content">
          <p class="title is-6 is-expanded version-history-title">
            Version History
            <button class="button is-outlined is-text is-small is-loading" v-if="!gitLog || gitLog.length === 0">
              Loading
            </button>
          </p>
          <table class="table" v-if="gitLog && gitLog.length > 0">
            <thead>
            <tr>
              <th class="is-size-7">Date</th>
              <th class="is-size-7">Version</th>
              <th class="is-size-7">Parents</th>
              <th class="is-size-7">Message</th>
              <th class="is-size-7">Author</th>
              <th class="is-size-7">Email</th>
              <th class="is-size-7"></th>
            </tr>
            </thead>
            <tbody>
            <tr v-for="(commit, index) in commits" :key="commit.version"
                @mouseleave="mouseLeave()"
                @mouseover="mouseOver(index)">
              <td class="is-size-7 is-vcentered py-3">{{ commit.date }}</td>
              <td class="is-size-7 is-vcentered py-3" :title="commit.version">
                {{ commit.version.substr(0, 7) }}
              </td>
              <td class="is-size-7 is-vcentered py-3">
                <p v-for="parent in commit.parents" :key="parent" :title="parent">
                  {{ parent.substr(0, 7) }}
                </p>
              </td>
              <td class="is-size-7 is-vcentered py-3">{{ commit.message }}</td>
              <td class="is-size-7 is-vcentered py-3">{{ commit.author }}</td>
              <td class="is-size-7 is-vcentered py-3">{{ commit.email }}</td>
              <td class="is-size-7 is-vcentered restore-cell">
                <p class="control has-text-centered" v-if="commitOverIndex === index">
                  <a class="button is-small restore-button" @click="restoreVersion(commit)" title="Restore Version">
                    <span class="icon is-small">
                      <i class="fas fa-history"></i>
                    </span>
                  </a>
                </p>
              </td>
            </tr>
            <tr v-if="!expanded && gitLog.length > init_max_rows">
              <td colspan="6">
                <a class="has-text-grey" @click="expanded = true">View More</a>
              </td>
            </tr>
            <tr v-if="expanded && gitLog.length > init_max_rows">
              <td colspan="6">
                <a class="has-text-grey" @click="expanded = false">View Less</a>
              </td>
            </tr>
            </tbody>
          </table>
          <span class="is-family-monospace has-text-grey-lighter">{{ apiPath }}</span>
        </div>
      </div>
    </div>
  </section>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({
  name: 'GitHistory',

  props: {
    gitLog: Array,
    apiPath: String
  },

  components: {},

  data() {
    return {
      expanded: false,
      init_max_rows: 5,
      commitOverIndex: null,
    }
  },

  computed: {
    commits() {
      if (this.expanded)
        return this.gitLog

      return this.gitLog.slice(0, this.init_max_rows)
    }
  },

  methods: {
    restoreVersion(commit) {
      this.$emit('restore-version', commit)
    },

    mouseLeave() {
      this.commitOverIndex = null
    },

    mouseOver(index) {
      this.commitOverIndex = index
    },
  },

  mounted() {

  },

  created() {
  }
})
</script>
<style scoped>

table.is-borderless td, table.is-borderless th {
  border: 0;
}

table.inner-table td, table.inner-table th {
  border: 0;
  padding-left: 0;
  padding-right: 0;
}

.version-history-title {
  line-height: 30px;
}

.restore-cell {
  width: 50px
}

</style>
