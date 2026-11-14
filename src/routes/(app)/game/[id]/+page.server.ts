import { db } from "#lib/server/db/index.ts";
import { games } from "#lib/server/db/schema/games.schema.ts";
import { eq } from "drizzle-orm";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
  const gameId = parseInt(params.id);
  const game = await db.query.games.findFirst({
    where: eq(games.id, gameId)
  });
  
  return { game }
};