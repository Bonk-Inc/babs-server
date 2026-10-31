import type { Handle } from "@sveltejs/kit/hooks";
import { building } from "$app/env";
import { auth } from "#lib/auth.ts";
import { svelteKitHandler } from "better-auth/svelte-kit";
import { redirect } from "@sveltejs/kit";

const handleBetterAuth: Handle = async ({ event, resolve }) => {
  // If no auth check is required, continue the routing
  if (!event.route.id?.startsWith("/(app)/")) {
    return svelteKitHandler({ event, resolve, auth, building });
  }

  // Get the current user based on the routing headers.
  const session = await auth.api.getSession({
    headers: event.request.headers,
  });

  // If a valid user is found, continue the routing
  if (session) {
    event.locals.session = session.session;
    event.locals.user = session.user;

    return svelteKitHandler({ event, resolve, auth, building });
  }
  
  // Redirect the user to the sign-in page if not authenticated.
  redirect(307, "/sign-in");
};

export const handle: Handle = handleBetterAuth;
