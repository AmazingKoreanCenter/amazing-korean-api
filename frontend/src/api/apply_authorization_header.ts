import type { AxiosRequestConfig } from "axios";

export type AxiosLikeHeaders = AxiosRequestConfig["headers"] | null;

/**
 * 401 retry 시 originalRequest.headers 에 신규 Authorization 토큰 주입.
 *
 * axios Headers 객체 (`.set`), plain object, undefined/null 세 형태 모두 처리.
 */
export function applyAuthorizationHeader(
  headers: AxiosLikeHeaders,
  token: string,
): AxiosRequestConfig["headers"] {
  if (!headers) {
    return { Authorization: token };
  }

  const maybeSet = (headers as { set?: (key: string, value: string) => void })
    .set;
  if (typeof maybeSet === "function") {
    maybeSet.call(headers, "Authorization", token);
    return headers;
  }

  return {
    ...(headers as Record<string, string>),
    Authorization: token,
  };
}
