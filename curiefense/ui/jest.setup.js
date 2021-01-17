import '@testing-library/jest-dom'
import Vue from 'vue'
import _ from 'lodash'
import VueAxios from 'vue-axios'
import axios from 'axios'
import {jest} from '@jest/globals'
Object.defineProperty(Vue.prototype, 'ld', { value: _ })
Vue.use(VueAxios, axios)
global.URL.createObjectURL = jest.fn()
