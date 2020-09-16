import del from 'del'
import { PrismaClient } from '@prisma/client'

/**
 * Resets migrations locally and in the server
 */
export const resetMigrations = async () => {
  console.log('🍁 Cleaning migrations')
  // Clear local migrations
  await del('./prisma/migrations')
  // Clear migrations on server
  const prisma = new PrismaClient()
  await prisma.$executeRaw('TRUNCATE "_Migration";')
  await prisma.$disconnect()
}

resetMigrations()
  .then(() => {
    console.log('✨ Migrations reset!')
    process.exit(0)
  })
  .catch((error) => {
    console.log(`💥 Migrations didn't exist`)
    // Will error if DB hasn't been made yet, but that's fine
    process.exit(0)
  })
