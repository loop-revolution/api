module.exports = {
  rules: {
    'prefer-const': ['warn'],
    'no-magic-numbers': [
      'warn',
      {
        ignore: [-1, 0, 1],
      },
    ],
  },
  env: {
    es2020: true,
  },
  ignorePatterns: ['nexus-test-environment.js'],
}
