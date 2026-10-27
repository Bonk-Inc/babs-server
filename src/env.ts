import { defineEnvVars } from '@sveltejs/kit/env';

export const variables = defineEnvVars({
	DATABASE_HOST: { description: 'The host of the database.' },
  DATABASE_NAME: { description: 'The name of the database to connect to.' },
  DATABASE_USER: { description: 'The name of the user to connect with.' },
  DATABASE_PASSWORD: { description: 'The password of the user to connect with.' },
  DATABASE_PORT: { description: 'The port of the connection.' },
  ORIGIN: {
    description: 'The app origin (base URL), e.g. `http://localhost:5173`.'
  },
  BETTER_AUTH_SECRET: {
    description: 'Secret used to sign tokens. For production use 32 characters generated with high entropy. See [Better Auth installation](https://www.better-auth.com/docs/installation).'
  },
  OAUTH_CLIENT_ID: { description: 'Client ID of the OAuth provider.' },
  OAUTH_CLIENT_SECRET: { description: 'Client Secret of the OAuth provivder.' },
  OAUTH_PROVIDER: { description: 'Url to the OAuth provider.' }
});
