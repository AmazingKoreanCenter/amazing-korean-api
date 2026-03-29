import { useQueries, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";

import { fetchPageImage, fetchPageTile } from "../ebook_api";

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
  sessionId?: string,
  hmacSecret?: string,
) => {
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: ["ebook", "page", code, page],
    queryFn: () => fetchPageImage(code, page, sessionId, hmacSecret),
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
          queryFn: () => fetchPageImage(code, adjacentPage, sessionId, hmacSecret),
          staleTime: 5 * 60 * 1000,
        });
      }
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [code, page, totalPages, enabled, viewMode, sessionId, hmacSecret]);

  return query;
};

/**
 * 타일 분할 모드: 한 페이지의 모든 타일(gridRows × gridCols)을 병렬 fetch.
 * 프리페치 범위: ±2 페이지 (타일 수가 많으므로 축소)
 */
export const usePageTiles = (
  code: string,
  page: number,
  totalPages: number,
  gridRows: number,
  gridCols: number,
  enabled = true,
  sessionId?: string,
  hmacSecret?: string,
) => {
  const queryClient = useQueryClient();

  // 모든 타일 좌표 생성
  const tileCoords: Array<{ row: number; col: number }> = [];
  for (let r = 0; r < gridRows; r++) {
    for (let c = 0; c < gridCols; c++) {
      tileCoords.push({ row: r, col: c });
    }
  }

  const results = useQueries({
    queries: tileCoords.map(({ row, col }) => ({
      queryKey: ["ebook", "tile", code, page, row, col],
      queryFn: () => fetchPageTile(code, page, row, col, sessionId, hmacSecret),
      enabled: enabled && !!code && page > 0,
      staleTime: 5 * 60 * 1000,
      gcTime: 10 * 60 * 1000,
    })),
  });

  // 인접 페이지 프리페치 (±2)
  useEffect(() => {
    if (!code || !enabled) return;

    for (const offset of [1, -1, 2, -2]) {
      const adjPage = page + offset;
      if (adjPage > 0 && adjPage <= totalPages) {
        for (const { row, col } of tileCoords) {
          queryClient.prefetchQuery({
            queryKey: ["ebook", "tile", code, adjPage, row, col],
            queryFn: () => fetchPageTile(code, adjPage, row, col, sessionId, hmacSecret),
            staleTime: 5 * 60 * 1000,
          });
        }
      }
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [code, page, totalPages, enabled, gridRows, gridCols, sessionId, hmacSecret]);

  const isLoading = results.some((r) => r.isLoading);
  const tiles = results.map((r) => r.data);

  return { tiles, isLoading, tileCoords };
};
