module.exports = {
  'env': {
    'browser': true,
    'es6': true,
  },
  'extends': [
    'plugin:vue/essential',
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
  'ignorePatterns': ['public/index.html'],
  'rules': {
    'semi': 'off',
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
    }],
  },
}
