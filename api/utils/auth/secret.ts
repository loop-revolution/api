/** For signing JWTs */
export const appSecret = process.env.APP_SECRET as string

const minSecretLength = 30
if (!appSecret || appSecret.length < minSecretLength) {
  throw new Error('You must provide an APP_SECRET environment variable to set up JWT.')
}

