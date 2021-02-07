import '@testing-library/jest-dom'
import Vue from 'vue'
import VueAxios from 'vue-axios'
import axios from 'axios'
import {jest} from '@jest/globals'
Vue.use(VueAxios, axios)
window.URL.createObjectURL = jest.fn()
