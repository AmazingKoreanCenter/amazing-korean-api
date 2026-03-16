import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";

import { fetchPageImage } from "../ebook_api";

/**
 * 페이지 이미지를 ArrayBuffer로 캐시.
 * Canvas 렌더링 시 임시 blob URL 생성 → drawImage → 즉시 revoke.
 * TanStack Query에는 ArrayBuffer만 저장되므로 blob URL 노출 없음.
 */
export const usePageImage = (
  code: string,
  page: number,
  totalPages: number,
  enabled = true,
  viewMode: "single" | "spread" = "single",
) => {
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: ["ebook", "page", code, page],
    queryFn: () => fetchPageImage(code, page),
    enabled: enabled && !!code && page > 0,
    staleTime: 5 * 60 * 1000,
    gcTime: 10 * 60 * 1000,
  });

  // 인접 페이지 프리로드 (한 쪽 보기 ±3, 두 쪽 보기 ±4)
  useEffect(() => {
    if (!code || !enabled) return;

    const offsets =
      viewMode === "spread"
        ? [1, -1, 2, -2, 3, -3, 4, -4]
        : [1, -1, 2, -2, 3, -3];

    for (const offset of offsets) {
      const adjacentPage = page + offset;
      if (adjacentPage > 0 && adjacentPage <= totalPages) {
        queryClient.prefetchQuery({
          queryKey: ["ebook", "page", code, adjacentPage],
          queryFn: () => fetchPageImage(code, adjacentPage),
          staleTime: 5 * 60 * 1000,
        });
      }
    }
  }, [code, page, totalPages, enabled, viewMode, queryClient]);

  return query;
};
