/// <reference types="stylelint" />
export = linter;
/**
 * @param {string|undefined} key
 * @param {Options} options
 * @param {Compilation} compilation
 * @returns {{api: InternalApi, lint: Linter, report: Reporter, threads: number}}
 */
declare function linter(
  key: string | undefined,
  options: Options,
  compilation: Compilation
): {
  api: InternalApi;
  lint: Linter;
  report: Reporter;
  threads: number;
};
declare namespace linter {
  export {
    Stylelint,
    LintResult,
    InternalApi,
    Compiler,
    Compilation,
    Options,
    FormatterType,
    FormatterFunction,
    GenerateReport,
    Report,
    Reporter,
    Linter,
    LintResultMap,
  };
}
type Options = import('./options').Options;
type Compilation = import('webpack').Compilation;
type InternalApi = import('stylelint').InternalApi;
type Linter = (files: string | string[]) => void;
type Reporter = () => Promise<Report>;
type Stylelint = import('postcss').PluginCreator<
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
type LintResult = import('stylelint').LintResult;
type Compiler = import('webpack').Compiler;
type FormatterType = import('./options').FormatterType;
type FormatterFunction = (results: LintResult[]) => string;
type GenerateReport = (compilation: Compilation) => Promise<void>;
type Report = {
  errors?: StylelintError;
  warnings?: StylelintError;
  generateReportAsset?: GenerateReport;
};
type LintResultMap = {
  [files: string]: import('stylelint').LintResult;
};
import StylelintError = require('./StylelintError');
