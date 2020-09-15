// tests/__helpers.ts
import { createTestContext as originalCreateTestContext, TestContext } from 'nexus/testing'
import {nanoid} from 'nanoid'

process.env.APP_SECRET = nanoid(30)

export function createTestContext(): TestContext {
  let ctx = {} as TestContext

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