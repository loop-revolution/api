/** A type of error for when a person isn't or is improperly logged in */
export class AuthenticationError extends Error {
  constructor(message?: string) {
    super(message)
    this.name = 'AuthenticationError'
  }
}

/** A type of error for when a person doesn't have access to something */
export class AuthorizationError extends Error {
  constructor(message?: string) {
    super(message)
    this.name = 'AuthorizationError'
  }
}
