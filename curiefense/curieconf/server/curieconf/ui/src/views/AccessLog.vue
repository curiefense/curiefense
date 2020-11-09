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
                <select v-model="log_filter_interval">
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
              <input
                  class="input is-small"
                  placeholder="123.45.87.219, Verizon, POST, /login"
                  x-placeholder="type to filter the logs"
                  v-model="log_filter_input">
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
                    <tbody v-for="(row, idx) in rows" :key="idx" style="border-bottom: 1px solid lightgray;">
                    <tr @click="row_entry_idx = (row_entry_idx === idx ? -1 : idx)"
                        class="has-row-clickable"
                        :class="[row_entry_idx === idx ? ' has-background-white-bis' : '']"
                    >
                      <td class="is-size-7 has-text-centered  " :class="statuscode_class(row.responsecode)">
                        {{ row.responsecode }}
                      </td>
                      <td class="is-size-7" :title="row.curiefense.attrs.ip + ':' + row.downstreamremoteaddressport">
                        {{ row.curiefense.attrs.ip }}
                      </td>
                      <td class="is-size-7"><span
                          class="has-text-weight-medium is-family-secondary">{{ row.requestmethod }}</span> <span
                          :title="fulluri(row)">{{ suburi(row) }}</span></td>

                      <td class="is-size-7 is-120-px">&#8593;{{ row.requestheadersbytes + row.requestbodybytes }}
                        &#8595;{{ row.responseheadersbytes + row.responsebodybytes }}
                      </td>


                      <td v-if="row.upstreamremoteaddress" class="is-size-7"
                          :title="row.upstreamremoteaddress + ':' + row.upstreamremoteaddressPort">
                        {{ row.upstreamremoteaddress }}
                      </td>
                      <td v-else class="is-size-7">terminated</td>
                      <td class="is-size-7 is-150-px">{{ isodate(row.starttime) }}</td>
                      <td class="is-size-7" :rowspan="row_entry_idx === idx ? '2' : '1'">
                        <a class="has-text-grey" title="more details"
                        >{{ row_entry_idx === idx ? 'close' : 'expand' }}</a>
                      </td>

                    </tr>
                    <tr
                        v-if="row_entry_idx === idx"
                        class="expanded borderless has-background-white-bis">
                      <td colspan="12" style="padding: 14px;">
                        <!--  TOP TILE -->
                        <div class="tile is-ancestor">
                          <div class="tile is-parent">
                            <article class="tile is-child box">
                              <div class="content" style="overflow-wrap: anywhere;">
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
                              <div class="content" style="overflow-wrap: anywhere;">
                                <label class="has-text-weight-bold has-text-danger-dark">Risk details <span
                                    class="has-text-family-uppercase"
                                    v-if="row.curiefense.attrs.block_reason.initiator"
                                >({{ row.curiefense.attrs.block_reason.initiator }})</span></label><br/>
                                <pre class="is-size-7 is-family-code has-background-danger-light"
                                     style="width: 932px; overflow-wrap: anywhere;">{{ break_reason(row.curiefense.attrs.block_reason) }}</pre>
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
                                  <div class="content" style="overflow-wrap: anywhere;">
                                    <section v-if="!isempty(row.curiefense.headers)">
                                      <label class="label">Headers<span
                                          class="is-pulled-right">{{ row.requestheadersbytes }} bytes</span></label>
                                      <table class="table is-narrow borderless is-fullwidth">
                                        <tr v-for="(value, name) in row.curiefense.headers" :key="name">
                                          <td class="has-text-weight-medium is-size-7 is-200-px">{{ name }}</td>
                                          <td class="is-size-7">{{ value }}</td>
                                        </tr>
                                      </table>
                                    </section>
                                    <section v-if="!isempty(row.curiefense.cookies)">
                                      <hr/>
                                      <label class="label">Cookies</label>
                                      <table class="table is-narrow borderless">
                                        <tr v-for="(value, name) in row.curiefense.cookies" :key="name">
                                          <td class="has-text-weight-medium is-size-7 is-200-px">{{ name }}</td>
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
                                          <td class="has-text-weight-medium is-size-7 is-200-px">{{ name }}</td>
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
                                              <span>Remote: {{ row.upstreamremoteaddress }}:{{ row.upstreamremoteaddressport }}</span>
                                            </section>
                                            <section class="is-size-7" v-if="row.upstreamlocaladdress">
                                              <span>Local: {{ row.upstreamlocaladdress }}:{{ row.upstreamlocaladdressport }}</span>
                                            </section>
                                            <section class="is-size-7" v-if="row.upstreamcluster">
                                              <span>Cluster: {{ row.upstreamcluster }}</span></section>
                                            <section class="is-size-7" v-if="row.upstreamtransportfailurereason"><span>Failure: {{ row.upstreamtransportfailurereason }}</span>
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
                                              <span>Remote: {{ row.downstreamremoteaddress }}:{{ row.downstreamremoteaddressport }}</span>
                                            </section>
                                            <section class="is-size-7">
                                              <span>Local: {{ row.downstreamlocaladdress }}:{{ row.downstreamlocaladdressport }}</span>
                                            </section>
                                            <section class="is-size-7">
                                              <span>Direct: {{ row.downstreamdirectremoteaddress }}:{{ row.downstreamdirectremoteaddressport }}</span>
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
                                                :title="row.tlssessionid">Session Id: {{ row.tlssessionid.substr(0, 12) }}</span>
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
<script>

import DatasetsUtils from '@/assets/DatasetsUtils.js'
import RequestsUtils from '@/assets/RequestsUtils'

export default {

  name: 'AccessLog',
  props: {},
  components: {},


  data() {
    return {

      loading: false,

      dbrows: [],
      row_entry_idx: -1,

      log_filter_input: '',
      log_filter_interval: '30 minutes',
      log_filter_criteria: '',
      log_filter_sql: `${DatasetsUtils.ACCESSLOG_SQL} ${DatasetsUtils.ACCESSLOG_SQL_SUFFIX}`,

      apiRoot: DatasetsUtils.LogsAPIRoot,
      apiVersion: DatasetsUtils.LogsAPIVersion,
      titles: DatasetsUtils.Titles,

    }
  },

  computed: {
    rows() {
      return this.ld.filter(this.dbrows, (row) => {
        return row.curiefense.attrs
      })
    }
  },

  methods: {

    buildQuery() {
      this.log_filter_criteria = `WHERE (starttime > now() - interval '${this.log_filter_interval}')`

      if (this.log_filter_input.trim().length > 0) {
        let dynamic_filter = this.ld.join(this.ld.map(this.log_filter_input.split(', '), (val) => {
          return `json_row ~ '${val}'`
        }), ' AND ')
        this.log_filter_criteria += ` AND (${dynamic_filter})`
      }

      this.log_filter_sql = `${DatasetsUtils.ACCESSLOG_SQL} ${this.log_filter_criteria} ${DatasetsUtils.ACCESSLOG_SQL_SUFFIX}`
      this.loadDatabases()
    },

    fulluri(row) {
      let [scheme, authority, path] = [
        row.curiefense.headers['x-forwarded-proto'],
        row.curiefense.attrs.authority,
        row.curiefense.attrs.path
      ]

      return `${scheme}://${authority}${path}`
    },

    suburi(row) {
      let [scheme, authority, path] = [
        row.curiefense.headers['x-forwarded-proto'],
        row.curiefense.attrs.authority,
        row.curiefense.attrs.path
      ]

      return `${scheme}://${authority}${path}`.substring(0, 58)
    },

    subNum10(num) {
      return num.toString().substring(0, 10)
    },

    break_reason(reason) {
      const width = 16
      const spacer = ' '.repeat(16)

      return this.ld.map(reason, (value, key) => {
        return `${key}${spacer}:`.substring(0, width) + value
      }).join('\n')

    },

    statuscode_class(code) {
      if (code < 400) return 'has-text-primary-dark'
      if (code < 500) return 'has-text-danger-dark '
      if (code < 1000) return 'has-text-danger-dark has-background-danger-light '
    },
    isempty(obj) {
      return this.ld.isEmpty(obj)
    },
    isodate(timestamp) {
      return (new Date(timestamp)).toISOString().substr(0, 19)
    },

    tags(tags_dict) {
      return this.ld.sortBy(this.ld.keys(tags_dict))
    },

    loadDatabases() {
      this.loading = true
      console.log('loadDatabases')
      let payload = {
        statement: this.log_filter_sql,
        parameters: []
      }

      RequestsUtils.sendLogsRequest('POST', 'exec/', payload)
          .then((response) => {
            this.loading = false
            let rows = response.data
            this.dbrows = this.ld.map(rows, (row) => {
              return JSON.parse(row.slice(-1))
            })
          })
          .catch(() => {
            this.loading = false
          })
    },

    downloadDoc() {
    },

  },

  mounted() {
    this.loading = true
    this.buildQuery()
  }

}

</script>
<style type="text/css" scoped>

.is-200-px {
  min-width: 200px;
  max-width: 200px;
  width: 200px;
}

.is-150-px {
  min-width: 150px;
  max-width: 150px;
  width: 150px;
}

.is-120-px {
  min-width: 120px;
  max-width: 120px;
  width: 120px;
}

tr.has-row-clickable > td {
  cursor: pointer;
}

.content table tbody tr:last-child td {
  border-bottom-width: 1px;
}

.content table tbody tr.borderless:last-child td {
  border-bottom-width: 0;
}

tr.borderless > td {
  padding-top: 8px;
  border-bottom-width: 0;
}

tr.expanded > td {
  padding-bottom: 20px;
}

section.tag2 {
  padding: .21em .75em .25em;
  margin-bottom: 4px;
  vertical-align: middle;
  line-height: 1.5;
}

</style>