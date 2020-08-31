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
      resolve(_root, { data }, ctx) {
        const personId = getPersonId(ctx.token)
        return ctx.db.block.create({
          data: {
            data: data ?? '',
            owner: {
              connect: {
                id: personId,
              },
            },
          },
        })
      },
    })
  },
})
