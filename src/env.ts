import { defineEnvVars } from '@sveltejs/kit/env';

export const variables = defineEnvVars({
	DATABASE_HOST: { description: 'The host of the database.' },
  DATABASE_NAME: { description: 'The name of the database to connect to.' },
  DATABASE_USER: { description: 'The name of the user to connect with.' },
  DATABASE_PASSWORD: { description: 'The password of the user to connect with.' },
  DATABASE_PORT: { description: 'The port of the connection.' }
});
