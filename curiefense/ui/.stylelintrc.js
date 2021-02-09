module.exports = {
  'processors': ['stylelint-processor-html'],
  'extends': [
    'stylelint-config-recommended',
    'stylelint-config-sass-guidelines',
  ],
  'plugins': [
    'stylelint-scss'
  ],
  'rules': {
    'at-rule-no-unknown': null,
    'scss/at-rule-no-unknown': true,
  },
}
