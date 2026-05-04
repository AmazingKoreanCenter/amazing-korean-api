# AMK_DEBTS — 미해결 부채 카탈로그

> **목적**: amazing-korean-api 의 미해결 부채를 한 곳에 정리. 부채 발견 시 본 문서에 즉시 등재. 작업 우선순위 결정 + 조회 시 진입점.
> **작성 원칙**: 사실 기반. 가정·해석 배제. 위치(파일:줄/명령어/공식 ID) + 영향 범위 + 처리 시점 명시. **라인 번호는 작성 시점 (HEAD) 기준 — commit 후 stale 가능, 사용 시 grep 재확인**.
> **작성일**: 2026-05-04 (PR #205 진행 중 일괄 조사 + 5 agent 정합성 검증 + 경로 2 추가 조사 통합)
> **갱신 규칙**: 부채 신규 발생 시 해당 카테고리에 추가. 처리 완료 시 행 시작에 `~~취소선~~` + 완료일/PR 명시. 본 문서 직접 갱신 (메모리 미동기화 — 메모리는 본 문서 참조 포인터만).
> **참조 SSoT**:
> - production 인시던트 (INC-NNN): `AMK_CHANGELOG.md` + `feedback_deploy_env_sync.md`
> - AI 작업 사고 (M-NNN): `AMK_AI_MISTAKES.md`
> - 본 문서: 그 외 모든 부채 (기능/품질/보안/인프라/자동화)

---

## 0. 요약 (카테고리별 카운트, 2026-05-04 정합성 검증 후)

| 카테고리 | 미해결 건수 | 비고 |
|---------|:---:|------|
| A. 운영/배포 부채 | **15** | KYB 의존 4 + 인프라 이전 3 + 진행 예정 큐 4 + **신규 8** (SSL/HTTPS, 백업, 디스크 모니터링 등) |
| B. 보안 부채 (취약점) | **7** | Rust 4 + npm **3** (postcss + follow-redirects + basic-ftp HIGH) |
| B. 보안 부채 (unsound/unmaintained) | 7 | core2 yanked + paste + imageproc 3 + rand 2 |
| B. 보안 부채 (panic 위험) | 2 | unwrap 잠재 위험 (9건 중) |
| B. 보안 부채 (외부 통신) | **1** | ipgeo HTTP-only (신규 발견) |
| C. 코드 품질 부채 | **13** | ESLint 27 + lint:ui 9 + rustfmt 90+ + docs.rs 2 + bundle 27MB + 신규 6 (allow 53건 + TS any 3 + eslint-disable 11) |
| D. 인프라 부채 | 4 | RDS 이전 묶음 (A2 와 중복) |
| E. 기능 부채 (보류/조건부) | **11** | 9 (보류 8 + STATUS #11 이메일 수신 ✅) + **신규 3** (콘텐츠 시딩, SpeechSuper, 번들 최적화) |
| F. 모바일/데스크탑 앱 부채 | 5 | 외부 리포 SSoT |
| G. 자동 검증 부재 (CI 부채) | **13** | 기존 8 + **신규 5** (CODEOWNERS/PR template/cargo-deny/cargo-geiger/src 테스트 부족) |
| H. 문서/메모리 부채 | 2 | H1 메모리 stale (`user_profile.md` 72일) + H2 docs↔코드 검증 자동 X |
| I. AI 작업 사고 | **7** | `AMK_AI_MISTAKES.md` SSoT (M-006 → 신규 M-007 = 라인 번호 복사 시 미검증) |
| J. 환경변수/Secrets 정합성 | **4** | 신규 — APPLE_*/RATE_LIMIT_TEXTBOOK_* 미동기화 + INC-001 패턴 위험 |

**총 미해결 부채 = 약 91건** (카테고리 중복 미배제, 단순 카운트).

---

## A. 운영/배포 부채

### A1. Paddle Live 전환 (KYB/Onfido 인증 의존)

| 항목 | 위치 (HEAD 2026-05-04) | 심각도 | 처리 시점 |
|------|------|:--:|----------|
| A1-1 | 12개 PADDLE_* Secret 일괄 교체 | `.github/workflows/deploy.yml:92-103` | CRITICAL | KYB 완료 후 |
| A1-2 | Webhook Secret 1회성 (재발급 필요) | `docs/AMK_DEPLOY_OPS.md:985` | CRITICAL | 동일 |
| A1-3 | KYB/Onfido 인증 지연 가능 | `docs/AMK_DEPLOY_OPS.md:947` (§8.5) | HIGH | 외부 처리 대기 |
| A1-4 | SPF 레코드 병합 (Resend + Cloudflare) | `docs/AMK_DEPLOY_OPS.md:1023` | MEDIUM | DNS 작업 |

> SSoT: `AMK_STATUS.md §8.5` 체크리스트.

### A2. RDS/ElastiCache 이전

| 항목 | 위치 (HEAD) | 심각도 | 비고 |
|------|------|:--:|------|
| A2-1 | E-book fs::read 9곳 — service.rs 8 + watermark.rs 1 | `src/api/ebook/service.rs:63, 381, 627, 641, 650, 731, 746, 755` + `src/api/ebook/watermark.rs:13` | CRITICAL | RDS 이전 시 S3 SDK 전환 (Q9) |
| A2-2 | PostgreSQL SSL 미사용 (DATABASE_URL localhost 기본값) | `src/config.rs:109-110` | HIGH | RDS 전환 시 SSL 강제 |
| A2-3 | Redis AUTH 토큰 부재 (REDIS_URL = `redis://127.0.0.1:6379` 기본값) | `src/config.rs:113` | HIGH | ElastiCache 이전 시 |

### A3. 진행 예정 큐 (사용자 트리거 대기)

| ID | 항목 | 처리 시점 |
|:--:|------|----------|
| Q14 | E-book 페이지 이미지 EC2 업로드 (books-api-bridge §3 Stage 2 #3-B) | 사용자 트리거 + EC2 디스크 확인 |
| Q15 | E-book 보안 옵션 A (행동 기반 봇 탐지) | 사용자 트리거 |
| Q16 | Frontend lint + lint:ui baseline cleanup (36 errors) | 디자인 토큰 결정 + 1-2일 |
| Q17 | Backend cargo test + Frontend playwright e2e CI 도입 | Q16 후 |

> SSoT: `AMK_STATUS.md §8.2`.

### A4. 운영 인프라 신규 부채 (2026-05-04 정합성 검증 발견, AMK_STATUS 미등재)

| ID | 항목 | 위치 / 사실 | 심각도 |
|:--:|------|-------------|:--:|
| A4-1 | nginx HTTPS 미활성 (HTTP-only) | `nginx/nginx.conf` 의 SSL 블록 주석 처리 상태. HSTS 미설정 | HIGH |
| A4-2 | Let's Encrypt + certbot 자동 갱신 정책 부재 | `docker-compose.prod.yml` 에 `amk-certbot` 컨테이너 존재하나 갱신 cron / 자동화 미명시 | HIGH (90일 만료 시 다운) |
| A4-3 | EC2 디스크 모니터링 자동화 부재 | `df -h` 임계값 / 자동 정리 / 알림 X. UptimeRobot 은 HTTP 만 (#71) | MEDIUM |
| A4-4 | DB / Redis 백업 정책 부재 (DR 0) | `docker-compose.prod.yml` volume-only. EC2 스냅샷 / pg_dump 자동화 X | HIGH |
| A4-5 | Docker log 로테이션 미설정 | `docker-compose.prod.yml` 에 logging driver 옵션 없음 — 무한 누적 | MEDIUM |
| A4-6 | Cloudflare DNS / Email Routing 운영 정책 미문서화 | 변경 시 수동 작업, SSoT 위치 명확화 X | MEDIUM |
| A4-7 | nginx Rate Limiting 모니터링 부재 | `nginx.conf:31` 정의만, 실 위반 로그 / 대시보드 X | MEDIUM |
| A4-8 | Docker base image 자동 업데이트 정책 부재 | postgres:16 / redis:7 / nginx:alpine 보안 패치 자동 모니터링 X | MEDIUM |

---

## B. 보안 부채

### B1. Rust 의존성 보안 취약점 (4 vulnerabilities, cargo audit 2026-05-04)

| ID | Crate | Version | Severity | Title | 처리 가능성 |
|:--:|:-----:|:-------:|:--------:|-------|----------|
| RUSTSEC-2023-0071 | rsa | 0.9.10 | medium 5.9 | Marvin Attack: timing sidechannel key recovery | **No fixed upgrade — 의존성 회피 검토 필요** |
| RUSTSEC-2026-0099 | rustls-webpki | 0.103.10 | — | Name constraints accepted for wildcard certificates | upgrade ≥0.103.12 |
| RUSTSEC-2026-0104 | rustls-webpki | 0.103.10 | — | Reachable panic in CRL parsing | upgrade ≥0.103.13 |
| RUSTSEC-2026-0098 | rustls-webpki | 0.103.10 | — | Name constraints for URI names incorrectly accepted | upgrade ≥0.103.12 |

> rustls-webpki 3건 = 한 번 cargo update 로 처리 가능.

### B2. Rust 의존성 unsound/unmaintained (7건, 2026-05-04 검증 후 7건으로 정정)

| ID | Crate | Version | Warning |
|:--:|:-----:|:-------:|---------|
| RUSTSEC-2026-0105 | core2 | 0.4.0 | unmaintained, **all versions yanked** |
| RUSTSEC-2024-0436 | paste | 1.0.15 | no longer maintained |
| RUSTSEC-2026-0116 | imageproc | 0.25.0 | unsound — improper invariant check |
| RUSTSEC-2026-0117 | imageproc | 0.25.0 | unsound — fragile bounds check (sampling) |
| RUSTSEC-2026-0115 | imageproc | 0.25.0 | unsound — fragile bounds check (sampling) |
| RUSTSEC-2026-0097 | rand 0.8.5 | unsound — custom logger interaction |
| RUSTSEC-2026-0097 | rand 0.9.2 | 동일 |

> imageproc = `src/api/ebook/watermark.rs:2,44,106` 사용 (텍스트 오버레이). agent 검증 = 3건 unsound 모두 기하학 변환/샘플링 관련, 텍스트 오버레이 경로 영향 낮음. rand = 우리 시스템 custom logger 미사용으로 영향 낮음.

### B3. npm 의존성 보안 취약점 (2026-05-04 정합성 검증 후 3건으로 정정)

| Severity | Package | 상세 |
|:--------:|:-------:|------|
| moderate | postcss <8.5.10 | XSS via Unescaped `</style>` (GHSA-qx2v-qp2m-jg93) |
| moderate | follow-redirects ≤1.15.11 | Custom Auth Header leak |
| **HIGH** | basic-ftp ≤5.2.2 | DoS via unbounded memory |

> `npm audit fix` 로 자동 해결 시도 가능 (미실행).

### B4. panic 위험 잠재 — `unwrap()` (9건 중 2건 위험)

| 위치 | 코드 | 위험 | 비고 |
|------|------|:--:|------|
| `src/error.rs` | `to_string().parse().unwrap()` | 안전 | round-trip |
| `src/api/user/service.rs` (3곳) | `NaiveDate::from_ymd_opt(1900,1,1).unwrap()` / Argon2 `Params::new` | 안전 | 정적 값 |
| **`src/api/auth/service.rs:358`** | `Some(user) => PasswordHash::new(user.user_password.as_ref().unwrap())` | **위험 잠재** | Option 가 None 시 panic. 인증 흐름 |
| **`src/api/auth/service.rs:1123`** | `let user_info = user.unwrap()` | **위험 잠재** | Option 가 None 시 panic |
| `src/api/ebook/watermark.rs:170` | `hash[..8].try_into().unwrap()` | 안전 | 길이 검증 후 |
| `src/api/admin/user/service.rs` (2곳) | `NaiveDate::from_ymd_opt(1900,1,1).unwrap()` | 안전 | 정적 값 |

**B4 처리**: 위험 잠재 2건 = 명시적 에러 매핑 (`AppError::Internal`) 으로 교체. 시간 1-2시간.

### B5. `expect()` 47건 (전수 점검 미실행)

> 카운트만 정확. 위험도 평가 = 별도 트랙. config.rs 의 panic 게이트 = 의도된 fail-fast (정상). 데이터 처리 흐름의 expect 만 검토 대상.

### B6. ipgeo HTTP-only (2026-05-04 신규 발견)

| 위치 | 사실 |
|------|------|
| `src/external/ipgeo.rs:11~12` (또는 그 부근) | ip-api.com 무료 이용권 = HTTP only (HTTPS 는 유료). IP 기반 위치 조회 시 평문 전송 → 중간자 공격 위험 |

**처리**: ip-api 유료 전환 또는 MaxMind GeoLite2 로컬 DB 전환 (E-9 와 통합 가능).

---

## C. 코드 품질 부채

### C1. Frontend ESLint baseline (Q16) — 27 errors + 13 warnings

> 카테고리 분류 정정 (2026-05-04 agent 검증):
> - `react-hooks/incompatible-library` **10건**
> - `react-hooks/static-components` **9건**
> - `react-refresh/only-export-components` **7건**
> - 기타 (prefer-const, no-empty 등) 1건씩

**처리**: shadcn 컴포넌트 파일 분할 + react-hooks 위반 fix + prefer-const fix. 시간 1-2일.

### C2. Frontend lint:ui baseline (Q16) — 9 errors

| 위치 | 카운트 | 색상 |
|------|:--:|------|
| `frontend/src/category/textbook/page/textbook_order_page.tsx` | 2 | emerald, amber (정정: 이전 4 → 2) |
| `frontend/src/category/textbook/receipt_parts.tsx` | 1 | red |
| `frontend/src/category/book/page/book_hub_page.tsx` | 4 | emerald, amber, rose, teal |
| `frontend/src/category/study/component/writing/HangulKeyboardKey.tsx` | 1 | amber |

**합계 정정**: 9 (이전 표기 4+1+4+1=10 → 실제 2+1+4+1+? = 재확인 필요. agent 보고 = 9). 핵심 = 9 카운트 일치.

**처리**: 디자인 토큰 결정 + 9곳 교체. 시간 0.5-1일.

### ~~C3. Rust rustfmt baseline~~ ✅ 해결 (2026-05-04 밤, commit 후속)

> 95 파일 cleanup 완료. `cargo fmt --check --all` exit=0. C4 trailing whitespace 수동 fix 후 cargo fmt --all 재실행 성공.

### ~~C4. `src/docs.rs:92, 94` trailing whitespace~~ ✅ 해결 (2026-05-04 밤)

> 수동 제거 완료. rustfmt internal error 해소. C3 cleanup 진행 가능해짐.

### ~~C5. enum sqlx::Type derive 미전환~~ ✅ 해결됨

> 2026-05-04 검증 결과 = `src/types.rs` 의 enum 들은 이미 `#[sqlx(type_name = "...", rename_all = "...")]` 패턴 적용 완료. `AMK_STATUS.md §8.2 보류 #13` 도 정정 필요.

### C6. TODO/FIXME 주석 — 1건

| 위치 | 내용 |
|------|------|
| `src/api/video/repo.rs:233` | `video_last_ip_log는 현재 항상 NULL. IP 수집 시 암호화 필수 (Phase 3 참조)` |

### C7. Frontend bundle 사이즈 모니터링 부재 (27MB)

| 큰 파일 (top 5) | 사이즈 |
|---|---:|
| `vendor-react-D3yu5mlF.js` | 226KB |
| `index-Bmlfndn9.js` | 208KB |
| `vendor-BqtW7tHd.js` | 183KB |
| `bn-BgM32R8S.js` | 113KB |
| `lo-xZsRdXQX.js` | 110KB |

vite-bundle-analyzer 미설정. bundle 비대화 자동 감지 X. (참고 = AMK_DEPLOY_OPS.md:29 TODO "번들 크기 최적화" — 같은 부채)

### C8-C13. Rust/TS 룰 회피 카운트 (2026-05-04 신규 조사)

| ID | 항목 | 카운트 | 비고 |
|:--:|------|:--:|------|
| C8 | Rust `#[allow(dead_code)]` | **33건** | 죽은 코드 회피 |
| C9 | Rust `#[allow(clippy::*)]` | **11건** | 특정 clippy 룰 회피 |
| C10 | Rust `#[allow(unused_imports)]` | **8건** | 미사용 import 회피 |
| C11 | Rust `#[allow(unused_assignments)]` | 1건 | 미사용 할당 회피 |
| C12 | TypeScript `any` 사용 | **3건** | 타입 안전성 회피 |
| C13 | TypeScript `eslint-disable` 인라인 | **11건** | 인라인 룰 회피 |

> 안전 (참고): Rust `unsafe` **0건** ✅ / TypeScript `@ts-ignore` **0건** ✅

---

## D. 인프라 부채 (RDS 이전 묶음, A2 + Q9 / Q14 와 중복)

| ID | 항목 | 처리 시점 |
|:--:|------|----------|
| D1 | E-book 9곳 fs::read → S3 SDK 전환 (Q9) | RDS 이전 시 |
| D2 | PostgreSQL SSL 미사용 → 강제 | RDS 이전 시 |
| D3 | Redis AUTH 토큰 부재 → 강제 | ElastiCache 이전 시 |
| D4 | E-book WebP 페이지 이미지 EC2 → S3 (Q14 + Q9 통합) | RDS 이전 시 |

---

## E. 기능 부채

### E1. 보류/조건부 (AMK_STATUS §8.2, 8건)

| ID | 항목 | 트리거 |
|:--:|------|--------|
| E-9 | GeoIP 전환 (ip-api.com → MaxMind GeoLite2) | 트래픽 증가 시 (B6 와 통합 가능) |
| E-10 | step-up MFA | 결제/비밀번호 변경 시 추가 인증 |
| E-12 | 토큰 Redis 캐싱 | 동시접속 10K+ |
| E-14 | Keyset 페이징 | 데이터 1만 건+ |
| E-15 | Lesson 통계 endpoint (`/admin/lessons/stats`) | 필요 시 |
| E-16 | 학습 문제 동적 생성 | 커리큘럼 완비 후 |
| E-17 | 통계 비동기/배치 분리 | 집계 복잡화 시 |
| E-18 | OAuth 중복 통합 | 세 번째 OAuth 추가 시 |
| E-19 | manager 역할 구현 | class 테이블 구현 후 |

### E2. AMK_API_FUTURE.md 미구현 (2026-05-04 신규)

| ID | 항목 | 위치 |
|:--:|------|------|
| E-FUTURE-1 | 콘텐츠 시딩 Phase 2/3 | `docs/AMK_API_FUTURE.md` |
| E-FUTURE-2 | 발음/조음/TTS 평가 (3건) | 동일 |

### E3. AMK_API_TEXTBOOK.md 미구현 (2026-05-04 신규)

| ID | 항목 | 위치 |
|:--:|------|------|
| E-TEXTBOOK-1 | SpeechSuper API 프로토타이핑 | `docs/AMK_API_TEXTBOOK.md` Phase 2 |

---

## F. 모바일/데스크탑 앱 부채

> 외부 리포 SSoT: `docs/AMK_APP_ROADMAP.md`. 본 문서는 api 측 영향 항목만 추적.

| ID | 항목 | 심각도 | 위치 |
|:--:|------|:--:|------|
| F1 | Flutter `flutter_rust_bridge` 버전 핀닝 필수 | HIGH | AMK_APP_ROADMAP.md R1 |
| F2 | Flutter E-book 뷰어 메모리 OOM (14MB/페이지) | HIGH | AMK_APP_ROADMAP.md R7 |
| F3 | Flutter iOS isSecureTextEntry 비공식 API | MEDIUM | AMK_APP_ROADMAP.md R2 |
| F4 | Flutter 앱 백그라운드 시 세션 만료 (TTL 90초) | MEDIUM | `src/config.rs:91, 375-378` (`EBOOK_SESSION_TTL_SEC`) |
| F5 | Tauri macOS 캡처 방지 불가 (Apple 정책) | MEDIUM (수용) | AMK_APP_ROADMAP.md R5 |

---

## G. 자동 검증 부재 (CI 부채)

### G1-G2. 명시 보류 (Q17)

| ID | 항목 | 사유 |
|:--:|------|------|
| G1 | `cargo test` CI 실행 | PostgreSQL service container 셋업 필요 |
| G2 | playwright e2e CI 실행 | 브라우저 + 시나리오 + CI 분 사용 큼 |

### G3-G8. 미점검 영역

| ID | 항목 | 현재 상태 |
|:--:|------|----------|
| G3 | `cargo audit` 자동 실행 | ❌ 미설정. 2026-05-04 수동 실행 = B1 4건 + B2 7건 발견 |
| G4 | `npm audit` 자동 실행 | ❌ 미설정. 2026-05-04 수동 = B3 3건 |
| G5 | `cargo outdated` / `npm outdated` | ❌ 미실행 |
| G6 | dependabot 자동 PR | ❌ `.github/dependabot.yml` 미존재 |
| G7 | secret scanning / GitHub Advanced Security | ❌ 미확인 (repo 설정 점검 필요) |
| G8 | main branch protection | ❌ 명시 보류 (1인 환경 force push 자유도 우선) |

### G9. PR 검사 워크플로 한계

`.github/workflows/pr-check.yml` (2026-05-04 도입) 안에서:
- ✅ `cargo fmt --check --all`
- ✅ `cargo clippy --lib --bins --locked -- -D warnings`
- ✅ `cargo check --locked --workspace`
- ✅ `npm run build` (= `tsc -b && vite build`)
- ⚠️ `npm run lint` continue-on-error (Q16 cleanup 까지)
- ⚠️ `npm run lint:ui` continue-on-error (Q16 cleanup 까지)

**한계**: cargo test / e2e / cargo audit / npm audit / outdated 모두 미포함.

### G10-G14. 신규 미점검 영역 (2026-05-04 발견)

| ID | 항목 | 사실 |
|:--:|------|------|
| G10 | src/ 테스트 부족 — 본체 테스트 4건만 (`crates/crypto` 46건 OK) | `grep -rn '#\[test\]' src` 4건 |
| G11 | `cargo-deny` 미설치 (라이선스 호환성 / 의존성 정책 검증) | `deny.toml` 미존재 |
| G12 | `cargo-geiger` 미설치 (unsafe 코드 분석) | unsafe 0건이라 우선순위 낮음 |
| G13 | `.github/CODEOWNERS` 미존재 | PR 자동 reviewer 지정 불가 |
| G14 | PR template / issue template 미존재 | 일관된 이슈 추적 불가 |

---

## H. 문서/메모리 부채

| ID | 항목 | 사실 |
|:--:|------|------|
| H1 | 메모리 stale 위험 (자동 갱신 부재) | 메모리 30개 중 가장 오래된 = `user_profile.md` (Mar 24, **72일 미갱신**). `reference_qa_automation.md` (Apr 8) / `reference_figma.md` (Apr 9) 등 |
| H2 | docs ↔ 코드 일관성 자동 검증 없음 | 예: `AMK_API_TEXTBOOK.md` 35 lang 명시 = `src/types.rs::TextbookLanguage` 35 variant 일치 (2026-05-04 검증 OK), 단 자동 도구 X |

---

## I. AI 작업 사고 (별도 SSoT)

> 본 카테고리는 `docs/AMK_AI_MISTAKES.md` 가 SSoT.

**M-001 ~ M-007 (2026-05-04 누적 7건)**:
- 사전 상태 미확인 카테고리: M-001, M-003, M-004
- 추정 단정 카테고리: M-002, M-005, M-006, **M-007** (다른 문서 라인 번호 복사 시 직접 검증 X)

---

## J. 환경변수 / Secrets 정합성 (2026-05-04 정정)

| 영역 | 카운트 | 상세 |
|------|:---:|------|
| `.env.example` 정의 | **65건** | (이전 표기 57 → 정정) |
| `.github/workflows/deploy.yml` 안 secrets 사용 | **22건** | `secrets.X` 호출 |
| `deploy.yml` heredoc env 변수 | **33건** | secrets + hardcoded (APP_ENV=production 등) |
| `src/config.rs` `env::var()` 호출 | **78건** | env::var + ENCRYPTION_KEY 레거시 폴백 |

### J1-J4. 정합성 문제 (신규 발견)

| ID | 항목 | 위험 |
|:--:|------|------|
| **J1** | `RATE_LIMIT_TEXTBOOK_WINDOW_SEC` / `RATE_LIMIT_TEXTBOOK_MAX` config.rs `expect()` panic 사용 + `.env.example` 미정의 + `deploy.yml` 미명시 | **INC-001 패턴 잠재** (production 배포 시 환경변수 부재 → expect panic → 컨테이너 crash). config.rs:191-198 |
| J2 | `APPLE_CLIENT_ID` / `APPLE_TEAM_ID` config.rs `Option` 사용 (panic X) + `.env.example` 미정의 | LOW (Apple OAuth 미구현 시 정상). config.rs:234-235 |
| J3 | 정합성 검증 자동 도구 X | deploy.yml heredoc / .env.example / config.rs 3중 동기화 수동 |
| J4 | `panic` 게이트 추가 시 동기화 누락 위험 | INC-001 사후 학습. feedback_deploy_env_sync.md 룰 강제 X |

---

## 우선순위 매트릭스 (2026-05-04 정정)

### 즉시 처리 권장 (위험 vs 비용 = 위험 우세)

| 우선 | 항목 | 사유 |
|:-:|------|------|
| 1 | **B1 rustls-webpki 3건 upgrade** | `cargo update` 1 명령 |
| 2 | **B3 npm postcss + follow-redirects + basic-ftp HIGH** | `npm audit fix` 1 명령 |
| 3 | **J1 RATE_LIMIT_TEXTBOOK_* 동기화** | INC-001 패턴 잠재. deploy.yml + .env.example 동시 추가 |
| 4 | **B4 unwrap 위험 2건 (auth/service.rs:358, 1123)** | 1-2시간, 명시적 에러 매핑 |
| 5 | **C3+C4 rustfmt baseline** | 본 PR 결정 대기 |
| 6 | **G6 dependabot 도입** | `.github/dependabot.yml` 1 파일 |
| 7 | **A4-1, A4-2 SSL/HTTPS + certbot 자동 갱신** | 90일 만료 대비 (외부 트리거 없으면 잊기 쉬움) |
| 8 | **A4-4 DB/Redis 백업 정책** | DR 0 = 데이터 손실 위험 큼 |

### 중기 (1-2주 내)

| 우선 | 항목 | 사유 |
|:-:|------|------|
| 9 | **Q16 ESLint + lint:ui baseline (36 errors)** | 디자인 토큰 결정 + 1-2일 |
| 10 | **B2 imageproc unsound 3건** | watermark 영향 점검 |
| 11 | **A4-3, A4-5, A4-7 디스크 모니터링 + log 로테이션 + nginx rate limit 모니터링** | 무한 누적 방지 |
| 12 | **C5 STATUS §8.2 #13 정정 (이미 해결)** | 문서 갱신 |
| 13 | **C8-C13 룰 회피 카운트 점검** | 죽은 코드 / clippy allow 정리 |

### 장기 (트리거 조건 충족 시)

A1 Paddle Live (KYB), A2 RDS 이전 (앱 개발 후), E 기능 부채 (트리거 조건), F 앱 부채 (앱 개발 시).

### 보류 명시

- G8 branch protection (1인 환경 force push 자유도 우선)
- B1 rsa Marvin Attack (No fixed upgrade — 대안 검토 필요)
- C12 cargo-geiger (unsafe 0건이라 우선순위 낮음)

---

## 처리 트리거 / 진입점

| 카테고리 | 진입점 | 작업 위치 |
|---------|--------|----------|
| A1 Paddle | KYB 인증 완료 | `AMK_DEPLOY_OPS.md §8.5` |
| A2 RDS 이전 | 앱 개발 완료 (~1.5개월) | `AMK_DEPLOY_OPS.md §8` |
| A3 Q14/Q15/Q16/Q17 | 사용자 트리거 | `AMK_STATUS.md §8.2` |
| A4 운영 인프라 | 본 문서 직접 진입 | nginx / docker-compose / EC2 운영 |
| B1-B3 의존성 보안 | 본 문서 직접 진입 | `cargo audit` / `npm audit` 재실행 |
| B4 unwrap 위험 | 본 문서 직접 진입 | `src/api/auth/service.rs:358, 1123` |
| B6 ipgeo HTTP | E-9 와 통합 | MaxMind 전환 |
| C 품질 | Q16 (lint), Q17 (test) | `AMK_STATUS.md §8.2` |
| D 인프라 | A2 와 통합 | 동일 |
| E 보류 기능 | 트리거 조건 충족 시 | `AMK_STATUS.md §8.2 보류/조건부` |
| E2-E3 미구현 | 콘텐츠/SpeechSuper 트리거 시 | `AMK_API_FUTURE.md` / `AMK_API_TEXTBOOK.md` |
| F 앱 | 앱 리포 개발 시 | `amazing-korean-mobile`, `amazing-korean-desktop` |
| G 자동화 | 본 문서 직접 진입 | 신규 워크플로 / 설정 파일 |
| H 문서 | 본 문서 직접 진입 | 메모리 갱신 흐름 |
| I AI 사고 | 별도 SSoT | `docs/AMK_AI_MISTAKES.md` |
| J Secrets | INC-001 패턴 회피 | deploy.yml + .env.example + config.rs 3중 동기화 |

---

## 갱신 규칙

### 신규 부채 발견 시

1. 해당 카테고리에 추가
2. 사실만 기재 (위치 / 영향 / 처리 가능성). 가정/해석 배제
3. 라인 번호 = HEAD 기준, **사용 시 grep 재확인 의무 (라인 stale 위험)**
4. 우선순위 매트릭스 영향 시 갱신
5. 카운트 (§0) 갱신
6. **메모리 갱신 불필요** (메모리 = `feedback_debts_reference.md` 포인터만)

### 부채 처리 완료 시

1. 행 시작에 `~~취소선~~` + 완료일 + 커밋/PR 명시
2. 카운트 (§0) 갱신
3. 우선순위 매트릭스에서 제거

### 트리거 조건 변경 시

1. 처리 시점 컬럼 갱신
2. 우선순위 매트릭스 재정렬

### 정합성 검증 (정기)

본 문서 자체의 stale 위험 — 분기 1회 (또는 새 부채 5건+ 발견 시) 5 agent 정합성 검증 권장.

---

## 관련 SSoT

- `AMK_STATUS.md §8.2` 진행 예정 + 검증된 리스크 + 보류/조건부
- `AMK_DEPLOY_OPS.md` 배포/운영 절차 + 인시던트 패턴
- `AMK_CHANGELOG.md` 변경 이력 + 인시던트 (INC-NNN) 사후 기록
- `AMK_AI_MISTAKES.md` AI 작업 사고 (M-NNN)
- `AMK_APP_ROADMAP.md` 모바일/데스크탑 앱 로드맵 + 리스크 (R-NNN)
- `AMK_API_FUTURE.md` 미구현 기능 (콘텐츠 시딩, 발음, 조음, TTS)
- `AMK_API_TEXTBOOK.md` SpeechSuper Phase 2
- 메모리 `feedback_deploy_env_sync.md` 인시던트 패턴 (INC-001~005)
- 메모리 `feedback_migration_safety.md` 마이그레이션 학습 + INC-002/003/004
- 메모리 `feedback_debts_reference.md` 본 문서 포인터
