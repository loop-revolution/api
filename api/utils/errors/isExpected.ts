import { AuthenticationError, AuthorizationError } from './auth'
import { InvalidPasswordError } from './password'

/**
 * Returns true if the error is expected through a query.
 * @param error The error to check
 */
export const isExpected = (error: Error) => {
  switch (error.name) {
    case AuthenticationError.name:
      return true
    case AuthorizationError.name:
      return true
    case InvalidPasswordError.name:
      return true
    default:
      return false
  }
}
