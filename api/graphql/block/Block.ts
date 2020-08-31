import { schema } from 'nexus'

export const Block = 'Block'

schema.objectType({
  name: Block,
  definition(t) {
    t.model.id()
    t.model.data()
    t.model.owner()
  },
})
