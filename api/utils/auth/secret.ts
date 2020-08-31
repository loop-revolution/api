/** For signing JWTs */
export const appSecret = process.env.APP_SECRET as string

if (!appSecret || appSecret.length < 30) {
  throw new Error('You must provide an APP_SECRET environment variable to set up JWT.')
}
