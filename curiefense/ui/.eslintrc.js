module.exports = {
  'root': true,
  'env': {
    'browser': true,
    'es6': true,
    'node': true,
  },
  'extends': [
    'plugin:vue/essential',
    'eslint:recommended',
    '@vue/typescript',
    'google',
  ],
  'globals': {
    'Atomics': 'readonly',
    'SharedArrayBuffer': 'readonly',
  },
  'parserOptions': {
    'ecmaVersion': 2018,
    'parser': '@typescript-eslint/parser',
    'sourceType': 'module',
  },
  'plugins': [
    'vue',
    '@typescript-eslint',
  ],
  'ignorePatterns': ['public/*', 'dist/*', 'coverage/*', '**/*.js'],
  'rules': {
    'semi': ['error', 'never'],
    'max-len': ['warn', {
      'code': 120,
      'comments': 140,
      'ignoreTrailingComments': true,
      'ignoreUrls': true,
    }],
    'require-jsdoc': 'off',
    'indent': ['error', 2, {
      'FunctionDeclaration': {
        'parameters': 'first',
      },
      'FunctionExpression': {
        'parameters': 'first',
      },
      'CallExpression': {
        'arguments': 'off',
      },
      'SwitchCase': 1,
    }],
    'space-in-parens': [2, 'never'],
    'array-bracket-spacing': [2, 'never'],
    'computed-property-spacing': [2, 'never'],
  },
}
