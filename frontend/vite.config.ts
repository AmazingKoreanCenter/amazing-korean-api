import path from "path"
import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  server: {
    proxy: {
      // 0. /api 로 시작하는 요청은 백엔드로 보냄
      "/api": {
        target: "http://127.0.0.1:3000", // 백엔드 주소 (localhost 대신 127.0.0.1 권장)
        changeOrigin: true,
        secure: false,
        // 프론트에서 /api/healthz 로 요청하면 -> 백엔드에는 /healthz 로 전달됨
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
      // 1. /auth 로 시작하는 요청은 백엔드로 보냄
      '/auth': {
        target: 'http://127.0.0.1:3000', // ⚠️ 백엔드 포트 확인 필수! (예: 8000, 3000, 8080)
        changeOrigin: true,
        secure: false,
      },
      // 2. /users 로 시작하는 요청도 백엔드로 보냄
      '/users': {
        target: 'http://127.0.0.1:3000', // ⚠️ 백엔드 포트 확인 필수!
        changeOrigin: true,
        secure: false,
      },
    },
  },
})