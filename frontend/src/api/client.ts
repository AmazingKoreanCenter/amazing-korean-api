const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? "";

type RequestOptions = Omit<RequestInit, "headers"> & {
  headers?: HeadersInit;
};

export class ApiError extends Error {
  status: number;

  constructor(status: number, message: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
  }
}

const parseErrorMessage = async (response: Response) => {
  const fallback = `Request failed with status ${response.status}`;
  const text = await response.text();

  if (!text) {
    return fallback;
  }

  try {
    const data = JSON.parse(text) as {
      error?: { message?: string };
      message?: string;
    };

    if (typeof data?.error?.message === "string" && data.error.message) {
      return data.error.message;
    }

    if (typeof data?.message === "string" && data.message) {
      return data.message;
    }
  } catch {
    return text;
  }

  return text || fallback;
};

export async function request<T>(
  path: string,
  options: RequestOptions = {}
): Promise<T> {
  const headers = new Headers(options.headers);
  if (!headers.has("Content-Type")) {
    headers.set("Content-Type", "application/json");
  }

  const response = await fetch(`${API_BASE_URL}${path}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    const message = await parseErrorMessage(response);
    throw new ApiError(response.status, message);
  }

  return (await response.json()) as T;
}
