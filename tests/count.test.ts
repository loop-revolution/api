import { createTestContext } from './__helpers'

const ctx = createTestContext()

const countQuery = `
query {
  count
}
`

type countQueryResult = {
  count: number
}

describe('counter', () => {
  test('query count', async () => {
    const result: countQueryResult = await ctx.client.send(countQuery)
    // Should have a count
    expect(result).toHaveProperty('count')
    // Count should be a number
    expect(typeof result.count).toBe('number')
  })
  test('update count should increment', async () => {
    // The first count, to base everything off of
    const { count: initialCount }: countQueryResult = await ctx.client.send(countQuery)
    // Value to increase by
    const by = 1
    // What the count should be at the end
    const projectedCount = initialCount + by
    // Do the update
    const mutation = await ctx.client.send(`
      mutation {
        updateCount(by: ${by})
      }
    `)
    // The result should be the projected
    expect(mutation).toHaveProperty('updateCount', projectedCount)
    // Making sure the end count should be the projected end count
    expect(await ctx.client.send(countQuery)).toHaveProperty('count', projectedCount)
  })
})
