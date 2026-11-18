import { superValidate } from "sveltekit-superforms";
import type { PageServerLoad } from "./$types";
import { zod4 } from "sveltekit-superforms/adapters";
import { gameFormSchema } from '../../schema';

export const load: PageServerLoad = async ({ parent }) => {
  const { game } = await parent();
  const form = await superValidate(game, zod4(gameFormSchema));

  return { form };
}