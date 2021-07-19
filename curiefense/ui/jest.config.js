module.exports = {
  moduleFileExtensions: ['js', 'jsx', 'ts', 'tsx', 'json', 'vue'],
  transform: {
    '^.+\\.vue$': 'vue-jest',
    '^.+\\.jsx?$': 'babel-jest',
    '^.+\\.tsx?$': 'ts-jest',
  },
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/src/$1',
    'ace-builds': '<rootDir>/node_modules/ace-builds',
  },
  testMatch: [
    '<rootDir>/**/*.spec.(js|jsx|ts|tsx)',
  ],
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts'],
  transformIgnorePatterns: ['<rootDir>/node_modules/'],
  collectCoverageFrom: [
    'src/**/*.{ts,vue}',
    '!src/router/index.ts',
    '!coverage/*',
    '!src/main.ts',
    '!**/*.d.ts',
  ],
  // Accept global coverage of 90% or higher, and 80% for each individual file
  // This should ensure most of the code is covered and leaves us room to skip hard-to-test areas
  // Please only write meaningful and comprehensive tests
  // Please do not write fake tests for higher coverage (e.g. a test that runs all functions without asserting anything)
  coverageThreshold: {
    'global': {
      branches: 90,
      functions: 90,
      lines: 90,
      statements: 90,
    },
    './src/**': {
      branches: 80,
      functions: 80,
      lines: 80,
      statements: 80,
    },
    './src/assets/RequestsUtils.ts': {
      branches: 100,
      functions: 100,
      lines: 100,
      statements: 100,
    },
  },
}
