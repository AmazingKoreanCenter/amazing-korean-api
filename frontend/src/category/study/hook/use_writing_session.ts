import { keepPreviousData, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type {
  FinishWritingSessionReq,
  StartWritingSessionReq,
  WritingSessionListReq,
} from "@/category/study/types";

import {
  finishWritingSession,
  listWritingSessions,
  startWritingSession,
} from "../study_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

/**
 * 한글 자판 연습 세션 시작 (POST /studies/writing/sessions)
 * 성공 시 session_id 를 포함한 세션 전체를 반환한다.
 */
export const useStartWritingSession = () => {
  return useMutation({
    mutationFn: (body: StartWritingSessionReq) => startWritingSession(body),
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};

/**
 * 한글 자판 연습 세션 완료 (PATCH /studies/writing/sessions/{id})
 * 클라이언트가 측정한 total_chars/correct_chars/duration_ms/mistakes 를 제출하면
 * 서버가 accuracy_rate 와 chars_per_minute 를 계산해서 반환한다.
 */
export const useFinishWritingSession = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      sessionId,
      body,
    }: {
      sessionId: number;
      body: FinishWritingSessionReq;
    }) => finishWritingSession(sessionId, body),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["writing-sessions"] });
      queryClient.invalidateQueries({ queryKey: ["writing-stats"] });
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};

/**
 * 내 한글 자판 연습 세션 목록 조회 (GET /studies/writing/sessions)
 */
export const useWritingSessionList = (params: WritingSessionListReq = {}) => {
  const query = useQuery({
    queryKey: ["writing-sessions", params],
    queryFn: () => listWritingSessions(params),
    placeholderData: keepPreviousData,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
