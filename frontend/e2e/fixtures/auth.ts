import type { BrowserContext, APIRequestContext } from "@playwright/test";

export interface TestLogin {
  user_id: number;
  access_token: string;
}

// Default account is auto-created once via /users signup when backend runs
// with EMAIL_PROVIDER=none (auto email verification). Override via env if needed.
export const TEST_EMAIL = process.env.E2E_TEST_EMAIL ?? "e2e_p10c@amazingkorean.net";
export const TEST_PASSWORD = process.env.E2E_TEST_PASSWORD ?? "password123!";

// 백엔드에 직접 로그인 POST → 세션 쿠키(ak_refresh) 가 context 쿠키 jar 에 저장되고,
// access_token 은 반환받아 페이지 진입 시 zustand persist 스키마로 localStorage 주입한다.
export async function apiLogin(request: APIRequestContext): Promise<TestLogin> {
  const res = await request.post("/api/auth/login", {
    data: { email: TEST_EMAIL, password: TEST_PASSWORD },
  });
  if (!res.ok()) {
    throw new Error(
      `E2E login failed: ${res.status()} ${res.statusText()} ${await res.text()}`,
    );
  }
  const body = (await res.json()) as {
    user_id: number;
    access: { access_token: string };
  };
  return { user_id: body.user_id, access_token: body.access.access_token };
}

// zustand persist 미들웨어가 사용하는 키 형태로 auth-storage 를 미리 심어둔다.
// 이렇게 하면 `/studies/writing/*` PrivateRoute 가 user 존재를 확인하고 통과시킨다.
export async function seedAuthStorage(context: BrowserContext, login: TestLogin) {
  await context.addInitScript(
    ({ userId, accessToken }) => {
      const persisted = {
        state: {
          user: { user_id: userId },
          accessToken,
          isLoggedIn: true,
        },
        version: 0,
      };
      window.localStorage.setItem("auth-storage", JSON.stringify(persisted));
    },
    { userId: login.user_id, accessToken: login.access_token },
  );
}
