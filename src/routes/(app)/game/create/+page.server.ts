import { superValidate } from 'sveltekit-superforms';
import { zod4 } from 'sveltekit-superforms/adapters';
import { z } from 'zod';
import type { PageServerLoad } from './$types';
import { gameFormSchema } from '../schema';

export const load: PageServerLoad = async () => {
  const form = await superValidate(zod4(gameFormSchema));

  return { form };
}