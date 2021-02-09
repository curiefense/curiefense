// Add in the top of the file
const StyleLintPlugin = require('stylelint-webpack-plugin');

module.exports = {
  devServer: {
    // proxy: 'http://localhost:5000'
    proxy: {
      '^/conf/api': {
        pathRewrite: {'^/conf/api': '/api'},
        target: 'http://localhost:5000',
      },
      '^/logs/api': {
        pathRewrite: {'^/logs/api': '/api'},
        target: 'http://localhost:5001',
      },
    },
  },
  plugins: [
    new StyleLintPlugin({
      files: 'src/**/*.{vue,scss}',
    }),
  ],
}

