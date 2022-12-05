module.exports = {
  extends: [
    './index.js',
    'plugin:@typescript-eslint/recommended'
  ],
  
  // the ts-eslint recommended ruleset sets the parser so we need to set it back
  parser: require.resolve('vue-eslint-parser'),

  rules: {
    // this rule, if on, would require explicit return type on the `render` function
    '@typescript-eslint/explicit-function-return-type': 'off'
  },

  overrides: [
    {
      files: ['shims-tsx.d.ts'],
      rules: {
        '@typescript-eslint/no-empty-interface': 'off',
        '@typescript-eslint/no-explicit-any': 'off',
        '@typescript-eslint/no-unused-vars': 'off'
      }
    },
    {
      files: ['*.js', '*.cjs'],
      rules: {
        // in plain CommonJS modules, you can't use `import foo = require('foo')` to pass this rule, so it has to be disabled
        '@typescript-eslint/no-var-requires': 'off'
      }
    }
  ]
}
