# perf-audit

Lighthouse 기반 프론트엔드 성능 측정 도구. 속도 개선 작업의 베이스라인 + 회귀 검증용.

## 사용법

```bash
# frontend/ 디렉터리에서
npm run build               # production 빌드 먼저 (preview는 dist/를 띄움)
node perf-audit/audit.mjs <label>
```

`<label>`은 측정 회차 식별자. 권장 명명:
- `baseline-pre`     — Quick Win 적용 전
- `baseline-post`    — Quick Win 적용 후
- `after-route-split` — 후속 작업 후

## 동작

1. `vite preview`를 백그라운드 child process로 기동 (`:4173`)
2. Playwright Chromium을 lighthouse가 사용 (WSL2에서 시스템 chrome 부재 가정)
3. `pages.mjs`의 `AUDIT_PAGES`를 순회하며 lighthouse 측정 (Performance / Accessibility / Best Practices / SEO 4 카테고리 전부)
4. 각 페이지별 전체 JSON 리포트 + `_summary.json` 저장
5. 콘솔에 비교용 표 출력
6. preview + chrome 종료

## 출력

```
perf-audit/artifacts/<label>/
├── home.json              # lighthouse 전체 리포트
├── about.json
├── ...
└── _summary.json          # 페이지별 점수/지표 요약
```

`artifacts/`는 `.gitignore` 처리. 도구/설정만 커밋.

## 측정 페이지 (8개)

| 그룹 | 페이지 | 라우트 |
|------|--------|--------|
| P1 공개 | home, about, faq, coming-soon | `/`, `/about`, `/faq`, `/pricing` |
| P2 Book | book-hub, textbook-catalog, ebook-catalog | `/book`, `/book/textbook`, `/book/ebook` |
| P3 인증 | login | `/login` |

이 8페이지는 `figma-capture/pages.ts`의 16페이지 중 대표 서브셋. 나머지 8페이지(auth 5 + legal 3)는 구조가 유사해 대표 측정으로 갈음. 확장하려면 `pages.mjs`의 `AUDIT_PAGES`에 추가.

## 측정 환경 한계

- **로컬 측정**: 네트워크 RTT 0ms, CPU throttling 미적용 (lighthouse 기본은 4x slowdown 적용함). 실제 사용자 환경과는 다름
- **콜드 캐시**: lighthouse는 매 측정마다 새 세션. 캐시 효과는 별도 측정 필요
- **Field vs Lab**: INP 등 field 메트릭은 측정 안 됨. TBT를 INP 프록시로 사용

따라서 절대 점수보다 **회차 간 델타**가 중요. 베이스라인을 잡고 변경 후 비교.

## 트러블슈팅

- **Chrome not found**: `CHROME_PATH` 환경변수로 수동 지정
  ```bash
  CHROME_PATH=/usr/bin/chromium node perf-audit/audit.mjs my-label
  ```
- **Preview port 충돌**: `lsof -i :4173` 로 점유 프로세스 확인
- **lighthouse FAIL**: 페이지 자체 에러일 수 있음. 브라우저로 `http://localhost:4173/<route>` 직접 확인
