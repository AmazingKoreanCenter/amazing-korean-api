# figma-capture

Figma 재구축을 위한 레퍼런스 스크린샷 생성 도구.

## 목적

`project_figma_plan.md`의 **Phase A — Playwright 캡처** 구현.

현재 프론트엔드의 실제 렌더링을 고해상도 이미지로 캡처해 Figma 레퍼런스 레이어로
임포트하기 위함. Figma MCP로 그 위에 편집 가능한 네이티브 컴포넌트(Phase C)를
쌓는 하이브리드 전략의 출발점.

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
