import path from "path";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  test: {
    environment: "jsdom",
    globals: true,
    setupFiles: ["./src/test/setup.ts"],
    css: false,
    include: ["src/**/*.{test,spec}.{ts,tsx}"],
    exclude: ["e2e/**", "node_modules/**", "dist/**"],
    coverage: {
      provider: "v8",
      reporter: ["text", "html"],
      // 명시적 화이트리스트 — 커버된 모듈만 측정 + perFile 회귀 방지.
      // 신규 모듈 cover 시 본 리스트 + 신규 *.test 파일 함께 추가.
      include: [
        "src/lib/**/*.ts",
        "src/utils/**/*.ts",
        "src/hooks/use_auth_store.ts",
        "src/hooks/use_language_sync.ts",
        "src/api/client.ts",
        "src/api/parse_error_message.ts",
        "src/api/apply_authorization_header.ts",
        "src/components/blocks/empty_state.tsx",
        "src/components/blocks/pagination_bar.tsx",
        "src/components/blocks/stat_card.tsx",
        "src/components/blocks/skeleton_grid.tsx",
        "src/components/blocks/section_container.tsx",
        "src/components/blocks/cover_card.tsx",
        "src/components/blocks/list_stats_bar.tsx",
        "src/components/blocks/feature_grid.tsx",
        "src/components/blocks/hero_section.tsx",
        "src/components/layout/footer.tsx",
        "src/components/layout/header.tsx",
        "src/api/health_api.ts",
        "src/category/health/hook/use_health.ts",
        "src/category/health/page/health_page.tsx",
        "src/category/video/page/video_list_page.tsx",
        "src/category/payment/payment_api.ts",
        "src/category/payment/hook/use_payment_plans.ts",
        "src/category/error/page/not_found_page.tsx",
        "src/category/error/page/access_denied_page.tsx",
        "src/category/error/page/error_page.tsx",
        "src/category/textbook/textbook_api.ts",
        "src/category/textbook/hook/use_catalog.ts",
        "src/category/textbook/hook/use_my_orders.ts",
        "src/category/auth/page/reset_password_page.tsx",
        "src/category/ebook/ebook_api.ts",
        "src/category/ebook/hook/use_ebook_catalog.ts",
        "src/category/ebook/hook/use_my_purchases.ts",
        "src/category/study/study_api.ts",
        "src/category/study/hook/use_study_list.ts",
        "src/category/study/hook/use_writing_stats.ts",
      ],
      exclude: [
        "src/**/*.{test,spec}.{ts,tsx}",
        "src/test/**",
        "src/main.tsx",
        "src/vite-env.d.ts",
      ],
      // 본 thresholds = 현재 cover 된 모듈의 회귀 방지용 (점진 상향 가능).
      // pagination_bar `pointer-events-none` 분기 + footer Dialog onOpenChange
      // + header NavLink isActive·tier separator 분기 = 직접 호출 어려움.
      thresholds: {
        perFile: true,
        statements: 90,
        branches: 75,
        functions: 60,
        lines: 90,
      },
    },
  },
});
