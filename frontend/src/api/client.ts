import axios, { type AxiosRequestConfig } from "axios";

import type { LoginRes } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";
import { parseErrorMessage } from "./parse_error_message";
import { applyAuthorizationHeader } from "./apply_authorization_header";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? "/api";

type RequestOptions = Omit<AxiosRequestConfig, "url"> & {
  skipAuthRefresh?: boolean;
};

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

export const api = axios.create({
  baseURL: API_BASE_URL,
  withCredentials: true,
});

api.interceptors.request.use((config) => {
  const token = useAuthStore.getState().accessToken;
  if (token && !config.headers.Authorization) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// 진행 중인 refresh 를 공유하는 single-flight 게이트.
// 동시 401 다발이 와도 /auth/refresh 는 1번만 호출하고, 나머지 요청은
// 그 결과를 기다렸다가 새 토큰으로 재시도한다.
// (없으면 각 요청이 따로 refresh → 서버 토큰 회전 후 나머지가 옛 토큰 제시
//  → reuse 감지 → 세션 compromised → 로그아웃. 단일 탭에서도 발생.)
let refreshPromise: Promise<LoginRes> | null = null;

function refreshAccessToken(): Promise<LoginRes> {
  if (!refreshPromise) {
    refreshPromise = api
      .post(
        "/auth/refresh",
        {}, // body에 undefined 대신 빈 객체 {} 전달 (415 에러 방지)
        { skipAuthRefresh: true } as RetryableRequestConfig
      )
      .then((response) => {
        const loginData = response.data as LoginRes;
        api.defaults.headers.common["Authorization"] =
          `Bearer ${loginData.access.access_token}`;
        useAuthStore.getState().login(loginData);
        return loginData;
      })
      .finally(() => {
        // 성공/실패 무관 게이트 해제 → 다음 만료 사이클은 새 refresh 1회.
        refreshPromise = null;
      });
  }
  return refreshPromise;
}

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
        const loginData = await refreshAccessToken();
        const newToken = `Bearer ${loginData.access.access_token}`;

        // originalRequest.headers가 존재하지 않을 수 있으므로 안전하게 처리
        originalRequest.headers = applyAuthorizationHeader(
          originalRequest.headers || {},
          newToken
        );

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
