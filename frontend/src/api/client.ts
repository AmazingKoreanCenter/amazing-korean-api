import axios, { type AxiosRequestConfig } from "axios";

import type { LoginRes } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? "/api";

type RequestOptions = Omit<AxiosRequestConfig, "url">;

type RetryableRequestConfig = AxiosRequestConfig & {
  _retry?: boolean;
  skipAuthRefresh?: boolean;
};

export class ApiError extends Error {
  status: number;

  constructor(status: number, message: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
  }
}

const parseErrorMessage = (data: unknown, status?: number) => {
  const fallback = status ? `Request failed with status ${status}` : "Request failed";

  if (!data) {
    return fallback;
  }

  if (typeof data === "string") {
    try {
      const parsed = JSON.parse(data) as {
        error?: { message?: string };
        message?: string;
      };

      if (typeof parsed?.error?.message === "string" && parsed.error.message) {
        return parsed.error.message;
      }

      if (typeof parsed?.message === "string" && parsed.message) {
        return parsed.message;
      }
    } catch {
      return data;
    }

    return data || fallback;
  }

  if (typeof data === "object") {
    const parsed = data as {
      error?: { message?: string };
      message?: string;
    };

    if (typeof parsed?.error?.message === "string" && parsed.error.message) {
      return parsed.error.message;
    }

    if (typeof parsed?.message === "string" && parsed.message) {
      return parsed.message;
    }
  }

  return fallback;
};

const applyAuthorizationHeader = (
  headers: RetryableRequestConfig["headers"],
  token: string
) => {
  if (!headers) {
    return { Authorization: token };
  }

  if (typeof (headers as { set?: (key: string, value: string) => void }).set === "function") {
    (headers as { set: (key: string, value: string) => void }).set(
      "Authorization",
      token
    );
    return headers;
  }

  return { ...(headers as Record<string, string>), Authorization: token };
};

export const api = axios.create({
  baseURL: API_BASE_URL,
  withCredentials: true,
});

api.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config as RetryableRequestConfig | undefined;

    if (
      error.response?.status === 401 &&
      originalRequest &&
      !originalRequest._retry &&
      !originalRequest.skipAuthRefresh
    ) {
      originalRequest._retry = true;
      try {
        // [수정 1] body에 undefined 대신 빈 객체 {} 전달 (415 에러 방지)
        // [수정 2] config 객체를 'RetryableRequestConfig'로 캐스팅 (skipAuthRefresh 에러 해결)
        const refreshResponse = await api.post(
          "/auth/refresh",
          {}, 
          {
            skipAuthRefresh: true,
          } as RetryableRequestConfig
        );

        // [수정 3] 응답 데이터를 LoginRes로 명시적 타입 단언 (T 타입 에러 해결)
        const loginData = refreshResponse.data as LoginRes;
        const newToken = `Bearer ${loginData.access.access_token}`;

        api.defaults.headers.common["Authorization"] = newToken;
        
        // originalRequest.headers가 존재하지 않을 수 있으므로 안전하게 처리
        originalRequest.headers = applyAuthorizationHeader(
            originalRequest.headers || {}, 
            newToken
        );

        useAuthStore.getState().login(loginData);

        return api(originalRequest);
      } catch (refreshError) {
        useAuthStore.getState().logout();
        window.location.href = "/login";
        return Promise.reject(refreshError);
      }
    }

    return Promise.reject(error);
  }
);

export async function request<T>(
  path: string,
  options: RequestOptions = {}
): Promise<T> {
  try {
    const response = await api.request<T>({
      url: path,
      ...options,
    });

    if (response.status === 204 || response.data === "") {
      return undefined as T;
    }

    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      const status = error.response?.status ?? 0;
      const message =
        parseErrorMessage(error.response?.data, status) || error.message;
      throw new ApiError(status, message);
    }

    throw error;
  }
}
