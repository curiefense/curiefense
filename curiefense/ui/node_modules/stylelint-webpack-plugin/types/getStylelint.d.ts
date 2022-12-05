/// <reference types="stylelint" />
export = getStylelint;
/**
 * @param {string|undefined} key
 * @param {Options} options
 * @returns {Linter}
 */
declare function getStylelint(
  key: string | undefined,
  { threads, ...options }: Options
): Linter;
declare namespace getStylelint {
  export {
    Stylelint,
    LintResult,
    Options,
    AsyncTask,
    LintTask,
    Linter,
    Worker,
  };
}
type Options = import('./options').Options;
type Linter = {
  api: import('stylelint').InternalApi;
  stylelint: Stylelint;
  lintFiles: LintTask;
  cleanup: AsyncTask;
  threads: number;
};
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
type AsyncTask = () => Promise<void>;
type LintTask = (files: string | string[]) => Promise<LintResult[]>;
type Worker = JestWorker & {
  lintFiles: LintTask;
};
import { Worker as JestWorker } from 'jest-worker';
