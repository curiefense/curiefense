<template>
  <div>
    <div class="card">
      <div class="card-content">
        <div class="media">
          <div class="media-content">
            <div class="columns">
              <div class="column">
                <div class="field is-grouped" >
                  <input class="input is-small is-fullwidth" type="text" placeholder="Document name" v-model="selectedDoc.name" readonly>
                </div>
                <p class="subtitle is-6 has-text-grey" title="Document ID">{{ selectedDoc.id }}</p>
              </div>
              <div class="column"></div>
              <div class="column"></div>
            </div>
          </div>
        </div>

        <div class="content">
          <div class="field">
            <p class="control has-icons-left">
              <input class="input is-small" type="text"
                placeholder="matching domain(s) regex" required
                v-model="selectedDoc.operand" readonly>
              <span class="icon is-small is-left has-text-grey">
                <i class="fas fa-code"></i>
              </span>
            </p>
          </div>
          <div class="field is-grouped is-grouped-multiline">
            <div class="control">
              <div class="tags has-addons">
                <span class="tag">severity</span>
                <span class="tag has-text-info">{{ selectedDoc.severity }}</span>
              </div>
            </div>

            <div class="control">
              <div class="tags has-addons">
                <span class="tag">certainty</span>
                <span class="tag has-text-info">{{ selectedDoc.certainity }}</span>
              </div>
            </div>

            <div class="control">
              <div class="tags has-addons">
                <span class="tag">category</span>
                <span class="tag has-text-info">{{ selectedDoc.category }}</span>
              </div>
            </div>

            <div class="control">
              <div class="tags has-addons">
                <span class="tag">subcategory</span>
                <span class="tag has-text-info">{{ selectedDoc.subcategory }}</span>
              </div>
            </div>

          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>

export default {
  name: 'WAFSigsEditor',
  props: {
    selectedDoc: Object
  }
}
</script>
