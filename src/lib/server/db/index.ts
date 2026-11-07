import { drizzle } from 'drizzle-orm/node-postgres';
import { migrate } from 'drizzle-orm/node-postgres/migrator';
import { DATABASE_HOST, DATABASE_NAME, DATABASE_PASSWORD, DATABASE_PORT, DATABASE_USER } from '$app/env/private';
import * as schema from './schema';

const invalidConfig = !DATABASE_HOST || !DATABASE_NAME || !DATABASE_PASSWORD || !DATABASE_PORT || !DATABASE_USER;
if (invalidConfig)
  throw new Error('Database config is not set');

const pgUri = `postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}`;
export const db = drizzle(pgUri, { schema });

await migrate(db, {
  migrationsFolder: './drizzle',
});