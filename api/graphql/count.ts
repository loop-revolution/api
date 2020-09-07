// This is for setting up connections with GraphQL
import { schema } from 'nexus'

const data = {
  count: 1,
}

/** Returns a list of blocks */
schema.extendType({
  type: 'Query',
  definition(t) {
    t.field('count', {
      type: 'Int',
      resolve(_root, _args, ctx) {
        return data.count
      },
    })
  },
})

schema.extendType({
  type: 'Mutation',
  definition(t) {
    t.field('updateCount', {
      type: 'Int',
      args: {
        by: schema.intArg({ required: true }),
      },
      resolve(_root, { by }) {
        data.count += by
        return data.count
      },
    })
  },
})
