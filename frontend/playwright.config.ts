import { defineConfig, devices } from "@playwright/test";

// P10-C Playwright E2E for Amazing Korean writing practice.
// 전제: 로컬에서 backend + frontend(vite dev) 가 이미 떠 있어야 한다.
//   - backend: cargo run  (default BIND_ADDR=0.0.0.0:3000 또는 3100 재정의 가능)
//   - frontend: npm run dev  (Vite, 기본 5173. /api 프록시는 VITE_PROXY_TARGET 로 조정)
//   - 시드 사용자: KKR@KKR.com / password123! (AMK dev DB 기본 계정)

const baseURL = process.env.E2E_BASE_URL ?? "http://localhost:5173";

export default defineConfig({
  testDir: "./e2e",
  outputDir: "./test-results/e2e",
  timeout: 60_000,
  expect: { timeout: 10_000 },
  fullyParallel: false,
  retries: 0,
  workers: 1,
  reporter: [["list"]],
  use: {
    baseURL,
    trace: "retain-on-failure",
    video: "retain-on-failure",
    screenshot: "only-on-failure",
    locale: "ko-KR",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
});
