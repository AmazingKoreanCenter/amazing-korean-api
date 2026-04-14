import path from "path"
import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"
import checker from "vite-plugin-checker";

export default defineConfig({
  plugins: [
    react(),
    checker({
      typescript: true,
    }),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  server: {
    proxy: {
      "/api": {
        target: process.env.VITE_PROXY_TARGET ?? "http://127.0.0.1:3000",
        changeOrigin: true,
        secure: false,
        rewrite: (path) => path.replace(/^\/api/, ""),
      }
    },
  },
  build: {
    // 메인 번들에서 무거운 라이브러리를 분리해 첫 페인트 부담 감소.
    // route-level lazy(React.lazy)와 함께 사용. perf-audit/baseline-pre 결과
    // (home Perf 48, TBT 2572ms, FCP 4s)가 Quick Win 적용의 동기.
    rollupOptions: {
      output: {
        manualChunks: (id) => {
          if (!id.includes("node_modules")) return undefined;

          // 핵심 React 런타임 — 거의 모든 페이지에 필요, 별도 청크로 캐싱
          if (/[\\/]node_modules[\\/](react|react-dom|react-router|react-router-dom|scheduler)[\\/]/.test(id)) {
            return "vendor-react";
          }
          // Radix UI primitives 묶음 — UI 컴포넌트 다수가 의존
          if (/[\\/]node_modules[\\/]@radix-ui[\\/]/.test(id)) return "vendor-radix";
          // 결제 — 결제 흐름 진입 시에만 필요
          if (/[\\/]node_modules[\\/]@paddle[\\/]/.test(id)) return "vendor-paddle";
          // 캐러셀 — Book 페이지 전용
          if (/[\\/]node_modules[\\/]swiper[\\/]/.test(id)) return "vendor-swiper";
          // DnD — Admin 정렬 등 일부 페이지에서만
          if (/[\\/]node_modules[\\/]@dnd-kit[\\/]/.test(id)) return "vendor-dnd";
          // 비디오 플레이어 — Video/Lesson 페이지에서만 (현재는 Coming Soon)
          if (/[\\/]node_modules[\\/](react-player|@vimeo)[\\/]/.test(id)) {
            return "vendor-video";
          }
          // i18n — 거의 모든 페이지가 사용하지만 별도 청크가 캐시 효율 ↑
          if (/[\\/]node_modules[\\/](i18next|react-i18next)[\\/]/.test(id)) {
            return "vendor-i18n";
          }
          // Form 라이브러리 — 인증/주문 페이지에서만
          if (/[\\/]node_modules[\\/](react-hook-form|@hookform|zod)[\\/]/.test(id)) {
            return "vendor-forms";
          }
          // TUS 업로드 — Admin 영상 업로드에서만
          if (id.includes("tus-js-client")) return "vendor-tus";
          // TanStack Query — 서버 상태 관리, 다수 페이지가 사용
          if (id.includes("@tanstack")) return "vendor-tanstack";
          // 나머지 vendor
          return "vendor";
        },
      },
    },
    chunkSizeWarningLimit: 600, // vendor-react는 보통 ~140KB지만 여유 있게
  },
})