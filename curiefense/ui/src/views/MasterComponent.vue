<template>
  <div>
    <nav class="navbar is-fixed-top is-light" role="navigation" aria-label="main navigation">
      <header-main/>
    </nav>
    <br/>
    <div class="container">
      <div class="columns">
        <div class="column is-2">
          <side-menu></side-menu>
        </div>
        <div class="column is-10">
          <router-view></router-view>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import HeaderMain from '@/components/HeaderMain.vue'
import SideMenu from '@/components/SideMenu.vue'
import Vue from 'vue'

export default Vue.extend({
  name: 'MasterComponent',
  components: {HeaderMain, SideMenu},
  methods: {},
})
</script>
<style type="text/css">
html {
  background: #ECF0F3;
}

nav.navbar {
  border-top: 4px solid #276cda;
  margin-bottom: 1rem;
}

.columns {
  width: 100%;
  height: 100%;
  margin-left: 0;
}

</style>
