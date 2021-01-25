import Vue from 'vue'
import VueRouter from 'vue-router'

import axios from 'axios'
import VueAxios from 'vue-axios'

import _ from 'lodash'

import DatasetsUtils from '@/assets/DatasetsUtils.ts'
import ip6addr from 'ip6addr'
import MasterComponent from '@/views/MasterComponent.vue'
import DocumentEditor from '@/views/DocumentEditor.vue'
import DBEditor from '@/views/DBEditor.vue'
import Publish from '@/views/Publish.vue'
import AccessLog from '@/views/AccessLog.vue'
import VersionControl from '@/views/VersionControl'
import DocumentSearch from '@/views/DocumentSearch'

Object.defineProperty(Vue.prototype, 'ld', {value: _})
Object.defineProperty(Vue.prototype, 'dsutils', {value: DatasetsUtils})

Vue.use(VueRouter)
Vue.use(VueAxios, axios)
Vue.use(ip6addr)

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
                                        component: DocumentEditor
                                    },
                                ]
                            },
                        ]
                    },
                ]
            },
            {path: 'db', name: 'DBEditor', component: DBEditor},
            {path: 'publish', name: 'Publish', component: Publish},
            {path: 'accesslog', name: 'AccessLog', component: AccessLog},
            {path: 'versioncontrol', name: 'VersionControl', component: VersionControl},
            {path: 'search', name: 'DocumentSearch', component: DocumentSearch},
        ]
    },
    {
        path: '*',
        redirect: '/config'
    }
]

const router = new VueRouter({
    mode: 'history',
    base: process.env.BASE_URL,
    routes
})


export default router
