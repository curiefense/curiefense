<template>
  <section class="version-history">
    <div class="mb-0" :class="{card: !notCollapsible}">
      <p class="title is-6 is-expanded version-history-title" v-if="notCollapsible">
        Version History
      </p>
      <header class="card-header" v-else>
        <p class="card-header-title collapsible-header">
          <a
            href="#collapsible-card"
            data-action="collapse"
            @click="toggleCollapsed"
          >
            <i :class="`arrow ${isAccordionCollapsed ? 'down' : 'up'}`" />
            {{ isAccordionCollapsed ? 'Show' : 'Hide' }} Version History
          </a>
        </p>
      </header>
      <div id="collapsible-card" class="is-collapsible">
        <span class="is-family-monospace">{{ apiPath }}</span>
        <div class="pt-4" :class="{'card-content pl-4': !notCollapsible}">
          <div class="content">
            <button class="button is-outlined is-text is-loading is-100" v-if="loading">
              Loading
            </button>
            <template v-else-if="viewData && viewData.length">
              <table class="table is-striped">
                <thead>
                  <tr>
                    <th
                      class="is-size-7 is-11"
                      :class="thSortableClass('date')"
                      @click="columnToggleSort('date')"
                    >
                      <span>Date</span>
                    </th>
                    <th class="is-size-7 is-11">
                      Time (UTC)
                    </th>
                    <th
                      class="is-size-7 is-10"
                      :class="thSortableClass('version')"
                      @click="columnToggleSort('version')"
                    >
                      Version
                    </th>
                    <th
                      class="is-size-7 is-40"
                      :class="thSortableClass('message')"
                      @click="columnToggleSort('message')"
                    >
                      Message
                    </th>
                    <th
                      class="is-size-7 is-15"
                      :class="thSortableClass('author')"
                      @click="columnToggleSort('author')"
                    >
                      Author
                    </th>
                    <th class="is-size-7 is-13"></th>
                  </tr>
                </thead>
                <tbody>
                  <template v-for="commit in commits">
                    <tr :key="commit.version">
                      <td class="is-size-7 is-vcentered py-3 is-11" :title="formatDate(commit.date)">
                        {{ formatDate(commit.date) }}
                      </td>
                      <td class="is-size-7 is-vcentered py-3 is-11" :title="formatTime(commit.date)">
                        {{ formatTime(commit.date) }}
                      </td>
                      <td class="is-size-7 is-vcentered py-3 is-10" :title="commit.version">
                        {{ commit.version.substr(0, 7) }}
                      </td>
                      <td class="is-size-7 is-vcentered py-3 is-40">{{ commit.message }}</td>
                      <td class="is-size-7 is-vcentered py-3 is-15">{{ commit.author }}</td>
                      <td class="is-size-7 is-vcentered actions-cell is-13 has-text-right">
                        <button class="button is-small restore-button"
                                @click="restoreVersion(commit)"
                                tabindex="1"
                                title="Restore version">
                          <span class="icon is-small">
                            <i class="fas fa-history"></i>
                          </span>
                        </button>
                        <button class="button is-small view-details ml-3"
                                @click="toggleCommitDetails(commit.version)"
                                tabindex="1"
                                title="View details">
                          <span class="icon is-small">
                            <i :class="`arrow ${isCommitCollapsed(commit.version) ? 'down' : 'up'}`" />
                          </span>
                        </button>
                      </td>
                    </tr>
                    <tr
                      v-if="!isCommitCollapsed(commit.version)"
                      class="commit-details"
                      :key="`${commit.version}-details`"
                    >
                      <td class="is-size-7 py-3 is-11 has-text-weight-bold">
                        <div>Date:</div>
                        <div>Time:</div>
                        <div>Version:</div>
                        <div>Parents:</div>
                      </td>
                      <td class="is-size-7 py-3 is-11">
                        <div>{{ formatDate(commit.date) }}</div>
                        <div>{{ formatTime(commit.date) }}</div>
                        <div>{{ commit.version.substr(0, 7) }}</div>
                        <div>
                          <p v-for="parent in commit.parents" :key="parent" :title="parent">
                            {{ parent.substr(0, 7) }}
                          </p>
                        </div>
                      </td>
                      <td class="is-size-7 py-3 is-10 has-text-weight-bold">
                        <div>Message:</div>
                        <div>Author:</div>
                        <div>API:</div>
                      </td>
                      <td colspan="3" class="is-size-7 py-3">
                        <div>{{ commit.message }}</div>
                        <div>{{ commit.author }}</div>
                        <div>{{ commit.email }}</div>
                      </td>
                    </tr>
                  </template>
                </tbody>
              </table>
              <div class="load-more card-content py-3 pl-3" v-if="viewData.length > maxRows">
                <a class="button is-small is-info is-light" @click="loadMore">Load More</a>
              </div>
            </template>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script lang="ts">
import Vue, {PropType} from 'vue'
import {Commit} from '@/types'
import DateTimeUtils from '@/assets/DateTimeUtils'
import BulmaCollapsible from '@creativebulma/bulma-collapsible'
import {isEqual, sortBy} from 'lodash'

const MAX_ROWS = 5

export default Vue.extend({

  name: 'GitHistory',

  props: {
    gitLog: Array as PropType<Commit[]>,
    apiPath: String,
    loading: Boolean,
    notCollapsible: Boolean,
  },

  data() {
    return {
      isAccordionCollapsed: true,
      maxRows: MAX_ROWS,
      viewCommitIndex: undefined,
      sort: {} as {
        field?: keyof Commit,
        desc?: boolean,
      },
      viewData: this.gitLog,
    }
  },

  watch: {
    gitLog: {
      handler( newVal ) {
        if ( !isEqual( newVal, this.viewData )) {
          this.viewData = newVal
          this.sort = {
            field: 'date',
            desc: true,
          }
          this.sortData()
        }
      },
    },
  },

  computed: {
    commits(): Commit[] {
      return this.viewData.slice(0, this.maxRows)
    },
    isSortable(): boolean {
      return this.viewData.length > 1
    },
  },

  methods: {
    restoreVersion(commit: Commit) {
      this.$emit('restore-version', commit)
    },

    formatDate(date: string) {
      return DateTimeUtils.isoToNowDateCuriefenseFormat(date)
    },

    formatTime(date: string) {
      return DateTimeUtils.isoToNowTimeCuriefenseFormat(date)
    },

    toggleCollapsed() {
      this.isAccordionCollapsed = !this.isAccordionCollapsed
      this.viewCommitIndex = undefined
    },

    toggleCommitDetails(commitIndex: number) {
      this.viewCommitIndex = this.isCommitCollapsed(commitIndex) ? commitIndex : undefined
    },

    isCommitCollapsed(commitIndex: number) {
      return this.viewCommitIndex !== commitIndex
    },

    columnToggleSort(colName: keyof Commit) {
      if ( !this.isSortable ) {
        this.sort = {}
        return
      }
      if (colName === this.sort.field) {
        this.sort.desc = !this.sort.desc
      } else {
        this.sort = {
          field: colName,
          desc: true,
        }
      }
      this.sortData();
    },

    sortData() {
      const sortedData = sortBy( this.viewData, this.sort.field )
      this.viewData = this.sort.desc ? sortedData.reverse() : sortedData
    },

    thSortableClass( fieldName: keyof Commit ) {
      const result = this.isSortable ? ['sortable'] : []
      if ( this.sort.field === fieldName ) {
        result.push( this.sort.desc ? 'desc' : 'asc' )
      }
      return result.join( ' ' )
    },

    loadMore() {
      this.maxRows += MAX_ROWS
    },
  },

  mounted() {
    if ( !this.notCollapsible ) {
      BulmaCollapsible.attach('.is-collapsible')
    }
  },
})
</script>
<style scoped lang="scss">
@import '~@creativebulma/bulma-collapsible';

.is-collapsible .is-family-monospace {
  align-items: center;
  background-color: #f5f5f5;
  color: #737373;
  display: flex;
  height: 32px;
  padding-left: 1rem;
}

.version-history .arrow {
  border: solid #111;
  border-width: 0 2px 2px 0;
  display: inline-block;
  margin-right: 10px;
  padding: 4px;

  &.up {
    margin-bottom: -2px;
    transform: rotate(-135deg);
  }

  &.down {
    margin-bottom: 2px;
    transform: rotate(45deg);
  }
}

.view-details .arrow {
  margin: 4px 0 0;

  &.down {
    border-color: #014ac6;
    margin-top: 0;
  }
}

.version-history-title {
  line-height: 30px;
}

.actions-cell {
  width: 50px;

  .button {
    color: #014ac6;
  }
}

.content .card-header-title.collapsible-header {
  margin-bottom: 0;

  a {
    color: #000;
    font-family: 'Red Hat Display', sans-serif;
    font-size: 14px;
    font-style: normal;
    font-weight: 400;
    width: 100%;
  }
}

.is-100 .columns > .column {
  flex: 1 1 auto;
}

.commit-details td div {
  margin-bottom: 5px;
}

.is-collapsible .sortable {
  cursor: pointer;
  position: relative;

  span {
    float: left;
  }

  &::before,
  &::after {
    border: solid #555;
    border-width: 0 1px 1px 0;
    content: '';
    display: inline-block;
    height: 7px;
    position: absolute;
    right: 5px;
    top: 50%;
    width: 7px;
  }

  &::before {
    margin-top: -5px;
    transform: rotate(-135deg);
  }

  &::after {
    margin-top: -1px;
    transform: rotate(45deg);
  }

  &.desc::before {
    display: none;
  }

  &.desc::after {
    margin-top: -5px;
  }

  &.asc::before {
    margin-top: -2px;
  }

  &.asc::after {
    display: none;
  }
}

.load-more {
  background-color: #edeef0;
}

</style>
