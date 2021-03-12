import Vue from 'vue'
import VueRouter from 'vue-router'
import axios from 'axios'
import VueAxios from 'vue-axios'
import MasterComponent from '@/views/MasterComponent.vue'
import DocumentEditor from '@/views/DocumentEditor.vue'
import DBEditor from '@/views/DBEditor.vue'
import Publish from '@/views/Publish.vue'
import VersionControl from '@/views/VersionControl.vue'
import DocumentSearch from '@/views/DocumentSearch.vue'

Vue.use(VueRouter)
Vue.use(VueAxios, axios)

const routes = [
  {
    path: '/', name: 'MasterComponent', component: MasterComponent, redirect: '/config',
    children: [
      {
        path: 'config',
        name: 'DocumentEditor',
        component: DocumentEditor,
        children: [
          {
            path: ':branch',
            name: 'DocumentEditor/Branch',
            component: DocumentEditor,
            children: [
              {
                path: ':doc_type',
                name: 'DocumentEditor/Branch/DocType',
                component: DocumentEditor,
                children: [
                  {
                    path: ':doc_id',
                    name: 'DocumentEditor/Branch/DocType/DocID',
                    component: DocumentEditor,
                  },
                ],
              },
            ],
          },
        ],
      },
      {path: 'db', name: 'DBEditor', component: DBEditor},
      {path: 'publish', name: 'Publish', component: Publish},
      {path: 'versioncontrol', name: 'VersionControl', component: VersionControl},
      {path: 'search', name: 'DocumentSearch', component: DocumentSearch},
    ],
  },
  {
    path: '*',
    redirect: '/config',
  },
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes,
})


export default router
