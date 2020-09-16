// tests/__helpers.ts
import { createTestContext as originalCreateTestContext, TestContext } from 'nexus/testing'
import { nanoid } from 'nanoid'

const secretLength = 30
process.env.APP_SECRET = nanoid(secretLength)

export function createTestContext(): TestContext {
  const ctx = {} as TestContext

  beforeAll(async () => {
    Object.assign(ctx, await originalCreateTestContext())
    await ctx.app.start()
  })
  afterAll(async () => {
    await ctx.app.stop()
    await ctx.app.db.client.$disconnect()
  })
  return ctx
}
