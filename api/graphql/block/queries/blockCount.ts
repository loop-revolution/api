import { schema } from 'nexus'

/** Returns a list of blocks */
schema.extendType({
  type: 'Query',
  definition(t) {
    t.field('blockCount', {
      type: 'Int',
      resolve(_root, _args, ctx) {
        return ctx.db.block.count()
      }
    })
  },
})
