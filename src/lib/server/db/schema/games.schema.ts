import { pgTable, serial, text, timestamp, varchar } from "drizzle-orm/pg-core";
import { users } from "./auth.schema";
import { relations } from "drizzle-orm";

export const games = pgTable('games', {
  id: serial('id').primaryKey(),
  name: varchar({ length: 50 }).notNull(),
  created_at: timestamp({ withTimezone: true }).defaultNow(),
  created_by: text('created_by')
    .notNull()
    .references(() => users.id, { onDelete: "cascade" }),
  updated_at: timestamp({ withTimezone: true }),
  updated_by: text('updated_by')
    .references(() => users.id, { onDelete: "cascade" }),
});

export const gamesRelations = relations(games, ({ one }) => ({
  user: one(users, {
    fields: [games.id],
    references: [users.id]
  })
}));