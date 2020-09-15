const { join } = require('path')

module.exports = {
  preset: 'ts-jest',
  globals: {
    'ts-jest': {
      diagnostics: { warnOnly: true },
    },
  },
  testEnvironment: join(__dirname, 'tests/nexus-test-environment.js'),
  testResultsProcessor: 'jest-sonar-reporter',
  collectCoverage: true,
  collectCoverageFrom: ['api/**/*.ts'],
}