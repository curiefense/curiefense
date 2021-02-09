module.exports = {
  'extends': [
    'stylelint-config-recommended',
    'stylelint-config-sass-guidelines',
  ],
  'plugins': [
    'stylelint-scss',
  ],
  'rules': {
    'at-rule-no-unknown': null,
    'scss/at-rule-no-unknown': true,
    'no-empty-source': null,
    'selector-pseudo-element-no-unknown': [
      true,
      {
        ignorePseudoElements: ['v-deep']
      }
    ],
    'scss/at-extend-no-missing-placeholder': null,
  },
}
