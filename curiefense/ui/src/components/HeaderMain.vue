<template>
  <div class="container">
    <div class="navbar-brand">
      <a class="navbar-item">
        <img src="../assets/logo.png" class="logo" alt="logo">
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
      version: version,
    }
  },

})
</script>

<style scoped lang="scss">
.container {
  display: flex;
  justify-content: space-between;
}

.version-box {
  align-self: center;
}

.logo {
  max-height: 42px;
  min-height: 42px;
}
</style>
