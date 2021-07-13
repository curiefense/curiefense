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
import RequestsUtils from '@/assets/RequestsUtils.ts'
import Vue from 'vue'

type menuItem = {
  title: string
  url?: string
  external?: boolean
  items?: {
    [key: string]: menuItem
  }
}

export default Vue.extend({
  name: 'SideMenu',
  data() {
    const swaggerURL = `${location.protocol}//${location.hostname}:30000/api/v1/`
    const kibanaURL = `${location.protocol}//${location.hostname}:5601/app/discover`
    const grafanaURL = `${location.protocol}//${location.hostname}:30300/`
    const prometheusURL = `${location.protocol}//${location.hostname}:9090/`

    return {
      defaultSwaggerURL: swaggerURL,
      defaultKibanaURL: kibanaURL,
      defaultGrafanaURL: grafanaURL,
      defaultPrometheusURL: prometheusURL,
      menuItems: {
        settings: {
          '/config': {
            title: 'Policies & Rules',
            items: {
              '/search': {title: 'Search'},
            },
          },
          '/db': {
            title: 'Databases',
          },
          '/publish': {
            title: 'Publish Changes',
          },
          'swagger': {
            title: 'API',
            url: swaggerURL,
            external: true,
          },
        },
        analytics: {
          'kibana': {
            title: 'Kibana',
            url: kibanaURL,
            external: true,
          },
          'grafana': {
            title: 'Grafana',
            url: grafanaURL,
            external: true,
          },
          'prometheus': {
            title: 'Prometheus',
            url: prometheusURL,
            external: true,
          },
        },
        git: {
          '/versioncontrol': {
            title: 'Version Control',
          },
        },
        docs: {
          'curiebook': {
            title: 'Curiebook',
            url: 'https://docs.curiefense.io/',
            external: true,
          },
        },
      } as {
        [key: string]: {
          [key: string]: menuItem
        }
      },
    }
  },
  computed: {
    currentRoutePath() {
      return this.$route.path
    },
  },
  methods: {
    async loadLinksFromDB() {
      const systemDBData = (await RequestsUtils.sendRequest( {methodName: 'GET', url: `db/system/`} ))?.data
      const swaggerURL = systemDBData?.links?.swagger_url ? systemDBData.links.swagger_url : this.defaultSwaggerURL
      const kibanaURL = systemDBData?.links?.kibana_url ? systemDBData.links.kibana_url : this.defaultKibanaURL
      const grafanaURL = systemDBData?.links?.grafana_url ? systemDBData.links.grafana_url : this.defaultGrafanaURL
      const prometheusURL = systemDBData?.links?.prometheus_url ? systemDBData.links.prometheus_url : this.defaultPrometheusURL
      this.menuItems.settings.swagger = {
        title: 'API',
        url: swaggerURL,
        external: true,
      }
      this.menuItems.analytics.kibana = {
        title: 'Kibana',
        url: kibanaURL,
        external: true,
      }
      this.menuItems.analytics.grafana = {
        title: 'Grafana',
        url: grafanaURL,
        external: true,
      }
      this.menuItems.analytics.prometheus = {
        title: 'Prometheus',
        url: prometheusURL,
        external: true,
      }
    },
  },
  async mounted() {
    await this.loadLinksFromDB()
  },
})
</script>
<style scoped lang="scss">
.menu-item {
  margin-top: 1.5rem;

  &:first-child {
    margin-top: 0;
  }
}

.menu-label {
  color: #8f99a3;
  font-weight: 700;
  margin-bottom: 0;
}

.menu-list {
  a {
    color: #0f1d38;
    font-size: 14px;
    font-weight: 700;
  }

  a:hover {
    background-color: transparent;
    color: #276cda;
  }

  .is-active {
    background-color: transparent;
    color: #276cda;
    font-weight: 700;
  }
}

</style>
