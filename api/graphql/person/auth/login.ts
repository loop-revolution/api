import { appSecret } from '../../../utils/auth/secret';
import { authenticatePerson } from '../../../utils/auth/password';
import { AuthPayload } from './AuthPayload';
import { makeReferenceName } from '../../../utils/person/name';
import { schema } from 'nexus';
import { sign } from 'jsonwebtoken';

/**
 * To log in to an account.
 * Returns the person data & a token for auth.
 */
schema.extendType({
  type: 'Mutation',
  definition(t) {
    t.field('login', {
      type: AuthPayload,
      args: {
        name: schema.stringArg({ nullable: false }),
        password: schema.stringArg({ nullable: false }),
      },
      resolve: async (_parent, { name, password }, ctx) => {
        const refName = makeReferenceName(name)
        /** Validates the person with password */
        const person = await authenticatePerson({
          refName,
          password,
          db: ctx.db,
        })
        return {
          token: sign({ userId: person.id }, appSecret, { expiresIn: '150d' }),
          person,
        }
      },
    })
  },
})
