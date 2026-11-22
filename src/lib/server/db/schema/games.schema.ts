import { pgTable, serial, text, timestamp, varchar } from "drizzle-orm/pg-core";
import { users } from "./auth.schema";
import { relations } from "drizzle-orm";
import { levels } from "./levels.schema";

export const games = pgTable('games', {
  id: serial('id').primaryKey(),
  name: varchar({ length: 50 }).notNull(),
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

export const gamesRelations = relations(games, ({ one, many }) => ({
  user: one(users, {
    fields: [games.created_by],
    references: [users.id]
  }),
  levels: many(levels)
}));