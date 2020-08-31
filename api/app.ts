import { appSecret } from './utils/auth/secret'
import { auth } from 'nexus-plugin-jwt-auth'
import { permissions } from './utils/auth/shield'
import { prisma } from 'nexus-plugin-prisma'
import { use } from 'nexus'

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