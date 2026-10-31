import { getRequestEvent } from '$app/server';
import { OAUTH_CLIENT_ID, OAUTH_CLIENT_SECRET, OAUTH_PROVIDER } from '$app/env/private';
import { betterAuth } from 'better-auth';
import { drizzleAdapter } from 'better-auth/adapters/drizzle';
import { genericOAuth } from 'better-auth/plugins';
import { sveltekitCookies } from 'better-auth/svelte-kit';
import { db } from '../db';
import * as schema from '../db/schema';

export const auth = betterAuth({
  database: drizzleAdapter(db, { 
    provider: 'pg',
    schema
  }),
  emailAndPassword: {
    enabled: false
  },
  plugins: [
    sveltekitCookies(getRequestEvent),
    genericOAuth({
      config: [
        {
          providerId: 'bonk-sso',
          clientId: OAUTH_CLIENT_ID,
          clientSecret: OAUTH_CLIENT_SECRET,
          discoveryUrl: `${OAUTH_PROVIDER}/.well-known/openid-configuration`,
          disableSignUp: false,
          scopes: ['profile', 'openid', 'email', 'offline_access']
        }
      ]
    })
  ],
  user: { modelName: 'users' },
  account: { modelName: 'accounts' },
  session: { modelName: 'sessions' },
  verification: { modelName: 'verifications' }
});