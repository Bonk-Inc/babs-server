import { GameDTO } from "#lib/dtos/GameDTO";
import type { Transport } from "@sveltejs/kit/hooks";

export const transport: Transport = {
  GameDTO: {
    encode: (value) => value instanceof GameDTO && [value.id, value.name],
    decode: ([id, name]) => new GameDTO(id, name),
  }
};