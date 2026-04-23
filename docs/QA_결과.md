# QA 자동화 — API 팀 협조 요청 (2026-04-22)

> **발신**: `amazing-korean-ai/scripts/qa` (Mac Mini, HYMN)
> **수신**: `amazing-korean-api` 팀
> **근거 run**: `tests/qa-results/2026-04-22T01-35-53Z/` (22 lang × 전 라우트 × Cat A+B+C, full 런, 2026-04-22 KST 10:35 ~ 14:00)
> **범위**: Playwright 1838 tests + Gemma 4 26b 3444 calls + Fuzz 1200 requests

---

## 1. 요약

- **Playwright** 87 fails 로 떨어졌지만 실제 프로덕트 이슈는 **4건**. 나머지는 QA 하네스 버그(12) + QA 인프라 한계(70 + 1 OpenAPI drift).
- **Gemma Tier 2** 가 자주 놓치는 시각적 버그 **3종** 포착 — 2026-04-21 QA 도입 이후 처음 자동 탐지된 실 회귀.
- **Fuzz** 60 mutation endpoints × 20 attempts = 1200 requests → **unhandled 5xx 0건**. 백엔드 입력 검증 레이어는 건강함.

---

## 2. 프론트엔드 수정 요청 (3건)

### 2.1 🔴 `/book/ebook` 카탈로그 모바일 — "22 languages,available" 공백 누락

**증상**: 모바일 뷰포트에서 hero subtitle 이 `"...22 languages,available to read online instantly."` 로 쉼표·단어가 붙어 보임. 데스크톱은 정상.

**근거**:
- 자동 탐지: Gemma `color_contrast` + `text_overflow` 프롬프트, **14 언어 (de/en/es/fr/id/ja/kk/pt/...) × 2 체크 = 28 flag**.
- 재현: `tests/qa-results/2026-04-22T01-35-53Z/screenshots/{lang}/book-ebook_mobile.png`.

**원인**: [`frontend/src/category/ebook/page/ebook_catalog_page.tsx:97-102`](../frontend/src/category/ebook/page/ebook_catalog_page.tsx#L97-L102)

```tsx
subtitle={t("ebook.catalog.subtitle").split("\n").map((line, i) => (
  <span key={i}>
    {i > 0 && <br className="hidden sm:block" />}
    {line}
  </span>
))}
```

- `en.json:1171` 의 `"Student and teacher E-books in 22 languages,\navailable to read online instantly."` 를 `\n` 으로 split.
- 각 조각을 `<span>` 으로 감싸고, **2번째 span 앞에만 `<br>` 을 삽입하는데 `hidden sm:block`** 이라 모바일에서는 `<br>` 이 렌더링되지 않음.
- 결과: 모바일에서는 `"languages,"` 와 `"available"` span 이 공백 없이 인접 → `"languages,available"` 로 보임.

**수정 제안** (택 1):
- (a) `<br>` 대신 `<> <br className="hidden sm:block" /></>` — 모바일에서는 공백, 데스크톱에서는 br 우선 (가장 minimal).
- (b) source string 에 `\n` 대신 ` \n` 처럼 공백 포함.
- (c) split 제거하고 CSS `white-space: pre-line` 으로 대체.

---

### 2.2 🔴 `/book/textbook` 카탈로그 모바일 — "journeywith" 공백 누락

**증상**: 모바일에서 `"... Korean learning journeywith student and teacher editions..."` 로 단어가 붙어 보임.

**근거**:
- Gemma `text_overflow` 프롬프트, **3 언어 (km/my/th) × 1 check = 3 flag**. 다른 언어에서는 단어 길이 차이로 같은 자리에서 자연 줄바꿈이 일어나 감지되지 않음.
- 재현: `tests/qa-results/2026-04-22T01-35-53Z/screenshots/{km,my,th}/book-textbook_mobile.png`.

**원인**: `en.json:939` — `"description": "Start your Korean learning journey\nwith student and teacher editions in 22 languages."` + textbook_catalog_page.tsx 의 동일 split 로직 추정. 파일 위치는 confirmation 필요 (2.1 과 동일 패턴).

**수정 제안**: 2.1 과 동일.

---

### 2.3 🟡 `/` 루트 pt 로케일 데스크톱 — footer 텍스트 겹침

**증상**: 포르투갈어 데스크톱 뷰에서 copyright `"© 2016 Amazing Korean. Todos os direitos reservados."` 와 `"Termos de Uso"` / `"Política de Privacidade"` 링크가 시각적으로 오버랩.

**근거**:
- Gemma `text_overflow`, **1 flag** (pt only).
- 재현: `tests/qa-results/2026-04-22T01-35-53Z/screenshots/pt/root_desktop.png`.

**원인 추정**: footer 컴포넌트가 `flex`/`gap` 기반인데 pt 카피라이트 문구가 길어서 gap 부족, 링크 영역과 충돌. 타 언어는 문구가 더 짧아 회피.

**수정 제안**: footer 컴포넌트에서 carbon-copy 링크 묶음에 `flex-wrap` + `min-gap` 또는 link 를 copyright 아래 줄로 개행.

---

### 2.4 🟡 `/book` 캐러셀 dot 인디케이터 — aria-label 누락

**증상**: Playwright `button_coverage` 스펙이 `/book` 에서 empty-label 버튼 탐지 (en/ja × mobile+desktop = 4 건).

**근거**:
```html
<button type="button" class="w-2 h-2 rounded-full transition-colors bg-primary"></button>
```
- 텍스트 없음, `aria-label` 없음, `href` 없음 → 접근성상 dead-button.
- 재현: `tests/qa-results/2026-04-22T01-35-53Z/button_coverage/{en,ja}/book.json`.

**수정 제안**: 캐러셀 dot 에 `aria-label={t('common.goToSlide', { n })}` 추가.

**QA 쪽 선택**: 고정 전까지 dead-button spec 에서 이 패턴을 whitelist 로 제외 (decision 2-1 참조).

---

## 3. 백엔드 협조 요청 (2건)

### 3.1 JWT 토큰 TTL — QA 자동화의 user/admin 라우트 커버리지 차단

**현상**: 22 lang full run 은 Playwright Cat A/B 72 분 + Gemma 110 분 + 나머지 = **2시간 30분 이상 소요**. stage 3 에서 발급한 `QA_USER_TOKEN` / `QA_ADMIN_TOKEN` 이 런 후반에 만료되어, `design_capture` / `design_geometry` 의 user·admin 라우트가 **70건 실패** (`page.reload: net::ERR_ABORTED; maybe frame was detached` 후 `navigated to "http://localhost:5173/login"` 확인).

**근거**:
- 3 lang (13분) verify run = 0 브릿지 실패.
- 22 lang (150분) full run = **70 브릿지 실패**.
- 만료 외 다른 원인 후보 (zustand-persist race 등) 검토했으나 stage 3 ~ 실패 시점의 시간차가 결정적.

**요청 사항** (택 1 알려주시면 QA 에서 따라갑니다):

- **(A)** QA 전용 `.env` 에서 JWT TTL 을 **6시간** 이상으로 설정 허용.
  - 구체적으로는 `JWT_ACCESS_TTL_SEC` (또는 해당 키) 값을 `21600`(6h) 로. QA `.env` 는 `amazing-korean-api` 의 `.env.qa` 처럼 별도 파일로 두거나 `run_qa.sh` 가 일시적으로 주입.
  - 프로덕션 `.env` 에는 영향 없음.
- **(B)** TTL 은 프로덕션과 동일하게 두고, QA 가 주기적으로 `POST /auth/refresh` 호출해 storageState 를 갱신.
  - 이 경우 `refresh_token` 쿠키가 storageState `origins[].cookies` 에 포함되도록 백엔드 login 응답 구조 (쿠키 vs 바디) 를 알려주시면 QA 에서 합성 로직 확장.

현재 JWT TTL 이 얼마인지만 한 번 알려주셔도 QA 쪽에서 대응 가능합니다.

---

### 3.2 OpenAPI drift — `/api-docs/openapi.json` 에 1328 cell drift

**현상**: `authz_matrix.spec.ts` 가 143 엔드포인트 × 3 roles = 429 cell 을 체크. 현재 드리프트 **1328건**.

**근거**: `tests/qa-results/2026-04-22T01-35-53Z/authz_matrix.json` 에 드리프트 상세 JSON.

**원인**: `amazing-korean-ai/scripts/qa/fixtures/api_endpoints.overrides.ts` 는 2026-04-19 시점 엔드포인트 기준. 이후 API 팀이 신규 엔드포인트 (admin bulk / studies tasks / translations bulk 등) 추가 — overrides 미갱신.

**요청 사항**:
- 신규 엔드포인트들의 **기대 응답 (anon/user/admin 각 역할)** 을 알려주시거나, API 팀이 `api_endpoints.overrides.ts` 를 주기적으로 동기화하는 프로세스 합의.
- QA 쪽 임시 조치: drift 를 fail 이 아니라 warn 으로 격하 (spec 변경). Security 위반 (ex. anon 이 admin 엔드포인트 200) 만 fail.

---

## 4. QA 쪽 자체 수정 (알림)

API 팀 개입 없이 QA 하네스에서 고쳐야 할 이슈. **동 문서와 별개로 진행 완료 후 통보 예정**.

| 항목 | 증상 | 조치 |
|---|---|---|
| `critical_path/03_signup` selector | 이메일 Collapsible 트리거 대신 헤더 DropdownMenu 매칭 (4 fails) | 01_login 에서 했던 `:not([aria-haspopup])` 필터 전파 |
| `critical_path/04_ebook_pre_purchase` + `05_textbook_pre_order` selector | `[class*="aspect-[3/4]"]` 의 CSS bracket 파싱 실패로 element not found (8 fails). **프론트는 정상** | `button.bg-card.rounded-2xl` (CoverCard 루트) 로 교체 |
| Playwright reporter artifact | `test.skip(title, body)` 를 "failed" 로 카운팅 (438 건 허수) | 커밋 b49437d 에서 수정 완료 |
| storageState 합성 로직 | JWT 재발급 누락 (위 3.1) | API 팀 응답 받는 대로 구현 |

---

## 5. 확인 요청 항목 — 체크박스로 답 주시면 됩니다

- [x] **2.1 ebook subtitle** 수정 예정? → **예, 이번 세션 (2026-04-22 밤)** 에 처리 예정
- [x] **2.2 textbook subtitle** 수정 예정? → **예, 이번 세션** 에 처리 예정 (2.1 과 같은 PR)
- [x] **2.3 pt footer 오버랩** 수정 예정? → **우선순위 낮음**. Q10 (2.1/2.2/2.4) 완료 후 여유 시.
- [x] **2.4 캐러셀 dot aria-label** 수정 예정? → **예, 이번 세션** 에 처리 예정 (2.1 과 같은 PR)
- [x] **3.1 JWT TTL** → **옵션 (A) 권장**. 현재 기본값 `JWT_ACCESS_TTL_MIN=15` (분). 상세는 §6 참조.
- [x] **3.2 OpenAPI drift** → **(B) QA drift tolerance 권장**. API 팀이 overrides.ts 수작업 동기화 불가. Security 위반(anon→admin 200)만 fail 로 격하 부탁. overrides 공동 관리는 엔드포인트 추가 속도를 고려하면 현실적으로 불가.

답 주시는 대로 QA 쪽 action item (섹션 4) 동시 진행하고, 다음 full run (24-48h 안) 에 재검증하겠습니다.

---

## 6. API 팀 답변 (2026-04-22 밤)

### 6.1 JWT TTL — 옵션 A 권장 (QA env 연장)

**현재 값**:
- `JWT_ACCESS_TTL_MIN` = **15분** (default, env override 지원)
- `REFRESH_TTL_DAYS` = 30일 (Learner), 7일 (Admin), 1일 (HYMN)
- env 변수 이름은 QA 요청 문서의 `JWT_ACCESS_TTL_SEC` 가 아니라 `JWT_ACCESS_TTL_MIN` (분 단위). `src/config.rs:126`.

**QA 권장 조치**:
```bash
# amazing-korean-ai 의 .env.qa 또는 run_qa.sh 에서 주입
JWT_ACCESS_TTL_MIN=360   # 6시간 — 22 lang full run (≈2h30m) 커버
```

- 프로덕션 `.env` (`AWS EC2`) 에는 영향 없음. QA 전용 `.env.qa` 또는 `run_qa.sh` 가 일시 주입.
- API 코드 변경 불필요 — 현재 이미 env override 지원.

**옵션 B (refresh 플로우) 참고 정보** (필요 시):
- **웹**: refresh_token 은 `ak_refresh` **HttpOnly 쿠키** (SameSite=Lax, Secure in prod). Playwright `storageState().origins[].cookies[]` 에 자동 저장됨. QA 가 페이지 탐색 중 401 수신 → `POST /auth/refresh` 한 번 치면 새 쿠키 세팅 + 신규 `access_token` 바디 반환.
  - 쿠키 이름 env: `REFRESH_COOKIE_NAME=ak_refresh` (default)
  - refresh 엔드포인트: `POST /auth/refresh` (쿠키 기반, body 없음)
- **모바일**: refresh_token 을 **JSON body** 로 반환 (`MobileLoginRes.refresh_token`). 경로가 다르므로 웹 QA 에서는 사용 안 함.
- 옵션 A 가 단순하므로 우선 권장. 옵션 B 로 가면 QA 하네스가 401 감지 → refresh 재시도 레이어를 추가해야 함 (약간 복잡).

### 6.2 OpenAPI drift — QA drift tolerance 권장

- API 팀이 `api_endpoints.overrides.ts` 를 수작업 동기화하는 건 엔드포인트 추가 속도 (Q1a/b/c + Q5 + Q6 만 이번 주 10+ 신규) 를 고려할 때 현실적이지 않음.
- QA 쪽에서 **Security 위반 (anon→admin 200) 만 fail 로 격하** 부탁. 나머지 drift 는 warn.
- 중장기: OpenAPI spec 기반 다이프 자동화 검토 여지 있음 (공통 schema 추출). 이번 스프린트 범위 외.

### 6.3 처리 상태

| 체크박스 | 상태 | 커밋/PR |
|---|---|---|
| 2.1 ebook subtitle | 진행 중 | 이 PR |
| 2.2 textbook subtitle | 진행 중 | 이 PR |
| 2.4 캐러셀 aria-label | 진행 중 | 이 PR |
| 2.3 pt footer | 큐 (우선도 낮음) | 별도 |
| 3.1 JWT TTL | 답변 완료 (본 문서) | 코드 변경 없음 |
| 3.2 OpenAPI drift | 답변 완료 (본 문서) | 코드 변경 없음 |

---

## 부록 — 지표 세부

### 최종 숫자

| 항목 | 값 |
|---|---:|
| Playwright total | 1838 |
| Playwright passed | 1748 |
| Playwright failed | 87 (실제 프로덕트 이슈 4 / QA 하네스 12 / JWT 만료 70 / OpenAPI drift 1) |
| Playwright skipped | 3 |
| Gemma total calls | 3444 |
| Gemma flagged | 32 (실질 이슈 3종 × 여러 언어) |
| Gemma false positive rate | <1% (flag 중 실제 false 관측치 0) |
| Fuzz attempts | 1200 |
| Fuzz **unhandled 5xx** | **0** ✓ |
| Fuzz HTTP 401/422/400 응답 분포 | 1032 / 104 / 44 |
| 총 런 시간 | 약 2시간 27분 |

### Run 경로

- `tests/qa-results/2026-04-22T01-35-53Z/summary.md` — 집계
- `tests/qa-results/2026-04-22T01-35-53Z/triage.md` — 이슈 리스트
- `tests/qa-results/2026-04-22T01-35-53Z/playwright-html/index.html` — 인터랙티브 리포트
- `tests/qa-results/2026-04-22T01-35-53Z/ai_checks/{lang}/*.json` — Gemma 응답 전수
- `tests/qa-results/2026-04-22T01-35-53Z/fuzz/_summary.json` — Fuzz 집계

### 본 QA 시스템 소스

> 별도 저장소 [`amazing-korean-ai`](https://github.com/AmazingKoreanCenter/amazing-korean-ai) 기준 경로.

- 오케스트레이터: `scripts/qa/run_qa.sh` (Mac Mini 로컬)
- 아키텍처: `docs/qa/ARCHITECTURE.md`
- 전략: `docs/AMK_AI_QA.md`
