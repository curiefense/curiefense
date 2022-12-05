export = StylelintError;
declare class StylelintError extends Error {
  /**
   * @param {string=} messages
   */
  constructor(messages?: string | undefined);
  stack: string;
}
