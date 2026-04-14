import { expect, test } from "@playwright/test";

import { apiLogin, seedAuthStorage, type TestLogin } from "./fixtures/auth";

// P10-C: 한글 자판 연습 자유 연습 플로우 E2E
// 커버 범위: 레벨 선택 → 유형 선택 → 자유 연습 시드 로드 → 타이핑 → 세션 완료 → 통계 반영 확인
//
// 전제:
//   - backend + vite dev 가 실행 중 (README 의 E2E 가이드 참조)
//   - EMAIL_PROVIDER=none 로 backend 를 띄운 뒤 e2e_p10c@amazingkorean.net 계정이
//     사전에 생성되어 있어야 함. 자세한 셋업은 frontend/e2e/README 참조.

let login: TestLogin;

test.beforeAll(async ({ playwright }) => {
  const baseURL = process.env.E2E_BASE_URL ?? "http://localhost:5173";
  const request = await playwright.request.newContext({ baseURL });
  login = await apiLogin(request);
  await request.dispose();
});

test.describe("writing practice — beginner / jamo free practice", () => {
  test("로그인 후 레벨·유형 선택 → 자유 연습 1회 완료 → 통계 total_sessions 증가", async ({
    page,
    context,
    request,
  }) => {
    await seedAuthStorage(context, login);

    // 0) 시작 전 통계 스냅샷 — GET /studies/writing/stats 를 직접 때려 기준선 확보.
    //    PATCH 완료 후 값이 +1 되는지 검증.
    const statsBefore = await request.get("/api/studies/writing/stats?days=1", {
      headers: { Authorization: `Bearer ${login.access_token}` },
    });
    expect(statsBefore.ok()).toBeTruthy();
    const beforeJson = (await statsBefore.json()) as { total_sessions: number };
    const totalBefore = beforeJson.total_sessions;

    // 1) 레벨 선택 페이지 (기본 i18n 로케일 = ko)
    await page.goto("/studies/writing");
    await expect(
      page.getByRole("heading", { name: "한글 자판 연습" }),
    ).toBeVisible();

    // 2) 초급 시작 — 레벨 카드 3개 중 첫 번째가 초급
    await page.getByRole("link", { name: "연습 시작" }).first().click();
    await expect(page).toHaveURL(/\/studies\/writing\/beginner$/);

    // 3) 유형 선택: 자모 (jamo). 이 네비게이션과 동시에 WritingTask 가 마운트되면서
    //    POST /studies/writing/sessions 가 1회 발사된다. 세션 시작 응답이 와야 finishBtn
    //    의 disabled 가 풀리므로 응답을 명시적으로 대기한다.
    const sessionStart = page.waitForResponse(
      (r) =>
        /\/studies\/writing\/sessions\b/.test(r.url()) &&
        r.request().method() === "POST" &&
        r.ok(),
    );
    await page.getByRole("link", { name: "자모" }).click();
    await expect(page).toHaveURL(/\/studies\/writing\/beginner\/jamo$/);
    await sessionStart;

    // 4) 결과 확인 버튼 DOM 출현 대기
    const finishBtn = page.getByRole("button", { name: "결과 확인" });
    await expect(finishBtn).toBeVisible();

    // 5) prompt 추출 — "따라 쓸 문장" 라벨 바로 아래 p.text-2xl.
    //    초급은 char 단위 span 으로 쪼개지므로 innerText 로 통합 문자열을 얻는다.
    const promptContainer = page.locator("p.text-2xl").first();
    await expect(promptContainer).toBeVisible();
    const promptText = (await promptContainer.innerText()).trim();
    expect(promptText.length).toBeGreaterThan(0);

    // 6) 타이핑 — pressSequentially 는 각 문자에 대해 keydown/keypress/input/keyup 을 발사해
    //    React controlled input 의 onChange 를 자연스럽게 트리거한다.
    const textarea = page.getByPlaceholder("여기에 한글로 타이핑하세요");
    await textarea.focus();
    await textarea.pressSequentially(promptText, { delay: 30 });
    // duration_ms = now() - firstInputAt. 첫 input 이후 최소 몇 ms 가 필요 (CPM 계산용).
    await page.waitForTimeout(400);

    // 7) 세션 완료 — PATCH /studies/writing/sessions/:id
    await expect(finishBtn).toBeEnabled();
    await finishBtn.click();

    // 8) 결과 카드 확인 — 제목 + stat 라벨
    await expect(page.getByRole("heading", { name: "연습 결과" })).toBeVisible();
    await expect(page.getByText("정확도").first()).toBeVisible();
    await expect(page.getByText("분당 타수").first()).toBeVisible();

    // 9) "다음 문제" 또는 "마지막 결과 확인" 버튼으로 교체됨
    await expect(
      page.getByRole("button", { name: /다음 문제|마지막 결과 확인/ }),
    ).toBeVisible();

    // 10) 통계 페이지 → 헤딩 렌더 확인
    await page.goto("/studies/writing/stats");
    await expect(
      page.getByRole("heading", { name: "자판 연습 통계" }),
    ).toBeVisible();

    // 서버 통계도 직접 한 번 더 확인 (UI 렌더와 분리).
    const statsAfter = await request.get("/api/studies/writing/stats?days=1", {
      headers: { Authorization: `Bearer ${login.access_token}` },
    });
    expect(statsAfter.ok()).toBeTruthy();
    const afterJson = (await statsAfter.json()) as { total_sessions: number };
    expect(afterJson.total_sessions).toBeGreaterThanOrEqual(totalBefore + 1);
  });
});
