# figma-capture

디자인 작업 상시 시각 레퍼런스 + 시각 회귀 감지 도구. (이름은 역사적 사유 — 원래는 Figma 임포트 준비용이었음)

## 목적

현재 프론트엔드의 실제 렌더링을 고해상도 이미지(32 PNG)로 캡처해 **3계층 디자인 SSoT**의 시각 레퍼런스 계층 역할을 수행.

| 계층 | 위치 | 역할 |
|------|------|------|
| 디자인 의도 | `docs/AMK_DESIGN_SYSTEM.md` | 토큰, 컴포넌트 권장, Do/Don't, Agent Prompt Guide |
| **시각 레퍼런스** | **이 디렉터리의 `artifacts/screenshots/`** | **현재 모습, 디자인 회귀 감지** |
| 실제 구현 | `frontend/src/` + tailwind config + index.css | 토큰/컴포넌트 진짜 진실 |

**활용 시나리오**:
1. 디자인 작업 전 — 현재 모습 확인용 시각 레퍼런스
2. 디자인 작업 후 — 재캡처 후 변경 영향 확인 (시각 회귀 감지)
3. Claude Code 협업 시 — 작업 대상 페이지 PNG 첨부로 컨텍스트 전달

> **참고**: 원래 Figma 컴포넌트 라이브러리 재구축(Phase B/C)을 위한 1단계로 만들어졌으나, 2026-04-10에 Figma 도입 자체가 보류되면서 이 도구는 **상시 운영 시각 레퍼런스 도구**로 위치가 재정의되었음. 결정 배경은 `docs/AMK_DESIGN_SYSTEM.md §08` 참조.

## 사용법

```bash
# frontend/ 디렉터리에서
cd figma-capture
npx playwright test
```

Playwright `webServer` 설정이 `npm run dev`를 자동 기동하므로, 별도 dev 서버를
띄울 필요 없음. 이미 dev 서버가 5173에서 돌고 있다면 `reuseExistingServer: true`
때문에 재사용한다.

## 출력

`figma-capture/artifacts/screenshots/{group}/{slug}--{theme}.png`

- 뷰포트: 1440×900, deviceScaleFactor 2 (Retina)
- 테마: light, dark (`next-themes` localStorage 주입)
- 대상: `pages.ts`의 `CAPTURE_PAGES` (16 페이지 × 2 테마 = 32 프레임)

## 캡처 안정화 장치

1. `document.fonts.ready` — Pretendard Variable 완전 로드 대기
2. 풀페이지 점진 스크롤 → 맨 위 복귀 — lazy 이미지 트리거
3. 전체 `<img>` decoded 대기
4. `next-themes` localStorage 주입 + `emulateMedia` 동시 적용 — 테마 flash 방지

## 제외 대상 (이번 Phase)

- **5순위**: MyPage, Settings (로그인 필요)
- **Admin**: 우선순위 낮음
- **Videos/Studies/Lessons 실제 콘텐츠**: 시딩 이전이라 ComingSoon으로 대체됨
