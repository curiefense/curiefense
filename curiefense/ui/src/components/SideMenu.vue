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
             :href="menuItemDetails.url"
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

<script lang="ts">
import RequestsUtils from '@/assets/RequestsUtils'
import Vue from 'vue'

export default Vue.extend({
  name: 'SideMenu',
  data() {
    const swaggerURL = `${location.protocol}//${location.hostname}:30000/api/v1/`

    return {
      defaultKibanaURL: `${location.protocol}//${location.hostname}:5601/app/discover`,
      defaultGrafanaURL: `${location.protocol}//${location.hostname}:30300/`,
      menuItems: {
        settings: {
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
          'swagger': {
            title: 'API',
            url: swaggerURL,
            external: true
          },
        },
        analytics: {
          'kibana': {
            title: 'Access Log (ELK)',
            url: this.defaultKibanaURL,
            external: true
          },
          'grafana': {
            title: 'Grafana',
            url: this.defaultGrafanaURL,
            external: true
          }
        },
        git: {
          '/versioncontrol': {
            title: 'Version Control'
          },
        },
        docs: {
          'curiebook': {
            title: 'Curiebook',
            url: 'https://docs.curiefense.io/',
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
  methods: {
    async loadLinksFromDB() {
      const systemDBData = (await RequestsUtils.sendRequest('GET', `db/system/`)).data
      const kibanaURL = systemDBData?.links?.kibaba_url ? systemDBData.links.kibaba_url : this.defaultKibanaURL
      const grafanaURL = systemDBData?.links?.grafana_url ? systemDBData.links.grafana_url : this.defaultGrafanaURL
      this.menuItems.analytics.kibana = {
        title: 'Access Log (ELK)',
        url: kibanaURL,
        external: true
      }
      this.menuItems.analytics.grafana = {
        title: 'Grafana',
        url: grafanaURL,
        external: true
      }
    }
  },
  mounted() {
    this.loadLinksFromDB()
  }
})
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
