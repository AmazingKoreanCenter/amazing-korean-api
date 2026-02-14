import { useEffect } from "react";
import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { TranslationListReq, ContentType, SupportedLanguage } from "@/category/admin/translation/types";
import {
  getAdminTranslations,
  getAdminTranslation,
  getContentRecords,
  getSourceFields,
  searchTranslations,
  getTranslationStats,
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

export const useTranslationList = (params: TranslationListReq) => {
  const query = useQuery({
    queryKey: ["admin", "translations", params],
    queryFn: () => getAdminTranslations(params),
    placeholderData: keepPreviousData,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};

export const useTranslationDetail = (id: number | undefined) => {
  const query = useQuery({
    queryKey: ["admin", "translation", id],
    queryFn: () => getAdminTranslation(id!),
    enabled: typeof id === "number" && Number.isFinite(id),
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};

export const useContentRecords = (contentType: ContentType | undefined) => {
  const query = useQuery({
    queryKey: ["admin", "translations", "content-records", contentType],
    queryFn: () => getContentRecords(contentType!),
    enabled: !!contentType,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};

export const useSourceFields = (contentType: ContentType | undefined, contentId: number | undefined) => {
  const query = useQuery({
    queryKey: ["admin", "translations", "source-fields", contentType, contentId],
    queryFn: () => getSourceFields(contentType!, contentId!),
    enabled: !!contentType && typeof contentId === "number" && Number.isFinite(contentId),
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};

export const useSearchTranslations = (lang?: SupportedLanguage) => {
  const query = useQuery({
    queryKey: ["admin", "translations", "search", lang],
    queryFn: () => searchTranslations(lang),
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};

export const useTranslationStats = () => {
  const query = useQuery({
    queryKey: ["admin", "translations", "stats"],
    queryFn: () => getTranslationStats(),
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
