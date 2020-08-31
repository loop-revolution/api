import { AuthenticationError } from '../errors/auth'

/**
 * A function for extracting a person id from a token.
 * @param token This is the token given to the resolver.
 */
export const getPersonId = (token: any | null | undefined) => {
  /** The id of the person logged in */
  const personId = token.userId as number
  if (!personId) {
    throw new AuthenticationError(`There was no userId in token`)
  }

  return personId
}
