import { drizzle } from 'drizzle-orm/node-postgres';
import { DATABASE_HOST, DATABASE_NAME, DATABASE_PASSWORD, DATABASE_PORT, DATABASE_USER } from '$app/env/private';

const invalidConfig = !DATABASE_HOST || !DATABASE_NAME || !DATABASE_PASSWORD || !DATABASE_PORT || !DATABASE_USER;
if (invalidConfig)
  throw new Error('Database config is not set');

export const db = drizzle(`postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}`);
