import { z } from 'zod';

export const gameFormSchema = z.object({
  name: z.string().nonempty('Naam is verplicht').max(50, 'Naam mag niet langer zijn dan 50 tekens')
});

export type GameFormSchema = typeof gameFormSchema;