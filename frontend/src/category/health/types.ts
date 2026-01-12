import { z } from "zod";

export const healthResSchema = z.object({
  status: z.string(),
  uptime_ms: z.number().int().nonnegative(),
  version: z.string(),
});

export type HealthRes = z.infer<typeof healthResSchema>;

export const readyResSchema = z.object({
  ready: z.boolean(),
  reason: z.string().optional(),
});

export type ReadyRes = z.infer<typeof readyResSchema>;
