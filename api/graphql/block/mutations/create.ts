import { schema } from 'nexus'
import { Block } from '../Block'
import { getPersonId } from '../../../utils/auth/getPersonId'

/** Creates a block */
schema.extendType({
  type: 'Mutation',
  definition(t) {
    t.field('createBlock', {
      type: Block,
      args: {
        data: schema.stringArg(),
      },
      async resolve(_root, {data}, ctx) {
        const personId = getPersonId(ctx.token)
        const block = await ctx.db.block.create({
          data: {
            data: data ?? '',
            owner: {
              connect: {
                id: personId,
              },
            },
          },
        })
        return block
      },
    })
  },
})
