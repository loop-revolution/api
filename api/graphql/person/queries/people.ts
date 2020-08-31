import { schema } from 'nexus'

/** Returns a list of people */
schema.extendType({
  type: 'Query',
  definition(t) {
    t.crud.people()
  },
})
