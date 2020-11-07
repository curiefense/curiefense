<template>
  <div class="card">
    <div class="card-content">
      <div class="content">
        <div class="columns">
          <div class="column is-4" style="border-right:solid 2px #f8f8f8; ">
            <div class="field">
              <label class="label is-small">Name</label>
              <div class="control">
                <input class="input is-small" placeholder="list name" v-model="selectedDoc.name"
                  :readonly="selectedDoc.source === 'reblaze-managed'" />
              </div>
              <p class="subtitle is-7 has-text-grey">{{selectedDoc.id + '\t|\t' + listTotalEntries + ' entries.'}}</p>
            </div>
            <div class="field">
              <a v-if="selectedDoc && selectedDoc.source && selectedDoc.source.indexOf('http') === 0"
                class="is-small has-text-grey is-size-7 is-pulled-right"
                @click="fetchList"
                >update now</a>
              <label class="label is-small">Last update</label>
              <div class="control">
                <input class="input is-small" v-model="selectedDoc.mdate" readonly />
              </div>
            </div>
            <div class="field">
              <label class="label is-small">Tags</label>
              <div class="control">
                <tag-autocomplete-input :initialTag="selectedDocTags"
                                        :selectionType="'multiple'"
                                        @tagChanged="selectedDocTags = $event">
                </tag-autocomplete-input>
              </div>
              <p class="help">Separated by space.</p>
            </div>
            <div class="field">
              <label class="label is-small">Source</label>
              <div class="control">
                <input class="input is-small" v-model="selectedDoc.source" :readonly="selectedDoc.source === 'reblaze-managed'" />
              </div>
              <p class="help">Only 'self-managed' lists are fully editable. For Internet sourced lists, only metadata is editable.</p>
            </div>
            <div class="field">
              <label class="label is-small">Entries Relation</label>
              <div class="control is-expanded">
                <div class="select is-small is-size-7 is-fullwidth">
                  <select
                    v-model="selectedDoc.entries_relation"
                    :readonly="selectedDoc.source === 'reblaze-managed'"
                    :disabled="selectedDoc.source === 'reblaze-managed'">
                    <option value="OR">OR</option>
                  </select>
                </div>
              </div>
              <p class="help">Logical relations between different entries in different categories.</p>
            </div>
            <div class="field">
              <label class="checkbox is-size-7">
                <input type="checkbox"
                  :readonly="selectedDoc.source === 'reblaze-managed'"
                  :disabled="selectedDoc.source === 'reblaze-managed'"
                  v-model="selectedDoc.active">
                Active
              </label>
            </div>
            <div class="field">
              <label class="label is-small">Notes</label>
              <div class="control">
                <textarea class="is-small textarea" v-model="selectedDoc.notes" rows="2" :readonly="selectedDoc.source === 'reblaze-managed'"></textarea>
              </div>
            </div>
          </div>
          <div class="column is-7">
            <div v-if="newentry && editable">
              <table class="table is-narrow is-fullwidth" >
                <thead>
                  <tr>
                    <th class="is-size-7">Category</th>
                    <th class="is-size-7">Entry</th>
                    <th class="is-size-7 is-48-px">
                      <a class="is-small has-text-grey" title="cancel" @click="newentry = false">cancel</a>
                    </th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td class="is-size-7">
                      <div class="select is-small is-fullwidth">
                        <select v-model="newentry_category" class="select">
                          <option v-for="(entry, category) in list_entry_types" :key="category" :value="category">{{ entry.title }}</option>
                        </select>
                      </div>
                    </td>
                    <td class="is-size-7">
                      <textarea rows="3"
                        class="textarea is-small is-fullwidth"
                        :placeholder="inputDescription"
                        v-model="newentry_items"></textarea>
                    </td>
                    <th class="is-size-7 is-48-px">
                      <a class="is-small has-text-grey" title="add entry" @click="addEntry">add</a>
                    </th>
                  </tr>
                </tbody>
              </table>
              <hr/>
            </div>
            <table class="table is-narrow">
              <thead>
                <tr>
                  <th class="is-size-7 is-48-px">
                  <th class="is-size-7">Category</th>
                  <th class="is-size-7">Entry</th>
                  <th class="is-size-7">Annotation</th>
                  <th class=" is-size-7 is-48-px">
                    <a v-if="editable "
                      class="has-text-grey-dark is-small is-pulled-right" title="Add new entry" @click="newentry = true">
                      <span class="icon is-small"><i class="fas fa-plus"></i></span>
                    </a>
                  </th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(entry,idx) in selectedDocPage" :key="idx">
                  <td class="is-size-7 is-48-px has-text-right has-text-grey-light">{{ ((idx+1) + ((currentPage-1)*rowsPerPage) )}}</td>
                  <td class="is-size-7">{{ list_entry_types[entry[0]].title }}</td>
                  <td class="is-size-7"><span v-html="dualCell(entry[1])"></span></td>
                  <td class="is-size-7" :title="entry[2]">{{ entry[2] ? entry[2].substr(0,40) : ""}}</td>
                  <td class="is-size-7 is-48-px">
                    <a v-if="editable "
                      class="is-small has-text-grey" title="remove entry"
                      @click="removeEntry(currentPage, idx)"
                    >remove</a>
                  </td>
                </tr>
                <tr v-if="totalPages > 1">
                  <td colspan="5">
                    <nav class="pagination is-small" role="navigation" aria-label="pagination">
                      <a :disabled="currentPage === 1" class="is-pulled-left pagination-previous" @click="navigate(currentPage - 1)">Previous Page</a>
                      <a :disabled="currentPage === totalPages" class="is-pulled-right pagination-next" @click="navigate(currentPage + 1)">Next page</a>
                    </nav>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
        <span class="is-family-monospace  has-text-grey-lighter">{{apiPath}}</span>
      </div>
    </div>
  </div>
</template>

<script>

import _ from 'lodash'

import axios from 'axios'
import DatasetsUtils from '@/assets/DatasetsUtils.js'
import TagAutocompleteInput from '@/components/TagAutocompleteInput'

export default {
  name: 'ProfilingListEditor',

  components: {
    TagAutocompleteInput
  },

  props: {
    selectedDoc: Object,
    apiPath: String
  },

  data() {
    return {
      newlist: false,

      list_idx: 0,
      rowsPerPage : 20,
      currentPage: 1,

      lists: [],

      list_entry_types: {
        "path":     {"title": "Path", "pair": false},
        "query":    {"title": "Query", "pair": false},
        "uri":      {"title": "URI", "pair": false},
        "method":   {"title": "Method", "pair": false},
        "ip":       {"title": "IP Address", "pair": false},
        "asn":      {"title": "ASN", "pair": false},
        "country":  {"title": "Country", "pair": false},
        "headers":  {"title": "Headers", "pair": true},
        "args":     {"title": "Arguments", "pair": true},
        "cookies":  {"title": "Cookies", "pair": true},
      },

      newentry: false,
      // start with most common
      newentry_category: "ip",
      newentry_items: ""
    }
  },
  computed: {

    inputDescription() {
      if ((new RegExp("(args|cookies|headers)")).test(this.newentry_category)) {
        return "1st line: NAME\n2nd line: VALUE\n3rd line: optional annotation"
      }
      return "One entry per line, use '#' for annotation\ne.g. 12.13.14.15 #San Jose Office"
    },

    totalPages () {
      return Math.ceil(this.listTotalEntries/this.rowsPerPage)
    },

    listTotalEntries() {
      return this.selectedDoc?.entries?.length
    },

    readonly () {
      return this.selectedDoc.source === 'reblaze-managed'
    },

    editable () {
      return this.selectedDoc.source === 'self-managed'
    },

    selectedDocPage () {

      if (!this.selectedDoc.entries) { return [] }

      let entries = this.selectedDoc.entries
      entries = this.ld.slice(entries, (this.currentPage-1) * this.rowsPerPage, this.rowsPerPage*this.currentPage)
      return entries
    },

    selectedDocTags: {
      get: function () {
        if (this.selectedDoc.tags)
          return this.selectedDoc.tags.join(" ")
        return ""
      },
      set: _.debounce(function(tags) {
          this.selectedDoc.tags = this.ld.map(tags.split(" "), (tag)=>{ return tag.trim()})
      }, 500)
    },

  },

  methods: {
    dualCell(cell) {
      if (this.ld.isArray(cell)) {
        return "name: " + cell[0] +"<br/>" + "value: " + cell[1]
      }
      else {
        return cell
      }
    },
    navigate(pagenum) {
      if (pagenum >= 1 && pagenum <= this.totalPages) {
        this.currentPage = pagenum
      }
    },

    listSelected(event) {
      console.log(event)
      this.currentPage = 1
    },

    notify(message, error) {
      this.notification.message = message

      if (error)
        this.notification.class = "is-danger"
      else
        this.notification.class = "is-success"

      this.notification.view = true

    },

    addEntry() {
      let list_entries = this.selectedDoc.entries;
      // dual cell
      if ((new RegExp("(args|cookies|headers)")).test(this.newentry_category))  {
        let entries = this.newentry_items.trim().split("\n")
        if (entries.length > 3 || entries.length < 2) {
          this.notify("Use 2 lines for headers, arguments and cookies, name and value each. 3rd line for annotation is optional", true)
        }
        else {
          list_entries.unshift([ this.newentry_category, [ entries[0].trim(), entries[1].trim() ], entries[2].trim() ])
        }
      }
      // single line entry
      else {

        this.ld.each(this.newentry_items.split("\n"), (line) => {
          let [entry, annotation] = line.trim().split("#")
          annotation = annotation && annotation.trim()
          list_entries.unshift([this.newentry_category, entry.trim(), annotation])
        })
      }
      this.newentry_items = ""
      this.newentry = false
    },


    tryMatch(data, regex, type) {
      let matches, entries = [];

      matches = regex.exec(data)
      while (matches) {

        let entry = [type,  matches[1], null]

        if (matches.length > 2){
          entry[2] = (matches.slice(-1)[0] || "").slice(0,128)
        }

        entries.push( entry );
        matches = regex.exec(data)
      }

      return entries
      },

    fetchList() {
      const line_matching_ip = /^[^;]((((\d{1,3})\.){3}\d{1,3}(\/\d{1,2}))|([0-9a-f]+:+){1,8}([0-9a-f]+)?(\/\d{1,3})?)\s+([#;/].+)/gm,
            line_matching_asn    = /(as\d{3,6})((\s+)?([#;/?].+))?/gmi,
            single_ip = /^((((\d{1,3})\.){3}\d{1,3}(\/\d{1,2}))|([0-9a-f]+:+){1,8}([0-9a-f]+)?(\/\d{1,3})?)$/,
            single_asn = /(as\d{3,6})/i;

      // try every node / element of String type with the regex.
      let object_parser = (data, store) => {
            this.ld.each(data,
              (item) => {
                if  (this.ld.isArray(item) || this.ld.isObject(item)) {
                  object_parser(item, store)
                }
                if (this.ld.isString(item)) {
                  if (single_ip.test(item)) {
                    store.push(["ip", item, null])
                  }
                  else if (single_asn.test(item)) {
                    store.push(["asn", item, null])
                  }
                }
            })
      }

      let apiroot = DatasetsUtils.ConfAPIRoot,
          apiversion = DatasetsUtils.ConfAPIVersion,
          url = this.selectedDoc.source;


      axios.get(`${apiroot}/${apiversion}/tools/fetch?url=${url}`).then(
        (response) => {

          let data = response.data,
              entries = [];

          entries = this.tryMatch(data, line_matching_ip, "ip")

          if (entries.length === 0 ) {
            entries = this.tryMatch(data, line_matching_asn, "asn")
          }

          if (entries.length === 0 ) {
            try {
              object_parser(data, entries)
            }
            catch (e) {
              console.log(e)
            }
          }

          if (entries.length > 0 ) {
            this.selectedDoc.entries = entries;
            this.selectedDoc.mdate = (new Date).toISOString()
          }

        })
        .catch()

    },
    removeEntry(currentPage, idx) {
      let pointer = ((this.currentPage-1)*this.rowsPerPage)+idx
      this.selectedDoc.entries.splice(pointer,1)
    },

  },

}
</script>
