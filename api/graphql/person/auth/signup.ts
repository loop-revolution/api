import { schema } from 'nexus'
import { hashPassword, MinimumPasswordLength } from '../../../utils/auth/password'
import { InvalidPasswordError } from '../../../utils/errors/password'
import { formatName, makeReferenceName } from '../../../utils/person/name'
import { sign } from 'jsonwebtoken'
import { appSecret } from '../../../utils/auth/secret'
import { AuthPayload } from './AuthPayload'

/**
 * To log in to an account.
 * Returns the person data & a token for auth.
 */
schema.extendType({
  type: 'Mutation',
  definition(t) {
    t.field('signup', {
      type: AuthPayload,
      args: {
        name: schema.stringArg({ nullable: false }),
        password: schema.stringArg({ nullable: false }),
      },
      resolve: async (_parent, { name, password }, ctx) => {
        if (password.length < MinimumPasswordLength) {
          throw new InvalidPasswordError('The given password is too short.')
        }
        const person = await ctx.db.person.create({
          data: {
            name: formatName(name),
            refName: makeReferenceName(name),
            hashedPassword: await hashPassword(password),
          },
        })
        return {
          token: sign({ userId: person.id }, appSecret, { expiresIn: '150d' }),
          person,
        }
      },
    })
  },
})
