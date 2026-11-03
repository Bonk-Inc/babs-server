import type { PageServerLoad } from './$types';
import { redirect } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals }) => {
  const session = locals.session;

  if (session) {
    redirect(307, '/dashboard');
  }

  return {};
}