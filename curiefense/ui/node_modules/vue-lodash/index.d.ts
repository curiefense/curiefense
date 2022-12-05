import Vue, { PluginFunction, PluginObject } from "vue"
import { LoDashStatic } from "lodash"

declare module "vue/types/vue" {

  interface Vue {
    lodash: LoDashStatic
    _: LoDashStatic
  }

  interface VueConstructor {
    lodash: LoDashStatic
    _: LoDashStatic
  }
}

declare class VueLodash {
  static install: PluginFunction<LoDashStatic>
}

export default VueLodash