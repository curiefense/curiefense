/// <reference types="stylelint" />
export type stylelint = import('postcss').PluginCreator<
  import('stylelint').PostcssPluginOptions
> & {
  lint: (
    options: import('stylelint').LinterOptions
  ) => Promise<import('stylelint').LinterResult>;
  rules: {
    [k: string]: import('stylelint').Rule<any, any>;
  };
  formatters: {
    [k: string]: import('stylelint').Formatter;
  };
  createPlugin: (
    ruleName: string,
    plugin: import('stylelint').Plugin<any, any>
  ) => {
    ruleName: string;
    rule: import('stylelint').Rule<any, any>;
  };
  createLinter: (
    options: import('stylelint').LinterOptions
  ) => import('stylelint').InternalApi;
  resolveConfig: (
    filePath: string,
    options?:
      | Pick<
          import('stylelint').LinterOptions,
          'cwd' | 'config' | 'configFile' | 'configBasedir'
        >
      | undefined
  ) => Promise<import('stylelint').Config | undefined>;
  utils: {
    report: (problem: import('stylelint').Problem) => void;
    ruleMessages: <
      T extends import('stylelint').RuleMessages,
      R extends { [K in keyof T]: T[K] }
    >(
      ruleName: string,
      messages: T
    ) => R;
    validateOptions: (
      result: import('stylelint').PostcssResult,
      ruleName: string,
      ...optionDescriptions: import('stylelint').RuleOptions[]
    ) => boolean;
    checkAgainstRule: <T_1, O extends Object>(
      options: {
        ruleName: string;
        ruleSettings: import('stylelint').ConfigRuleSettings<T_1, O>;
        root: import('postcss').Root;
      },
      callback: (warning: import('postcss').Warning) => void
    ) => void;
  };
};
export type StylelintOptions = import('stylelint').LinterOptions;
export type FormatterType = import('stylelint').FormatterType;
export type OutputReport = {
  filePath?: string | undefined;
  formatter?: FormatterType | undefined;
};
export type PluginOptions = {
  context: string;
  emitError: boolean;
  emitWarning: boolean;
  exclude?: (string | string[]) | undefined;
  extensions: string | string[];
  failOnError: boolean;
  failOnWarning: boolean;
  files: string | string[];
  formatter: FormatterType;
  lintDirtyModulesOnly: boolean;
  quiet: boolean;
  stylelintPath: string;
  outputReport: OutputReport;
  threads?: (number | boolean) | undefined;
};
export type Options = Partial<PluginOptions & StylelintOptions>;
/** @typedef {import("stylelint")} stylelint */
/** @typedef {import("stylelint").LinterOptions} StylelintOptions */
/** @typedef {import("stylelint").FormatterType} FormatterType */
/**
 * @typedef {Object} OutputReport
 * @property {string=} filePath
 * @property {FormatterType=} formatter
 */
/**
 * @typedef {Object} PluginOptions
 * @property {string} context
 * @property {boolean} emitError
 * @property {boolean} emitWarning
 * @property {string|string[]=} exclude
 * @property {string|string[]} extensions
 * @property {boolean} failOnError
 * @property {boolean} failOnWarning
 * @property {string|string[]} files
 * @property {FormatterType} formatter
 * @property {boolean} lintDirtyModulesOnly
 * @property {boolean} quiet
 * @property {string} stylelintPath
 * @property {OutputReport} outputReport
 * @property {number|boolean=} threads
 */
/** @typedef {Partial<PluginOptions & StylelintOptions>} Options */
/**
 * @param {Options} pluginOptions
 * @returns {Partial<PluginOptions>}
 */
export function getOptions(pluginOptions: Options): Partial<PluginOptions>;
/**
 * @param {Options} pluginOptions
 * @returns {Partial<StylelintOptions>}
 */
export function getStylelintOptions(
  pluginOptions: Options
): Partial<StylelintOptions>;
