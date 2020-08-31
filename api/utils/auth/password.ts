import SecurePassword from 'secure-password'
import { PrismaClient } from '@prisma/client'
import { AuthenticationError } from '../errors/auth'

export const MinimumPasswordLength = 8
export const MaximumPasswordLength = 100

const SP = new SecurePassword()

/** Turns a raw string password into a hashed version to store in db */
export const hashPassword = async (password: string) => {
  const hashedBuffer = await SP.hash(Buffer.from(password))
  return hashedBuffer.toString('base64')
}

export const verifyPassword = async ({ hashedPassword, password }: { hashedPassword: string; password: string }) =>
  SP.verify(Buffer.from(password), Buffer.from(hashedPassword, 'base64'))

/** Authenticate whether or not a person's name and password match */
export const authenticatePerson = async ({
  refName,
  password,
  db,
}: {
  refName: string
  password: string
  db: PrismaClient
}) => {
  const person = await db.person.findOne({ where: { refName } })
  if (!person || !person.hashedPassword) {
    throw new AuthenticationError()
  }
  switch (await verifyPassword({ hashedPassword: person.hashedPassword, password })) {
    case SecurePassword.VALID:
      break
    case SecurePassword.VALID_NEEDS_REHASH:
      // Upgrade hashed password with a more secure hash
      const improvedHash = await hashPassword(password)
      await db.person.update({ where: { id: person.id }, data: { hashedPassword: improvedHash } })
      break
    default:
      throw new AuthenticationError()
  }
  return person
}
