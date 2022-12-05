# vue-lodash

[![NPM](https://nodei.co/npm/vue-lodash.png?downloadRank=true)](https://nodei.co/npm/vue-lodash/)
[![Known Vulnerabilities](https://snyk.io/test/github/ewocker/vue-lodash/badge.svg)](https://snyk.io/test/github/ewocker/vue-lodash)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/ewocker/vue-lodash/master/LICENSE)

> A small wrapper for integrating lodash to Vuejs
(Inspired by [vue-axios](https://github.com/imcvampire/vue-axios) plugin)

## Install

```
npm install --save vue-lodash lodash
```

## Usage

```javascript
import Vue from 'vue'
import VueLodash from 'vue-lodash'
import lodash from 'lodash'

// name is optional
Vue.use(VueLodash, { name: 'custom' , lodash: lodash })

new Vue({
  el: '#app',
  methods: {
    test() {
      console.log(this.lodash.random(20))
      console.log(this._.random(20))
      console.log(this.custom.random(20))
    },
  }
})
console.log(Vue.lodash.random(20))
console.log(Vue._.random(20))
console.log(Vue.custom.random(20))
```

## Typescript

Using __custom name__ with typescript is currently unsupported without casting.

```typescript
import Vue from 'vue'
import VueLodash from 'vue-lodash'
import lodash from 'lodash'

Vue.use(VueLodash, { name: 'custom', lodash: { map, random } })

new Vue({
  el: '#app',
  methods: {
    testLodash() {
      console.log(this.lodash.random(20))
      console.log(this._.random(20))
      console.log((this as any).custom.random(20))
    },
  }
})
console.log(Vue.lodash.random(20))
console.log(Vue._.random(20))
console.log((Vue as any).custom.random(20))
```

## Tree shaking with lodash

Only import the packages you need, note that lodash tree shaking has to do import with path to module.

```typescript
import Vue from 'vue'
import VueLodash from 'vue-lodash'
import random from 'lodash/random'
import map from 'lodash/map'

Vue.use(VueLodash, { name: 'custom', lodash: { map, random } })
```
