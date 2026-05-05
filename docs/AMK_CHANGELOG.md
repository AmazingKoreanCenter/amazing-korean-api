---
title: AMK_CHANGELOG — Amazing Korean API 변경 이력
updated: 2026-05-05 (PR #213 머지 후 A4-6/A4-7/A4-8 + G6 처리. 본 세션 누계 27 부채 처리)
owner: HYMN Co., Ltd. (Amazing Korean)
---

# AMK_CHANGELOG — 변경 이력

> `AMK_API_MASTER.md` Section 9에서 분리됨 (2026-02-17).
> 마스터 스펙 문서의 변경 이력을 시간 역순으로 기록한다.

---

- **2026-05-05 — M-008 등재 + 정책 검증 4건 + AMK_AUDIT 부채 처리 11건**

  본 세션 = M-008 등재 + Codex/Gemini CLI 정책 검증 + 정책 결정 3건 (N-32/N-35/N-36) + 작은 묶음 8건 (N-8/N-9/N-10/N-12/N-14/N-20/N-21/N-22) + N-24 동시 해결.

  ## M-008 등재 (commit `11d5801`)

  B4 commit 시 cargo fmt 검증 누락 → CI fail → 추가 fmt commit. M-006 (cargo fmt 결과 의미 잘못 해석) 의 다른 발현 (단계 자체 누락). `docs/AMK_AI_MISTAKES.md` 사고 카탈로그에 등재. 룰 추가 X (사용자 결정 정책 따름).

  ## 정책 검증 (codex GPT-5.2 + gemini 2.0 Flash 독립 검증, commit `48cd3d9`)

  - 4 파일 = `docs/AMK_POLICY_REVIEW_2026-05-05_PROMPT.md` / `_CODEX.md` / `_GEMINI.md` / 통합 `AMK_POLICY_REVIEW_2026-05-05.md`
  - 4 부채 모두 옵션 일치: N-31 A (HTTPS 선행) / N-32 A (Report-Only) / N-35 D (1회 남음 시) / N-36 D (인증/비밀번호만 generic)

  ## 정책 결정 부채 처리

  - **N-32 CSP** (commit `4ceadc8`) — `frontend/public/_headers` 신규, Report-Only 모드 (위반 차단 X, Cloudflare Reports 로깅). 외부 도메인 화이트리스트 (Paddle/Google OAuth/Vimeo/Pretendard/Google Fonts)
  - **N-35 remaining_attempts** (commit `dc28492`) — `Option<i64>` + `serde skip_serializing_if`, 1회 남음 시만 노출 (anti-enumeration 정합)
  - **N-36 Validation** (commit `a8440a9`) — `AppError::ValidationGeneric` 신규 variant + 인증 service 6 위치 + signup 1 위치 명시 사용. 일반 폼 (`update_me`/`update_settings`) = 그대로 유지

  ## 작은 묶음 부채 처리 (8건 + N-24 동시)

  - **N-10 외부 timeout** (commit `b567f62`) — 5 외부 서비스 (Resend/RevenueCat/Google/Apple/Vimeo) `Client::builder().timeout(15s)` 적용. ip-api.com 은 기존 5초 유지. production hang 차단 (HIGH 위험)
  - **N-8 + N-9 RevenueCat 보안** (commit `fcfaaf2`) — constant-time 비교 (subtle 2.6 의존성 추가, 타이밍 공격 차단) + event_timestamp_ms 5분 variance 검증 (replay 차단)
  - **N-12 EBOOK_PAGE_IMAGES_DIR** (commit `18f5682`) — `.env.example` + `deploy.yml` 명시 추가
  - **N-14 textbook migration stale** (commit `9fa6f14`) — AMK_API_TEXTBOOK.md:13 4 → 7 직접 + 1 supported_language 분리 표기
  - **N-20 Dockerfile HEALTHCHECK** (commit `254a5e1`) — `curl --fail http://localhost:3000/health` (interval 30s / timeout 5s / start-period 15s / retries 3)
  - **N-21 nginx 버전 핀** (commit `46ad698`) — `nginx:alpine` → `nginx:1.27-alpine`
  - **N-22 dev 도구** — `.gitattributes` (LF 강제) + `.editorconfig` (Rust 4 / 기본 2 스페이스)
  - **N-24 index.html CSP 메타** = N-32 _headers 처리로 동시 해결 (헤더 > 메타 강도)

  ## SSoT 갱신

  - AMK_AUDIT 신규 미해결 30 → 27 → 18 (정책 결정 + 작은 묶음 후)
  - 남은 18건 카테고리: frontend N-1~N-7 (Q16) / 인프라 N-11/N-13/N-31 (HTTPS 트랙) / Cargo N-16/N-17/N-18 / 문서 N-23 / dev N-25 / 큰 작업 N-26/N-27 / 보안 헤더 N-34 (무해)

  ## 학습

  - **M-008 패턴 회피 정착**: 이번 세션 모든 부채 처리 시 cargo check + cargo fmt + cargo clippy 3단계 검증 명시. CI fail 재발 0건
  - **정책 검증 cross-check 효과**: 사용자 권고 + 2 LLM 만장일치 = 결정 신뢰도 상승. 4 부채 모두 동일 옵션
  - **외부 타이밍 검증**: Paddle 패턴 (HMAC + 5분 variance) 을 RevenueCat 에 동일 적용 = defense-in-depth 정합

  ## 추가 작업 (본 세션 후속, 토큰 여유)

  - **N-16 Cargo license** (commit `2211aaa`) — `license = "UNLICENSED"` + `publish = false` (workspace + crates/crypto)
  - **N-17 [workspace.lints]** (commit `327a89c`) — `unsafe_code = "deny"` + clippy `dbg_macro/todo/unimplemented = "warn"` (사전 검증 anti-pattern 0건)
  - **N-11 / N-25 / N-34 🟡 수용 결정** (commit `51f810c`):
    - N-11 (postgres dev 호스트 포트) = `docker-compose.yml` `.gitignore` 정책 + 1인 dev 환경
    - N-25 (skipLibCheck=true) = false 변경 시 빌드 시간 + lib 타입 에러 다수, 비용 큼
    - N-34 (X-XSS-Protection 0) = legacy 호환 무해, CSP 가 대체

  ## 머지 전 검증 (옵션 A 채택, codex/gemini 합치 후 frontend 호환성 점검)

  - 🔴 **N-35 frontend 위험 발견**: `types.ts` 의 3 Zod schema = `remaining_attempts: z.number()` 필수. 백엔드 Option<i64> + skip_serializing_if 단독 머지 시 frontend Zod parse 실패 → 인증 흐름 ALL FAIL 위험.
  - **fix (commit `33edc22`)**: `z.number().optional()` + `setRemainingAttempts(res.remaining_attempts ?? null)` 3 페이지 5 위치
  - ✅ N-9 RevenueCat: 코드 구조 (`payload.get("event")`) = RevenueCat 표준 webhook = `event_timestamp_ms` 표준 필드 = 안전 추정
  - ✅ N-36: frontend 가 인증 422 message 파싱 안 함 (status code 만 사용)

  ## 머지 + 배포 + 검증 (PR #212, commit `33edc22`)

  - PR #212 머지, deploy workflow `25360295173` 1m3s, 모든 단계 통과
  - **외부 검증 (https://api.amazingkorean.net/health)**:
    - status 200, 0.81s
    - 보안 헤더 모두 적용 (referrer-policy / x-content-type-options / x-frame / x-xss-protection / x-robots-tag / permissions-policy)
    - 🟢 **`strict-transport-security: max-age=15552000; includeSubDomains` 발견** = Cloudflare edge 자동 HSTS 적용 중 (origin code 변경 X)
  - **외부 검증 (https://amazingkorean.net, Cloudflare Pages)**:
    - ✅ `content-security-policy-report-only` 헤더 production 적용 (N-32 정확 반영)
    - 정책 화이트리스트 (Paddle / Google OAuth / Vimeo / Pretendard / Google Fonts) 모두 명시
    - frame-ancestors 'none' / form-action 'self' / upgrade-insecure-requests

  ## N-31 발견 = Cloudflare edge 사실상 활성

  Cloudflare 통과 트래픽 = 모두 HSTS 적용 = 일반 사용자 보호 활성. Origin layer (`src/main.rs` 또는 nginx) 미추가 = Cloudflare 우회 (직접 EC2 접근) 시 미보호. EC2 직접 접근 = 기본 차단 대상이라 우선순위 낮음. AMK_AUDIT 의 N-31 = 🟢 발견 표기로 갱신 (수용 X, origin layer 추가 다음 세션 결정).

  ## PR #212 머지 후속 작업 (본 세션 후반)

  ### N-18 / N-23 처리 (commit `0bb3d16`)

  - **N-18 🟡 수용**: `cargo tree -d` 검증 = 9+ 그룹 모두 transitive 의존 (rand/getrandom/darling/itertools/socket2/hashbrown/indexmap/schemars/redox_syscall/webpki-roots). 통합 = 의존 라이브러리 자체 업데이트 대기 필요 = 작업 비용 vs 가치 (영향 작음) trade-off
  - **N-23 ✅ 해결** (commit 직전): `LICENSE` (proprietary, HYMN Co., Ltd.) + `README.md` (스택/구조/문서 포인터/개발 가이드) 신규 작성

  ### N-1~N-7 frontend Q16 묶음 (commit `04e74f3` + `56f1f39`)

  ✅ **해결 3건 (1 commit 묶음)**:
  - **N-1** window.open rel: audit 4건 표기 → 실측 5건 (admin_video_detail external + textbook 4 internal). 모두 `"noopener,noreferrer"` 추가
  - **N-4** non-null assertion 3 위치: `params.page!` → `(params.page ?? 1)`, `choiceCorrectValue!` → nullish guard
  - **N-7** fullscreen catch 무음 2건: `.catch(() => {})` → `console.warn(err)` (진단 보존)

  🟡 **수용 4건**:
  - **N-2** admin 한국어 정책 (영역 전체 한국어 일관, 1 줄만 t() 변환 시 일관성 깨짐)
  - **N-3** ebook DRM 의도 (`window.createImageBitmap` readonly override = `as any` 필수)
  - **N-5** 9 인라인 회피 = 각 의도된 사용 (mount-once / DRM / devtools_detect)
  - **N-6** Tailwind blue/yellow = 디자인 토큰 결정 대기

  ## SSoT 갱신 누계

  - 본 세션 신규 미해결 27 → 4건 (-23)
  - 처리 ✅ 16건 / 수용 🟡 7건 / 발견 🟢 1건 / 사고 등재 1건
  - 남은 4건: N-13 (nginx HTTPS, A4-1 묶음) / N-26 (i18n) / N-27 (OpenAPI) / N-31 origin

  ## PR #213 머지 후속 인프라 묶음 (commit `6e7b006` / `f82dd0d` / `9367f72` / `312ec54` + `f10f900`)

  ### A4-6 ✅ Cloudflare 운영 정책 통합 SSoT (commit `6e7b006`)

  `AMK_DEPLOY_OPS §7.6` 신규 — DNS / Pages / SSL / WAF / Email Routing 통합 + 변경 절차 + 외부 의존성 + 비상 시 절차 + Free 플랜 한계 + 변경 이력 추적 정책. 기존 = §2/§3/§4/§5/§7.5 분산.

  ### A4-7 ✅ nginx Rate Limit 모니터링 절차 (commit `f82dd0d`)

  `AMK_DEPLOY_OPS §6` (EC2 유지보수) 안에 nginx limit_req 위반 모니터링 sub-section 추가. 조회 명령 (`docker logs amk-nginx | grep 'limiting requests'`) + 로그 형식 예시 + 대응 정책 (단발성 / 동일 IP / zone 차원) + 향후 자동화 후속.

  ### A4-8 + G6 ✅ dependabot.yml 신규 (commit `9367f72`)

  `.github/dependabot.yml` 신규 — Cargo / npm (frontend) / Docker / GitHub Actions 4 ecosystems. 주간/월간 스케줄 (KST 09:00 월요일 또는 매월 1일). 보안 업데이트 = 별도 즉시 PR. **A4-8 (Docker base image 정책) + G6 (dependabot 미존재) 동시 해결**.

  ## SSoT 갱신 누계

  - 본 세션 신규 미해결 27 → 4 (-23)
  - AMK_DEBTS 합계 92 → 81 (본 세션 -11)
  - A 운영/배포 14 → 11 (A4-5/A4-6/A4-7/A4-8 처리)
  - G 자동 검증 13 → 12 (G6 처리)
  - 처리 ✅ **20건** (정책 결정 3 + 작은 묶음 8 + frontend Q16 3 + N-16/N-17 + N-23 + A4-6/A4-7/A4-8/G6 + N-24 동시)
  - 수용 🟡 7건 / 발견 🟢 1건 / 사고 등재 1건 = **본 세션 누계 29 작업** (commits 36+)

  ## 다음 세션 진입점

  1. **N-26 i18n 21언어 legal/admin** (ai 측 번역 의존, ai 세션 트리거 후 진행)
  2. **N-27 OpenAPI ~43건** (도메인별 PR 분할 — auth 10 / payment 4 / textbook 4 / ebook 7 등)
  3. **A4-1/A4-2 + N-13 + N-31 origin 인프라 묶음** = HTTPS + certbot + nginx HTTPS + origin HSTS layer (1일+, production 영향)
  4. **A4 잔여 인프라** (A4-1/A4-2/A4-3/A4-4)
  5. **AMK_DEBTS 잔여**: B 보안 (rsa Marvin / unsound 7건 / expect 48건) / C 코드 품질 (ESLint Q16 / lint:ui Q16) / J Secrets (J3/J4 자동 도구) / G 자동 검증 잔여 (G3/G4/G5/G7/G8/G10~14)

- **2026-05-04 (밤, 후속 3) — Phase 1+2 부채 처리 10건 일괄 + 검증 2/3회차 정정**

  본 세션 = 검증 2회차 (6 agent 병렬) + 옵션 C (직접 순차) + 부채 처리 (Phase 1 병렬 agent 3건 + Phase 2 직접 5건).

  ## 검증 2/3회차 (commit `5d1132f`)

  M-007 사고 (라인/카운트 stale) 후속. 6 agent 분담 병렬로 ~116 부채 검증, 정정 17건 적용:
  - 라인: A2-1 ebook fs::read 9곳 / B4 unwrap / B6 ipgeo / C6 video/repo.rs
  - 카운트: B5 expect 47→48 / H1 메모리 stale 72→41일 / J 카운트 3건
  - 카테고리: C1 warnings / C2 합계 / N-5 / N-6 / N-15 종결 (false flag)
  - N-27 도메인별 정확 매핑 = "58+" → "16" → **약 43건 + 8 도메인 완전 누락**
  - 옵션 C 직접 순차로 §3.1~§3.4 부분 검증 영역 모두 종결 + N-38 신규 (Paddle amount)

  ## Phase 1 부채 처리 (병렬 3 agent)

  - **B7 / N-38** Paddle 웹훅 amount defense-in-depth (commit `c744efc`) — 서명 통과 후 server price_cents() 비교 추가, 불일치 시 fail-closed (500 → Paddle 자동 재시도)
  - **B4** auth/service.rs unwrap 위험 2건 → AppError::Internal 매핑 (commit `ad239ed`) — anti-enumeration 유지
  - **N-37** RUST_LOG 기본값 debug → info, 옵션 B 이중 안전망 (commit `90585ab`) — 검증 결과 deploy.yml 미명시 = production 사실상 debug 로 운영 중이었음 (이론 부채 → 실제 활성 발견)

  ## Phase 2 부채 처리 (직접 5건)

  - **B3** npm audit fix — postcss + follow-redirects + basic-ftp HIGH 3건 (commit `ee68c7c`) — semver 호환, breaking change 0
  - **N-19** Dockerfile USER appuser 추가 (commit `28022ca`) — `useradd -r -u 1001 -M` + chown + USER, docker build 통과 검증
  - **N-33** Referrer-Policy strict-origin-when-cross-origin (commit `f94e135`)
  - **N-28~30** 결제 조회 인덱스 3건 신규 마이그 (commit `a4a47fb`) — `migrations/20260504_payment_lookup_indexes.sql`
  - **A4-5** Docker log 로테이션 5 서비스 YAML anchor 일괄 (commit `7e86592`) — 서비스당 max 30MB

  ## SSoT 갱신

  - **부채 합계 92 → 85** (10건 처리)
  - **신규 미해결 (AMK_AUDIT) 37 → 30**
  - AMK_DEBTS B 보안 (취약점) 4 → 1 / (panic 위험) 2 → 0 / (외부 통신) 2 → 1 / A 운영 15 → 14
  - 우선순위 매트릭스 갱신 (#2 B3, #4 B4 취소선)

  ## 학습

  - **이론 부채 → 실제 활성 발견**: N-37 검증 시 deploy.yml RUST_LOG 미명시 = production 이 사실상 debug 로 운영 중. 검증 깊이가 위험 분류를 바꿈
  - **YAML anchor 패턴**: 5 서비스 logging 통합 적용 = 변경 시 한 곳만 수정 (DRY)
  - **Defense-in-depth**: B7 처럼 1차 차단 (Paddle 서명) 통과 후에도 2차 검증 (server vs webhook amount) 추가 = SDK 버그 / 키 유출 시 차단 layer

  ## 잔존 우선순위 (다음 세션)

  - 정책 결정 필요: N-31 HSTS / N-32 CSP / N-35 remaining_attempts / N-36 Validation 에러 노출
  - 큰 작업: N-26 i18n 21언어 (ai 측 의존) / N-27 OpenAPI 약 43건 누락
  - 추가 작은 작업: A4-1/A4-2 SSL/HTTPS / A4-3/A4-4/A4-6/A4-7/A4-8 인프라

- **2026-05-04 (밤, 후속 2) — B1 rustls-webpki 보안 취약점 3건 해결**

  AMK_DEBTS B1 우선순위 1번 처리 (cargo audit 발견 RUSTSEC-2026-0098/0099/0104 = rustls-webpki 0.103.10 의 wildcard certificate / CRL panic / URI name constraints 3건).

  ## 변경

  `cargo update -p rustls-webpki` 실행:
  - `rustls-webpki 0.103.10 → 0.103.13`
  - 변경 파일 = `Cargo.lock` 1건만

  ## 검증

  - `cargo audit` 재실행 = rustls-webpki 매칭 없음 (3건 모두 사라짐)
  - `cargo fmt --check --all` exit=0
  - `cargo check --locked --workspace` exit=0
  - `cargo clippy --lib --bins --locked -- -D warnings` exit=0

  ## AMK_DEBTS 갱신

  - B1 표 RUSTSEC-2026-0098/0099/0104 행 ~~취소선~~ + 해결일/방법 명시
  - §0 카운트 갱신 (B 보안 취약점 7 → 4)
  - 잔존 = rsa Marvin Attack (RUSTSEC-2023-0071) — no fixed upgrade, 의존성 회피 검토 필요

  ## 다음 부채 후보 (AMK_DEBTS 우선순위)

  - B3 npm audit fix (postcss + follow-redirects + basic-ftp HIGH) — `npm audit fix` 1 명령
  - J1 RATE_LIMIT_TEXTBOOK_* INC-001 패턴 차단
  - B4 unwrap 위험 2건 fix (auth/service.rs:358, 1123)

---

- **2026-05-04 (밤, 후속) — rustfmt baseline cleanup (C3, C4 해결)**

  AMK_DEBTS.md C3 (rustfmt 90+ 파일 unformatted) + C4 (`docs.rs:92,94` trailing whitespace) 정리. 사용자 결정 = 옵션 1 (cleanup 즉시).

  ## 작업

  1. `src/docs.rs:92, 94` trailing whitespace 수동 제거 (rustfmt internal error 원인 해소)
  2. `cargo fmt --all` 재실행 → exit=0 (성공)
  3. 검증:
     - `cargo fmt --check --all` exit=0 (baseline 통과)
     - `cargo check --locked --workspace` exit=0
     - `cargo clippy --lib --bins --locked -- -D warnings` exit=0

  ## 변경 규모

  **95 파일 / rustfmt 자동 포맷 + docs.rs trailing whitespace 1줄 수동 fix**.
  의미 변경 0 (코드 동작 동일, 단순 whitespace/import/brace 자동 정리).

  - `src/api/*` 79 파일
  - `src/external/*` 7 파일
  - `crates/crypto/*` 3 파일
  - `src/types.rs / main.rs / lib.rs / config.rs / bin/rekey_encryption.rs` 5 파일
  - `src/docs.rs` 1 파일 (trailing whitespace 수동 fix)

  ## AMK_DEBTS 갱신

  - C3 rustfmt baseline → ✅ 해결
  - C4 docs.rs trailing whitespace → ✅ 해결
  - PR #205 의 backend job 이제 cargo fmt --check 통과 예상

  ## 다음 단계

  cargo fmt 정책 영구 강제 정착 = pr-check.yml 의 cargo fmt --check --all step 이미 활성. 향후 새 unformatted 코드 머지 차단.

---

- **2026-05-04 (밤) — AMK_DEBTS.md 정합성 검증 + 정정 + 신규 부채 12건 등재**

  사용자 지시: 부채 카탈로그 (어제 작성) 의 사실관계 정확성을 5 독립 agent 분담 검증 + 추가 미점검 영역 조사 (경로 1+2 = 약 4시간 작업).

  ## 검증 결과 (이전 95% → 약 98% 도달)

  ### (1) 라인 번호 stale 다수 (M-007 사고)
  AMK_STATUS §8.2 검증된 리스크 표 + AMK_DEBTS 의 라인 번호 모두 stale. AMK_DEBTS 작성 시 `AMK_STATUS` 에서 직접 검증 없이 복사한 결과.

  정정 (HEAD 2026-05-04 기준):
  - `deploy.yml:87-98` → **L92-103** (PADDLE_*)
  - `AMK_DEPLOY_OPS.md:819` → **L985** (Webhook Secret)
  - `AMK_DEPLOY_OPS.md:781` → **L947** (KYB)
  - `AMK_DEPLOY_OPS.md:857` → **L1023** (SPF)
  - `ebook/service.rs:51,261,502,516,525,605,620,629` → **L63, 381, 627, 641, 650, 731, 746, 755** (8곳, 모두 stale)
  - `config.rs:97 SSL` → **L109-110** (DATABASE_URL localhost 기본)
  - `config.rs:101 Redis` → **L113**
  - `config.rs:325 세션 TTL` → **L91 + L375-378** (`EBOOK_SESSION_TTL_SEC`)
  - `auth/service.rs:397, 1396` (unwrap) → **L358 + L1123**
  - `video/repo.rs:237` (TODO) → **L233**

  ### (2) 카운트 정정
  - **B3 npm 취약점**: 2건 (postcss XSS) → **3건** (postcss + follow-redirects + basic-ftp **HIGH**)
  - **C1 ESLint 카테고리**: react-refresh 다수 → **react-hooks 25건 > react-refresh 7건** (분류 정정)
  - **C2 lint:ui 위치**: textbook_order_page.tsx 4곳 → **2곳** (총 9건 카운트 동일)
  - **J Secrets**: `.env.example` **57 → 65건**, `deploy.yml` secrets 22 + heredoc env 11 = **33건**

  ### (3) 부채 상태 변경 (해결됨)
  - **C5 enum sqlx::Type derive**: 보류 #13 → **이미 적용 완료** (`src/types.rs` 에 `#[sqlx(type_name)]` **36건**). AMK_STATUS §8.2 #13 행에 ~~취소선~~ + 해결 표시.

  ### (4) 신규 부채 12건 등재 (경로 2 추가 조사 발견)

  **A 운영/배포 신규 8건** (AMK_STATUS 미등재 운영 인프라 부채):
  - A4-1 nginx HTTPS 미활성 (HSTS 미설정)
  - A4-2 Let's Encrypt + certbot 자동 갱신 정책 부재 (90일 만료 위험)
  - A4-3 EC2 디스크 모니터링 자동화 부재
  - A4-4 DB/Redis 백업 정책 부재 (DR 0)
  - A4-5 Docker log 로테이션 미설정
  - A4-6 Cloudflare DNS / Email Routing 운영 정책 미문서화
  - A4-7 nginx Rate Limiting 모니터링 부재
  - A4-8 Docker base image 자동 업데이트 정책 부재

  **B 보안 신규 1건**:
  - B6 ipgeo HTTP-only (`src/external/ipgeo.rs`) — ip-api.com 평문 HTTP, 중간자 공격 위험. E-9 (GeoIP 전환) 와 통합 가능.

  **C 코드 품질 신규 6건** (Rust/TS 룰 회피 카운트):
  - C8 `#[allow(dead_code)]` **33건**
  - C9 `#[allow(clippy::*)]` **11건**
  - C10 `#[allow(unused_imports)]` **8건**
  - C11 `#[allow(unused_assignments)]` 1건
  - C12 TypeScript `any` **3건**
  - C13 TypeScript `eslint-disable` 인라인 **11건**
  - 안전 (참고): Rust `unsafe` **0건** ✅, TypeScript `@ts-ignore` **0건** ✅

  **E 기능 신규 3건** (다른 docs 미구현 항목):
  - E-FUTURE-1 콘텐츠 시딩 Phase 2/3 (`AMK_API_FUTURE.md`)
  - E-FUTURE-2 발음/조음/TTS 평가 (`AMK_API_FUTURE.md`)
  - E-TEXTBOOK-1 SpeechSuper API 프로토타이핑 (`AMK_API_TEXTBOOK.md`)

  **G 자동 검증 신규 5건**:
  - G10 src/ 테스트 부족 (4건만, `crates/crypto` 46건 OK)
  - G11 cargo-deny 미설치 (라이선스 검증 X)
  - G12 cargo-geiger 미설치 (unsafe 0건이라 우선순위 낮음)
  - G13 `.github/CODEOWNERS` 미존재
  - G14 PR template / issue template 미존재

  **J Secrets 정합성 신규 4건**:
  - **J1 (위험 잠재) `RATE_LIMIT_TEXTBOOK_*` config.rs `expect()` panic 사용 + .env.example 미정의 + deploy.yml 미명시** = INC-001 패턴 잠재 (production 배포 시 환경변수 부재 → expect panic → crash)
  - J2 `APPLE_CLIENT_ID/TEAM_ID` config.rs Option 처리 (panic X) + .env.example 미정의 (Apple OAuth 미구현)
  - J3 정합성 검증 자동 도구 X (deploy.yml/.env.example/config.rs 3중 동기화 수동)
  - J4 panic 게이트 추가 시 동기화 누락 룰 강제 X

  ### (5) AI 사고 신규 등재 (M-006 + M-007)

  - M-006: `cargo fmt --check --all` 결과 의미 잘못 해석 (exit=0 만 보고 통과 단정, 출력 diff 무시)
  - M-007: 다른 문서 라인 번호 직접 검증 X (AMK_STATUS 에서 복사 → stale)
  - 카테고리 분포 갱신: 추정 단정 = M-002, M-005, M-006, M-007

  ## 변경 파일

  - `docs/AMK_DEBTS.md` 전체 재작성 (정정 + 신규 12건 + 라인 번호 사용 정책 명시)
  - `docs/AMK_AI_MISTAKES.md` M-006 + M-007 등재 + 카테고리 분포 갱신
  - `docs/AMK_STATUS.md §8.2` 검증된 리스크 표 라인 정정 + #13 해결 표시
  - 본 CHANGELOG 엔트리

  ## 검증

  - 정정된 라인 번호 spot 재검증 (4건) — 모두 정확 (deploy.yml:92-103, ebook/service.rs:63, auth/service.rs:358, video/repo.rs:233)
  - cargo fmt --all working tree 90+ 파일 변경은 별도 결정 대기 (본 commit 미포함)

  ## 다음 단계 (사용자 결정 필요)

  - cargo fmt PR 처리 (옵션 1 폐기 vs 옵션 2 cleanup) — working tree 잔존
  - 신규 발견 우선 처리 후보: J1 RATE_LIMIT_TEXTBOOK 동기화 (INC-001 패턴 차단), B3 postcss/follow-redirects/basic-ftp `npm audit fix`, B1 rustls-webpki upgrade
  - Q16 / Q17 큐 진입 시점

---

- **2026-05-04 (저녁) — Dormant 정책 일괄 조사 + rustfmt 추가 + lint:ui 임시 비활성 + AI 사고 기록 SSoT 신규**

  사용자 지시: "도입됐으나 자동 강제 안 되는 정책" 일괄 조사. 발견 5종 (lint, lint:ui, rustfmt, cargo test, e2e). baseline 통과 1종 (rustfmt) 즉시 활성, 누적 부채 2종 (lint+lint:ui = 36 errors) Q16 단일 트랙 묶음, CI 셋업 필요 2종 (cargo test, e2e) Q17 별도.

  ## (1) `.github/workflows/pr-check.yml` 변경 2건
  - **신규**: backend job 에 `cargo fmt --check --all` step 추가. `rust-toolchain.toml` 에 rustfmt components 등재됐으나 자동 강제 부재로 dormant 였음. main HEAD baseline 통과 확인 (`exit=0`) 후 즉시 활성화.
  - **임시 비활성**: frontend job 의 `npm run lint:ui` 에 `continue-on-error: true` 추가. 첫 자동 실행 시 9건 누적 위반 검출 (디자인 토큰 결정 필요 = Q16 트랙). cleanup 후 제거.

  ## (2) `docs/AMK_STATUS.md §8.2`
  - **Q16 확장**: ESLint 27 errors + lint:ui 9 errors = 합 36 errors 단일 트랙으로 묶음. (a) shadcn/ui 컴포넌트 파일 분할, (b) react-hooks 룰 위반 fix, (c) 디자인 토큰 결정 + 9곳 색상 교체. 추정 1-2일.
  - **Q17 신규**: cargo test (DB 의존 CI 셋업 필요) + playwright e2e (브라우저 + CI 분 큼) 도입 검토. 우선순위 = Q16 완료 후.

  ## (3) `docs/AMK_AI_MISTAKES.md` 신규 (별도 SSoT)

  사용자 결정 (2026-05-04): "사고 회피 룰/훅 도입은 100% 확증 안 되면 X. 대신 사고 기록 → 작업 시 사전 참조 패턴 채택 (룰 무한 루프 회피 + 사용자 스트레스 감소)".

  - 본 문서 = AI 작업 실수 SSoT. 사실만 기재 (가정/해석 배제), 원인/결과 분리.
  - 오늘 사고 5건 등재 (M-001 ~ M-005). production 인시던트 (INC-001~005) 와 별도 카테고리.
  - 카테고리 분포: (a) 사전 상태 미확인 = M-001/M-003/M-004, (b) 추정을 사실로 단정 = M-002/M-005.
  - 메모리 `feedback_ai_mistakes_reference.md` 신규 (포인터만, 사고 내용 복붙 X). 작업 시작 전 본 문서 참조 강제.

  ## 검증

  - 로컬 `cargo fmt --check --all` 통과 (exit=0)
  - 로컬 git status clean (AMK_AI_MISTAKES.md 만 untracked, 본 commit 에 포함)

  ## 다음 단계

  - 본 PR (KKRYOUN push) → pr-check.yml 자동 실행 → 새 backend step (cargo fmt --check --all) 통과 확인 = baseline 검증 끝
  - 통과 시 사용자 머지
  - Q16 작업은 별도 PR (디자인 결정 + baseline cleanup 1-2일)
  - Q17 = Q16 완료 후 검토

---

- **2026-05-04 (오후, 후속) — pr-check.yml self-test fail 대응: clippy 1줄 fix + lint 일시 비활성화**

  PR #205 push 직후 self-test 결과 = backend FAILURE + frontend FAILURE. 본 PR 코드 변경 0 = **기존 main baseline 의 lint 위반이 누적된 상태였던 것이 새 워크플로 검증으로 노출**.

  **자가 진단 (제 잘못)**: 워크플로 도입 전 main baseline 가 `cargo clippy / npm run lint` 모두 통과하는지 사전 검증 안 함. INC-005 학습 의 "정책 도입 전 의도 fail 케이스 검증" 룰은 잡았지만 그 inverse (정책 통과 baseline 확인) 는 빠뜨림. 사고 패턴 반복 — 메모리 강화 (별도 `feedback_pre_action_validation.md` + 훅 도입 검토 진행 중).

  ## fail 내역

  | Job | step | 위반 |
  |-----|------|------|
  | backend | cargo clippy | `useless_conversion` 1건 (`src/api/auth/service.rs:192`). Rust 1.95 신규 룰 (어제 1.94 시점엔 통과) |
  | frontend | npm run lint | ESLint 27 errors. 대부분 `react-refresh/only-export-components` (shadcn/ui 컴포넌트 + variants 같은 파일 export 패턴 vs 새 룰), 일부 `react-hooks/set-state-in-effect` / `react-hooks/incompatible-library` / `prefer-const` |
  | frontend | lint:ui (하드코딩 색상) | ✅ PASS |

  ## 즉시 조치

  1. **Backend clippy fix** — `src/api/auth/service.rs:192` `.into_iter()` 1줄 제거 (zip() 가 IntoIterator 받음)
  2. **`pr-check.yml` 의 `npm run lint` 단계** = `continue-on-error: true` 임시 적용. 결과 표시되나 workflow status 는 fail 처리 X
  3. **Q16 신규 큐잉** (`docs/AMK_STATUS.md §8.2`) — Frontend ESLint baseline cleanup. 작업 = shadcn 컴포넌트 파일 분할 + react-hooks 위반 fix + prefer-const fix. 추정 1-2일. 완료 후 `continue-on-error` 제거

  ## 잡힌 가치 (역설적)

  본 워크플로 안 만들었으면 27 errors + 1 clippy 가 계속 누적되고 있었음. **워크플로 = 기존 부채 발견 도구로 정상 작동**. 다만 도입 시점에 한 번에 fix 어려운 양이라 baseline cleanup 별도 트랙 분리 (Q16).

  ## 학습

  - 정책 도입 시 fail 케이스 (의도 fail) 검증 + **통과 케이스 (현재 baseline)** 검증 둘 다 필요
  - lint baseline 누적은 자동 검증 부재의 자연 결과 — 도입 첫 날엔 부채 발견량 클 수 있음
  - 대량 위반 발견 시 즉시 전수 fix vs 임시 우회 + 별도 트랙 = 작업 분할 판단 (이번엔 후자)

---

- **2026-05-04 (오후) — PR 검사 워크플로 신규 (`.github/workflows/pr-check.yml`) — INC-005 학습 후속**

  ## 배경

  INC-005 fix (2026-05-04 오전) 로 `deploy.yml` trigger = `branches: [main]` 단일화. 그 결과 **KKRYOUN push 시 자동 검증 사라짐** = 컴파일/타입/lint 에러를 머지 후 deploy 단계 (Docker build) 에서야 발견하는 검증 갭 발생.

  사용자 결정: 옵션 A (PR 검사 워크플로 신규) 채택. 이유 = 검증 책임을 사람 → CI 로 이전, 1인 환경 인지 부담 감소, 머지 후 deploy fail 부담 회피.

  ## 변경

  **`.github/workflows/pr-check.yml` 신규**

  | Job | 명령 | 목적 |
  |-----|------|------|
  | `backend` | `cargo check --locked --workspace` | 컴파일 + Cargo.lock 정합성 (SQLX_OFFLINE=true, .sqlx/ 쿼리 캐시) |
  | `backend` | `cargo clippy --lib --bins --locked -- -D warnings` | lint + warning fail-closed |
  | `frontend` | `npm run build` (= `tsc -b && vite build`) | 타입 체크 + 빌드 |
  | `frontend` | `npm run lint` | ESLint |
  | `frontend` | `npm run lint:ui` | 하드코딩 색상 검사 (Tailwind 토큰화 정책) |

  **트리거**: `push: [KKRYOUN]` + `workflow_dispatch`.

  **concurrency**: `pr-check-${{ github.ref }}` 그룹 + `cancel-in-progress: true` — 같은 ref 에 빠른 연속 push 시 이전 실행 자동 취소 (CI 분 절약).

  **캐시**:
  - Rust: `Swatinem/rust-cache@v2` (target/ + ~/.cargo)
  - Node: `actions/setup-node@v4` 의 `cache: 'npm'` (cache-dependency-path = `frontend/package-lock.json`)

  ## 책임 분리 원칙

  | 워크플로 | 책임 | 트리거 |
  |---------|------|--------|
  | `deploy.yml` | EC2 배포 (Docker build + push + SCP + SSH + health check) | `push: [main]` |
  | `pr-check.yml` | 머지 전 검증 (backend cargo + frontend build/lint) | `push: [KKRYOUN]` |

  INC-005 학습 = 한 워크플로에 다중 책임 (배포 + 검증) 두면 트리거 충돌 + race condition 위험. **별도 파일로 영구 분리**.

  ## 효과 + 한계

  **잡을 수 있는 사고**:
  - 컴파일 에러 머지 (cargo check fail) → Docker build 단계 도달 전 차단
  - 타입 에러 머지 (tsc fail in npm run build) → frontend 깨진 상태 머지 차단
  - clippy warning 머지 (워닝도 fail) → 코드 품질 일관성
  - 하드코딩 색상 머지 (lint:ui fail) → Tailwind 토큰화 정책 강제

  **잡지 못하는 사고** (별도 트랙 필요):
  - INC-005 패턴 (병렬 PR migration 비대칭) — 이미 deploy.yml fix 로 막힘
  - INC-004 패턴 (적용된 migration 파일 수정) — sqlx checksum 문제, 본 PR 검사로는 감지 불가
  - 런타임 panic — 테스트 부재 (cargo test 도입 별도 검토 필요)

  ## 비용

  - GitHub Actions 분: 본 워크플로 평균 ~3-5분 (캐시 hit 시 1-2분). KKRYOUN push 빈도 = 일 평균 5-10회 추정 → 월 ~150-500분 추가
  - 무료 한도 2,000분/월 안에서 운영 가능 (deploy.yml 평균 3-4분 × 일 머지 1-3회 = 월 ~100-360분 + pr-check ~150-500분 = 합계 ~250-860분)

  ## 문서/메모리 갱신

  - `docs/AMK_DEPLOY_OPS.md §5` workflow 인벤토리 표 + §7 CI Gate 표
  - `memory/feedback_deploy_env_sync.md` PR 검사 정책 섹션 추가
  - 본 CHANGELOG 엔트리

  ## 다음 단계 (트리거 시)

  - 머지 후 KKRYOUN 의도 fail 케이스 (예: cargo check 깨지는 한 줄) push → pr-check fail 확인 → 정책 검증
  - 옵션 C (branch protection + required status check) 도입 여부는 사용자 1인 환경 force push 자유도 vs 강제 트레이드오프 판단 (현재 보류)

---

- **2026-05-04 (오전) — 🚨 INC-005 사후 기록 + deploy.yml KKRYOUN 트리거 제거**

  ## 사고 개요

  **2026-05-03 14:39 ~ 15:38 KST (약 1시간) production 다운**. nginx 502 응답. 사용자 인지 전 PR #201 머지 deploy 로 자가 복구.

  ## 타임라인 (UTC, 한국시간 +9h)

  | UTC | 한국 | 이벤트 | 결과 |
  |------|------|--------|:--:|
  | 02:44:53 | 11:44 | KKRYOUN `195e997` push (TextbookLanguage 14 + migration `20260503_textbook_language_expand.sql`) → **KKRYOUN deploy 트리거** | ✅ deploy SUCCESS, **production DB `_sqlx_migrations` 에 `20260503` row 적재됨** |
  | 05:36:16 | 14:36 | KKRYOUN `f4e0519` push (docs only) → KKRYOUN deploy | ✅ SUCCESS |
  | 05:36:43 | 14:36 | main `934f567` (PR #202 머지, ai 측 `i18n/phase2-it-lei` 브랜치, **migration `20260503` 파일 미포함**) → main deploy | 🔴 **FAILURE** |
  | ~05:39 | ~14:39 | amk-api crash loop (sqlx panic) → nginx 502 | **🚨 production 다운 시작** |
  | 06:34:25 | 15:34 | main `93a6fdd` (PR #201 머지, migration 파일 포함) → main deploy | ✅ SUCCESS |
  | ~06:38 | ~15:38 | api 컨테이너 정상 기동, /health 200 | **자가 복구** |

  ## 근본 원인 (Root Cause)

  `.github/workflows/deploy.yml` trigger = `branches: [main, KKRYOUN]`. KKRYOUN push 가 production 으로 직접 배포되는 부수 효과.

  메커니즘:
  1. KKRYOUN deploy → migration 적용 → production DB 에 `20260503` row 적재
  2. ai 측 PR #202 (별도 feature 브랜치 `i18n/phase2-it-lei`) main 머지. **PR #202 의 commit base 시점에 KKRYOUN 의 migration 파일 미반영** → main HEAD `934f567` 에 migration 파일 없음
  3. main push → deploy → 새 컨테이너 시작 → `sqlx::migrate::run` → DB 의 `20260503` row 검사 → 코드의 `migrations/` 디렉터리에 파일 없음 → **"Error: migration 20260503 was previously applied but is missing in the resolved migrations"** → panic
  4. amk-api restart loop → nginx upstream `api:3000` 응답 불가 → 502
  5. PR #201 머지 (migration 파일 포함) → deploy → 정상화

  ## sqlx 에러 의미

  ```
  Error: migration 20260503 was previously applied but is missing in the resolved migrations
  ```

  = "DB `_sqlx_migrations` 테이블에 `20260503` row 가 있는데, 현재 deploy 된 코드의 `migrations/` 디렉터리에 그 파일이 없다 → fail-closed (절대 자동 진행 X)"

  ## 비교: INC-004 vs INC-005

  | | INC-004 (2026-04-28) | INC-005 (2026-05-03) |
  |---|---|---|
  | 원인 | 이미 적용된 migration 파일 **수정** | 다른 브랜치 deploy 가 migration 적용 + 다른 PR 가 migration 없이 main 머지 |
  | sqlx 에러 | checksum 불일치 | "previously applied but missing" |
  | 트리거 | 주석 수정 commit | KKRYOUN deploy + 병렬 main 머지 (다른 PR) |
  | 다운타임 | 약 8분 (즉각 hotfix) | 약 1시간 (자가 복구 대기) |
  | 사용자 인지 | 즉시 | 미인지 (자가 복구로 수습) |

  ## 수정 (본 PR 묶음에 포함)

  **`.github/workflows/deploy.yml`**: trigger `branches: [main, KKRYOUN]` → `[main]`. 주석으로 변경 사유 + INC 참조 명시.

  효과:
  - KKRYOUN push = 더 이상 production 안 건드림
  - PR 머지 시점 (main push) 에만 deploy
  - INC-005 같은 병렬 deploy 경로 race 영구 차단
  - PR 페이지에서 build-and-push/deploy status check 사라짐 (cargo check 등 로컬 검증 + main push 후 deploy 결과로 판정)

  비용: 거의 0 (KKRYOUN push deploy 는 어차피 PR 머지 시 main deploy 와 중복이었음)

  ## 부수 사고: 본 세션의 reset --hard 우발적 데이터 손실

  본 INC-005 fix 작업 진입 시점에 KKRYOUN sync 절차로 `git reset --hard origin/main` 실행. 이때 working tree 의 미커밋 변경 3건 (`AMK_CHANGELOG.md`, `AMK_EBOOK_SECURITY.md`, `AMK_STATUS.md`) 폐기. 다른 세션이 대화 컨텍스트 + 생존한 memory (`reference_toonrader.md`) 기반으로 재작성, 본 세션이 즉시 commit (`00a7ec0`) 으로 보호. **재발 방지 학습 = `feedback_migration_safety.md` §INC-005 + 본 엔트리 하단 명시**.

  ## 재발 방지 (학습 정리)

  1. **destructive git 명령 (reset --hard, push --force, branch -D 등) 전 `git status` 필수** — 미커밋 변경 stash 또는 commit 후 진행
  2. **memory (`~/.claude/...`) = 별도 디스크라 git 사고 영향 없음** → 작업 컨텍스트 / 결정 / 외부 조사 결과는 memory 우선 저장 (이번 사고에서 `reference_toonrader.md` 가 복구 핵심 자산)
  3. **migration 파일 추가 PR 은 즉시 main 머지 우선** — KKRYOUN 위에 미머지 migration 있는 동안 다른 PR 머지 차단 (병렬 deploy 경로 막혔어도 신중하게 운영)
  4. **deploy 트리거는 main 단일 경로** — feature 브랜치/working 브랜치 deploy 금지. 긴급 시 `workflow_dispatch` 수동 실행

---

- **2026-05-04 (오전) — 조사: 네이버웹툰 TOONRADER 벤치마크 + e-book 보안 옵션 A 권고 (2026-05-03 작업 재작성)**

  > 본 엔트리는 2026-05-03 밤 최초 작성됐으나 2026-05-04 10:15 KKRYOUN→origin/main `git reset --hard` 시 미커밋 워킹 트리가 함께 사라져 손실. 대화 컨텍스트 + `memory/reference_toonrader.md`(생존) 기반으로 결정적 재작성. 코드 변경 없음, 문서 + memory 갱신만.

  **TOONRADER 핵심**: 네이버웹툰 자체 개발 안티-파이러시 (2017.07~). 비가시적 워터마크 + AI 이미지 매칭(2018) + 도용계정/자동화/행동 패턴 탐지(2025~2026 고도화) + DMCA·CDN 소환장 워크플로. 2026-04-22 발표 성과: 24h 내 유출 작품 약 90% 감소, 유료 결제액 평균 23% 증가, 유출 시도 계정당 평균 회차 1/10.

  **갭 분석**: AMK 는 워터마크 ✅ (LSB + 마이크로도트 + 풋터 + 접근로그 4중) / 단일 세션 강제 ✅ / accesslog 적재 ✅ 로 4축 중 워터마크·세션·로그는 보유. 부재 = (1) 자동화 봇/헤드리스 탐지, (2) 비정상 행동 패턴 시계열 분석, (3) 다중 IP 동시 접속 알림, (4) 외부 유출본 매칭 워크플로.

  **검토 옵션 (T2 방향 승인 요청)**:
  - **옵션 A (권장, 사용자 트리거 대기)** — 행동 기반 봇 탐지: 다중 IP/UA 감지 + 페이지 요청 간격 통계 + 헤드리스 시그널 (`navigator.webdriver` + WebGL/canvas 핑거프린트). 1~2일, 기존 Rate Limit/세션 인프라 재사용. 우리 갭 중 가장 명확하고 TOONRADER 2025~2026 고도화 핵심과 일치
  - **옵션 B (보류)** — 외부 유출 모니터링 (Google 검색 알림 + 워터마크 ID 매칭 cron): 3~5일. 1인 운영 한계로 부분적 효과만, 외부 사이트 적발 발생 후 재검토
  - **옵션 C (비권장)** — AI 이미지 매칭: 콘텐츠 규모(수십 권) 대비 과투자, 단속 대상 데이터 거의 없음

  **결정**: 옵션 A 사용자 트리거 대기 (Q15). 옵션 B/C 보류.

  **문서 갱신**:
  - `docs/AMK_EBOOK_SECURITY.md` §2.5 신규 (TOONRADER 기술 구성 + 성과 + 갭 분석 + 출처 6) / §1.2 표 #7-#8 (행동 기반 봇 탐지 + 다중 IP 감지) / §4 Phase 1-6 (옵션 A 추가) / §5 한 줄 (벤치마크 격차)
  - `docs/AMK_STATUS.md §8.2` Q15 신규 (사용자 트리거 대기)
  - 본 CHANGELOG 엔트리 + title `updated:` 갱신

  **메모리 (2026-05-03 작성, reset 영향 없이 생존)**:
  - `reference_toonrader.md` — 외부 사이트 6 + 핵심 데이터 + 갭 영역 (다음 e-book 보안 검토 시 진입점)
  - `project_status.md` 옵션 A 결정 대기 한 줄
  - `MEMORY.md` 인덱스 reference 섹션 1줄

  **재발 방지 (2026-05-04 신규 학습)**: 작업 완료 후 즉시 `git add` + 커밋. KKRYOUN 위에서 작업 후 reset 전 반드시 미커밋 변경 점검. 메모리는 `~/.claude/...` 별도 디스크라 git 사고 영향 없음 — docs/code 만 reset 으로 손실됐고 memory 가 복구의 핵심 자산이 됨 (이번 사고로 검증).

  **다음 단계 (사용자 트리거 시)**: Q15 진입. 백엔드 = Redis 세션 메타에 `last_ip` / `last_ua` / `req_intervals` 추가 + 다중 IP 알림 endpoint. 프론트 = `navigator.webdriver` + WebGL/canvas 핑거프린트 검사 + 헤더 추가.

---

- **2026-05-03 (저녁) — 결정: e-book 페이지 이미지 저장 위치 정책 (학습 콘텐츠 인프라)**

  books-api-bridge plan §3 Stage 2 #3-B (WebP 인프라 업로드) 진입 전 인프라 결정. 코드 변경 없음, docs + memory 만 갱신.

  **결정**: 옵션 A 채택 — **RDS 이전 전까지 EC2 local fs 정책 유지** (사용자 결정 2026-05-03).

  **검토한 옵션**:
  - **A. 지금 EC2 dir 업로드** — 즉시 동작, rsync 30분, api 코드 변경 0, books-api-bridge §3 Stage 2 즉시 진행 가능. 단점: RDS 이전 시 한 번 더 옮겨야 함, EC2 디스크 693MB 점유, 업로드 자동화 스크립트 EC2 전용
  - **B. RDS 이전 + Q9 (S3 전환) 완료 후 업로드** — 한 번에 끝, S3 + CDN 통합. 단점: 약 1.5개월+ 대기 (모바일 ~23일 + 데스크탑 7.5일 + RDS 이전 5일 + Q9 3-5일), e-book 출시 지연

  **A 채택 근거**:
  1. 1인 CEO 환경에서 1.5개월+ 대기 비용 > 한 번 더 옮기는 비용
  2. e-book 36 lang catalog 활성으로 즉각적 사용자 가치 + 피드백 루프 시작
  3. 마이그 비용 작음 = `aws s3 sync` 1회 (693MB / 5분 미만)
  4. Q9 (E-book 로컬 fs 의존 해소) 가 이미 CRITICAL 리스크 등록 — ebook 도메인 9곳 fs::read 전환은 어차피 필수. WebP 디렉터리 추가 = 같은 작업의 1줄 확장
  5. 업로드 자동화 스크립트는 destination 만 바꿔서 재사용 가능 (지금 작성 = 미래 자산)
  6. 검증된 패턴 — ebook 도메인 EC2 local fs 읽기로 이미 동작 (`docs/textbook/page-images` 기본값)

  **운영 리스크 + 완화**:
  - 디스크 압박 (693MB + 향후 콘텐츠) → `AMK_STATUS §8.4 #8` 주 1회 모니터링 등재
  - 단일 장애점 (백업 부재) → 일일 S3 cold storage 백업 스크립트 별도 트랙 검토
  - 업로드 자동화 부재 → books 측 빌드 후 books → EC2 동기화 (`rsync` 또는 cron) 스크립트는 books 세션 작업 범위

  **RDS 이전 시점 전환 트리거** (`AMK_STATUS §8.2 검증된 리스크` Q9):
  - ebook 도메인 9곳 `fs::read` → S3 SDK 호출 전환
  - `${EBOOK_PAGE_IMAGES_DIR}` 데이터 → S3 bucket 1회 `aws s3 sync`
  - 환경변수 `EBOOK_PAGE_IMAGES_DIR` → `EBOOK_PAGE_IMAGES_S3_BUCKET` (가칭) 전환
  - CloudFront signed URL 보안 강화 검토

  **문서 갱신**:
  - `docs/AMK_API_EBOOK.md` "페이지 이미지 저장 위치 정책" 섹션 신규 (정책 본문 SSoT)
  - `docs/AMK_DEPLOY_OPS.md §6` E-book 페이지 이미지 모니터링 가이드 + RDS 이전 트리거
  - `docs/AMK_STATUS.md §8.4 #8` 주 1회 모니터링 항목 등재 / §8.2 큐에 Q14 추가 (사용자 트리거 대기)
  - 본 CHANGELOG 엔트리

  **메모리 갱신** (api 세션):
  - `project_decisions.md` 2026-05-03 결정 추가
  - `project_ebook_webp_upload.md` 신규 (시점/조건/리스크 context)

  **다음 단계 (사용자 트리거 시)**: Q14 진입. EC2 디스크 여유 확인 → books 세션에서 동기화 스크립트 작성 → 업로드 → catalog endpoint 36 lang `available=true` 자동 활성 검증.

---

- **2026-05-03 (오후) — PR #201 묶음 ②: Mac Mini 와 안 부딪치는 작업 4건**

  PR #201 의 font_loader hotfix (오전 작업) 위에 사용자 지시 "Mac Mini 와 안 부딪치는 일 즉시 진행" 으로 4건 추가.

  ## (1) Stale i18n leaf key 제거 (13 lang)

  `admin.textbook.detail` = `"Detail"` (단순 string) 이 13 신규 lang (am/ar/bn/fa/it/ky/lo/pl/sw/tl/tr/uk/ur) 에 stale 하게 남아 있었음. en.json 은 동일 키가 dict (12 sub-key: grossAmount/finalAmount/discountAmount/...) 구조인데 leaf string 이 dict 구조를 막아 i18next dict 접근 (예: `t('admin.textbook.detail.grossAmount')`) 시 fallback 못 함. 13 파일에서 leaf 제거 → en.json fallback 정상화.

  변경: 13 파일 / -13 라인 (+ 트레일링 newline 정상화 13건).

  ## (2) books-api-bridge §3 Stage 1 #1 — TextbookLanguage enum 14 추가

  Plan SSoT (`~/.claude/plans/books-api-bridge.md`) §3 Stage 1 #1 진행. textbook + ebook 도메인 공유 enum 21 → 35 확장.

  **migration**: `migrations/20260503_textbook_language_expand.sql` — 14 ALTER TYPE ADD VALUE IF NOT EXISTS (am, ar, bn, es_es, fa, it, ky, lo, pl, pt_pt, sw, tr, uk, ur). `supported_language_enum` 동일 표기 체계 (snake_case in DB, BCP 47 'es-ES'/'pt-PT' in serde).

  **코드**:
  - `src/types.rs` `TextbookLanguage` enum 14 신규 variant (es_es/pt_pt 는 EsEs/PtPt + sqlx/serde rename)
  - `to_purchase_code()` + `Display` impl 14 신규 매핑
  - `src/api/textbook/service.rs`: `language_display_name()` (한국어 표시명) + `catalog_languages()` 5-tuple 14 추가. 신규 14 = `available=false, isbn_ready=false` (출판본 미준비, catalog 노출은 OK 주문은 차단)
  - `src/api/ebook/service.rs`: `to_code()` (DB enum → 디렉터리 경로) + `catalog_languages()` 3-tuple 14 추가. ebook catalog 응답은 `${EBOOK_PAGE_IMAGES_DIR}/{edition}/{lang}/manifest.json` 부재 시 자동 `available=false` (Stage 2 인프라 업로드 후 활성)

  **검증**: `cargo check` 18.34s 클린 / `cargo clippy --lib --bins -D warnings` 19.09s 클린 / sqlx prepare 캐시 영향 없음 (enum 값 추가는 query analysis 외).

  **마이그레이션 안전성** (`feedback_migration_safety` 적용): enum ADD 만, 기존 값 변경 X — 이미 발행된 textbook_orders 행 안전. 파일명 `20260503` (마지막 마이그 `20260428` 다음 날짜, 정수 버전 충돌 없음). 본 마이그 적용 후 로컬 DB 반영 필요: `cd /home/kkryo/dev/amazing-korean-api && sqlx migrate run` (또는 docker compose 재시작 시 자동 적용).

  ## (3) books-api-bridge plan 갱신 (진단 정정 + 신규 발견)

  Plan §1 진단표 2026-04-30 → 2026-05-03 변동 컬럼 추가:
  - #1 갭 ✅ 본 PR 로 진행
  - #2 갭 (locale 파일) 13/15 진행 (es-ES/pt-PT 잔여)
  - #3 갭 ✅ books 측 2026-05-01 완료 (8,928 페이지 / 693MB)
  - #2-1 신규 (`SUPPORTED_LANGUAGES` 활성 분리)
  - **#2-2 신규 발견**: 22 base lang × **518 keys 누락** (62.6% 완성도). Q1~Q12 시기 추가된 admin/study/textbook 키 미반영. Mac Mini Wave 1 Phase 1c (가칭) 트랙 큐잉.

  ## (4) docs 동기화

  - `docs/AMK_API_TEXTBOOK.md` 헤더 + Phase 12 라벨: 20언어 → 35언어 (신규 14 = `available=false`)
  - `docs/AMK_API_EBOOK.md` L504: TextbookLanguage 재활용 20개 → 35개
  - `docs/AMK_STATUS.md` Q13 행 후속 작업 segment 추가 (본 PR 묶음 (a)/(b)/(c))
  - 본 CHANGELOG 엔트리

  **다음 진입 후보** (Mac Mini 와 충돌 없는 작업 잔여):
  - es-ES / pt-PT locale 파일 manual inline diff (api 세션 단독, 작은 작업)
  - SUPPORTED_LANGUAGES 13 신규 활성 (Q13 S5, ai 측 PR #199 머지 후)

  **Mac Mini 큐 항목** (사용자 → 맥미니 세션 핸드오프):
  - 22 base lang × 518 keys 신규 번역 (Wave 1 Phase 1c 가칭)
  - 13 신규 lang × 37 textbook discount keys (PR #185 era)
  - ai repo Mac-Mini 브랜치 PR push 정체 11일 — 작업 결과 ai repo 에 commit 누락 (Wave 1 SSoT 깨짐, 보완 필요)

---

- **2026-05-03 (오전) — Q13 Phase 2 hotfix (PR #193 → 새 PR): Nastaliq Urdu weight 500 미지원 정정**

  **배경**: 2026-04-30 작성한 PR #193 (font_loader Nastaliq fix + Gemini docs 일관성 + S6 도록 등록 9건) 이 머지되지 않은 채 그 사이 PR #194~#198 (5건) 이 별도 브랜치로 main 에 머지됨. PR #193 의 docs 가 그 흐름을 모르는 stale 상태가 되어 옵션 'B — 버리고 새로 작성' 채택 (사용자 결정 2026-05-03). PR #193 close + KKRYOUN reset --hard origin/main + 본 PR 작성.

  **그 사이 머지된 PR 요약** (origin/main 기준):
  | # | 제목 | 머지 |
  |:-:|------|:----:|
  | #194 | Phase 2 — 13 신규 lang 번역 + Wave 1 stale cleanup + common.goToSlide | 2026-05-01 |
  | #195 | Phase 2 recovery — gemma 실패 65 keys 재번역 (6 lang) | 2026-05-01 |
  | #196 | Phase 2 Option C Phase A — cross-lang 5건 사용자 결정 패치 | 2026-05-03 |
  | #197 | Phase 2 Option C Phase B — major/high 52건 합의 패치 | 2026-05-03 |
  | #198 | Phase 2 Option C Phase C — minor 2-LLM exact match 22건 자동 패치 | 2026-05-03 |

  > 위 PR 들은 ai 측 Wave 1 번역 결과 도착 + 검증 패치 흐름. 상세 변경 이력은 각 PR 본문 + ai 리포 핸드오프 문서 (`amazing-korean-ai/docs/AMK_AI_TRANSLATION_HANDOFF.md`) 참조. 본 api 측 책임은 머지 후 `SUPPORTED_LANGUAGES` 활성 (S5) — **아직 미실행**, RTL 여전히 dormant.

  **본 PR 코드 변경** (1 파일 / +2 -1):
  - `frontend/src/utils/font_loader.ts` — `ur` (Nastaliq Urdu) 폰트 weight `wght@400;500;700` → `wght@400;700` 정정. Google Fonts 의 Noto Nastaliq Urdu 는 400/700 만 지원 (500 누락된 요청은 잘못된 API 호출). 주석 추가. dormant 상태 (SUPPORTED_LANGUAGES 미포함) 라 즉시 영향 없으나 S5 활성 시점에 발현 위험 차단.

  **본 PR docs 변경**:
  - `docs/AMK_CHANGELOG.md` — 본 엔트리.
  - `docs/AMK_STATUS.md` Q13 행 — PR #194~#198 진행 + 본 hotfix 반영.
  - 메모리 — `project_status.md` 갱신, 날짜 2026-05-03 으로.

  **PR #193 잔여 가치 처리**:
  - **font_loader 1줄** → 본 PR 에 보존
  - **Gemini docs 일관성 3건 (단위 곳/라인 명시)** → 이미 PR #192 docs sync 머지본에 적용 완료 (불필요)
  - **plan §2.9.10 (S6 도록 등록 9건)** → plan 파일 그대로 보존 (별도 변경 불필요, S6 시점에 처리)

  **검증**: `npm run build` 11.19s 클린 / `npx tsc --noEmit` 0 error.

  **운영 노트**: 사용자 재확인 (2026-05-03) — **KKRYOUN 단일 브랜치 정책 유지**. PR #194~#198 이 `i18n/phase2-*` feature 브랜치로 머지된 건 정책 외 흐름 (ai 측 결과 반영 작업의 일부, 별도 세션). 본 PR 부터는 KKRYOUN 정상 운영 복귀. `feedback_git_branching` 메모리 그대로 유지.

---

- **2026-04-30 (밤, S4) — Q13 Phase 2 PR-D 데스크탑 마이그 (amazing-korean-desktop 동기화)**

  S3 (PR #191) 머지 직후 동일 세션 내 S4 진입. plan §2.9.7 그대로 실행. INC 0 건. `amazing-korean-desktop` 리포는 별도 (no remote, main 직접 커밋, 코드 복사 정책).

  **desktop 커밋**: `e63e20e` — `feat(i18n): Phase 2 logical properties 마이그 + LTR 보호 + RTL 인프라 (PR-D)`. 93 파일 / +624 -587. api PR-A + PR-B-pre 와 동일 패턴 (S1 sed + S2 LTR 보호 + S3 RTL 인프라) 한 묶음.

  **Desktop 분포 vs api**:
  | 항목 | api | desktop | 비고 |
  |---|---:|---:|---|
  | S1 directional total | 707 | 664 | 코드 복사 시점 차이 |
  | S2 input | 45 (20 파일) | 37 (~17 파일) | desktop = email 10/url 2/tel 1/number 16/password 8 |
  | S2 pre/code | 12 (9 파일) | 10 | |
  | S2 WritingTask | 4 main | 0 | desktop 미포함 |
  | S2 receipt | 6 span | 0 | desktop 미포함 |
  | S3 RTL 인프라 | 3 파일 / +37 | 3 파일 / +37 | 동일 |

  **WritingTask / receipt desktop 미포함 이유**: desktop 은 React 코드 복사 시점이 receipt_parts.tsx 와 study/writing/ 추가 이전. 그 결과 S2 수동 작업 0 (자동 sed 만으로 끝).

  **검증**:
  - `npm run build`: 8.20s 클린 (Tauri 메인 번들 1,631 kB, baseline 1,630 kB 와 거의 동일)
  - `npx tsc --noEmit`: 0 error
  - LTR 환경 회귀 없음 (Tailwind logical = dir 의존, RTL 분기 dormant)

  **다음**:
  - S5 (api): `SUPPORTED_LANGUAGES` 13 신규 entry 추가 (RTL 활성). ai 측 첫 RTL 번역 PR 머지 후.
  - S6 (PR-C): RTL 시각 검증 + 미보호 가격 표시 (toLocaleString 30+ 곳) helper component 도입 검토.
  - S7 (desktop): SUPPORTED_LANGUAGES 동일 추가. ai 측 데스크탑 RTL 번역 PR 후.

  **plan SSoT**: `~/.claude/plans/supported-language-es-pt-variants-expansion.md` §2.9.7.

---

- **2026-04-30 (밤, S3) — Q13 Phase 2 PR-B-pre 인프라 코드 추가 (RTL dormant)**

  PR #190 머지 직후 동일 세션 내 S3 진입. plan §2.9.6 그대로 실행. 자가 점검 8/8 통과. INC 0 건.

  **변경 (3 파일 / +37)**:
  - **`frontend/src/utils/font_loader.ts`** — `FONT_MAP` 에 6 lang entry 추가:
    - RTL 3종: `ar`/`fa` = Noto Sans Arabic, `ur` = Noto Nastaliq Urdu (Urdu 전용 명조체)
    - 기타 3종: `bn` = Noto Sans Bengali, `am` = Noto Sans Ethiopic, `lo` = Noto Sans Lao
    - `loadFontForLanguage` 는 lang 키 lookup 이라 매핑만 추가하면 자동 동작 (단 `SUPPORTED_LANGUAGES` 미포함 = 호출 안 됨 = dormant)
  - **`frontend/src/utils/language_groups.ts`** — `RTL = new Set(["ar", "fa", "ur"])` + `isRTL` 헬퍼 + `LANG_CLASSES` 에 `"lang-rtl"` 추가
  - **`frontend/src/i18n/index.ts`** — `applyLangClasses` 에 `if (isRTL(lang)) root.add("lang-rtl")` + `document.documentElement.dir = isRTL(lang) ? "rtl" : "ltr"` 추가. `isRTL` import.

  **dormant 의도**: `SUPPORTED_LANGUAGES` 배열 변경 안 함 → 드롭다운에 ar/fa/ur 노출 안 됨 → 사용자가 선택 불가 → 인프라만 준비, 활성화는 PR-B / S5 에서 ai 측 RTL 번역 PR 도착 후. 미리 SUPPORTED_LANGUAGES 추가하면 빈 locale 호출 → en fallback (UX 깨짐) 위험.

  **검증 게이트**:
  - `npm run build`: 9.57s 클린 ✅
  - `npx tsc --noEmit`: 0 error ✅
  - 기존 22 lang 회귀 영향 없음 (RTL 분기는 dormant 라 호출 안 됨)

  **다음**: S4 (PR-D 데스크탑 동일 logical 마이그) 진입 가능 또는 다음 세션 위임. S5 (SUPPORTED_LANGUAGES 13 신규 활성) 는 ai 측 첫 RTL 번역 PR 머지 후.

  **plan SSoT**: `~/.claude/plans/supported-language-es-pt-variants-expansion.md` §2.9.6.

---

- **2026-04-30 (밤) — Q13 Phase 2 PR-A 머지 완료 (S1+S2 / PR #190 / deploy success × 3)**

  PR #190 (5 커밋) 사용자 머지 완료 (`ddf89a9`, 2026-04-30 03:17:50 UTC). 머지 후 자동 deploy run `25145538635` SUCCESS. `/health` 200. INC 0 건. KKRYOUN ff sync 완료.

  본 세션 누적 deploy 3건 모두 SUCCESS:
  - run `25144272236` (5dd74db, S1+S2 4 커밋 푸시)
  - run `25144553659` (934cce7, docs 5/5 푸시)
  - run `25145538635` (ddf89a9, PR #190 머지 커밋)

  **다음**: S3 (PR-B-pre 인프라) 또는 S4 (데스크탑) 진입 가능 — 사용자 선택.

---

- **2026-04-30 (밤) — Q13 Phase 2 PR-A 푸시 완료 (S1+S2 / PR #190 / deploy success)**

  세션 단위 실행 계획 §2.9.4 (S1) + §2.9.5 (S2) 그대로 실행. 자가 점검 8/8 양 세션 모두 통과. INC 0 건.

  **PR #190**: https://github.com/AmazingKoreanCenter/amazing-korean-api/pull/190 (OPEN, MERGEABLE, 4 커밋, ~130 파일 / +647 -647)

  **S1 — Tailwind logical sed (`6d33267`, PR-A 1/4)**: frontend `src/` `.tsx`/`.ts` 안 directional class 12쌍 `perl -i -pe` word-boundary lookahead 일괄 치환 = 707곳 / 96 파일 / +580 -580.
  - `text-left`→`text-start` 168 / `text-right`→`text-end` 94 / `mr-N`→`me-N` 176 / `ml-N`→`ms-N` 67 / `pl/pr`→`ps/pe` 29 / `left/right-N`→`start/end-N` 49 / `rounded-l/r`→`rounded-s/e` 115 / `border-l/r`→`border-s/e` 9
  - `rounded-lg` (size) 와 `border-rose-500` (color) 등 false-positive 는 perl `\b...(?=(\b|-))` lookahead 로 정확히 거름 (잔여 grep 카운트는 `\b` 만 매칭하는 grep 한계로 표시).

  **S2 — 의도적 LTR 영역 보호 (3 커밋 분할, plan 옵션 C 채택)**:
  - **input type LTR (`5efaaa5`, PR-A 2/4)**: `type="(email|url|tel|number|password)"` 의 input 45곳 / 20 파일 자동 sed (lookahead `(?![^>]*\bdir=)` 로 중복 회피). W3C i18n 표준 — 이메일/URL/전화/숫자/비밀번호 값 자체는 RTL 환경에서도 LTR 표시 유지.
  - **`<pre>`/`<code>` LTR (`897d49b`, PR-A 3/4)**: admin 페이지 코드/JSON 표시 12곳 / 9 파일 자동 sed.
  - **WritingTask + receipt 가격 (`5dd74db`, PR-A 4/4)**: 한글 자판 페이지 3개 main 컨테이너 (writing_practice 분기 2 + level_select + stats = 4곳) 수동 + 영수증 (`receipt_parts.tsx`) gross/discount/supply/vat/total + 사업자번호 6 span 수동.

  **plan §2.9.5 갱신** (S2 진입 시 사전 조사 결과 반영):
  - 옵션 C 채택 명시 (자동 sed + 수동 분할)
  - S2 안 4 커밋 분할 — 리뷰 부담 완화
  - 가격 보호 범위 좁힘: 사전 조사로 `toLocaleString()` 호출 30+ 곳 분포 발견 → PR-A 안에서는 영수증 핵심만 처리. 나머지는 PR-C (RTL 시각 회귀) 단계에서 helper component (`<Price>`) 도입 검토 또는 별도 패치로 미룸.

  **검증 게이트**: `npm run build` 8.67s 클린 / `npx tsc --noEmit` 0 error / input dir="ltr" 동반 grep 45/45 / pre/code dir="ltr" 동반 grep 12/12. LTR 환경 (현재 ko/en) 시각 회귀: Tailwind logical 은 dir 속성 의존 → LTR 환경 = 변경 없음 보장.

  **배포**: deploy.yml run `25144272236` (headSha `5dd74db`) — build-and-push 24s + deploy 40s 모두 SUCCESS, `/health` 200. INC 0 건. (PR #189 머지 후 deploy run `25143559147` 도 사전 점검 게이트로 통과 확인.)

  **다음**: PR #190 사용자 머지 → 다음 세션 진입. **S3 (PR-B-pre 인프라)** = font_loader 6 lang 신규 (ar/fa/ur/bn/am/lo) + RTL 그룹 분류 + dir toggle 헬퍼 (SUPPORTED_LANGUAGES 활성은 PR-B / S5 까지 ai 측 RTL 번역 PR 대기). **S4 (PR-D 데스크탑)** = `amazing-korean-desktop` 동일 logical 마이그.

---

- **2026-04-30 (저녁) — Q13 Phase 2 세션 단위 실행 계획 수립**

  사용자 제안: 민감한 작업 (RTL 707곳 마이그, INC-004 직후 신중 모드) 을 세션 단위로 분할하여 이슈 발생 시 처리 가능하게. 각 세션 = 작업 범위 + 검증 게이트 + 롤백 plan 명시.

  **plan §2.9 SSoT 신설** — 미래 claude 세션이 plan 만 보고 실행 가능하도록 self-contained 작성:
  - **§2.9.1 자가 점검 체크리스트** (모든 세션 진입 전 필수): 사전 조건 / KKRYOUN sync / clean working dir / 이미 적용된 마이그 변경 시도 금지 (INC-004) / KKRYOUN push 자동 배포 인지 / 변경 한도 명시
  - **§2.9.2 이슈 발생 시 hotfix 패턴** (INC-004 절차 재사용): 영향 파악 → baseline 복원 → 단일 hotfix 커밋 → 자동 재배포 모니터링 → 사후 기록
  - **§2.9.3 세션 정의 표** (S1~S7, 사전조건/시간/PR 매핑)
  - **§2.9.4 S1** — frontend 자동 sed 12쌍 (~707곳). perl word-boundary 매핑 명령 포함. 1-2h.
  - **§2.9.5 S2** — 의도적 LTR 영역 보호 (input email/url/tel/number, 가격, 코드 블록) + 검증 + PR-A 푸시. 2-3h.
  - **§2.9.6 S3** — PR-B-pre 인프라 dormant (font_loader 6 신규 / RTL 그룹 신설 / dir toggle 핸들러). SUPPORTED_LANGUAGES 제외. 2-3h.
  - **§2.9.7 S4** — 데스크탑 (`amazing-korean-desktop`) 동일 logical 마이그. 3-4h.
  - **§2.9.8 S5~S7** — ai 측 결과 도착 후 상세화 (SUPPORTED_LANGUAGES 활성 / RTL 시각 검증 / 데스크탑 활성).
  - **§2.9.9 세션 간 컨텍스트 인계** — `project_status.md` breadcrumb 갱신 + 검증 결과 + 발견 위험 기록.

  **저장 (git 외)**: plan SSoT `~/.claude/plans/supported-language-es-pt-variants-expansion.md` §2.9 + 메모리 `project_phase2_followup_research.md` 세션 단위 포인터 추가.

  **다음**: S1 진입 — PR #189 머지/배포 후 시작. 사전 조건 §2.9.1 자가 점검 통과 후.

---

- **2026-04-30 (오후) — Q13 Phase 2 api 측 후속 작업 사전 조사 완료**

  ai 측 (amazing-korean-ai Mac Mini) 의 13 lang 번역 PR 도착 시점에 api 측에서 즉시 진행할 후속 작업 분량·우선순위 실측 + 외부 벤치마크.

  **실측 데이터 (frontend `src/` grep)**:
  - directional Tailwind class 사용 **약 707곳**: text-left 168 / text-right 94 / mr- 176 / ml- 67 / pl- 23 / pr- 6 / left- 33 / right- 16 / rounded-l 114 / border-l/r 9 / rounded-r 1
  - logical alternatives 사용 현재 2건 (`start-*` 만) — 처음부터 LTR-only 코드베이스
  - Tailwind 3.4.17 — logical properties 지원 ✅

  **폰트 fallback 신규 매핑 6 lang**:
  - ar (Noto Sans Arabic), fa (Noto Sans Arabic), ur (**Noto Nastaliq Urdu** 고유 서체), bn (Noto Sans Bengali), am (Noto Sans Ethiopic), lo (Noto Sans Lao)
  - 추가 불필요 7 lang (Pretendard 커버): ky / tr / it / pl / uk / sw / tl

  **자매 서비스 영향**:
  - 데스크탑 (`amazing-korean-desktop` Tauri+React) = api 와 동일 i18n 구조 (22 locale, `src/i18n/locales/`) → **PR-D 동일 patch**
  - 모바일 (`amazing-korean-mobile` Flutter) = i18n 미구현 (l10n/intl 없음) → Phase 2 영향 없음

  **외부 벤치마크 (Korean 학습 + RTL)**:
  - Duolingo: RTL 후발 도입, Korean 학습 효과 비판
  - TTMIK: 영어 위주, RTL 정보 없음
  - 일반론: 한국어 학습 + 본격 RTL 지원 거의 없음 → **차별화 가치** (아랍/페르시아/우르두권 시장)

  **작업 PR 4분할** (총 2.5-3.5일):
  - **PR-A** logical 마이그 (707곳 sed + LTR 보호) — 1일, 번역 무관
  - **PR-B** RTL 인프라 (groups.ts + dir toggle + font_loader 6 신규) + `SUPPORTED_LANGUAGES` 13 추가 — 반나절, ai PR 머지 후
  - **PR-C** RTL 시각 검증 + 깨진 곳 패치 — 반나절~1일, 실제 번역 텍스트 필수
  - **PR-D** 데스크탑 동일 작업 (별도 리포) — 반나절~1일

  **의도적 LTR 유지 영역** (RTL 토글 시 `dir="ltr"` 보호): 이메일/URL/전화/숫자/가격/코드 블록/한국어 자판 연습/캐러셀 화살표.

  **미리 진행 vs 대기 결정**: 사용자 동의 — **대기**. 이유: (1) RTL 검증은 실제 번역 텍스트 필수 (2) ai 측 PR 분할/lang set 미확정 (3) INC-004 학습.

  **저장**:
  - plan SSoT `~/.claude/plans/supported-language-es-pt-variants-expansion.md` §2.4 ~ §2.4.6 상세화
  - 메모리 `project_phase2_followup_research.md` 신규 + `MEMORY.md` 인덱스
  - `AMK_STATUS.md §8.2 Q13` 인라인 갱신

  **Tailwind logical properties 참조**: Tailwind 3.3 blog (`ms-`/`me-`/`text-start`/`rounded-s` 등 자동 LTR/RTL 처리, `ltr:`/`rtl:` variant 불필요).

---

- **2026-04-30 — PR #188 Gemini 리뷰 MEDIUM 2건 즉시 반영 (supported_language_enum 표기 일관화)**

- **2026-04-30 — PR #188 Gemini 리뷰 MEDIUM 2건 즉시 반영 (supported_language_enum 표기 일관화)**

  PR #188 머지 (2026-04-28 03:06 UTC) 후 Gemini 리뷰 MEDIUM 2건. `feedback_work_rules` "머지 후 Gemini 리뷰 즉시 반영" 원칙 적용.

  - **MEDIUM #1** — `docs/AMK_API_MASTER.md §4.8 (L1334)`: DB enum 정의 섹션이므로 BCP 47 hyphen 표기 (`zh-CN`/`zh-TW`/`es-ES`/`pt-PT`) 가 부적절. 실제 DB 저장값 = snake_case (`zh_cn`/`zh_tw`/`es_es`/`pt_pt`) 로 정정. **API 응답·요청 시 BCP 47 (serde rename)** 은 별도 명시 추가.
  - **MEDIUM #2** — `docs/AMK_SCHEMA_PATCHED.md L34`: `CREATE TYPE supported_language_enum` 정의가 22 lang 만 포함 (snapshot 시점 그대로). L587 주석 ("37개 지원 언어") 와 일관성 깨짐. L34 도 37 lang 모두 포함하도록 갱신 (2026-04-21 +13 + 2026-04-28 +es_es/pt_pt).

  변경: docs 2건 (코드/마이그 영향 없음).

---

- **🚨 2026-04-28 — INC-004 hotfix: 이미 적용된 마이그레이션 20260421 주석 수정으로 sqlx checksum 실패, 프로덕션 약 8분 다운**

  **타임라인** (UTC):
  - 02:42 — KKRYOUN push (커밋 `4fcc068`, Q13 한눈 요약 박스) → deploy.yml 자동 트리거 (deploy.yml `on.push.branches: [main, KKRYOUN]`)
  - 02:43~02:47 — 빌드 + SSH 배포. `amk-api` 컨테이너 startup 시 sqlx 마이그레이션 무결성 체크 실패 → panic → crash loop. nginx 502 (api upstream 연결 불가). Health check 5회 모두 502.
  - 02:50 — hotfix 커밋 `a5cd2ea` push (20260421 파일 원본 복원)
  - 02:55+ — 재배포 완료, `/health` 200 OK, uptime 28s

  **원인**: Phase 1 작업 (커밋 `731d4d1`, 2026-04-28 오전) 중 정책 번복 이력을 추적하기 위해 `migrations/20260421_expand_supported_languages.sql` 의 주석에 `[2026-04-28 번복]` 4줄 노트를 추가. 사실은 sqlx 가 마이그레이션 파일 **전체 hash 를 checksum** 으로 사용하므로 주석/공백/EOL 변경도 checksum 변경. 프로덕션 `_sqlx_migrations` 테이블에 저장된 기존 checksum 과 startup 시 비교 → mismatch → `migration 20260421 was previously applied but has been modified` panic.

  **복구**: `migrations/20260421_expand_supported_languages.sql` 을 `070af73` (Q13 plan 수립 시점, 머지된 baseline) 으로 원본 복원. 정책 번복 노트는 신규 마이그 `20260428_add_es_pt_variants.sql` 주석에 이미 명시되어 있어 정보 손실 없음.

  **교훈** (`feedback_migration_safety.md` §17~19 추가):
  17. **이미 적용된 마이그레이션은 SQL/주석/공백/EOL 모두 변경 절대 금지**. §3 항 "파일명 변경 금지" 의 확장. sqlx checksum 은 파일 전체 hash. 정책 번복·이력 추적은 신규 마이그 주석 또는 CHANGELOG/메모리에서.
  18. **`deploy.yml` 트리거 = `main` + `KKRYOUN` 양쪽 push**. KKRYOUN push 만으로 자동 프로덕션 배포 트리거. PR 머지 전 검증 단계 없음. 단일 브랜치 워크플로 (`feedback_git_branching`) 와 결합 시 "PR 작성 시점 = 배포 시점" 동치. 마이그 변경 시 이 차이 인지 필수.
  19. **INC-002 (study_task_explain) / INC-003 (날짜 충돌) / INC-004 (적용된 마이그 내용 수정) 3종 모두 sqlx checksum 체크에서 startup panic**. 공통: "마이그레이션은 한 번 적용되면 read-only artifact" 로 취급.

  **관련 문서/메모리 정정**:
  - `AMK_STATUS.md §8.2 Q13` 인라인 — "기존 마이그 20260421 주석에 정책 번복 노트" → "🚨 INC-004 발생 후 hotfix 로 되돌림" 정정
  - `feedback_migration_safety.md` §17~19 추가
  - `project_status.md` / `project_q13_briefing.md` 메모리 INC-004 인지

  **PR #188 영향**: 본 PR 의 다른 변경 (Phase 1 enum + Phase 2 인프라 + Q13 briefing) 은 정상. 머지/배포 가능. KKRYOUN 에 hotfix 커밋 `a5cd2ea` 포함된 상태.

- **2026-04-28 (오후) — Q13 Phase 2 인프라 확정 (amazing-korean-ai Wave 1 파이프라인 SSoT 채택)**

- **2026-04-28 (오후) — Q13 Phase 2 인프라 확정 (amazing-korean-ai Wave 1 파이프라인 SSoT 채택)**

  **배경**: Phase 2 (frontend UI locale 15 신규) 실행 직전 amazing-korean-ai (Mac Mini) 측에서 Wave 1 번역 인프라 브리핑 회신. 4,330 translations × 20 lang 실전 운영 결과. 본 plan §2.3 의 옵션 A/B/C 비교는 인프라 미반영. 단일 옵션 = ai 측 gemma 파이프라인.

  **주요 변경**:
  - **plan SSoT 정정** (`~/.claude/plans/supported-language-es-pt-variants-expansion.md` §2): 작업 그룹 분기 (13 lang gemma 자동 / es-ES·pt-PT 수동 inline diff), 옵션 A/B 폐기, 검증 4종 명시 (E1/M01/Q-prefix/orthography), api 측 후속 책임 분리, 사전 준비 ai 측 위임 (orthography validator + Unicode block + 도메인 용어집), 시점 분기 (그룹 1 즉시 / 그룹 2 Phase 1 머지 후).
  - **api 측 reference 메모리 신규** (`reference_translation_pipeline.md`): SSoT 위치, 핵심 규약 7항, api 책임, known issues 5종, 절대 금지 3항.
  - **`AMK_API_LEARNING.md §9` 번역 인프라 정책 정정**: "Claude Code 직접 번역" → "frontend UI locale 은 amazing-korean-ai Wave 1 파이프라인 SSoT, 자체 도구 금지" cross-link.
  - **STATUS Q13 갱신**: Phase 2 인프라 확정 인라인 추가, 실행은 ai 세션 위임.

  **검증된 인프라 핵심**:
  - Ollama `gemma4:26b` 고정 (변경 금지 — known issues 가 모델 의존)
  - batch-size 5 + retry (1/3/9s × 3, wrapper batch 10→5 fallback)
  - 검증 4종 모두 CRITICAL=0 머지 조건
  - 외부 LLM 합의 (Codex CLI + Gemini CLI 2-LLM): legal/RTL 전수, UI 샘플링
  - `merge_to_api_i18n.js` → api `frontend/src/i18n/locales/{lang}.json`

  **잔여**: Phase 2 실행은 amazing-korean-ai 세션에서 진행 (api 측 의존성 = supported_language_enum 추가 — 그룹 1 13 lang 은 2026-04-21 마이그로 충족, 그룹 2 es_es/pt_pt 는 Phase 1 머지/배포 완료 후). api 측 후속 PR = ai PR 머지 후 `frontend/src/i18n/index.ts` `SUPPORTED_LANGUAGES` 확장 + RTL (`<html dir="rtl">`) + 폰트 매핑.

- **2026-04-28 — Q13 Phase 1 완료 (supported_language_enum es_es/pt_pt 확장)**

  **배경**: 2026-04-27 plan SSoT (`~/.claude/plans/supported-language-es-pt-variants-expansion.md`) §1 의 Phase 1 단일 세션 실행. 2026-04-21 "pt_pt → pt 병합" 정책 번복.

  **변경 사항**:
  - **신규 마이그레이션** `migrations/20260428_add_es_pt_variants.sql` — `ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'es_es' / 'pt_pt'`. enum 값 35 → 37.
  - ~~기존 마이그레이션 20260421 주석에 [2026-04-28 번복] 노트 추가~~ → **🚨 INC-004 발생, 동일 날짜 hotfix `a5cd2ea` 로 되돌림**. 이미 적용된 마이그레이션은 sqlx checksum 불일치로 crash loop. 정책 번복 노트는 신규 마이그 20260428 주석에 이미 명시되어 있어 정보 손실 없음.
  - **`src/types.rs` `SupportedLanguage` enum** +`EsEs`/`PtPt` variant. sqlx rename `es_es` / `pt_pt`, serde rename `es-ES` / `pt-PT` (BCP 47 hyphen, zh-CN/zh-TW 와 동일 패턴). doc 주석 35→37.
  - **문서 정정 4건**:
    - `AMK_SCHEMA_PATCHED.md` L587 `content_translations.lang` 주석 "22개" → "37개 (2026-04-21 +13, 2026-04-28 +es_es/pt_pt)".
    - `AMK_API_MASTER.md` §4.8 (L1334) supported_language_enum 목록 35 → 37 + `'es-ES'` / `'pt-PT'` 추가 + 정책 번복 노트.
    - `AMK_API_LEARNING.md` L605 "지원 언어 21개" → "36개 (`ko` 원본 제외)" + 그룹 재구성 (동남아 +tl/lo, 중앙북아 +ky, 남아시아 +bn/ur, 유럽 +es-ES/pt-PT/it/pl/uk/tr, 중동/아프리카 신규 그룹 ar/fa/sw/am).
    - `AMK_API_LEARNING.md` L919 "비디오 100개 × 21 언어 × 3 필드 = 6,300+" → "× 36 언어 × 3 필드 = 10,800+".

  **검증**: `cargo check` 17.26s ✅, `cargo clippy --lib --bins -- -D warnings` 18.12s 0 warnings ✅. (DB 적용은 배포 시 sqlx 자동.)

  **잔여**: Phase 2 (frontend UI locale 15 신규, 별도 세션 3 PR 분할), Phase 3 (메타 표시), Phase 4 (books `gen_seed_sql.py` skip 로직 제거 — books 측 handoff). Phase 1 머지 + 프로덕션 배포 완료 후 Phase 2/3/4 활성화.

  **TextbookLanguage enum 21 (교재 출간 언어) 확장은 본 plan 범위 외** — 별도 결정 (출판 의도·ISBN 분리 발급 검토 필요).

- **2026-04-27 — Q13 supported_language 확장 plan 수립 (es_es/pt_pt + UI locale 22→35)**

  **배경**: books `sentences.json` 500 문장 전수조사 결과 **35 언어** 번역 보유 (am, ar, bn, de, es, **es_es**, fa, fr, hi, id, it, ja, kk, km, ky, lo, mn, my, ne, pl, pt, **pt_pt**, ru, si, sw, tg, th, tl, tr, uk, ur, uz, vi, zh_cn, zh_tw). 그러나 api 측은:
  - DB `supported_language_enum`: 33 (es_es, pt_pt 의도적 제외)
  - frontend UI locale: 22 (DB 보다 13 부족)

  **2026-04-21 정책 번복**: "pt_pt → pt 병합" 결정을 사용자가 번복 (2026-04-27). 근거: "스페인어/포르투갈어 유럽 variant 는 엄연히 구분되는 언어적 표현". books 35 언어 전체를 DB enum 으로 수용 + frontend UI 도 35 확장.

  **4 Phase plan 수립** (`~/.claude/plans/supported-language-es-pt-variants-expansion.md`, SSoT):
  - **Phase 1** — DB enum 확장 (`migrations/20260428_add_es_pt_variants.sql` ALTER TYPE ADD VALUE 'es_es'/'pt_pt') + `src/types.rs` `SupportedLanguage` 에 `EsEs`/`PtPt` variant 추가 (sqlx 'es_es', serde 'es-ES') + 코드/문서 문구 정정. 1-2시간, 단일 세션.
  - **Phase 2** — frontend UI locale 15 신규 (am, ar, bn, fa, it, ky, lo, pl, sw, tl, tr, uk, ur + es-ES, pt-PT). 약 22,500 키 자동 번역 필요. 5 locale × 3 PR 분할 권장. 8-16시간, 별도 세션.
  - **Phase 3** — about/faq/카탈로그 페이지에 "본 콘텐츠는 35 언어로 번역" 메타 표시. Phase 2 와 병행 가능.
  - **Phase 4** — books `gen_seed_sql.py` 의 "es_es/pt_pt skip" 로직 제거. handoff 프롬프트 plan 내 포함. books 측 처리.

  **신규 마이그레이션 prefix**: `20260428_` (INC-003 교훈 — books seed_output `20260427_` 와도 충돌 없음 확인).

  **`TextbookLanguage` enum 21 (교재 출간 언어) 확장 여부는 별개 결정** (출판 의도·ISBN 분리 발급 등 별도 검토 필요).

  **AMK_STATUS.md §8.2** Q13 신규 행 추가, plan 파일 SSoT 포인터 명시.

- **🚨 2026-04-23 (저녁) — INC-003 hotfix: 마이그레이션 파일명 prefix 충돌, 프로덕션 약 8분 다운**

  **사상**: PR #184 (`377ea31`) 배포 직후 `amk-api` crash loop. 약 8분 다운 (06:01~06:09 UTC). hotfix 커밋 `16691a1` 배포 후 복구. `/health` 200 OK 확인.

  **에러 로그** (crash loop):
  ```
  Error: migration 20260423 was previously applied but has been modified
  ```

  **원인**:
  - PR #183 에서 `migrations/20260423_textbook_order_discount.sql` 생성 → 프로덕션 적용 완료
  - PR #184 에서 `migrations/20260423_textbook_orderer_email_optional.sql` 생성 (같은 `20260423` prefix)
  - sqlx migrator 는 파일명 prefix 를 version 번호로 사용
  - 프로덕션 `_sqlx_migrations` 테이블에 `20260423` 버전이 이미 등록된 상태에서, 디스크에 같은 버전 파일이 추가 → 체크섬 불일치 → `amk-api` 부팅 시 마이그레이션 단계에서 crash loop
  - 로컬 환경에서는 `cargo check` 와 `sqlx offline cache` 만 검증했는데, 실제 DB 의 `_sqlx_migrations` 테이블과의 충돌은 프로덕션에서만 감지 가능

  **조치** (`16691a1`):
  - 파일명 변경: `20260423_textbook_orderer_email_optional.sql` → `20260424_textbook_orderer_email_optional.sql`
  - 파일 내부 주석에 "날짜 2026-04-23, 파일명 20260424 로 설정 (버전 충돌 회피)" 명시
  - `git mv` 로 rename 감지 + commit + push → KKRYOUN push 가 배포 트리거라 자동 재배포

  **재발 방지 (memory 확장)**:
  - `feedback_migration_safety.md` §"2026-04-23 INC-003 추가 교훈" 4 항목 추가:
    - (13) 하루에 2개 이상 마이그레이션 필요 시 두 번째부터 다음 날짜 사용 (동일 YYYYMMDD prefix 금지)
    - (14) 마이그레이션 작성 전 `ls migrations/ | tail -5` 로 같은 날짜 prefix 파일 확인 체크리스트화
    - (15) INC-003 은 기존 교훈 §21 의 연장선. 동일 세션 2개 생성 시나리오 명시적으로 추가
    - (16) 하나의 PR 머지 후 생기는 새 PR 이 같은 날짜 마이그레이션 추가하는 시나리오가 특히 위험 (프로덕션 DB 상태 의존성)

  **AI 책임**: 마이그레이션 파일 생성 시 `feedback_migration_safety.md` §"sqlx 마이그레이션 파일 네이밍" 이미 있던 "같은 날짜 충돌 시 다음 날짜 사용" 규칙을 놓침. 동일 세션 내 2개 생성 시나리오를 구체적으로 인지하지 못하고 기계적으로 파일명을 붙였음. 재발 방지 위해 체크리스트 메모리에 추가.

  **INC 대비 비교**:
  | INC | 날짜 | 다운 시간 | 원인 | 복구 |
  |---|---|---:|---|---|
  | INC-001 | 2026-04-15 | 2h33m | deploy.yml + Secrets 불일치 (`panic!` 게이트) | `.env.prod` placeholder → 영구 수정 |
  | INC-002 | 2026-04-18 | ~30m | `study_task_explain` 로컬/프로덕션 테이블명 불일치 + nginx race | 테이블명 3곳 수정 + nginx reload 재시도 |
  | INC-003 | 2026-04-23 | ~8m | 마이그레이션 파일명 버전 충돌 | 파일명 `20260423` → `20260424` |

- **2026-04-23 (저녁) — #73 추가 UX 개선 5건 (사용자 프로덕션 테스트 피드백 반영)**

  사용자가 PR #183 머지 후 프로덕션 환경에서 관리자 대리 주문 생성/조회/인쇄 플로우 실사용 중 발견한 이슈 5건 일괄 처리.

  ### 1. 비회원/회원 주문 모드 명시적 구분

  - **기존 문제**: "user_id 직접 입력 / 검색 콤보박스 / 비워두면 비회원" 3 모드가 단일 UI 에 혼재 → 의도 불명확
  - **해결**: 주문 생성 페이지 상단에 **세그먼트 컨트롤** 신규 (`guest` / `member`)
    - `guest` 선택 시: user_id 관련 UI 전체 숨김, payload 에서 user_id 미전송
    - `member` 선택 시: 검색 콤보박스 or 수동 user_id 입력 토글 표시. member 모드인데 사용자 미선택 시 submit 검증 실패
  - **i18n**: `admin.textbook.create.{orderMode, guestOrder, memberOrder, guestOrderHint, memberOrderHint}` + `err.memberRequired` 신규

  ### 2. 이메일 필수 해제 (DB + Backend + Frontend)

  - **기존 문제**: 오프라인·전화 주문 대리 입력 시 이메일 수집이 어려운 경우가 있음. 필수 요구가 현실에 맞지 않음
  - **DB**: [migrations/20260423_textbook_orderer_email_optional.sql](../migrations/20260423_textbook_orderer_email_optional.sql) — `ALTER TABLE textbook ALTER COLUMN orderer_email DROP NOT NULL`
  - **Backend**:
    - `TextbookOrderRow.orderer_email: Option<String>`
    - `InsertOrderParams.orderer_email: Option<&str>`
    - `OrderRes.orderer_email: Option<String>`
    - `AdminCreateOrderReq.orderer_email: Option<String>` (사용자 `CreateOrderReq` 는 여전히 `String` 필수 유지 — UI validate 와 확인 메일 필요)
    - 이메일 발송 로직을 `if let (Some(email_sender), Some(recipient_email))` 패턴으로 감싸 이메일 없을 때 발송 스킵
  - **Frontend**:
    - Zod 스키마 `orderer_email: z.string().nullable()`
    - admin create 페이지: `required` 속성 제거 + "선택 입력" 힌트 + 이메일 입력됐을 때만 형식 검증 (`^[^\s@]+@[^\s@]+\.[^\s@]+$`)
    - admin detail 페이지: `order.orderer_email ?? "-"`
    - 사용자 주문 조회: `{order.orderer_email && <p>...</p>}`
    - 인쇄 페이지 (영수증/견적서/주문확인서): 동일 가드
  - **i18n**: `admin.textbook.create.emailOptional` 힌트 + `err.emailInvalid` 신규

  ### 3. 주문 상태 자유 전환 (state machine 완화)

  - **기존 문제**: `is_valid_status_transition` 이 엄격한 forward-only 상태 머신 (pending → confirmed → paid → printing → shipped → delivered, 역방향 불가). 관리자가 실수로 잘못된 상태로 바꿨을 때 되돌릴 방법 없음
  - **해결**:
    - [src/api/admin/textbook/service.rs](../src/api/admin/textbook/service.rs) `is_valid_status_transition`: 동일 상태 재설정만 금지하고 나머지 모든 쌍 허용 (`current != next`)
    - [src/api/textbook/repo.rs](../src/api/textbook/repo.rs) `update_status` / `update_status_in_tx`: timestamp 갱신을 `SET col = $3` → `SET col = COALESCE(col, $3)` 로 변경. 역행 전환 시에도 기존 첫 전환 시점 보존. 최근 변경 시각은 `updated_at` 으로 추적
    - admin detail 페이지 `getValidNextStatuses`: `ALL_STATUSES.filter(s => s !== current)` 로 전체 노출

  ### 4. 인쇄/PDF 시 admin 레이아웃 격리 (`@media print`)

  - **기존 문제**: 관리자 인쇄 페이지 (`/admin/textbook/orders/:id/print?type=receipt`) 가 `AdminLayout` 안에서 렌더링되어 **sidebar + header 가 PDF 에 함께 출력됨**
  - **해결**: [admin_layout.tsx](../frontend/src/category/admin/page/admin_layout.tsx) 에 Tailwind `print:hidden` / `print:block` / `print:p-0` 등 추가
    - `<aside>` 에 `print:hidden` (sidebar 숨김)
    - `<header>` 에 `print:hidden` (top bar 숨김)
    - `<main>` 에 `print:p-0 print:overflow-visible` (패딩·스크롤 제거)
    - 루트 `<div>` 에 `print:block print:bg-white` (flex → block 으로 전환, 배경 흰색)
  - 루트 가드가 CSS 레이어에서 작동하므로 AdminLayout 내 모든 인쇄 페이지 (현재 textbook + 향후 추가분) 에 자동 적용

  ### 5. 영수증 디자인 강화

  기존 영수증 (`isReceipt` 분기) 이 단순 border + text 구성이라 정식 문서 느낌이 부족한 피드백 반영. 견적서/주문확인서는 기존 유지 (피드백 범위 밖).

  - **헤더 강화**: 양쪽 정렬 (좌: `RECEIPT · 영수증` 태그 + 큰 제목 / 우: `No. ORDER_CODE` + 영수일). 하단 3px border.
  - **공급자 + 수신자 2컬럼 그리드**: `ReceiptSupplierBox` + 신규 수신자 박스를 나란히 배치. 동일 스타일 (2px border, 작은 uppercase 라벨, 본문 정렬).
  - **품목 테이블 개선**: 영수증일 때만 헤더 배경색 (`bg-muted/40`) + 셀 패딩 증가 (`py-3 px-3`) + uppercase 헤더 + 숫자 `font-mono`.
  - **ReceiptTotalBreakdown 리디자인**:
    - 외곽 `border-2 border-black rounded overflow-hidden`
    - 중간 라인들은 `text-muted-foreground` 라벨 + `font-mono` 값
    - 최종 "영수 금액" 은 **검정 배경 + 흰 텍스트 + `text-2xl font-bold`** 로 강조
    - 할인 라인은 `text-destructive` 색 + 사유 괄호 병기
    - "정히 영수함" 은 회색 배경으로 분리
  - **ReceiptSignature 개선**: "(인)" 텍스트를 24×24 dashed border 박스로 교체 (실제 인감 사이즈 참조)
  - 공급자/수신자 박스 둘 다 `h-full` 로 높이 일치

  ### 검증

  - `cargo check` 13.04s + `cargo clippy --lib --bins -- -D warnings` 0 warnings (clippy needless_borrow 1건 수정)
  - `npm run build` 8.45s 성공
  - 마이그레이션은 CI 자동 실행 대상

  ### 변경 파일

  - **Migrations**: 1 (`20260423_textbook_orderer_email_optional.sql`)
  - **Backend**: 5 (`repo.rs`, `textbook/dto.rs`, `textbook/service.rs`, `admin/textbook/dto.rs`, `admin/textbook/service.rs`)
  - **Frontend**: 8 (`admin_layout`, `admin_textbook_order_create`, `admin_textbook_order_detail`, `admin_textbook_order_print`, `textbook/types`, `textbook/page/textbook_order_print`, `textbook/page/textbook_order_status_page`, `textbook/receipt_parts`, `admin/textbook/types`)
  - **i18n**: 2 (ko + en)
  - **Docs**: 1 (CHANGELOG)

- **2026-04-23 (오후) — #73 후속: 교재 주문 할인 기능 + 수량 입력 버그 fix**

  - **수량 입력 버그 fix** (`admin_textbook_order_create.tsx`, 독립 커밋 `cdd4c43`):
    - 증상: admin 대리 주문 생성 페이지 품목 수량 input 에서 "1" 을 지울 수 없음 → "2" 로 덮어쓰기 불가
    - 원인: controlled input 에서 `Number("") || 1 = 1` 로 빈 문자열이 즉시 1 로 복귀
    - 해결: `OrderItem.quantity: number → number | ""` 타입 확장, onChange 에서 빈 값 "" 저장, onBlur 에서 1 fallback

  - **할인 기능 — 결정 3건 (2026-04-23 사용자)**:
    - 결정 1 (B): 세 필드 DB 저장 (gross_amount + discount_amount + discount_reason) — 영수증 표기 + 감사 추적성
    - 결정 2 (B): 주문 상세 페이지에서도 편집 가능 — `PATCH /admin/textbook/orders/{id}/discount` 신규
    - 결정 3 (A): 세법 정확 — 공급가액(과세표준) 은 **할인 후 금액** 기준, VAT 재계산

  - **DB 마이그레이션** [migrations/20260423_textbook_order_discount.sql](../migrations/20260423_textbook_order_discount.sql):
    - `ALTER TABLE textbook` + `gross_amount INT NOT NULL` + `discount_amount INT NOT NULL DEFAULT 0` + `discount_reason TEXT`
    - 기존 주문 백필: `UPDATE textbook SET gross_amount = total_amount` (할인 없었으므로)
    - CHECK 제약 3건: discount ≥ 0, discount ≤ gross, total = gross - discount (DB 레벨 데이터 무결성 가드)
    - 관계: `total_amount = gross_amount - discount_amount`

  - **Backend** ([src/api/textbook/repo.rs](../src/api/textbook/repo.rs), [src/api/textbook/dto.rs](../src/api/textbook/dto.rs), [src/api/admin/textbook/service.rs](../src/api/admin/textbook/service.rs)):
    - `TextbookOrderRow` + `InsertOrderParams` + `OrderRes` 에 `gross_amount` / `discount_amount` / `discount_reason` 3 필드 추가
    - 신규 repo 함수 `update_discount` — gross 불변, discount + total 만 UPDATE. DB CHECK 로 자동 검증.
    - `AdminCreateOrderReq` 에 `discount_amount: i32 (default 0)` + `discount_reason: Option<String> (max 500)` 신규
    - `AdminTextbookService::create_order` 에 검증 (0 ≤ discount ≤ gross) + 감사 로그 jsonb 확장 + 사유 trim 정규화
    - 신규 `AdminTextbookService::update_discount` + `AdminUpdateDiscountReq` DTO + `update_discount` handler + router entry
    - 사용자 `create_order` 는 `gross = total`, `discount = 0` 으로 기록 (일반 사용자 주문은 할인 없음)

  - **Frontend 프론트 타입·API·hook** ([frontend/src/category/admin/textbook/types.ts](../frontend/src/category/admin/textbook/types.ts), [frontend/src/category/admin/admin_api.ts](../frontend/src/category/admin/admin_api.ts), [frontend/src/category/admin/textbook/hook/use_admin_textbook.ts](../frontend/src/category/admin/textbook/hook/use_admin_textbook.ts)):
    - `AdminCreateOrderReq` 에 `discount_amount?` + `discount_reason?` 추가
    - 신규 `AdminUpdateDiscountReq` interface
    - `updateAdminTextbookOrderDiscount` API 함수 + `useAdminUpdateTextbookDiscount` mutation
    - `OrderRes` zod 스키마에 `gross_amount` / `discount_amount` / `discount_reason` 필드 추가

  - **Frontend UI — admin 생성 페이지** ([admin_textbook_order_create.tsx](../frontend/src/category/admin/textbook/page/admin_textbook_order_create.tsx)):
    - 할인 금액 (type=number) + 할인 사유 (text, disabled 할인=0) 신규 Card
    - 품목 합계 / 할인 / 최종 합계 3단 요약 표시
    - 할인 > gross 시 빨간 경고 + submit 거부
    - discount 입력은 수량과 동일한 `number | ""` 패턴 (빈 값 허용 + blur fallback)

  - **Frontend UI — admin 상세 페이지** ([admin_textbook_order_detail.tsx](../frontend/src/category/admin/textbook/page/admin_textbook_order_detail.tsx)):
    - 주문 항목 테이블 tfoot 을 "품목 합계 / 할인 / 최종 합계" 3행 구조로 재구성 (할인 0 이면 단일 행)
    - "할인 편집" 버튼 + Dialog (discount_amount + discount_reason 수정, save mutation 호출)
    - 감사 로그 기록 자동 (서비스 레이어가 `AdminAction::Update` 기록)

  - **Frontend UI — 사용자 주문 조회 페이지** ([textbook_order_status_page.tsx](../frontend/src/category/textbook/page/textbook_order_status_page.tsx)):
    - 할인 있는 주문은 "품목 합계 / 할인(사유 병기) / 합계" 3행 표시
    - 할인 없는 주문은 기존 단일 Total 행 유지 (UX 일관성)

  - **영수증·인쇄 페이지** ([receipt_parts.tsx](../frontend/src/category/textbook/receipt_parts.tsx), [textbook_order_print.tsx](../frontend/src/category/textbook/page/textbook_order_print.tsx), [admin_textbook_order_print.tsx](../frontend/src/category/admin/textbook/page/admin_textbook_order_print.tsx)):
    - `ReceiptTotalBreakdown` 시그니처 확장 — `grossAmount?` + `discountAmount?` + `discountReason?` 선택적 props
    - 할인 있을 때 표시 구조 (세법 정확):
      ```
      품목 합계:       gross
      - 할인 (사유):   - discount
      ─────────────────
      공급가액:        (gross - discount) / 1.1
      부가세 (10%):    (gross - discount) × 0.1 / 1.1
      ─────────────────
      영수 금액:       total = gross - discount
      ```
    - 공급가액은 **할인 후 기준** (과세표준). VAT 는 과세표준 × 10%. 세금계산서 발행 시에도 정확.

  - **i18n** ([ko.json](../frontend/src/i18n/locales/ko.json), [en.json](../frontend/src/i18n/locales/en.json)):
    - `admin.textbook.create.{discount, discountAmount, discountReason, discountReasonPlaceholder, discountHint, grossAmount, finalAmount}` 신규
    - `admin.textbook.create.err.{discountNegative, discountExceedsGross}` 신규
    - `admin.textbook.detail.{grossAmount, finalAmount, discountAmount, discountReason, editDiscount, editDiscountTitle, editDiscountDesc, saveDiscount, discountSaved, discountSaveFailed}` 신규 섹션
    - `admin.textbook.print.{subtotal, discount}` + `textbook.print.{subtotal, discount}` + `textbook.status.{grossAmount, discount}` 신규
    - ko + en 양쪽 추가. 나머지 20 locale 은 en fallback (Q4 영수증 번역 정책 동일).

  - **검증**: `cargo check` + `cargo clippy --lib --bins -- -D warnings` 0 warnings + `npm run build` 8.93s 클린. sqlx offline cache 기존 유지 (신규 raw SQL 이지만 기존 패턴과 동일 구조).

  - **프로덕션 배포 순서**:
    1. PR 머지 → CI 자동 마이그레이션 실행 (`20260423_textbook_order_discount.sql`)
    2. 기존 주문은 `gross_amount = total_amount`, `discount_amount = 0` 로 백필 (마이그레이션 내부)
    3. 배포 완료 → 관리자가 새 주문 생성 시 할인 옵션 사용 가능
    4. 기존 주문에 할인 소급 적용은 주문 상세 페이지 "할인 편집" 으로

- **2026-04-23 — Q11 pt footer 판단: 시나리오 B (QA prompt 보강) 채택, API 팀 코드 수정 없음**
  - **배경**: 2026-04-22 overnight full run (`tests/qa-results/2026-04-22T06-39-53Z/`) 에서 PR #182 효과 검증 완료 — Gemma flag 32→1 (-97%), JWT 만료 70→2 (Q12 효과). 잔존 1건이 Q11 (pt 데스크톱 footer overlap). 맥미니가 "진짜 버그 여부 판단 요청" 으로 `docs/QA_결과.md` 재작성, API 팀에 5 시나리오 판단 위임.
  - **판단 — 시나리오 B 채택** (footer 수정 안 함, QA prompt 보강으로 흡수):
    1. **실 버그 증거 부재** — 1440px 스크린샷에 overlap 시각적으로 확인 안 되고, 잠재 문제 구간 (768-1023px) 은 QA 뷰포트 매트릭스 밖이라 측정된 적 없음. 존재·확인 모두 안 된 버그.
    2. **Gemma reason hallucination 실증** — reason 절반이 `"You are a web UI quality reviewer..."` 프롬프트 첫 문장을 페이지 텍스트로 착각. 구조적 FP 신호.
    3. **전역 footer 수정 리스크 > 가치** — A-a (`lg:flex-row`) 는 22 locale × 전 페이지 레이아웃 변경. A-b (`flex-wrap`) 는 `justify-between` 과의 브라우저별 차이. A-c (pt 문구 축약) 는 법적·브랜드 검토 선행 필요.
    4. **Gemma FP 누적 데이터 부족** (MVP 1회) → prompt 개선은 정성적이지만 즉시 착수 가능한 저리스크 개선.
  - **QA 쪽 권장 조치**:
    - **우선**: `ollama_check/prompts/text_overflow.md` 에 가드 문구 추가 ("footer copyright+legal 조밀 배치는 정상, 명확히 가려지거나 잘린 경우만 flag").
    - **보조**: `check_runner.py` path+check 단위 whitelist 지원 추가 (prompt 효과 부족 시 도입).
    - **tablet viewport (768-1023px) 추가 권장** — B 선택과 별개 트랙. footer 외 다른 회귀 잠복 가능 구간의 구조적 공백 해소.
  - **"양치기 소년" 리스크 대응**: Prompt 보강 후 2-3 full run 관찰. 다른 영역 (subtitle/carousel/text overlap) 실 회귀 발생 시 Gemma 감지력 유지되는지 검증. 감지력 저하 시 prompt 재조정 or A-a 재고.
  - **변경 파일**: docs 3건 (`docs/QA_결과.md §6` 답변 섹션 신규 + `docs/AMK_STATUS.md §8.2` Q11 ✅ 답변 완료 처리 + 이 CHANGELOG 엔트리).
  - **커밋 스코프**: PR #183 결합 (단일 브랜치 정책). PR #183 은 `#67 Phase 2~5 + Gemini fix + Q11 판단 답변` 의 3 스코프 누적.
  - **사용자 액션**: B 채택 판단 맥미니 쪽에 이미 전달 완료 (2026-04-23 사용자 확인).

- **2026-04-23 — #67 E-book session_id 필수화 Phase 2~5 전환 완료 (D+7 관측 0건 통과)**
  - **D+7 관측 결과**: `docker logs amk-api --since 168h 2>&1 | grep EBOOK_SESSION_AUDIT | wc -l` = **0** (2026-04-23 사용자 실측). Phase 1 로깅 배포(2026-04-16) 이후 7일간 구버전 클라이언트 / 어뷰즈 / SPA 캐시 모두 부재 확인. D+8(2026-04-24) 예정이었으나 조건 충족으로 하루 앞당김.
  - **배경 — 관측 모드 선택 근거**: INC-001(2026-04-15 프로덕션 2h33m 다운) 경험으로 "fail-closed 게이트 추가는 코드 분석만 신뢰 말고 프로덕션 로그로 선확인" 방침 확립. 트래픽 표본이 작아 24~48h 불충분 판단으로 5~7일 관측. D+7 0건으로 방침 성공 실증.
  - **Phase 2 — Backend 전환** (`amazing-korean-api`):
    - [src/api/ebook/service.rs](../src/api/ebook/service.rs) `verify_session(st, user_id, session_id: &str)` — `Option<&str>` 제거. 진입부 `is_none()` 분기 + `tracing::warn!("EBOOK_SESSION_AUDIT: ...")` 블록 제거. 내부 Option 언래핑 제거 — 저장된 Redis JSON 의 `session_id` ↔ 요청 헤더 `session_id` **엄격 비교** (항상 수행). 불일치 → `Forbidden("Viewer session invalid")`.
    - [src/api/ebook/handler.rs](../src/api/ebook/handler.rs) `get_page_image` + `get_page_tile` 2곳 — `x-ebook-session` 헤더 파싱 `.map(|s| s.to_string())` → `.ok_or_else(|| AppError::Forbidden("Missing session header".into()))?` 즉시 거부. `session_id.as_deref()` 제거 → 직접 `&str` 전달.
  - **Phase 3 — Web frontend 전환** (`amazing-korean-api/frontend`):
    - [frontend/src/category/ebook/ebook_api.ts](../frontend/src/category/ebook/ebook_api.ts) `fetchPageImage` + `fetchPageTile` — `sessionId?: string` → `sessionId: string`. 삼항 `...(sessionId ? { "X-Ebook-Session": sessionId } : {})` → 직접 `"X-Ebook-Session": sessionId`. HMAC 가드 `hmacSecret && sessionId` → `hmacSecret` 만 (sessionId 는 이제 required).
    - [frontend/src/category/ebook/hook/use_page_image.ts](../frontend/src/category/ebook/hook/use_page_image.ts) `usePageImage` + `usePageTiles` — default value `sessionId = ""` 로 전환. meta 미로드 시 `enabled: !!meta && ...` 로 fetch 차단 → fetchPage 가 빈 sessionId 로 호출될 일 없음. TypeScript 는 `string | undefined` 를 default 파라미터로 수용.
    - [frontend/src/category/ebook/page/ebook_viewer_page.tsx](../frontend/src/category/ebook/page/ebook_viewer_page.tsx) — 변경 없음. 기존 `const sessionId = meta?.session_id` + `!!meta` enabled 가드로 동작 정확성 유지.
  - **Phase 4 — Desktop 전환** (`amazing-korean-desktop`, 별도 PR 진행 중): 웹과 동일 3 파일 (`ebook_api.ts` + `use_page_image.ts` + `ebook_viewer_page.tsx`).
  - **Phase 5 — Mobile**: `lib/api/ebook_api.dart` 에 `required String sessionId` + 헤더 항상 전송 이미 준수 (2026-04-07 구현 시점). **작업 없음**.
  - **검증**: `cargo check` 14.66s + `cargo clippy --lib --bins -- -D warnings` 0 warnings + `npm run build` 9.77s 클린.
  - **배포 후 모니터링**: `docker logs amk-api --since 24h | grep "Missing session header"` — 정상 트래픽에서는 0 or 극소 (의도된 거부만). 24h 내 403 급증 시 롤백.
  - **롤백 plan** (1 분): Backend revert PR (handler 2곳 + service.rs 원복). 프론트는 sessionId 전송 상태이므로 backend Option 복귀 후 무해. 트리거: 24h 내 `"Missing session header"` 403 > 예상값 10배 or 사용자 이슈.
  - **변경 파일**: 백 2 (service.rs + handler.rs) + 웹 2 (ebook_api.ts + use_page_image.ts) + docs 3 (AMK_STATUS.md §8.1 #67 + AMK_CHANGELOG.md 엔트리 + AMK_API_EBOOK.md 변경 없음 — 이미 "필수 헤더" 로 기재됨).
  - **플랜 SSoT**: `~/.claude/plans/ebook-session-required-phase25.md` (아카이브).

- **2026-04-23 — PR #182 Gemini 리뷰 MEDIUM 2건 즉시 반영**
  - **배경**: PR #182 (Q10 + Q12) 머지 직후 (2026-04-22T05:59Z) Gemini 가 `docs/QA_결과.md` 의 로컬 경로 상대링크 2곳 지적. `feedback_work_rules.md` "PR 머지 후 Gemini 리뷰 즉시 반영" 원칙에 따라 머지 후 13시간 내 처리 (2026-04-23 세션 시작 시점).
  - **L28 — 저장소 내부 링크가 로컬 WSL 경로 포함**:
    - Before: `[...](../../../../dev/amazing-korean-api/frontend/...)`
    - After: `[...](../frontend/...)` — `docs/` 에서 저장소 루트 기준 상대 경로.
    - 이 링크는 QA 팀이 원본 요청 시 작성한 것. GitHub 에서 404 났을 것.
  - **L230~L232 — 별도 저장소 `amazing-korean-ai` 참조를 상대링크로 작성**:
    - Before: `[...](../../../scripts/qa/run_qa.sh)`, `[...](./ARCHITECTURE.md)`, `[...](../AMK_AI_QA.md)`
    - After: 링크 제거, 경로만 backtick 으로 표시 + "별도 저장소 `amazing-korean-ai` 기준" 주석 + GitHub 메인 링크.
    - `amazing-korean-ai/scripts/qa/` 는 Mac Mini 로컬 전용이라 GitHub 에 없을 가능성 → 링크 제거가 가장 안전.
  - **전수조사**: `grep -nE "\.\./\.\./\.\./"` 로 `docs/QA_결과.md` + `amazing-korean-ai/docs/AMK_AI_QA_HANDOFF_2026-04-22.md` 동시 스캔 — 추가 위반 0건 확인.
  - **변경**: 1 파일 (`docs/QA_결과.md`) + CHANGELOG.
  - **검증 불필요**: docs 단독 변경, 빌드/테스트 영향 없음.

- **2026-04-22 (심야) — Q10 프론트 3건 fix + Q12 JWT TTL QA 답변**
  - **배경**: 2026-04-22 저녁에 수신한 QA Mac Mini 자동 런 결과 (`2026-04-22T01-35-53Z`) 의 Q10 (프론트 수정 3건 묶음) + Q12 (JWT TTL QA 전용 연장 답변) 를 이번 세션에서 처리. Q11 (pt footer 오버랩) 은 footer breakpoint 변경이 전역 디자인 영향이라 별도 PR 로 남김.
  - **Q10.1/.2 — subtitle `<br className="hidden sm:block" />` 공백 누락 fix (전수 6곳)**:
    - QA 리포트가 지적한 파일: `ebook_catalog_page.tsx:97-102`, `textbook_catalog_page.tsx:97`. 원인: `i > 0 && <br className="hidden sm:block" />` 에서 `sm` 미만 (모바일) 은 `<br>` 이 `display:none` 이라 단어 사이 공백이 사라짐 (`"languages,available"` 처럼 붙음).
    - **전수조사 결과 동일 패턴 6곳** — QA 지적 2곳 외에 `coming_soon_page.tsx:54`, `error/error_page.tsx:25`, `error/access_denied_page.tsx:26`, `error/not_found_page.tsx:26` 에서도 동일 버그 잠복. 모두 같은 fix 적용 (feedback_work_rules 전수조사 원칙).
    - **수정안**: `{i > 0 && <br className="hidden sm:block" />}` → `{i > 0 && <>{" "}<br className="hidden sm:block" /></>}`. 모바일에서는 공백 문자가 보존되어 단어 구분, 데스크톱에서는 `<br>` 우선 (공백은 시각적 무시).
  - **Q10.3 — `/book` 캐러셀 dot `aria-label` 누락 fix (전수 3곳)**:
    - QA 리포트가 지적한 파일: `book_hub_page.tsx:112` (dot 버튼에 `aria-label` / `aria-current` 없음 → 접근성상 dead-button).
    - **전수조사 결과 동일 패턴 3곳** — `book/page/book_hub_page.tsx`, `ebook/page/ebook_detail_modal.tsx:107`, `textbook/page/textbook_detail_modal.tsx:116`. 모두 같은 fix 적용.
    - **수정안**: `<button>` 에 `aria-label={t("common.goToSlide", { n: i + 1 })}` + `aria-current={i === slideIndex ? "true" : undefined}` 추가. i18n 키 `common.goToSlide` 신규 추가 (ko: `"{{n}}번 슬라이드로 이동"`, en: `"Go to slide {{n}}"`). 나머지 20개 locale 은 en 영어 fallback.
  - **Q12 — JWT TTL QA 전용 연장 답변 (비코딩)**:
    - 현재 값: `JWT_ACCESS_TTL_MIN=15` (분 단위 default). env override 지원 (`src/config.rs:126`).
    - **권장: 옵션 A** — QA 전용 `.env.qa` 에서 `JWT_ACCESS_TTL_MIN=360` (6h) 오버라이드. 프로덕션 `.env` 에는 영향 없음. API 코드 변경 불필요.
    - **옵션 B (refresh 플로우) 참고정보** — 웹 로그인은 `ak_refresh` HttpOnly 쿠키 (SameSite=Lax, Secure in prod) 로 refresh_token 전달. Playwright `storageState().origins[].cookies[]` 자동 저장. 모바일은 JSON body 로 반환 (`MobileLoginRes.refresh_token`). `POST /auth/refresh` 쿠키 기반, body 없음.
    - **API 팀 답변**: `docs/QA_결과.md §6.1` 신규 섹션 + 체크박스 6건 모두 답변 기록. §6.2 에 OpenAPI drift 도 QA drift tolerance 권장 답변.
  - **QA 3.2 OpenAPI drift**: API 팀이 overrides 수작업 동기화 불가 → QA 가 warn 격하 (Security 위반 anon→admin 200 만 fail). `docs/QA_결과.md §6.2` 에 답변 기록.
  - **검증**: `npm run build` 9.74s 성공, 번들 크기 변화 없음.
  - **변경 파일 수**: 프론트 9 파일 (subtitle fix 6 + aria-label fix 3) + i18n 2 파일 (ko.json + en.json 각 `common.goToSlide` 키 1개) + docs 2 파일 (QA_결과.md §6 + STATUS.md Q10/Q12 완료 처리).
  - **큐 상태**: Q10 ✅ / Q12 ✅. Q11 (pt footer) 은 별도 PR 로 남김 — footer 컴포넌트 breakpoint 변경은 전 언어 영향이라 디자인 레이어 검토 필요.

- **2026-04-22 (밤) — QA 자동화 런 결과 수신 + 오늘 세션 종료 + 처리 계획**
  - **QA run 근거**: `amazing-korean-ai/scripts/qa` Mac Mini 자동 QA 오케스트레이터 (Playwright 1838 tests + Gemma 4 26b 3444 calls + Fuzz 1200 requests). 근거 run `tests/qa-results/2026-04-22T01-35-53Z/`. 총 2시간 27분 소요.
  - **핵심 지표**:
    - Playwright: 1748 pass / 87 fail. 실제 프로덕트 이슈 **4건**, 나머지는 QA 하네스 버그(12) + JWT 만료 70 + OpenAPI drift 1.
    - Gemma 시각 검사: 3444 calls, 실질 이슈 **3종** (언어별 분산). False positive rate <1%.
    - Fuzz: 1200 requests, **unhandled 5xx 0건** ✅. 백엔드 입력 검증 레이어 건강.
  - **실질 이슈 분류** (처리 방향):
    - **Q10 (높음, 30분)** — 프론트 수정 3건 묶음: 2.1 ebook subtitle 공백 누락 (14 locale) + 2.2 textbook subtitle 공백 누락 (km/my/th) + 2.4 `/book` 캐러셀 dot `aria-label` 누락 (en/ja). 2.1/2.2 원인 동일 (`<br className="hidden sm:block" />` 모바일 공백 대체 없음).
    - **Q11 (낮음, ~1h)** — 2.3 pt 데스크톱 footer 텍스트 오버랩 (포르투갈어만 해당).
    - **Q12 (중간, 15분 응답)** — 3.1 JWT TTL QA 전용 연장 설정 답변. QA full run 2h30m 중 token 만료로 70건 fail. 권장: QA `.env` 에서 `JWT_ACCESS_TTL_SEC=21600` (6h) 오버라이드 허용. 현재 TTL `config.rs` 확인 후 QA 에 응답.
    - **QA 쪽 회송** — 3.2 OpenAPI drift 1328 cell: API 팀이 overrides 수작업 동기화 대신 QA 가 drift tolerance (warn) 로 전환. Security 위반만 fail. → QA 팀이 조치.
  - **처리 plan**: `~/.claude/plans/qa-mac-mini-20260422-fixes.md` (신규 세션 시작점 SSoT).
  - **AMK_STATUS.md §8.2** Q10/Q11/Q12 신규 행 추가.
  - **다음 세션**: Q10 착수 (프론트 수정 3건 묶음 PR) + Q12 응답 (config.rs 확인 + 답변 문서화).

- **2026-04-22 (저녁) — Q1c 잔여: admin video 편집 UI 에 video_title/subtitle 별도 필드 (반나절)**
  - **배경**: Q1c B (2026-04-21) 에서 `video` 테이블에 `video_title`/`video_subtitle` 물리 컬럼을 추가했으나, admin 프론트엔드 UI 는 여전히 `video_tag_title`/`video_tag_subtitle` 만 입력 받음. 백엔드 backward-compat (video_title 미제공 시 video_tag_title 폴백) 로 동작은 하고 있었지만, 관리자가 "비디오 자체 제목" 과 "태그 분류 제목" 을 **별도로 설정** 할 수 없는 상태. Q1c 잔여 공사로 분리했던 프론트 UI 확장 이번 커밋에서 완료.
  - **백엔드 확장**:
    - [src/api/admin/video/dto.rs](../src/api/admin/video/dto.rs) `AdminVideoRes` 에 `video_title: Option<String>`, `video_subtitle: Option<String>` 필드 추가 (`#[serde(skip_serializing_if = "Option::is_none")]`). 기존 `title` 필드는 `video_tag_title` MAX 집계 호환용으로 유지 (legacy)
    - [src/api/admin/video/repo.rs](../src/api/admin/video/repo.rs) — list/detail/update RETURNING SQL 3곳 모두 `v.video_title`, `v.video_subtitle` SELECT + GROUP BY 추가. create 에서는 INSERT 시 사용한 값 그대로 response 에 반영.
  - **프론트 타입 확장**:
    - [frontend/src/category/admin/types.ts](../frontend/src/category/admin/types.ts) (aggregator) — `adminVideoSummarySchema` 에 `video_title`/`video_subtitle` 응답 필드(nullable+optional) 추가. `videoCreateReqSchema`, `videoUpdateReqSchema`, `videoBulkUpdateItemSchema` 에 `video_title`/`video_subtitle` 요청 필드 추가
    - [frontend/src/category/admin/video/types.ts](../frontend/src/category/admin/video/types.ts) (domain) — 동일 변경 (중복 정의 동기화)
  - **프론트 UI**:
    - [frontend/src/category/admin/page/admin_video_create.tsx](../frontend/src/category/admin/page/admin_video_create.tsx) — "Video Title" / "Video Subtitle" 필드 신규. Vimeo 메타데이터 auto-fill 시 video_title 과 tag_title 모두 세팅. onSubmit 시 빈 문자열은 undefined 로 전달해 백엔드 backward-compat 폴백과 호환. "Tag Information" 섹션으로 video_tag_* 필드 명시 분리 + 안내 문구.
    - [frontend/src/category/admin/page/admin_video_detail.tsx](../frontend/src/category/admin/page/admin_video_detail.tsx) — 수정 모드에 동일 필드 추가. form.reset 에서 video.video_title/subtitle 우선 세팅. onSubmit 에서 빈 문자열은 undefined 로 전달 (UPDATE 스킵 → 기존 값 유지).
    - [frontend/src/category/admin/page/admin_video_bulk_create.tsx](../frontend/src/category/admin/page/admin_video_bulk_create.tsx) — CSV 컬럼 `video_title`/`video_subtitle` 인식 (optional). 기존 CSV 와 backward-compat.
  - **검증**:
    - `cargo check` 10.22s 클린
    - `cargo clippy --lib --bins -- -D warnings` 14.66s 클린
    - `frontend: npm run build` 8.72s 성공
  - **회귀 리스크**:
    - AdminVideoRes 에 신규 필드만 추가 (`skip_serializing_if` 로 null 직렬화 생략) → 기존 응답 shape 비파괴적
    - 기존 admin UI 가 여전히 `video.title` 을 쓰는 경우에도 동작 유지 (legacy title 필드 보존)
    - Create/Update 경로: `video_title` 미제공 시 backend polyfill 로 video_tag_title 사용 → 기존 CSV/자동화 호환
  - **다음 단계**: #67 Phase 2~5 D-Day (2026-04-24) 또는 Q3 영수증 고유번호 체계 (옵션 결정 후).

- **2026-04-22 (오후) — Q6: admin_textbook_log 감사 로그 조회 UI + 신규 API (반나절)**
  - **배경**: `admin_textbook_log` 테이블에 관리자 작업 이력은 기록되고 있었으나 (textbook/repo.rs:504 `insert_admin_log`), 조회 API 가 없어 admin UI 에서 "언제 누가 어떤 주문을 create/update/banned 했는지" 확인 불가. Q5 (사용자 검색 UI) 와 짝을 이루는 후속 공사.
  - **백엔드 — 신규 엔드포인트 `GET /admin/textbook/logs`** (필터 + 페이지네이션):
    - [src/api/admin/textbook/dto.rs](../src/api/admin/textbook/dto.rs) — `AdminTextbookLogQuery` (action/order_id/admin_user_id/page/per_page), `AdminTextbookLogItem`, `AdminTextbookLogListRes`, `AdminTextbookLogMeta`
    - [src/api/textbook/repo.rs](../src/api/textbook/repo.rs) — `AdminLogRow` 신규 구조체 (admin_email_enc 는 암호화 상태). `list_admin_logs` 함수 (JOIN users + textbook, 필터 조건부 바인딩, 총 개수 + 페이지 데이터 반환)
    - [src/api/admin/textbook/service.rs](../src/api/admin/textbook/service.rs) — `list_admin_logs` 서비스. `CryptoService::new(...)` 로 admin_email 복호화 후 응답 DTO 조립. `per_page` 는 `.clamp(1, 100)`.
    - [src/api/admin/textbook/handler.rs](../src/api/admin/textbook/handler.rs) — `list_admin_logs` 핸들러 (`Query<AdminTextbookLogQuery>` + AuthUser)
    - [src/api/admin/textbook/router.rs](../src/api/admin/textbook/router.rs) — `.route("/logs", get(handler::list_admin_logs))`
  - **프론트**:
    - [frontend/src/category/admin/textbook/types.ts](../frontend/src/category/admin/textbook/types.ts) — `AdminAction` 타입, `AdminTextbookLogQuery`, `AdminTextbookLogItem`, `AdminTextbookLogListRes`, `AdminTextbookLogMeta` 신규
    - [frontend/src/category/admin/admin_api.ts](../frontend/src/category/admin/admin_api.ts) — `getAdminTextbookLogs(params)` 함수
    - [frontend/src/category/admin/textbook/hook/use_admin_textbook.ts](../frontend/src/category/admin/textbook/hook/use_admin_textbook.ts) — `useAdminTextbookLogs` 훅 + `adminTextbookKeys.logs`/`logList` query key
    - **신규 페이지** [frontend/src/category/admin/textbook/page/admin_textbook_logs_page.tsx](../frontend/src/category/admin/textbook/page/admin_textbook_logs_page.tsx) — 필터 (action drop-down + order_id text + admin_user_id text) + 테이블 (timestamp/action badge/admin/order_code/diff) + 페이지네이션. `before_data`/`after_data` JSONB 는 `<details>` 토글로 raw JSON pretty-print.
    - [frontend/src/app/routes.tsx](../frontend/src/app/routes.tsx) — `textbook/logs` 라우트 추가, lazy import
    - [frontend/src/category/admin/textbook/page/admin_textbook_orders_page.tsx](../frontend/src/category/admin/textbook/page/admin_textbook_orders_page.tsx) — 상단에 "감사 로그" outline 버튼 추가 (`/admin/textbook/logs` 링크)
  - **i18n** (ko + en 신규 키, 나머지 18 locale 은 en 폴백):
    - `admin.textbook.logs.{title,subtitle,empty,loadError,filter,actions,table,diff}` 섹션
  - **보안 고려**:
    - admin 이메일은 DB 에 AES-256-GCM 암호화 상태. 서비스에서 복호화 후 응답 — 관리자(JWT auth)만 접근 가능한 엔드포인트이므로 PII 정책 준수.
    - `CryptoError` → `AppError::Internal(500)` 매핑 (`feedback_security_patterns.md` 불투명화 원칙). decrypt 실패 시 스택 오류 로그 + generic 500.
  - **검증**:
    - `cargo check` 16.23s 클린
    - `cargo clippy --lib --bins -- -D warnings` 22.75s 클린
    - `frontend: npm run build` 8.04s 성공
    - `query_as::<_, AdminLogRow>` 런타임 체크 방식이라 sqlx offline cache 재생성 불필요
  - **테스트 plan (머지 후)**:
    - admin 로그인 → `/admin/textbook/orders` → "감사 로그" 버튼 → `/admin/textbook/logs` 진입
    - 액션 필터 Create 선택 → `admin_create_video` 호출 결과 로그 확인
    - order_id / admin_user_id 조합 필터 → 특정 관리자가 특정 주문에 한 액션만 조회 확인
    - before/after JSON pretty-print 토글 동작 확인
  - **다음 단계**: PR #180 (Q5) + 오늘 Q6 를 같은 KKRYOUN 에 스택. 머지 시 함께 배포.

- **2026-04-22 (오후) — Q5: 사용자 검색 UI — admin 대리 주문 생성 자동완성 (프론트 전용, 반나절)**
  - **배경**: PR #174 (#75 관리자 대리 주문 생성) 후속. `admin_textbook_order_create.tsx` 의 `user_id` 텍스트 입력을 자동완성 콤보박스로 개선. 백엔드 `GET /admin/users?q=` 이미 지원 (이메일 blind index exact match / 닉네임 LIKE) → 재사용.
  - **신규 컴포넌트**: [frontend/src/category/admin/components/user_search_combobox.tsx](../frontend/src/category/admin/components/user_search_combobox.tsx) `UserSearchCombobox`
    - props: `value: AdminUserSummary | null`, `onChange: (user | null) => void`
    - 300ms debounce → `useAdminUsers({ q, size: 10, page: 1 })` 자동 호출
    - 드롭다운에 `nickname + email + ID` 표시, 선택 시 `onChange` 호출
    - 선택 상태일 때는 카드 뷰 + X 버튼으로 해제
    - 2글자 이상일 때만 검색 (`enabled: debounced.length >= 2 && open`), `staleTime: 60s`
    - 외부 클릭 시 드롭다운 닫힘 (mousedown 이벤트)
    - shadcn Command 미사용 (vendor 번들 최소화) — Input + absolute div 조합
  - **admin_textbook_order_create.tsx 통합**:
    - `selectedUser: AdminUserSummary | null` + `manualUserIdMode: boolean` state 신규
    - 기본은 검색 모드 (UserSearchCombobox 렌더), 토글 버튼으로 수동 `user_id` 입력 폴백
    - `handleSubmit` 에서 selectedUser 있으면 `user.id` 사용, 아니면 수동 모드일 때 기존 엄격 파싱(`/^\d+$/`) 유지
  - **i18n** (ko + en 키 신규 — 나머지 18 locale 은 en 폴백):
    - `admin.textbook.create.userSearch.placeholder`/`hint`/`empty`/`minChars`/`clear`/`noNickname`/`toggleManual`/`toggleSearch`
  - **검증**:
    - `frontend: npm run build` 9.09s 성공 (admin_textbook_order_create 청크 재빌드 확인)
    - 기존 수동 파싱 로직은 폴백 경로로 보존 → 회귀 없음
  - **외부 의존 없음**: 백엔드 코드 0줄 변경 (기존 엔드포인트 재사용).
  - **다음 단계**: Q6 (`admin_textbook_log` Create 액션 조회 UI, 반나절, 백엔드 신규 API + 프론트) 착수 예정.

- **2026-04-22 (오전) — Q1c Gemini 2차 리뷰 반영: count_field 중복 → TranslatedField::count_to 메서드 통합**
  - **배경**: PR #179 스코프 확장 후 사용자가 `/gemini review` 수동 트리거 → 2026-04-22 01:15Z Gemini 2차 리뷰 (MEDIUM 5건, 모두 동일 패턴). `count_field` 헬퍼 함수가 4개 Consumer service 파일에 중복 정의돼 있으니 `TranslatedField` 구조체의 메서드로 이동해 중복 제거 권장.
  - **반영 내역** (커밋 `7ace98d`, 5 파일 / −82 +46 순 감소):
    - [src/api/admin/translation/dto.rs](../src/api/admin/translation/dto.rs) — `TranslatedField` 에 `count_to(&self, user_lang, translated, fallback)` 메서드 추가
    - [src/api/course/service.rs](../src/api/course/service.rs) — 로컬 `count_field` 제거, 호출 사이트를 `t.count_to(...)` 로 변경. 미사용 `TranslatedField` import 제거.
    - [src/api/lesson/service.rs](../src/api/lesson/service.rs) — 동일
    - [src/api/study/service.rs](../src/api/study/service.rs) — 동일
    - [src/api/video/service.rs](../src/api/video/service.rs) — 동일. 단, `TranslatedField` 는 `apply_tag_translations` 함수 시그니처에 여전히 사용되므로 import 유지. `apply_tag_translations` 내부 호출도 `t.count_to(...)` 로 변경.
  - **Gemini 리뷰 전수 반영**: MEDIUM 5건 / HIGH/CRITICAL 0건. 미처리 0건.
  - **검증**:
    - `cargo check` 17.25s 클린
    - `cargo clippy --lib --bins -- -D warnings` 22.12s 클린
  - **다음 단계**: PR #179 (4 커밋 스택) 사용자 머지 대기 → Gemini 3차 리뷰 가능성 관찰.

- **2026-04-21 (심야) — Q1c: 응답 스키마 최종 정렬 (결정 3건 확정 + 구현 완료)**
  - **배경**: PR #179 (Q1a Gemini fix + Q1b) 후속. 플랜 `translation-field-name-alignment.md §4` 의 Q1c 사용자 결정 3건을 확정하고 구현.
  - **결정 A — 덮어쓰기 유지 + 루트 메타 2필드 추가** (하이브리드 방식):
    - [src/api/admin/translation/dto.rs](../src/api/admin/translation/dto.rs) — `TranslationMeta { translation_lang: Option<SupportedLanguage>, translation_coverage: TranslationCoverage }` + `TranslationCoverage` enum (`NotRequested`/`Full`/`Partial`/`None`) 신규
    - `TranslatedField.actual_lang` 신규 필드 — `find_translations_for_contents` 가 실제 반환 언어 추적 (fallback 여부 판단용)
    - `TranslationMeta::from_counts` / `ko_full` / `not_requested` 헬퍼 + `Default` impl
    - 10 Consumer 엔드포인트 응답 루트에 `translation_meta` 필드 주입:
      - [src/api/course/service.rs](../src/api/course/service.rs) — `CourseListRes` + `CourseDetailRes` 신규 래퍼 (기존 `Vec<CourseListItem>` → wrapper 반환). 프론트 소비 0건 확인 후 shape 변경.
      - [src/api/lesson/service.rs](../src/api/lesson/service.rs) — `LessonListRes`/`LessonDetailRes` 에 필드 추가
      - [src/api/video/service.rs](../src/api/video/service.rs) — `VideoListRes`/`VideoDetailRes` 에 필드 추가 (`#[sqlx(skip)]` + `#[serde(default)]` 로 FromRow 호환)
      - [src/api/study/service.rs](../src/api/study/service.rs) — `StudyListResp`/`StudyDetailRes`/`StudyTaskDetailRes`/`TaskExplainRes` 에 필드 추가
    - 채택 근거: (1) 구현 비용 최소 (필드 double 없음), (2) fallback 감지 UX 가능, (3) 교육 서비스 원문 토글은 `?lang=ko` 재호출로 대체
  - **결정 B — Video 테이블에 title/subtitle 물리 컬럼 추가**:
    - 마이그레이션 `migrations/20260422_video_title_subtitle.sql` — `ALTER TABLE video ADD video_title VARCHAR(150) NOT NULL, ADD video_subtitle VARCHAR(250)`. `MAX(video_tag_title)`/`MAX(video_tag_subtitle)` 집계로 백필 후 DEFAULT 제거. 로컬 검증 16행 백필 완료.
    - [src/api/video/repo.rs](../src/api/video/repo.rs) `list_videos`/`get_video_detail` SQL — `MAX(video_tag_title) as title` → `v.video_title as title` 로 교체. 검색 필터에도 `v.video_title`/`v.video_subtitle` 추가.
    - [src/api/admin/translation/repo.rs](../src/api/admin/translation/repo.rs) `find_source_fields` Video — Q1b 의 `source_text=None` stub 제거, 실 컬럼 매핑.
    - [src/api/admin/video/repo.rs](../src/api/admin/video/repo.rs) `admin_create_video`/`admin_update_video` — `video_title`/`video_subtitle` INSERT/UPDATE 추가. `VideoCreateReq`/`VideoUpdateReq` 확장 (backward-compat: 미제공 시 `video_tag_title`/`video_tag_subtitle` 폴백).
    - 채택 근거: (1) 의미 명확화 (video_tag = 분류 ≠ video_title = 제목), (2) M05~M08 교재 시딩 본격화 전 정리 적기, (3) Q1b stub 부채 해소
  - **결정 C — video_tag 번역 주입 (`VideoTagDetail.id` 노출)**:
    - `VideoTagDetail` 구조체에 `id: i64` 필드 신규 ([src/api/video/dto.rs](../src/api/video/dto.rs))
    - Repo `get_video_detail` SQL 의 `jsonb_build_object` 에 `'id', vt.video_tag_id` 포함
    - Service `get_video_detail` 에서 tags[] 의 id 수집 → `content_type=VideoTag content_id IN (ids)` 로 번역 일괄 조회 → `video_tag_title`/`video_tag_subtitle` 오버라이드
    - `GET /videos` (목록) 의 `tags: Vec<String>` (tag_key 만) 은 그대로 유지 — 목록에선 분류 키만 사용
    - 채택 근거: (1) 단순성, (2) admin API 에 이미 노출된 ID 라 보안 영향 없음, (3) 프론트에서 tag 클릭 → 관련 영상 검색 등 확장 가능
  - **Frontend admin UI** — Q1c 범위에 포함됐으나 backward-compat (video_title 미제공 시 video_tag_title 폴백) 로 기존 admin UI 동작 유지. 별도 필드 노출은 **후속 공사로 이연**.
  - **문서 동기화**:
    - `AMK_API_LEARNING.md §9-841` — Q1c 구현 완료 섹션 + TranslationMeta/TranslationCoverage 스펙 명시. Fallback 동작에 coverage 매핑 추가.
    - `AMK_STATUS.md §8.2 Q1c` ✅ 완료 처리.
    - `plans/translation-field-name-alignment.md §4 Q1c` — 결정 3건 락인.
  - **검증**:
    - 마이그레이션 로컬 적용 성공 (UPDATE 16행 백필)
    - `cargo sqlx prepare` 재생성 성공
    - `cargo check` 8.59s 클린
    - `cargo clippy --lib --bins -- -D warnings` 13.65s 클린
    - `frontend: npm run build` 10.33s 성공
  - **회귀 리스크**:
    - **Course 응답 shape 변경**: `GET /courses` → `Vec<CourseListItem>` 에서 `CourseListRes { items, translation_meta }` 로, `GET /courses/{id}` → `CourseListItem` 에서 `CourseDetailRes { course, translation_meta }` 로. 프론트/모바일/데스크탑 Consumer `/courses` 호출 0건 grep 확인 완료 → 실사용 영향 없음.
    - 다른 엔드포인트는 기존 wrapper 에 `translation_meta` 필드 추가만 (비파괴적).
    - admin 기존 `VideoCreateReq` 는 `video_title` Optional + 폴백으로 backward-compat 유지.
  - **다음 단계**: Q2~Q9 선택 (영수증 Q2~Q4, admin UI Q5/Q6, Paddle Q7, K6 Q8, E-book RDS Q9).

- **2026-04-21 (늦은 밤) — Q1b: Consumer `?lang=` 미구현분 구현 (videos/{id} + studies/tasks/{id} + /explain)**
  - **배경**: [PR #178](https://github.com/AmazingKoreanCenter/amazing-korean-api/pull/178) (Q1a field_name 잠복 버그 fix) + [PR #179](https://github.com/AmazingKoreanCenter/amazing-korean-api/pull/179) (Gemini MEDIUM 1건 반영) 머지 후속. 플랜 `.claude/plans/translation-field-name-alignment.md §2.2` Q1b 스코프 이행.
  - **3개 엔드포인트 `?lang=` 파라미터 추가 + 번역 주입**:
    - [src/api/video/handler.rs](../src/api/video/handler.rs) `get_video_detail` — `Query<VideoDetailReq>` 수용
    - [src/api/video/dto.rs](../src/api/video/dto.rs) — `VideoDetailReq { lang }` 신규, `VideoDetailRes` 에 `title: Option<String>`/`subtitle: Option<String>` 필드 추가
    - [src/api/video/repo.rs](../src/api/video/repo.rs) `get_video_detail` SQL — `MAX(video_tag_title)`/`MAX(video_tag_subtitle)` 집계 추가 (`video` 테이블 자체엔 title/subtitle 컬럼 부재)
    - [src/api/video/service.rs](../src/api/video/service.rs) `get_video_detail` — `lang: Option<SupportedLanguage>` 인자 추가, `content_type=Video` `field_name=video_title`/`video_subtitle` 오버라이드
    - [src/api/study/handler.rs](../src/api/study/handler.rs) `get_study_task` + `get_task_explain` — `Query<StudyTaskDetailReq>`/`Query<TaskExplainReq>` 수용
    - [src/api/study/dto.rs](../src/api/study/dto.rs) — `StudyTaskDetailReq { lang }`, `TaskExplainReq { lang }` 신규
    - [src/api/study/service.rs](../src/api/study/service.rs) `get_study_task` — task kind 별 `ContentType::StudyTask*` 로 분기 + payload 필드 오버라이드 (choice 5필드 question/1~4, typing 1필드 question, voice 1필드 question, writing 3필드 prompt/answer/hint). 헬퍼 `content_type_for_task_kind` 추가.
    - [src/api/study/service.rs](../src/api/study/service.rs) `get_task_explain` — `content_type=StudyTaskExplain` `field_name=explain_title`/`explain_text` 오버라이드
  - **field_name 규약**: Q1a 에서 확정한 긴 이름 (`{table}_{column}`) 표준 준수. `explain_*` 는 예외 (study_explain 테이블이 `study_` prefix 이미 중첩).
  - **video_tag 번역 주입 (옵셔널 — 이연)**: 플랜 §2 는 video list/detail 에 video_tag 번역 주입도 Q1b 스코프에 포함시켰으나, response 에 `video_tag_id` 노출 필요성 때문에 **Q1c (응답 스키마 최종 정렬) 로 이연**. 덮어쓰기 vs `_translated` 접미사 결정과 함께 일괄 설계.
  - **문서 동기화**:
    - `AMK_API_LEARNING.md §9-841` — "⬜ Q1b 미구현 부분" → "🟢 Q1b 구현 완료". `🟢 이미 구현된 부분` 표에 Q1b 3개 엔드포인트 행 추가.
    - `AMK_API_LEARNING.md §5.4-2` (videos/{id}) — `?lang=` 파라미터 및 title/subtitle 신규 필드 명시.
    - `AMK_API_LEARNING.md §5.5-3` (studies/tasks/{id}) — `?lang=` 파라미터 및 task kind 별 번역 필드 명시.
    - `AMK_API_LEARNING.md §5.5-6` (studies/tasks/{id}/explain) — `?lang=` 파라미터 및 explain_title/explain_text 오버라이드 명시.
    - `AMK_STATUS.md §8.2` Q1b 행을 ✅ 완료 처리.
  - **검증**:
    - `cargo check` 23.34s 클린
    - `cargo clippy --lib --bins -- -D warnings` 22.99s 클린
    - `sqlx prepare` 재생성 불필요 (`get_video_detail` 의 `query_as::<_, VideoDetailRes>` 는 매크로 아님. 다른 수정 파일은 SQL 변경 없음)
  - **회귀 리스크**: 프론트·모바일·데스크탑 Consumer `?lang=` 호출 0건 (2026-04-21 확인) → 사용자 가시 변화 없음. 번역 데이터가 `content_translations` 에 없으면 원본 그대로 반환.
  - **다음 단계**: Q1c (응답 스키마 최종 정렬) — 덮어쓰기 vs `_translated` 접미사 + `translation_lang`/`translation_coverage` 메타 + Video 테이블 title/subtitle 물리 컬럼 추가 여부 + video_tag 번역 주입 설계 사용자 최종 결정.

- **2026-04-21 (저녁) — Q1a: field_name 잠복 버그 fix (Consumer `?lang=` 정합 + admin 매핑 보강)**
  - **배경**: PR #176 (Phase 0 문서 정합) 머지 완료 직후 진입. 플랜 `.claude/plans/translation-field-name-alignment.md §2.2` 를 SSoT 로 Q1a 코드 작업. 프로덕션 `content_translations` 실 데이터는 긴 이름(`lesson_title` 등)으로 저장돼 있으나 Consumer service 4곳이 짧은 이름(`"title"` 등)으로 조회 → `?lang=` 호출 시 번역이 절대 반환되지 않는 잠복 버그 해소.
  - **Consumer service 4곳 field_name 치환** (긴 이름 `{table}_{column}` 표준):
    - [src/api/course/service.rs](../src/api/course/service.rs) — `"title"`→`"course_title"`, `"subtitle"`→`"course_subtitle"` (list + get_by_id)
    - [src/api/lesson/service.rs](../src/api/lesson/service.rs) — `"title"`→`"lesson_title"`, `"description"`→`"lesson_description"` (list_lessons + get_lesson_detail)
    - [src/api/study/service.rs](../src/api/study/service.rs) — `"title"`→`"study_title"`, `"subtitle"`→`"study_subtitle"` (list_studies + get_study_detail)
    - [src/api/video/service.rs](../src/api/video/service.rs) — `"title"`→`"video_title"`, `"subtitle"`→`"video_subtitle"` (list_videos)
  - **admin [src/api/admin/translation/repo.rs](../src/api/admin/translation/repo.rs) 매핑 보강**:
    - `find_content_records` — `ContentType::Course` 브랜치 신규 (course_id/course_idx/course_title). `ContentType::StudyTaskWriting` 브랜치 신규 (study_task join + study_task_writing_prompt LEFT 50). 기존 `_ => Vec::new()` 는 `VideoTag` 만 남음.
    - `find_source_fields` — `ContentType::Course` 브랜치 신규 (course_idx/title/subtitle/description). `ContentType::StudyTaskWriting` 브랜치 신규 (prompt/answer/hint). Video 에 `video_title`/`video_subtitle` 필드 노출하되 `source_text=None` (video 테이블에 물리 컬럼 없음, video_tag 집계 기반이라 관리자가 비디오 레벨 오버라이드 번역 입력용으로만 노출).
    - Row 구조체 신규 추가: `CourseSourceRow`, `WritingSourceRow`.
  - **문서 동기화**:
    - `AMK_API_LEARNING.md §9-841` — "🚨 잠복 버그 (Q1a 에서 수정)" → "🟢 Q1a 잠복 버그 해소 완료". 각 엔드포인트 `field_name` 긴 이름으로 명시.
    - `AMK_API_LEARNING.md §9-9` content_type 허용 목록 주의 문구 갱신 — Course·StudyTaskWriting 모두 구현 완료, VideoTag 만 직접 선택 불가.
    - `AMK_STATUS.md §8.2` Q1a 행을 ✅ 완료 처리 (2026-04-21).
  - **검증**:
    - `cargo check` 19.63s 클린 (eslint/warning 0)
    - `cargo clippy --lib --bins -- -D warnings` 23.17s 클린
    - 마이그레이션 불필요 — 실 DB 8 row 전부 이미 긴 이름 규약 준수
    - 스키마 하드코딩 쿼리(비 매크로) 사용이라 `cargo sqlx prepare` 재생성 불필요
  - **행동 변화**: 프로덕션에 존재하는 `lesson_title = 'approved'` 1건이 처음으로 `?lang=en` 호출 시 정상 반환됨 = 의도된 동작 복원. 프론트·모바일·데스크탑 Consumer `?lang=` 호출 0건이라 회귀 리스크 없음.
  - **다음 단계**: Q1b (미구현분 구현 — `GET /videos/{id}` + `GET /studies/tasks/{id}` + `GET /studies/tasks/{id}/explain` 에 `?lang=` 추가) → Q1c (응답 스키마 최종 정렬).

- **2026-04-21 (오후) — Q1 선행 정합 조사: `field_name` 규약 확정 + §9-841 재작성 + Q1 → Q1a/b/c 분해**
  - **머지 커밋**: `7035131` (PR [#176](https://github.com/AmazingKoreanCenter/amazing-korean-api/pull/176), `docs/translation-phase-0-alignment → main`, 2026-04-21 머지 완료 후 브랜치 삭제됨)
  - **배경**: Q1 (`?lang=` Consumer API 확장) 착수 전 문서·코드·DB 3자 대조 결과, `AMK_STATUS.md §8.2 Q1` + `AMK_API_LEARNING.md §9-841` 의 "⬜ 미구현" 표기가 사실과 불일치. Consumer service 4 도메인 중 6 엔드포인트는 이미 번역 주입 로직 보유. 단, (1) `field_name` 짧은이름(`"title"`) vs 실 DB 긴이름(`lesson_title`) 불일치로 번역이 반환되지 않는 잠복 버그, (2) 스펙은 `_translated` 접미사 방식이나 구현은 덮어쓰기 방식이라 양방향 편차.
  - **DB 실태 쿼리 결과** (2026-04-21): `content_translations` 총 8 row 전부 긴 이름 (`lesson_title`, `lesson_description`, `lesson_subtitle`, `lesson_idx`, `video_tag_key`/`title`/`subtitle`, `video_idx`). 프로덕션에 `lesson_title = 'approved'` 1건 존재하지만 Consumer `?lang=` 호출 시 절대 반환되지 않는 상태 = **#76 (`study_task_explain`) 과 동종의 실증된 잠복 버그**.
  - **프론트 영향도 조사**: Consumer `?lang=` 호출은 amazing-korean-api/frontend 0건, amazing-korean-mobile 0건, amazing-korean-desktop 0건. 덮어쓰기 vs `_translated` 접미사 선택의 회귀 리스크 사실상 0 → 설계 자유도 확보.
  - **정합 조치 (코드 0줄, 문서만)**:
    - **plans/translation-field-name-alignment.md** 신규 — field_name 매핑 매트릭스 10 ContentType × (consumer 조회 키 / admin source_fields 제안 키 / 실 DB 저장값 / 정합 방향) + Q1a/b/c 분해 근거 문서화. Q1 코드 작업 진입 전 SSoT.
    - **AMK_SCHEMA_PATCHED.md §2.7.1 content_translations** — field_name 주석 `(예: title, subtitle, description, question, choice_1 등)` → `(예: lesson_title, study_subtitle, study_task_choice_question, explain_title 등)` 긴 이름 규약 명시. content_type 나열에 `study_task_explain`, `study_task_writing` 보강.
    - **AMK_API_MASTER.md §4.8** — field_name 설명 동일 기준으로 긴 이름 규약 명시. content_type_enum 에 `study_task_writing` 추가.
    - **AMK_API_LEARNING.md §9-1~§9-3** — 예시 JSON 의 `field_name: "title"` → `"lesson_title"`. §9-2 바로 아래 "field_name 규약" 각주 추가.
    - **AMK_API_LEARNING.md §9-9** — `content_type` 허용 목록에서 `study_task_writing` 은 현재 stub 이므로 각주 명시. Course·VideoTag 도 Q1a 에서 매핑 보강 예정 표기.
    - **AMK_API_LEARNING.md §9-13** — 응답 예시 `{"stats": {...}}` → 실제 구조 `{"items": [...], "total_translations": N}` 로 교체.
    - **AMK_API_LEARNING.md §9-841** — "⬜ 미구현" → "🟡 부분 구현, 편차 있음" 으로 전면 재작성. 🟢 이미 구현 / 🚨 잠복 버그 / ⬜ 미구현 / ⬜ 스펙 정렬 4 섹션 재구성. 번역 입력 경로 "서버 사이드 스크립트" 방향 명시.
    - **AMK_STATUS.md §8.2** — Q1 한 줄 → **Q1a (field_name 잠복 버그 fix, 0.5일)** / **Q1b (미구현분 구현, 0.5~1일)** / **Q1c (응답 스키마 최종 정렬, 0.5일)** 3행으로 분해. 각 공수 + 선후관계 명시.
  - **수정 대상 코드 (Q1a 이후 PR 에서 처리)**: `src/api/{course,lesson,study,video}/service.rs` 7곳 field_name 치환, `src/api/admin/translation/repo.rs` 의 `find_content_records` / `find_source_fields` Course·StudyTaskWriting 매핑 추가, Video source_fields 에 `video_title`·`video_subtitle` 필드 보강.
  - **교훈**: `feedback_migration_safety.md` 의 "로컬 DB 이상 상태를 실 DB 로 오판" (INC-002) + `feedback_work_rules.md` 의 "검증 필수" 원칙을 **문서 측에도 확장** — 문서만 읽고 "미구현" 이라 가정하지 말 것. 코드 + DB 실측 + 프론트 사용 현황 3자 대조가 스펙 작업 PR 의 사전 필수 스텝.
  - **검증**: 이 PR 은 코드 0줄. `git diff --stat main -- 'src/**/*.rs' 'migrations/*.sql' 'frontend/**/*'` 가 비어있어야 함.

- **2026-04-21 — 작업 큐 재정리 (이 리포 독립 착수 가능 Q1~Q9 소섹션 신설)**
  - **목적**: PR #174/#175 머지 완료 후 이 리포(amazing-korean-api) 내에서 **외부 의존 없이 단독 진행 가능한 작업**을 한 곳에 명시화. M05~M08 시딩 본체(`amazing-korean-books gen_seed_sql.js` 선행 필요)를 기다리는 동안 병행 가능 큐 정리.
  - **`AMK_STATUS.md §8.2`** 하단에 **"이 리포(amazing-korean-api) 독립 착수 가능 큐"** 소섹션 신설. 9건 (Q1~Q9) 을 우선순위·공수·근거 테이블로 정리.
  - **Q1 (높음)**: `?lang=` Consumer API 확장 — `/courses`·`/lessons/{id}`·`/studies/{id}`·`/studies/tasks/{id}`·`/videos/{id}` 에 쿼리 파라미터 + `_translated` 필드 + fallback(lang → en → ko). 스펙은 `AMK_API_LEARNING.md §9-841` 에 확정돼 있으나 "⬜ 미구현" 상태. M05~M08 시딩 완료 시 소비자가 자국어 학습 가능해야 하는 필수 기능.
  - **Q2~Q6 (중간)**: #73 영수증 후속 3건 (법인 인감 이미지 업로드 / 영수증 고유번호 체계 / 기타 locale 영수증 번역) + #75 대리 주문 후속 2건 (사용자 검색 UI / `admin_textbook_log` Create 액션 조회 UI). 각 반나절 공수.
  - **Q7~Q9 (낮음)**: Paddle Live 전환 (KYB 대기) / K6 성능 테스트 (테스트 계정 필요) / E-book 로컬 파일시스템 의존 해소 (RDS 이전 선행).
  - **큐 외 일정 대기**: #67 D+7 관측 로그 체크 (2026-04-23) + Phase 2~5 일괄 전환 (2026-04-24).
  - **SSoT**: `AMK_STATUS.md §8.2` (이 문서). `memory/project_status.md` + `memory/MEMORY.md` 도 동일 구조로 동기화.

- **2026-04-19 — #76 잠복 버그 fix: `study_task_explain` → `study_explain` Rust 코드 참조 일치화**
  - **발견 경위**: PR #174 (#73 영수증 + #75 대리 주문) 작업 중 `cargo sqlx prepare` 실행 시 `relation "study_task_explain" does not exist` 에러. 원본 `migrations/20260208_AMK_V1.sql` + 프로덕션 DB (`docker exec amk-pg-prod psql -c "\dt study*"` 으로 실증) + 로컬 DB 모두 **`study_explain`** 이지만 Rust 코드 16곳이 `study_task_explain` 을 참조 중. 현재 study 콘텐츠가 프로덕션에 없어 이 경로가 호출된 적 없었기에 런타임 에러가 노출되지 않은 장기 잠복 버그.
  - **수정 범위** (총 16곳):
    - **SQL 테이블명 참조** (10곳): `src/api/study/repo.rs` (1), `src/api/admin/study/repo.rs` (7), `src/api/admin/translation/repo.rs` (2) — `FROM`/`JOIN`/`INSERT INTO`/`UPDATE` 절의 테이블명.
    - **audit_log target_table** (5곳): `src/api/admin/study/service.rs` — `write_audit_log(..., "study_task_explain", ...)` 호출 5건. `feedback_audit_log_convention.md` 의 "target_table = 실 테이블명" 원칙에 맞춤.
    - **OpenAPI tag** (5곳): `src/api/admin/study/handler.rs` — `tag = "admin_study_task_explain"` → `tag = "admin_study_explain"`. Swagger UI 카테고리 이름. 내부 전용이라 외부 참조 없음 확인 후 일관성 위해 변경.
  - **변경 대상 아님**: `content_type_enum` 값 `'study_task_explain'` (20260212 마이그레이션에서 추가, `content_translations.content_type` 에 저장된 enum 값) — enum 레이어이지 물리 테이블명과 독립. 유지.
  - **검증**:
    - `DATABASE_URL=... cargo sqlx prepare --workspace -- --all-targets` ✅ — 이전 PR #174 에서 실패하던 쿼리 캐시 이제 생성 성공
    - `cargo check --all-targets` 클린 ✅
    - `cargo clippy --all-targets -- -D warnings` 0 warnings ✅
    - `cd frontend && npm run build` 클린 ✅ (프론트 변경 없음)
  - **교훈**: INC-002 (2026-04-18) 때 "로컬 DB 의 이상 rename 을 실 DB 로 오판" 실수 이후, 로컬/prod DB 상태를 실 명령으로 대조한 덕에 발견. `feedback_migration_safety.md §"2026-04-18 INC-002 추가 교훈"` 의 "Rust 코드·DB 테이블명 cross-check" 원칙이 다음 잠복 버그 예방에 계속 적용될 것.

- **2026-04-19 — 관리자 대리 주문 생성 기능 (`POST /admin/textbook/orders`)**
  - **배경**: 영수증 발급 기능(#73) 과 페어. 외부(전화·이메일·오프라인) 주문을 시스템에 입력하거나, 영수증·통계 관리를 위해 관리자가 직접 주문 생성. 대리 주문을 `paid` 로 즉시 생성 → 영수증 발급 버튼 활성화 → 인쇄.
  - **백엔드**:
    - `textbook/repo.rs InsertOrderParams.user_id: i64 → Option<i64>` — `textbook.user_id` DB 컬럼은 원래 NULLABLE, 비회원 주문 저장 가능. Rust params 만 NOT NULL 로 강제돼 있던 것 정리. 사용자 `create_order` 는 `Some(user_id)` 전달 (동작 불변).
    - `admin/textbook/dto.rs AdminCreateOrderReq` 신규 — `user_id: Option<i64>` (귀속 or NULL), `initial_status: pending|confirmed|paid`, `enforce_min_quantity: bool` (기본 false), 나머지 `CreateOrderReq` 와 동일.
    - `admin/textbook/service.rs::create_order` — 검증(min_quantity enforce 옵션 따름) → `generate_order_code` + `insert_order` + `insert_items` → `initial_status != pending` 이면 `update_status` 호출하여 `paid_at` 자동 세팅 → `admin_textbook_log` 에 `AdminAction::Create` 기록 (after 스냅샷) → confirmation 이메일 fire-and-forget.
    - `admin/textbook/handler.rs::admin_create_order` + `router.rs`: `POST /admin/textbook/orders` 등록.
    - `textbook/service.rs` 의 `UNIT_PRICE`, `MIN_TOTAL_QUANTITY`, `catalog_languages` 를 `pub(crate)` 로 승격 (admin service 재사용).
  - **프론트엔드**:
    - `admin/textbook/types.ts AdminCreateOrderReq` 인터페이스.
    - `admin_api.ts::createAdminTextbookOrder` + `use_admin_textbook::useAdminCreateTextbookOrder` 훅 (onSuccess 시 orders list invalidate).
    - `admin/textbook/page/admin_textbook_order_create.tsx` 신규 — 관리자 옵션(initial_status/user_id/enforce_min_quantity) + 신청자/배송/품목/세금계산서/비고 섹션. 품목 동적 배열. 카탈로그 훅으로 언어 목록 로드 (available: true 만 노출). 제출 성공 시 생성된 주문 상세로 네비게이션.
    - `routes.tsx`: `/admin/textbook/orders/new` 등록. `:orderId` 동적 세그먼트 앞에 배치 (순서 충돌 방지).
    - `admin_textbook_orders_page.tsx` 헤더에 "+ 새 주문 생성" 버튼.
    - i18n ko/en: `admin.textbook.newOrder` + `admin.textbook.create.*` (title/subtitle/adminOptions/initialStatus/userId/enforceMinQty/orderer/delivery/items/taxInvoice/notes/cancel/submit/success/err 등 40여 키). 관리자 섹션 정책 상 다른 locale 스킵.
  - **재사용**: `TextbookRepo::{generate_order_code, insert_order, insert_items, update_status, insert_admin_log}`, `TextbookService::get_order_by_id`, `EmailTemplate::TextbookOrderConfirmation`.
  - **검증**: `cargo check --all-targets` 클린 ✅, `cd frontend && npm run build` 8.95s 클린 ✅ (메인 번들 204kB). `cargo sqlx prepare` 는 별도 잠복 버그 (`study_task_explain` Rust 참조 vs 실 DB `study_explain`) 로 실패 — 이번 변경과 무관, 후속 PR 에서 별도 처리.
  - **후속 작업 (별도 PR)**: (1) 잠복 버그 fix — `study_task_explain` → `study_explain` Rust 9곳, (2) 사용자 검색 UI (`user_id` 수동 입력 → 자동완성), (3) `admin_textbook_log` 에 `Create` 액션 조회 UI.

- **2026-04-19 — 교재 주문 간이 영수증 발급 기능 (`?type=receipt`)**
  - **배경**: 실 주문 1건에서 간이 영수증 발급 요청. 기존 견적서(`?type=quote`) / 주문확인서(`?type=confirmation`) 인쇄 페이지에 3번째 타입 추가로 시스템화.
  - **신규 공급자 상수** — `frontend/src/category/textbook/supplier_info.ts`: HYMN 법인 정보 (상호 `(주) 힘 HYMN Co., Ltd.` / 사업자등록번호 `505-88-03252` / 대표자 `김경륜 (Kyoung Ryun KIM)` / 주소 `세종시 한누리대로 350 6층 SB3호`). 법인 고정값이라 i18n 아닌 상수 파일.
  - **사용자 인쇄 페이지** — `frontend/src/category/textbook/page/textbook_order_print.tsx`: `isReceipt = type === "receipt"` 분기. (a) 공급자 정보 박스 추가, (b) 발행일을 `paid_at` 기준으로 표시, (c) 합계 박스를 공급가액 / VAT (10%) / 합계 3단 분리 (`total_amount / 1.1` 역산), (d) 입금 계좌 박스 비노출, (e) 서명란 (HYMN 대표자명 + 인감 라인) 추가, (f) `paid_at` 이 null 이면 발급 거부 안내.
  - **관리자 인쇄 페이지** — `frontend/src/category/admin/textbook/page/admin_textbook_order_print.tsx`: 위와 동일 분기 적용.
  - **버튼 추가**:
    - 사용자 주문 상태 페이지 (`textbook_order_status_page.tsx`): 기존 견적서/주문확인서 버튼 옆에 영수증 버튼. `order.paid_at != null` 조건으로 조건부 노출.
    - 관리자 주문 상세 (`admin_textbook_order_detail.tsx`): 동일 조건부 노출.
  - **i18n 업데이트**:
    - `textbook.print` (22개 locale 전체): 영수 관련 키 10개 추가 (`receiptTitle`/`receiptTotal`/`receiptNotice`/`receiptUnpaid`/`paidDate`/`supplier`/`supplyAmount`/`vatAmount`/`issuedBy`/`sealLine`). ko/en 완전 번역, 나머지 20개 locale 은 영어 fallback.
    - `admin.textbook.print` (ko/en 만 — 관리자 섹션은 원래 2개 언어 지원): 동일 10개 키.
    - `admin.textbook.printReceipt`: 버튼 레이블.
  - **VAT 분리 표시**: 총액이 VAT 포함이라는 기존 가정 유지. `supplyAmount = Math.round(total_amount / 1.1)` / `vatAmount = total_amount - supplyAmount`. 간이영수증은 세법상 서식 자유이나 실무상 공급가액/세액 분리가 명확한 편이 증빙용으로 유리.
  - **검증**: `cd frontend && npm run build` 9.31s 클린 ✅.
  - **다음 단계 (후속 요청 시)**: 법인 인감 이미지 업로드 기능 (현재는 `(인)` 텍스트), 영수증 고유번호 체계 (현재는 order_code 재사용), 다른 locale 개별 번역 (현재는 영어 fallback).

- **2026-04-18 — 🚨 INC-002: 프로덕션 crash loop (M02 테이블명 오인 + nginx reload race) 2중 복구**
  - **INC-002 발생 타임라인**:
    - 05:15 UTC — PR #172 `b75134e` (M01+M04) 배포 ✅ 성공
    - 05:27 UTC — PR #172 `d009ff8` (M02+M03 추가) 배포 → api 컨테이너 crash loop 시작
    - 05:33 UTC — PR #172 `2970b48` (Gemini 반영) 배포 → 동일 crash loop 지속
    - 05:32~05:54 UTC — 외부 `https://api.amazingkorean.net/health` HTTP 502 (약 20분 다운)
  - **원인 1 — M02 테이블명 오인 (primary root cause)**: `20260419_reset_test_studies.sql` 에서 테이블명을 `study_task_explain` 으로 작성. 프로덕션 실 테이블명은 `study_explain` (원본 `20260208_AMK_V1.sql` 기준). 로컬 DB 의 이상 rename 상태 (`study_task_explain` 로 존재) 를 실 상태로 오판. 프로덕션 마이그레이션 실행 시 `relation "study_task_explain" does not exist` 에러 → api Rust 부팅 시 `sqlx migrate` 실패 → `exit 1` → `restart: always` crash loop.
  - **원인 2 — nginx reload DNS race (secondary)**: `docker compose up -d` 비동기 recreate 직후 `docker exec amk-nginx nginx -t` 가 실행되면 api 컨테이너가 docker internal DNS (127.0.0.11) 에 재등록되기 전이라 `host not found in upstream "api:3000"` syntax error. 기존 deploy.yml 은 이때 `exit 1` 로 워크플로 실패 처리 → GitHub Actions "failure" 시그널. 본질적으로 service 는 nginx 기존 config 로 살아있지만 GA 는 "실패" 로 기록되고 후속 디버깅 혼란.
  - **복구 방안**:
    - **원인 1**: `migrations/20260419_reset_test_studies.sql` 의 `study_task_explain` → `study_explain` 정정 (테이블명 참조 3곳). `content_type = 'study_task_explain'` enum 값은 그대로 유지 (enum 값과 물리 테이블명은 별개). `AMK_SCHEMA_PATCHED.md §2.4.6` 도 원본 `study_explain` 으로 원복.
    - **원인 2**: `.github/workflows/deploy.yml` 의 nginx reload 블록을 **재시도 루프 (6회 × 5초)** + **fail-safe `exit 0`** 로 변경. api DNS 재등록 대기 + 최종 실패 시에도 기존 config 로 서빙되므로 워크플로 실패로 만들지 않음.
  - **로컬 DB 정리**: 로컬의 이상 rename 된 `study_task_explain` 을 `ALTER TABLE study_task_explain RENAME TO study_explain` 으로 prod 와 동기화.
  - **교훈 (feedback_migration_safety.md 에 추가 예정)**:
    1. 마이그레이션 SQL 작성 시 **프로덕션 실 DB 상태를 기준으로 검증**. 로컬 DB 는 누적된 수동 변경으로 실 상태와 다를 수 있음. 작성 전 `docker exec -it amk-pg-prod psql ...` 로 실제 `\dt` 확인이 필수.
    2. nginx reload (volume-mount) 처럼 **docker internal DNS 에 의존하는 step 은 재시도 + fail-safe** 필수. `exit 1` 는 실제 서비스 다운이 있을 때만.
    3. deploy GitHub Actions "failure" ≠ 서비스 다운. 이번처럼 nginx reload 실패만으로도 GA 는 failure 처리되므로, 진짜 서비스 상태는 별도 외부 모니터 (UptimeRobot #71, 2026-04-17 도입) 로 판단.
  - **검증**: `cargo check` 클린. 로컬 DB rename 완료. 코드 변경 4건 (M02 SQL, SCHEMA_PATCHED, deploy.yml, CHANGELOG).

- **2026-04-18 — Gemini 리뷰 반영 (PR #172 MEDIUM 4건)**
  - **`.claude/settings.json`**: 문법 오류가 있는 Bash permission entry 2건 제거 — (1) `perl ... src/... grep -rn ...` 가 하나의 entry 에 병합돼 있어 `grep` 이 `perl` 의 파일 인자로 취급되는 구조, (2) 같은 entry 에 로컬 절대경로 `/home/kkryo/...` 포함. 두 entry 모두 실제 매칭 불가능한 형태였으므로 삭제.
  - **`migrations/20260419_reset_test_studies.sql`**: DELETE 조건을 의도 명시적으로 변경 — (a) `lesson_item WHERE study_task_id IS NOT NULL` → `WHERE study_task_id IN (SELECT study_task_id FROM study_task)`, (b) `admin_study_log` 전량 삭제 → `WHERE admin_pick_study_id IN (SELECT study_id FROM study) OR admin_pick_task_id IN (SELECT study_task_id FROM study_task)` (실 관리자 로그 누적 시 보호), (c) `writing_practice_session WHERE study_task_id IS NOT NULL` → `WHERE study_task_id IN (SELECT study_task_id FROM study_task)`. 현재 study_task 전량 삭제 전 시점에 실행되므로 삭제 범위는 종전과 동일하지만, DELETE 문만 독립적으로 읽어도 의도가 명확해지고 혹시 실 데이터가 존재하는 경우에도 보호됨.
  - **검증**: `cargo check` 클린. SQL 의미 동일성 유지 (study_task IN (SELECT ...) 조건이 study_task 전량 삭제 대상 집합과 일치).

- **2026-04-18 — 교재 500문장 시딩 선행 마이그레이션 4/4 (M03 — `basic_900` → `basic_500` enum 개명)**
  - **M03 — `migrations/20260420_rename_basic_900.sql`**: `ALTER TYPE study_program_enum RENAME VALUE 'basic_900' TO 'basic_500'` (PostgreSQL 10+ atomic). 500문장 교재 시딩을 앞두고 레거시 명칭(실은 500문장) 을 정정.
  - **Rust 동기화**: `src/types.rs` `StudyProgram::Basic900` → `Basic500`, `src/api/study/service.rs` enum 매칭 + 에러 메시지, `src/api/study/handler.rs` Swagger description, `src/api/admin/study/stats/repo.rs` + `dto.rs` 통계 필드 `basic_900` → `basic_500`.
  - **Frontend 동기화**: `frontend/src/category/admin/study/types.ts` Zod 필드 + `frontend/src/category/study/types.ts` + 관리자/학습 페이지 5건 + **i18n locale 22개** (`programBasic900` 키 → `programBasic500`, 레이블 숫자 "900" → "500"; 크메르어 ៩០០ → ៥០០, 미얀마어 ၉၀၀ → ၅၀၀, 네팔어 ९०० → ५०० 등 다국어 숫자 포함).
  - **문서 동기화**: `AMK_API_MASTER §4.7` enum 목록, `AMK_SCHEMA_PATCHED §2.4.1` ENUM 정의 + "(enum값 basic_900은 레거시)" 주석 제거, `AMK_API_FUTURE §시딩 계획` 프로그램 매핑 5곳, `AMK_CODE_PATTERNS §Triple Derive 예시` (serde rename 샘플).
  - **검증**: `cargo check --all-targets` ✅, `cargo clippy --all-targets -- -D warnings` 0 warnings ✅, `cargo sqlx prepare` (enum rename 후 offline 캐시 재생성, 변경 없음 — 쿼리 레벨 변화 없음) ✅, 로컬 DB `SELECT unnest(enum_range(NULL::study_program_enum))` → `basic_500` 포함/`basic_900` 부재 ✅, `cd frontend && npm run build` 9.36s 클린 ✅.
  - **실행 전제**: 본 마이그레이션은 M02 의 test-* 리셋(`basic_900` program 5건 포함 전량 삭제) 직후 실행됨. 따라서 rename 대상 실 데이터 0 행 — enum 타입 정의만 갱신. 프로덕션 적용 시에도 M02 가 선행 필요.

- **2026-04-18 — 교재 500문장 시딩 선행 마이그레이션 2/4 (M02 — test-* 레거시 리셋)**
  - **M02 — `migrations/20260419_reset_test_studies.sql`**: `seeds/20260208_AMK_V1_SEED.sql` 로 주입된 `test-1`~`test-9` 더미 study + 연관 데이터 전량 삭제. `study` / `study_task` IDENTITY 시퀀스 `RESTART WITH 1` 로 리셋하여 후속 시딩(M05~M08) 시점에 `study_id = 1` 부터 할당되도록 함.
  - **삭제 순서**: `content_translations` (study/study_task_*) → `study_task_log` → `study_task_status` → `study_task_explain` → `lesson_item` (task 연결) → `admin_study_log` 전량 → `study_task_choice/typing/voice/writing` → `writing_practice_session` (task 기반만, 자유 연습은 보존) → `study_task` → `study`.
  - **로컬 검증**: 적용 후 `SELECT COUNT(*) FROM study; FROM study_task;` → 0, `SELECT last_value, is_called FROM study_study_id_seq; study_task_study_task_id_seq;` → (1, false) = 다음 INSERT 가 1 할당, 자유 연습 세션 25개 보존 ✅.
  - **스키마 문서 불일치 동기 수정**: `AMK_SCHEMA_PATCHED.md §2.4.6` 에서 `study_explain` 으로 표기돼 있던 3곳을 **실 DB 테이블명 `study_task_explain`** 으로 정정. 원본 `20260208_AMK_V1.sql` 은 `study_explain` 으로 생성하나 어느 시점에 DB 레벨 rename 이 적용됐고 (원인 마이그레이션 파일 미발견), 20260212 이후 신규 마이그레이션은 `study_task_explain` 전제로 작성됨 — 문서가 실 DB 에 뒤처진 상태였음.
  - **프로덕션 적용 전 사전 확인 필수**: `SELECT COUNT(*) FROM study WHERE study_idx NOT LIKE 'test-%'` = 0 확인. 0 이 아니면 실 콘텐츠 존재 가능성 → 별도 백업 전략 선행.

- **2026-04-18 — 교재 500문장 시딩 선행 마이그레이션 1/4 (M01 + M04)**
  - **목적**: 교재(`amazing-korean-books/scripts/textbook/data/sentences.json`) 의 500문장 × 36개 언어 번역 시딩을 앞두고, 실 데이터 INSERT 전 스키마를 정리하는 단계. 총 8개 마이그레이션 (M01~M08) 중 스키마 확장 2건만 우선.
  - **M01 — `migrations/20260418_study_task_idx.sql`**: `study_task` 테이블에 `study_task_idx varchar(100) NOT NULL UNIQUE` 컬럼 추가. 해설집 문장 참조 안정 키 + 재시딩 멱등성 + `content_translations.content_id` 논리 연결 안정성의 3가지 요구를 단일 컬럼으로 해결. 기존 레거시 행은 `'legacy-' || study_task_id` 로 백필 (후속 M02 에서 전량 삭제 예정).
  - **M04 — `migrations/20260421_expand_supported_languages.sql`**: `supported_language_enum` 에 13개 언어 추가 (`tl`, `tr`, `bn`, `ar`, `ur`, `fa`, `lo`, `ky`, `it`, `sw`, `uk`, `am`, `pl`). sentences.json 의 36개 번역 중 기존 enum(22개) 미지원 13개를 커버. `pt_pt` (Portuguese-Portugal variant) 는 `pt` (Brazil) 로 병합 — UX 이득 대비 이중 저장 오버헤드 불분명. `user_language_enum` (UI 언어) 는 별도 정책으로 미확장.
  - **Rust 동기화**: `src/types.rs` `SupportedLanguage` enum 에 13개 variant 추가 (`Tl`, `Tr`, `Bn`, `Ar`, `Ur`, `Fa`, `Lo`, `Ky`, `It`, `Sw`, `Uk`, `Am`, `Pl`). 주석 "21개 → 35개 (ko, en 포함)" 갱신.
  - **문서 동기화**: `AMK_API_MASTER.md §4.8` supported_language_enum 목록 22개 → 35개로 갱신. `AMK_SCHEMA_PATCHED.md §2.4.2` study_task 정의에 `study_task_idx` 행 추가.
  - **검증**: `cargo check --all-targets` ✅, `cargo clippy --all-targets -- -D warnings` 0 warnings ✅, `cargo sqlx prepare` 쿼리 캐시 재생성 (변경 없음 — 신규 enum 값은 아직 쿼리에서 사용 안 됨) ✅, 로컬 DB (`docker exec amk-pg ...`) 에 M01 + M04 적용 성공 — `\d study_task` 에서 `study_task_idx` + `uq_study_task_idx` UNIQUE 인덱스 확인, `enum_range(NULL::supported_language_enum)` 35개 ✅.
  - **다음 단계 (이 PR 범위 밖)**: PR #2 = M02 (test-* 레거시 리셋), PR #3 = M03 (basic_900 → basic_500 enum 개명 + Rust/Frontend 동기화). 그 후 M05~M08 시딩 본체 (sentences.json → SQL 생성 스크립트는 amazing-korean-books 쪽 one-off).

- **2026-04-17 — INC-001 재발 방지 외부 모니터링 구축 (UptimeRobot HTTP 모니터) + nginx gzip 튜닝**
  - **UptimeRobot Free 플랜 HTTP 모니터 세팅** — `https://api.amazingkorean.net/health`, 5분 간격, 이메일 알림 → `amazingkoreancenter@gmail.com`. GitHub Actions deploy "success" false positive 와 독립된 외부 감시 체계. 발사 테스트 (Keyword 모니터 keyword 를 `__DOWN_TEST__` 로 임시 변경) 에서 DOWN 알림 1~2분 내 수신 확인 완료.
  - **`nginx/nginx.conf`**:
    - `gzip_min_length 1024;` 추가 → 작은 응답(< 1KB) 전반 압축 제외. CPU 이익 무 + keyword 매칭 기반 외부 모니터 호환성 개선.
    - `location = /health { gzip off; ... }` 블록 추가 → origin 레벨 명시 (이중 안전장치). 응답 평문 유지로 향후 grey-cloud 도입 시 즉시 keyword 매칭 가능한 기반 보존.
  - **`.github/workflows/deploy.yml`**:
    - `Sync docker-compose.prod.yml + nginx config to EC2` step 의 scp source 에 `nginx/nginx.conf` 추가 (기존엔 `docker-compose.prod.yml` 만 동기화 → nginx 설정 수정해도 EC2 미반영 버그).
    - `Deploy to EC2` step 에 `nginx -t && nginx -s reload` 추가. volume-mount 된 config 는 컨테이너 재시작 없이 반영 안 되므로 명시적 reload. syntax error 시 스킵 + exit 1 (fail-safe).
  - **`docs/AMK_DEPLOY_OPS.md §7.5`** 신규 — 외부 모니터링 구성·발사 테스트 절차·향후 업그레이드 옵션 섹션.
  - **`docs/AMK_STATUS.md §8.1`** #71 완료 행 추가 + #63 INC-001 행의 "Cloudflare 502 알림" 항목을 UptimeRobot 로 해결 표기.
  - **학습 (Keyword 모니터 포기 경위)**: 초기에 Keyword 모니터(`live` substring 검색) 로 세팅했으나 CF Free 플랜 edge 가 `Accept-Encoding: gzip` 요청에 대해 **자체 Brotli/gzip 재압축** 수행 → UptimeRobot probe 가 raw 바이트에서 `live` 검색 실패 → 영원히 DOWN. 3가지 회피 시도 후 불가 확정:
    1. nginx `gzip off` → CF 가 edge 에서 재압축, 무관.
    2. `add_header Cache-Control "no-transform" always;` → curl 실측에서 origin 응답에 헤더가 안 나옴 (nginx add_header 가 `proxy_pass` 와 조합 시 이슈인지, CF 가 strip 하는지 불명. 추가 디버그 ROI 낮음).
    3. CF Custom Request Headers (UptimeRobot `Accept-Encoding: identity`) → CF Pro 이상만 가능, Free 불가.
  - **현실적 판단**: INC-001 감지 목적은 HTTP 상태코드 200 모니터로 100% 달성 (컨테이너 crash → CF 521 → 200 아님 → DOWN). Keyword 가 추가로 커버하는 "200 OK + 이상한 body" 는 현실적 발생 빈도 낮음. 과도한 엔지니어링 회피.
  - **향후 업그레이드 옵션**: Cloudflare Pro 구독 시 CF Health Checks (60초 간격) 로 전환 가능. 또는 `origin-*` grey-cloud 서브도메인 + 자체 SSL 로 CF 우회 시 Keyword 복원 가능 (작업 공수 중).

- **2026-04-16 — JsonRejection → AppError 통합: `AppJson<T>` 커스텀 extractor 로 에러 응답 envelope + trace_id 전 경로 통일**
  - **배경**: #69 trace_id 구현 런타임 검증에서 발견된 갭. axum `Json<T>` extractor 는 JsonRejection 시 `text/plain` 응답을 직접 반환 → `AMK_API_MASTER §3.4` 표준 에러 envelope (`code/http_status/message/details/trace_id`) 과 `x-request-id` body 매칭 규약 우회. 프론트엔드 에러 파싱 분기 불가능.
  - **신규 `src/extract.rs`** — `AppJson<T>(pub T)` + `FromRequest` 구현. 내부적으로 `Json<T>::from_request()` 호출하고 `JsonRejection` 을 `map_json_rejection()` 으로 `AppError` 변환:
    - `JsonDataError` (필드 누락/타입 불일치) → `AppError::Unprocessable` (422, "Invalid JSON data: ...")
    - `JsonSyntaxError` (깨진 JSON) → `AppError::BadRequest` (400, "Malformed JSON: ...")
    - `MissingJsonContentType` → `AppError::BadRequest` (400, "Expected Content-Type: application/json")
    - `BytesRejection` → `AppError::BadRequest` (400, "Failed to read request body: ...")
    - 기타 (non-exhaustive 대비) → `AppError::Unprocessable` (body_text 전달)
  - **`src/lib.rs`** — `pub mod extract;` 추가.
  - **19 핸들러 파일 × 80 call sites 치환** — `Json(x): Json<T>` (요청 바디 extractor) → `AppJson(x): AppJson<T>`. 응답 직렬화 `Json(res)` 는 실패 가능성 없어 `axum::Json` 그대로 유지 (변경 X). 치환 방식: `perl -i -pe 's/Json\(([a-zA-Z_][a-zA-Z0-9_]*)\): Json<([^>]+)>/AppJson(\1): AppJson<\2>/g'` + 파일별 `use crate::extract::AppJson;` 임포트 삽입. 대상 파일: auth / user / video / study / lesson / course / textbook / ebook / payment 일반 9개 + admin/email / admin/translation / admin/video / admin/lesson / admin/user / admin/upgrade / admin/textbook / admin/study / admin/ebook / admin/payment 10개 = 19개.
  - **`docs/AMK_CODE_PATTERNS.md`** — §1.3(에러 처리 핵심 포인트+규칙) 에 `AppJson<T>` 문단 추가, §1.4(handler 예제) login 시그니처에 `AppJson(req): AppJson<LoginReq>` 반영 + 주석 블록으로 사용 규약 설명, 파일 개요 표의 Extractor 열을 `Json` → `AppJson` 로 일괄 갱신.
  - **`docs/AMK_STATUS.md`** — §8.1 #70 완료 행 추가, §8.2 #20 백로그를 strikethrough + ✅ 완료 참조로 갱신.
  - **검증**: `cargo check --lib` 11.77s + `cargo clippy --lib --bins` 24.53s **0 warnings**. 기존 `axum::Json` import 는 응답 직렬화에서 여전히 사용 중이라 unused-import 경고 없음.
  - **런타임 검증 (로컬 `127.0.0.1:3100` + `curl -i` 4/4 전부 통과)**:
    1. 필드 누락 `POST /auth/login -d '{}'` → **이전** 422 `text/plain`, **현재** 422 JSON `{"error":{"code":"UNPROCESSABLE_ENTITY","http_status":422,"message":"Invalid JSON data: Failed to deserialize the JSON body into the target type: missing field \`email\` at line 1 column 2","trace_id":"test-data-err"}}`
    2. JSON 문법 오류 `-d '{invalid json}'` → 400 `BAD_REQUEST` "Malformed JSON: Failed to parse the request body as JSON: key must be a string at line 1 column 2"
    3. Content-Type 누락 → 400 `BAD_REQUEST` "Expected Content-Type: application/json"
    4. 유효 JSON + 잘못된 자격 → 401 `UNAUTHORIZED` AUTH_401_BAD_CREDENTIALS (기존 핸들러 AppError 경로, 회귀 없음)
  - **효과**: 프론트엔드는 모든 에러 응답을 단일 JSON 스키마로 파싱 가능. `x-request-id` body ↔ 헤더 매칭이 JsonRejection 경로에서도 성립해 trace_id 기반 디버깅 커버리지 100% 달성.

- **2026-04-16 — 요청 trace_id 구현: `task_local!` + UUID v7 미들웨어로 에러 응답/로그 상관추적**
  - **신규 `src/trace_id.rs`** — `tokio::task_local! REQUEST_ID: String` + `middleware` async fn + `current() -> Option<String>` + `TraceId(String)` newtype. 들어오는 `x-request-id` 헤더가 유효(ASCII alphanumeric/`-`/`_`, ≤128자) 하면 승계, 없으면 `uuid::Uuid::now_v7().to_string()` 생성. `task_local.scope(id, next.run(req))` 로 하위 전 지점에서 `crate::trace_id::current()` 동기 조회 가능. 응답 헤더 `x-request-id` 로 에코백.
  - **`src/error.rs:210`** — `"trace_id": "req-TODO"` 하드코딩 플레이스홀더 → `crate::trace_id::current().unwrap_or_else(|| "unknown".to_string())` 로 교체. 표준 에러 바디(`AMK_API_MASTER §3.4`) 의 `trace_id` 필드가 드디어 실 값 전달. INC-001 (2026-04-15 2h33m 다운) 진단 속도 저하 원인 중 하나 제거.
  - **`src/main.rs`** — trace_id 미들웨어를 CORS + `security_headers` 보다 **바깥쪽** 에 바인딩 (요청 진입 시 가장 먼저, 응답 나갈 때 가장 마지막). CORS `allow_headers` 에 `x-request-id` 추가(업스트림 승계용) + `expose_headers([x-request-id])` 추가(브라우저 fetch 에서 응답 헤더 읽기 허용).
  - **`src/lib.rs`** — `pub mod trace_id;` 추가.
  - **`Cargo.toml`** — `uuid` feature 에 `"v7"` 추가 (기존 `"v4"` 유지). UUID v7 은 시간 정렬 형식으로 로그/DB 인덱스 조회에 유리.
  - **`docs/AMK_CODE_PATTERNS.md:319`** — `"PLACEHOLDER"` 주석 + `req_id` 변수 미정의 제거. 실제 구현 참조 주석 (`src/trace_id.rs` 의 `task_local!` 에서 주입) 로 교체. 에러 처리 섹션 "🔑 핵심 포인트" 에 trace_id 문단 4줄 추가 (미들웨어 위치, 승계 규칙, 응답 헤더, Extension 추출).
  - **`docs/AMK_STATUS.md §8.1`** — 완료 항목 #69 행 추가.
  - **스코프 변경**: `project_status.md` 의 D+4~D+5 예정 🅱️ 작업을 D+0 앞당겨 처리 (Gemini 백로그 완료 후 잔여 여력). 🅰️ E-book session_id 필수화 Phase 1 관측 일정과 완전 독립.
  - **정적 검증**: `cargo check` 31.65s + `cargo clippy --lib --bins` 28.37s 둘 다 0 warnings 클린. 프론트엔드 build 영향 없음 (에러 응답 스키마 동일, `trace_id` 필드는 이미 존재).
  - **런타임 검증 (로컬 `127.0.0.1:3100` 실기동 + `curl -i` 6 시나리오 전부 통과)**:
    1. 헤더 누락 404 → 신규 UUID v7 생성 (예: `019d947b-7db2-7121-9bba-8f819a65dffa`, 3번째 그룹 첫 nibble=`7` 버전, 4번째 그룹 첫 nibble=`9` RFC 4122 variant 확인)
    2. 유효한 업스트림 `x-request-id: cf-ray-abc123-DEF_456` → 그대로 승계 (응답 헤더 + body `trace_id` 양쪽)
    3. 악성 업스트림 값 `hack; rm -rf /` → 형식 검증 거부, UUID v7 재생성
    4. 200자 초과 업스트림 값 → 길이 검증 거부, UUID v7 재생성
    5. body `trace_id` ↔ 응답 헤더 `x-request-id` 동일성 (MATCH) — `task_local!` scope 가 핸들러/에러 변환 전 지점에서 동일 값 조회함을 증명
    6. 핸들러 경유 AppError (`POST /auth/login` 잘못된 자격 → 401 `AUTH_401_BAD_CREDENTIALS`) → 업스트림 ID `handler-path-2` body + 헤더 반영
  - **알려진 갭 (별개)**: Axum `Json<T>` extractor 실패 (잘못된 JSON 바디 → 422) 는 표준 에러 envelope 을 우회해 `text/plain` 응답으로 빠져나감. `x-request-id` 응답 헤더는 붙으나 body 는 AppError 스키마 아님. trace_id 구현과 무관한 기존 동작. 향후 `JsonRejection` 커스텀 추출기로 AppError 매핑 시 전 경로 통일 가능 (`AMK_STATUS.md §8.2 보류/조건부` 에 등록).

- **2026-04-16 — Gemini 백로그 auth 성능 MEDIUM 5건 반영: Redis 파이프라인/배치 + JWKS DCL**
  - **PR #157 L190 (exists 파이프라인화 × 2)** — `enforce_session_limit` 의 유령 세션 탐지 2단계(`ak:session` 존재 확인 + `ak:refresh` 존재 확인) 를 `redis::pipe()` 로 묶어 단일 파이프라인으로 전송. 세션 N개일 때 네트워크 왕복 2N → 2 회로 감소. `refresh_hashes` 에 없는 sid 는 DB 레코드 자체가 없어 보존 근거 없음 → 파이프라인 체크 전에 즉시 유령 처리.
  - **PR #157 L216 (유령 정리 SREM 배치)** — `redis_conn.srem(&session_key, &ghost_ids)` 로 단일 호출. 기존 sid 별 루프(N 왕복) → 1 회.
  - **PR #157 L264 (eviction DEL+SREM 배치)** — Learner 세션 초과 시 FIFO eviction 에서 각 세션별 3 키 삭제를 DEL one-shot(2N 키) + SREM one-shot(N 멤버) 으로 전환. 3N → 2 회.
  - **PR #157 L1044 (logout_all DEL+SREM 배치)** — `logout` 의 `sessions_to_invalidate` 루프를 DEL one-shot + SREM one-shot 으로 전환. 세션 수에 비례하는 왕복 → 2 회. `refresh_hashes.get(sid).is_none()` 케이스는 DEL 키 목록에서 제외해 Redis 서버에 불필요한 NIL 키 전달 방지.
  - **PR #157 L92 (apple.rs JWKS Double-Checked Locking)** — `get_decoding_key` 의 write-lock 획득 직후 재확인 (`if let Some(key) = cache.get(kid) { return Ok(key.clone()); }`) 추가. read-lock 해제와 write-lock 획득 사이에 다른 동시 요청이 이미 JWKS 를 적재했을 경우 중복 `DecodingKey::from_rsa_components` 생성/삽입 회피.
  - **Soundness 검증**: `session_ids: Vec<String>` 를 `.into_iter()` 로 소비하나 이후 `session_expired_candidates` 만 사용 → 안전. `ghosts` 순서는 DB UPDATE 가 set 의미라 무관. 배치 SREM 은 존재하지 않는 멤버 무시.
  - **검증**: `cargo check` 7.85s + `cargo clippy --lib --bins` 0 warnings 13.92s (최초 clippy `filter_map_bool_then` 1건 → `filter + map` 으로 리팩터 후 클린).

- **2026-04-16 — Gemini 백로그 MEDIUM 2건 반영: robots 크롤러 배열화 + security_headers 정적 검증**
  - **PR #161 L148 (robots.txt 크롤러 배열화)** — `src/api/mod.rs::robots_txt` 의 하드코딩된 긴 문자열 리터럴 본문을 `const CRAWLERS: &[&str]` 배열 + `format!`/`join("\n")` 구조로 전환. 크롤러 추가/삭제 시 한 줄 편집으로 가능. 출력 본문은 바이트 단위 동일 (각 엔트리 끝 `\n` + join 사이 `\n` → 기존 빈 줄 포맷 유지).
  - **PR #161 L223 (security_headers 정적 검증)** — `src/main.rs::security_headers` 의 5개 `.parse().unwrap()` 호출을 `HeaderName::from_static` + `HeaderValue::from_static` 패턴으로 전환. 런타임 panic 방지 + 컴파일 타임 헤더 이름/값 유효성 검증. 대상 헤더: `x-content-type-options`, `x-frame-options`, `x-xss-protection`, `permissions-policy`, `x-robots-tag`.
  - **검증**: `cargo check` 6.81s 클린 + `cargo clippy --lib --bins` 0 warnings 13.87s. robots.txt 응답 본문 형식 동일성 수동 확인.

- **2026-04-16 — Gemini 백로그 문서 MEDIUM 3건 반영 + 2건 NIT**
  - **PR #162 L27 (BingBot → Bingbot)**: `docs/AMK_CHANGELOG.md` SEO hardening 검증 항목의 `BingBot` 표기를 `Bingbot` 으로 수정. 검색 엔진 공식 명칭 + 내부 일관성.
  - **PR #158 L319 (`req_id` placeholder 명확화)**: `docs/AMK_CODE_PATTERNS.md:319` 의 `"trace_id": req_id` (변수 미정의) 를 `"trace_id": "PLACEHOLDER" // 실제 구현 시 AppError 필드 또는 Axum Extension 에서 추출한 ID 사용 (현재 src/error.rs:210 은 placeholder 상태)` 로 변경.
  - **PR #158 L560 (`payment` → `transactions` 테이블명)**: `docs/AMK_SCHEMA_PATCHED.md:560` `user_course_pay_id` 주석의 `Paddle/payment 테이블` 를 `Paddle/transactions 테이블` 로 수정. 스키마에 `payment` 테이블은 없고 실제 결제는 `transactions` (`migrations/20260215_payment_system.sql:57`) 에 저장.
  - **NIT 2건 (PR #162 L29 + PR #158 L77, 메모리 파일 외부 참조)**: 1인 CEO 프로젝트 + 메모리 시스템은 CLAUDE.md 에 정의된 개인 컨텍스트 저장소로 의도적 분리 + 교훈 핵심 요약은 이미 CHANGELOG/STATUS 본문에 인라인 포함.

- **2026-04-16 — Gemini 백로그 i18n MEDIUM 4건 반영: kk/hi/mn locale 자연스러움 개선**
  - **PR #163 L744 (kk)** — `frontend/src/i18n/locales/kk.json:744` `notFoundDesc` 의 `ISBN мұрақаты` (= "ISBN 아카이브") → `ISBN нөмірі` (= "ISBN 번호") 로 수정. 카자흐어 단어 의미 오류 (archive ↔ number).
  - **PR #163 L722, L727 (hi)** — `frontend/src/i18n/locales/hi.json:722,727` 의 `टू-फैक्टर` (Two-Factor) 표기를 `टू-फ़ैक्टर` (nukta `फ़` 사용) 로 변경. 같은 mfa 섹션 내 title (line 702) 과의 표기 일관성 확보.
  - **PR #163 L804 (mn)** — `frontend/src/i18n/locales/mn.json:804` `nextButton` 의 `Дараа нь` (= "afterwards") → `Дараах` (= "Next", UI 관습) 로 수정.
  - **검증**: i18n JSON 3 파일 모두 string value 내부 텍스트만 변경 — JSON 구조 무영향. 추후 frontend 빌드 시 자동 검증.

- **2026-04-16 — Gemini 백로그 HIGH 2건 반영: `login_session_id` UUID 인덱스 활용**
  - **PR #157 L578/L603 HIGH 2건** — `src/api/auth/repo.rs::find_login_refresh_hashes_by_session_ids` (SELECT) 와 `update_login_states_by_sessions` (UPDATE) 가 `WHERE login_session_id::text = ANY($1)` 로 컬럼 측 캐스팅을 사용. `login_session_id uuid UNIQUE NOT NULL` 의 자동 UNIQUE 인덱스가 무력화되어 풀 테이블 스캔 위험.
  - **수정**: 컬럼 측 캐스팅 제거 + 파라미터 측 캐스팅 (`WHERE login_session_id = ANY($1::uuid[])`). `&[String]` (sqlx 인코딩 = `text[]`) 을 SQL 단에서 `uuid[]` 로 변환 → 컬럼 비교는 `uuid = uuid` 유지 → UNIQUE 인덱스 사용. SELECT 의 `login_session_id::text` 출력 캐스팅은 반환 타입 `(String, String)` 매칭 위해 유지 (인덱스 영향 없음).
  - **호출부 영향 범위 검증**: 5곳 모두 입력이 UUID 문자열 보장. (a) `service.rs:179,896,1272` — `redis_conn.smembers("ak:user_sessions:{uid}")` (Redis SET 멤버는 login_session_id), (b) `service.rs:999` — `find_user_session_ids_tx(uid)` (DB 쿼리), (c) `service.rs:1031` — `sessions_to_invalidate` (위 두 경로의 합). `::uuid[]` 캐스팅 시 invalid UUID 노출 가능성 0.
  - **시그니처 변경 없음**: `&[String]` 유지 — Gemini 가 제안한 `&[Uuid]` 변경은 5개 호출부 + 호출부의 String→Uuid 변환 로직 추가가 필요해 영향 범위 확대. 파라미터 측 SQL 캐스팅이 동등한 인덱스 효과를 가져와 더 작은 변경 surface 선택.
  - **검증**: `cargo check` 7.25s + `cargo clippy --lib` 0 warnings 13.88s. 통합 테스트는 마이그레이션 + DB 실행 필요라 별도 — 두 함수 모두 sqlx::query/query_as (런타임 SQL) 라 컴파일 타임 검증 없으나, `::uuid[]` 는 PostgreSQL 표준 캐스트로 검증 부담 낮음.
  - **남은 Gemini 백로그**: MEDIUM 16건 (D+2~3 세션에서 처리 예정 — `project_gemini_review_backlog.md`).

- **2026-04-16 — #67 E-book session_id 필수화 Phase 1 관측 로깅 배포**
  - **목적**: `verify_session(session_id: Option<&str>)` 의 `Option` 제거 + `None → Forbidden("Missing session header")` 로 fail-closed 전환하기 전에 프로덕션 트래픽 표본으로 미전송 케이스 0건 확인. INC-001 (2026-04-15 프로덕션 2h33m 다운) 경험 반영 — "fail-closed 게이트 추가는 코드 분석만 신뢰하지 말고 프로덕션 로그로 선확인" 방침.
  - **코드 변경**: `src/api/ebook/service.rs:504` `verify_session` 진입부에 `session_id.is_none()` 분기 추가 — `tracing::warn!(user_id, "EBOOK_SESSION_AUDIT: verify_session called without x-ebook-session header")`. 기존 로직(미제공 시 Redis 키 존재만 확인)은 그대로 유지. doc 코멘트의 `TODO` 에 전환 목표일 `2026-04-24` 명시.
  - **모바일/데스크탑 리포 사전 grep 결과**:
    - `amazing-korean-mobile/lib/api/ebook_api.dart:50,68` — `required String sessionId` + `'X-Ebook-Session': sessionId` 항상 전송 → 안전 ✓
    - `amazing-korean-desktop/src/category/ebook/ebook_api.ts:76,92,114,132` — 웹과 동일한 optional 패턴 (`sessionId?: string` + `...(sessionId ? {...} : {})`). `ebook_viewer_page.tsx:404` 에서 `meta?.session_id` 를 sessionId 변수에 넣고 모든 호출에 전달 → meta 로드 후엔 항상 sessionId 있음. 다만 D+8 Phase 2 일괄 전환 시 데스크탑 `ebook_api.ts` + `use_page_image.ts` + `ebook_viewer_page.tsx` 도 웹과 동일하게 필수화 필요.
  - **관측 계획**: 5~7일 (D+0 ~ D+7=2026-04-23). 완료 조건 `docker logs amk-api --since 168h 2>&1 | grep EBOOK_SESSION_AUDIT | wc -l` → **0건**. 0건 아닐 시 샘플 user_id/UA/시간대 분석. D+8=2026-04-24 Phase 2~5 일괄 전환 (백+웹+데스크탑 동일 PR 동시 배포).
  - **검증**: `cargo check` 14.86s 클린. 핵심 변경은 조건부 로그 1개라 회귀 위험 최소.
  - **메모리**: `feedback_security_patterns.md` §session_id 필수화 마이그레이션 Phase 1 진행 중 표기 유지. `project_status.md` D+0 단계 갱신 (별도 커밋).

- **2026-04-16 — 속도 개선 Phase S5: Swiper CSS 분리 + Pretendard 비동기 + 추가 lazy 5개**
  - **S5-1 Swiper CSS 지연 로딩**: `frontend/src/index.css` 의 `@import "swiper/css"` 4줄을 제거하고 실제 사용처 `frontend/src/category/textbook/page/seal_list.tsx` 로 이동. 카탈로그 페이지(`TextbookCatalogPage`/`EbookCatalogPage`)도 eager → lazy 로 전환해 Swiper CSS 가 비-Book 페이지에서 완전히 제외되도록 함. `frontend/src/vite-env.d.ts` 신규 — `swiper/css` 등 sub-path CSS import 의 TypeScript 타입 선언 (`vite/client` 기본 타입은 sub-path 미커버).
  - **S5-2~S5-3 SKIP**: 히어로 이미지 preload (book-hub) 와 cover srcset 검토 결과 — 히어로 이미지는 이미 `loading="eager"` + `fetchPriority="high"` 적용됨(S3) + URL 이 i18n 언어 기반 동적 결정이라 HTML 정적 preload 불가. cover 이미지는 이미 40KB webp 로 충분히 작아 srcset 효과 미미. **FCP 개선이 우선 병목**으로 판단해 폰트/번들 최적화에 집중.
  - **S5-4 Pretendard 비동기 로딩**: `frontend/index.html` Pretendard CSS 를 `media="print" onload="this.media='all'"` 패턴으로 전환 (Noto Color Emoji 와 동일 패턴). render-blocking 제거. preload hint 는 유지해 다운로드 자체는 조기 시작. `<noscript>` fallback 추가.
  - **S5-5 SKIP**: Critical CSS inline (`critters` 등) 은 SPA 에서 비실용적 — 빌드 타임 HTML body 가 `<div id="root"></div>` 만 있어 분석 불가. 진정한 효과를 보려면 SSR/prerender (Cloudflare Workers, vite-plugin-ssg) 도입이 필요한데 이는 아키텍처 변경 수준이라 본 Phase 스코프 밖.
  - **S5-6 추가 lazy 전환**: 측정 중 book-hub FCP 8s 비정상값 발견 — `BookHubPage`/`BookLandingPage`/`AboutPage`/`LoginPage`/`SignupPage` 5개를 eager → lazy 로 전환 (HomePage 만 eager 유지, 랜딩 페이지). `frontend/src/app/routes.tsx` 정적 import 5개 → `lazy(() => import(...))` 으로 변경.
  - **번들 크기 변화**: `index-*.js` (메인 청크) **277KB → 199KB (-78KB, -28%)**. Swiper CSS 가 `vendor-swiper-*.css` (4.7KB) 로 분리되어 비-Book 페이지에서 다운로드 안 됨.
  - **로컬 Lighthouse 측정 결과** (S4 Prod 베이스라인 → S5 Local v3, 비교 제한적이나 개선 트렌드 명확): home **64→87** (FCP 5785→2612ms), faq **72→88** (FCP 4039→2719ms), book-hub **61→76** (LCP 9287→4375ms, **-53%**), textbook **67→74** (LCP 7644→4058ms), ebook **64→71** (LCP 8228→4391ms). 90+ 목표 부분 달성 (home/faq 근접). 잔여 갭은 SPA 구조적 한계 (CSS 66KB render-blocking + JS 199KB 파싱 시간) 로 SSR 없이는 추가 개선 어려움.
  - **검증**: `npm run build` 8.27s 클린, 8 페이지 Lighthouse 측정 모두 통과 (Perf 71~88).
  - **메모리 갱신**: `project_perf_plan.md` — S5 완료 표기, 잔여 90+ 미달분은 SSR 도입 시점으로 이연.

- **2026-04-16 — 후속 작업 3건: DEPLOY_OPS 환경변수 동기화 + 배포 헬스체크 + sitemap 보강**
  - **DEPLOY_OPS §4 환경변수 표 동기화**: REVENUECAT_API_KEY + REVENUECAT_WEBHOOK_AUTH_TOKEN 2건 주석 해제 (deploy.yml 활성화 반영). PADDLE_DISCOUNT_MONTH_3/6/12 3건 추가 (deploy.yml에만 존재하던 누락분).
  - **deploy.yml 배포 후 헬스체크 단계 추가**: `deploy` job에 Health check step 신규. SSH로 EC2 내부 `curl http://localhost:3000/health` 실행, 최대 5회 재시도(10초 간격). 실패 시 `docker compose logs --tail=50` 출력 후 workflow 실패 처리. INC-001(2h33m 다운, 2026-04-15) 재발 방지 — 당시 `docker compose up -d` exit 0으로 "success" 표기돼 탐지 실패했던 문제 해소.
  - **sitemap.xml에 /book, /book/textbook, /book/ebook 3건 추가**: #34 Book 허브 라우트 재구성(2026-03-25) 이후 누락됐던 경로. priority: /book 0.8, /book/textbook·/book/ebook 0.7, changefreq monthly.

- **2026-04-15 — SEO hardening: Cloudflare Managed robots.txt 우회 + X-Robots-Tag 전역 미들웨어**
  - **배경**: 커밋 `c8014df` 의 `GET /robots.txt` 핸들러(`User-agent: *\nDisallow: /\n`) 배포 후 외부 검증 중 발견 — Cloudflare 가 zone 레벨에서 `# BEGIN Cloudflare Managed content` 블록을 **본문 앞에 자동 주입**하고 있었다. 주입된 블록의 `User-agent: *` + `Allow: /` 가 우리의 `User-agent: *` + `Disallow: /` 와 경로 길이가 같아 Google 의 tie-breaking 규칙(`Allow` 승리)에서 **우리의 Disallow 가 무력화**됨. 프론트엔드(`amazingkorean.net/robots.txt`) 도 동일 주입 확인 — zone-wide 설정.
  - **원인**: Cloudflare 의 "Content Signals" / "Managed robots.txt" 신규 기능 (2024~2025 롤아웃). 사용자가 의도적으로 활성화했는지 Cloudflare 가 자동으로 켰는지 기록 없음.
  - **Fix 1 — `src/api/mod.rs::robots_txt` 확장 (방법 A)**: `User-agent: *` 한 블록 대신 **명시적 크롤러 이름 10종** 각각 `Disallow: /` 를 나열. RFC 9309 + Google 의 robots.txt 파싱 규칙상 **더 구체적인 user-agent 그룹이 우선**하므로, Cloudflare 가 `*` 블록을 앞에 주입해도 Googlebot/Bingbot 등은 자기 이름이 명시된 그룹을 따른다. 등록 크롤러: `Googlebot`, `Googlebot-Image`, `Googlebot-News`, `Googlebot-Video`, `AdsBot-Google`, `Bingbot`, `DuckDuckBot`, `Yeti`(Naver), `NaverBot`, `Daum`(Kakao) + 마지막에 `*` 폴백. 한국 시장 메인 검색엔진 전수 대응.
  - **Fix 2 — `src/main.rs::security_headers` 미들웨어 확장 (방법 B-2 전역 주입)**: 기존 보안 헤더 미들웨어(x-content-type-options, x-frame-options, x-xss-protection, permissions-policy) 에 `x-robots-tag: noindex, nofollow` 한 줄 추가. **Axum `.layer()` 체인이므로 api 서브도메인 모든 응답에 자동 주입됨**. Cloudflare 는 응답 본문(robots.txt) 은 조작하지만 **HTTP 헤더는 우회 불가**. robots.txt 와 독립적으로 작동하는 2중 방어.
  - **Fix 3 — Cloudflare Dashboard 설정 (방법 C, 사용자 작업)**: 사용자가 Cloudflare Zone 설정에서 Managed robots.txt 주입 기능 자체를 끄거나 `api.*` 서브도메인에서만 제외하는 Transform Rule 추가. 이번 배포 스코프 밖, 사용자 후속 처리 예정.
  - **검증**: `SQLX_OFFLINE=true cargo check` 6.72s + `cargo clippy --lib` 0 warnings 11.53s. 재배포(PR #161 머지, 3m31s success) 후 외부 검증 통과:
    - `curl -I https://api.amazingkorean.net/` → `x-robots-tag: noindex, nofollow` ✅
    - `curl -I https://api.amazingkorean.net/health` → `x-robots-tag: noindex, nofollow` ✅ (전역 적용 확인)
    - `curl -I https://api.amazingkorean.net/auth/google` → `x-robots-tag: noindex, nofollow` ✅
    - `curl "https://api.amazingkorean.net/robots.txt?bust=..."` (캐시 우회) → Googlebot/Bingbot/DuckDuckBot/Yeti/NaverBot/Daum 블록 모두 포함된 확장된 본문 ✅ (서버는 확장된 버전 정상 응답)
    - `/courses` → 301 /book + 기존 엔드포인트 회귀 없음 ✅
  - **Cloudflare 캐시 관찰**: 일반 요청은 `cf-cache-status: HIT` + `age: 1310s` + `cache-control: max-age=14400` (4h TTL 자동) → 04:21:45 UTC 캐시 버전 서빙 중. 4시간 후 자동 만료, 또는 Cloudflare Dashboard 수동 퍼지 가능. X-Robots-Tag 가 이미 결정적 차단 역할을 하므로 캐시 만료 대기가 가장 안전한 선택.
  - **Cloudflare 관리형 robots.txt 상태 확인**: AI Crawl Control 페이지에서 토글 ON 확인. zone-wide 단일 토글, 호스트/경로 제외 미지원. 프론트엔드 AI 봇 자동 차단 유지 혜택 때문에 **OFF 하지 않고 유지** (저희 코드 수정이 Cloudflare 와 독립 작동하도록 설계됨). AI Crawl Control 대시보드 지표: 지난 24시간 AI 크롤러 23건 감지, 15건 HTTP 403 차단, Bingbot 7건 허용 (75% 증가), OAI-SearchBot 1건 허용.
  - **중요**: 이 수정은 **백엔드 전체 응답에 `X-Robots-Tag: noindex, nofollow`** 를 붙이므로, 만약 향후 api 경로 중 **Google 에게 색인되길 원하는 공개 엔드포인트**가 생기면 해당 엔드포인트에서 헤더를 override/제거해야 한다. 현재는 전 api 가 색인 대상이 아니므로 문제 없음.
  - **교훈 메모리**: `feedback_seo_api_subdomain.md` 신규 — "API 서브도메인 색인 차단은 X-Robots-Tag 전역 헤더 메인 + robots.txt 보조. Cloudflare 관리형 robots.txt 주입 우회 전략".

- **2026-04-15 — SEO 3건 수정 (Google Search Console 유효성 검사 실패 대응)**
  - **배경**: Search Console "페이지 색인이 생성되지 않는 이유" 대시보드에서 3건 발견. "리디렉션이 포함된 페이지" 3건(`/intro`, `/register`, `http://amazingkorean.net/`) 은 모두 `_redirects` 의 301 이 정상 작동 중으로 확인 — Google 이 "수정 완료" 버튼을 잘못 눌렀을 때 발생하는 validation 루프라 조치 불필요(무해). 실제 문제는 Soft 404(1건) + 404(1건) = 2건.
  - **Fix 1 — `/courses` Soft 404**: React Router catch-all(`<Route path="*">`) 이 200 응답 + NotFoundPage 를 반환해 Google 이 Soft 404 로 분류. 과거 "수강권 상품 목록" 을 가리켰을 레거시 경로. `frontend/public/_redirects` 에 `/courses /book 301` 추가. 현재 `/pricing` 은 ComingSoonPage 라 의미 일치보다 **실 콘텐츠 `/book` 허브**로 보내 Soft 404 재발 방지. 수강권 콘텐츠 시딩 이후 `/pricing` 실페이지 전환 시 목적지 재검토.
  - **Fix 2 — `api.amazingkorean.net/` 404**: Google 이 이 URL 을 알게 된 경로 전수 조사 결과 (1) `GET /auth/google` JSON 응답의 `auth_url` 에 `redirect_uri=https%3A%2F%2Fapi.amazingkorean.net%2Fauth%2Fgoogle%2Fcallback` 평문 노출, (2) Google Cloud Console OAuth Client 의 승인된 Redirect URI 등록, (3) Search Console 도메인 속성(sc-domain:amazingkorean.net) 의 서브도메인 자동 감지 — 3경로 모두 차단 불가(OAuth 서비스 필수 + DNS 공개). 대응: 서버 레벨 처리.
    - `src/api/mod.rs` 에 `GET /` 추가 — 200 JSON 서비스 메타 응답 (`{"service":"Amazing Korean API","status":"ok","docs":null}`). 과거 fallback_404 처리 → Search Console "찾을 수 없음 404" 탈출.
    - `src/api/mod.rs` 에 `GET /robots.txt` 추가 — `User-agent: *\nDisallow: /\n` 로 전체 크롤링 금지. Google 봇이 이후 api 서브도메인 어떤 URL 도 크롤링하지 않음.
    - 두 핸들러 모두 `use axum::{http::{header, StatusCode}, ..., Json}` + `serde_json::json!` 사용. Content-Type 명시로 robots.txt 는 `text/plain; charset=utf-8`.
  - **검증**: `SQLX_OFFLINE=true cargo check` + `cargo clippy --lib` 0 warnings + `cd frontend && npm run build` 클린. `frontend/dist/_redirects` 복사 확인. 머지 후 프로덕션 재배포 시 `curl https://api.amazingkorean.net/robots.txt` → 200 + `curl https://amazingkorean.net/courses` → 301 Location:/book 예상.
  - **Search Console 영향**: 다음 크롤링(며칠 내)에 (1) `/courses` 는 "리디렉션이 포함된 페이지" 카테고리로 이동 (무해), (2) `api.amazingkorean.net/*` 전체가 "크롤링됨 — 현재 색인되지 않음" (robots.txt 차단) 으로 이동 → "찾을 수 없음(404)" + "Soft 404" 카테고리에서 자동 제외. 재발 없음.
  - **Property 확인**: Search Console 좌상단 드롭다운 검사 결과 `amazingkorean.net` (도메인 속성) 1개만 등록. `api.amazingkorean.net` 별도 URL 접두사 property 없음. 도메인 속성은 서브도메인 자동 포함 특성상 property 삭제는 부적절(메인 사이트 색인 보고 손실). 서버 레벨 대응만 유효.
  - **무시 가능**: "리디렉션이 포함된 페이지" 3건 ("실패함" 상태)의 **"유효성 검사 종료"** 버튼이 Search Console 에 있으면 눌러도 무해. 단 "수정 완료" 는 절대 다시 누르지 말 것 (같은 실패 루프 재발).

- **2026-04-15 — 🚨 INC-001 프로덕션 백엔드 2h33m 다운 + 복구**
  - **원인**: 2026-04-14 머지된 Gemini 리뷰 H1 IAP fail-closed 커밋(`017e8c1`) 에서 `src/config.rs:441` 에 `APP_ENV=production + REVENUECAT_API_KEY.is_none()` → `panic!` 게이트가 추가됐으나, `.github/workflows/deploy.yml` 의 `.env.prod` heredoc 에 해당 환경변수를 함께 반영하지 않았고 GitHub Secrets 에도 등록되지 않음. 배포 후 Rust 바이너리가 부팅 직후 panic (exit 101) → `docker restart: always` 로 crash loop → nginx upstream 연결 실패 → Cloudflare 502 Bad Gateway.
  - **다운타임**: 2026-04-15 00:32~03:05 UTC (약 **2시간 33분**)
  - **발견 경위**: Google Search Console 의 `api.amazingkorean.net/` "404" 기록을 SEO 작업 중 조사하다 `curl` 로 실제 상태 확인 → 502 발견. 저트래픽 새벽 시간대라 사용자 제보 없었음. SEO 작업이 의도치 않은 경보 역할.
  - **탐지 실패 이유**: GitHub Actions 는 `docker compose up -d` 의 exit code 0 만 확인. 컨테이너가 부팅 직후 panic 해도 "success" 표기. 배포 후 실제 헬스체크 단계 부재. Cloudflare 502 알림 미설정.
  - **긴급 복구 (03:05 UTC)**: EC2 SSH 접속 → `~/amazing-korean-api/.env.prod` 에 `REVENUECAT_API_KEY=placeholder_not_configured_yet` 한 줄 추가 → `docker compose -f docker-compose.prod.yml --env-file .env.prod up -d api` 로 `amk-api` 컨테이너 recreate. 서버 정상 부팅 확인 (`✅ Server listening on http://0.0.0.0:3000`, `RevenueCat client initialized`, `📦 Database migrations applied`).
  - **영구 수정 (03:17 UTC, 커밋 `d528e04` / PR #159)**: `.github/workflows/deploy.yml` 의 `.env.prod` heredoc 에 `REVENUECAT_API_KEY=${{ secrets.REVENUECAT_API_KEY }}` + `REVENUECAT_WEBHOOK_AUTH_TOKEN=${{ secrets.REVENUECAT_WEBHOOK_AUTH_TOKEN }}` 2줄 추가. GitHub repository secrets 에 동일 이름 2개 등록 (값: `placeholder_not_configured_yet`). 다음 배포부터 `.env.prod` 재생성 시 placeholder 자동 주입.
  - **현재 보안 영향**: IAP 경로는 **runtime fail-closed** (placeholder 키로 RevenueCat API 호출 실패 → 에러 리턴) 이므로 결제 우회 취약점 재발 없음. RevenueCat 실 키 준비 시 GitHub Secret 값만 교체하면 되고 `deploy.yml` 재수정 불필요.
  - **검증**: 긴급 복구 직후 + #159 재배포 후 모두 외부 `curl https://api.amazingkorean.net/health` 3회 연속 HTTP 200 (0.69~0.73s). `/auth/login` POST 빈 body → 422 `missing field 'email'`. `docker ps` → `amk-api Up`. 최신 GH Actions `#159` success 49s.
  - **교훈 메모리**: `feedback_deploy_env_sync.md` 신규 — "production `panic!` 게이트 추가 시 같은 PR 에서 deploy.yml + Secrets + AMK_DEPLOY_OPS 환경변수 표 4건 동시 반영 필수"
  - **향후 개선 (별도 작업)**:
    - (1) `.github/workflows/deploy.yml` 에 배포 후 헬스체크 단계 추가 (`curl https://api.amazingkorean.net/health` 5초 후 200 확인, 실패 시 job fail + 알림)
    - (2) Cloudflare 502 감지 이메일/Slack 알림 연동
    - (3) `AMK_DEPLOY_OPS.md §4` 환경변수 표에 `REVENUECAT_API_KEY` + `REVENUECAT_WEBHOOK_AUTH_TOKEN` 2건 추가
    - (4) 저트래픽 새벽 배포 위험 재평가 (활동 시간대 배포 정책 검토)

- **2026-04-15 — 문서·메모리 전수 조사 정리 (Plan mode 5 Phase 워크플로)**: 누적된 메모리 20개 + docs 24개 + 모듈 README/플랜 파일을 Phase 0~5 + 교차검증 게이트로 전수 조사. 커버리지 **97% 달성** (목표 95%+).
  - **[메모리 정리]** 완료된 플랜 파일 5개 삭제 (`project_writing_practice_plan.md`, `project_strictmode_audit_plan.md`, `project_gemini_review_backlog.md`, `project_design_md_plan.md`, `project_risk_analysis.md`). 축약 2개 (`project_perf_plan.md` → S5 후속 계획만, `project_figma_plan.md` → 재개 breadcrumb 만). `project_status.md` + `MEMORY.md` 갱신. 메모리 파일 20→14개 (-30%).
  - **[문서 고아 파일 삭제]** `docs/AMK_SCHEMA_PATCHED.sql` (26KB, 2026-02-14, 코드·CI 내 0건 참조 확인), `docs/AMK_PIPELINE.md` + `docs/AMK_MACMINI_SETUP.md` (껍데기, amazing-korean-ai 로 이관 완료), `docs/AMK_DESIGN_MD_ANALYSIS.md` (v4.2 통합 전 분석 문서), `frontend/DESIGN.md` (Stitch AI 전용 영문 미러, 실사용 0), `.claude/plans/EXTERNAL_API_INTEGRATION_PLAN.md` (Stripe→Paddle 전환으로 전면 폐기됨).
  - **[문서 참조 정리]** `AMK_PIPELINE.md`/`AMK_MACMINI_SETUP.md` 참조 9곳을 `amazing-korean-ai/docs/AMK_AI_*.md` 로 재매핑 (`AMK_API_MASTER.md:111/120`, `AMK_DEPLOY_OPS.md:3`, `AMK_STATUS.md:275/637/642`, `AMK_APP_ROADMAP.md:287-288`, `AMK_MARKET_ANALYSIS.md:149`, `CLAUDE.md:50-51`).
  - **[AMK_STATUS §8.2 완료 항목 제거]** strikethrough 로 남아있던 완료 10행 일괄 삭제 (e-book Paddle, 동시 세션, 모바일 OAuth, IAP, 모바일 인증, crypto 크레이트, 다국어 반응형, 코드 점검, 디자인 시스템, E-book 웹 보안). §8.1 과의 중복 제거.
  - **[AMK_API_PAYMENT.md 스펙-코드 갭 해소]** `⚠️ 미구현 (코드에 없음)` 마커 8건 (`provider`, `currency` ×2, `granted_by`, `reason`, `created_at`, `user_nickname`, `active_courses`, `earliest_enrolled`, `latest_expire`) 문서에서 제거. 실제 코드에 매핑되는 `course_count`, `expire_at` 으로 통일.
  - **[AMK_CODE_PATTERNS.md 품질]** `line 319` `"trace_id": "req-TODO"` 플레이스홀더 → Axum trace middleware 주입 패턴으로 교체. 백엔드 섹션 1.3~1.6 (repo/service/handler/router/auth 유틸) 의 "(2025-01-22)" 타임스탬프 5곳을 "최종 갱신 2026-04-08" 로 일괄 교체.
  - **[AMK_SCHEMA_PATCHED.md 주석]** `line 561` `user_course_pay_id` 주석 "추후 pay 테이블 연동" → "Paddle/payment 테이블 연동" (payment 시스템 #12 완료 반영).
  - **[크로스 프로젝트 인계]** mobile M5.5 IAP "미구현" 기록 (`~/.claude/projects/-home-kkryo-dev-amazing-korean-mobile/memory/project_decisions.md:84-90`) + desktop CORS "수정 필요" 기록 (`~/.claude/projects/-home-kkryo-dev-amazing-korean-desktop/memory/project_decisions.md:30-32`) 를 stale 로 식별. 본 세션 수정 금지 원칙에 따라 해당 프로젝트 세션에서 직접 반영 예정.
  - **[워크플로]** Plan mode 에서 5 Phase 조사 방법론을 `~/.claude/plans/calm-twirling-ember.md` 에 작성·승인 후 실행. Phase 0 인벤토리 → Phase 1 에이전트 5대 병렬 도메인 스캔 + Phase 4 에이전트 1대 크로스 프로젝트 스캔 → Phase 2 스테일 마커 전역 Grep → Phase 3 스키마↔마이그레이션 diff → 교차검증 게이트 → Phase 5 정리안. feedback_*.md 5개는 정리 대상 영구 제외.
  - **검증**: `cargo check` + `cd frontend && npm run build` 클린 통과 예정.

- **2026-04-14 — Gemini 리뷰 전수 반영 (PR #152~#155 세션 A~D 일괄)**: 2026-04-07~2026-04-14 머지된 4개 PR 의 Gemini 지적 13건 중 미처리 12건(HIGH 5 + MED 7) 을 하나의 묶음 커밋/PR 로 처리. 남은 미처리 0건.
  - **[세션 A / 보안: H1 IAP fail-closed]** `src/config.rs` 프로덕션 fail-fast 블록에 RevenueCat 게이트 추가. `APP_ENV=production` + `REVENUECAT_API_KEY` 미설정 시 서버 부팅 panic. 기존 코드(`src/api/ebook/service.rs:229`)는 `if let Some(ref rc_client) = st.revenuecat` 구조라 키 미설정 → `st.revenuecat == None` → 영수증 검증 블록 전체 skip → `insert_iap_purchase(status=completed)` 로 무료 교재 획득 가능한 결제 우회 취약점 존재. Gemini PR #152 HIGH 지적. 부팅 단계 panic 으로 차단 (AMK_API_MASTER 의 "프로덕션 안전장치" 패턴 — `EMAIL_PROVIDER=none` + production 과 동일 전략)
  - **[세션 A / M1 RFC3339]** `src/external/revenuecat.rs:114-119` — `v.expires_date` 를 `chrono::DateTime::parse_from_rfc3339` 로 파싱 후 UTC 비교. 기존에는 `d > &chrono::Utc::now().to_rfc3339()` 의 문자열 사전순 비교였고, 타임존 포맷(`Z` vs `+00:00`) 이나 초 정밀도 차이에서 오판 가능. Gemini PR #152 MEDIUM 지적
  - **[세션 C / auth N+1 + fail-closed: H2+H3+M4+M5]** `src/api/auth/service.rs::enforce_session_limit` 전면 리팩터 + `src/api/auth/repo.rs` 에 배치 repo 함수 2개 신규.
    - **H2 유령 세션 정리 N+1 제거**: 기존 루프 안에서 `find_login_by_session_id` 를 세션마다 1회씩 호출(유저당 최대 5회) 하던 패턴 제거. 신규 `find_login_refresh_hashes_by_session_ids(&[sid]) -> HashMap<sid, refresh_hash>` 배치 조회로 **DB 쿼리 1회** 로 통합. 배치 DB UPDATE 는 신규 `update_login_states_by_sessions(&[sid], state, reason)` 로 일괄 처리.
    - **H2 Redis fail-closed**: `redis_conn.exists(...).await.unwrap_or(false)` 패턴을 `?` 전파로 전환. 일시적 Redis 장애가 유효 세션을 "만료" 로 오판해 학습자를 강제 로그아웃시키는 문제를 차단. `security_patterns` 메모리의 fail-closed 원칙 반영.
    - **H3 eviction 루프 N+1 제거**: 기존에 evict 대상마다 `find_login_by_session_id` 를 한 번 더 호출해 `refresh_hash` 를 조회하던 구조 제거. `find_active_sessions_oldest` 반환 타입을 `Vec<String>` → `Vec<(String, String)>` 로 변경(**M5** 와 한 덩어리) 해 1회 조회에 session_id + refresh_hash 를 함께 가져오도록 수정. DB 상태 업데이트도 배치 UPDATE 로 통합.
    - **M4 silent random eviction 방지**: DB fallback 이 빈 결과이면 Redis SET 의 `smembers` 를 재호출해 `take(evict_count)` 하던 기존 로직 제거. Redis Set 은 무순서라 FIFO 가 파괴되는 문제. 대체로 경고 로그 + `AppError::Internal("session eviction aborted")` 반환으로 명시적 실패 처리.
    - **단일 호출부**(`login` → `enforce_session_limit`) 확인 + clippy 0 경고.
  - **[세션 D / 성능 테스트: H4 K6 tags + M7 progress 시나리오]** `k6/scenario_load.js` + `k6/scenario_smoke.js` 재작성.
    - `new Trend("http_req_duration{endpoint:auth}")` 같은 커스텀 Trend 4개 삭제. 같은 이름의 별개 metric 을 만들던 패턴 → config 의 `http_req_duration{endpoint:auth}` thresholds 는 기본 metric 에 `tag` 필터를 거는 문법이므로 커스텀 Trend 로는 절대 매칭 안 됨 (지금까지 K6 thresholds 검증이 실질 no-op 였던 원인).
    - http.get/post 호출부에 `tags: { endpoint: "auth" | "list" | "detail" | "progress" }` 직접 부여 → config.js 의 thresholds 가 기본 `http_req_duration` 에 태그 필터로 매칭되어 정상 동작.
    - progress 시나리오 신규 추가: 존재하지 않던 `/api/studies/{id}/progress` 대신 기존 `GET /api/studies/tasks/{id}/status` 를 `endpoint:progress` 로 태그해 "진도 확인" 지표 대표 엔드포인트로 사용 (study 상세에서 tasks 목록 추출 후 랜덤 선택).
  - **[세션 D / 성능: M2+M3 Apple OAuth 싱글톤 + JWKS 캐시]** `src/external/apple.rs` 리라이트 + `src/state.rs` 에 `apple_oauth: Option<Arc<AppleOAuthClient>>` 신규 + `src/main.rs` 초기화 블록 6.6 신규 + `apple_mobile_login` 핸들러에서 `st.apple_oauth` 재사용.
    - `AppleOAuthClient` 에 `jwks_cache: Arc<RwLock<HashMap<String, DecodingKey>>>` 필드 추가. `get_decoding_key(kid)` 가 read-lock 캐시 조회 후 miss 시 JWKS 1회 fetch + 전체 키 배치 적재. 모바일 Apple 로그인 요청마다 `https://appleid.apple.com/auth/keys` 를 매번 호출하던 문제 해결.
    - `reqwest::Client` 도 싱글톤 내부에 1회만 생성되어 커넥션 풀 재사용.
    - 기존 `AppleOAuthClient::new(client_id.clone())` 를 handler 에서 매 호출 시 생성하던 패턴 제거 (`st.revenuecat` / `st.google_oauth` 기존 싱글톤 패턴과 동일).
  - **[세션 D / UX: M6 Noto Color Emoji noscript fallback]** `frontend/index.html` 의 `<link rel="stylesheet" ... onload="this.media='all'">` 비동기 로드 패턴은 JS 비활성 환경에서 동작하지 않아 이모지 폰트가 로드되지 않음. `<noscript>` 블록 신규 추가로 JS off 환경에서 동기 로드 fallback. 기존 SEO 본문용 `<noscript>` 와 별개.
  - **[세션 B / 학습 UX: H5 한글 IME 조합 버그]** `frontend/src/category/study/component/writing/WritingPracticeInput.tsx` — 3개 문제 동시 수정.
    - **(1) charResults 음절 단위 비교 오작동**: '가' 입력 중 'ㄱ' 만 친 순간 `actual === expected` 실패로 즉시 `wrong` 표시 → 마지막 글자(`i === lastActualIdx`) 가 `isComposing=true` 면 `pending` 으로 유지. `isComposing` 을 useMemo 의존성에 추가.
    - **(2) 다음 자모 하이라이트 점프**: 기존 `nextCharIdx = actualChars.length` 는 'ㄱ' 조합 중 `length=1` 이 되어 다음 음절의 첫 자모로 점프해 엉뚱한 키 하이라이트. 조합 중 분기 신규 — `decomposeSyllable(actualChars[last])` 와 `decomposeSyllable(expectedChars[last])` 를 비교해 "현재 조합 중 음절에서 이미 입력된 자모 수만큼 건너뛴 다음 기대 자모" 를 반환.
    - **(3) isComposing 반영 누락**: 기존 `isComposing` state 가 mistakes 통계 skip 에만 쓰이고 charResults 계산에는 영향 없던 문제 — charResults useMemo 와 키보드 하이라이트 useEffect 의 의존성 배열에 `isComposing` 추가.
    - **검증 한계**: Playwright 는 native IME composition 이벤트를 생성하지 못해 자동화된 E2E 로 재현 불가. 수동 브라우저(macOS 2-set 한국어 IME) 검증 필요 — 마지막 검증 단계에서 수기 수행.
  - **[보너스 / 감사 중 발견한 동일 안티패턴 4건 추가 반영]** 커밋 전 검증 과정에서
    `enforce_session_limit` 와 동일한 `unwrap_or(false)` fail-open + `find_login_by_session_id`
    N+1 패턴이 다른 세션 정리 경로 4개에도 존재함을 확인 → 같은 묶음 PR 로 일괄 처리.
    - **`reset_password`** (service.rs:892-915): Redis 세션 정리 블록을 `smembers(?) + find_login_refresh_hashes_by_session_ids` 배치 조회 + 전체 Redis 호출 `?` 전파로 교체
    - **`logout`** (service.rs:942-950): 단일 세션 정리의 `unwrap_or(())` 3건을 `?` 전파로 교체 (N+1 없음, fail-closed 만)
    - **`logout_all`** (service.rs:1020-1035): `find_login_by_session_id` 루프 N+1 제거 → 배치 조회. Redis 호출 전부 `?` 전파
    - **`reset_password_with_token`** (service.rs:1253-1274): `reset_password` 와 동일 패턴 일괄 교체
    - **미수정 유지**: rate-limit DECR 롤백 / verification-code / OAuth-state / MFA-token 의 `unwrap_or(())` 는 TTL 이 안전망이라 의도적으로 fail-open 유지 (security-critical 경로 아님)
  - **[파일 변경]**
    - Rust: `src/config.rs`, `src/state.rs`, `src/main.rs`, `src/external/revenuecat.rs`, `src/external/apple.rs`, `src/api/auth/service.rs`, `src/api/auth/repo.rs`
    - Frontend: `frontend/index.html`, `frontend/src/category/study/component/writing/WritingPracticeInput.tsx`
    - K6: `k6/scenario_load.js`, `k6/scenario_smoke.js`
    - 문서: `docs/AMK_API_MASTER.md`, `docs/AMK_API_EBOOK.md`, `docs/AMK_STATUS.md`, `docs/AMK_CHANGELOG.md`
  - **[검증]** `cargo check`, `cargo clippy --lib` (0 warnings), `cd frontend && npm run build` (tsc + vite 클린 통과). Playwright E2E 미실행(기존 `writing_practice.spec.ts` 는 비조합 jamo 경로만 커버).
  - **[남은 Gemini 백로그]** 0건. 추후 PR 머지 시 Gemini 리뷰는 리뷰 달린 당시 세션에 즉시 반영 원칙으로 변경 검토 (feedback_work_rules 갱신 후보).

- **2026-04-14 — 속도 개선 Phase S4 + React 19 StrictMode 위험 패턴 전수 점검**
  - [S4-1 audit 툴] `frontend/perf-audit/audit.mjs` — `PERF_TARGET_URL` env 오버라이드 추가. 설정 시 vite preview 기동을 건너뛰고 원격 URL 을 baseURL 로 사용 (프로덕션 재측정용). `LOCAL_PREVIEW_URL` / `BASE_URL` / `SKIP_PREVIEW` 플래그 분리, preview 종료 처리에 null 체크 추가. 부수 버그 수정: `userDataDir` 를 `chrome-launcher` 에 넘기기 전에 `mkdirSync(..., {recursive:true})` 로 선생성 (기존에는 `ENOENT: no such file or directory, open '/tmp/lighthouse-profile-.../chrome-out.log'` 로 실패)
  - [S4-1 측정] https://amazingkorean.net 대상 8페이지(home/about/faq/coming-soon/book-hub/textbook-catalog/ebook-catalog/login) × 3회 Lighthouse 측정 → 중앙값 집계. S3-5 베이스라인 대비 전 페이지 Performance **+20 ~ +35**, 특히 `about` TBT **13,500→199ms (-98.5%)**, `faq` LCP **14.1s→5.0s**, `book-hub` LCP **18.6s→9.3s**, home TBT 810→209ms. S3 배포가 실제로 반영됐음이 수치로 확인됨
  - [S4-1 이상치] home run1 만 Perf=39 / TBT=1755ms 로 단독 역행처럼 보였으나 run2/run3 에서 Perf=67/64, TBT=209/0 으로 회복. 중앙값 사용 정당성 확인. 1회 측정은 신뢰 불가
  - [S4-2 BP 82 원인 특정] 실패 audit 는 `deprecations` (weight 5). 출처 스크립트는 `https://amazingkorean.net/cdn-cgi/challenge-platform/scripts/jsd/main.js` — Cloudflare 봇 방어 챌린지 JS. deprecated API 3종(`SharedStorage`, `StorageType.persistent`, `Fledge`) 사용. **우리 코드 아님**. CF 가 첫 요청에 challenge JS 를 주입하고 이후 페이지는 `_cf_bm` 쿠키 덕에 스킵 → S4-1 측정 순서상 `home` 에만 3회 모두 BP=82 로 고정되고 다른 7페이지는 전부 100/100/100. 측정 순서를 바꾸면 BP=82 가 이동하는 측정-아티팩트. 자체 코드로 수정 불가, 문서화만
  - [S4-3 about TBT] S3-5 에서 13.5s 였던 about TBT 가 S4-1 재측정 median 199ms 로 자동 해결. S3 변경(vendor 청크 분리 + Pretendard preload + Noto Color Emoji 비동기) 만으로 해결된 것으로 추정. 별도 trace 덤프 / 원인 조사 생략
  - [S4 갭 분석] 목표 Perf 90+ 까지 페이지별 21~31 점 남음. 주요 병목 (1) text-LCP 페이지 FCP 4~5s — Critical CSS inline (`vite-plugin-critical`) 검토 조건 근접 (2) image-LCP 페이지(book-hub/textbook/ebook) LCP 8~9s — 히어로 Swiper 의 추가 preload + `srcset` 반응형. S5(가칭) 별도 스코프로 분리
  - [StrictMode Track A] `useRef(false)` + `useEffect([])` 가드 4건 triage: `use_oauth_callback.ts` (이미 `refreshToken()` 직접 호출 + `.then()/.catch()` 로 권장 패턴과 동일), `use_paddle.ts` (Paddle SDK `initializePaddle()` 직접 호출), `use_language_sync.ts` (i18next `changeLanguage()` 직접 호출), `devtools_detect.ts` (DOM 이벤트/interval 가드). 4건 전부 `useMutation` observer 미사용 → 무해 확정
  - [StrictMode Track B] `useEffect` 블록 내 `.mutate(` 호출 전수 스캔. 정규식 `useEffect\(\(\) => \{[\s\S]*?\.mutate\(` 매치 12파일 수동 검토 결과 모든 `.mutate(` 가 onClick/onSubmit/form handler 또는 DOM 이벤트 콜백 내부에 있음. `useEffect` 본문에서 동기로 fire 되는 사례 **0건**
  - [결론] 코드베이스 내 동일 StrictMode 버그 패턴 없음. `WritingTask.tsx` (P10-C 에서 수정 완료) 가 유일한 사례로 확정. `feedback_react_strictmode_mutation.md` 메모리에 전수 조사 결과 반영
  - [검증] audit.mjs 수정 후 `PERF_TARGET_URL=https://amazingkorean.net node perf-audit/audit.mjs prod-s4-1-runN` 3회 정상 실행. 8페이지 × 3 = 24건 lighthouse 보고서 생성
  - [배제] Critical CSS inline / 이미지 `srcset` 는 S4 스코프 밖 (S5 후속으로 분리). S4-3 원인 조사는 자동 해결로 skip. StrictMode 수정 작업은 0건이라 미실행
  - [문서] 본 changelog 항목 + STATUS #60 + `project_perf_plan.md` / `project_strictmode_audit_plan.md` / `feedback_react_strictmode_mutation.md` 메모리 동기화

- **2026-04-14 — 한글 자판 연습 (Writing Practice) P10-C: Playwright E2E + WritingTask StrictMode 버그 수정**
  - [P10-C config] `frontend/playwright.config.ts` 신규 — testDir `./e2e`, outputDir `./test-results/e2e`, chromium 단독, 1 worker, 60s timeout, baseURL `process.env.E2E_BASE_URL ?? http://localhost:5173`. `webServer` 는 쓰지 않고 dev 서버가 이미 떠 있다는 전제. `locale: ko-KR`
  - [P10-C fixture] `frontend/e2e/fixtures/auth.ts` 신규 — `apiLogin` (`POST /auth/login` 을 request context 로 직접 호출 후 `{user_id, access_token}` 반환) + `seedAuthStorage` (zustand persist 포맷으로 `auth-storage` localStorage 를 `addInitScript` 로 주입). 기본 계정 `e2e_p10c@amazingkorean.net / password123!`
  - [P10-C spec] `frontend/e2e/writing_practice.spec.ts` 신규 — 자유 연습 플로우 E2E. 레벨 선택 → 초급 → 자모 → 세션 시작 응답 대기 → 첫 아이템 prompt innerText 추출 → textarea `pressSequentially` → "결과 확인" → 결과 카드 + stat 라벨 검증 → `/studies/writing/stats` 헤딩 확인. `GET /studies/writing/stats?days=1` 를 fixture 전/후로 직접 호출해 `total_sessions` 가 +1 이상 증가했는지 서버 기준으로 검증
  - [P10-C vite] `frontend/vite.config.ts` — `server.proxy["/api"].target` 을 `process.env.VITE_PROXY_TARGET ?? "http://127.0.0.1:3000"` 으로 변경. 다른 프로젝트가 3000 을 점유 중일 때 backend 를 3100 등 대체 포트로 띄우고 프록시만 붙여 쓸 수 있게 함
  - [P10-C npm] `frontend/package.json` — `"test:e2e": "playwright test"` 스크립트 추가
  - [P10-C doc] `frontend/e2e/README.md` 신규 — 전제 조건(backend+vite 상시 기동), EMAIL_PROVIDER=none 로 backend 1회 띄운 뒤 `/users` 로 테스트 계정 자동 인증 생성 절차, Redis 레이트 리미트 수동 해제 커맨드까지 포함
  - [P10-C gitignore] `.gitignore` — `frontend/test-results/`, `frontend/playwright-report/` 제외 추가
  - [P10-C 버그수정] `frontend/src/category/study/component/writing/WritingTask.tsx` — 기존 `useRef(false)` + `useStartWritingSession().mutate(..., { onSuccess })` 패턴이 React 19 StrictMode 에서 완전히 깨지는 것을 E2E 에서 최초로 재현. StrictMode 의 이펙트 이중 호출에서 첫 번째 invocation 이 `mutate` 를 쏜 뒤 두 번째 invocation 은 `startedRef.current===true` 로 스킵되지만, TanStack Query 의 mutation observer 는 double invocation 사이에서 파괴되어 첫 호출의 onSuccess 가 영영 드랍 → `sessionId` 가 `null` 로 남아 "결과 확인" 버튼이 영구 비활성화되는 증상. 수정: `useStartWritingSession` 훅 사용 대신 `startWritingSession` 을 직접 await 하고 `cancelled` 플래그 기반 cleanup 으로 첫 호출의 결과를 drop. 이 방식은 StrictMode + 운영 모드 모두 정상 작동
  - [검증] 로컬에서 backend `BIND_ADDR=127.0.0.1:3100 EMAIL_PROVIDER=none`, frontend `VITE_PROXY_TARGET=http://127.0.0.1:3100 npm run dev` 로 띄우고 `npm run test:e2e` → 1 passed (7.4s → 이후 재측정 6.4s). `npm run build` 클린 통과 (9.56s)
  - [진행 상황] P10-C 완료. 한글 자판 연습 Phase 1 (P1~P10) 100% 완료
  - [배제] CI GitHub Actions 워크플로우 통합은 이번 스코프 밖. 맥미니 QA 셋업(amazing-korean-ai Day 6) 에서 별도로 묶을 예정
  - [문서] 본 changelog 항목 + STATUS #59 + 메모리 동기화

- **2026-04-13 — 한글 자판 연습 (Writing Practice) P10-B: 프론트 자유 연습 실연결**
  - [P10-B types] `frontend/src/category/study/types.ts` — `writingPracticeSeedReqSchema`/`writingPracticeSeedItemSchema`/`writingPracticeSeedResSchema` Zod 스키마 3종 신규. 백엔드 `WritingPracticeSeedReq`/`Item`/`Res` 파리티. `level`/`practice_type`은 기존 enum 스키마 재사용
  - [P10-B api] `frontend/src/category/study/study_api.ts` — `getWritingPracticeSeed({level, practice_type, limit?})` 함수 신규. `GET /studies/writing/practice` 호출 (비인증 엔드포인트)
  - [P10-B hook] `frontend/src/category/study/hook/use_writing_practice_seed.ts` 신규 — `useWritingPracticeSeed` 쿼리 훅. queryKey `["writing-practice-seed", level, practice_type, limit]`, `staleTime` 10분 (시드는 거의 변하지 않음)
  - [P10-B task] `component/writing/WritingTask.tsx` — props `taskId: number | null`로 타입 완화. `mutate` 호출 시 `study_task_id: taskId ?? null`. 기존 `study_task_page.tsx` 호출부는 number 전달이라 영향 없음. 자유 연습 모드에선 null 전달 → 세션 `study_task_id=null`로 저장
  - [P10-B page] `page/writing_practice_page.tsx` 완전 리라이트 — "준비 중" placeholder 카드 제거. `FreePracticeRunner` 서브 컴포넌트 신규: `useWritingPracticeSeed` 로드 → `currentIndex` state → 현재 아이템을 `WritingPayload` 형태로 재구성(초급만 answer 전달, `keyboard_visible=beginner 여부`) → `WritingTask` 렌더. 결과 확인 버튼 → `finishWritingSession` → 결과 카드 표시 → "다음 문제" 버튼. 마지막 아이템 완료 시 "처음부터 다시" / "다른 유형 선택" CTA. WritingTask `key` prop은 `seed_id` + `attempt` 기반으로 아이템/재시도 시 재마운트 보장(세션 중복 방지)
  - [P10-B i18n] `study.writing.*`에서 `freePracticeComingTitle`/`freePracticeComingDescription`/`freePracticeMeta` 3개 키 제거. `freePracticeProgress`/`freePracticeLoadError`/`freePracticeEmpty`/`finishItem`/`finishAll`/`nextItem`/`allCompleted`/`restart`/`selectOther` 9개 키 ko/en 추가
  - [검증] `cd frontend && npm run build` 클린 통과 (tsc -b + vite build 8.63s)
  - [진행 상황] P10-B 완료 (95%). 다음: P10-C Playwright E2E (로그인 → 레벨/유형 선택 → 시드 렌더 → 타이핑 → 세션 완료 → 통계 반영 확인)
  - [배제] B안(한 세션 = 전체 묶음)은 세션 finish 1회 전제가 깨지므로 보류. 진행도 persistence(localStorage/서버) 스코프 밖
  - [문서] 본 changelog 항목 + STATUS #59 + 메모리 동기화

- **2026-04-13 — 한글 자판 연습 (Writing Practice) P10-A: 자유 연습 시드 컨텐츠 + 백엔드 엔드포인트**
  - [P10-A migration] `migrations/20260413_writing_practice_seed.sql` 신규 — `writing_practice_seed` 테이블 (pk/level/practice_type/seq/prompt/answer/hint). UNIQUE (level, practice_type, seq) + `(level, practice_type, seq)` 복합 인덱스. 자유 연습은 study 수강 흐름과 독립된 드릴 컨텐츠라 study_task_writing 재사용 대신 별도 테이블로 분리
  - [P10-A seed] 동일 마이그레이션 INSERT ~190개 — 초급 jamo 40 (기본 자음 14 + 기본 모음 10 + 쌍자음 5 + 이중모음 11) / syllable 30 (CV 음절) / word 30 (일상어). 중급 word 30 (2~4음절 복합어) / sentence 30 (기초 회화). 고급 sentence 20 (복문·경어법) / paragraph 10 (2~4문장). 초급 jamo만 hint에 로마자 포함
  - [P10-A dto] `src/api/study/dto.rs` — `WritingPracticeSeedReq`/`WritingPracticeSeedItem`/`WritingPracticeSeedRes` 신규. 요청=`level`+`practice_type`+`limit?` (기본 20, 최대 100). 응답=level/practice_type/items
  - [P10-A repo] `src/api/study/repo.rs` — `StudyRepo::list_writing_practice_seed(pool, level, practice_type, limit)` 신규. `writing_practice_seed`에서 `(level, practice_type)` 필터로 `seq ASC` 조회 + LIMIT
  - [P10-A service] `src/api/study/service.rs` — `StudyService::list_writing_practice_seed`: limit 검증(0/>100), repo 호출 후 response 구성. 인증 불필요
  - [P10-A handler+router] `src/api/study/handler.rs` `list_writing_practice_seed` 핸들러 + `GET /studies/writing/practice` 라우트 등록. `OptionalAuthUser`조차 생략해 비인증 허용
  - [P10-A openapi] `src/docs.rs` — handler + 3개 DTO (Req/Item/Res) 스키마 등록
  - [P10-A 문서] `docs/AMK_API_LEARNING.md` §5.5 표에 5-11 행 추가 + 5.5-11 상세 시나리오 추가. `docs/AMK_SCHEMA_PATCHED.md` §2.4.5-3 `writing_practice_seed` 테이블 문서화
  - [검증] `cargo sqlx prepare` 쿼리 캐시 업데이트 완료, `SQLX_OFFLINE=true cargo check` 클린 통과 (0 warning/error). 로컬 DB 마이그레이션 적용 후 `SELECT COUNT(*) GROUP BY level/practice_type`로 행 수 검증 (beginner: 40+30+30, intermediate: 30+30, advanced: 20+10 = 190)
  - [진행 상황] P10-A 완료. 다음: P10-B 프론트 자유 연습 실연결 (writing_practice_page에서 시드 fetch → WritingTask 재사용 → finishWritingSession 플로우) → P10-C Playwright E2E

- **2026-04-13 — 한글 자판 연습 (Writing Practice) P9: 관리자 프론트 writing 폼 + CSV**
  - [P9 types] `frontend/src/category/admin/study/types.ts` — `studyTaskCreateReqSchema` / `studyTaskUpdateReqSchema` / `studyTaskUpdateItemSchema` / `adminStudyTaskDetailResSchema` 4곳에 writing 전용 필드 4개 (`writing_level`, `writing_practice_type`, `writing_hint`, `writing_keyboard_visible`) 추가. `../../study/types`에서 `writingLevelSchema`/`writingPracticeTypeSchema` import 재사용. 백엔드 AdminStudyTaskDetailRes 파리티 확보
  - [P9 create] `admin_study_create.tsx` — 단일 Task / Bulk Tasks Select 드롭다운에 `writing` 옵션 추가. `study_task_kind === "writing"` 조건부 필드 박스 (level Select / practice_type Select / hint Input / keyboard_visible Checkbox). `taskFormSchema` / `bulkTaskFormSchema`에 4개 필드 optional 추가. `getTaskKindBadgeVariant` writing→outline 분기. `onCSVTasksSubmit` 매핑에 writing 4개 필드 전달
  - [P9 csv] 같은 파일의 `parseCSV` — `validKinds`에 `"writing"` 추가. 4개 writing 컬럼 헤더 인덱스 파싱(`writing_level`, `writing_practice_type`, `writing_hint`, `writing_keyboard_visible`). level/type enum 화이트리스트 검증, `writing_keyboard_visible`은 "true"/"false" 문자열을 boolean으로 변환(공란=undefined). writing row 검증: prompt(question) + answer + level + practice_type 필수. CSV 포맷 가이드 pre 블록에 writing 예시 행 + 설명 문구 추가
  - [P9 detail] `admin_study_detail.tsx` — `TaskDetailsContent` props 타입에 writing 4개 필드 추가. `taskKind === "writing"` 조건부 카드 블록 (Level/Practice Type/Hint/Keyboard Visible 읽기 전용 표시). `getTaskKindBadgeVariant`에도 writing 분기 추가
  - [참고] bulk edit 다이얼로그는 question/answer만 지원하므로 writing 세부 편집은 단일 태스크 플로우(신규 생성)로 한정. 기존 writing 태스크 필드 업데이트는 P10 이후 필요 시 추가
  - [검증] `cd frontend && npm run build` 클린 통과 (tsc -b + vite build 8.43s)
  - [진행 상황] P1~P9 완료 (90%). 다음: P10 시드 데이터 (자모→단어→문장) + writing_practice_page 자유 연습 실연결 + E2E 검증
  - [문서] 본 changelog 항목 + STATUS #59 + 메모리 동기화

- **2026-04-13 — 한글 자판 연습 (Writing Practice) P8: 통계 대시보드**
  - [P8 page] `frontend/src/category/study/page/writing_stats_page.tsx` 신규 — `useWritingStats({ days })` 훅 기반. 상단 기간 Select (7/14/30/60/90/180/365일, 기본 30). `formatNumber`/`formatPercent`/`formatCpm` 로컬 헬퍼
  - [P8 summary] Summary 카드 3개 — 총 세션(Keyboard), 평균 정확도(Target, %), 평균 CPM(Gauge, "분당 글자 수" subtitle). admin_study_stats_page 패턴 재사용 (h-12 w-12 원형 아이콘)
  - [P8 levels] `LevelBreakdownCard` — `level_breakdown` 기반 progress bar (sessions / maxSessions * 100 폭). 초→중→고 순 정렬(모듈 레벨 `LEVEL_ORDER` 상수). 하단 정확도/CPM 라인
  - [P8 weak] `WeakCharsCard` — `weak_chars` 기반 Top 10. destructive 톤 카드 (h-10 w-10 글자 박스 + 상대 bar, `Math.max(widthPercent, 10)`으로 최소 가시성 확보). 비어있을 때 "잘하고 있어요!" 문구
  - [P8 trend] `DailyTrendCard` — `recent_trend` 일별 테이블 (날짜 내림차순, 세션/정확도/CPM 컬럼). overflow-x-auto로 모바일 대응
  - [P8 empty] `total_sessions === 0 && !isPending` 시 dashed border CTA 카드 (TrendingUp 아이콘 + 연습 시작 버튼)
  - [P8 route] `app/routes.tsx` — `WritingStatsPage` lazy import + Private 하위 `/studies/writing/stats` 라우트 등록. React Router v6 랭킹 규칙상 static segment "stats"가 `:level` 파라미터보다 우선 매칭되므로 JSX 순서와 무관하게 안전 (가독성 위해 구체 경로를 먼저 선언)
  - [P8 link] `writing_level_select_page.tsx` — P7에서 P8 구현 전까지 임시 제거했던 stats 버튼(`BarChart3` 아이콘) 복원. 제목 우측 정렬
  - [P8 i18n] `study.writing.stats.*` 네임스페이스 신규 30+ 키 — pageTitle/pageDescription/lastDays, totalSessions/avgAccuracy/avgCpm/cpmUnit, levelTitle/sessionCount/accuracyLabel/cpmLabel/noLevelData, trendTitle/trendDay/trendSessions/trendAccuracy/trendCpm/noTrendData, weakTitle/weakSubtitle/missCount/noWeakData, emptyTitle/emptyDescription/emptyAction (ko/en 병렬)
  - [검증] `npm run build` 클린 통과 (tsc -b + vite build 9.78s)
  - [진행 상황] P1~P8 완료 (80%). 다음: P9 관리자 프론트엔드 writing 폼 필드 + CSV import / P10 시드 데이터 + E2E
  - [문서] 본 changelog 항목 + STATUS #59 + 메모리 동기화

- **2026-04-12 — 한글 자판 연습 (Writing Practice) P7: 연습 페이지 + study_task_page writing 분기**
  - [P7 input] `frontend/src/category/study/component/writing/WritingPracticeInput.tsx` 신규 — `compositionstart`/`compositionend` 이벤트로 한글 IME 조합 중에는 검증 보류. 초급(+answer)=글자별 실시간 피드백 (정답/오답/미입력 3상태 색상). Array.from으로 서로게이트 페어 안전 분할. 오탈자는 `mistakesRef`(Map) 누적 — 자가교정해도 최초 오답 기록은 유지. 타이핑 stats 콜백(`total_chars`/`correct_chars`/`mistakes`/`duration_ms`) + 다음 기대 자모 콜백(`decomposeSyllable` 활용)
  - [P7 result] `WritingResultPanel.tsx` 신규 — 세션 결과 카드 (accuracy/CPM/correct_chars stat grid + 오답 글자 Top 20 하이라이트 badge, 95%+ 시 default variant badge)
  - [P7 task] `WritingTask.tsx` 신규 — WritingPracticeInput + HangulKeyboard 합성 래퍼. 태스크 마운트 시 `useStartWritingSession.mutate()` 1회 호출(중복 방지 flag) → `onSessionStart(session_id)` 콜백으로 상위 state 끌어올림. 가상 키보드 클릭은 학습 참고용으로 현재 텍스트 뒤에 자모 append. 서버 stats 응답(`finishedSession`)이 오면 WritingResultPanel 렌더
  - [P7 integration] `study_task_page.tsx` writing 분기 채움 — writingText/writingStats/writingSessionId/writingResult state + `useFinishWritingSession` 훅 추가. `handleSubmit` writing 분기: `finishWritingSession(session_id, stats)` + `submitAnswer({kind:"writing", text, session_id})` 순차 호출. `canSubmit`에 writing + finishWritingMutation.isPending 가드. renderTask에 WritingTask 추가. 태스크 전환/재시도 시 writing state 완전 초기화
  - [P7 pages] `writing_level_select_page.tsx` 신규 (/studies/writing) — 3단 레벨 카드 (초/중/고급) + 통계 버튼 링크. `writing_practice_page.tsx` 신규 (/studies/writing/:level[/:practiceType]) — 유형 미지정 시 레벨별 유형 선택(beginner=jamo/syllable/word, intermediate=word/sentence, advanced=sentence/paragraph), 유형 지정 시 자유 연습 "준비 중" 카드 (P10 시드 이후 실연습 연결 예정). safeParse로 URL 파라미터 검증 후 실패 시 상위로 Navigate redirect
  - [P7 routes] `app/routes.tsx` — Writing 페이지 2개 lazy 등록 + Private 하위에 3개 라우트 추가: `/studies/writing`, `/studies/writing/:level`, `/studies/writing/:level/:practiceType`. React Router v6 랭킹 규칙상 static segment("writing")가 `/studies/:studyId`보다 우선 매칭
  - [P7 i18n] `ko.json`/`en.json` `study.writing` 확장 — 30+ 키 추가: promptLabel/inputPlaceholder/hintLabel/progress/liveAccuracy, resultTitle/statAccuracy/statCpm/statChars/mistakesLabel, landingBadge/Title/Description/viewStats/startLevel, selectPracticeType/backToLevels/backToTypes/freePracticeComing*, `level.{beginner|intermediate|advanced}.{title|description}`, `practiceType.{jamo|syllable|word|sentence|paragraph}`
  - [검증] `npm run build` 클린 통과 (tsc -b + vite build 8.68s)
  - [진행 상황] P1~P7 완료 (70%). 다음: P8 통계 대시보드 / P9 관리자 프론트 writing 폼 / P10 시드 데이터 + E2E
  - [문서] 본 changelog 항목 + STATUS #59 + 메모리 동기화

- **2026-04-12 — 한글 자판 연습 (Writing Practice) P6: HangulKeyboard 컴포넌트**
  - [P6 layout] `frontend/src/category/study/component/writing/keyboard_layout.ts` 신규 — 2벌식 (KS X 5002) 3행 `DUBEOLSIK_ROWS` 데이터 (KeyQ~KeyP / KeyA~KeyL / KeyZ~KeyM, 기본 자모 + Shift 쌍자모). `KeyCap` 타입. `findKeyForJamo` 헬퍼 (자모 → 키캡 + needsShift). `decomposeSyllable` 헬퍼 (한글 음절 U+AC00~U+D7A3 → 초/중/종성 자모 배열, (cho*21+jung)*28+jong 공식)
  - [P6 key] `HangulKeyboardKey.tsx` 신규 — 개별 키 버튼 (h-12 w-10 기본, sm:h-14 w-12). 기본 자모 중앙 + 오른쪽 위 Shift 변형 + 하단 영문 라벨. `isHighlighted`(primary ring) / `isShiftHighlighted`(amber ring, Shift 입력 필요 시) 상태. 클릭 시 `onPress(base | shift)`. 접근성 aria-label 포함
  - [P6 keyboard] `HangulKeyboard.tsx` 신규 — Props: `highlightKeys`, `onKeyPress`, `visible`, `onToggle`, `level`, `disabled`, `className`. 레벨별 동작: 초급=항상 표시 (토글 무시), 중급/고급=`visible` prop 존중 + 토글 버튼 (Eye/EyeOff/Keyboard 아이콘). 숨김 상태에선 "키보드 보기" 버튼만 노출. Card 래퍼 + 3행 레이아웃 (rowIdx*1rem 왼쪽 패딩으로 스태거). `useMemo`로 highlightKeys → Set<code> 변환 (base/shift 분리). `findKeyForJamo`로 매칭
  - [P6 i18n] `ko.json`/`en.json`에 `study.writing.showKeyboard` / `study.writing.hideKeyboard` 키 추가 (중첩 객체)
  - [검증] `npm run build` 클린 통과 (tsc -b + vite build)
  - [진행 상황] P1~P6 완료 (60%). 다음: P7 연습 페이지 (레벨 선택 → 연습 실행 → 결과 + study_task_page writing 분기)
  - [문서] 본 changelog 항목 + STATUS + 메모리 동기화

- **2026-04-12 — 한글 자판 연습 (Writing Practice) P5: 프론트 타입 + API + 훅**
  - [P5 types] `frontend/src/category/study/types.ts` — `writingLevelSchema`/`writingPracticeTypeSchema` Zod enum 신규. `studyTaskKindSchema`에 `writing` 추가. `submitAnswerReqSchema` discriminated union에 `{ kind: "writing", text, session_id? }` variant 추가. `writingPayloadSchema` (prompt/answer?/hint?/level/practice_type/keyboard_visible/image_url?/audio_url?) + `taskPayloadSchema` union에 포함. 세션 API용 request/response DTO 11종 (`StartWritingSessionReq`, `WritingMistake`, `FinishWritingSessionReq`, `WritingSessionListReq`, `WritingStatsReq`, `WritingSessionRes`, `WritingSessionListRes`, `WritingLevelStat`, `WritingDailyStat`, `WritingWeakChar`, `WritingStatsRes`) Zod 스키마 + z.infer 타입
  - [P5 API] `study_api.ts` — `startWritingSession` / `finishWritingSession` / `listWritingSessions` / `getWritingStats` 함수 추가. `sanitizeParams` 헬퍼를 제네릭 `<T extends Record<string, unknown>>`으로 일반화 (기존 StudyListReq 캐스트 hack 제거)
  - [P5 hooks] `hook/use_writing_session.ts` 신규 — `useStartWritingSession` + `useFinishWritingSession` (성공 시 `writing-sessions`/`writing-stats` 쿼리 invalidate) + `useWritingSessionList`. `hook/use_writing_stats.ts` 신규 — `useWritingStats`. 모두 에러 토스트 + ApiError 처리 패턴 준수
  - [P5 i18n/호환] `studyTaskKindSchema`에 `writing` 추가로 기존 `Record<StudyTaskKind, ...>` 매핑 3곳 (KIND_ICONS/KIND_LABELS×2) writing 엔트리 추가. `ko.json`/`en.json`에 `study.kindWriting` 키 추가 ("자판 연습" / "Hangul Typing"). lucide `PenLine` 아이콘 매핑. 기존 study_task_page의 kind switch 3개는 default/no-op 분기 유지 — 실제 writing UI는 P6~P7에서 구현
  - [검증] `npm run build` 클린 통과 (tsc -b + vite build)
  - [진행 상황] P1~P5 완료 (50%). 다음: P6 HangulKeyboard 컴포넌트 + P7 연습 페이지 (새 세션에서 진행)
  - [문서] 본 changelog 항목 + 메모리 동기화

- **2026-04-12 — 한글 자판 연습 (Writing Practice) P4: 세션 API (시작/완료/목록/통계)**
  - [P4 DTO] `src/api/study/dto.rs` — `StartWritingSessionReq`, `FinishWritingSessionReq`, `WritingMistake`, `WritingSessionListReq`, `WritingSessionRes`, `WritingSessionListRes`, `WritingStatsReq`, `WritingLevelStat`, `WritingDailyStat`, `WritingWeakChar`, `WritingStatsRes` 신규. NUMERIC(5,2)/(7,2) 컬럼은 응답 DTO에서 f64로 노출 (::float8 캐스트)
  - [P4 Repo] `StudyRepo` — `exists_writing_task` (writing 태스크 존재 + study open 검증), `create_writing_session` (INSERT RETURNING), `finish_writing_session` (UPDATE WHERE user_id 소유권 검증, 미존재 시 None), `list_writing_sessions` (QueryBuilder + level/finished_only 필터), `writing_stats_overall` / `writing_stats_by_level` / `writing_stats_daily` / `writing_stats_weak_chars` (jsonb_array_elements LATERAL JOIN으로 mistakes 펼쳐서 expected Top 10 집계)
  - [P4 Service] `StudyService` — 5개 메서드 추가. `finish_writing_session`에서 서버 사이드 accuracy/CPM 계산: `accuracy_rate = correct_chars/total_chars * 100`, `chars_per_minute = total_chars * 60000 / duration_ms`. 소수점 2자리 반올림 + NUMERIC(5,2)/(7,2) 범위 clamp. 검증: `correct_chars <= total_chars` (422), `duration_ms >= 0` (400), `days 1~365` (400/422)
  - [P4 Handler/Router] `/studies/writing/sessions` (POST 시작 / GET 목록), `/studies/writing/sessions/{id}` (PATCH 완료), `/studies/writing/stats` (GET). 모두 `AuthUser` 필수. utoipa path 4개 신규 등록
  - [P4 docs.rs] 신규 path 4개 + schema 11개(`StartWritingSessionReq`, `FinishWritingSessionReq`, `WritingMistake`, `WritingSessionListReq`, `WritingSessionRes`, `WritingSessionListRes`, `WritingStatsReq`, `WritingLevelStat`, `WritingDailyStat`, `WritingWeakChar`, `WritingStatsRes`) utoipa 등록
  - [검증] `cargo sqlx prepare` 캐시 업데이트, `cargo check` + `SQLX_OFFLINE=true cargo check` 클린 통과 (0건 warning/error)
  - [진행 상황] P1~P4 완료 (40%). 다음: P5~P10 프론트엔드 (타입/훅/HangulKeyboard/연습 페이지/통계 대시보드/관리자 UI/시드)
  - [문서] `AMK_API_LEARNING.md` §5.5 행렬에 5-7~5-10 추가 + 시나리오 상세 (5.5-7~5.5-10) 작성

- **2026-04-12 — 한글 자판 연습 (Writing Practice) Phase 1: DB + 백엔드 + 관리자 CRUD**
  - [P1 마이그레이션] `migrations/20260412_writing_practice.sql` 신규 — `study_task_kind_enum`에 `writing` 추가, `content_type_enum`에 `study_task_writing` 추가, 신규 enum 2종 (`writing_level_enum`: beginner/intermediate/advanced, `writing_practice_type_enum`: jamo/syllable/word/sentence/paragraph)
  - [P1 테이블] `study_task_writing` (서브테이블, PK=study_task_id FK→study_task): level, practice_type, prompt, answer, hint, keyboard_visible, image_url, audio_url. `writing_practice_session` (독립 통계 테이블): user_id, study_task_id(nullable), level, practice_type, started_at/finished_at, total_chars, correct_chars, accuracy_rate, chars_per_minute, mistakes(JSONB)
  - [P1 인덱스] `idx_stw_level`, `idx_stw_practice_type`, `idx_stw_level_type`, `idx_wps_user_id`, `idx_wps_user_level`, `idx_wps_user_started`, `idx_wps_task`
  - [P1 Rust 타입] `src/types.rs` — `StudyTaskKind::Writing`, `WritingLevel`, `WritingPracticeType`, `ContentType::StudyTaskWriting` 추가
  - [P2 study API] `WritingPayload` DTO 추가 (prompt/answer/hint/level/practice_type/keyboard_visible/image_url/audio_url). `TaskPayload::Writing` variant. `SubmitAnswerReq::Writing { text, session_id }` variant. 초급 레벨에만 `answer`를 응답에 포함 (클라이언트 실시간 피드백용). `find_task_detail`/`find_answer_key` 쿼리에 `study_task_writing` LEFT JOIN 추가. `submit_answer` 서비스에 Writing 분기 (현재는 단순 `==` 비교, 세부 통계는 P4 세션 API에서 처리)
  - [P3 관리자 CRUD] 기존 `question`/`answer` 필드 재사용 (typing/voice와 일관성): writing의 경우 `question`이 prompt로 매핑. `StudyTaskCreateReq`/`StudyTaskUpdateReq`/`StudyTaskUpdateItem`/`AdminStudyTaskDetailRes`에 writing 전용 필드 (level, practice_type, hint, keyboard_visible) 추가. 단일/벌크 생성 검증 분기 + `create_task_writing` repo 함수 + update match Writing 분기 + find_study_task_by_id 쿼리에 writing JOIN 추가. answer는 `w.study_task_writing_answer`로 COALESCE, question은 `w.study_task_writing_prompt`로 COALESCE
  - [검증] 로컬 DB 마이그레이션 적용 완료, `cargo sqlx prepare` 캐시 업데이트, `cargo check` 클린 통과 (0건 warning/error)
  - [진행 상황] 전체 설계 플랜 P1~P10 중 P1~P3 완료 (30%). 다음: P4 세션 API (POST/PATCH `/studies/writing/sessions`, GET `/studies/writing/stats`), P5+ 프론트엔드
  - [문서] `AMK_SCHEMA_PATCHED.md` §2.4 테이블/enum 동기화, `AMK_API_LEARNING.md` §5.5-4 answer flow에 writing 분기 추가

- **2026-04-12 — 속도 개선 Phase S3: LCP 수정 + Font preload + 이미지 최적화 + K6 + 프로덕션 측정**
  - [S3-1] book-hub LCP 18.6s 원인 수정: 커버 이미지 `loading="lazy"` → 초기 슬라이드만 `eager` + `fetchPriority="high"`
  - [S3-2] Pretendard Variable CSS `preload` 힌트 추가 (조기 fetch 유도). Noto Color Emoji 비동기 로딩 전환 (`media="print" onload` 패턴)
  - [S3-3] 로고 PNG 최적화: 6000×4000 1.7MB → 1200×800 52KB (-97%). favicon 분리 (32px + 192px + apple-touch-icon). 인증서 PNG→WebP (256KB→40KB, 100KB→28KB)
  - [S3-4] `k6/` 부하 테스트 디렉터리 신규: config.js (AMK_STATUS §8.2 목표치), scenario_smoke.js (VU=1), scenario_load.js (10→50→100 VU ramp-up). K6 v0.56.0 설치
  - [S3-5] 프로덕션 Lighthouse 베이스라인 확보 (배포 전 기준): home 52, about 34, faq 26 (LCP 14.1s), book-hub 33 (LCP 18.6s), login 35. A11y 91~100 ✅
  - [다음] 배포 후 프로덕션 재측정으로 S3-1~S3-3 효과 확인. Critical CSS inline, 서비스 워커 재평가

- **2026-04-10 — 속도 개선 Phase S1+S2: 측정 인프라 + Quick Win 번들 분할**
  - [인프라] `frontend/perf-audit/` 신규 — Lighthouse 기반 자동 성능 측정 도구 (`audit.mjs`, `pages.mjs`)
  - [의존성] `lighthouse` + `chrome-launcher` devDependency 추가. Playwright Chromium 동적 탐색
  - [Quick Win] `vite.config.ts` manualChunks — vendor 11종 분리 (react/radix/forms/swiper/dnd/paddle/video/i18n/tanstack/tus/기타)
  - [Quick Win] `routes.tsx` React.lazy — Admin 30+ 페이지, Auth 보조 5, Legal 3, Textbook/Ebook 후속 5, Error 3, ComingSoon/FAQ lazy 전환 + Suspense 래핑
  - [결과] **메인 번들 1,620KB → 271KB (-83.3%)**, home Performance 48→72 (+24), TBT 2572→315ms (-88%)
  - [베이스라인] 8페이지 Lighthouse 측정 완료 (pre/post). Performance 평균 61→66. 목표 90+ 도달까지 Phase S3 필요
  - [보안] npm audit 3건 동시 해결 (axios 1.15.0 + vite 7.3.2 + basic-ftp override)
  - [품질] 13건 코드 품질 이슈 일괄 수정 (Chromium 경로 동적화, cleanup hang 해결, LABEL 검증, regex 통일, 주석 보강 등)
  - [.gitignore] `perf-audit/artifacts/`, `test-results/` 제외 추가
  - [다음] Phase S3 — faq/book-hub LCP 12~17s 원인 조사, Font preload, 이미지 최적화

- **2026-04-10 — Figma 도입 보류 결정 + Phase A 도구 영구 자산 전환**
  - [결정] Figma Phase B (정리·임포트) / Phase C (네이티브 생성) 보류 — 1인 풀스택 환경에서 트리플 동기화 부담이 ④ 유지보수 효율 목표에 역행
  - [트리거] Phase B 착수 시 Figma MCP Starter 한도 초과 → 도입 자체를 재검토
  - [전환] Phase A Playwright 캡처 도구를 디자인 작업 상시 시각 레퍼런스 + 시각 회귀 감지 도구로 위치 재정의
  - [3계층 SSoT] AMK_DESIGN_SYSTEM.md (의도) + 32 PNG (시각 레퍼런스) + 코드 (구현) 운영 모델 확정
  - [재평가 트리거] (B) 디자이너 영입/외부 협업 시작 (C) 사용자 수 확대 → 다수 stakeholder 디자인 결정 / 모바일·데스크탑 멀티 플랫폼 일관성 관리 필요성 발생
  - [기존 Figma 파일] `AUYoLTYOsDWipKoNGfD3Fv` 그대로 유지 (삭제 X) — 향후 재개 시 시작점
  - [문서] `AMK_DESIGN_SYSTEM.md §08` 전면 재작성 (결정 배경 5가지 + 3계층 SSoT + Phase A 활용 시나리오 + Phase B/C 보류 사유/트리거)
  - [문서] `AMK_STATUS.md §8.1` #56 향후 확장 갱신
  - [다음 작업] 속도 개선 — K6 + Lighthouse/Web Vitals 측정 선행 후 백엔드/프론트 최적화

- **2026-04-09 — Figma 재구축 Phase A 완료 (Playwright 레퍼런스 캡처)**
  - [인프라] `frontend/figma-capture/` 신규 디렉터리 — Playwright 기반 레퍼런스 캡처 도구 일체
  - [의존성] `@playwright/test` devDependency 추가 + Chromium Headless Shell 설치
  - [설정] `playwright.config.ts`: 1440×900 viewport · deviceScaleFactor 2 (Retina) · Vite webServer 자동 기동 · ko-KR/Asia/Seoul 로캘
  - [스크립트] `tests/capture.spec.ts`: 페이지×테마 조합 자동화. next-themes localStorage 주입 + emulateMedia 동시 적용, 점진 스크롤/img decoded 대기/`document.fonts.ready`로 기존 3대 문제(한글/lazy/토큰) 근본 해결
  - [스크립트] `pages.ts`: 16개 페이지 정의 (P1 공개 4 + P2 Book 4 + P3 Auth 5 + P4 Legal 3) — 5순위(MyPage/Settings)는 이번 Phase 제외
  - [스크립트] `fixtures.ts`: 로컬 AMK 백엔드 부재 시에도 카탈로그 렌더링 보장 — textbook/ebook catalog API 모의 응답 (9/6 언어)
  - [산출물] `figma-capture/artifacts/screenshots/` — 16 페이지 × Light/Dark = **32 PNG 프레임** 생성 완료 (총 8.8MB)
  - [.gitignore] `figma-capture/artifacts/` 제외 — 스크립트/설정은 커밋, 이미지/테스트 결과는 제외
  - [샘플] Book Landing ISBN = `9791199772700` (`book_data.ts` 첫 엔트리, 영어/학생용)
  - [다음] Phase B — Figma 기존 34프레임 삭제 + 스크린샷 이미지 레이어 임포트

- **2026-04-09 — Figma 재구축 계획 수립 (하이브리드 전략)**
  - [조사] 기존 Figma 파일(AUYoLTYOsDWipKoNGfD3Fv) 점검 — 34프레임(17페이지×Light/Dark) 확인. 메모리 기록(54프레임)과 불일치
  - [조사] 기존 프레임 3대 문제 진단 — 한글 텍스트 누락(폰트 로딩 대기 실패), 이미지 누락(lazy loading + 미스크롤), 토큰 미연결(캡처 도구 특성)
  - [결정] 방안 A+C 하이브리드 확정 — Playwright 캡처(레퍼런스) + Figma MCP 생성(편집 가능)
  - [결정] 기존 34프레임 삭제 예정. 재사용 불가 판정
  - [문서] AMK_DESIGN_SYSTEM.md §08 Figma 섹션 현재화 — 재구축 전략, 파일 정보, Phase A/B/C 구조, 페이지 우선순위
  - [메모리] reference_figma.md 갱신 — 실제 프레임 수, 문제 진단, MCP OAuth 인증 완료
  - [메모리] project_figma_plan.md 신규 — 하이브리드 전략 상세, 페이지 우선순위, 결정 필요 사항

- **2026-04-09 — 디자인 시스템 문서 v4.2 + 코드 품질 수정 5건**
  - [문서] AMK_DESIGN_SYSTEM.md v4.2: 전수 조사 기반 19건 갭 보강
  - [문서] §00 Visual Theme & Atmosphere 신규 섹션 — 디자인 철학, 핵심 특성, 밀도 정의
  - [문서] §01 Color Tokens 전면 재구성 — 전체 토큰 HSL + Hex 병기 (Core/Brand/Status/Surface/Chrome/Badge/Chart)
  - [문서] §01 Shadow Scale에 실제 CSS box-shadow 값 추가 (라이트/다크)
  - [문서] §01 Radius 근사치 → 정확한 px 값 (--radius: 0.625rem 기반 계산식)
  - [문서] §01 Typography에 letter-spacing(-0.025em) + font-feature-settings 명시
  - [문서] §01 Icon/Gap/Container에 px 환산 값 추가
  - [문서] §03 CTA 패턴 3가지 변형 문서화 (Full/Nav/Inline) + gradient text-white 예외 명시
  - [문서] §04 Responsive Behavior 섹션 추가 — 브레이크포인트 테이블, 축소 전략
  - [문서] §05 Do/Don't 통합 섹션 + 허용 예외 테이블 — 산재 규칙 §01~§04에서 통합
  - [문서] §07-B Agent Prompt Guide 신규 — Quick Color Ref + 예제 프롬프트 5개 + Iteration Guide
  - [수정] index.css: --warning-foreground `0 0% 100%` → `20 14% 4%` (WCAG AA 준수)
  - [수정] ebook_viewer_page.tsx: fullscreen text-white(7건) + neutral/border 색상(8건) → 디자인 토큰 교체
  - [수정] pagination_bar.tsx: rounded-xl → rounded-md (Button 스펙 준수)
  - [수정] about_page.tsx: CTA hover:shadow-xl 누락 추가

- **2026-04-08 — 동시 세션 수 제한 구현**
  - [보안] `enforce_session_limit()`: 유령 세션 정리 (SMEMBERS + EXISTS) + SCARD 카운트 + 역할별 정책 분기
  - [보안] Learner: FIFO 자동 퇴장 (가장 오래된 세션 강제 로그아웃, `find_active_sessions_oldest` DB 쿼리)
  - [보안] Admin/Manager/HYMN: 로그인 거부 (`403 AUTH_403_SESSION_LIMIT:{max}`)
  - [보안] 적용 지점: `login()` + `create_oauth_session()` 2곳 (모든 로그인 경로 커버)
  - [설정] config.rs: MAX_SESSIONS_LEARNER(5), MAX_SESSIONS_MANAGER(3), MAX_SESSIONS_ADMIN(2), MAX_SESSIONS_HYMN(2)
  - [설정] docker-compose.prod.yml: 4개 환경변수 추가

- **2026-04-07 — 모바일 백엔드 API 5건 구현 완료 (OAuth + IAP + 웹훅)**
  - [기능] `POST /auth/google-mobile`: 모바일 Google OAuth (ID token 직접 검증 → 계정 연결/생성 → LoginMobileRes)
  - [기능] `POST /auth/apple-mobile`: 모바일 Apple OAuth (Apple JWKS RS256 검증, 최초 email 처리, 재인증 안내)
  - [기능] `POST /auth/mfa/login-mobile`: 모바일 MFA 2단계 (refresh_token JSON body 반환)
  - [기능] `POST /ebook/purchase/iap`: IAP 구매 확정 (RevenueCat 영수증 검증 → status=completed, iap_* 컬럼)
  - [기능] `POST /payment/webhook/revenuecat`: RevenueCat 웹훅 (Bearer 토큰 인증, 멱등성, 이벤트 분기)
  - [리팩토링] `OAuthUserInfo` 일반화 (GoogleUserInfo → OAuthUserInfo, Google/Apple 공통)
  - [리팩토링] `create_oauth_session` 반환에 refresh_token 추가 (모바일 OAuth용)
  - [신규] `src/external/apple.rs`: Apple JWKS 클라이언트 (google.rs 패턴 복제)
  - [신규] `src/external/revenuecat.rs`: RevenueCat REST API 클라이언트 (trait 패턴)
  - [DB] `migrations/20260407_mobile_iap.sql`: payment_provider_enum + ebook_payment_method_enum 확장, ebook_purchase iap_* 컬럼
  - [설정] config.rs: GOOGLE_MOBILE_CLIENT_ID, APPLE_CLIENT_ID, APPLE_TEAM_ID, REVENUECAT_API_KEY, REVENUECAT_WEBHOOK_AUTH_TOKEN 추가
  - [설정] docker-compose.prod.yml: 5개 신규 환경변수 추가

- **2026-04-07 — 모바일 앱 백엔드 API 스펙 문서화 (OAuth + IAP)**
  - [문서] AMK_API_AUTH.md: 모바일 OAuth 엔드포인트 3건 스펙 추가 (§5.3-16 google-mobile, §5.3-17 apple-mobile, §5.3-18 mfa/login-mobile)
  - [문서] AMK_API_EBOOK.md: IAP 구매 확정 엔드포인트 스펙 추가 (§12.5-2.5 POST /ebook/purchase/iap, RevenueCat 영수증 검증)
  - [문서] AMK_API_PAYMENT.md: RevenueCat 웹훅 엔드포인트 스펙 추가 (§11-4 POST /payment/webhook/revenuecat)
  - [문서] AMK_STATUS.md: 핵심 실행 순서에 4a(모바일 OAuth) + 4b(IAP 결제) 추가, Apple OAuth 보류→핵심으로 이동, IAP 리스크 스펙 완료 표시
  - 참조: `amazing-korean-mobile/docs/AMK_MOBILE_API_REQUIREMENTS.md`

- **2026-04-07 — 코드 품질/보안 점검 11건 수정 + 추가 보안 강화 2건**
  - [보안] ebook `verify_session` fail-open → fail-closed: JSON 파싱 실패 시 접근 거부 (OWASP fail-secure)
  - [보안] ebook `heartbeat` fail-open → fail-closed: 동일 패턴 수정
  - [보안] `CryptoError→AppError` 매핑: `InvalidFormat→BadRequest(400)` → `Internal(500)` 통일 (format oracle 방지, CWE-209/203)
  - [보안] DevTools 감지 `||` → `&&` (consoleDetected && checkWindowSize): 오탐 방지 우선, 5중 보안 최하위 레이어
  - [리팩토링] `CryptoError` 단일 variant → 3개 분리 (InvalidFormat/DecryptionFailed/Internal), `parse_enc_parts()` 공통 함수
  - [리팩토링] `NaiveDate::MIN` → `unwrap_or_default()` (5곳), `provider_data.clone()` → 참조 (2곳), `Some(target_table)` → 직접 바인딩 (2곳)
  - [수정] DataTable 페이지네이션: 하드코딩 1-5 → 슬라이딩 윈도우
  - [문서] ROADMAP 테이블 헤더 중복 제거, QA 스크립트 import 수정 (browser_use → langchain_ollama)
  - [TODO] `verify_session` session_id 필수화 — 프론트엔드 전송 확인 후 None → Forbidden 전환 예정

- **2026-04-06 — clippy 리팩토링 20건 완료 → clippy 경고 0건 달성**
  - target_table 대소문자 소문자 통일 (video 8, study 21, lesson 18건) + AMK_API_MASTER §3.2.3 감사 로그 규칙 추가
  - large_enum_variant 2건: LoginOutcome/OAuthLoginOutcome Success Box 래핑
  - AuditLogParams 구조체 + write_audit_log 헬퍼: 55곳 보일러플레이트 → 1곳 통합
  - LessonLogParams, CreateLessonParams, CreateStudyParams, AdminCreateUserParams(18→2), AdminUpdateUserParams 등 14개 구조체 도입
  - InsertOrderParams(24→2), CreateSubscriptionParams(13→2), TileRequest 등 나머지 전부 구조체화
  - 전수 검증: 계획 20건 vs 실제 20건 — 100% 일치 확인

- **2026-04-03 — 코드 점검 1~4단계 + 일괄 수정 완료**
  - `docs/AMK_CODE_AUDIT_PLAN.md` 신규: 4단계 점검 계획 (의존성 취약점, 코드 품질, 보안 리뷰, 문서 정합성)
  - 점검 1: 의존성 취약점 — cargo update (time/rustls-webpki 패치), npm audit fix (8건→0건), npm update
  - 점검 2: 코드 품질 — clippy --fix 20건 자동 + 수동 2건, unwrap() 위험 5건 수정, hooks 위반 2건 수정
  - 점검 3: 보안 — POST /courses AuthUser 추가, MFA rate limit 3건 추가, anti-enum 에러 메시지 2건 수정
  - 점검 4: 문서 정합성 — 스키마 9건 수정, API 문서 10건, 환경변수 9건 추가, 미구현 DTO 4건 표시
  - 미사용 의존성: aes-gcm 루트에서 제거 (hmac은 루트에서 사용 중 — 감사 오판 정정)
  - 전수 검증: 38건 중 37건 CONFIRMED, 2건 FALSE POSITIVE (ENCRYPTION_KEY + hmac)
  - 결과 기록: `docs/AMK_CODE_AUDIT_RESULT.md`

- **2026-04-02 — 순서 7.5: 다국어 반응형 디자인 규격**
  - `utils/language_groups.ts` 신규: CJK / Tall Script / Relaxed Tracking 언어 그룹 분류
  - `i18n/index.ts`: `changeLanguage` 시 `<html>`에 `lang-cjk`, `lang-tall-script`, `lang-relaxed-tracking` CSS 클래스 동적 관리 + 초기 로드 대응
  - `index.css`: tracking-tight 조건부 해제 (th, my, km, si, hi, ne, mn), tall script line-height 1.8 (th, my, km), `break-keep-cjk` 유틸리티 클래스
  - `hero_section.tsx`: `whitespace-nowrap` 제거 + `break-keep` → `break-keep-cjk` (marketing + list variant)
  - `book_landing_page.tsx`, `coming_soon_page.tsx`: `break-keep` → `break-keep-cjk`

- **2026-04-02 — Dockerfile 워크스페이스 빌드 수정 (2회 실패 → 해결)**
  - 1차: `crates/crypto/` 매니페스트+소스 COPY 누락 → Cargo 워크스페이스 멤버 미발견으로 빌드 실패
  - 2차: 2차 빌드 `touch`에 `crates/crypto/src/lib.rs` 누락 → Cargo가 더미 캐시 사용으로 빌드 실패
  - `docs/AMK_DEPLOY_OPS.md` §8-2에 워크스페이스 멤버 추가 시 Dockerfile 수정 체크리스트 추가

- **2026-04-01 — 순서 7: amazing-korean-crypto 크레이트 추출 (Cargo 워크스페이스)**
  - `src/crypto/{cipher,blind_index,service}.rs` → `crates/crypto/src/`로 이동
  - 자체 에러 타입 `CryptoError` + `CryptoResult` 정의 (thiserror), 백엔드에서 `From<CryptoError> for AppError` 변환
  - Cargo 워크스페이스: `[workspace] members = [".", "crates/crypto"]`
  - `src/crypto/mod.rs`는 re-export 래퍼로 유지 — 기존 14개 파일의 `use crate::crypto::*` 변경 0건
  - 크레이트 46/46 테스트 통과, 백엔드 cargo check 통과

- **2026-04-01 — 순서 6: 모바일 인증 엔드포인트 구현 (login-mobile + refresh-mobile)**
  - `POST /auth/login-mobile`: 기존 `AuthService::login()` 재사용, refresh token을 JSON body로 반환 (`LoginMobileRes`)
  - `POST /auth/refresh-mobile`: `RefreshReq` body에서 토큰 추출, `X-Platform: mobile` 헤더 필수 검증
  - `LoginOutcome::Success`에 `refresh_token: String` 필드 추가 (쿠키와 raw 값 동시 보존)
  - DTO: `LoginMobileRes { user_id, access, session_id, refresh_token, refresh_expires_in }`
  - 기존 `RefreshReq` DTO 재사용 (별도 `RefreshMobileReq` 불필요)
  - 수정: `dto.rs`, `handler.rs`, `service.rs`, `router.rs` + 문서 4건 동기화

- **2026-04-01 — 모바일 UX 79건 전수 수정 (§04 Mobile Checklist 완료)**
  - **Phase 1 터치 타겟**: `@media (pointer: coarse)` 글로벌 CSS — 모든 button/[role="button"]에 min-h/w 44px 강제 (터치 기기 전용, 데스크탑 유지). shadcn DropdownMenuItem/SelectItem 미영향 검증
  - **Phase 2 컴포넌트**: Dialog 닫기 아이콘 h-4→h-5 + p-1 패딩, EmptyState py-20→py-10 md:py-20, Lightbox 버튼 w-10→w-11
  - **Phase 3 오버플로우**: home/book_landing/book_hub grid-cols-3→grid-cols-1 sm:grid-cols-3, ebook_preview grid-cols-4→2 sm:4, textbook/ebook detail modal max-w-3xl→max-w-[calc(100vw-2rem)] sm:max-w-3xl, footer cert modal 동일
  - **Phase 4 타이포**: pricing text-4xl→2xl sm:4xl, order status 명암비 50→60%, verify_email/signup text-xs→sm, TOC leading-tight→snug, translation 10px→xs, study subtitle max-w-2xl
  - **Phase 5 간격**: empty state py-16→py-8 md:py-16 (7곳), 섹션 py-20→py-section-sm md:py-section-lg (6곳), 카드 p-8/p-10→p-5/p-6 md:p-8/p-10 (4곳), order status py-16→py-section-sm md:py-section-md
  - **Phase 6 모달 네비**: textbook/ebook detail modal 화살표 w-9→w-11 (44px)
  - **Phase 7 네비게이션**: header 햄버거 p-2→p-2.5 (44px), body 스크롤 잠금 useEffect 추가
  - **iOS safe area**: AMK_APP_ROADMAP에서 "보류 — 모바일 앱 개발 시" 확인, 명시적 제외

- **2026-03-31 — 남은 작업 리스크 분석 + 코드베이스 팩트체크**
  - 남은 작업 9개(Paddle Live~Tauri)에 대해 리스크 17건 식별, 코드베이스 전수 검증
  - 32개 주장 팩트체크: 31개 확인, 1개 수정 (Paddle Secret 13→12개)
  - `docs/AMK_STATUS.md §8.2` 검증된 리스크 테이블 추가 (근거 파일:라인 포함)

- **2026-03-31 — 디자인 시스템 v4 Phase V1-9~V1-10 (색상 토큰 교체 + 최종 문서)**
  - **Phase V1-9 색상 토큰 교체** (7파일):
    - status badge: `text-amber-600 bg-amber-50 border-amber-200` → `text-status-warning bg-status-warning/5 border-status-warning/20` (ebook_selected_detail, ebook_detail_modal, textbook_detail_modal, selected_book_detail)
    - status badge: `text-emerald-600 bg-emerald-50 border-emerald-200` → `text-status-success bg-status-success/5 border-status-success/20` (textbook_detail_modal, selected_book_detail)
    - order status: `text-emerald-600 bg-emerald-600/10` → `text-status-success bg-status-success/10` (textbook_order_status_page paid)
    - coming-soon badge: `bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400` → `bg-status-warning/10 text-status-warning` (home_page 2곳, dark: 접두사 불필요 — CSS 변수 자동 처리)
    - fullscreen viewer: `bg-neutral-900` → `bg-surface-inverted`, `text-white` → `text-surface-inverted-foreground`, `bg-neutral-100 dark:bg-neutral-800` → `bg-muted` (ebook_viewer_page 4곳)
    - warning icon: `text-amber-500` → `text-status-warning` (ebook_viewer_page 403 에러)
  - **의도적 유지** (장식용 8건): book_hub_page SLIDE_COLORS 6색 팔레트, textbook_order_page 주문 안내 4색 아이콘 — 시각 구분 목적, 시맨틱 의미 없음
  - **Phase V1-10 문서**: AMK_DESIGN_SYSTEM.md V1-9 반영 (changelog, surface-inverted 용도 확대, lint:ui 예외 기록), AMK_STATUS.md #51 추가, 메모리 갱신

- **2026-03-31 — 디자인 시스템 v4 Phase V1-5 (DataTable 블록 추출)**
  - **Phase V1-5 DataTable**: `components/blocks/data_table.tsx` 신규 — 제네릭 `DataTable<T>` 컴포넌트 + `useDataTable` 훅 (검색/정렬/선택/페이지네이션 상태 일괄 관리)
  - **적용**: admin_users_page (573→275줄), admin_lessons_page (502→235줄), admin_videos_page (483→240줄) — 각 ~200줄 보일러플레이트 제거
  - **Column 정의 방식**: `DataTableColumn<T>` 인터페이스 (key, header, sortField?, skeletonWidth, render)
  - **데이터 구조 차이 흡수**: 각 페이지의 다른 응답 구조(meta/pagination/flat)를 props로 정규화 (data, totalPages, totalCount, getId)

- **2026-03-31 — 디자인 시스템 v4 Phase V1-4~V1-8 (레이아웃/블록 추출 + 접근성)**
  - **Phase V1-4 AuthLayout**: `components/layouts/auth_layout.tsx` 신규, 6개 인증 페이지 8곳의 동일 래퍼(`flex min-h-screen` + `Card max-w-md`) → `<AuthLayout>` 추출 (login, signup, verify_email, reset_password, request_reset_password, account_recovery)
  - **Phase V1-6 CoverCard + FeatureGrid**: `blocks/cover_card.tsx` — textbook/ebook 카탈로그 인라인 표지 카드 → 공통 블록, `blocks/feature_grid.tsx` — home/about 가치 카드 3개 → 공통 블록 (4파일 적용)
  - **Phase V1-7 SectionContainer 확대**: study_list/detail, lesson_list, pricing 4파일 6곳의 수동 `<section>/<div>` 래퍼 → `<SectionContainer>` 교체 (ebook_purchase_complete, textbook_order는 컨테이너 불일치로 제외)
  - **Phase V1-8 lazy loading**: book_landing_page 국기 이미지 2곳 `loading="lazy"` 추가, 전체 `<img>` 누락 0건 확인

- **2026-03-31 — 디자인 시스템 v4 Phase V1-1~V1-3 (토큰 정리 + 아키텍처 전환)**
  - **Phase V1-1 토큰 정리**: `--table-header` CSS 변수 삭제 (light/dark, 0회 사용), Badge `info` variant 삭제 (manager→secondary 교체), Button `link` variant 삭제, `maxWidth` 컨테이너 토큰 3종 추가 (`container-default` 1350px, `container-narrow` 768px, `container-form` 448px), 글로벌 `prefers-reduced-motion` CSS 리셋 추가 (98.5% 애니메이션 접근성 갭 해소)
  - **Phase V1-2 컨테이너 마이그레이션**: `max-w-[1350px]` 하드코딩 17회 → `max-w-container-default` 토큰 교체 (11개 파일: header, footer, hero_section, section_container, 카탈로그, 리스트 페이지 등)
  - **Phase V1-3 아키텍처 전환**: `components/sections/` → `components/blocks/` 리네이밍 (3계층 아키텍처: ui/ → blocks/ → category/), 20개 소비 파일 41개 import 경로 교체
  - **디자인 시스템 v4 플랜**: 14개 영역 전수 감사 + 실증 검증 완료, 과잉 설계 5건 제거 (text-white 유지, Card elevated 유지, Layout 8→1개, text-sm 유지, Style Dictionary 연기)
  - **QA 자동화**: 3계층 QA 체계 설계 — Playwright 픽셀 비교 + Browser Use/Ollama AI 스모크 + Claude API 정밀 분석, `docs/AMK_MACMINI_SETUP.md` Day 6 추가

- **2026-03-30 — 홈/소개 페이지 리디자인 + 22개 locale 전면 업데이트**
  - **홈 페이지**: Hero 한 줄 타이틀 + 핵심가치 3카드(학습 보다 습득/효율적 학습/이해 중심) + 기능 4카드(교재/E-book/영상[준비중]/문제[준비중]) + CTA
  - **소개 페이지**: Hero("한국어는 어렵지 않습니다") + Why(Amazing Korean란?) + 차별점 3카드 상세(습득하는 한국어/학습 시간 단축/모국어 중심 이해) + Stats 카드(2블록) + CTA
  - **"30,000 표현" 전체 삭제**: ko/en 포함 22개 locale에서 완전 제거 (근거 없는 수치)
  - **숫자 갱신**: 20→34 언어, 44→68종 교재, 900→500 문장 (마스터 문서 동시 수정)
  - **HeroSection**: `whitespace-nowrap` + `max-w` 제거로 22개 언어 타이틀 한 줄 가운데 정렬
  - **22개 locale**: home/about 키 전면 교체 (5개 Agent 병렬 처리)
  - **SEO**: home/about description 갱신 ("1:1 수업" 등 미구현 기능 제거)

- **2026-03-30 — 앱 로드맵 + 문서/메모리 정비**
  - **신규**: `docs/AMK_APP_ROADMAP.md` — Flutter(모바일) + Tauri 2.x(데스크탑) 확정, 리스크 11건 검증, 전체 우선순위
  - **STATUS §8.2**: 의존성 기반 실행 순서 재정렬, 다국어 반응형 디자인 규격(#7.5) 추가
  - **기존 6개 문서**: React Native/TBD → Flutter/Tauri 참조 갱신
  - **archive**: 완료/1회성 문서 4건 삭제 (DEVELOP_POINT, FOOTER_ISSUE, GSC_REDIRECT, MEMORY_AUDIT)
  - **메모리**: project_completed_systems 축소(문서 포인터), plan_task6 축소(로드맵 포인터), decisions 갱신(이메일 완료, Flutter 확정)
  - **DB 암호화 Bug#1**: 재검증 결과 service.rs에서 이미 복호화 구현됨 → 해결됨 처리
  - **교재 번역**: amazing-korean-books 조사 — 22→34언어 확장 완료, PDF 재생성 남음
  - **마스터 문서**: 900문장 → 500문장 정정, basic_900 enum 레거시 표기

- **2026-03-29 — E-book 요청별 HMAC 서명 (Phase 1-2 완료)**
  - **HMAC 서명 검증**: 페이지/타일 요청마다 `X-Ebook-Signature` + `X-Ebook-Timestamp` 헤더 필수화, 세션 등록 시 32바이트 랜덤 secret 생성 → Redis 저장, `ViewerMetaRes`에 `hmac_secret` 추가
  - **서명 알고리즘**: HMAC-SHA256, payload = `{session_id}:{path}:{timestamp}`, ±30초 타임스탬프 윈도우, 상수 시간 비교 (타이밍 공격 방지)
  - **프론트엔드**: Web Crypto API `crypto.subtle.sign("HMAC")`, hex 인코딩, `fetchPageImage`/`fetchPageTile` 호출 시 자동 서명
  - **CORS**: `x-ebook-signature` + `x-ebook-timestamp` 헤더 허용 추가
  - **Phase 1 웹 보안 5/5 완료**: 저작권고지 + 진위확인 + 이미지암호화 + DevTools감지 + HMAC서명

- **2026-03-29 — E-book 보안 Phase 1 착수 + 보안 전략 문서**
  - **저작권 보호 고지 모달**: 뷰어 최초 진입 시 저작권법 제104조의2 고지 모달 표시 (ShieldCheck 아이콘, sessionStorage 중복 방지), 22개 locale 번역 (`copyrightTitle`/`copyrightNotice`/`copyrightLegal`/`copyrightWatermark`/`copyrightConfirm`)
  - **워터마크 진위확인 API**: `GET /admin/ebook/verify/{watermark_id}` — 관리자 전용, ebook_access_log JOIN ebook_purchase로 purchase_code/user_id/page_number/ip/user_agent/created_at 반환
  - **이미지 AES-256-GCM 암호화 저장**: `cipher.rs`에 `encrypt_bytes()`/`decrypt_bytes()` 추가, `EBOOK_IMAGES_ENCRYPTED=true` 시 `.webp.enc` 파일 복호화 후 워터마크 적용 (페이지+타일 모두), 테스트 25개 통과
  - **DevTools 감지**: `devtools_detect.ts` 신규 — 창 크기 변화(200px+) + console.log getter 호출 감지, `useDevToolsDetection` 훅 2초 폴링, 감지 시 3초 유예 후 콘텐츠 블러 (isObscured), DevTools 닫으면 자동 복원

- **2026-03-29 — E-book 보안 전략 문서 신규 작성**
  - **신규**: `docs/AMK_EBOOK_SECURITY.md` — 대한민국 공문서 발급 보안 체계, 한국 E-book DRM 사례 (교보/리디/알라딘), 알라딘 2023 유출 사건 분석, 플랫폼별 보안 역량 (Android/iOS/macOS/Windows), 앱 프레임워크 비교, 저작권법 법적 근거 정리
  - **조사 방법**: WebSearch 150+회 실증 검증 (AMK_WORKS_RULE.md 절차 준수), 모든 항목 출처 URL 명시
  - **AMK_STATUS.md**: 진행 예정 항목에 E-book 웹 보안 강화(Phase 1), 모바일 앱(Phase 2), 데스크탑 앱(Phase 3) 추가

- **2026-03-28 — E-book 보안 강화 + Gemini 코드 리뷰 반영**
  - **보안 강화 (커밋 9247413)**: CORS `x-ebook-viewer`/`x-ebook-session` 허용, verify_session session_id 비교, Content-Disposition/Referrer-Policy/Cache-Control `no-store` 헤더, Rate Limit TOCTOU 경합 수정 (3곳), 마이크로도트 y좌표 ±3px 분산, Heartbeat 실패 시 Canvas 즉시 클리어, print CSS `body *` 전체 숨김 강화
  - **Gemini 리뷰 반영 (커밋 7854e87)**: `AMK_DEPLOY_OPS.md` 마이그레이션 예시 HHMMSS 모순 수정, `AMK_CHANGELOG.md` embla 표현 수정, `embla-carousel-react` 미사용 패키지 제거, `use_page_image.ts` queryClient 불필요 의존성 제거, TiledPageCanvas `Promise.all` → `Promise.allSettled` (부분 렌더링 지원)

- **2026-03-27 — 교재 주문 안내 카드 UI 개선 + 30부 할인 안내 추가**
  - 주문 안내 카드 3열 → 4열 그리드 (sm:2 lg:4), 30부 이상 할인 카드 추가 (BadgePercent 아이콘)
  - 카드별 아이콘 색상 차별화 (blue/emerald/violet/amber)
  - 배송 문구 "무료 배송"으로 변경, 카드 텍스트 줄바꿈 (whitespace-pre-line)
  - ko/en locale 업데이트 (orderGuideDiscount 추가, 기존 키 줄바꿈 반영)

- **2026-03-26 — Book 허브 갤러리 재구성 + 상세 모달 확장 + 라이트박스 + 가격 통일 + 유효성 검증 i18n**
  - **허브 페이지 재구성**: `/book` 허브 페이지를 인터랙티브 6슬라이드 갤러리로 전면 개편 (커버 + 샘플 5장), 좌=이미지 갤러리(좌우 네비게이션+인디케이터 점), 우=제목+키워드 태그+설명+스펙 요약 카드+CTA 버튼
  - **슬라이드별 키워드 태그**: 슬라이드마다 3개 색상 태그 (SLIDE_COLORS 6색 매핑: blue, emerald, amber, violet, rose, teal)
  - **스펙 요약 카드**: 124 페이지 / 22개 언어 / 가격 — 아이콘+텍스트 가로 3열 그리드
  - **상세 모달 6이미지 확장**: textbook/ebook 상세 모달 3장(cover/inner/toc) → 6장(cover + p7/p18/p29/p53/p118), `SAMPLE_PAGES` 배열 + `getImageSrc()` 함수, 슬라이드별 설명(`bookHub.slideTitle/Desc0~5`) 추가
  - **신규 라이트박스**: `image_lightbox.tsx` — `createPortal`로 `document.body`에 렌더링, blur 배경(`bg-background/60 backdrop-blur-sm`), 좌우 ChevronLeft/Right 네비게이션, ESC/Arrow 키보드 지원
  - **Radix Dialog 호환**: Radix Dialog가 `document.body`에 `pointer-events: none`을 설정하므로 라이트박스에 `pointer-events-auto` + `z-[100]` + `stopPropagation` 적용, 라이트박스를 DialogContent 내부에서 포탈로 렌더링하여 포커스 트랩 우회
  - **카탈로그 타입 토글**: textbook/ebook 카탈로그 페이지에 "도서 | E-book" 전환 탭 추가 (학생/교사 탭 좌측), `CatalogTypeToggle` 컴포넌트 양쪽에 추가
  - **E-book 가격 단일화 (백엔드)**: `ebook/service.rs` — 학생/교사 분리 가격(`TEACHER_PRICE_KRW` 15,000 / `STUDENT_PRICE_KRW` 12,000) → 통일 `EBOOK_PRICE_KRW` 15,000, USD $10.00 → $9.99
  - **가격 표시 통일 (프론트)**: E-book 카드/모달/캐러셀에서 API 동적 가격 → i18n 고정 문자열(`15,000 KRW`) 변경, 도서도 `25,000 KRW` 통일 형식
  - **Zod 유효성 검증 i18n**: `auth/types.ts` — signup/login/reset/find 전 스키마의 `.email()`/`.min()`/`.max()`/`.date()` 에러 메시지를 `i18n.t()` 다국어 키로 교체 (10키 추가), `textbook_order_page.tsx` — 주문 폼 스키마 유효성 메시지 i18n화 (9키 추가)
  - **주문 안내 섹션 이동**: 교재 카탈로그 하단 주문 안내 3카드(최소수량/결제방법/배송) + CTA 버튼 → 주문 페이지 상단으로 이동
  - **내부 링크 수정**: `ebook_my_purchases_page.tsx` 뷰어 링크 `/ebook/viewer/` → `/book/ebook/viewer/`, `textbook_order_status_page.tsx` 견적서/주문확인서 링크 `/textbook/order/` → `/book/textbook/order/`, `textbook_order_page.tsx` 주문 추적/견적서 링크 동일 수정
  - **Google Translation API 잔여물 정리**: `types.rs` `to_gcp_code()` dead code 삭제, `deploy.yml` 환경변수 3개 제거, `docs/AMK_API_LEARNING.md` 문구 수정
  - **캐러셀 씰 선택 링 제거**: `seal_list.tsx` — 선택된 국기 씰의 `ring-2 ring-primary ring-offset-2` 제거 (크기 차이+언어명+opacity로 충분히 구분)
  - **캐러셀 씰 크기 조정**: 선택된 씰 `w-24 h-24 md:w-28 md:h-28` → `w-26 h-26 md:w-32 md:h-32`로 확대
  - **카탈로그 카드 이미지 구분선**: textbook/ebook 그리드 카드 이미지 영역에 `border-b` 추가 (이미지↔메타정보 시각 구분)
  - **상세 모달 학생/교사 표기 제거**: textbook/ebook 상세 모달 DialogDescription에서 학생용/교사용 텍스트 숨김 (`sr-only`)
  - **상세 모달 간격 조정**: textbook/ebook 상세 모달 교재명↔이미지 갤러리 사이 `mt-4` 추가
  - **E-book 모달 페이지 수 제거**: E-book 상세 모달에서 `total_pages` 표시 삭제
  - **E-book "곧 출판 예정" 뱃지**: E-book 상세 모달+캐러셀 우측 패널에 학생/교사 모두 amber warning 뱃지 (`AlertTriangle` + "E-book · 학생용/교사용 · 곧 출판 예정")
  - **뱃지 크기 통일**: textbook/ebook 전체 뱃지 `text-xs px-2.5 py-1` → `text-sm px-3 py-1.5`로 통일
  - **i18n**: 22개 로케일 일괄 업데이트 — `bookHub.slideTitle/Desc/Tags0~5` 18키, `bookHub.spec*` 3키, `bookHub.tab*` 2키, `auth.validation*` 10키, `textbook.order.validation*` 9키, `ebook.catalog.pricePerUnit/bookTitle/bookDescription` 3키, `ebook.detail.teacherComingSoon/studentComingSoon` 2키, "20개"→"22개" 전역 수정
  - **문구 수정**: 카탈로그 제목 "놀라운 한국어 도서"/"놀라운 한국어 E-book", 탭 "학생용"/"교사용", CTA "도서 보기"/"E-book 보기"

- **2026-03-25 — E-book 카탈로그 출판본 패턴 적용**
  - **리디자인**: E-book 카탈로그를 출판본(textbook) 패턴으로 통일 (그리드 CoverCard + 캐러셀 SealList + 상세 모달)
  - **신규**: `ebook_catalog_page.tsx` 전면 리라이트 — HeroSection, Tabs(학생/교사), 검색, 그리드/캐러셀 뷰 토글
  - **신규**: `ebook_detail_modal.tsx` — 좌우 스와이프 이미지 갤러리 (겉표지/속표지/목차), E-book 뱃지, 가격/페이지 표시
  - **신규**: `ebook_carousel_view.tsx` + `ebook_selected_detail.tsx` — 50/50 레이아웃 (SealList 재사용)
  - **신규**: `use_ebook_catalog_view.ts` — 에디션/검색/선택 상태 관리 훅
  - **공유**: SealList를 `SealItem` 인터페이스로 일반화하여 textbook/ebook 공유
  - **이미지**: 출판본과 동일 표지 이미지 공유 (`/covers/{edition}-{lang}.webp`)
  - **i18n**: 22개 로케일 `ebook.catalog.*` 7키 + `ebook.detail.*` 7키 추가
  - **구매 섹션**: 사용자 결정 대기 (기존 구매 플로우 유지 가능)

- **2026-03-25 — Book 허브 페이지 + 라우트 재구성**
  - **라우트**: `/textbook/*` → `/book/textbook/*`, `/ebook/*` → `/book/ebook/*` 전면 이동
  - **신규**: `/book` Book 허브 랜딩 페이지 (교재 소개 + 표지 + 샘플 페이지 + 출판본/E-book CTA)
  - **허브 기본 교재**: i18n 언어 기반 자동 선택 (ko/en → 랜덤, 기타 → 매칭 언어)
  - **리다이렉트**: 기존 `/textbook/*`, `/ebook/*` 경로 → 새 경로로 자동 리다이렉트 (하위 호환)
  - **헤더**: `nav.textbook` → `nav.book`, 경로 `/textbook` → `/book`
  - **내부 링크**: 13개 파일 전수 변경 (header, my_page, book_landing, catalog, order, viewer 등)
  - **E-book 카탈로그**: 에디션 선택 Button → Tabs 컴포넌트 통일 (textbook과 동일 패턴)
  - **신규 파일**: `book_hub_page.tsx`, `book_data.ts`에 `getDefaultLangKey()`, `SAMPLE_PAGES` 추가
  - **i18n**: 22개 로케일 `nav.book` + `bookHub.*` 10키 추가
  - **QR 랜딩**: `/book/:isbn` 유지 (React Router v6 정적 경로 우선 매칭으로 충돌 없음)
  - **샘플 이미지**: `/book-samples/{type}-{lang}-p{page}.webp` 경로 정의 (220장, amazing-korean-books에서 생성 예정)

- **2026-03-25 — 교재 그리드/상세 모달 개선**
  - **그리드 CoverCard**: 교재명 `bookTitle` i18n 키 통일, ISBN 뱃지 제거 → 상세 모달로 이동, 버튼 "주문하기" → "상세보기", 가격 오른쪽 정렬
  - **상세 모달**: 이미지 갤러리 썸네일 선택 → 좌우 화살표 + 인디케이터 도트 스와이프, 교재명 `bookTitle` 좌측 + ISBN 뱃지 우측 (emerald/amber 스타일), 가격 우측 정렬

- **2026-03-25 — 교재 캐러셀 모바일 최적화**
  - **seal_list.tsx**: 모바일에서 상단 Coverflow 숨김 (`hidden md:flex`), 하단 Thumbs만 표시 + 선택 언어명 표시
  - **selected_book_detail.tsx**: 모바일 세로 쌓기 (표지 위 → 설명 아래), 텍스트 중앙 정렬

- **2026-03-24 — 교재 ISBN 발급 상태 표시**
  - **백엔드**: `textbook/dto.rs` CatalogItem에 `isbn_ready: bool` 필드 추가
  - **백엔드**: `textbook/service.rs` `catalog_languages()` 21개 언어에 ISBN 발급 상태 하드코딩 (9개 완료: ja, zh_cn, vi, th, ne, ru, km, tl, id)
  - **프론트**: `types.ts` — `textbookLanguageSchema`에 누락된 `"tl"` 추가 (기존 버그 수정), `catalogItemSchema`에 `isbn_ready` 추가
  - **프론트**: 카탈로그 카드/캐러셀/상세 모달에 ISBN 미발급 시 "약 1주 추가 소요" 안내 텍스트
  - **프론트**: 주문 페이지에서 ISBN 미발급 언어 선택 시 안내 메시지 표시
  - **i18n**: ko/en에 `textbook.catalog.isbnPending`, `textbook.order.isbnNotice` 키 추가

- **2026-03-24 — E-book Paddle 결제 연동**
  - **수정**: `ebook_catalog_page.tsx` — 결제 방식 선택 UI (계좌이체/카드결제), Paddle 선택 시 `openEbookCheckout()` 호출
  - **수정**: `ebook_catalog_page.tsx` — `usePaddle` 훅 연동 (카탈로그 API의 `client_token`, `sandbox`, `paddle_ebook_price_id` 활용)
  - **수정**: `ebook_catalog_page.tsx` — Paddle 결제 완료 시 `checkout.completed` 이벤트 → 구매 완료 페이지 이동
  - **i18n**: 22개 로케일 `ebook.purchase.cardPayment`, `ebook.purchase.paddleNote` 키 추가

- **2026-03-24 — E-book 모바일 최적화**
  - **수정**: `ebook_viewer_page.tsx` — 터치 스와이프 페이지 네비게이션 (50px 수평 임계값, 세로 스크롤 무시)
  - **수정**: `ebook_viewer_page.tsx` — 모바일(768px 미만)에서 spread 모드 자동 비활성화 + 토글 버튼 숨김
  - **수정**: `ebook_viewer_page.tsx` — 모바일 UI 최적화 (페이지 표시/줌% 텍스트 숨김, 하단 바 축소, 슬라이더 flex 확장)

- **2026-03-24 — E-book 환불 정책**
  - **수정**: `refund_policy_page.tsx` — SECTIONS 4 → 5 (E-book 섹션 추가)
  - **i18n**: 22개 로케일 `legal.refund.s5Title/s5Content` 추가 (pending 즉시취소, 미열람 7일 환불, 열람 후 불가, Paddle 별도)
  - **수정**: `ebook_catalog_page.tsx` — 카탈로그 헤더에 "환불 정책 보기" 링크 추가
  - **i18n**: 22개 로케일 `ebook.catalog.refundPolicy` 키 추가

- **2026-03-24 — E-book 이메일 알림**
  - **email.rs**: `EbookPurchaseConfirmation` (구매 접수) + `EbookPurchaseCompleted` (결제 완료) 2개 템플릿 추가
  - **ebook/service.rs**: `create_purchase()` 후 구매 접수 확인 이메일 발송 (fire-and-forget)
  - **admin/ebook/service.rs**: `update_status()` → Completed 전환 시 결제 완료 이메일 발송
  - **payment/service.rs**: Paddle webhook `handle_ebook_transaction_completed()` → 결제 완료 이메일 발송
  - **ebook/repo.rs**: `find_user_encrypted_email()` 추가 (users 테이블에서 암호화 이메일 조회)
  - **ebook/service.rs**: `language_name_ko()`, `edition_label_ko()` 헬퍼 함수 추가 (pub)

- **2026-03-24 — E-book 샘플 미리보기**
  - **신규 파일**: `ebook_preview_modal.tsx` — 표지/목차/샘플1/샘플2 이미지 갤러리 모달 (교재 상세 모달 패턴 기반)
  - **수정**: `ebook_catalog_page.tsx` — 언어 카드에 "미리보기" 버튼 추가, 모달 상태 관리
  - **이미지 경로**: `/ebook-previews/{edition}/{language}/{cover|toc|sample-1|sample-2}.webp`
  - **i18n**: 22개 로케일 `ebook.preview` 섹션 8키 추가

- **2026-03-24 — E-book 구매 완료 안내 페이지**
  - **신규 파일**: `ebook_purchase_complete_page.tsx` — 구매 완료 전용 페이지 (`/ebook/purchase-complete`)
  - **구성**: 성공 아이콘 + 구매코드 (복사) + 에디션/가격/결제수단 요약 + 계좌이체 입금안내 (bank_transfer일 때) + 내 E-book/카탈로그 버튼
  - **수정**: `ebook_catalog_page.tsx` — `onSuccess` → `navigate("/ebook/purchase-complete", { state: data })` 변경, `routes.tsx` — PrivateRoute 내 `/ebook/purchase-complete` 추가
  - **i18n**: 22개 로케일 `ebook.purchaseComplete` 섹션 16키 추가

- **2026-03-24 — 견적서/주문확인서 사용자 공개**
  - **신규 파일**: `textbook_order_print.tsx` — 사용자용 견적서/주문확인서 인쇄 페이지 (`/textbook/order/:code/print?type=quote|confirmation`)
  - **수정**: `textbook_order_status_page.tsx` — 견적서/주문확인서 버튼 추가 (새 탭), `textbook_order_page.tsx` — 주문 완료 화면에 견적서 버튼 추가
  - **라우트**: `/textbook/order/:code/print` Public 라우트 추가
  - **i18n**: 22개 로케일 `textbook.print` 섹션 추가 (quoteTitle, confirmationTitle, pdfGuide, quoteNotice, confirmationNotice 등)
  - **방식**: `window.print()` + 브라우저 "PDF로 저장" (별도 PDF 라이브러리 없음)

- **2026-03-24 — 구매이력 + 비회원 주문 차단**
  - **마이그레이션**: `20260324_textbook_user_id.sql` — `textbook.user_id BIGINT REFERENCES users` + 인덱스 (NULLABLE: 기존 주문 호환)
  - **백엔드**: `POST /textbook/orders` 인증 필수화 (`AuthUser` extractor), `GET /textbook/my` 내 주문 목록 API 추가, `MyOrdersRes` DTO
  - **프론트**: `/textbook/order` → PrivateRoute 이동, `/textbook/my` 내 주문 목록 페이지, 마이페이지 "구매이력" 버튼 (Receipt 아이콘), 주문 폼 사용자 정보 자동 채움 (name, email)
  - **i18n**: 22개 로케일 `textbook.myOrders` 섹션 + `user.purchaseHistory` 키 추가

- **2026-03-24 — 교재 상세 모달 추가**
  - **신규 파일**: `textbook_detail_modal.tsx` — 겉표지/속표지/목차 이미지 갤러리 + 교재 설명 + 주문 버튼 (shadcn/ui Dialog, max-w-3xl)
  - **수정**: `textbook_catalog_page.tsx` — CoverCard 클릭 → 모달 열기 (기존 직접 주문 링크 → 모달 경유), `selected_book_detail.tsx` — "상세 보기" 버튼 추가
  - **i18n**: 22개 로케일 `textbook.detail` 섹션 9키 추가 (`modalTitle`, `coverImage`, `innerImage`, `tocImage`, `description`, `translationNote`, `orderNow`, `viewDetail`, `imageNotAvailable`)
  - **이미지 fallback**: 속표지/목차 이미지 미준비 시 ImageOff 아이콘 + "이미지 준비 중" 텍스트

- **2026-03-25 — 교재 캐러셀 Swiper 전환 + 국가 씰 SVG**
  - **전환**: embla-carousel + CSS 3D 수동 구현 → **Swiper v12** (EffectCoverflow + Thumbs + FreeMode 모듈)
  - **사유**: 커스텀 드래그 구현의 UX 한계 (부자연스러운 드래그, 센터링 이슈) → Swiper 네이티브 coverflow 효과로 해결
  - **삭제**: `carousel_3d.tsx` (embla-carousel 기반 3D 회전목마)
  - **신규**: `seal_list.tsx` — 상단 Coverflow 캐러셀 (국가 씰 SVG, center + 좌우 2개, 클릭/드래그 선택) + 하단 Thumbs 스트립 (클릭 동기화, opacity 피드백)
  - **수정**: `textbook_carousel_view.tsx` — Carousel3D → SealList 교체, 50/50 그리드 레이아웃 (씰 리스트 + 선택 교재 상세)
  - **수정**: `selected_book_detail.tsx` — 가로 레이아웃 (표지 좌측 고정 + 설명 우측), ISBN 뱃지, 학생용/교사용 설명 분리, i18n 키 추가
  - **수정**: `index.css` — Swiper CSS 임포트 4종 (swiper/css, effect-coverflow, free-mode, thumbs)
  - **기능**: 상단↔하단 양방향 동기화 (클릭/드래그 모두), 비가시 슬라이드 자동 숨김 (watchSlidesProgress)
  - **에셋**: `frontend/public/seals/*.svg` 24개 국가 씰 SVG 파일
  - **패키지**: `swiper@^12` 추가, `embla-carousel-react` 제거 (미사용)
  - **i18n**: 22개 로케일 — `bookTitle`, `bookDescriptionStudent`, `bookDescriptionTeacher`, `editionInfo` 키 추가

- **2026-03-24 — 교재 카탈로그 캐러셀 뷰 추가**
  - **신규 파일**: `textbook_carousel_view.tsx` (캐러셀 뷰 메인), `selected_book_detail.tsx` (선택 교재 상세), `use_catalog_view.ts` (캐러셀 상태 훅)
  - **수정**: `textbook_catalog_page.tsx` — 그리드 ↔ 캐러셀 뷰 토글 (LayoutGrid/Disc3 아이콘, localStorage 저장), 기본 뷰 모드 carousel로 변경
  - **i18n**: 22개 로케일 5키 추가 (`viewGrid`, `viewCarousel`, `searchPlaceholder`, `noResults`, `bookDescription`)

- **2026-03-24 — Google Cloud Translation API 해지**
  - **사유**: 번역은 Claude Code에서 직접 수행하고 있어 Google Translate API 불필요. 비용 절감 + 코드 단순화
  - **삭제**: `src/external/translator.rs` (GoogleCloudTranslator + TranslationProvider trait), `POST /auto`, `POST /auto-bulk` 엔드포인트, AppState.translator 필드
  - **삭제 (프론트)**: Auto Translate 모드 UI, useAutoTranslate/useAutoTranslateBulk 훅, auto API 함수, 자동번역 타입 스키마
  - **삭제 (환경변수)**: `TRANSLATE_PROVIDER`, `GOOGLE_TRANSLATE_API_KEY`, `GOOGLE_TRANSLATE_PROJECT_ID`
  - **유지**: 번역 CRUD (생성/수정/삭제/조회), 상태 관리, 통계, 검색, 콘텐츠 레코드/소스 필드 조회, Manual Input 모드

- **2026-03-23 — sqlx 마이그레이션 버전 순서 수정 (프로덕션 크래시 대응)**
  - **사고 원인 1**: 부트스트랩 스크립트에 프로덕션 미적용 마이그레이션 6개를 "적용됨"으로 등록 → sqlx가 건너뛰어 테이블 미생성
  - **사고 원인 2**: ebook 마이그레이션 파일명 `20260310000001`이 정수 비교 시 `20260312`보다 큰 값 → 테이블 생성 전에 ALTER 시도 → 크래시 루프
  - **수정**: `20260310000001_ebook.sql` → `20260311_ebook.sql`로 리네이밍
  - **교훈**: 같은 날짜 충돌 시 `000001` 접미사 대신 다음 날짜 사용. 부트스트랩은 프로덕션 DB 실제 상태 확인 후 작성

- **2026-03-23 — 구독 요금제 + E-book Paddle 결제 차단**
  - **사유**: 콘텐츠 미준비 상태에서 결제 방지
  - **구독 요금제**: `/pricing` → ComingSoonPage 교체, 헤더 "요금제" 메뉴 제거
  - **E-book Paddle**: 카탈로그에서 Paddle 결제 옵션 제거 (계좌이체만 유지), 내 구매 목록에서 Paddle 재시도 버튼 숨김
  - **홈페이지**: Paddle.js 초기화 + `/payment/plans` API 호출 제거 (불필요한 리소스 로딩 방지)
  - **유지**: 백엔드 API, Admin 결제 관리, 교재 주문, E-book 계좌이체 — 모두 정상 유지
  - **복원**: 콘텐츠 준비 완료 시 라우트/헤더/Paddle 코드 원복

- **2026-03-23 — sqlx 자동 마이그레이션 전환**
  - **기존**: EC2 SSH 접속 → `docker exec psql < migration.sql` 수동 실행
  - **변경**: `sqlx::migrate!().run(&pool)` — 앱 부팅 시 자동 실행 (`main.rs`)
  - **Cargo.toml**: sqlx features에 `"migrate"` 추가
  - **파일 구조 변경**: SEED 파일 `seeds/`로 분리, version 충돌 파일 리네이밍 (다음 날짜 사용: `20260311`, `20260210000001`, `20260214000001`)
  - **프로덕션 전환**: `scripts/bootstrap_sqlx_migrations.sql` 1회성 실행 (기존 13개 마이그레이션 이력 등록)
  - **문서**: `AMK_DEPLOY_OPS.md` 섹션 4 전체 재작성

- **2026-03-23 — 세금계산서 홈택스 필수 항목 추가**
  - **추가 필드 5개**: `tax_company_name`(상호), `tax_rep_name`(대표자명), `tax_address`(사업장 주소), `tax_biz_type`(업태), `tax_biz_item`(종목)
  - **홈택스 필수**: 상호 + 대표자명은 세금계산서 요청 시 필수 (DB CHECK 제약 + 백엔드 검증 + 프론트 * 표시)
  - **홈택스 권장**: 사업장 주소, 업태, 종목은 선택 입력
  - **영향 범위**: DB 마이그레이션, 백엔드 DTO/Repo/Service, 프론트 주문 폼, Admin 상세/인쇄, i18n (ko+en)

- **2026-03-23 — 교재 카탈로그 리디자인 + 헤더 네비게이션**
  - **카탈로그 페이지 신규** (`/textbook`): 표지 이미지 기반 상품 그리드 (학생용/교사용 섹션 분리), 44개 표지 이미지 (`amazing-korean-books` 에서 static 복사), 주문 안내 섹션 (최소수량/결제방법/배송 3카드)
  - **주문 페이지 분리** (`/textbook/order`): 기존 `/textbook` → `/textbook/order`로 이동, 카탈로그에서 선택 시 URL 파라미터(`?lang=&type=`)로 자동 항목 추가, 주문 항목에 표지 썸네일 표시
  - **헤더 네비게이션**: NAV_ITEMS에 "교재" 메뉴 추가 (`/textbook`)
  - **i18n**: `textbook.catalog.*` 키 13개 추가 (ko + en), `nav.textbook` 키 추가

- **2026-03-23 — Coming Soon 페이지 + 에러 페이지 개선**
  - **ComingSoonPage 신규**: 영상/학습/레슨 라우트(`/videos`, `/studies`, `/lessons` + 하위)를 "콘텐츠 준비 중" 페이지로 대체. HeroSection + 3개 Feature 미리보기 카드(영상/패턴/수업) + E-book/교재 CTA. 콘텐츠 오픈 시 원래 컴포넌트로 복원 가능
  - **에러 페이지 RootLayout 통합**: 404/403/Error 페이지를 RootLayout 내부로 이동 → Header/Footer 유지. Card 기반 → HeroSection 기반으로 UI 일관성 개선
  - **i18n**: `comingSoon.*` 키 13개 추가 (ko + en)

- **2026-03-23 — 홈/소개 페이지 문구 & UI 개선**
  - **문구 전면 교체** (ko.json + en.json): 일반적 마케팅 문구 → 실제 차별점 기반 구체적 문구
    - Hero: "효과적이고 즐거운 학습" → "500개 핵심 문장으로 30,000개 표현을 익히는 TOPIK 연계 체계적 커리큘럼"
    - Trust Indicators: 플레이스홀더 숫자(1,000+/50+/10,000+) → 실제 차별점(20 지원 언어/500+ 핵심 문장/TOPIK 연계)
    - Feature 3번: "1:1 수업" (미구현) → "교재로 정리하기" (실제 판매 중), 경로 `/lessons` → `/ebook/catalog`
    - 소개 페이지: Mission "우리의 미션" → "왜 Amazing Korean인가", 통계 카드 전면 교체, Core Values 3가지 재정의
  - **UI 개선**: Feature/Value 카드 `hover:border-accent/50` 추가, 링크 `text-accent font-medium` 가시성 향상, Trust Indicators `text-gradient` 적용
  - **아이콘 교체**: Users→BookMarked (교재), Target→Layers (패턴), Heart→Languages (모국어), Globe→GraduationCap (TOPIK)
  - **문서 검증**: 미구현 기능 5개(모국어 자막, 구간 반복, 음성 입력, 오답 복습, 발음 가이드)를 구현된 기능으로 수정

- **2026-03-20 — E-book 뷰어 보안 강화 5단계**
  - **Step 1: Canvas 추출 API 무력화** (프론트): `toDataURL`, `toBlob`, `getImageData`, `captureStream`, `OffscreenCanvas`, `createImageBitmap(canvas)` 프로토타입 오버라이드 (뷰어 mount 시 적용, unmount 시 복원)
  - **Step 2: 포커스/가시성 감지** (프론트): `visibilitychange` (primary) + `blur`/`focus` (secondary) + `beforeprint`/`afterprint` → `filter: blur(30px)` + `will-change: filter` (GPU 가속)
  - **Step 3: DOM 조작 감지** (프론트): `MutationObserver` (canvas 삭제, style 변경) + `getComputedStyle` 주기 검사 (2초, CSS 규칙 추가 감지) → 탬퍼링 시 canvas 클리어 + 강제 퇴장
  - **Step 4: 동시 세션 제한** (백엔드+프론트): Redis `SET EX` user별 단일 세션 (Last Writer Wins), 90초 TTL / 30초 heartbeat (3:1 비율), `POST /ebook/viewer/heartbeat` 엔드포인트 추가, 새 기기 접속 시 기존 세션 자동 만료
  - **Step 5: 타일 분할 전송** (백엔드+프론트): 3×3 그리드(9 타일/페이지), 기능 플래그 `EBOOK_TILE_MODE` (기본 false), 전체 이미지에 워터마크 적용 후 `crop_imm` 분할, 나머지 픽셀 자동 대응, 전용 Rate Limit 270/분, `TiledPageCanvas` 컴포넌트 + `usePageTiles` 훅 추가
  - **환경변수 7개 추가**: `EBOOK_SESSION_TTL_SEC`, `EBOOK_TILE_MODE`, `EBOOK_TILE_GRID_ROWS/COLS`, `RATE_LIMIT_EBOOK_TILE_MAX/WINDOW_SEC`, `RATE_LIMIT_EBOOK_PAGE/PURCHASE` (docker-compose + .env.example)
  - **엔드포인트 2개 추가**: `POST /ebook/viewer/heartbeat`, `GET /ebook/viewer/{code}/pages/{page_num}/tiles/{row}/{col}`
  - **보안 아키텍처 3중 → 7중 강화**: 기존 (구조+워터마크+플랫폼) + Canvas 무력화 + 포커스 블러 + DOM 감지 + 세션 제한

- **2026-03-20 — QR 교재 랜딩 페이지 (`/book/:isbn`)**
  - **라우트**: `/book/:isbn` — 교재 속표지 QR 코드 스캔 → 서비스 연결 랜딩 페이지
  - **데이터**: `book_data.ts` — 10개 언어 × 2종(학생/교사) ISBN 20개 하드코딩, `findBookByISBN()`, `formatISBN()`
  - **UI 구조**: 3섹션 (Hero+CTA, 서비스안내+다른언어 pill, 하단CTA), 인증 상태별 CTA 분기
  - **SEO**: PageMeta `titleParams`/`descriptionParams` 동적 보간 확장 (하위 호환)
  - **i18n**: `changeLanguage(book.i18nCode)` 자동 전환, Filipino(`tl`) → `en` fallback, `zh_cn` → `zh-CN` 매핑
  - **국기 SVG**: `frontend/public/flags/` 10개 복사 (amazing-korean-books 소스)
  - **i18n 키**: `ko.json`/`en.json`에 `seo.book` + `book` 네임스페이스 18개 키 추가

- **2026-03-18 — Paddle Live 전환 + E-book Paddle Checkout 연동**
  - **환불 웹훅**: `adjustment.created`/`adjustment.updated` 이벤트 핸들러 추가 (e-book + 구독 transaction 환불 처리)
  - **E-book 가격 분기**: Paddle = $10 USD, 계좌이체 = ₩12,000~₩15,000 KRW (에디션별)
  - **Catalog API 확장**: `paddle_ebook_price_id`, `client_token`, `sandbox`, `paddle_price_usd` 필드 추가
  - **구매 취소 API**: `DELETE /ebook/purchase/{code}` (pending soft delete)
  - **프론트 Paddle Checkout**: 카탈로그에서 Paddle overlay 호출, `/ebook/my`에서 pending 재결제/취소 버튼
  - **Config**: `PADDLE_PRICE_EBOOK` 환경변수 추가 (`config.rs` + `docker-compose.prod.yml`)
  - **use_paddle 훅 확장**: `openEbookCheckout()`, `onCheckoutComplete` 콜백
  - **자동 refetch**: pending+paddle 구매 시 5초 interval로 상태 자동 갱신
  - **pwCustomer (Go-Live)**: `Paddle.Initialize()`에 `pwCustomer: { email }` 전달 + `Paddle.Update()` 후속 업데이트 — Retain 동작 조건
  - **Deploy 가이드 확장**: Dashboard 전체 설정 목록 추가 (Balance Currency, Payout, Sales Tax, Default Payment Link, Payment Methods, Retain, Discount)
  - **Retain 홈페이지**: `home_page.tsx`에서 Paddle.js 초기화 (결제 실패 시 인앱 알림 표시용)
  - **이메일 인프라**: Cloudflare Email Routing (`support@amazingkorean.net` → Gmail), SPF 레코드 병합
  - **환경변수 정리**: `PADDLE_PRODUCT_ID` 전체 제거 (코드 미사용), `PADDLE_EBOOK_PRICE` → `PADDLE_PRICE_EBOOK` 통일
  - **가격 정가 전환**: 3개월 $25→$30, 6개월 $50→$60, 12개월 $100→$120 (Paddle Discount로 할인 적용)
  - **Paddle Discount 연동**: `config.rs`에 `PADDLE_DISCOUNT_MONTH_3/6/12` 추가, Plans API에 `discount_id` 포함, `openCheckout()`에서 `discountId` 자동 적용 (체크아웃에서 ~~$120~~ $100 표시)

- **2026-03-18 — Gemini 코드 리뷰 반영 (PR #133~#138)**
  - **코드**: `auth/service.rs` Redis DEL if/else 분기 → 단일 경로 간결화
  - **SEO**: `index.html` social crawler fallback meta 태그 추가 (title, description, canonical, og:*, twitter:*)
  - **문서 오류 수정**:
    - `AMK_API_EBOOK.md`: Cache-Control `no-store` → `private, max-age=300` (코드와 일치), purchase_code 포맷 수정, 워터마크 "비가시적" → "다층 (1 가시 + 3 비가시)"
    - `AMK_API_AUTH.md`: 시나리오 범위 `5.3-6` → `5.3-13`
    - `AMK_API_FUTURE.md`: 깨진 TOC 앵커 수정
    - `AMK_API_LEARNING.md`: phase 번호 `4-2/4-3/4-4` → `5-3/5-4/5-5`
  - **스킵 (근거 포함)**: IIFE 리팩터링 (과도한 추상화), react-helmet-async (React 19 네이티브), canonical URL 환경 분기 (SEO 정석 위반), Cargo.lock 중복 (transitive dependency 정상)

- **2026-03-09 — E-book 웹 뷰어 시스템 (Phase 12.5) ✅**
  - **핵심 설계**: 회원 전용 (로그인 필수), 웹 전용 (다운로드 없음), 3중 보안 아키텍처
  - **DB 마이그레이션** (`20260310_ebook.sql`):
    - ENUM 3개: `ebook_edition_enum`, `ebook_purchase_status_enum`, `ebook_payment_method_enum`
    - 테이블 3개: `ebook_purchase` (구매), `ebook_access_log` (감사), `admin_ebook_log` (관리자)
  - **백엔드 (Public API)**: `GET /ebook/catalog` (카탈로그), `POST /ebook/purchase` (구매, AuthUser+IP Rate Limit), `GET /ebook/my` (내 구매), `GET /ebook/viewer/{code}/meta` (뷰어 메타), `GET /ebook/viewer/{code}/pages/{num}` (워터마크 페이지 이미지)
  - **백엔드 (Admin API)**: `GET /admin/ebook/purchases` (목록), `GET /admin/ebook/purchases/{id}` (상세), `PATCH /admin/ebook/purchases/{id}/status` (상태 변경), `DELETE /admin/ebook/purchases/{id}` (삭제)
  - **보안**: 2중 워터마크 (가시적 대각선 텍스트 + LSB 스테가노그래피), Redis Rate Limit (30페이지/분/user), 브라우저 보호 (우클릭/인쇄/드래그 차단), blob:// URL, Cache-Control: no-store
  - **Paddle 연동**: `transaction.completed` 웹훅에서 `custom_data.type == "ebook"` 분기 → 구매 완료 처리
  - **프론트엔드**: 카탈로그 (`/ebook`), 웹 뷰어 (`/ebook/viewer/:code`), 내 구매 (`/ebook/my`), 관리자 목록+상세 (`/admin/ebook/purchases`)
  - **이미지 처리**: `image` + `imageproc` + `ab_glyph` Rust 크레이트 (OnceLock 폰트 로딩)

- **2026-03-03 — 교재 주문 시스템 개선 (Textbook Order System Improvements)**
  - **DB 마이그레이션** (`20260303_textbook_improvements.sql`):
    - Soft Delete 지원: `is_deleted`, `deleted_at` 컬럼 추가
    - 배송 추적: `tracking_number`, `tracking_provider` 컬럼 추가
    - FK 제약조건: `admin_textbook_log.order_id` CASCADE → RESTRICT (감사 로그 보존)
    - DB 레벨 CHECK: 세금계산서 요청 시 사업자등록번호 필수
    - 인덱스 추가: `(status, created_at)`, `orderer_email`, `is_deleted`
  - **백엔드 개선**:
    - 상태 머신 검증 (유효 전환만 허용, shipped 전환 시 추적정보 필수)
    - Advisory Lock 개선: `pg_try_advisory_xact_lock` → `pg_advisory_xact_lock` (blocking 방식으로 중복 주문번호 완전 방지)
    - IP 기반 Rate Limiting (Redis INCR, 기본 5회/시간)
    - N+1 쿼리 해결 (`find_items_by_orders` 배치 조회)
    - ILIKE 검색 특수문자 이스케이프 (`%`, `_`, `\`)
    - 중복 항목 검증 (같은 언어+유형 조합 거부)
    - 언어 가용성 검증 (비활성 언어 주문 차단)
    - Validate() 호출 (이메일, 길이 등 DTO 검증)
    - 페이지네이션 범위 제한 (page ≥ 1, per_page 1~100)
    - `TextbookLanguage`, `TextbookType`에 Hash derive 추가
  - **신규 API**: `PATCH /admin/textbook/orders/{id}/tracking` (배송 추적 정보 업데이트)
  - **이메일 알림**:
    - 주문 접수 확인 이메일 (`TextbookOrderConfirmation`)
    - 상태 변경 알림 이메일 (`TextbookOrderStatusUpdate`) — 발송 시 운송장번호 포함
  - **프론트엔드 개선**:
    - 주문 폼: 약관 동의 모달 추가 (6개 조항 — 주문 제출 전 필수 동의)
    - 주문 폼: 중복 항목 방지 (동일 언어+유형 Select 비활성화 + 제출 시 검증)
    - 주문 폼: 세금계산서 이메일 Zod `.email()` 검증 추가, 수량 최대값(9999) 제한
    - 주문 폼: 다크모드 색상 개선 (`bg-primary/5` → `bg-muted/50`, `bg-secondary` → `bg-muted/50`)
    - 관리자: 유효 상태 전환만 표시 (State Machine UI), 배송 추적 입력/수정 UI
    - 관리자: 페이지네이션 동적 페이지 범위 (현재 페이지 기준 5개 표시)
  - **i18n**: 약관 6개 조항 (ko/en) + 추적 관련 8개 키 + 중복 에러 + 다음 상태 선택 키 추가

- **2026-02-26 — 교재 주문 시스템 구현 (Textbook Order System)**
  - **DB 마이그레이션**: ENUM 4개 (`textbook_language_enum`, `textbook_type_enum`, `textbook_order_status_enum`, `textbook_payment_method_enum`) + 테이블 3개 (`textbook`, `textbook_item`, `admin_textbook_log`)
  - **백엔드 (Public API)**: `GET /textbook/catalog` (카탈로그), `POST /textbook/orders` (주문 생성), `GET /textbook/orders/{code}` (주문 조회) — 인증 불필요
  - **백엔드 (Admin API)**: `GET /admin/textbook/orders` (목록/필터/검색/페이지네이션), `GET /admin/textbook/orders/{id}` (상세), `PATCH /admin/textbook/orders/{id}/status` (상태 변경), `DELETE /admin/textbook/orders/{id}` (삭제)
  - **프론트엔드 (Public)**: 교재 주문 페이지 (`/textbook`), 주문 조회 페이지 (`/textbook/order/{code}`)
  - **프론트엔드 (Admin)**: 주문 목록 (`/admin/textbook/orders`), 주문 상세+상태변경 (`/admin/textbook/orders/{id}`), 견적서/주문확인서 인쇄 (`/admin/textbook/orders/{id}/print`)
  - **i18n**: `ko.json`, `en.json`에 `textbook`, `admin.textbook`, `seo.textbook` 키 추가
  - **비즈니스 규칙**: 비회원 주문 가능, 계좌이체 전용, 20개 언어 × 2종(학생용/교사용), ₩25,000/권, 최소 10권
  - **주문번호 형식**: `TB-YYMMDD-NNNN` (일별 순번)

- **2026-02-26 — Google Search Console SEO 수정 (PageMeta 컴포넌트)**
  - **문제**: `index.html`에 하드코딩된 `<link rel="canonical" href="/">` 때문에 SPA 모든 페이지가 `/`의 중복으로 인식되어 Google 색인 제외
  - **해결**: React 19 네이티브 metadata 호이스팅을 활용한 `PageMeta` 컴포넌트 구현
    - `frontend/src/components/page_meta.tsx` 신규 — 페이지별 동적 `<title>`, `<link rel="canonical">`, `<meta>` (description, OG, Twitter) 태그 관리
    - `index.html`에서 PageMeta와 중복되는 정적 태그 제거 (title, canonical, description, og:title/description/url/locale, twitter:title/description)
    - `index.html`에 정적 태그만 유지 (keywords, og:type/site_name/image/locale:alternate, twitter:card/image)
  - **i18n SEO 키 추가**: `ko.json`, `en.json`에 `seo` 섹션 (14개 페이지 × title + description)
  - **적용 페이지 (14개)**: `/`, `/about`, `/pricing`, `/videos`, `/studies`, `/lessons`, `/login`, `/signup`, `/find-id`, `/request-reset-password`, `/terms`, `/privacy`, `/refund-policy`, `/faq`
  - **기타**: `.gitignore` 교재 관련 파일 제외 추가 (scripts/, docs/pdf_check/, /node_modules/, .mcp.json)
  - **검증**: `npm run build` 통과 + 로컬 dev 서버에서 페이지별 canonical 동적 변경 확인

- **2026-02-20 — Gemini 코드 리뷰 반영 (PR #128~#132)**
  - **PR #128 — Redis DEL 배치 최적화** (`src/api/auth/service.rs`)
    - 세션 무효화 시 Redis DEL 루프(N+1) → 키 수집 후 단일 DEL 호출(1회)로 변경
  - **PR #129 — noscript lang 속성 추가** (`frontend/index.html`)
    - `<noscript>` 내부 `<div>`에 `lang="en"` 추가 (HTML lang 불일치 해소)
  - **PR #130 — 미사용 코드 정리**
    - `ListStatsBar`: 미사용 `total` prop 제거 + 호출처 3곳 정리 (video/study/lesson list pages)
    - `SkeletonGrid`: 동일 코드 `VideoCardSkeleton`/`ContentCardSkeleton` → `CardWithImageSkeleton` 병합
  - **PR #131 — Translation Dashboard sticky 열 hover 수정** (`admin_translation_dashboard.tsx`)
    - `<tr>`에 `group` 클래스, sticky `<td>`에 `group-hover:bg-muted/50` 추가
  - **PR #132 — Translations 페이지 들여쓰기 + 성능 최적화** (`admin_translations_page.tsx`)
    - Filters 컨테이너 내부 div 들여쓰기 수정
    - `SUPPORTED_LANGUAGES.find()` 중복 호출(2회) → 변수 할당 1회로 최적화
  - **검증**: `cargo check` + `npm run build` 통과

- **2026-02-20 — Admin 리스트 페이지 디자인 통일 + Enum Badge 색상 체계 구축**
  - **테이블 스타일 통일 (8개 페이지, Translations 기준)**
    - Wrapper: `bg-card rounded-lg border overflow-hidden shadow-sm`
    - Thead: `border-b-2 bg-secondary`, Th: `font-semibold text-secondary-foreground`
    - Data cell: `px-4 py-3`, Hover: `hover:bg-accent/10`
    - 대상: Users, Videos, Lessons, Studies, Subscriptions, Transactions, Grants, Translations
  - **검색 폼 카드 래핑 (7개 페이지)**
    - `bg-card rounded-lg border border-foreground/15 p-4 shadow-sm` 래퍼 추가
    - Input 테두리 강화: `border-foreground/20`
    - 대상: Users, Videos, Lessons, Studies, Subscriptions, Transactions, Translations
  - **기본 정렬 ID 내림차순 (8개 페이지)**
    - 프론트엔드: Users/Videos/Subscriptions/Transactions → `id`, Lessons → `lesson_id`, Studies → `study_id`
    - 백엔드: Grants `ORDER BY uc.user_id DESC`, Translations `ORDER BY translation_id DESC`
  - **Enum Badge 색상 체계 — 8개 CSS 변수 + 6개 Badge variant 신규**
    - CSS 변수: `--badge-blue/orange/purple/yellow/sky/indigo` (light/dark 분리, 테마 토큰과 독립)
    - Badge variant: `blue`, `orange`, `purple`, `yellow`, `sky`, `indigo` 추가
    - State: open→`success`, ready→`warning`, close→`destructive`
    - Access: public→`success`, paid→`destructive`, private→`blue`, promote→`warning`
    - Subscription: active→`success`, trialing→`blue`, past_due→`warning`, paused→`orange`, canceled→`destructive`
    - Transaction: completed→`success`, refunded→`destructive`, partially_refunded→`warning`
    - User Role: HYMN→`purple`, admin→`orange`, manager→`info`, learner→`success`
    - Translation Status: draft→`destructive`, reviewed→`warning`, approved→`success`
    - Study Program: basic→`sky`(하늘색), topik→`indigo`(남색), tbc→`outline`
  - **Warning Badge 글씨색 흰색 변경**: `--warning-foreground: 0 0% 100%` (라이트/다크 모두)
  - **언어 컬럼 국기 이모지**: Translations 페이지 `<Badge>en</Badge>` → `🇺🇸 English` (emoji-flag + nativeName)
  - **Admin 레이아웃 ThemeToggle 추가**: 관리자 헤더에 라이트/다크 모드 전환 버튼
  - **검증**: `npm run build` 통과

- **2026-02-19 — Design System v2/v3 + 다크모드 + CEO 이름 통일**
  - **Design System v2 — 공유 컴포넌트 추출 (6개 신규)**
    - `lib/pagination.ts` (getPageItems + ELLIPSIS Symbol), `sections/pagination_bar.tsx` (PaginationBar), `sections/empty_state.tsx` (EmptyState), `sections/skeleton_grid.tsx` (SkeletonGrid), `sections/list_stats_bar.tsx` (ListStatsBar), `sections/stat_card.tsx` (StatCard)
    - `ui/card.tsx` CVA 확장: default/elevated/interactive variant (focus-visible:ring-2, active:translate-y-0, motion-reduce)
    - `sections/hero_section.tsx` variant prop: marketing/list 레이아웃 전환
    - 리스트 페이지 5개 + admin_dashboard.tsx 전면 교체 (Hero + Empty + Skeleton + PaginationBar + ListStatsBar)
  - **Design System v3 — 다크모드 구현**
    - CSS 변수 이중 정의 (`:root` + `.dark`) — 60+ 토큰 라이트/다크 분리
    - `next-themes` ThemeProvider 연결 (`attribute="class"`, `defaultTheme="system"`, `disableTransitionOnChange`)
    - `theme_toggle.tsx` 신규 — Sun/Moon 토글 + 드롭다운 (Light/Dark/System)
    - 전용 Surface 토큰: `--footer`, `--surface-inverted` — `--primary` 반전 문제 해결
    - 다크 shadow 오버라이드: `shadow-card`/`shadow-card-hover` → 검정 기반 (흰 글로우 방지)
    - 다크 text-gradient 오버라이드: `--secondary → --accent` (라이트: `--primary → --accent`)
    - Header/Footer + 공개 페이지 6개 + Admin 페이지 10개 하드코딩 색상 전면 교체
    - 22개 로케일 테마 i18n 키 추가 (toggleTheme, themeLight, themeDark, themeSystem)
  - **Design System v3 — UI/UX 가이드라인 문서화**
    - `AMK_DESIGN_SYSTEM.md` 대폭 확장: Radius Scale (6단계), Typography Scale (Heading/Weight/Line-height), Shadow Scale (6단계), Icon Sizing (6단계), Container Sizes, Button Variants & Sizes, Animation & Duration, Grid Gap Standard
    - Anti-Pattern 6개 추가 (Named colors, Footer/CTA, Radius/Shadow/Typography 혼용)
    - PR 체크리스트 5개 항목 추가 (Radius, Typography, Shadow, Icon, 다크모드)
  - **CEO 영문 이름 통일**
    - i18n 18개 로케일: `Kyungyun Kim` → `Kyoung Ryun KIM` (베트남어: `KIM Kyoung Ryun`)
    - noscript: `KIM KYEONGRYUN` 유지 (사업자등록증 영문본 기준)
  - **검증**: `npm run build` + `npm run lint:ui` 0건 + QA 124항목 122 PASS / 2 MANUAL
  - **프로덕션 배포 완료**

- **2026-02-19 — Paddle 사업자 인증 (KYB) 서류 제출**
  - Cloudflare 보안 설정 확인: Bot Fight Mode Off, Security Level Automated, I'm Under Attack 비활성화 — Paddle 크롤러 차단 요소 없음 확인
  - 사업자등록증 (한글 + 영문) Paddle Dashboard 업로드 완료
  - 주주명세서 한글 원본 (법인인감 날인) + 영문 번역본 PDF 생성 및 업로드 완료
    - 주민등록번호 마스킹 처리, UBO (25%+ 지분) 명시
    - Puppeteer + Noto Sans KR 폰트로 영문 PDF 자동 생성

- **2026-02-19 — Paddle 도메인 검토 대응**
  - **환불 정책 재작성** (Paddle Seller Handbook 준수)
    - 7일 → 30일 무조건 전액 환불 보장 (30-Day Money-Back Guarantee)
    - 조건부 문구 전면 제거: "부분 환불", "환불 대상 아님", "예외 사항" 삭제
    - 갱신 결제에도 동일 환불 정책 적용 (Paddle Checkout Buyer Terms 정합)
    - 섹션 5→4 축소 (en.json, ko.json, refund_policy_page.tsx)
  - **사업자명 통일** (Terms + Privacy)
    - "HIM Co., Ltd." / "㈜ 힘" 단독 표기 → "Amazing Korean (operated by HIM Co., Ltd.)" / "Amazing Korean(운영: ㈜ 힘)" 으로 Paddle 제출명과 일치
  - **SPA 크롤러 접근성 개선**
    - `index.html`에 `<noscript>` 블록 추가: 서비스 소개, 법적 페이지 링크 (Terms/Privacy/Refund/FAQ), 연락처, 사업자 정보
    - JS 미실행 크롤러(Paddle 도메인 검토 등)에 필수 정보 제공
  - **검증**: `npm run build` 통과

- **2026-02-18 — Gemini 코드 리뷰 반영 (PR #125~#127)**
  - **백엔드**
    - `docker-compose.prod.yml`: `PAYMENT_PROVIDER` 기본값 제거 → `:?` 구문으로 변경 (누락 시 docker-compose 레벨에서 명확한 에러 메시지와 함께 즉시 실패)
    - `src/api/auth/repo.rs`: `find_user_sessions_with_refresh_tx()` 배치 쿼리 함수 추가 (session_id + refresh_hash 단일 쿼리 조회)
    - `src/api/auth/service.rs`: `mfa_disable` 함수 N+1 쿼리 해소 — 루프 내 `find_login_by_session_id` 개별 호출 → 배치 쿼리 1회로 리팩토링
  - **프론트엔드 — Admin 차트 색상 토큰 전환**
    - `index.css` + `tailwind.config.js`: `--chart-6` (Purple, 280 65% 60%) 토큰 추가 (라이트/다크)
    - `admin_login_stats_page.tsx`: deviceColors `bg-blue-500`/`bg-purple-500` → `bg-chart-1`/`bg-chart-2`/`bg-chart-5`
    - `admin_study_stats_page.tsx`: programItems 7개 + programBadgeColors 7개 + stateItems 3개 하드코딩 색상 → chart 토큰 전환
    - `admin_user_stats_page.tsx`: roleTypes `bg-purple-500`/`bg-blue-500` → `bg-chart-6`/`bg-chart-3`/`bg-chart-2`
  - **문서 복원**
    - `AMK_API_MASTER.md` §7.1: RBAC 권한 매트릭스 테이블 인라인 복원 (Section 8 재구성 시 유실)
    - `AMK_API_MASTER.md` §8.2: 엔드포인트별 K6 성능 목표치 테이블 복원 (RPS + P95 응답시간)
  - **검증**: `cargo check` + `npm run build` + `npm run lint:ui` 0건 통과

- **2026-02-18 — 디자인 시스템 v1 구축 (전문가 2명 승인)**
  - **Foundation 토큰 구축**
    - `tailwind.config.js`: brand(soft/soft-alt), status(success/warning/info + foreground), spacing(section-sm/md/lg/hero-lg) 토큰 추가
    - `index.css`: HSL CSS 변수 정의 (라이트 + 다크모드), `bg-hero-gradient` 유틸리티, iOS input zoom 방지(`@supports`)
    - gradient-primary/text-gradient/shadow-card의 직접 HEX → `hsl(var())` CSS 변수 참조로 전환, 미사용 gradient-primary-hover 삭제
    - `badge.tsx`: shadcn Badge에 success/warning/info variant 확장 (CVA)
    - `package.json`: `lint:ui` 스크립트 추가 (bg/text/border/ring/from/via/to/stroke/fill 프리픽스 + 임의 HEX + 금지 팔레트 탐지)
  - **공통 컴포넌트 추출**
    - `section_container.tsx` 신규: 섹션 래퍼 (size sm/md/lg + container default/narrow + as prop)
    - `hero_section.tsx` 신규: Hero 블록 (badge/title/subtitle/size/children, decorative blobs, bg-hero-gradient)
  - **페이지 토큰 적용 (30파일)**
    - 사용자 페이지 14파일: home, about, pricing, lesson(list/detail), study(list/detail/task), video(list/detail), login, my_page, legal, footer → HeroSection/SectionContainer 적용 + semantic 토큰
    - Admin 페이지 16파일 + 기타 3파일: stats(study/user/login), translation(list/dashboard/edit), bulk-create(lesson/video/study/user), detail(study/lesson), study_create, vimeo_uploader, upgrade_join, mfa_setup, health_page → bg-green-*/text-red-* 등 51건 → status 토큰
  - **WCAG AA 명암비 보정 (전문가 피드백 반영)**
    - success `160 84% 39%` → `28%`, info `217 91% 60%` → `53%`, destructive `0 84% 60%` → `50%` (4.5:1+ 확보)
    - 다크모드 비례 보정: success `45%` → `36%`, info `65%` → `58%`
  - **section-md 반응형 제거**: `py-section-md lg:py-section-lg` → `py-section-md` 고정 (의도 없는 확대 방지)
  - **문서**: `docs/AMK_DESIGN_SYSTEM.md` 신규 (01 Foundations ~ 07 Roadmap, 7개 섹션)
  - **검증**: 빌드 성공 + `lint:ui` 0건 위반 (전체 .tsx)

- **2026-02-18 — Paddle Live 전환 준비 + 관리자 대시보드/결제 QA + 환경변수 동기화**
  - **Paddle Live 전환 준비**
    - AMK_API_MASTER.md §8.5 Paddle Live 전환 체크리스트 신규 작성 (대시보드 작업 9항목, GitHub Secrets 10개, 배포 검증, 가격 변경 전략, 결제수단/세금 참고)
    - Paddle 계정 인증(Account Verification) 신청 완료 — 심사 대기 중
  - **관리자 대시보드 실제 데이터 연동**
    - `admin_dashboard.tsx` 하드코딩 "-" → 4개 통계 API 호출 (`useUserStatsSummary`, `useLoginStatsSummary`, `useVideoStatsSummary`, `useStudyStatsSummary`)
    - 로딩 Skeleton + 에러 시 "-" fallback + `toLocaleString()` 숫자 포맷
  - **관리자 결제 4개 페이지 i18n**
    - `ko.json`/`en.json`에 `admin.dashboard.*` (8키) + `admin.payment.*` (82키) 추가
    - `admin_subscriptions_page.tsx`, `admin_transactions_page.tsx`, `admin_subscription_detail.tsx`, `admin_grants_page.tsx` — 전체 하드코딩 영어 → `t()` 교체
  - **환경변수 동기화**
    - `.env.example` 전면 업데이트: `config.rs` SSoT 기준 누락 변수 ~19개 추가 (Rate Limit, TTL, Payment, Translation 등)
    - `deploy.yml`에 `VIMEO_ACCESS_TOKEN` 환경변수 추가
  - **QA**: 44/44 PASS, 0 BUG (`docs/QA_DASHBOARD_I18N.md`)

- **2026-02-17 — Gemini 코드 리뷰 반영 + 문서 현행화**
  - PR #118~#124 코드 리뷰 유효 11건 일괄 반영: 보안 2건, 버그 1건, 코드 개선 5건, 문서 3건
  - **보안**: MFA 백업 코드 `constant_time_eq` 적용, MFA 비활성화 시 리프레시 토큰 정리
  - **버그**: 수동 수강권 만료일 쿼리 `MIN()` → `MAX()` 수정
  - **코드**: Paddle 상태 enum 직접 매치, `/admin/payment` 리다이렉트, invalidateWithDelay 즉시 호출 제거, useMemo 이중 반복 최적화, PAYMENT_PROVIDER 기본값 제거
  - **문서**: `provider_meta` → `provider_data`, `partially_refunded` 추가, UNIQUE 인덱스 표기
  - Section 8 "Open Questions & 설계 TODO" → **"작업 현황"** 전면 재구성
    - 8.1~8.11 (11개 서브섹션) → 8.1 완료 항목 (12행 표) + 8.2 진행 예정 (19행 표) + 8.3 세부 검토 (발음 AI)
    - 기존 8.7 서브섹션 (내부 DB, 외부 API, 보안, 하드닝, 다국어, 향후 계획) 모두 완료 표에 통합
    - 결제 시스템 ✅ 완료 반영 (Stripe → Paddle)
  - Section 9 변경 이력 → `AMK_CHANGELOG.md` 분리
  - Section 0.1 목적, Section 7 RBAC 참조 — 오래된 섹션명 수정

- **2026-02-16 — 결제 시스템 (Paddle Billing) 전체 구현 + 프로덕션 배포**
  - **데이터 모델**: Section 4.9 결제 도메인 추가 — 4 ENUMs + 3 Tables (subscriptions, transactions, webhook_events)
  - **외부 서비스**: Section 2.4.5 Paddle Billing 연동 추가
  - **Phase 11** (사용자 결제): `GET /payment/plans` (공개), `GET /payment/subscription` (인증), `POST /payment/webhook` (Paddle)
  - **Phase 10** (관리자 결제): 구독 CRUD 6개 + 수동 수강권 3개 = 총 9개 엔드포인트
  - **Webhook**: 8 subscription + 1 transaction 이벤트 처리, HMAC-SHA256 서명 검증, 멱등성 보장
  - **user_course 연동**: 구독 활성화 시 수강권 자동 부여, 취소 시 자동 회수
  - **프론트엔드**: Pricing 페이지 (Paddle.js overlay checkout), 프로모 코드 입력, 관리자 결제 관리 UI
  - **프로덕션 배포**: DB 마이그레이션 + Paddle Sandbox Webhook 연동 완료

- **2026-02-15 — 문서 정리 (코드-문서 동기화)**
  - Section 8.7 다국어 콘텐츠 확장: 항목 4,6,7,8,9 📋→✅ (Phase 1B/2/3 완료 반영)
  - Section 8.7 "향후 작업 계획" 통합 섹션 추가: 8.5 보안, 8.7 외부 API 분산 📋 항목을 한 곳으로 정리
  - Section 8.5 남은 항목 → 8.7 향후 작업 계획 참조로 통합
  - Section 8.9 다국어 UI 대응: 22개 언어 → 21개 언어 (아랍어 RTL 제외 확정 반영)

- **2026-02-14 — Admin MFA (TOTP 2단계 인증) 구현 + QA 완료**
  - **백엔드 (Rust/Axum)**
    - DB 마이그레이션: `users` 테이블에 MFA 컬럼 4개 추가 (`user_mfa_secret`, `user_mfa_enabled`, `user_mfa_backup_codes`, `user_mfa_enabled_at`)
    - `Cargo.toml`: `totp-rs = { version = "5", features = ["qr", "gen_secret"] }` 의존성 추가
    - `src/api/auth/dto.rs`: MFA DTO 7개 (MfaChallengeRes, MfaLoginReq, MfaSetupRes, MfaVerifySetupReq, MfaVerifySetupRes, MfaDisableReq, MfaDisableRes)
    - `src/api/auth/repo.rs`: `UserLoginInfo`에 `user_mfa_enabled` 추가 + MFA repo 함수 7개
    - `src/api/auth/service.rs`: `LoginOutcome`/`OAuthLoginOutcome` enum, `login()`/`google_auth_callback()` MFA 분기, MFA 메서드 4개 (mfa_setup, mfa_verify_setup, mfa_login, mfa_disable)
    - `src/api/auth/handler.rs`: MFA 핸들러 4개 + login/OAuth 핸들러 반환 타입 변경 (`impl IntoResponse`)
    - `src/api/auth/router.rs`: `/mfa/setup`, `/mfa/verify-setup`, `/mfa/login`, `/mfa/disable` 라우트 추가
    - `src/config.rs`: MFA 환경변수 3개 (MFA_TOKEN_TTL_SEC=300, RATE_LIMIT_MFA_MAX=5, RATE_LIMIT_MFA_WINDOW_SEC=300)
    - `src/api/user/dto.rs` + `repo.rs`: `ProfileRes`에 `mfa_enabled: bool` 추가
    - `src/docs.rs`: MFA 핸들러 4개 + DTO 7개 Swagger 등록
  - **프론트엔드 (React/TypeScript)**
    - `auth/types.ts`: MfaChallengeRes, MfaLoginReq(zod), MfaSetupRes, MfaVerifySetupRes
    - `auth/auth_api.ts`: mfaLogin, mfaSetup, mfaVerifySetup API 함수
    - `auth/hook/use_login.ts`: MFA 챌린지 감지 (`isMfaChallenge` 타입가드) + `mfaPending` 상태
    - `auth/hook/use_oauth_callback.ts`: OAuth MFA 리다이렉트 파라미터 처리
    - `auth/page/login_page.tsx`: MFA 코드 입력 UI (6~8자 TOTP/백업코드)
    - `admin/page/admin_mfa_setup_page.tsx`: 3단계 위저드 (QR스캔→코드확인→백업코드)
    - `routes/admin_route.tsx`: MFA 강제 설정 가드 (`!mfa_enabled` → `/admin/mfa/setup`)
    - `app/routes.tsx`: `/admin/mfa/setup` 라우트 추가 (AdminLayout 밖, AdminRoute 안)
    - `user/types.ts`: `mfa_enabled: z.boolean().optional()` 추가
    - i18n: MFA 관련 키 추가 (ko.json, en.json + 20개 언어)
  - **보안**
    - TOTP 비밀키: AES-256-GCM 암호화 (AAD: `users.user_mfa_secret`)
    - 백업 코드: SHA-256 해시 → JSON → AES-256-GCM 암호화
    - MFA 토큰: Redis UUID (5분 TTL, 일회용)
    - Rate Limit: `rl:mfa:{user_id}:{ip}` (5회/5분)
    - MFA 비활성화: HYMN 전용, 자기 자신 비활성화 불가, 대상 전체 세션 무효화
  - **QA (39/39 PASS)**
    - H-1 수정: `login_method: "login"` → `"email"` (login_method_enum 불일치)
    - M-1 수정: docs.rs에 MFA 핸들러/스키마 Swagger 등록 누락
  - **프로덕션 배포 완료** (2026-02-14)
    - DB 마이그레이션 수동 실행 (EC2 SSH → psql)
    - Admin/HYMN MFA 설정 정상 작동 확인

- **2026-02-10 — Phase 1A 다국어 인프라 + QA 수정 + 프로덕션 QA**
  - **Phase 1A 다국어 인프라 (백엔드)**
    - `content_translations` 테이블 + 21개 언어 enum (`SupportedLanguage`) 구현
    - Admin 번역 CRUD API 7개 엔드포인트 (목록/생성UPSERT/벌크/상세/수정/상태변경/삭제)
    - 기존 콘텐츠 API `?lang=` 확장: courses, lessons, videos, studies에 번역 fallback 주입
    - Fallback 순서: 사용자 언어 → en → ko (서비스 계층 post-fetch merge)
  - **Phase 1A QA 수정 (10개 이슈)**
    - H-1: Course `GET /courses/{id}` 번역 지원 — handler→service 리팩토링, `?lang=` 파라미터 추가
    - H-2: `ContentType::Video` 추가 — video title/subtitle 번역과 video_tag 번역 의미 분리, migration 추가
    - M-1: `CourseListItem`에 `course_subtitle` 필드 추가 + 번역 주입
    - M-2: Course DTO OpenAPI 스키마 등록 (`IntoParams`, `ToSchema` derive)
    - M-3: UPSERT 정책 개선 — 텍스트 변경 시에만 `status='draft'` 리셋 (SQL CASE 조건)
    - L-1~L-5: `CourseListQuery` derive 추가, Video DTO import 정리
  - **프로덕션 QA 수정 (PROD-4 ~ PROD-8)**
    - PROD-4: API 보안 헤더 미들웨어 추가 (`main.rs`) — `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `X-XSS-Protection: 0`, `Permissions-Policy: camera=(), microphone=(), geolocation=()`
    - PROD-5: Health `version` 필드 프로덕션 숨김 — `Option<String>` + `skip_serializing_if`, `APP_ENV=production`이면 None
    - PROD-6: OpenAPI Swagger UI 프로덕션 비활성화 — `enable_docs` config에 따라 조건부 merge
    - PROD-7: Guard 401/403 JSON 통일 — `ip_guard.rs`, `role_guard.rs` plain text → `AppError::Forbidden/Unauthorized` JSON 응답
    - PROD-8: 404 Fallback 핸들러 추가 — 존재하지 않는 라우트에 JSON `AppError::NotFound` 반환
  - **파일 변경 목록**
    - `src/main.rs` — `security_headers` 미들웨어 함수 추가 + 레이어 적용
    - `src/api/mod.rs` — 조건부 SwaggerUi merge + `fallback_404` 핸들러
    - `src/api/health/handler.rs`, `dto.rs` — version `Option<String>`, 프로덕션 숨김
    - `src/api/admin/ip_guard.rs` — `AppError::Forbidden` JSON 응답
    - `src/api/admin/role_guard.rs` — `AppError::Unauthorized/Forbidden` JSON 응답
    - `src/api/course/` — dto.rs, repo.rs, service.rs, handler.rs (H-1, M-1, M-2, L-1)
    - `src/api/video/service.rs` — `ContentType::Video` 적용 (H-2)
    - `src/api/video/dto.rs` — import 정리 (L-5)
    - `src/types.rs` — `ContentType::Video` 추가 (H-2)
    - `src/api/admin/translation/repo.rs` — UPSERT 조건부 status 리셋 (M-3)
    - `src/docs.rs` — Course DTO 스키마 등록 (M-2)
    - `migrations/20260210_i18n_add_video_content_type.sql` — 신규

- **2026-02-09 — 이메일 인증 + 계정 복구 + Rate Limiting 강화**
  - **이메일 인증 시스템**
    - 회원가입 → 인증코드 발송 → 검증 → 로그인 가능 플로우 구현
    - `POST /auth/verify-email` (3-7): HMAC-SHA256 해시 비교, `user_check_email=true` 업데이트
    - `POST /auth/resend-verification` (3-8): Enumeration Safe, 잔여 횟수 반환
    - 로그인 시 `user_check_email=false` → **403** 차단 (`AUTH_403_EMAIL_NOT_VERIFIED:email`)
    - OAuth 자동 인증: 미인증 이메일로 OAuth 로그인 시 `user_check_email=true` 자동 업데이트
    - Redis 저장: HMAC-SHA256 해시 (평문 코드 저장 금지), TTL 10분
    - 프로덕션 fail-fast: `EMAIL_PROVIDER=none` + `APP_ENV=production` → 서버 부팅 실패
    - EmailSender trait: Resend (`src/external/email.rs`)
  - **계정 복구 (아이디/비밀번호 찾기) 통합**
    - `POST /auth/find-password` (3-9): 본인확인(이름+생일+이메일) → 인증코드 발송
    - `/account-recovery` 페이지: 탭 UI (아이디 찾기 / 비밀번호 찾기)
    - OAuth 전용 계정 경고 문구 (warning 스타일, 비밀번호 찾기 탭)
  - **Rate Limiting 강화**
    - 이메일 발송 제한: 5회/1시간 → 5회/5시간 (환경변수 조정 가능)
    - 환경변수: `RATE_LIMIT_EMAIL_WINDOW_SEC` (기본 18000초), `RATE_LIMIT_EMAIL_MAX` (기본 5)
    - 응답에 `remaining_attempts` 필드 추가 (FindPasswordRes, RequestResetRes, ResendVerificationRes)
    - 프론트: 잔여 발송 횟수 표시 + 한도 도달 시 재전송 버튼 비활성화
  - **프론트엔드 변경**
    - `verify_email_page.tsx` 신규 — 이메일 인증코드 확인 페이지
    - `account_recovery_page.tsx` 신규 — 아이디/비밀번호 찾기 통합 (Tabs)
    - `signup_page.tsx` — 가입 성공 시 `/verify-email`로 이동
    - `use_login.ts` — 403 이메일 미인증 시 `/verify-email`로 이동
    - i18n: 이메일 인증, 계정 복구, Rate Limiting 관련 키 추가 (ko.json, en.json)

- **2026-02-08 — 프로덕션 클린 배포 (DB 보안 Phase 2D+3 반영)**
  - **마이그레이션 통합**
    - 기존 11개 마이그레이션 파일 → 단일 `20260208_AMK_V1.sql` 통합 (22 ENUMs, 35 Tables, FKs, Indexes)
    - 암호화 컬럼 직접 포함 (`user_email` TEXT, `user_email_idx` TEXT 등), `ip_address` INET→TEXT 반영
  - **시드 데이터**
    - `20260208_AMK_V1_SEED.sql` 생성 (콘텐츠 10개 테이블, ~200행)
    - 컬럼 순서 불일치 수정: `lesson`, `video`, `study` 테이블에 명시적 컬럼명 추가
  - **Dockerfile 수정**
    - 멀티바이너리 빌드 지원 (`amazing-korean-api` + `rekey_encryption`)
    - `--bin` 플래그로 개별 바이너리 빌드
  - **docker-compose.prod.yml 환경변수 추가**
    - `ENCRYPTION_KEY_V1`, `ENCRYPTION_CURRENT_VERSION`, `HMAC_KEY`, `APP_ENV`
    - `GOOGLE_CLIENT_ID/SECRET`, `GOOGLE_REDIRECT_URI`, `OAUTH_STATE_TTL_SEC`
    - `FRONTEND_URL`, `ADMIN_IP_ALLOWLIST`
  - **EC2 배포 완료**
    - DB 볼륨 삭제 → 스키마 마이그레이션 → 시드 데이터 투입 → 전체 서비스 시작
    - `.env.prod` 완전 구성 (프로덕션 전용 암호화 키 생성)
    - Google OAuth redirect URI 프로덕션 설정 (`https://api.amazingkorean.net/auth/google/callback`)
  - **배포 검증 완료**
    - healthz: `{"status":"live","version":"v1.0.0"}`
    - DB 암호화 확인: `user_email` = `enc:v1:...` 형태 정상 저장
    - 시드 데이터: video=16, lesson=8 정상
  - **문서 업데이트**
    - Section 8.7: 프로덕션 클린 배포 항목 추가, 이메일 인증 상태 변경 (📋→보류)
    - `AMK_DEPLOY_OPS.md`: .env.prod 전체 변수 목록, 클린 배포 절차, 트러블슈팅 추가

- **2026-02-08 — 문서 구조 재편 (3파일 분할 + 불일치 수정)**
  - **구조 변경**
    - `AMK_API_MASTER.md` 단일 파일(8,100줄) → 3파일 분할(MASTER ~3,700줄 + CODE_PATTERNS ~4,000줄 + DEPLOY_OPS ~620줄)
    - `AMK_CODE_PATTERNS.md` 신규 — 기존 Section 7.7 코드 예시 전체 이동
    - `AMK_DEPLOY_OPS.md` 신규 — 기존 Section 6.6.2~6.6.4 배포/운영 가이드 + Phase 8 운영 도구 통합
    - `docs/patchs/` → `docs/archive/patchs/` 아카이브 이동
  - **삭제 항목**
    - Section 0.4 (웹 LLM 협업 가이드 90줄) → 5줄 AI 에이전트 규칙으로 대체
    - Section 8 (LLM 협업 규칙 74줄) 전체 삭제
    - Phase 8 (scripts 테이블) 삭제 → Course Phase로 대체
  - **불일치 수정 23건 (Section 2~5)**
    - Section 2: `src/api/docs.rs` → `src/docs.rs`, 암호화 모듈 추가, EmailTemplate 4종, Vimeo 경로 명시
    - Section 3: 액세스 토큰 TTL 1시간 → 15분, 리프레시 토큰 역할별 분리 명시
    - Section 4: 암호화 컬럼(`_enc`, `_idx`) 반영, `ip_address` INET→TEXT, Course 도메인 추가, `user_oauth` 테이블 추가
    - Section 5: Auth 라우트 3개 추가, Course 엔드포인트 3개 추가, Admin email/stats 엔드포인트 추가
  - **섹션 번호 재구성**
    - Section 9 (Open Questions) → Section 8
    - Section 10 (변경 이력) → Section 9
    - Section 6.6 "빌드/배포" → "로컬 개발" (배포 내용 DEPLOY_OPS 이관)
  - **기타**
    - Section 7.2 개발 플로우: Gemini 템플릿 단계 제거, CODE_PATTERNS 참조 추가
    - Section 0.3 관련 파일 목록 갱신 (CODE_PATTERNS, DEPLOY_OPS 추가)
    - 교차 참조 정리 (분할 파일 참조 업데이트)
    - 목차(TOC) 전면 갱신 + 앵커 링크 검증

- **2026-02-06 — Gemini 코드 리뷰 반영**
  - **백엔드 — 코드 수정 (8건)**
    - `google.rs`: ID Token 서명 검증을 Google JWKS 공개키 기반으로 변경 (RS256, kid 매칭)
    - `ipgeo.rs`: `lookup()` 반환 타입 `Option<GeoLocation>` → `GeoLocation`, `is_private_ip()`를 `std::net::IpAddr` 파싱으로 개선
    - `auth/service.rs`: 이메일 미설정 시 `AppError::ServiceUnavailable` 반환, 인라인 Argon2 해싱 → `password::hash_password()` 통합, 실패 로깅 `let _ =` → `if let Err(e)` + `warn!`
    - `admin/upgrade/service.rs`: 로컬 `hash_password()` 제거 → `password::hash_password()` 사용, 이메일 미설정 시 `ServiceUnavailable` 반환
    - `lesson/repo.rs`: DB 에러 `.unwrap_or(false)` → `?` 전파
    - `user/service.rs`: ipgeo `.unwrap_or_default()` 제거
  - **문서 정리**
    - Section 8.5/9.7에 추후 작업 항목 5건 추가 (토큰 캐싱, GeoIP 전환, i18n async, OAuth 중복 통합, enum 매핑)
    - 불일치 문서 4건 삭제: `AMK_BACKEND_STATUS.md`, `AMK_FRONTEND_STATUS.md`, `homepage_layout_design.md`, `login_table_plan.md`
    - `.gitignore`에 `.aws/` 추가
    - Section 5.3-1 소셜 전용 계정 에러 응답 형식 수정

- **2026-02-05 — Login/Login_log 테이블 개선**
  - **백엔드 — User-Agent 서버사이드 파싱**
    - `woothee` 라이브러리 추가, `ParsedUa` 구조체 및 `parse_user_agent()` 함수 구현
    - `login_os`, `login_browser`, `login_device`를 서버에서 자동 채움 (프론트엔드 전송 제거)
    - OAuth/일반 로그인/회원가입 모두 동일하게 처리
  - **백엔드 — login 테이블 컬럼 활성화**
    - `login_expire_at`: `NOW() + refresh_ttl_secs` 기록, 토큰 갱신 시 갱신
    - `login_active_at`: 토큰 갱신(refresh) 시 `NOW()` 업데이트
    - `login_revoked_reason`: 상태 변경 시 사유 기록 (기본값 `none`, revoke 시 `password_changed`/`security_concern` 등)
  - **백엔드 — login_log 테이블 감사 컬럼 활성화**
    - `login_access_log`: access token SHA-256 해시 (64자)
    - `login_token_id_log`: JWT `jti` claim (UUID v4)
    - `login_fail_reason_log`: 실패 사유 (기본값 `none`, 실패 시 `invalid_credentials`/`account_disabled`/`token_reuse`)
    - `login_expire_at_log`: 세션 만료 시각 기록
    - login_log geo 컬럼에 COALESCE 기본값 추가 (`LC`/`0`/`local`)
  - **백엔드 — JWT jti claim 추가**
    - `jwt::create_token()`에서 UUID v4 기반 `jti` 생성, `Claims` 구조체에 `jti` 필드 추가
  - **백엔드 — Geo/NULL 기본값 정책 변경**
    - Private IP 기본값: `ZZ`→`LC`, `Unknown`→`local` (login/login_log 모든 COALESCE)
    - `login_revoked_reason` NULL→`none`, `login_fail_reason_log` NULL→`none`
  - **프론트엔드 — 버그 수정**
    - `client.ts`: request interceptor 추가 (zustand → axios Authorization 헤더 자동 설정)
    - `use_user_settings.ts`: `enabled` 옵션 + `staleTime: 5분` 추가 (미로그인 시 401 루프 방지)
    - `use_language_sync.ts`: `{ enabled: isLoggedIn }` 전달
    - `types.ts`: `LoginReq`에서 불필요 필드(`device`/`browser`/`os`) 제거
  - **파일 변경 목록**
    - `Cargo.toml` — `woothee` 의존성 추가
    - `src/api/auth/handler.rs` — `ParsedUa`, `parse_user_agent()` 추가
    - `src/api/auth/dto.rs` — `LoginReq` 간소화
    - `src/api/auth/jwt.rs` — `jti` claim 추가
    - `src/api/auth/repo.rs` — INSERT/UPDATE 쿼리에 신규 컬럼 반영, COALESCE 기본값 변경
    - `src/api/auth/service.rs` — UA/geo/audit 파라미터 전달, revoked_reason/fail_reason 기본값
    - `src/api/user/handler.rs` — UA 파싱 호출
    - `src/api/user/service.rs` — 회원가입 로그에 audit 파라미터 추가
    - `frontend/src/api/client.ts` — request interceptor 추가
    - `frontend/src/category/auth/types.ts` — LoginReq 필드 제거
    - `frontend/src/category/user/hook/use_user_settings.ts` — enabled/staleTime 추가
    - `frontend/src/hooks/use_language_sync.ts` — enabled 조건 추가

- **2026-02-05 — DB 보안 강화 계획 수립**
  - 애플리케이션 레벨 AES-256-GCM 암호화 방식 결정 (pgcrypto, AWS KMS 비교 후)
  - 암호화 대상 필드 식별: `user_email`, `user_name`, `user_birthday`, `oauth_email`, `oauth_subject`, `login_ip` 등
  - Blind Index (HMAC-SHA256) 설계: 검색 필요 필드(email, oauth_subject)는 같은 테이블에 `_idx` 컬럼 추가
  - 키 관리: `ENCRYPTION_KEY` + `HMAC_KEY` (환경변수, 각 32바이트)
  - 마이그레이션 전략: 3단계 점진적 (호환 모드 → 일괄 암호화 → 정리)
  - 보안 로드맵: 1단계 앱 레벨 AES → 2단계 AWS KMS → 3단계 HSM
  - Section 8.7 로드맵에 "보안 & 데이터 보호" 섹션 추가

- **2026-02-05 — 다국어 콘텐츠 확장 계획 수립**
  - 22개 언어 지원 계획: en, zh-CN, zh-TW, ja, vi, id, th, my, km, mn, ru, uz, kk, tg, ne, si, hi, es, pt, fr, de, ar
  - `content_translations` 번역 테이블 설계 (정규화, fallback 패턴)
  - 폰트 전략: Noto Sans 패밀리 동적 로딩 (50MB+ → 언어별 선택 로드)
  - RTL 대응 (아랍어): CSS Logical Properties, direction: rtl
  - 번역 파이프라인: AI 자동 초안 → 관리자 검수 → 승인
  - 단계적 접근: Phase 1 기반 → Phase 2 핵심 5개(en,ja,zh-CN,zh-TW,vi) → Phase 3 나머지 17개
  - Section 8.7 로드맵에 "다국어 콘텐츠 확장" 섹션 추가, Section 8.9에 다국어 UI 대응 추가

- **2026-02-05 — 다국어 지원 (i18n) 구현**
  - 상세: Section 6.2.4 참조

- **2026-02-03 — MyPage UI 리디자인 & 비밀번호 재설정 플로우**
  - **백엔드**
    - `ProfileRes`에 `has_password: bool` 필드 추가 (OAuth 전용 계정 구분)
    - `GET /users/me`, `POST /users/me` 응답에 `has_password` 포함
  - **프론트엔드**
    - MyPage UI 리디자인
      - 프로필 헤더: 닉네임 + user_auth 뱃지만 표시
      - 보기 모드 필드 순서: 닉네임 → 이름 → 이메일 → 가입일 → 생년월일 → 언어 → 국가 → 성별
      - 환경 설정 버튼을 수정 버튼 옆으로 이동
      - 비밀번호 재설정 버튼 추가 (OAuth 전용 계정은 숨김)
    - `/request-reset-password` 페이지 생성 (PrivateRoute 보호)
      - 로그인 사용자 이메일 자동 채우기
      - OAuth 전용 계정 접근 시 마이페이지로 리다이렉트
      - 이메일 입력 → 인증번호 전송 → 인증번호 확인 UI (백엔드 API 연동 대기)
    - 환경 설정 페이지에 마이페이지 돌아가기 링크 추가
    - `UserDetail` 타입에 `has_password: boolean` 추가
  - **문서**
    - Section 7.7.1-1 ProfileRes 코드 예시 업데이트

- **2026-02-03 — Google OAuth 소셜 로그인 구현**
  - **백엔드**
    - `GET /auth/google` — OAuth 시작 (auth_url 반환)
    - `GET /auth/google/callback` — OAuth 콜백 처리 (토큰 발급, 프론트엔드 리다이렉트)
    - `src/external/google.rs` — Google OAuth 클라이언트 구현
    - `migrations/20260203_ADD_OAUTH_SUPPORT.sql` — `user_oauth` 테이블 추가, `users.user_password` NULL 허용
  - **프론트엔드**
    - 로그인 페이지에 "Google로 로그인" 버튼 추가
    - `use_google_login.ts` 훅 생성
    - OAuth 콜백 처리 (refreshToken 호출 → 스토어 업데이트)
  - **문서**
    - Section 5.3 Phase 3 auth에 3-6 Google OAuth 엔드포인트 추가
    - Section 8.7 외부 API 연결 로드맵 업데이트

- **2025-11-18**
  - `AMK_Feature_Roadmap.md`, `AMK_PROJECT_JOURNAL.md`, `AMK_ENGINEERING_GUIDE.md`, `AMK_API_OVERVIEW_FULL.md`, `README_for_assistant.md`의 핵심 내용을 통합.
  - 이 문서(`AMK_API_MASTER.md`)를 프로젝트의 단일 기준 문서로 지정.
- **2026-01-21**
  - Section 0.4 "LLM 협업 가이드" 추가 (LLM 활용 프롬프트 템플릿 및 참조 방법)
  - Section 3.7 "인증 & 세션 관리 (통합)" 추가 (산재된 인증 관련 내용 통합)
  - Section 5.0 "Phase 로드맵 체크박스 범례" 추가 (✅🆗⚠️❌🔄 의미 명확화)
  - 문서 전체 목차(TOC) 추가 및 양방향 링크 구현 (각 섹션 시작/끝에 "목차로 돌아가기" 링크)
  - 외부 파일 참조 링크 업데이트 (AMK_SCHEMA_PATCHED.md, LLM_PATCHS_TEMPLATE_*.md)
- **2026-01-22**
  - Section 7.7.2 "프론트엔드 패턴" 실제 코드 기반으로 전면 재작성 (기존 LLM 분석 내용 제거)
  - Section 5 Phase 번호 체계 정리 (5.3 video → 5.4, 5.4 study → 5.5, 5.5 lesson → 5.6, 5.5.6 admin → 5.7, 5.7 scripts → 5.8)
  - 목차(TOC) 실제 섹션 헤딩과 동기화 (Section 6, 7, 8, 9 하위 항목 추가)
  - Section 8.6 "코드 일관성 (Technical Debt)" 추가
  - Section 8.7 "추후 작업 항목 (문서 내 TODO 통합)" 추가
- **2026-01-28 — Vimeo API 연동 & Admin Video 문서화**
  - **Vimeo API 연동 (Phase 5 & 6 계획 기반)**
    - `GET /admin/videos/vimeo/preview` — Vimeo 메타데이터 미리보기 (7-10)
    - `POST /admin/videos/vimeo/upload-ticket` — Vimeo tus 업로드 티켓 생성 (7-11)
    - `video` 테이블에 `video_duration`, `video_thumbnail` 컬럼 추가
  - **Admin Video 엔드포인트 정비**
    - `GET /admin/videos/{id}` 상세 조회 추가 (7-9)
    - Phase 7 엔드포인트 번호 재정렬 (7-8 ~ 7-57, 이후 Study Stats 추가로 7-67까지 확장)
  - **문서 업데이트**
    - Section 4.3 비디오 도메인에 신규 컬럼 명세 추가
    - Section 5.4 Phase 4 video에 응답 스키마 상세 추가 (VideoListItem, VideoDetailRes, VideoProgressRes)
    - Section 5.7 Phase 7 admin video 엔드포인트 목록 갱신
- **2026-01-26 — v1.0.0 MVP 릴리스**
  - **MVP 배포 완료**
    - Frontend: Cloudflare Pages (`amazingkorean.net`)
    - Backend: AWS EC2 (`api.amazingkorean.net`)
    - SSL: Cloudflare Flexible 모드
  - **GitHub Actions CI/CD 파이프라인 구축**
    - Section 6.6.2-3 "GitHub Actions CI/CD 파이프라인" 추가
    - EC2에서 빌드 불필요 → t2.micro 유지 가능
    - `git push`만으로 자동 배포
  - **배포 최적화**
    - `.dockerignore` 추가 (docs, frontend, .git 등 제외)
    - `docker-compose.prod.yml` Docker Hub 이미지 사용으로 변경
    - Section 6.6.2-4 "EC2 유지보수 가이드" 추가
  - **버전 관리**: Cargo.toml `version = "1.0.0"`, Git tag `v1.0.0` 생성
  - **Section 9 확장** (Open Questions & 설계 TODO)
    - Section 8.8 "LLM 협업 도구 전환" 추가 (Patch 템플릿 처리 + GitHub Gemini)
    - Section 8.9 "인프라 로드맵 (RDS 이전)" 추가 (이전 순서 및 시점 기준)
    - Section 8.10 "데이터 모니터링 & 접근" 추가 (SSH 터널, Admin 대시보드, 동기화)
    - 이후 변경 사항은 커밋 메시지 `docs: update AMK_API_MASTER <요약>` 형식으로 관리하고, 필요 시 이 섹션에 중요한 방향 전환만 추가한다.
- **2026-01-28 — User/Login Stats & TODO 정비**
  - **User/Login Stats 구현 (현재 7-63 ~ 7-67로 재번호)**
    - `GET /admin/users/stats/summary` — 역할별(HYMN/admin/manager/learner) 통계로 변경
    - `GET /admin/users/stats/signups` — 역할별 일별 가입 통계
    - `GET /admin/logins/stats/summary` — 로그인 성공/실패/고유사용자/활성세션
    - `GET /admin/logins/stats/daily` — 일별 로그인 통계
    - `GET /admin/logins/stats/devices` — 디바이스별 통계
  - **버그 수정**
    - Video 상세 조회 시 `video_state = 'open'` 필터 추가 (비공개 영상 직접 접근 차단)
  - **Section 9 TODO 업데이트**
    - Section 8.2 로그 테이블 역할별 구분 항목 추가
    - Section 8.7 기능 개발에 Admin 폼 검증, 영상 시청 시간, 토픽 정답 검사, 학습 문제 생성 추가
    - Section 8.11.2 에러 페이지 항목 추가
    - Section 8.12 "마케팅 & 데이터 분석" 신규 추가
- **2026-01-29 — Admin Study Stats & Phase 7 정비**
  - **Study Stats 구현 (7-42 ~ 7-44)**
    - `GET /admin/studies/stats/summary` — 총 학습수/Task수/시도수/해결수/해결률, Program별(basic_pronunciation/basic_word/basic_900/topik_read/topik_listen/topik_write/tbc)/State별(ready/open/close) 분포
    - `GET /admin/studies/stats/top` — TOP 학습 조회 (시도수/해결수/해결률 정렬, limit 1-50)
    - `GET /admin/studies/stats/daily` — 일별 시도수/해결수/활성사용자, 제로필
  - **Phase 7 엔드포인트 번호 재정렬 (7-1 ~ 7-67)**
    - 중복된 번호 수정 (7-23, 7-28 중복 해소)
    - `GET /admin/studies/{id}` (7-23), `GET /admin/studies/tasks/{id}` (7-29) 명확화
    - Study Stats 추가로 인한 후속 번호 조정 (Lessons: 7-45~7-62, User/Login Stats: 7-63~7-67)
  - **프론트엔드 Study Stats 페이지 구현**
    - `/admin/studies/stats` 라우트 추가
    - Summary Cards, Program/State 분포 차트, TOP Studies 테이블, Daily Stats 테이블
    - Studies 목록 페이지에 Stats 버튼 추가
- **2026-01-31 — Admin Lesson 프론트엔드 & Phase 7 Lesson 정비**
  - **Admin Lesson 프론트엔드 완성**
    - `/admin/lessons` — 목록 (검색/정렬/페이지네이션/벌크 수정)
    - `/admin/lessons/new` — 단건 생성
    - `/admin/lessons/bulk-create` — CSV 벌크 생성
    - `/admin/lessons/:lessonId` — 상세/수정 (Info/Items/Progress 탭)
  - **Lesson Items DELETE 엔드포인트 추가 (7-57, 7-58)**
    - `DELETE /admin/lessons/{id}/items/{seq}` — 수업 아이템 단건 삭제
    - `DELETE /admin/lessons/bulk/items` — 수업 아이템 다중 삭제
  - **Phase 7 엔드포인트 번호 재정렬 (7-45 ~ 7-67)**
    - Lessons: 7-45~7-62 (DELETE 추가로 +2)
    - User/Login Stats: 7-63~7-67 (기존 7-61~7-65에서 +2)
  - **Study Task 접근 제어 개선**
    - `study_state = 'open'` 필터 추가 (부모 Study가 닫히면 Task 접근 차단)
    - `find_task_detail`, `find_answer_key`, `get_try_count`, `find_task_explain`, `exists_task` 함수에 INNER JOIN study 추가
  - **Progress 수정 UI 구현**
    - Lesson Progress 탭에 단건/벌크 수정 다이얼로그 추가
    - Last Item Seq 필드에 max 제약 (lesson items 기준)
- **2026-02-02 — URL/함수명 통일 리팩토링**
  - **Handler 네이밍 통일**
    - `create_video_handler` → `admin_create_video`
    - `get_vimeo_preview_handler` → `admin_get_vimeo_preview`
    - `create_vimeo_upload_ticket_handler` → `admin_create_vimeo_upload_ticket`
    - `get_task_explain_handler` → `get_task_explain`
    - `admin_get_lesson_detail` → `admin_get_lesson`
  - **Admin User logs 함수명 prefix 통일**
    - `get_admin_user_logs` → `admin_get_user_logs`
    - `get_user_self_logs` → `admin_get_user_self_logs`
  - **Video repo 함수명 통일**
    - `find_list_dynamic` → `list_videos`
    - `find_detail_by_id` → `get_video_detail`
    - `find_progress` → `get_progress`
    - `upsert_progress` → `update_progress`
  - **Section 8.7 "보류/낮음 우선순위" 업데이트**
    - URL/함수명 통일 ✅ 완료
    - Login 정보/로그 추가 ✅ — ip-api.com 연동 완료
    - Lesson 통계 기능 — 추후 구현 예정
- **2026-02-04 — Admin Upgrade (관리자 초대) 시스템 구현**
  - **백엔드 (7-68 ~ 7-70)**
    - `POST /admin/upgrade` — 관리자 초대 코드 생성 + 이메일 발송
    - `GET /admin/upgrade/verify` — 초대 코드 검증 (Public)
    - `POST /admin/upgrade/accept` — 관리자 계정 생성 (Public, OAuth 불가)
    - RBAC 정책: HYMN→Admin/Manager, Admin→Manager, Manager→불가
    - Redis TTL 10분, 일회용 코드 (ak_upgrade_{uuid})
    - `EmailTemplate::AdminInvite` 추가 (invite_url, role, invited_by, expires_in_min)
  - **프론트엔드**
    - `types.ts` — Upgrade 타입 추가 (UpgradeInviteReq/Res, UpgradeVerifyRes, UpgradeAcceptReq/Res)
    - `admin_api.ts` — API 함수 추가 (createAdminInvite, verifyAdminInvite, acceptAdminInvite)
    - `/admin/upgrade/join` — 초대 수락 페이지 (Public 라우트)
    - `/admin/users` — "Invite Admin" 버튼 및 초대 다이얼로그 추가
  - **파일 변경 목록**
    - `src/api/admin/upgrade/` — dto.rs, service.rs, handler.rs, router.rs, mod.rs (신규)
    - `src/api/admin/mod.rs`, `src/api/admin/router.rs` — upgrade 모듈 등록
    - `src/api/user/repo.rs` — find_user_by_email, find_user_by_nickname, create_admin_user 추가
    - `src/external/email.rs` — AdminInvite 템플릿 추가
    - `frontend/src/category/admin/types.ts` — Section 9 (Upgrade 타입)
    - `frontend/src/category/admin/admin_api.ts` — Section 9 (Upgrade API)
    - `frontend/src/category/admin/page/admin_upgrade_join.tsx` — 신규
    - `frontend/src/category/admin/page/admin_users_page.tsx` — 초대 다이얼로그 추가
    - `frontend/src/app/routes.tsx` — /admin/upgrade/join 라우트 추가
- **2026-02-04 — IP Geolocation 기능 구현**
  - **기능**: 로그인 시 IP 기반 지리정보 자동 조회 (ip-api.com 연동)
  - **저장 필드**: `login_country`, `login_asn`, `login_org`
  - **적용 테이블**: `login` (활성 세션), `login_log` (이력)
  - **파일 변경 목록**
    - `src/external/ipgeo.rs` — IpGeoClient 구현 (신규)
    - `src/external/mod.rs` — ipgeo 모듈 등록
    - `src/state.rs` — AppState에 `Arc<IpGeoClient>` 추가
    - `src/main.rs` — IpGeoClient 초기화
    - `src/api/auth/repo.rs` — insert_login_record_tx, insert_login_record_oauth_tx에 지리정보 파라미터 추가
    - `src/api/auth/service.rs` — 로그인/OAuth 세션 생성 시 geo 데이터 전달
    - `src/api/user/service.rs` — 회원가입 자동 로그인에 geo 데이터 전달
