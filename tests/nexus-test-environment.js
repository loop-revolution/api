const { PrismaClient } = require('@prisma/client')
const NodeEnvironment = require('jest-environment-node')
const { nanoid } = require('nanoid')
const del = require('del')
const util = require('util')
const exec = util.promisify(require('child_process').exec)

const prismaBinary = './node_modules/.bin/prisma'

/**
 * Custom test environment for nexus and Postgres
 */
class PrismaTestEnvironment extends NodeEnvironment {
  constructor(config) {
    super(config)

    // Generate a unique schema identifier for this test context
    this.schema = `test_${nanoid()}`

    // Generate the pg connection string for the test schema
    this.connectionString = `postgres://postgres:postgres@localhost:5432/testing?schema=${this.schema}`
  }

  async setup() {
    // Set the required environment variable to contain the connection string
    // to our database test schema
    process.env.DATABASE_URL = this.connectionString
    this.global.process.env.DATABASE_URL = this.connectionString
    await del('./prisma/migrations')

    // Run the migrations to ensure our schema has the required structure
    await exec(`npx @prisma/cli@latest migrate up --experimental -c`)
    // ^ This uses the latest version of prisma because nexus-plugin-prisma doesn't use it yet
    // https://github.com/prisma/migrate/issues/572#issuecomment-689879825

    return super.setup()
  }

  async teardown() {
    const prisma = new PrismaClient()
    await prisma.$executeRaw(`DROP SCHEMA IF EXISTS "${this.schema}" CASCADE`)
    await prisma.$disconnect()
  }
}

module.exports = PrismaTestEnvironment
