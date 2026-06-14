import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import {
  adminGetGuide,
  adminGuideStale,
  adminListGuides,
  adminUpdateGuideBlock,
  adminUpdateGuideMeta,
} from "../admin_guide_api";
import type { GuideBlockUpdate, GuideMetaUpdate } from "../types";

export const useAdminGuides = () =>
  useQuery({ queryKey: ["admin", "guides"], queryFn: adminListGuides });

export const useAdminGuide = (guideIdx: string | undefined) =>
  useQuery({
    queryKey: ["admin", "guide", guideIdx],
    queryFn: () => adminGetGuide(guideIdx!),
    enabled: typeof guideIdx === "string" && guideIdx.length > 0,
  });

export const useAdminGuideStale = (lang?: string) =>
  useQuery({
    queryKey: ["admin", "guide-stale", lang ?? "all"],
    queryFn: () => adminGuideStale(lang),
  });

export const useUpdateGuideMeta = (guideIdx: string) => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: GuideMetaUpdate) => adminUpdateGuideMeta(guideIdx, body),
    onSuccess: (res) => {
      toast.success(res.message ?? "단원 정보 수정 완료");
      qc.invalidateQueries({ queryKey: ["admin", "guide", guideIdx] });
      qc.invalidateQueries({ queryKey: ["admin", "guides"] });
    },
    onError: (e: Error) => toast.error(e.message),
  });
};

export const useUpdateGuideBlock = (guideIdx: string) => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ blockId, body }: { blockId: number; body: GuideBlockUpdate }) =>
      adminUpdateGuideBlock(blockId, body),
    onSuccess: (res) => {
      toast.success(res.message ?? "블록 수정 완료");
      qc.invalidateQueries({ queryKey: ["admin", "guide", guideIdx] });
      qc.invalidateQueries({ queryKey: ["admin", "guides"] });
      qc.invalidateQueries({ queryKey: ["admin", "guide-stale"] });
    },
    onError: (e: Error) => toast.error(e.message),
  });
};
