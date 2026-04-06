# AMK 코드 점검 결과

> 점검별 결과를 누적 기록. 수정은 전체 점검 완료 후 일괄 수행.

---

## 점검 1: 의존성 취약점 + 업데이트 (2026-04-03)

### 1-1. cargo audit — Rust 보안 취약점

| # | 크레이트 | 버전 | 심각도 | ID | 문제 | 수정 방법 | 비고 |
|---|----------|------|--------|----|------|-----------|------|
| 1 | rustls-webpki | 0.103.9 | Medium | RUSTSEC-2026-0049 | CRL Distribution Point 매칭 오류 | `>=0.103.10` | reqwest/sqlx 간접 의존 → `cargo update` |
| 2 | time | 0.3.46 | Medium (6.8) | RUSTSEC-2026-0009 | Stack Exhaustion DoS | `>=0.3.47` | **직접 의존** + jsonwebtoken/cookie/paddle-sdk |
| 3 | rsa | 0.9.10 | Medium (5.9) | RUSTSEC-2023-0071 | Marvin Attack 타이밍 사이드채널 | 수정판 없음 | sqlx-mysql 경유, **PostgreSQL만 사용 → 실질 영향 없음** |
| 4 | paste | 1.0.15 | Warning | RUSTSEC-2024-0436 | unmaintained | — | imageproc/rav1e 간접 의존, 조치 불필요 |

**조치 계획**:
- `cargo update` → rustls-webpki, time 패치 자동 적용
- rsa: sqlx-mysql feature 비활성화 검토 (또는 무시 — MySQL 미사용)
- paste: 무시 (간접 의존, 대안 없음)

### 1-2. npm audit — 프론트엔드 보안 취약점 (8건)

| # | 패키지 | 심각도 | 문제 | 경유 | 프로덕션 영향 |
|---|--------|--------|------|------|--------------|
| 1 | basic-ftp <5.2.0 | **Critical** | Path Traversal (downloadToDir) | puppeteer | devDep — 없음 |
| 2 | rollup 4.0~4.58 | **High** | Arbitrary File Write via Path Traversal | vite | devDep — 빌드 도구 |
| 3 | flatted <=3.4.1 | **High** | unbounded recursion DoS + Prototype Pollution | eslint | devDep — 없음 |
| 4 | minimatch <=3.1.3 | **High** | ReDoS (복수 패턴) | eslint/ts-estree | devDep — 없음 |
| 5 | picomatch <=2.3.1 | **High** | Method Injection + ReDoS | anymatch/micromatch | devDep — 없음 |
| 6 | ajv <6.14.0 | Moderate | ReDoS ($data 옵션) | eslint | devDep — 없음 |
| 7 | brace-expansion | Moderate | Zero-step sequence hang | ts-estree | devDep — 없음 |
| 8 | yaml 2.0~2.8.2 | Moderate | Stack Overflow (deeply nested) | postcss? | devDep — 없음 |

**조치 계획**: `npm audit fix` 일괄 수정. 전부 devDependencies 경유 → 프로덕션 번들 미포함.

### 1-3. cargo outdated — Rust 업데이트 가능 패키지 (21건)

#### 패치/마이너 업데이트 (안전 — `cargo update`로 적용)

| 크레이트 | 현재 | 호환 최신 | 비고 |
|----------|------|-----------|------|
| anyhow | 1.0.100 | 1.0.102 | |
| chrono | 0.4.43 | 0.4.44 | |
| clap | 4.5.57 | 4.6.0 | |
| hyper | 1.8.1 | 1.9.0 | |
| image | 0.25.9 | 0.25.10 | |
| time | 0.3.46 | **0.3.47** | **취약점 수정 포함** |
| tokio | 1.49.0 | 1.50.0 | |
| totp-rs | 5.7.0 | 5.7.1 | |
| tracing-subscriber | 0.3.22 | 0.3.23 | |
| uuid | 1.20.0 | 1.23.0 | |

#### 메이저 업데이트 (breaking change — 별도 작업 필요)

| 크레이트 | 현재 | 최신 | 비고 |
|----------|------|------|------|
| axum-extra | 0.10.3 | 0.12.5 | axum 버전과 연동, 대규모 마이그레이션 |
| deadpool-redis | 0.16.0 | 0.23.0 | redis 크레이트 메이저 업데이트 연동 |
| hmac | 0.12.1 | 0.13.0 | crypto 크레이트 체인 (sha2, password-hash) |
| jsonwebtoken | 9.3.1 | 10.3.0 | API 변경 예상 |
| password-hash | 0.5.0 | 0.6.0 | argon2 연동 |
| rand / rand_core | 0.8.5 / 0.6.4 | 0.10.0 / 0.10.0 | 광범위 영향 |
| redis | 0.26.1 | 1.1.0 | deadpool-redis와 함께 업데이트 |
| reqwest | 0.12.28 | 0.13.2 | paddle-rust-sdk 호환 확인 필요 |
| sha2 | 0.10.9 | 0.11.0 | hmac/password-hash와 함께 업데이트 |
| imageproc | 0.25.0 | 0.26.1 | image 크레이트 연동 |

### 1-4. npm outdated — 프론트엔드 업데이트 가능 패키지 (27건)

#### 패치/마이너 업데이트 (안전 — `npm update`로 적용)

| 패키지 | 현재 | 최신 |
|--------|------|------|
| @tanstack/react-query | 5.90.16 | 5.96.1 |
| @types/react | 19.2.8 | 19.2.14 |
| @vimeo/player | 2.30.1 | 2.30.3 |
| autoprefixer | 10.4.23 | 10.4.27 |
| axios | 1.13.5 | 1.14.0 |
| i18next | 25.8.3 | 25.10.10 |
| postcss | 8.5.6 | 8.5.8 |
| puppeteer | 24.37.2 | 24.40.0 |
| react | 19.2.3 | 19.2.4 |
| react-dom | 19.2.3 | 19.2.4 |
| react-hook-form | 7.71.0 | 7.72.0 |
| react-i18next | 16.5.4 | 16.6.6 |
| react-router-dom | 7.12.0 | 7.14.0 |
| tailwind-merge | 3.4.0 | 3.5.0 |
| typescript-eslint | 8.52.0 | 8.58.0 |
| zod | 4.3.5 | 4.3.6 |
| zustand | 5.0.9 | 5.0.12 |

#### 메이저 업데이트 (breaking change — 별도 작업 필요)

| 패키지 | 현재 | 최신 | 비고 |
|--------|------|------|------|
| tailwindcss | 3.4.17 | 4.2.2 | v4 대규모 마이그레이션 (설정 체계 전면 변경) |
| vite | 7.3.1 | 8.0.3 | 빌드 도구 메이저 |
| typescript | 5.9.3 | 6.0.2 | TS 6 새 기능/breaking |
| eslint | 9.39.2 | 10.1.0 | flat config 전환 이미 완료, 메이저 변경 확인 필요 |
| @eslint/js | 9.39.2 | 10.0.1 | eslint와 함께 |
| @vitejs/plugin-react | 5.1.2 | 6.0.1 | vite 메이저와 함께 |
| @types/node | 24.10.7 | 25.5.0 | |
| eslint-plugin-react-refresh | 0.4.26 | 0.5.2 | |
| globals | 16.5.0 | 17.4.0 | |
| i18next | 25.8.3 | 26.0.3 | 메이저 (wanted 범위 내 25.10.10 먼저) |
| lucide-react | 0.562.0 | 1.7.0 | 아이콘 import 경로 변경 가능 |
| react-i18next | 16.5.4 | 17.0.2 | i18next 메이저와 함께 |

---

## 점검 1 종합 판정

| 구분 | Critical | High | Medium | Low/Warning | 합계 |
|------|----------|------|--------|-------------|------|
| Rust (cargo audit) | 0 | 0 | 3 | 1 | 4 |
| npm (npm audit) | 1 | 4 | 3 | 0 | 8 |
| **합계** | **1** | **4** | **6** | **1** | **12** |

**프로덕션 즉시 위험**: 없음
- npm Critical/High 전부 devDependencies 경유 → 프로덕션 번들 미포함
- Rust Medium 3건 중 rsa는 MySQL 미사용으로 무관, time/rustls-webpki는 `cargo update`로 해결

**일괄 수정 시 작업 순서**:
1. `cargo update` (time, rustls-webpki 패치 + 기타 마이너)
2. `npm audit fix` (devDep 취약점)
3. `npm update` (마이너/패치 안전 업데이트)
4. `cargo check` + `cd frontend && npm run build` 검증
5. 메이저 업데이트는 점검 완료 후 별도 계획

---

## 점검 2: 코드 품질 — 정적 분석 (2026-04-03)

### 2-1. cargo clippy — Rust 린터 (42 warnings)

#### 카테고리 A: 함수 인자 과다 (too many arguments) — 18건

| # | 파일 | 줄 | 인자 수 | 비고 |
|---|------|----|---------|------|
| 1 | `src/api/admin/lesson/repo.rs` | 113 | 8/7 | |
| 2 | `src/api/admin/lesson/repo.rs` | 693 | 8/7 | |
| 3 | `src/api/admin/lesson/repo.rs` | 741 | 8/7 | |
| 4 | `src/api/admin/lesson/repo.rs` | 773 | 9/7 | |
| 5 | `src/api/admin/lesson/repo.rs` | 813 | 9/7 | |
| 6 | `src/api/admin/payment/repo.rs` | 15 | 8/7 | |
| 7 | `src/api/admin/study/repo.rs` | 96 | 9/7 | |
| 8 | `src/api/admin/study/repo.rs` | 773 | 9/7 | |
| 9 | `src/api/admin/translation/repo.rs` | 110 | 8/7 | |
| 10 | `src/api/admin/user/repo.rs` | 135 | **18/7** | 최다 인자 |
| 11 | `src/api/admin/user/repo.rs` | 242 | 9/7 | |
| 12 | `src/api/admin/user/repo.rs` | 325 | 8/7 | |
| 13 | `src/api/admin/user/repo.rs` | 357 | 8/7 | |
| 14 | `src/api/ebook/repo.rs` | 97 | 8/7 | |
| 15 | `src/api/ebook/service.rs` | 557 | 8/7 | |
| 16 | `src/api/payment/repo.rs` | 146 | 13/7 | |
| 17 | `src/api/payment/repo.rs` | 229 | 12/7 | |
| 18 | `src/api/textbook/repo.rs` | 113 | **24/7** | 최다 인자 |

> **정정**: 초기 감사에서 #16을 1행으로 합쳐 17건으로 집계했으나 실제 2개 함수 → 18건.

**판정**: repo 레이어에서 SQL 파라미터를 개별 인자로 받는 패턴. 구조체로 묶으면 개선 가능하나 기능에 문제 없음 → **수정 권장 (리팩토링)**

#### 카테고리 B: 불필요한 변환/복사 — 11건

| # | 유형 | 파일 | 줄 |
|---|------|------|----|
| 1 | `useless conversion String→String` | `src/api/ebook/handler.rs` | 191, 281 |
| 2 | `useless conversion String→String` | `src/api/ebook/service.rs` | 263, 266 |
| 3 | `useless conversion String→String` | `src/api/ebook/watermark.rs` | 38, 58 |
| 4 | `clone on Copy type` | `src/api/admin/upgrade/service.rs` | 284 |
| 5 | `clone on Copy type` | `src/api/admin/user/service.rs` | 710, 712 |
| 6 | `unnecessary i64→i64 cast` | `src/api/admin/study/service.rs` | 1842 |
| 7 | `unnecessary i64→i64 cast` | `src/api/admin/user/service.rs` | 768 |

**판정**: `cargo clippy --fix`로 자동 수정 가능 → **즉시 수정**

#### 카테고리 C: 코드 개선 제안 — 10건

| # | 유형 | 파일 | 줄 |
|---|------|------|----|
| 1 | `map_or` → `is_none_or` | `src/api/admin/textbook/service.rs` | 80 |
| 2 | `map_or` → `is_none_or` | `src/api/textbook/service.rs` | 90, 95, 100, 105 |
| 3 | `unwrap_or_else` → `unwrap_or` | `src/api/video/service.rs` | 87 |
| 4 | `contains_key`+`insert` → `entry` | `src/config.rs` | 428 |
| 5 | `&String` → `&str` | `src/api/admin/study/dto.rs` | 499 |
| 6 | needless borrow | `src/external/google.rs` | 108 |
| 7 | `.last()` → `.next_back()` | `src/external/vimeo.rs` | 171 |
| 8 | match → `matches!` | `src/api/admin/upgrade/service.rs` | 55 |

**판정**: `cargo clippy --fix`로 자동 수정 가능 → **즉시 수정**

#### 카테고리 D: 구조적 경고 — 2건

| # | 유형 | 파일 | 줄 |
|---|------|------|----|
| 1 | large size difference between variants | `src/api/auth/service.rs` | 31 |
| 2 | large size difference between variants | `src/api/auth/service.rs` | 45 |

**판정**: enum variant를 `Box`로 감싸면 해결. 성능 영향 미미 → **수정 권장**

#### Clippy 종합

| 카테고리 | 건수 | 자동 수정 | 우선순위 |
|----------|------|-----------|----------|
| A. 함수 인자 과다 | 18 | 불가 | 리팩토링 |
| B. 불필요한 변환/복사 | 11 | **가능** → ✅ 수정 완료 | 즉시 |
| C. 코드 개선 제안 | 10 | **가능** → ✅ 수정 완료 | 즉시 |
| D. 구조적 경고 | 2 | 수동 | 권장 |
| **합계** | **42** (중복 제거) → **잔여 20** | B+C 22건 수정 완료 | — |

> **수정 후 잔여 20건**: too_many_arguments 18건 + large_enum_variant 2건 (리팩토링급, 기능 정상)

### 2-2. `#[allow(dead_code)]` 사용처 — 30건

| # | 파일 | 줄 | 용도 | 판정 |
|---|------|----|------|------|
| 1 | `src/config.rs` | 9~21 | `EmailProvider` enum variants (7건) | **정당** — 환경변수 분기용 |
| 2 | `src/state.rs` | 32 | AppState 필드 | 확인 필요 |
| 3 | `src/error.rs` | 220, 226 | 에러 유틸 함수 | 확인 필요 |
| 4 | `src/external/google.rs` | 20, 47~66, 73, 197 | Google OAuth 응답 구조체 필드 (13건) | **정당** — 역직렬화에 필요, 주석 설명 있음 |
| 5 | `src/api/admin/video/dto.rs` | 225 | DTO 필드 | 확인 필요 |
| 6 | `src/external/payment.rs` | 97 | Paddle 응답 필드 | **정당** — 역직렬화용 |
| 7 | `src/api/payment/repo.rs` | 40 | 쿼리 결과 구조체 | 확인 필요 |
| 8 | `src/api/auth/password.rs` | 27 | 패스워드 유틸 | 확인 필요 |
| 9 | `src/api/auth/repo.rs` | 13, 23, 35, 984 | 쿼리 결과 구조체 (4건) | **정당** — SQLx `FromRow`에 필요 |

**판정**: 대부분 역직렬화/SQLx `FromRow`에 필요한 필드. 5건은 실제 사용 여부 확인 필요 → **기록, 수정 시 확인**

### 2-3. `unwrap()` 사용처 — 20건

| # | 파일 | 줄 | 컨텍스트 | 판정 |
|---|------|----|----------|------|
| 1 | `src/main.rs` | 187~192 | HTTP 헤더 상수 파싱 | **안전** — 컴파일 타임에 알려진 상수 |
| 2 | `src/error.rs` | 213 | `retry_after_secs.to_string().parse()` | **안전** — 숫자→문자열→파싱 항상 성공 |
| 3 | `src/api/study/repo.rs` | 66~67 | `choice_1/2.unwrap()` | **위험** — DB NULL 시 panic |
| 4 | `src/api/admin/video/stats/repo.rs` | 47, 181 | `try_get().unwrap()` | **위험** — DB 컬럼 누락 시 panic |
| 5 | `src/api/admin/user/service.rs` | 243, 413 | `NaiveDate::from_ymd_opt().unwrap()` | **안전** — 고정 날짜 (1900-01-01) |
| 6 | `src/api/auth/service.rs` | 183 | `user_password.as_ref().unwrap()` | **조건부 안전** — 로직상 password 있는 경우만 도달 |
| 7 | `src/api/auth/service.rs` | 920 | `user.unwrap()` | **확인 필요** — 상위 로직 검증 |
| 8 | `src/api/user/service.rs` | 85 | `NaiveDate::from_ymd_opt().unwrap()` | **안전** — 고정 날짜 |
| 9 | `src/api/user/service.rs` | 126, 145 | `Params::new().unwrap()` | **안전** — 고정 파라미터 |
| 10 | `src/api/admin/user/stats/repo.rs` | 122, 232 | `try_get().unwrap()` | **위험** — DB 컬럼 누락 시 panic |
| 11 | `src/api/ebook/watermark.rs` | 170 | `try_into().unwrap()` | **안전** — 해시 8바이트 슬라이스 |
| 12 | `src/api/admin/study/stats/repo.rs` | 224 | `try_get().unwrap()` | **위험** — DB 컬럼 누락 시 panic |

**위험 건수**: 5건 (study/repo, admin stats repos) → **수정 권장** (`unwrap()` → `unwrap_or_default()` 또는 `?` 연산자)

### 2-4. `todo!()` / `unimplemented!()` — 0건 ✅

잔존 없음.

### 2-5. 미사용 의존성 (Cargo.toml)

| # | 크레이트 | 위치 | 비고 |
|---|----------|------|------|
| 1 | `aes-gcm` | 루트 `Cargo.toml` | crypto 워크스페이스 멤버에서만 사용, 루트에서 직접 사용 없음 → **제거 완료** |
| 2 | ~~`hmac`~~ | 루트 `Cargo.toml` | **FALSE POSITIVE** — `user/service.rs`, `ebook/service.rs`에서 직접 사용 중 → 유지 |

**판정**: `aes-gcm`만 제거. `hmac`은 루트에서 직접 사용 확인 → 감사 오판 정정.

---

### 2-6. TypeScript 빌드 (`tsc --noEmit`) — 0 errors ✅

에러 없음. 타입 체크 통과.

### 2-7. ESLint — 41건 (28 errors, 13 warnings)

#### 카테고리 A: React Hooks 규칙 위반 — **실제 버그 가능성** (7건)

| # | 파일 | 줄 | 규칙 | 문제 |
|---|------|----|------|------|
| 1 | `seal_list.tsx` | 54 | `rules-of-hooks` | useCallback 조건부 호출 |
| 2 | `seal_list.tsx` | 62 | `rules-of-hooks` | useCallback 조건부 호출 |
| 3 | `study_task_page.tsx` | 329 | `set-state-in-effect` | useEffect 내 동기 setState — 연쇄 렌더링 |
| 4 | `study_task_page.tsx` | 335 | `exhaustive-deps` | submitMutation 의존성 누락 |
| 5 | `use_paddle.ts` | 41 | `exhaustive-deps` | email 의존성 누락 |
| 6 | `use_oauth_callback.ts` | 185 | `refs` | 렌더 중 ref 접근 (2건) |
| 7 | `signup_page.tsx` | 123 | `no-unused-vars` | 미사용 변수 `_` |

**판정**: `rules-of-hooks` 2건은 **실제 런타임 버그 가능** → **즉시 수정 필요**

#### 카테고리 B: 렌더 중 Ref 접근 — 8건

| # | 파일 | 줄 |
|---|------|----|
| 1 | `use_paddle.ts` | 20, 96 (3건) |
| 2 | `seal_list.tsx` | 89 (3건) |
| 3 | `use_oauth_callback.ts` | 185 (2건) |

**판정**: React 19 strict mode에서 문제 가능 → **수정 권장**

#### 카테고리 C: 렌더 중 컴포넌트 생성 — 9건

| # | 파일 | 줄 |
|---|------|----|
| 1 | `admin_subscriptions_page.tsx` | 156, 161, 165, 169, 174 |
| 2 | `admin_transactions_page.tsx` | 130, 135, 139, 146 |

**판정**: 테이블 컬럼 정의에서 JSX 반환 — `useMemo`로 감싸면 해결 → **수정 권장**

#### 카테고리 D: Fast Refresh 비호환 — 5건

| # | 파일 |
|---|------|
| 1 | `components/blocks/data_table.tsx` |
| 2 | `components/ui/badge.tsx` |
| 3 | `components/ui/button.tsx` |
| 4 | `components/ui/card.tsx` |
| 5 | `components/ui/form.tsx` |

**판정**: shadcn/ui 컴포넌트 패턴 (variants + component 동일 파일). HMR에만 영향 → **무시 (shadcn 공식 패턴)**

#### 카테고리 E: 기타 경고 — 12건

| # | 유형 | 건수 | 비고 |
|---|------|------|------|
| 1 | `incompatible-library` (react-hook-form) | 10 | React Compiler 호환 경고 — 기능 정상 |
| 2 | `no-empty` (빈 catch 블록) | 1 | `admin_translation_edit.tsx:136` |
| 3 | `unused eslint-disable` | 1 | `devtools_detect.ts:31` |

**판정**: 기능 정상 → **수정 권장 (낮은 우선순위)**

### 2-8. `any` 타입 사용처 — 3건

| # | 파일 | 줄 | 비고 |
|---|------|----|------|
| 1 | `ebook_viewer_page.tsx` | 256 | `(window as any).createImageBitmap` — PDF.js 워커 패치 |
| 2 | `ebook_viewer_page.tsx` | 266 | `(origCreateImageBitmap as any).apply` |
| 3 | `ebook_viewer_page.tsx` | 277 | 복원 시 재할당 |

**판정**: PDF.js 브라우저 API 패치로 타입 정의 불가 — `eslint-disable` 주석 있음 → **무시 (정당)**

### 2-9. `@ts-ignore` / `@ts-expect-error` — 0건 ✅

### 2-10. `console.log` 잔존 — 1건

| # | 파일 | 줄 | 비고 |
|---|------|----|------|
| 1 | `devtools_detect.ts` | 32 | DevTools 감지 로직의 핵심 — `console.log` getter 트랩 |

**판정**: 보안 기능의 핵심 메커니즘 → **무시 (의도된 사용)**

---

## 점검 2 종합 판정

| 구분 | 즉시 수정 | 수정 권장 | 무시 | 합계 |
|------|-----------|-----------|------|------|
| Clippy (Rust) | 20 (자동) | 22 (수동) | 0 | 42 |
| unwrap() 위험 | 5 | 0 | 15 | 20 |
| `#[allow(dead_code)]` | 0 | 5 (확인 필요) | 25 | 30 |
| 미사용 의존성 | 2 | 0 | 0 | 2 |
| ESLint errors | 2 (hooks 위반) | 17 | 9 | 28 |
| ESLint warnings | 0 | 2 | 11 | 13 |
| TypeScript | 0 | 0 | 0 | 0 |
| any / @ts-ignore / console.log | 0 | 0 | 4 | 4 |
| **합계** | **29** | **46** | **64** | **139** |

**핵심 발견**:
1. **TypeScript 빌드 에러 0건** — 타입 안전성 양호
2. **`todo!()`/`unimplemented!()` 0건** — 미완성 코드 없음
3. **React Hooks `rules-of-hooks` 위반 2건** (`seal_list.tsx`) — **실제 런타임 버그 가능, 최우선 수정**
4. **unwrap() 위험 5건** — DB 쿼리 결과에서 panic 가능, 프로덕션 안정성 위협
5. **Clippy 20건 자동 수정 가능** — `cargo clippy --fix`로 일괄 적용

**일괄 수정 시 작업 순서**:
1. `seal_list.tsx` Hooks 규칙 위반 수정 (early return 이후 Hook 호출 → 최상단으로 이동)
2. `cargo clippy --fix` — 자동 수정 20건 적용
3. unwrap() 위험 5건 → `?` 또는 `unwrap_or_default()` 전환
4. 루트 `Cargo.toml`에서 `aes-gcm`, `hmac` 제거
5. 나머지 ESLint 권장 사항 순차 처리
6. `cargo check` + `cd frontend && npm run build` 검증

---

## 점검 3: 보안 리뷰 — OWASP Top 10 기반 (2026-04-03)

### 3-1. SQL Injection — **통과** ✅

전체 `src/` 디렉토리 SQL 쿼리 패턴 전수 조사 결과:

| 패턴 | 사용 여부 | 판정 |
|------|-----------|------|
| SQLx 바인딩 파라미터 (`$1`, `$2`, ...) | 전체 사용 | **안전** |
| `format!`으로 WHERE절 구성 | 있음 (textbook/repo, ebook/repo, admin repos) | **안전** — 플레이스홀더 `$N`만 생성, 실제 값은 `.bind()` |
| ORDER BY 동적 구성 | 있음 (admin/user, admin/payment, admin/study, admin/video) | **안전** — `match`로 하드코딩된 컬럼명만 허용 |
| LIMIT/OFFSET 직접 삽입 | `ebook/repo.rs:355` (`{per_page}`, `{offset}`) | **안전** — Rust `i64` 타입 보장, SQL 문자열 삽입 불가 |
| `query_unchecked` / `raw_sql` | 미사용 | — |
| `QueryBuilder` + `push_bind` | admin/video, admin/study, admin/lesson, video | **모범 패턴** |

**코드 품질 개선 제안** (보안 문제 아님):
- `ebook/repo.rs:355` — LIMIT/OFFSET을 `.bind()`로 변경하면 패턴 일관성 향상

### 3-2. 인증/인가 — **1건 발견** ⚠️

전체 라우터 82개 라우트 대조 결과:

#### 인증 메커니즘
- **JWT Bearer 토큰**: `AuthUser` extractor (필수) / `OptionalAuthUser` (선택)
- **Admin 라우트**: JWT + 역할 검증 (HYMN/Admin) + IP allowlist guard — 3중 보호

#### 발견된 문제

| # | 라우트 | 문제 | 심각도 |
|---|--------|------|--------|
| 1 | `POST /courses` | `AuthUser` extractor 없음 — 미인증 사용자가 코스 생성 가능 | **HIGH** |

**상세**: `src/api/course/handler.rs:23` — `create` 함수에 `AuthUser` 파라미터 없음. `src/api/course/router.rs:7`에서 `.post(handler::create)`로 미들웨어 없이 등록.

**에이전트 오판 정정**:
- `POST /videos/{id}/progress` → `AuthUser` extractor 적용 확인 (`handler.rs:111`) ✅
- `POST /lessons/{id}/progress` → `AuthUser` extractor 적용 확인 (`handler.rs:135`) ✅

#### 정상 확인된 보호 체계

| 영역 | 보호 방식 | 상태 |
|------|-----------|------|
| `/admin/*` 전체 | JWT + Role(HYMN\|Admin) + IP Guard | ✅ |
| `/users/me` (GET/PUT/POST) | `AuthUser` | ✅ |
| `/studies/tasks/{id}/answer` (POST) | `AuthUser` | ✅ |
| `/ebook/purchase`, `/ebook/viewer/*` | `AuthUser` | ✅ |
| `/textbook/orders` (POST) | `AuthUser` | ✅ |
| `/payment/subscription`, `/payment/subscription/cancel` | `AuthUser` | ✅ |
| `/payment/webhook` (POST) | Paddle 서명 검증 (인증 불필요) | ✅ |
| `/admin/upgrade/verify`, `/admin/upgrade/accept` | 초대 코드 검증 (의도적 공개) | ✅ |

### 3-3. Anti-enumeration — **2건 발견** ⚠️

| # | 위치 | 에러 메시지 | 문제 | 심각도 |
|---|------|------------|------|--------|
| 1 | `auth/service.rs:1014` | `"AUTH_401_USER_NOT_FOUND"` | `verify_reset_code()`에서 사용자 존재 여부 노출 | **MEDIUM** |
| 2 | `auth/service.rs:479` | `"User not found"` | `refresh()`에서 삭제된 사용자 구분 가능 | **LOW** |

**수정 방법**:
- #1 → `"AUTH_401_INVALID_OR_EXPIRED_CODE"` (이미 동일 함수 내 다른 분기에서 사용 중)
- #2 → `"AUTH_401_INVALID_REFRESH"` (이미 동일 함수 내 line 473에서 사용 중)

**정상 확인된 Anti-enumeration 패턴**:

| 흐름 | 보호 방식 | 상태 |
|------|-----------|------|
| 로그인 | dummy password hash + 통합 에러 `"AUTH_401_BAD_CREDENTIALS"` | ✅ 우수 |
| 비밀번호 재설정 요청 | `"If the email exists..."` 통일 메시지 (존재/미존재/OAuth 동일) | ✅ 우수 |
| 이메일 인증 | `"AUTH_401_INVALID_OR_EXPIRED_CODE"` 통일 (미존재/만료/불일치 동일) | ✅ 우수 |
| 인증 메일 재발송 | `"If the email needs verification..."` 통일 | ✅ 우수 |
| 회원가입 | `"Email already exists"` (409, 의도적 — UX 필요) | ✅ 적절 |
| MFA 로그인 | `"MFA_TOKEN_EXPIRED"` / `"MFA_INVALID_CODE"` 통일 | ✅ |
| ID 찾기 | 마스킹된 이메일 목록 반환 | ✅ |

### 3-4. Rate Limiting — **3건 누락 발견** ⚠️

#### 적용된 Rate Limiting (15건)

| # | 엔드포인트 | Redis 키 패턴 | 방식 | 한도 | 이메일 DECR |
|---|-----------|--------------|------|------|------------|
| 1 | 로그인 | `rl:login:{idx}:{ip}` | 이메일+IP | 10/15분 | — |
| 2 | 회원가입 | `rl:signup:{idx}:{ip}` | 이메일+IP | 10/15분 | — |
| 3 | ID 찾기 | `rl:find_id:{ip}` | IP | 10/15분 | — |
| 4 | 비밀번호 찾기 | `rl:find_password:{ip}` | IP | 5/5시간 | ✅ |
| 5 | 비밀번호 재설정 (레거시) | `rl:reset_pw:{ip}` | IP | 10/15분 | — |
| 6 | 재설정 요청 | `rl:request_reset:{idx}:{ip}` | 이메일+IP | 5/5시간 | ✅ |
| 7 | 재설정 코드 검증 | `rl:verify_reset:{idx}:{ip}` | 이메일+IP | 5/5시간 | — |
| 8 | 이메일 인증 | `rl:verify_email:{idx}:{ip}` | 이메일+IP | 10/1시간 | — |
| 9 | 인증 메일 재발송 | `rl:resend_verify:{idx}:{ip}` | 이메일+IP | 5/5시간 | ✅ |
| 10 | MFA 로그인 | `rl:mfa:{user_id}:{ip}` | 사용자+IP | 5/5분 | — |
| 11 | 스터디 정답 제출 | 사용자 기반 | 사용자 | 30/60초 | — |
| 12 | E-book 구매 | 사용자 기반 | 사용자 | 5/1시간 | — |
| 13 | E-book 페이지 | 사용자 기반 | 사용자 | 30/60초 | — |
| 14 | E-book 타일 | 사용자 기반 | 사용자 | 270/60초 | — |
| 15 | 교재 주문 | IP 기반 | IP | 5/1시간 | — |

**이메일 실패 시 DECR 롤백**: 3건 모두 정상 구현 확인 ✅

#### 누락된 Rate Limiting

| # | 엔드포인트 | 위치 | 위험 | 심각도 |
|---|-----------|------|------|--------|
| 1 | `POST /auth/mfa/verify-setup` | `auth/service.rs:1720` | TOTP 코드 brute-force (6자리 = 100만 가지) | **HIGH** |
| 2 | `POST /auth/mfa/disable` | `auth/service.rs:1912` | 반복 시도로 MFA 비활성화 | **MEDIUM** |
| 3 | `POST /auth/mfa/setup` | `auth/service.rs:1673` | 반복 요청으로 MFA 비밀키 재생성 | **LOW** |

**참고**: `GET /auth/google/callback`은 OAuth state 검증이 일회용 토큰이므로 별도 rate limit 불필요.

### 3-5. 암호화 (PII + 인증코드) — **통과** ✅

#### PII 암호화

| 점검 항목 | 상태 | 상세 |
|-----------|------|------|
| AES-256-GCM 사용 | ✅ | `crates/crypto/src/cipher.rs` — 12바이트 랜덤 nonce, OsRng |
| AAD (Associated Authenticated Data) | ✅ | 필드별 고유 AAD (`"users.user_email"` 등) — ciphertext 스와핑 방지 |
| 키 버전 관리 | ✅ | `KeyRing` 다중 버전 지원, 암호문에 `enc:v{N}:` 버전 포함 |
| Blind Index (HMAC-SHA256) | ✅ | 대소문자 정규화 후 해시, DB 유니크 인덱스 적용 |
| 암호화 대상 필드 | ✅ | email, name, birthday, OAuth subject, MFA secret, MFA backup codes, login IP |
| 평문 저장 필드 | ✅ 적절 | nickname, country, language, gender — PII 해당 안 됨 |

#### 인증코드 저장

| 점검 항목 | 상태 | 상세 |
|-----------|------|------|
| 이메일 인증코드 → HMAC 해시 저장 | ✅ | `UserService::hmac_verification_code()` → Redis `ak:email_verify:{idx}` |
| 비밀번호 재설정 코드 → HMAC 해시 저장 | ✅ | 동일 함수 → Redis `ak:reset_code:{idx}` |
| MFA 백업 코드 → SHA256 해시 + AES-GCM 암호화 | ✅ | DB `users.user_mfa_backup_codes` 이중 보호 |
| Constant-time 비교 | ✅ | `constant_time_eq()` — XOR fold 방식 타이밍 공격 방지 |
| 일회용 삭제 | ✅ | 성공 후 Redis `DEL` / 백업 코드 목록에서 제거 |
| Redis 키에 평문 이메일 미노출 | ✅ | blind index 사용 |

### 3-6. CORS — **통과** ✅

| 점검 항목 | 상태 | 상세 |
|-----------|------|------|
| Origin 제한 | ✅ | `CORS_ORIGINS` 환경변수 기반, 와일드카드(`*`) 아님 |
| 허용 메서드 | ✅ | GET, POST, PUT, PATCH, DELETE, OPTIONS 명시적 지정 |
| 허용 헤더 | ✅ | Authorization, Content-Type, Accept + E-book 커스텀 (`x-ebook-*`) |
| Credentials | ✅ | `allow_credentials(true)` — refresh token 쿠키 교환용 |

**위치**: `src/main.rs:138-157`

### 3-7. 환경변수 / 하드코딩 시크릿 — **통과** ✅

| 점검 항목 | 상태 | 상세 |
|-----------|------|------|
| 모든 시크릿 환경변수 로드 | ✅ | JWT_SECRET, ENCRYPTION_KEY_V*, HMAC_KEY, API 키 전부 `env::var()` |
| JWT_SECRET 최소 길이 검증 | ✅ | `config.rs` — 32바이트 미만 시 panic |
| `.env` gitignore | ✅ | `.env`, `.env.local`, `.env.*.local`, `.env.*` 제외 |
| 하드코딩 키/토큰 패턴 | ✅ | `sk_*`, `pk_*`, Bearer 토큰 문자열 등 미발견 |

### 3-8. 에러 메시지 정보 노출 — **통과** ✅

| 점검 항목 | 상태 | 상세 |
|-----------|------|------|
| DB 에러 → 클라이언트 | ✅ | `"Database error"` 고정 반환 (`error.rs:150-158`) |
| Redis 에러 → 클라이언트 | ✅ | `"Cache error"` 고정 반환 (`error.rs:170-178`) |
| 외부 API 에러 → 클라이언트 | ✅ | `"External service error"` 고정 반환 (`error.rs:140-148`) |
| 내부 에러 → 클라이언트 | ✅ | `"Internal server error"` 고정 반환 (`error.rs:64-72`) |
| 스택 트레이스 노출 | ✅ | 미노출 — Debug 포맷(`:?`) HTTP 응답에 미사용 |

**참고**: 외부 API raw 응답은 `tracing::error!()` 서버 로그에만 기록 (email.rs:97, vimeo.rs:100, google.rs:133). 클라이언트에는 도달하지 않음.

### 3-9. 프론트엔드: XSS — **통과** ✅

| 점검 항목 | 상태 | 상세 |
|-----------|------|------|
| `dangerouslySetInnerHTML` | ✅ | 0건 — 사용처 없음 |
| `innerHTML` / `__html` | ✅ | 0건 |
| DOMPurify 필요성 | — | 현재 불필요 (유저 입력 HTML 렌더링 없음) |

### 3-10. 프론트엔드: 토큰 저장 — **1건 INFO**

| 점검 항목 | 상태 | 상세 |
|-----------|------|------|
| Access Token 저장 | ⚠️ INFO | `localStorage` (Zustand persist) — XSS 취약점 존재 시 노출 가능 |
| Refresh Token 저장 | ✅ | httpOnly 쿠키 (서버 설정) — XSS로 접근 불가 |
| 토큰 전송 | ✅ | `Authorization: Bearer` 헤더 + `withCredentials: true` |

**판정**: Access Token localStorage 저장은 업계 일반적 패턴. 현재 XSS 벡터 부재 (dangerouslySetInnerHTML 0건, 유저 HTML 렌더링 없음) → **실질 위험 낮음**. 추후 유저 입력 HTML 렌더링 추가 시 재검토.

### 3-11. 프론트엔드: HMAC 서명 — **통과** ✅

| 점검 항목 | 상태 | 상세 |
|-----------|------|------|
| 서명 알고리즘 | ✅ | HMAC-SHA256 (Web Crypto API `crypto.subtle`) |
| 서명 페이로드 | ✅ | `{sessionId}:{path}:{timestamp}` — 리플레이 공격 방지 |
| 적용 대상 | ✅ | E-book 페이지/타일 요청 (`X-Ebook-Signature`, `X-Ebook-Timestamp`) |
| HMAC Secret 전달 | ✅ | 뷰어 메타데이터 API에서 세션별 발급 |

**위치**: `frontend/src/category/ebook/ebook_api.ts:34-52`

### 3-12. 프론트엔드: DevTools 감지 — **INFO** (설계대로)

| 점검 항목 | 상태 | 상세 |
|-----------|------|------|
| 감지 방식 | ✅ | console.log getter 트랩 + 윈도우 크기 비교 |
| 우회 가능성 | ⚠️ 인지됨 | 코드 주석에 명시: "결정적 공격자는 우회 가능. 억제(deterrent) 목적" |
| 감지 시 동작 | ✅ | E-book 콘텐츠 blur 처리 |

**판정**: 억제 목적의 설계. E-book 보안은 서버 측 HMAC + 워터마크 + Rate Limit이 핵심 방어선.

---

## 점검 3 종합 판정

| 구분 | 통과 | 발견 | 심각도 |
|------|------|------|--------|
| SQL Injection | ✅ | 0건 | — |
| 인증/인가 | ⚠️ | **1건** (`POST /courses` 인증 없음) | HIGH |
| Anti-enumeration | ⚠️ | **2건** (에러 메시지 사용자 존재 노출) | MEDIUM |
| Rate Limiting | ⚠️ | **3건** (MFA setup/verify/disable 누락) | HIGH/MEDIUM |
| PII 암호화 | ✅ | 0건 | — |
| 인증코드 해시 저장 | ✅ | 0건 | — |
| CORS | ✅ | 0건 | — |
| 하드코딩 시크릿 | ✅ | 0건 | — |
| 에러 메시지 노출 | ✅ | 0건 | — |
| XSS | ✅ | 0건 | — |
| 토큰 저장 | ✅ | 0건 (INFO 1건) | — |
| HMAC 서명 | ✅ | 0건 | — |
| **합계** | **9/12 통과** | **6건** | — |

### 핵심 발견 (수정 필요 6건)

| 우선순위 | 문제 | 위치 | 수정 방법 |
|----------|------|------|-----------|
| 🔴 HIGH | `POST /courses` 인증 없음 | `course/handler.rs:23` | `AuthUser` extractor 추가 (또는 admin 전용으로 이동) |
| 🔴 HIGH | MFA verify-setup rate limit 누락 | `auth/service.rs:1720` | `rl:mfa_setup:{user_id}` 추가 (5회/5분) |
| 🟡 MEDIUM | MFA disable rate limit 누락 | `auth/service.rs:1912` | `rl:mfa_disable:{user_id}` 추가 |
| 🟡 MEDIUM | verify_reset_code 사용자 존재 노출 | `auth/service.rs:1014` | `→ "AUTH_401_INVALID_OR_EXPIRED_CODE"` |
| 🟡 MEDIUM | refresh 사용자 존재 노출 | `auth/service.rs:479` | `→ "AUTH_401_INVALID_REFRESH"` |
| 🟢 LOW | MFA setup rate limit 누락 | `auth/service.rs:1673` | `rl:mfa_setup:{user_id}` 추가 (5회/1시간) |

### 보안 강점 (양호 사항)

1. **암호화 체계 우수** — AES-256-GCM + AAD + 키 버전 관리 + blind index 전체 구현
2. **인증코드 보안 우수** — HMAC 해시 + constant-time 비교 + 일회용 삭제
3. **Rate Limiting 체계적** — 15개 엔드포인트 적용, 이메일 DECR 롤백 3건 모두 정상
4. **에러 메시지 sanitization** — DB/Redis/외부 API 에러 모두 고정 메시지 반환
5. **XSS 벡터 부재** — dangerouslySetInnerHTML 0건, 유저 HTML 렌더링 없음
6. **Admin 3중 보호** — JWT + Role + IP Guard 전체 적용

---

## 점검 4: 문서 정합성 — docs ↔ 코드 대조 (2026-04-03)

### 4-1. AUTH 문서 (`AMK_API_AUTH.md`) — **3건 발견**

| # | 유형 | 위치 | 문서 내용 | 코드 실제 | 심각도 |
|---|------|------|-----------|-----------|--------|
| 1 | **DTO 필드 오류** | §5.3-4 `FindIdReq` | "식별 정보(이름 + **이메일**)" | `name + birthday` (`dto.rs:50-54`) | **HIGH** — 문서 오기재 |
| 2 | **상태 코드 불일치** | §3-2a `LogoutAll` | 성공 → **204** No Content | **200** OK + `LogoutRes` body (`handler.rs:367`) | **MEDIUM** |
| 3 | **상태 코드 불일치** | §5.3-4 `FindId` | 실패 → **422** | 422 미구현, 400만 사용 | **LOW** — 문서에서 422 삭제 가능 |

### 4-2. USER 문서 (`AMK_API_USER.md`) — **1건 발견**

| # | 유형 | 위치 | 문서 내용 | 코드 실제 | 심각도 |
|---|------|------|-----------|-----------|--------|
| 1 | **HTTP 메서드 누락** | §5.2-3 `/users/me` 수정 | `POST` only | `PUT` + `POST` 둘 다 (`router.rs:12`) | **LOW** — 문서에 PUT 추가 필요 |

### 4-3. LEARNING 문서 (`AMK_API_LEARNING.md`) — **3건 발견**

| # | 유형 | 위치 | 문서 내용 | 코드 실제 | 심각도 |
|---|------|------|-----------|-----------|--------|
| 1 | **엔드포인트 미문서화** | Health | `/healthz`만 문서화 | `/health`, `/ready` 도 존재 (`api/mod.rs:55-56`) | **LOW** |
| 2 | **엔드포인트 미문서화** | Translation | — | `GET /admin/translations/stats` 존재 | **LOW** |
| 3 | **엔드포인트 일치** | Video, Study, Lesson, Course | — | 전부 일치 ✅ | — |

### 4-4. PAYMENT 문서 (`AMK_API_PAYMENT.md`) — **4건 발견**

| # | 유형 | 위치 | 문서 내용 | 코드 실제 | 심각도 |
|---|------|------|-----------|-----------|--------|
| 1 | **DTO 필드 누락** | §11-2 `SubscriptionInfo` | `currency` 필드 포함 | `currency` 미구현 (`payment/dto.rs:49-60`) | **MEDIUM** |
| 2 | **DTO 필드 누락** | §10-2 `AdminSubDetail` | `provider`, `currency` 포함 | 두 필드 미구현 (`admin/payment/dto.rs:66-91`) | **MEDIUM** |
| 3 | **DTO 필드 불일치** | §10-6 `AdminGrantSummary` | `user_nickname`, `active_courses`, `earliest_enrolled`, `latest_expire` | `course_count` (이름 다름), 3개 필드 미구현 (`admin/payment/dto.rs:163-170`) | **MEDIUM** |
| 4 | **DTO 필드 누락** | §10-5 `AdminGrantRes` | `granted_by`, `reason`, `created_at` | 3개 필드 미구현 (`admin/payment/dto.rs:150-155`) | **LOW** |

### 4-5. TEXTBOOK 문서 (`AMK_API_TEXTBOOK.md`) — **1건 발견**

| # | 유형 | 위치 | 문서 내용 | 코드 실제 | 심각도 |
|---|------|------|-----------|-----------|--------|
| 1 | **상태 코드 불일치** | `POST /textbook/orders` | **201** Created | **200** OK (`textbook/handler.rs:41`) | **LOW** |

### 4-6. EBOOK 문서 (`AMK_API_EBOOK.md`) — **2건 발견**

| # | 유형 | 위치 | 문서 내용 | 코드 실제 | 심각도 |
|---|------|------|-----------|-----------|--------|
| 1 | **엔드포인트 미문서화** | Admin | — | `GET /admin/ebook/verify/{watermark_id}` 미문서화 | **MEDIUM** |
| 2 | **페이지네이션 이름 불일치** | Admin 목록 | — | ebook `page` vs textbook `current_page` (도메인 간 불일치) | **LOW** |

### 4-7. SCHEMA 문서 (`AMK_SCHEMA_PATCHED.md`) — **9건 발견**

#### 테이블 수준

| # | 유형 | 문서 | 마이그레이션 | 심각도 |
|---|------|------|-------------|--------|
| 1 | **테이블 누락** | `user_oauth` 미기재 | `20260208_AMK_V1.sql:83-94`에 존재 | **HIGH** |
| 2 | **테이블명 오류** | `study_task_explain` | 실제: `study_explain` (`20260208_AMK_V1.sql:313`) | **HIGH** |

#### 컬럼 수준

| # | 유형 | 테이블 | 컬럼 | 문서 | 마이그레이션 | 심각도 |
|---|------|--------|------|------|-------------|--------|
| 3 | **컬럼 누락** | `users` | `user_email_idx` | 미기재 | TEXT NOT NULL | **HIGH** |
| 4 | **컬럼 누락** | `users` | `user_name_idx` | 미기재 | TEXT NOT NULL | **HIGH** |
| 5 | **Nullable 불일치** | `users` | `user_password` | NOT NULL | nullable (OAuth 사용자) | **MEDIUM** |
| 6 | **타입 불일치** | `users` | `user_email` | varchar(255) | TEXT (암호화 → 길이 예측 불가) | **MEDIUM** |
| 7 | **타입 불일치** | `users` | `user_name`, `user_birthday` | varchar/date | TEXT (암호화 저장) | **MEDIUM** |
| 8 | **타입 불일치** | 3개 테이블 | IP 관련 컬럼 (`login_ip`, `login_ip_log`, `ip_address`) | INET | TEXT (암호화 저장) | **MEDIUM** |
| 9 | **타입 불일치** | `users_log` | `user_birthday_log` | date | TEXT (암호화 저장) | **MEDIUM** |

**공통 원인**: 암호화 구현 후 `TEXT`로 변경되었으나 스키마 문서가 미갱신. blind index 컬럼(`_idx`)도 암호화 추가 시 신설.

### 4-8. 환경변수 (`config.rs` ↔ `AMK_DEPLOY_OPS.md` ↔ `docker-compose.prod.yml`) — **10건 발견**

#### 코드에 있지만 문서에 없는 환경변수

| # | 변수명 | 위치 | 비고 |
|---|--------|------|------|
| 1 | `JWT_EXPIRE_HOURS` | `config.rs:108` | 기본값 24 |
| 2 | `SKIP_DB` | `config.rs:113` | 기본값 false |
| 3 | `RATE_LIMIT_STUDY_WINDOW_SEC` | `config.rs:150` | 기본값 60 |
| 4 | `RATE_LIMIT_STUDY_MAX` | `config.rs:154` | 기본값 30 |
| 5 | `RATE_LIMIT_TEXTBOOK_WINDOW_SEC` | `config.rs:173` | 기본값 3600 |
| 6 | `RATE_LIMIT_TEXTBOOK_MAX` | `config.rs:177` | 기본값 5 |
| 7 | `EBOOK_PAGE_IMAGES_DIR` | `config.rs:307` | 기본값 "docs/textbook/page-images" |
| 8 | `EBOOK_IMAGES_ENCRYPTED` | `config.rs:349` | 기본값 false |
| 9 | ~~`ENCRYPTION_KEY` (레거시)~~ | ~~`config.rs:429`~~ | **FALSE POSITIVE** — `ENCRYPTION_KEY_V1` 하위 호환 폴백, 의도적 미문서화 |
| 10 | `ENCRYPTION_KEY_V2~V255` | `config.rs:413` | 키 로테이션용, docker에 V1만 |

> **전수 검증 결과 (2026-04-03)**: 환경변수 10건 중 #9 `ENCRYPTION_KEY`는 FALSE POSITIVE. 실제 발견 **9건**.

#### 코드에 있지만 docker-compose에 없는 환경변수

| # | 변수명 | 비고 |
|---|--------|------|
| 1 | `RATE_LIMIT_TEXTBOOK_WINDOW_SEC` | 기본값 사용됨 |
| 2 | `RATE_LIMIT_TEXTBOOK_MAX` | 기본값 사용됨 |
| 3 | `JWT_EXPIRE_HOURS` | 기본값 사용됨 |
| 4 | `EBOOK_PAGE_IMAGES_DIR` | 기본값 사용됨 |
| 5 | `EBOOK_IMAGES_ENCRYPTED` | 기본값 사용됨 |

---

## 점검 4 종합 판정

| 문서 | 엔드포인트 일치 | DTO 일치 | 상태코드 일치 | 발견 건수 |
|------|----------------|---------|-------------|-----------|
| AUTH | ✅ (19/19) | ⚠️ 1건 | ⚠️ 2건 | **3** |
| USER | ✅ (18/18) | ✅ | ✅ | **1** |
| LEARNING | ⚠️ 2건 미문서화 | ✅ | ✅ | **3** |
| PAYMENT | ✅ (14/14) | ⚠️ 4건 | ✅ | **4** |
| TEXTBOOK | ✅ (8/8) | ✅ | ⚠️ 1건 | **1** |
| EBOOK | ⚠️ 1건 미문서화 | ✅ | ✅ | **2** |
| SCHEMA | ⚠️ 2건 | ⚠️ 7건 | — | **9** |
| 환경변수 | — | — | — | **9** (1건 FALSE POSITIVE 제외) |
| **합계** | | | | **32** |

### 핵심 발견 (우선순위별)

#### HIGH (5건) — 즉시 문서 수정

| # | 문서 | 문제 | 수정 방법 |
|---|------|------|-----------|
| 1 | AUTH | FindIdReq "이메일" → 실제 "birthday" | 문서에서 "이메일" → "생년월일" 수정 |
| 2 | SCHEMA | `user_oauth` 테이블 누락 | 스키마 문서에 테이블 추가 |
| 3 | SCHEMA | `study_task_explain` → 실제 `study_explain` | 테이블명 수정 |
| 4 | SCHEMA | `user_email_idx`, `user_name_idx` 컬럼 누락 | 스키마 문서에 blind index 컬럼 추가 |
| 5 | SCHEMA | 암호화 필드 타입 (varchar/date/INET → TEXT) | 타입을 TEXT로 일괄 수정, 주석에 암호화 명시 |

#### MEDIUM (8건) — 다음 작업 시 수정

| # | 문서 | 문제 |
|---|------|------|
| 1 | AUTH | LogoutAll 204 → 200 (문서 또는 코드 수정) |
| 2 | PAYMENT | SubscriptionInfo `currency` 필드 미구현 |
| 3 | PAYMENT | AdminSubDetail `provider`, `currency` 미구현 |
| 4 | PAYMENT | AdminGrantSummary 필드 4개 불일치 |
| 5 | EBOOK | `/admin/ebook/verify/{watermark_id}` 미문서화 |
| 6 | SCHEMA | `user_password` nullable 불일치 |
| 7 | SCHEMA | IP 컬럼 INET → TEXT 미반영 |
| 8 | SCHEMA | `user_birthday_log` date → TEXT 미반영 |

#### LOW (20건) — 환경변수 문서 보완 + 기타

| 구분 | 건수 |
|------|------|
| 환경변수 문서 미기재 | 9건 (1건 FALSE POSITIVE 제외) |
| 환경변수 docker-compose 미기재 | 5건 |
| LEARNING 미문서화 엔드포인트 | 2건 |
| USER PUT 메서드 미문서화 | 1건 |
| TEXTBOOK 201→200 상태코드 | 1건 |
| PAYMENT AdminGrantRes 필드 | 1건 |
