---
paths:
  - "frontend/src/**/*.{ts,tsx}"
---

# 프론트엔드 규칙 (React / Vite / TypeScript)

## 모듈 구조

각 도메인(`frontend/src/category/{domain}/`)은 동일한 파일 구조를 따른다:
- `types.ts` — Zod 스키마 + TypeScript 타입 정의
- `{domain}_api.ts` — API 함수 (Axios 호출)
- `hook/` — TanStack Query 훅 (useQuery, useMutation)
- `page/` — 페이지 컴포넌트

## 핵심 파일

- `api/client.ts` — Axios 인스턴스 + ApiError + 토큰 리프레시 인터셉터
- `app/routes.tsx` — 라우팅 + 접근 제어
- `hooks/` — 전역 훅 (use_auth_store 등)
- `i18n/locales/` — 다국어 (ko.json, en.json)

## 규칙

- shadcn/ui 컴포넌트 우선 사용
- 상태 관리: TanStack Query (서버 상태) + Zustand (클라이언트 상태)
- 타입: Zod 스키마에서 infer로 추출, 수동 interface 금지
- Silent Failure 금지: 사용자에게 명확한 피드백 (toast, 에러 페이지)
- 상세 패턴: `docs/AMK_FRONTEND.md`, `docs/AMK_CODE_PATTERNS.md` 참조
