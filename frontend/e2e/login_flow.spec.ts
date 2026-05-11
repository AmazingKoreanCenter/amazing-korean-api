import { expect, test } from "@playwright/test";

import { TEST_EMAIL, TEST_PASSWORD } from "./fixtures/auth";

// G2 e2e 시나리오 2 (2026-05-11) — 일반 로그인 흐름.
// 커버 범위: /login 폼 입력 → useLogin mutation → onSuccess → /about 리다이렉트
//
// 전제 (writing_practice.spec.ts 와 동일):
//   - backend + vite dev 가 실행 중 (e2e/README 참조)
//   - EMAIL_PROVIDER=none 으로 backend 띄운 뒤 e2e_p10c 계정 사전 생성
//
// rate limit 주의: RATE_LIMIT_LOGIN_MAX=10 / WINDOW=900s. 본 spec 의 로그인 1회 +
// writing_practice.spec.ts 의 apiLogin 1회 = 합 2회 → 한 CI run 안에서는 안전.

test.describe("login flow — happy path", () => {
  test("이메일·패스워드 입력 → 로그인 성공 → /about 리다이렉트", async ({
    page,
  }) => {
    await page.goto("/login");

    const emailInput = page.locator('input[name="email"]');
    await expect(emailInput).toBeVisible();
    const passwordInput = page.locator('input[name="password"]');
    await emailInput.fill(TEST_EMAIL);
    await passwordInput.fill(TEST_PASSWORD);

    // MFA 모달의 submit 와 구분 = 폼 내부 첫 submit.
    await page.locator('button[type="submit"]').first().click();

    await expect(page).toHaveURL(/\/about$/);
  });
});
