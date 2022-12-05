(function () {

  /**
   * Install plugin
   * @param Vue
   * @param lodash
   */
  
  function plugin(Vue, opt) {
  
    if (plugin.installed) {
      return
    }
    plugin.installed = true
  
    if (!opt || !opt.lodash) {
      console.error('You have to install lodash')
      return
    }

    Vue.lodash = opt.lodash
    Vue._ = opt.lodash
  
    Object.defineProperties(Vue.prototype, {
      lodash: { get() { return opt.lodash } },
      _: { get() { return opt.lodash } }
    })

    if (opt.name) {
      Vue[opt.name] = opt.lodash
      Object.defineProperties(Vue.prototype, {
        [opt.name]: { get() { return opt.lodash } }
      })
    }
  }
  
  if (typeof exports == "object") {
    module.exports = plugin
  } else if (typeof define == "function" && define.amd) {
    define([], function(){ return plugin })
  } else if (typeof window !== 'undefined' && window.Vue && window.lodash) {
    Vue.use(plugin, window.lodash)
  }
  
  })();