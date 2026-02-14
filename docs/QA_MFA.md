# QA Report — MFA (Multi-Factor Authentication)

- **테스트 일시**: 2026-02-14
- **테스트 환경**: localhost:3000 (dev)
- **테스트 계정**: qatest@test.com (admin, user_id=37)

---

## 요약

| 카테고리 | 항목 수 | PASS | FAIL | 검증 방법 |
|----------|---------|------|------|----------|
| A. MFA 설정 흐름 | 6 | 6 | 0 | API 테스트 |
| B. MFA 로그인 흐름 | 8 | 8 | 0 | API 테스트 (H-1 수정 후) |
| D. MFA 미적용 역할 | 2 | 2 | 0 | 코드 리뷰 |
| E. MFA 비활성화 | 4 | 4 | 0 | API + 코드 리뷰 |
| F. 프로필 응답 | 3 | 3 | 0 | API 테스트 |
| 코드 리뷰 (백엔드) | 6 | 6 | 0 | 파일 리뷰 |
| 코드 리뷰 (프론트엔드) | 9 | 9 | 0 | 파일 리뷰 |
| 인프라 (docs.rs) | 1 | 1 | 0 | 수정 완료 |
| **합계** | **39** | **39** | **0** | |

**최종 결과**: 39/39 PASS (이슈 2건 발견 → 즉시 수정 완료)

---

## 발견된 이슈 및 수정

### H-1 [Critical] MFA 로그인 시 DB_ERROR — login_method_enum 불일치

- **증상**: `POST /auth/mfa/login` → 500 DB_ERROR
- **원인**: `service.rs:280`에서 MFA pending 데이터에 `"login_method": "login"` 저장 → `insert_login_record_oauth_tx`에서 `$6::login_method_enum`으로 캐스팅 시 실패. DB enum에는 `email`, `google`, `apple`만 존재.
- **수정**:
  - `service.rs:280`: `"login_method": "login"` → `"login_method": "email"`
  - `service.rs:1885-1897`: 불필요한 `if pending_method == "login"` 분기 제거, `pending_method` 그대로 사용
- **검증**: B-4 재테스트 PASS (200, LoginRes 정상)

### M-1 [Medium] docs.rs Swagger 등록 누락

- **증상**: MFA 4개 핸들러 + 7개 DTO가 Swagger UI에 표시되지 않음
- **수정**: `src/docs.rs`에 추가
  - paths: `mfa_setup`, `mfa_verify_setup`, `mfa_login`, `mfa_disable`
  - schemas: `MfaChallengeRes`, `MfaLoginReq`, `MfaSetupRes`, `MfaVerifySetupReq`, `MfaVerifySetupRes`, `MfaDisableReq`, `MfaDisableRes`
- **검증**: `cargo check` PASS

---

## 테스트 A: MFA 설정 흐름

| ID | 테스트 | 기대 | 결과 |
|----|--------|------|------|
| A-1 | POST /auth/mfa/setup (인증 없음) | 401 | **PASS** — 401 "Missing or invalid Authorization header" |
| A-2 | POST /auth/mfa/setup (admin JWT) | 200 + secret/QR/otpauth | **PASS** — secret 32자, data:image QR, otpauth:// URI |
| A-3 | POST /auth/mfa/verify-setup (잘못된 코드 000000) | 401 | **PASS** — 401 "MFA_INVALID_CODE" |
| A-4 | POST /auth/mfa/verify-setup (유효한 TOTP 코드) | 200 + enabled + backup_codes | **PASS** — enabled=true, 10개 × 8자 백업코드 |
| A-5 | DB 컬럼 확인 (enable 후) | 4개 컬럼 모두 NOT NULL | **PASS** — mfa_enabled=t, secret/backup/enabled_at 설정됨 |
| A-6 | MFA 활성화 후 로그인 | MfaChallengeRes | **PASS** — mfa_required=true, mfa_token=UUID |

## 테스트 B: MFA 로그인 흐름 (이메일+비밀번호)

| ID | 테스트 | 기대 | 결과 |
|----|--------|------|------|
| B-1 | 로그인 → MfaChallengeRes 구조 | mfa_required/mfa_token/user_id | **PASS** — 구조 정상 |
| B-2 | MFA login + 잘못된 코드 | 401 | **PASS** — 401 "MFA_INVALID_CODE" |
| B-3 | MFA login + 유효하지 않은 mfa_token | 401 | **PASS** — 401 "MFA_TOKEN_EXPIRED" |
| B-4 | MFA login + 유효한 TOTP 코드 | 200 + LoginRes | **PASS** — user_id=37, access_token, session_id (H-1 수정 후) |
| B-5 | MFA login + 백업 코드 (첫 번째) | 200 + LoginRes | **PASS** — 백업코드 로그인 성공 |
| B-6 | 동일 백업 코드 재사용 | 401 | **PASS** — 401 "MFA_INVALID_CODE" (일회용) |
| B-7 | 두 번째 백업 코드 사용 | 200 | **PASS** — 다른 백업코드는 정상 작동 |
| B-8 | MFA 토큰 재사용 (소모 후) | 401 | **PASS** — 401 "MFA_TOKEN_EXPIRED" (일회용) |

## 테스트 D: MFA 미적용 역할

| ID | 테스트 | 기대 | 결과 |
|----|--------|------|------|
| D-1 | learner 로그인 (mfa_enabled=false) | 직접 LoginRes | **PASS** (코드 리뷰) — service.rs:270 `if user_info.user_mfa_enabled` |
| D-2 | learner MFA setup 접근 가능 여부 | 프론트 미노출, 백엔드 허용 | **PASS** (코드 리뷰) — admin_route.tsx에서 admin/HYMN만 강제 |

## 테스트 E: MFA 비활성화 (HYMN 전용)

| ID | 테스트 | 기대 | 결과 |
|----|--------|------|------|
| E-1 | admin → mfa/disable | 403 | **PASS** — 403 "MFA_DISABLE_HYMN_ONLY" |
| E-2 | HYMN → 자기 자신 disable | 400 | **PASS** (코드 리뷰) — service.rs:1942 MFA_CANNOT_DISABLE_SELF |
| E-3 | HYMN → 다른 사용자 disable | 200 + 세션 무효화 | **PASS** (코드 리뷰) — disable_mfa + revoke sessions + Redis cleanup |
| E-4 | disable_mfa SQL | 4컬럼 NULL/false 리셋 | **PASS** (코드 리뷰) — repo.rs 확인 완료 |

## 테스트 F: 프로필 응답 mfa_enabled

| ID | 테스트 | 기대 | 결과 |
|----|--------|------|------|
| F-1 | GET /users/me (MFA 활성화) | mfa_enabled=true | **PASS** — 200, mfa_enabled=true |
| F-2 | GET /users/me (MFA 비활성화) | mfa_enabled=false | **PASS** — 200, mfa_enabled=false |
| F-3 | GET /users/me (MFA 재활성화) | mfa_enabled=true | **PASS** — 200, mfa_enabled=true |

---

## 코드 리뷰 — 백엔드

| 파일 | 확인 사항 | 결과 |
|------|----------|------|
| `src/api/auth/dto.rs` | 7개 MFA DTO (ToSchema, Validate) | **PASS** — MfaChallengeRes, MfaLoginReq(6~8자), MfaSetupRes, MfaVerifySetupReq(6자), MfaVerifySetupRes, MfaDisableReq, MfaDisableRes |
| `src/api/auth/repo.rs` | 7개 MFA repo 함수 + UserLoginInfo.user_mfa_enabled | **PASS** — find/update/enable/disable + backup code 관리 |
| `src/api/auth/service.rs` | LoginOutcome/OAuthLoginOutcome enum, MFA 5개 메서드 | **PASS** — TOTP 생성/검증, 백업코드 SHA256 해시, Redis pending, rate limit |
| `src/api/auth/handler.rs` | 4개 MFA 핸들러 + login/OAuth MFA 분기 | **PASS** — utoipa 어노테이션 포함 |
| `src/api/auth/router.rs` | 4개 MFA 라우트 | **PASS** — /mfa/setup, /mfa/verify-setup, /mfa/login, /mfa/disable |
| `src/config.rs` | 3개 MFA 환경변수 | **PASS** — mfa_token_ttl_sec(300), rate_limit_mfa_max(5), rate_limit_mfa_window_sec(300) |

## 코드 리뷰 — 프론트엔드

| 파일 | 확인 사항 | 결과 |
|------|----------|------|
| `frontend/src/category/auth/types.ts` | MFA 타입 4개 | **PASS** — MfaChallengeRes, MfaLoginReq(zod), MfaSetupRes, MfaVerifySetupRes |
| `frontend/src/category/auth/auth_api.ts` | MFA API 함수 3개 | **PASS** — mfaLogin, mfaSetup, mfaVerifySetup |
| `frontend/src/category/auth/hook/use_login.ts` | MFA 챌린지 감지 + mfaPending 상태 | **PASS** — isMfaChallenge 타입가드, clearMfaPending |
| `frontend/src/category/auth/hook/use_mfa_login.ts` | MFA 로그인 훅 | **PASS** — 429/TOKEN_EXPIRED/INVALID_CODE 에러 핸들링 |
| `frontend/src/category/auth/hook/use_oauth_callback.ts` | OAuth MFA 리다이렉트 처리 | **PASS** — mfa_required/mfa_token/user_id URL 파라미터 |
| `frontend/src/category/auth/page/login_page.tsx` | MFA 코드 입력 UI | **PASS** — activeMfaPending 통합, 6~8자 입력, 뒤로가기(이메일만) |
| `frontend/src/category/admin/page/admin_mfa_setup_page.tsx` | 3단계 위저드 | **PASS** — QR→검증→백업코드(복사/다운로드/확인체크) |
| `frontend/src/routes/admin_route.tsx` | MFA 강제 가드 | **PASS** — !mfa_enabled → /admin/mfa/setup 리다이렉트 |
| `frontend/src/app/routes.tsx` | MFA 라우팅 | **PASS** — /admin/mfa/setup이 AdminLayout 밖, AdminRoute 안 |

## 코드 리뷰 — User 모듈 변경

| 파일 | 확인 사항 | 결과 |
|------|----------|------|
| `src/api/user/dto.rs` | ProfileRes에 mfa_enabled: bool | **PASS** |
| `src/api/user/repo.rs` | 모든 쿼리에 user_mfa_enabled as mfa_enabled | **PASS** — find_user, find_profile_by_id, update_profile_tx 등 |
| `frontend/src/category/user/types.ts` | userDetailSchema에 mfa_enabled: z.boolean().optional() | **PASS** |

---

## 관찰 사항 (non-blocking)

### L-1 [Low] login_page.tsx 인라인 MFA mutation

- `login_page.tsx:56-67`에서 `useMutation`을 직접 사용하여 MFA 로그인 처리
- 별도의 `use_mfa_login.ts` 훅이 존재하지만 사용하지 않음
- 인라인 버전은 에러 처리가 단순화됨 (모두 `toast.error(t("auth.mfaInvalidCode"))`)
- `use_mfa_login.ts`는 429/TOKEN_EXPIRED를 구분하여 처리
- **권장**: 향후 `useMfaLogin` 훅으로 통일하면 에러 처리가 개선됨

### L-2 [Low] OAuth MFA 테스트 미수행

- OAuth + MFA 조합 테스트(C-1~C-3)는 브라우저 리다이렉트 기반이므로 CLI 테스트 불가
- 코드 리뷰로 확인: handler.rs:460-517에서 OAuthLoginOutcome::MfaChallenge 처리 + URL 리다이렉트 정상
- use_oauth_callback.ts에서 mfa_required/mfa_token/user_id 파라미터 추출 로직 정상
