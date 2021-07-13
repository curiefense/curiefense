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
<style scoped lang="scss">
html {
  background: hsl(206, 23%, 94%);
}

.navbar {
  margin-bottom: 1rem;
}

.columns {
  height: 100%;
  margin-left: 0;
  width: 100%;
}

</style>
