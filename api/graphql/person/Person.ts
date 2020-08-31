import { schema } from 'nexus'

export const Person = 'Person'

schema.objectType({
  name: Person,
  definition(t) {
    t.model.id()
    t.model.name()
    t.model.owned()
  },
})
