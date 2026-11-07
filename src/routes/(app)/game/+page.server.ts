import { fail } from '@sveltejs/kit';
import { message, superValidate } from 'sveltekit-superforms';
import { zod4 } from 'sveltekit-superforms/adapters';
import { gameFormSchema } from './schema';
import { db } from '#lib/server/db/index.js';
import { games } from '#lib/server/db/schema.js';
import { GameDTO } from '#lib/dtos/GameDTO.js';

export const actions = {
  create: async ({ request, locals }) => {
    const form = await superValidate(request, zod4(gameFormSchema));

    if (!form.valid) {
      // Return { form } and things will just work.
      return fail(400, { form });
    }

    // Insert new game record into the database.
    const insertedGame = await db.insert(games)
      .values({
        name: form.data.name,
        created_by: locals.user?.id,
        updated_by: locals.user?.id
      })
      .returning();

    const game = new GameDTO(insertedGame[0].id, insertedGame[0].name);

    // Return the form with a status message.
    return message(form, game);
  }
};