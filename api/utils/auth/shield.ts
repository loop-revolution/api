import { rule, not, shield } from 'nexus-plugin-shield'
import { getPersonId } from './getPersonId'
import { isExpected } from '../errors/isExpected'
import { init, captureException } from '@sentry/node'

// Error reporting
const dsn = process.env.SENTRY_DSN
if (dsn) {
  init({ dsn: process.env.SENTRY_DSN })
}

/** Makes sure that a person id exists */
const isAuthenticated = rule({ cache: 'contextual' })(async (_parent, _args, ctx) => {
  const userId = getPersonId(ctx.token)
  return Boolean(userId)
})

/** The rules to block off query paths */
export const permissions = shield({
  rules: {
    Query: {
      blocks: isAuthenticated,
    },
    Mutation: {
      createBlock: isAuthenticated,
      updateBlockData: isAuthenticated,
      signup: not(isAuthenticated),
      login: not(isAuthenticated),
    },
  },
  options: {
    fallbackError: (thrown) => {
      // If it's unexpected, report it to Sentry
      const err = thrown as Error
      if (isExpected(err)) {
        return err
      }
      captureException(err)
      return err
    },
  },
})
