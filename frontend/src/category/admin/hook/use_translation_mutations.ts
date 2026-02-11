import { useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type {
  TranslationCreateReq,
  TranslationUpdateReq,
  TranslationStatusUpdateReq,
  AutoTranslateReq,
} from "@/category/admin/translation/types";
import {
  createAdminTranslation,
  updateAdminTranslation,
  updateAdminTranslationStatus,
  deleteAdminTranslation,
  autoTranslateContent,
} from "../admin_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useCreateTranslation = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: TranslationCreateReq) => createAdminTranslation(data),
    onSuccess: () => {
      toast.success("번역이 생성되었습니다.");
      queryClient.invalidateQueries({ queryKey: ["admin", "translations"] });
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};

export const useUpdateTranslation = (id: number) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: TranslationUpdateReq) => updateAdminTranslation(id, data),
    onSuccess: () => {
      toast.success("번역이 수정되었습니다.");
      queryClient.invalidateQueries({ queryKey: ["admin", "translations"] });
      queryClient.invalidateQueries({ queryKey: ["admin", "translation", id] });
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};

export const useUpdateTranslationStatus = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: TranslationStatusUpdateReq }) =>
      updateAdminTranslationStatus(id, data),
    onSuccess: () => {
      toast.success("상태가 변경되었습니다.");
      queryClient.invalidateQueries({ queryKey: ["admin", "translations"] });
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};

export const useDeleteTranslation = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: number) => deleteAdminTranslation(id),
    onSuccess: () => {
      toast.success("번역이 삭제되었습니다.");
      queryClient.invalidateQueries({ queryKey: ["admin", "translations"] });
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};

export const useAutoTranslate = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: AutoTranslateReq) => autoTranslateContent(data),
    onSuccess: (res) => {
      toast.success(`자동 번역 완료: ${res.success_count}/${res.total} 성공`);
      queryClient.invalidateQueries({ queryKey: ["admin", "translations"] });
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};
