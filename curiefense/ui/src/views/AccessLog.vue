<template>
  <div class="card">
    <div class="card-content">
      <div class="media">
        <div class="media-content">
          <div class="columns">
            <div class="column">
              Access Logs
            </div>
            <div class="column">
              <div class="field is-grouped is-pulled-right">
                <p class="control">
                  <a class="button is-small" @click="downloadDoc" title="Download log">
                    <span class="icon is-small">
                      <i class="fas fa-download"></i>
                    </span>
                  </a>
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="content">
        <hr/>
        <div class="field">
          <div class="field is-grouped">
            <div class="control">
              <div class="select is-small">
                <select v-model="logFilterInterval"
                        title="Filter interval">
                  <option value="30 minutes">Last 30 minutes</option>
                  <option value="60 minutes">Last hour</option>
                  <option value="3 hours">Last 3 hours</option>
                  <option value="24 hours">Last 24 hours</option>
                  <option value="3 days">Last 3 days</option>
                  <option value="7 days">Last week</option>
                  <option value="30 days">Last month</option>
                </select>
              </div>
            </div>
            <p class="control has-icons-right is-small is-expanded is-fullwidth">
              <input class="input is-small"
                     title="Filter input"
                     placeholder="123.45.87.219, Verizon, POST, /login"
                     v-model="logFilterInput">
            </p>
            <p class="control">
              <button class="button is-small" @click="buildQuery">
                <span class="icon is-small">
                  <i class="fas fa-search"></i>
                </span>
              </button>
            </p>
          </div>
        </div>
        <div class="card">
          <div class="card-content">
            <div class="content">
              <div class="field">
                <label class="label" v-if="loading">Loading data...</label>
                <label class="label" v-else>{{ rows.length }} rows</label>
                <div class="control">
                  <table class="table is-narrow is-fullwidth">
                    <tbody v-for="(row, idx) in rows" :key="idx" class="data-entry-wrapper">
                    <tr @click="rowEntryIndex = (rowEntryIndex === idx ? -1 : idx)"
                        class="has-row-clickable"
                        :class="[rowEntryIndex === idx ? ' has-background-white-bis' : '']">
                      <td class="is-size-7 has-text-centered" :class="statuscode_class(row.responsecode)">
                        {{ row.responsecode }}
                      </td>
                      <td class="is-size-7" :title="row.curiefense.attrs.ip + ':' + row.downstreamremoteaddressport">
                        {{ row.curiefense.attrs.ip }}
                      </td>
                      <td class="is-size-7"><span
                          class="has-text-weight-medium is-family-secondary">{{ row.requestmethod }}</span> <span
                          :title="fulluri(row)">{{ suburi(row) }}</span></td>

                      <td class="is-size-7 width-120px">&#8593;{{ row.requestheadersbytes + row.requestbodybytes }}
                        &#8595;{{ row.responseheadersbytes + row.responsebodybytes }}
                      </td>


                      <td v-if="row.upstreamremoteaddress" class="is-size-7"
                          :title="row.upstreamremoteaddress + ':' + row.upstreamremoteaddressPort">
                        {{ row.upstreamremoteaddress }}
                      </td>
                      <td v-else class="is-size-7">terminated</td>
                      <td class="is-size-7 width-150px">{{ isodate(row.starttime) }}</td>
                      <td class="is-size-7" :rowspan="rowEntryIndex === idx ? '2' : '1'">
                        <a class="has-text-grey" title="more details">
                          {{ rowEntryIndex === idx ? 'close' : 'expand' }}
                        </a>
                      </td>

                    </tr>
                    <tr
                        v-if="rowEntryIndex === idx"
                        class="expanded borderless has-background-white-bis">
                      <td colspan="12" class="px-3 py-3">
                        <!--  TOP TILE -->
                        <div class="tile is-ancestor">
                          <div class="tile is-parent">
                            <article class="tile is-child box">
                              <div class="content overflow-anywhere">
                                <p class="is-size-7" title="URL">
                                  <span class="has-text-grey ">{{ row.curiefense.attrs.authority }}</span>
                                  <span class="has-text-grey-dark ">{{ row.curiefense.attrs.uri }}</span>
                                </p>
                                <p v-if="row.useragent" class="is-size-7" title="User-agent">
                                  <span class="has-text-grey-dark ">{{ row.useragent }}</span>
                                </p>
                                <p v-if="row.referer" class="is-size-7" title="Referer">
                                  <span class="has-text-grey-dark">{{ row.referer }}</span>
                                </p>
                              </div>
                            </article>
                          </div>
                        </div>

                        <!--  BLOCK REASON TILE -->

                        <div class="tile is-ancestor " v-if="row.curiefense.attrs.blocked">
                          <div class="tile is-parent ">
                            <article class="tile is-child box has-background-danger-light">
                              <div class="content overflow-anywhere">
                                <label class="has-text-weight-bold has-text-danger-dark">
                                  Risk details
                                  <span class="has-text-family-uppercase"
                                        v-if="row.curiefense.attrs.block_reason.initiator">
                                    ({{ row.curiefense.attrs.block_reason.initiator }})
                                  </span>
                                </label>
                                <br/>
                                <pre class="is-size-7 is-family-code has-background-danger-light
                                            overflow-anywhere is-fullwidth">
                                  {{ break_reason(row.curiefense.attrs.block_reason) }}
                                </pre>
                              </div>
                            </article>
                          </div>
                        </div>

                        <!--  H/C/A TILE -->
                        <div class="tile is-ancestor">
                          <div class="tile is-vertical is-9">
                            <div class="tile">
                              <div class="tile is-parent">
                                <article class="tile is-child box">
                                  <div class="content overflow-anywhere">
                                    <section v-if="!isempty(row.curiefense.headers)">
                                      <label class="label">Headers<span
                                          class="is-pulled-right">{{ row.requestheadersbytes }} bytes</span></label>
                                      <table class="table is-narrow borderless is-fullwidth">
                                        <tr v-for="(value, name) in row.curiefense.headers" :key="name">
                                          <td class="has-text-weight-medium is-size-7 width-200px">{{ name }}</td>
                                          <td class="is-size-7">{{ value }}</td>
                                        </tr>
                                      </table>
                                    </section>
                                    <section v-if="!isempty(row.curiefense.cookies)">
                                      <hr/>
                                      <label class="label">Cookies</label>
                                      <table class="table is-narrow borderless">
                                        <tr v-for="(value, name) in row.curiefense.cookies" :key="name">
                                          <td class="has-text-weight-medium is-size-7 width-200px">{{ name }}</td>
                                          <td class="is-size-7">{{ value }}</td>
                                        </tr>
                                      </table>
                                    </section>
                                    <section v-if="!isempty(row.curiefense.args)">
                                      <hr/>
                                      <label class="label">Arguments<span
                                          class="is-pulled-right">{{ row.requestbodybytes }} bytes</span></label>
                                      <table class="table is-narrow borderless">
                                        <tr v-for="(value, name) in row.curiefense.args" :key="name">
                                          <td class="has-text-weight-medium is-size-7 width-200px">{{ name }}</td>
                                          <td class="is-size-7">{{ value }}</td>
                                        </tr>
                                      </table>
                                    </section>
                                    <hr/>
                                    <section>
                                      <label class="label">Network Metrics</label>
                                      <div class="content">
                                        <div class="columns">
                                          <div class="column">
                                            <label class="has-text-weight-semibold">Upstream</label>
                                            <section class="is-size-7">
                                              <span>TTFB: {{ subNum10(row.timetofirstupstreamtxbyte) }}</span></section>
                                            <section class="is-size-7">
                                              <span>TTFB RX: {{ subNum10(row.timetolastupstreamtxbyte) }}</span>
                                            </section>
                                            <section class="is-size-7">
                                              <span>TTLB RX: {{ subNum10(row.timetofirstupstreamrxbyte) }}</span>
                                            </section>
                                            <section class="is-size-7">
                                              <span>TTLB TX: {{ subNum10(row.timetolastupstreamrxbyte) }}</span>
                                            </section>
                                            <section class="is-size-7" v-if="row.upstreamremoteaddress">
                                              <span>
                                                Remote: {{ row.upstreamremoteaddress }}:{{
                                                  row.upstreamremoteaddressport
                                                }}
                                              </span>
                                            </section>
                                            <section class="is-size-7" v-if="row.upstreamlocaladdress">
                                              <span>
                                                Local: {{ row.upstreamlocaladdress }}:{{ row.upstreamlocaladdressport }}
                                              </span>
                                            </section>
                                            <section class="is-size-7" v-if="row.upstreamcluster">
                                              <span>Cluster: {{ row.upstreamcluster }}</span></section>
                                            <section class="is-size-7" v-if="row.upstreamtransportfailurereason">
                                              <span>
                                                Failure: {{ row.upstreamtransportfailurereason }}
                                              </span>
                                            </section>
                                            <section class="is-size-7" v-if="row.routename">
                                              <span>Route Name: {{ row.routename }}</span></section>
                                          </div>
                                          <div class="column">
                                            <label class="has-text-weight-semibold">Downtream</label>
                                            <section class="is-size-7">
                                              <span>TTFB TX: {{ subNum10(row.timetofirstdownstreamtxbyte) }}</span>
                                            </section>
                                            <section class="is-size-7">
                                              <span>TTLB TX: {{ subNum10(row.timetolastdownstreamtxbyte) }}</span>
                                            </section>
                                            <section class="is-size-7">
                                              <span>Remote: {{
                                                  row.downstreamremoteaddress
                                                }}:{{ row.downstreamremoteaddressport }}</span>
                                            </section>
                                            <section class="is-size-7">
                                              <span>Local: {{
                                                  row.downstreamlocaladdress
                                                }}:{{ row.downstreamlocaladdressport }}</span>
                                            </section>
                                            <section class="is-size-7">
                                              <span>Direct: {{
                                                  row.downstreamdirectremoteaddress
                                                }}:{{ row.downstreamdirectremoteaddressport }}</span>
                                            </section>
                                            <section class="is-size-7">
                                              <span>Status code: {{ row.responsecodedetails }}</span></section>
                                          </div>
                                          <div class="column">
                                            <label class="has-text-weight-semibold">TLS Info</label>
                                            <section class="is-size-7"><span>Version: {{ row.tlsversion }}</span>
                                            </section>
                                            <section class="is-size-7">
                                              <span>Cipher Suite: {{ row.tlsciphersuite }}</span></section>
                                            <section class="is-size-7">
                                              <span>Cert: {{ row.localcertificateproperties }}</span></section>
                                            <section class="is-size-7">
                                              <span>SNI Hostname: {{ row.tlssnihostname }}</span></section>
                                            <section class="is-size-7"><span
                                                :title="row.tlssessionid">Session Id: {{
                                                row.tlssessionid.substr(0, 12)
                                              }}</span>
                                            </section>
                                          </div>
                                        </div>
                                      </div>
                                    </section>
                                  </div>
                                </article>
                              </div>
                            </div>
                          </div>
                          <!--  TAGS TILE -->
                          <div class="tile is-parent">
                            <article class="tile is-child box">
                              <div class="content">
                                <label class="label">Tags</label>
                                <section class="tag2 has-background-white-bis"
                                         v-for="tag in tags(row.curiefense.attrs.tags)" :key="tag">
                                    <span>
                                      <span class="is-size-7">{{ tag }}</span>
                                    </span>
                                </section>
                                <hr/>
                                <label class="label">More info</label>
                                <section class="is-size-7 tag2 has-background-white-bis-3">AS
                                  {{ row.curiefense.attrs.asn }}
                                </section>
                                <section class="is-size-7 tag2 has-background-white-bis-3">
                                  {{ row.curiefense.attrs.company }}
                                </section>
                                <section class="is-size-7 tag2 has-background-white-bis-3">
                                  {{ row.curiefense.attrs.country }}
                                </section>
                                <section class="is-size-7 tag2 has-background-white-bis-3">rowid: {{
                                    row.rowid
                                  }}
                                </section>
                                <section class="is-size-7 tag2 has-background-white-bis-3">{{ row.requestid }}</section>
                              </div>
                            </article>
                          </div>
                        </div>
                      </td>
                    </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
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
import Utils from '@/assets/Utils.ts'
import Vue from 'vue'

export default Vue.extend({

  name: 'AccessLog',
  props: {},
  components: {},


  data() {
    return {

      loading: false,

      databaseRows: [],
      rowEntryIndex: -1,

      logFilterInput: '',
      logFilterInterval: '30 minutes',
      logFilterCriteria: '',
      logFilterSQL: `${DatasetsUtils.ACCESSLOG_SQL} ${DatasetsUtils.ACCESSLOG_SQL_SUFFIX}`,

      apiRoot: DatasetsUtils.LogsAPIRoot,
      apiVersion: DatasetsUtils.LogsAPIVersion,
      titles: DatasetsUtils.Titles,

    }
  },

  computed: {
    rows(): any[] {
      return _.filter(this.databaseRows, (row) => {
        return row.curiefense.attrs
      })
    },
  },

  methods: {

    buildQuery() {
      this.logFilterCriteria = `WHERE (starttime > now() - interval '${this.logFilterInterval}')`

      if (this.logFilterInput.trim().length > 0) {
        const dynamicFilter = _.join(_.map(this.logFilterInput.split(', '), (val) => {
          return `json_row ~ '${val}'`
        }), ' AND ')
        this.logFilterCriteria += ` AND (${dynamicFilter})`
      }

      this.logFilterSQL =
          `${DatasetsUtils.ACCESSLOG_SQL} ${this.logFilterCriteria} ${DatasetsUtils.ACCESSLOG_SQL_SUFFIX}`
      this.loadDatabases()
    },

    fulluri(row: any) {
      const [scheme, authority, path] = [
        row.curiefense.headers['x-forwarded-proto'],
        row.curiefense.attrs.authority,
        row.curiefense.attrs.path,
      ]

      return `${scheme}://${authority}${path}`
    },

    suburi(row: any) {
      const [scheme, authority, path] = [
        row.curiefense.headers['x-forwarded-proto'],
        row.curiefense.attrs.authority,
        row.curiefense.attrs.path,
      ]

      return `${scheme}://${authority}${path}`.substring(0, 58)
    },

    subNum10(num: number) {
      return num.toString().substring(0, 10)
    },

    break_reason(reason: string) {
      const width = 16
      const spacer = ' '.repeat(width)
      console.log(reason)
      return _.map(reason, (value, key) => {
        return `${key}${spacer}:`.substring(0, width) + value
      }).join('\n')
    },

    statuscode_class(code: number) {
      if (code < 400) return 'has-text-primary-dark'
      if (code < 500) return 'has-text-danger-dark '
      if (code < 1000) return 'has-text-danger-dark has-background-danger-light '
    },

    isempty(obj: any) {
      return _.isEmpty(obj)
    },

    isodate(timestamp: any) {
      return (new Date(timestamp)).toISOString().substr(0, 19)
    },

    tags(tagsDict: any) {
      return _.sortBy(_.keys(tagsDict))
    },

    loadDatabases() {
      this.loading = true
      console.log('loadDatabases')
      const payload = {
        statement: this.logFilterSQL,
        parameters: [] as any[],
      }

      RequestsUtils.sendLogsRequest('POST', 'exec/', payload).then((response: any) => {
        this.loading = false
        const rows = response.data
        this.databaseRows = _.map(rows, (row) => {
          return JSON.parse(row.slice(-1))
        })
      }).catch(() => {
        this.loading = false
      })
    },

    downloadDoc() {
      Utils.downloadFile('AccessLog', 'json', this.rows)
    },

  },

  mounted() {
    this.loading = true
    this.buildQuery()
  },

})
</script>
<style scoped lang="scss">

.has-row-clickable > td {
  cursor: pointer;
}

.borderless > td {
  border-bottom-width: 0;
  padding-top: 8px;
}

.expanded > td {
  padding-bottom: 20px;
}

.tag2 {
  line-height: 1.5;
  margin-bottom: 4px;
  padding: 0.21em 0.75em 0.25em;
  vertical-align: middle;
}

.data-entry-wrapper {
  border-bottom: 1px solid hsl(0, 0%, 83%);
}

tr:last-child > td {
  border-bottom-width: 1px;
}

.borderless:last-child > td {
  border-bottom-width: 0;
}

</style>
