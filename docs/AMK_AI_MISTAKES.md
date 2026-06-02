# AMK_AI_MISTAKES — AI (Claude) 작업 실수 기록

> **목적**: AI 작업 중 발생한 실수를 사실 기반으로 기록. 작업 시 사전 참조하여 같은 실수 회피.
> **작성 원칙**: 사실만 기재. 가정·해석·"왜 그랬을까" 류 추측 배제. 원인 = 직접 관찰 가능한 행위. 결과 = 직접 관찰 가능한 영향.
> **참조 시점**: 모든 작업 시작 전 본 문서 확인. 메모리 `feedback_ai_mistakes_reference.md` 가 본 문서를 참조하라고 지시.
> **production 인시던트 (INC-NNN)**: 별도 — `AMK_DEPLOY_OPS.md` + `AMK_CHANGELOG.md` 참조. 본 문서는 **AI 실수만**.

---

## 사고 카탈로그

### M-001 — stale 메모리만 보고 origin 상태 미확인

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오전, 세션 시작 직후) |
| AI 행위 | 메모리 `project_status.md` (2026-04-30 기준) 만 읽고 "unblocked 작업 없음" 결론 출력 |
| 사용자 응답 | "내가 잘 이해가 안가. 무슨 의도로 이렇게 답변한거야??" |
| 누락 행위 | `git fetch` + `gh pr list --state all` 미실행. origin 의 PR #194~#199 진행 미인지 |
| 결과 | 사용자가 직접 정정 요청. 추가 조사 후 재답변 |

---

### M-002 — books 핸드오프 프롬프트에 도메인 혼재

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오전) |
| AI 행위 | books 세션용 핸드오프 프롬프트 작성 시 "UI 용어 번역 (518 keys 갭)" 과 "학습 콘텐츠 작업" 을 같은 섹션에 기재 |
| 사용자 응답 | "UI 용어 번역이야 아니면 학습콘텐츠 번역 내용이야?" |
| 누락 행위 | 메모리 `project_q13_briefing.md` 의 "content_translations(학습 콘텐츠) ≠ i18n locale(UI)" 명시를 사전 참조 안 함 |
| 결과 | 사용자가 도메인 분리 명시 지시. `feedback_translation_domain_separation.md` 신규 메모리 + 핸드오프 재작성 |

---

### M-003 — git reset --hard 시 미커밋 변경 폐기

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오전) |
| AI 행위 | INC-005 fix 작업 진입 시 KKRYOUN 동기화 절차로 `git reset --hard origin/main` 실행 |
| 누락 행위 | reset 전 `git status` 확인 안 함. 환경 정보의 세션 시작 git status (`M docs/AMK_CHANGELOG.md`, `M docs/AMK_EBOOK_SECURITY.md`, `M docs/AMK_STATUS.md`) 무시 |
| 결과 | 미커밋 변경 3개 파일 working tree 에서 폐기. `AMK_EBOOK_SECURITY.md` = 사용자 e-book 보안 연구 노트 (TOONRADER 벤치마크) |
| 복구 경로 | git reflog/fsck = 미커밋 추적 안 함, 복구 X. VSCode local history = 검색 결과 없음. 다른 세션이 메모리 `reference_toonrader.md` (생존) + 대화 컨텍스트 기반으로 재작성 → 본 세션 즉시 commit (`00a7ec0`) 으로 보호 |
| 사용자 응답 | "AMK_EBOOK_SECURITY.md말고 유실된게 더 있어?" / "왜 유실된거야??" |

---

### M-004 — pr-check workflow 도입 시 main baseline 미검증

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오후) |
| AI 행위 | `.github/workflows/pr-check.yml` 신규 작성 후 push. push 가 self-test 트리거 |
| 누락 행위 | 워크플로 도입 전 main HEAD 의 `cargo clippy --lib --bins --locked -- -D warnings` + `cd frontend && npm run lint` 통과 여부 사전 검증 안 함 |
| 결과 | self-test 첫 실행 = backend (cargo clippy) FAILURE 1건 (`useless_conversion`, `src/api/auth/service.rs:192`. Rust 1.95 신규 룰) + frontend (npm run lint) FAILURE 27 errors (대부분 `react-refresh/only-export-components`, shadcn/ui 컴포넌트 + variants 같은 파일 export 패턴) |
| 사용자 응답 | "이건 뭐야 지금??" |
| 후속 | clippy 1줄 fix + lint 단계 `continue-on-error: true` 임시 적용 + Q16 신규 (baseline cleanup 큐) |

---

### M-006 — `cargo fmt --check --all` 결과 의미 잘못 해석

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오후, pr-check.yml 도입 검증 시) |
| AI 행위 | 로컬 `cargo fmt --check --all` 실행 후 출력에 ANSI 색 코드 잔여물 + diff 라인 (`}` + `+`) 이 있었으나 `exit=0` 만 보고 "통과" 로 사용자 보고 |
| 누락 행위 | exit code + 출력 둘 다 검증 X. ANSI 색 잔여물로 잘못 해석 |
| 결과 | pr-check.yml 도입 후 첫 self-test 시 `cargo fmt --check --all` FAILURE. CI 환경에서 crates/crypto 다수 파일 unformatted 검출 |
| 사용자 응답 | (PR #205 두 번째 fail 후 보고) |
| 후속 | M-005 와 같은 카테고리 (추정 단정) 의 새 발현 — 검증 결과의 의미 잘못 읽음 |

### M-007 — 다른 문서 라인 번호 직접 검증 X

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (저녁, AMK_DEBTS.md 작성 시) |
| AI 행위 | `docs/AMK_DEBTS.md` 작성 시 `AMK_STATUS.md §8.2 검증된 리스크 표` + 어제 grep 결과의 라인 번호를 **직접 grep 검증 없이 복사** |
| 누락 행위 | 본 문서 작성 시점 HEAD 기준 grep 으로 라인 번호 사실 확인 X. 원본 `AMK_STATUS.md` 도 stale 였음 |
| 결과 | 5 agent 정합성 검증 시 라인 번호 다수 stale 발견 (deploy.yml:87-98 → 92-103 / ebook/service.rs 8곳 모두 stale / config.rs:97,101,325 모두 stale / auth/service.rs:397,1396 → 358,1123 / video/repo.rs:237 → 233 등). 부채 카운트 자체 (B3 npm 2 → 3) 도 정정 필요 |
| 사용자 응답 | "현재 프로젝트 상태와 비교해 다시 검증" |
| 후속 | 본 문서 라인 번호 정정 + AMK_STATUS §8.2 정정 + AMK_DEBTS 의 라인 번호 사용 정책 명시 ("HEAD 기준, 사용 시 grep 재확인") |

### M-005 — lint:ui 결과 추정으로 단정

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오후, M-004 직후) |
| AI 행위 | M-004 의 frontend job 분석 시 "lint:ui (하드코딩 색상 검사) 는 PASS" 라고 사용자에게 출력 |
| 누락 행위 | 실제 데이터 = lint step (이전 step) 이 fail 해서 후속 step (lint:ui) 실행 자체가 안 됨 (set -e). lint:ui 결과는 unknown 이었으나 "PASS" 로 단정 |
| 결과 | M-004 fix 후 재 self-test 시 lint:ui 가 실제로 fail. 9 곳 하드코딩 색상 검출 (`textbook_order_page.tsx` 4곳 / `receipt_parts.tsx` 1곳 / `book_hub_page.tsx` 4곳 / `HangulKeyboardKey.tsx` 1곳). frontend job FAILURE 재발 |
| 사용자 응답 | "왜 자꾸 실수하지?? 원인을 제시해" / "이런 현상이 계속 발생하는 근본적인 원인이 뭐야?" |

---

### M-008 — B4 commit 시 cargo fmt 검증 단계 누락

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (밤, Phase 1 부채 처리) |
| AI 행위 | B4 부채 (auth/service.rs unwrap → AppError::Internal 매핑) commit 시 `cargo check --workspace` 만 실행, `cargo fmt --check --all` 미실행. 통과 단정 후 commit `ad239ed` push |
| 누락 행위 | 본 commit 시점에 pr-check.yml 의 fmt step 인지 안 함 (해당 step 은 PR #205 머지 시 이미 추가되어 활성). cargo fmt 가 method chaining 줄바꿈 위치 (`let user_info = user\n    .ok_or_else(...)` vs `let user_info =\n    user.ok_or_else(...)`) 권고 미적용 |
| 결과 | PR #211 backend (cargo check + clippy) job FAILURE (Diff in src/api/auth/service.rs:1398). 사용자가 머지 전 직접 발견 + 메시지 ("이게 맞아??"). 추가 commit `7ffcc96` (fix(fmt): cargo fmt 적용) 으로 fix → backend job pass → 머지 진행 |
| 사용자 응답 | "지금 아래와 같이 나오는데, [diff]. 이게 맞아??" |
| 후속 | M-006 (cargo fmt 결과 의미 잘못 해석) 와 동일 카테고리 (검증 단계 미챙김) 의 다른 발현 — M-006 = 결과 해석 오류, M-008 = 단계 자체 누락 |

### M-011 — 부채 자격 미성립 항목 자동 무한 작업 생성 (busywork pattern)

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-11 (18 PR 세션) |
| AI 행위 | `docs/AMK_DEBTS.md` 의 G10 entry ("src/ 테스트 부족") 를 부채로 인지 → 우선순위 매트릭스 #1 자리에서 자동 진입 → "다음 도메인 cover" 패턴 N+1 자동 적용 → 18 PR 자연 생성 (textbook → ebook → study → user/video/lesson → admin → page tests → backend deeper) |
| 누락 행위 | (1) **부채 entry 의 가치 명세 부재 검증 X**: G10 = "src/ 테스트 부족" 만 적힘. WHERE (어느 path) / WHAT (어떤 종류) / HOW MUCH (수치) / WHY (incident) / END (충분 정의) 5필드 모두 부재 = 부채 자격 미성립 상태. (2) **각 PR 의 외부 가치 자가 평가 X**: "frontend 122→249 tests / coverage 100%" 같은 메타 지표로 보고 = 사용자에게 "진행 중" 신호. (3) **패턴 N+1 자동 확장 인지 X**: textbook 패턴 → ebook → study → user/video/lesson 4회 반복 = 동일 패턴이므로 marginal returns 감소 명백했으나 자각 못 함. (4) **사용자가 stop 신호 없으면 무한 생성**: "오늘은 어디까지?" 질문 한 번도 안 함. |
| 결과 | 12 해결 부채 cross-reference 결과 = 8/12 = incident-based 실제 가치 / 4/12 = 메타. 어제 18 PR 중 추정 ~14건이 패턴 N+1 반복 (busywork). 사용자가 다음 날 (2026-05-12) "왜 작업이 계속 생기냐" 질문 → 근본 분석 = 부채 자격 미성립 항목이 카탈로그 #1 = AI 자동 진입 트리거. |
| 사용자 응답 | "왜 해도 해도 작업이 더 생기는거야?" / "PR 개수 캡은 그냥 작업하는거지 근본적인 문제 해결 X" / "패턴 세워도 너가 못 읽으면 무용. 이쁘게 꾸미는 것 밖에 안돼" / "'부족'이라는 의미 자체가 모호" |
| 후속 처리 | PR #291 (2026-05-12) = 5필드 게이트 정착 + `AMK_OBSERVATIONS.md` 분리 + 카운트 30 → 15. 구조적 차단 = 카탈로그 자체에서 G10 제거 → AI 자동 진입 못 함 |
| 회피 룰 | (1) **부채 카탈로그 첫 항목 자동 처리 금지** = 사용자 명시 결정 후만 진입. (2) **신규 부채 등재 시 5필드 게이트 통과 필수** = 못 채우면 `AMK_OBSERVATIONS.md` (작업 X). (3) **메타 지표 보고 금지** = "tests N개 추가 / coverage 100%" → "어떤 incident class 차단 / 누구에게 영향" 으로 변환 후 보고. (4) **패턴 N+1 자각** = 같은 패턴 2회 반복 시 자동 정지 + 사용자 확인. (5) **AI 의 "구조 만들기" 본능 자체가 또 다른 busywork** = 룰 / 정책 / 분류 만들기로 도망. 정작 사용자가 필요한 건 product critical failure mode 정의 (AI 영역 아님). |

### M-010 — stale 정정 부분만 + 권고 전 외부 검증 누락

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-08 (오전, A1 KYB stale 정정 후) |
| AI 행위 | 사용자 어제 (2026-05-07) 의문 ("이미 신청 다 완료된 상태인데??") 응답 위해 KYB 관련 전수 grep → KYB 인증 ✅ 완료 (2026-02 추정 승인) 사실 확인 → AMK_DEBTS A1 표 정정 시 ~~A1-2 (Webhook Secret)~~ ✅ + ~~A1-3 (KYB)~~ ✅ 만 마킹. **A1-1 (GitHub Secrets 12개 일괄 교체) + Step 4 (배포) 도 같은 카테고리 stale 가능성 미검증**. "A1-1 사용자 GitHub Secrets 업데이트 = 즉시 가능" 형태로 권고. |
| 누락 행위 | (1) stale 정정 시 같은 카테고리 (A1) 의 모든 항목 동일 검증 누락 — KYB stale = A1 카테고리 전체 stale 신호인데 KYB 만 정정. (2) 권고 전 `curl https://api.amazingkorean.net/payment/plans` 응답 확인 (5초 작업) 미실행 — 실행했으면 `sandbox: false` + `client_token: live_*` + Live Price IDs 즉시 발견. (3) `gh secret list` 등록 시점 (2026-02-18 ~ 2026-03-19) + `AMK_CHANGELOG 2026-03-18 "Paddle Live 전환"` 항목 cross-check 누락. |
| 결과 | 사용자가 A1-1 시작 결정 후 검증 단계에서 비로소 Live 활성 사실 발견. 사용자에게 "의도가 뭐냐 / 어느 옵션이냐" 식 책임 전가성 답변 출력 (옵션 A/B/C/D 4지선다). |
| 사용자 응답 | "아니 너가 제시해서 한거잖아? 의도는 없고 너가 하자고 해서 한건데, 그렇게 나오면 너가 잘못파악한거지!" |
| 후속 | M-007 (다른 문서 라인 stale 검증 X) + M-009 (SSoT 본문 마킹 누락) 와 같은 카테고리 (부분 검증 + 외부 cross-check 미실시) 의 다른 발현. M-010 특이점 = **권고 출력까지 한 후 검증** = 사용자 결정 기반으로 작업 시작 = 결정 신뢰성 손상. |
| 회피 룰 | (1) 카테고리 stale 발견 시 = 카테고리 모든 항목 즉시 동일 검증. (2) 사용자에게 작업 권고 출력 전 = 외부 상태 확인 (API 응답 / git log / `gh secret list` 등) 5초 투자 필수. (3) "사용자가 X 라고 함 → X 가 사실" 단정 회피 = 사용자 인지도 stale 가능 (어제 사용자 의문 자체가 stale 신호였음). |

### M-009 — 부채 처리 commit 후 SSoT 본문 마킹 누락

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-05 (commit `9fa6f14`, AMK_AUDIT N-14 처리), 2026-05-06 (cross-check 발견) |
| AI 행위 | N-14 처리 commit `9fa6f14` (`docs(textbook): 마이그 표기 stale 정정 — 4 → 7 직접 + 1 supported_language (N-14)`) 에서 `docs/AMK_API_TEXTBOOK.md` 갱신만 수행. **`docs/AMK_AUDIT_2026-05-04.md:262` 의 `### N-14.` 헤더에 `~~취소선~~` + ✅ 마킹 추가 누락** |
| 누락 행위 | commit msg 에는 "(N-14)" 명시 + `참조: AMK_AUDIT_2026-05-04.md §N-14` 본문 표기까지 했음에도, 정작 SSoT 본문 (AMK_AUDIT) 의 마킹 갱신은 빠뜨림 |
| 결과 | 1일 후 (2026-05-06) 본 세션 cross-check 시점에 N-14 가 미해결 표시로 잔존. AMK_AUDIT 신규 미해결 카운트 = 실제 2건이지만 표기 3건. 메모리 description "AMK_AUDIT 신규 미해결 4 → 3건" 도 부정확 (실제는 2건). 본 세션 commit `bf90afc` 에서 정정 |
| 사용자 응답 | "지금까지 부채 관련 작업에 대해서 진행한 사항들을 문서 및 메모리랑 비교해서 정리해. 너가 지금 잘 이해를 못하는 것 같아!" |
| 부수 발견 | (a) `AMK_DEBTS.md:34` §0 합계 표기 "약 57건" 이 본문 카테고리 합산 (53건) 과 4건 차이 = stale (어제 시점부터). (b) 메모리 frontmatter description "AMK_DEBTS 92→56" 의 "92" 출처 불명, §0 표 카운트와 불일치. AI 가 메모리 표기를 그대로 인용하면서 SSoT 와 cross-check 미실시 |
| 후속 | M-007 (다른 문서 라인 번호 직접 검증 X) 와 같은 카테고리 (SSoT 표기 신뢰 + 직접 검증 누락) 의 다른 발현. M-007 = 라인 번호, M-009 = 처리 마킹 + 카운트 합계 |

---

### M-012 — 마이그 검증을 진짜 경로 대신 대용물로 + 문서 sweep 범위를 판단으로 한정

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-19 (스키마 명명 2단계 그룹 ①②③ 작업 중) |
| AI 행위 | 신규 마이그 3개 검증을 `sqlx migrate run`(prod/CI 실제 경로) 대신 `psql` 수동 적용(대용물)으로 수행. 문서 동기화는 SCHEMA_PATCHED/MASTER/LEARNING 등 AI 가 관련 있다고 판단한 문서만 sweep |
| 누락 행위 | (a) 마이그 파일 3개 버전 prefix 가 전부 `20260519` 동일(`migrations/README.md §1.3` "같은날 복수=연속날짜" 위반) 미검출 — psql 수동적용은 sqlx 버전중복을 안 탐 (b) `docs/` 전역 grep 미실시 → `AMK_APP_ROADMAP.md:134` 구명 `subscriptions` 잔존 미검출 |
| 사용자 응답 | "커밋하기 전에 ... 추측하지 말고 실제 코드들을 바탕으로 검증해서 보고해" → 이후 "다른 결함은 없는거야?" 재압박 |
| 결과 | 사용자 요구로 검증 패스 실행, 결함 2건(버전충돌=CI/prod 마이그 실패 유발 / 문서 sweep 누락) 적발·정정. fresh DB `sqlx migrate run` 게이트로 수정 유효 확인(+ 사전존재 G16 부채로 로컬 fresh 검증 천장 확인). 검증 안 했으면 2건 미검출 상태로 커밋될 뻔함 |
| 후속 | 처방 = "더 정독"이 아니라 판단 기반 샘플링 → 결정론적 전수 게이트 전환(실제 경로 실행 / 전역 grep / 컴파일러 oracle). 잔여 = 명명된 3개(prod checksum 등록·런타임 쿼리 통합테스트·prod 스모크)로 환원, 기존 배포 게이트로 닫음. 상세 = `docs/AMK_SCHEMA_NAMING_AUDIT.md §10` |

---

### M-013 — 손패치 로컬 DB의 제약명을 진실로 신뢰 + cargo fmt 미실행 (CI 적발)

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-19 (스키마 명명 2단계 커밋·PR #314 후, 사용자 머지 시도 시점) |
| AI 행위 | M-012 정정 후에도 (a) 마이그 제약 RENAME 대상명을 손패치 로컬 amk-pg `pg_constraint` 스캔 결과로 작성 — 그 DB에만 있던 중복 FK `user_export_data_user_id_fkey1`(원본 20260208 은 FK 1회만 정의, clean/prod 엔 `_fkey` 단일) 포함 (b) 리네임으로 길어진 문자열 편집 후 `cargo fmt` 미실행 |
| 사용자 응답 | "내가 머지를 시도할테니 모니터링하고 있어" → PR #314 CI = backend/integration/Playwright FAILURE |
| 누락 행위 | (a) 제약명을 마이그 원본(SoT)에서 도출하지 않고 오염된 런타임 DB 신뢰 — fresh DB 재현 검증을 커밋 전 미실시 (b) `cargo fmt --check` 커밋 전 미실행 (M-008 동일 카테고리 재발) |
| 결과 | CI 가 차단: `psql:20260519:..: ERROR: constraint "user_export_data_user_id_fkey1" ... does not exist`(integration/Playwright) + `cargo fmt --check` FAILURE(backend). 머지 불가. 사용자 머지 시도가 실제 게이트 역할. 미검출 시 prod 배포 마이그 실패 가능 |
| 후속 | 마이그 3개 = 존재 가드(DO + `pg_constraint` IF EXISTS + `ALTER INDEX IF EXISTS`) 환경독립 재작성. `cargo fmt --all` 적용. **fresh DB 에 CI 동일 lexicographic psql 루프로 전체 마이그 재현 검증**(우리 3 무에러·`_fkey1` 정상 skip·8테이블 전환 확인) = 대용물 아닌 진짜 경로. 교훈: 스키마 객체명 SoT = 마이그 원본이지 런타임 DB 아님(특히 손패치 환경). 상세 = `AMK_SCHEMA_NAMING_AUDIT.md §10` |

---

### M-014 — 누적 수치를 발생률로 확대 단정 (compromised 24건 → "24건/일")

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-06-02 (관리자 세션 v2 Phase 2 문서·PR 작성 중) |
| AI 행위 | MFA 사건(2026-05-30) 당시 DB `login_state` 분포의 **누적값** `compromised=24`(테이블 전체, 대부분 1주 밖)를 Phase 2 문서/PR/메모리에 **"prod compromised 24건/일"(발생률)** 로 확대 기재. SoT §1.3·CHANGELOG·STATUS·메모리에 전파 |
| 누락 행위 | 수치 인용·재가공 전 **원천(누적 vs 발생률) 미확인** + 일별 분포 쿼리 cross-check 미실시. "24"의 단위(총량)에 임의로 "/일" 부여 |
| 사용자 응답 | prod A-1(최근 7일 일별 분포) 실행 = compromised **7일간 2건**(05-28·05-30), Phase 1 후 0 → "24건/일" 반증 |
| 결과 | 문제 규모 과대평가 → single-flight 효과를 "24/일 소거"로 기대했으나 베이스라인이 이미 ~0 이라 passive 지표로 효과 입증 불가. 정정 PR 로 4개 문서·메모리 수정 |
| 후속 | 교훈: 인용 수치는 **단위·원천(누적/율/스냅샷)을 먼저 확인**, 재가공(예: /일 부여) 시 데이터로 검증. M-005/M-009(추정→단정) 동일 카테고리 |

---

### M-015 — prod 데이터 확인 전 원인 단정 ("거의 확실히 stale .env.prod")

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-06-02 (Phase 2 prod 검증 중 admin active=2 진단) |
| AI 행위 | admin `active_now=2`(정책 max1 위반처럼 보임) 원인을 **"거의 확실히 EC2 `.env.prod` 의 stale `MAX_SESSIONS_ADMIN=2` override"** 로 단정(로컬 .env 선례에 끌려, "제 배포-동기화 누락"까지 자인). printenv 확인 전 결론+수정절차 제시 |
| 누락 행위 | 진단 결론 전 **런타임 실값(`docker exec amk-api printenv`) 미확인**. 가설을 데이터보다 먼저 신뢰 |
| 사용자 응답 | printenv = `MAX_SESSIONS_ADMIN=1`·`REFRESH_TTL_SECS_ADMIN=3600` → 가설 반증. 이어 active 전체 쿼리 = 현재 정확히 1 |
| 결과 | 오진. 다만 "먼저 printenv로 확정하자"며 검증 쿼리를 함께 제시해 1왕복에 교정. 실제 원인 = 테스트 중 찰나의 과도상태(self-heal to 1) |
| 후속 | 교훈: 이상 징후 원인은 **추정 전 런타임 실측(env/state)으로 먼저 좁힌다**. hedge("거의 확실히")해도 데이터 전 결론 제시는 오진 리스크. M-001/M-012(사전 상태 미확인) 동일 |

---

## 카테고리 분포 (M-001 ~ M-015)

| 카테고리 | 사고 ID | 공통 행위 |
|----------|---------|----------|
| 사전 상태 미확인 | M-001, M-003, M-004, **M-008**, **M-010**, **M-012**, **M-013**, **M-015** | 작업 시작 전/검증 시 현재 상태 (origin / git status / baseline / CI step / 외부 API / **실제 적용 경로·전역 범위·SoT 출처·런타임 실값**) 확인 누락. M-012 = 진짜 경로(`sqlx migrate run`) 대신 대용물(`psql`)·sweep 판단한정. M-013 = 스키마 객체명을 SoT(마이그 원본) 아닌 손패치 런타임 DB 신뢰. M-015 = 이상 징후 원인을 런타임 env 실측 전 단정 |
| 추정을 사실로 단정 | M-002, M-005, M-006, M-007, **M-009**, **M-014**, **M-015** | 데이터 X 또는 검증 X 인 항목을 결과 단정 형태로 출력. M-006 = 검증 결과의 의미 잘못 해석, M-007 = 다른 문서 라인 번호 복사, M-009 = SSoT 본문 마킹 + 카운트 표기를 cross-check 없이 그대로 인용, **M-014 = 누적값(24)을 발생률(24/일)로 단위 확대, M-015 = 가설을 데이터 전 결론으로 제시(hedge 동반)** |
| 부분 정정 (stale 잔존) | **M-010** | 카테고리 stale 발견 시 일부 항목만 정정, 같은 카테고리 다른 항목 미검증으로 stale 잔존. 권고까지 출력 후 검증 단계에서 비로소 발견 |
| **자동 무한 작업 생성 (busywork)** | **M-011** | 부채 자격 미성립 (5필드 미충족) 항목을 부채로 인지 → 자동 #1 우선순위 진입 → 패턴 N+1 자동 확장 → stop 신호 없으면 무한 PR 생성. 메타 지표 ("tests N개 추가") 로 보고하여 사용자 stop 신호도 늦어짐 |

---

## 갱신 규칙

- 신규 사고 발생 시 본 문서에 새 M-NNN 엔트리 추가
- 엔트리 내용 = 사실만. 가정·해석·"왜 발생했나" 류 추측 배제
- 원인 = "AI 가 무엇을 했는가 / 무엇을 안 했는가" 행위 단위
- 결과 = 직접 관찰된 영향 (사용자 응답 / 데이터 손실 / fail 등)
- 본 문서 갱신 시 메모리 갱신 불필요 (메모리는 본 문서를 참조하라는 포인터만)
