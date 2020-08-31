import { schema } from 'nexus'
import { Person } from '../Person'

export const AuthPayload = 'AuthPayload'

schema.objectType({
  name: AuthPayload,
  definition(t) {
    t.string('token')
    t.field('person', { type: Person })
  },
})
