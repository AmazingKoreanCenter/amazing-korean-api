import { test, expect } from "@playwright/test";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { CAPTURE_PAGES, THEMES, type PageSpec, type Theme } from "../pages";
import { TEXTBOOK_CATALOG_FIXTURE, EBOOK_CATALOG_FIXTURE } from "../fixtures";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

/**
 * Figma 레퍼런스 캡처 테스트
 *
 * 각 페이지 × {light, dark} 조합으로 풀페이지 스크린샷 생성.
 * 핵심 안정화 장치:
 *   1) next-themes localStorage 강제 주입 (flash 방지)
 *   2) document.fonts.ready 대기 (Pretendard Variable 완전 로드)
 *   3) 풀페이지 스크롤 → 맨 위 복귀 (lazy 이미지 트리거)
 *   4) lazy img 대기 (decoded == true)
 *
 * 출력:
 *   figma-capture/artifacts/screenshots/{group}/{slug}--{theme}.png
 */

const SCREENSHOT_ROOT = path.resolve(__dirname, "..", "artifacts", "screenshots");

async function stabilize(page: import("@playwright/test").Page) {
  // 1) 폰트 로드 완료 대기
  await page.evaluate(async () => {
    if ("fonts" in document) {
      await (document as Document & { fonts: { ready: Promise<void> } }).fonts.ready;
    }
  });

  // 2) 전체 높이 측정 후 천천히 스크롤해 lazy 이미지 트리거
  const totalHeight = await page.evaluate(() => document.documentElement.scrollHeight);
  const viewportHeight = await page.evaluate(() => window.innerHeight);

  for (let y = 0; y < totalHeight; y += Math.floor(viewportHeight * 0.8)) {
    await page.evaluate((yy) => window.scrollTo({ top: yy, behavior: "instant" as ScrollBehavior }), y);
    await page.waitForTimeout(200);
  }
  await page.evaluate(() => window.scrollTo({ top: 0, behavior: "instant" as ScrollBehavior }));
  await page.waitForTimeout(300);

  // 3) 모든 <img>가 decoded 상태가 될 때까지 대기 (lazy/srcset 포함)
  await page.evaluate(async () => {
    const imgs = Array.from(document.images);
    await Promise.all(
      imgs.map((img) =>
        img.complete && img.naturalWidth > 0
          ? Promise.resolve()
          : new Promise<void>((resolve) => {
              img.addEventListener("load", () => resolve(), { once: true });
              img.addEventListener("error", () => resolve(), { once: true });
              // 이미 로드 완료된 경우를 대비한 fallback
              if (img.complete) resolve();
            })
      )
    );
  });

  // 4) 최종 렌더링 안정화 여유
  await page.waitForTimeout(400);
}

function describePage(spec: PageSpec, theme: Theme) {
  return `[P${spec.priority}] ${spec.group}/${spec.slug} (${theme})`;
}

for (const spec of CAPTURE_PAGES) {
  for (const theme of THEMES) {
    test(describePage(spec, theme), async ({ page, context }) => {
      // next-themes: localStorage 'theme' 키에 값을 넣고, prefers-color-scheme도 동기화
      await context.addInitScript((t) => {
        try {
          window.localStorage.setItem("theme", t);
        } catch {
          /* ignore */
        }
      }, theme);
      await page.emulateMedia({ colorScheme: theme });

      // API 모의 응답 — 로컬 AMK 백엔드 미실행 시에도 카탈로그가 실제 콘텐츠로 렌더링되도록.
      // Vite 프록시는 /api/* → :3000/* 로 리라이트하므로 /api/textbook/catalog, /api/ebook/catalog 매칭.
      await page.route("**/api/textbook/catalog", async (route) => {
        await route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify(TEXTBOOK_CATALOG_FIXTURE),
        });
      });
      await page.route("**/api/ebook/catalog", async (route) => {
        await route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify(EBOOK_CATALOG_FIXTURE),
        });
      });

      const response = await page.goto(spec.path, { waitUntil: "networkidle" });
      expect(response, `nav: ${spec.path}`).not.toBeNull();
      const status = response?.status() ?? 0;
      expect(status, `HTTP ${status} for ${spec.path}`).toBeLessThan(400);

      // 테마가 실제로 적용됐는지 검증 (next-themes attribute="class")
      await page.waitForFunction(
        (t) => document.documentElement.classList.contains(t) || (t === "light" && !document.documentElement.classList.contains("dark")),
        theme
      );

      await stabilize(page);

      const outPath = path.join(SCREENSHOT_ROOT, spec.group, `${spec.slug}--${theme}.png`);
      await page.screenshot({
        path: outPath,
        fullPage: true,
        animations: "disabled",
        caret: "hide",
      });
    });
  }
}
