import { appSecret } from './utils/auth/secret'
import { auth } from 'nexus-plugin-jwt-auth'
import { permissions } from './utils/auth/shield'
import { prisma } from 'nexus-plugin-prisma'
import { use, settings } from 'nexus'

// Plugins
use(
  prisma({
    features: {
      crud: true,
    },
  }),
)
use(
  auth({
    appSecret,
  }),
)
use(permissions)

settings.change({
  server: {
    playground: true,
    path: '/',
    graphql: {
      introspection: true,
    },
    cors: {
      origin: (origin, callback) => callback(null, callback),
      methods: 'GET,HEAD,PUT,PATCH,POST,DELETE',
      preflightContinue: false,
      optionsSuccessStatus: 204,
    },
  },
})
