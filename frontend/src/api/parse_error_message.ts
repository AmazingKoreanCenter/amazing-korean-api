/**
 * Axios error response body → 사용자에게 표시할 메시지 추출.
 *
 * 백엔드 envelope = `{ error: { message } }` 우선, 그 다음 `{ message }`,
 * 그 다음 status-code fallback.
 */
export function parseErrorMessage(data: unknown, status?: number): string {
  const fallback = status
    ? `Request failed with status ${status}`
    : "Request failed";

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
}
