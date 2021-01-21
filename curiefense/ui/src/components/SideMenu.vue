<template>

  <aside class="menu mt-3">
    <div v-for="(sectionItems, sectionTitle) in menuItems" :key="sectionTitle" class="menu-item">
      <p class="menu-label">
        {{ sectionTitle }}
      </p>
      <ul class="menu-list">
        <li v-for="(menuItemDetails, menuItemKey) in sectionItems" :key="menuItemKey" class="section-item">
          <a v-if="menuItemDetails.external"
             :data-curie="menuItemKey"
             :href="menuItemKey"
             target="_blank">
            {{ menuItemDetails.title }}
          </a>
          <router-link v-else
                       :data-curie="menuItemKey"
                       :to="menuItemKey"
                       :class="{ 'is-active': currentRoutePath.includes(menuItemKey) }">
            {{ menuItemDetails.title }}
          </router-link>
          <ul v-if="menuItemDetails.items"
              class="my-0">
            <li v-for="(menuSubItemDetails, menuSubItemKey) in menuItemDetails.items" :key="menuSubItemKey">
              <router-link :data-curie="menuSubItemKey"
                           :to="menuSubItemKey"
                           :class="{ 'is-active': currentRoutePath.includes(menuSubItemKey) }">
                {{ menuSubItemDetails.title }}
              </router-link>
            </li>
          </ul>
        </li>
      </ul>
    </div>
  </aside>

</template>

<script>
export default {
  name: 'SideMenu',
  data() {
    const swaggerURL = `${location.protocol}//${location.hostname}:30000/api/v1/`
    const kibanaURL = `${location.protocol}//${location.hostname}:5601/app/discover`
    const grafanaURL = `${location.protocol}//${location.hostname}:30300/`

    return {
      menuItems: {
        Settings: {
          '/config': {
            title: 'Policies & Rules',
            items: {
              '/search': {'title': 'Search'}
            }
          },
          '/db': {
            title: 'System DB'
          },
          '/publish': {
            title: 'Publish Changes'
          },
          [swaggerURL]: {
            title: 'API',
            external: true
          },
        },
        Analytics: {
          [kibanaURL]: {
            title: 'Access Log (ELK)',
            external: true
          },
          [grafanaURL]: {
            title: 'Grafana',
            external: true
          },
        },
        Git: {
          '/versioncontrol': {
            title: 'Version Control'
          },
        },
        Docs: {
          'https://docs.curiefense.io/': {
            title: 'Curiebook',
            external: true
          },
        }
      }
    }
  },
  computed: {
    currentRoutePath() {
      return this.$route.path
    }
  },
  methods: {},
}
</script>
<style scoped lang="scss">
.menu-item {
  margin-top: 1.5rem;

  &:first-child {
    margin-top: 0
  }
}

.menu-label {
  color: #8f99a3;
  font-weight: 700;
  margin-bottom: 0;
}

.menu-list a {
  color: #0f1d38;
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
