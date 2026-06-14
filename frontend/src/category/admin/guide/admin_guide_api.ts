import { request } from "@/api/client";

import type {
  AdminGuideDetail,
  AdminGuideListRes,
  GuideBlockUpdate,
  GuideMetaUpdate,
  StaleDashboard,
} from "./types";

export const adminListGuides = () => request<AdminGuideListRes>("/admin/guides");

export const adminGetGuide = (guideIdx: string) =>
  request<AdminGuideDetail>(`/admin/guides/${encodeURIComponent(guideIdx)}`);

export const adminUpdateGuideMeta = (guideIdx: string, body: GuideMetaUpdate) =>
  request<{ ok: boolean; message: string }>(`/admin/guides/${encodeURIComponent(guideIdx)}`, {
    method: "PATCH",
    data: body,
  });

export const adminUpdateGuideBlock = (blockId: number, body: GuideBlockUpdate) =>
  request<{ ok: boolean; message: string }>(`/admin/guides/blocks/${blockId}`, {
    method: "PATCH",
    data: body,
  });

export const adminGuideStale = (lang?: string) =>
  request<StaleDashboard>("/admin/guides/stale", {
    params: lang ? { lang } : undefined,
  });

/** 디프 export — 브라우저 다운로드용 raw JSON */
export const adminGuideDiffExport = (lang: string) =>
  request<unknown>("/admin/guides/diff-export", { params: { lang } });
