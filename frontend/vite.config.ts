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
      "/api": {
        target: "http://127.0.0.1:3000", // 백엔드 주소 (localhost 대신 127.0.0.1 권장)
        changeOrigin: true,
        secure: false,
        // 프론트에서 /api/healthz 로 요청하면 -> 백엔드에는 /healthz 로 전달됨
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
  },
})