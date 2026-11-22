import { pgTable, serial, text, timestamp, varchar, integer } from "drizzle-orm/pg-core";
import { users } from "./auth.schema";
import { relations } from "drizzle-orm";
import { games } from "./games.schema";

export const levels = pgTable("levels", {
  id: serial('id').primaryKey(),
  name: varchar({ length: 50 }).notNull(),
  game_id: integer('game_id')
    .notNull()
    .references(() => games.id, { onDelete: "cascade" }),
  created_at: timestamp({ withTimezone: true })
    .defaultNow()
    .notNull(),
  created_by: text('created_by')
    .notNull()
    .references(() => users.id, { onDelete: "cascade" }),
  updated_at: timestamp({ withTimezone: true })
    .defaultNow()
    .notNull()
    .$onUpdate(() => new Date()),
  updated_by: text('updated_by')
    .notNull()
    .references(() => users.id, { onDelete: "cascade" }),
});

export const levelsRelations = relations(levels, ({ one }) => ({
  game: one(games, {
    fields: [levels.game_id],
    references: [games.id]
  })
}));