import type { GameDTO } from "#lib/dtos/GameDTO.ts";
import { db } from "#lib/server/db/index.ts";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals }) => {
  const fetchedGames = await db.query.games.findMany();
  const gamesList = fetchedGames.map(game => ({ id: game.id, name: game.name }) as GameDTO);
  
  return {
    games: gamesList
  }
}