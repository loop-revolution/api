import { schema } from 'nexus'
import { Block } from '../Block'

/** Changes a block's data */
schema.extendType({
  type: 'Mutation',
  definition(t) {
    t.field('updateBlockData', {
      type: Block,
      args: {
        newData: schema.stringArg({ required: true }),
        blockId: schema.intArg({ required: true }),
      },
      resolve(_root, args, ctx) {
        return ctx.db.block.update({
          where: { id: args.blockId },
          data: {
            data: args.newData,
          },
        })
      },
    })
  },
})
