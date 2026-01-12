import type { HealthRes } from "@/category/health/types";

import { request } from "./client";

export const getHealth = async (): Promise<HealthRes> => {
  return request<HealthRes>("/api/healthz");
};
