import { defineConfig } from 'drizzle-kit';

const { DATABASE_HOST, DATABASE_NAME, DATABASE_PASSWORD, DATABASE_PORT, DATABASE_USER } = process.env;

const invalidConfig = !DATABASE_HOST || !DATABASE_NAME || !DATABASE_PASSWORD || !DATABASE_PORT || !DATABASE_USER;
if (invalidConfig)
  throw new Error('Database config is not set');

export default defineConfig({
	schema: './src/db/schema',
	dialect: 'postgresql',
	dbCredentials: { 
    host: DATABASE_HOST,
    database: DATABASE_NAME,
    user: DATABASE_USER,
    port: DATABASE_PORT,
    password: DATABASE_PASSWORD
  },
	verbose: true,
	strict: true
});
