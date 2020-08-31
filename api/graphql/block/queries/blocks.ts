import { schema } from 'nexus'

/** Returns a list of blocks */
schema.extendType({
  type: 'Query',
  definition(t) {
    t.crud.blocks()
  },
})
