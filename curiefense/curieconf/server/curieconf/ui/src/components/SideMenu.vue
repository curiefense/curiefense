<template>

  <aside class="menu " style="position: fixed;">
    <div v-for="(sectionItems, sectionTitle) in menuItems" :key="sectionTitle">
      <p class="menu-label" style="margin-top: 12px">
        {{ sectionTitle }}
      </p>
      <ul class="menu-list">
        <li v-for="(menuItemDetails, menuItemKey) in sectionItems" :key="menuItemKey">
          <a :data-curie="menuItemKey"
             :href="menuItemKey"
             :class="{ 'is-active': currentRoutePath === menuItemKey }">
            {{ menuItemDetails.title }}
          </a>
        </li>
      </ul>
    </div>
  </aside>

</template>

<script>

export default {

  name: 'SideMenu',

  data() {
    return {
      selectedMenuItem: null,
      menuItems: {
        Configuration: {
          "/config": {"title": "Document Editor"},
          "/db": {"title": "System Settings"},
          "/publish": {"title": "Publish Configuration"},
        },
        Analytics: {
          "/accesslog": {"title": "Access Log"}
        },
        Git: {
          "/versioncontrol": {"title": "Version Control"}
        },
      }
    }
  },

  computed: {
    currentRoutePath() {
      return this.$route.path;
    }
  },

  methods: {},

}
</script>
<style scoped>
  .menu-label {
    color: #8F99A3;
    font-weight: 700;
  }
  .menu-list a {
    color: #0F1D38;
    font-size: 14px;
    font-weight: 700;
  }
  .menu-list a:hover {
    background-color: transparent;
    color: #276cda;
  }
  .menu-list a.is-active {
    background-color: transparent;
    color: #276cda;
    font-weight: 700;
  }
</style>
