import { db } from "#lib/server/db/index.ts";
import { games } from "#lib/server/db/schema/games.schema.ts";
import { eq } from "drizzle-orm";
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ params }) => {
  const gameId = parseInt(params.id);
  const game = await db.query.games.findFirst({
    where: eq(games.id, gameId)
  });
  
  if (!game) {
    error(404, "Game not found");
  }

  return { game }
};