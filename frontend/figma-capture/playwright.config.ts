import { defineConfig, devices } from "@playwright/test";

/**
 * Figma 레퍼런스용 페이지 풀캡처 Playwright 설정
 * - Vite dev 서버 자동 기동/종료
 * - 1440×900 데스크탑 뷰포트 기준 (Figma 작업 해상도)
 * - light/dark 테마는 test 파일에서 projects로 분기
 */
export default defineConfig({
  testDir: "./tests",
  timeout: 90_000,
  fullyParallel: false, // 같은 페이지를 테마별로 덮어쓰지 않도록 직렬
  forbidOnly: true,
  retries: 0,
  workers: 1,
  reporter: [["list"]],
  outputDir: "./artifacts/test-results",
  use: {
    baseURL: "http://localhost:5173",
    viewport: { width: 1440, height: 900 },
    deviceScaleFactor: 2, // Retina 캡처 (Figma 임포트 선명도)
    locale: "ko-KR",
    timezoneId: "Asia/Seoul",
    actionTimeout: 15_000,
    navigationTimeout: 30_000,
  },
  projects: [
    {
      name: "desktop-chromium",
      use: { ...devices["Desktop Chrome"], viewport: { width: 1440, height: 900 } },
    },
  ],
  webServer: {
    command: "npm run dev -- --host 127.0.0.1 --port 5173 --strictPort",
    cwd: "..",
    url: "http://localhost:5173",
    reuseExistingServer: true,
    timeout: 120_000,
    stdout: "pipe",
    stderr: "pipe",
  },
});
