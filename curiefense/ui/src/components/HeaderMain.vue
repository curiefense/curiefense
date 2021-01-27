<template>
  <div class="container">
    <div class="navbar-brand">
      <a class="navbar-item">
        <img src="../assets/logo.png" class="logo">
      </a>
    </div>
    <div class="version-box is-size-7 has-text-grey">
      client version: {{ version || '0.0.0' }}
    </div>
  </div>
</template>

<script lang="ts">

import {version} from '@/../package.json'
import Vue from 'vue'

export default Vue.extend({

  name: 'HeaderMain',
  data() {
    return {
      version: version
    }
  }

})
</script>

<style scoped>
.container {
  display: flex;
  justify-content: space-between;
}

.version-box {
  align-self: center;
}
</style>
