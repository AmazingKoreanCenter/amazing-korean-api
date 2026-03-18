# AMK_API_AUTH — 인증 API 스펙

> 공통 규칙(인증, 에러, 페이징): [AMK_API_MASTER.md §3](./AMK_API_MASTER.md)
> DB 스키마: [AMK_SCHEMA_PATCHED.md](./AMK_SCHEMA_PATCHED.md)
> 코드 패턴: [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md)

---

### 5.3 Phase 3 — auth ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 3-1 | `POST /auth/login` | `/login` | 로그인 | ***LOGIN/LOGIN_LOG 저장 + Redis 세션/리프레시 발급***<br>성공: Auth pass / Page login init→ready / Form login pristine→dirty→validating→submitting→success / Request login pending→success / Data login present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page login init→ready / Form login pristine→dirty→validating→error.client / Request login pending→error / Data login empty → **400**<br>실패(도메인 제약): Auth pass / Page login init→ready / Form login pristine→dirty→validating→error.client / Request login pending→error / Data login error → **422**<br>실패(자격증명 불일치): Auth stop / Page login ready / Form login error.client / Request login pending→error / Data login error → **401**<br>실패(계정 상태/차단): Auth forbid / Page login ready / Form login error.client / Request login pending→error / Data login error → **403**(또는 **423**)<br>실패(레이트리밋): Auth pass / Page login ready / Form login error.client / Request login pending→error / Data login error → **429** | [✅🆗] |
| 3-2 | `POST /auth/logout` | `/logout` | 로그아웃 | ***세션/리프레시 키 제거, LOGIN_LOG 저장***<br>성공: Auth pass / Page logout ready / Request logout pending→success / Data logout present → **204**(또는 **200**)<br>실패(미인증/세션 없음): Auth stop / Page logout ready / Request logout pending→error / Data logout error → **401** | [✅🆗] |
| 3-2a | `POST /auth/logout/all` | (전역처리) | 전체 로그아웃 | ***사용자의 모든 세션/리프레시 키 일괄 제거, LOGIN_LOG 저장***<br>성공: Auth pass / Request logout_all pending→success → **204**<br>실패(미인증): Auth stop → **401** | [✅] |
| 3-3 | `POST /auth/refresh` | (전역처리) | 토큰 재발급 | ***리프레시 로테이션/검증/재사용탐지 + 로그(rotate)***<br>성공: Auth pass / Page app ready / Request refresh pending→success / Data refresh present → **200**<br>실패(형식/누락): Auth pass / Page app ready / Request refresh pending→error / Data refresh empty → **400**<br>실패(도메인 제약): Auth pass / Page app ready / Request refresh pending→error / Data refresh error → **422**<br>실패(리프레시 무효/만료): Auth stop / Page app ready / Request refresh pending→error / Data refresh error → **401**<br>실패(재사용탐지/위조): Auth forbid / Page app ready / Request refresh pending→error / Data refresh error → **409**(또는 **403**) | [✅🆗] |
| 3-4 | `POST /auth/find-id` | `/find-id` | 회원 아이디 찾기 | ***개인정보 보호: 결과 폭로 금지(Enumeration Safe), USERS_LOG 저장***<br>성공(요청 수락/존재 여부와 무관):<br> Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→submitting→success / Request find_id pending→success / Data find_id present → **200**(항상 동일 메시지)<br>실패(형식/누락): Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→error.client / Request find_id pending→error / Data find_id empty → **400**<br>실패(도메인 제약): Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→error.client / Request find_id pending→error / Data find_id error → **422**<br>실패(레이트리밋): Auth pass / Page find_id ready / Form find_id error.client / Request find_id pending→error / Data find_id error → **429** | [✅🆗] |
| 3-5a | `POST /auth/request-reset` | `/reset-password` | 비밀번호 재설정 요청 | ***이메일 기반 인증코드 발송 (Resend), Redis 코드 저장 (TTL 10분)***<br>성공(항상 동일 응답): Auth pass / Request pending→success → **200** `{ message, remaining_attempts }`<br>실패(형식/누락): **400** / 실패(레이트리밋): **429** | [✅🆗] |
| 3-5b | `POST /auth/verify-reset` | `/reset-password` | 비밀번호 재설정 검증 | ***인증코드 검증 + 새 비밀번호 설정, 관련 세션 전부 무효화***<br>성공: Auth pass / Request pending→success → **200**<br>실패(코드 만료/무효): **401** / 실패(형식): **400** / 실패(레이트리밋): **429** | [✅🆗] |
| 3-5 | `POST /auth/reset-pw` | `/reset-password` | 회원 비밀번호 재설정 (legacy) | ***요청→검증→재설정의 단일 엔드포인트(토큰/코드 포함), USERS_LOG 저장***<br>성공(재설정 완료):<br> Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→submitting→success / Request reset_pw pending→success / Data reset_pw present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→error.client / Request reset_pw pending→error / Data reset_pw empty → **400**<br>실패(도메인 제약): Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→error.client / Request reset_pw pending→error / Data reset_pw error → **422**<br>실패(토큰/코드 무효·만료): Auth stop / Page reset_pw ready / Form reset_pw error.client / Request reset_pw pending→error / Data reset_pw error → **401**<br>실패(레이트리밋): Auth pass / Page reset_pw ready / Form reset_pw error.client / Request reset_pw pending→error / Data reset_pw error → **429** | [✅🆗] |
| 3-6 | `GET /auth/google`<br>`GET /auth/google/callback` | `/login` | Google OAuth 로그인 | ***Google OAuth 2.0 Authorization Code Flow, 자동 계정 연결/생성, USER_OAUTH/LOGIN/LOGIN_LOG 저장***<br>성공(OAuth 시작): Auth pass / Page login ready / Request google pending→success / Data google_auth_url present → **200**<br>성공(OAuth 콜백): Auth pass / Page login redirect→ready / Request callback pending→success / Data login present → **302**(프론트엔드 리다이렉트)<br>실패(OAuth 설정 누락): Auth pass / Page login ready / Request google pending→error / Data google error → **500**<br>실패(State 검증 실패/CSRF): Auth stop / Page login ready / Request callback pending→error / Data callback error → **401**<br>실패(사용자 취소): Auth pass / Page login ready / Request callback pending→error / Data callback error → **302**(에러 정보와 함께 리다이렉트) | [✅🆗] |
| 3-7 | `POST /auth/verify-email` | `/verify-email` | 이메일 인증코드 확인 | ***회원가입 이메일 인증, HMAC-SHA256 해시 비교 (constant-time), user_check_email=true 업데이트***<br>성공: **200** `{ message, verified: true }`<br>실패(코드 무효/만료): **401** / 실패(형식): **400** / 실패(레이트리밋): **429** (10회/시간) | [✅] |
| 3-8 | `POST /auth/resend-verification` | `/verify-email` | 이메일 인증코드 재발송 | ***미인증 사용자에게 새 인증코드 발송 (Enumeration Safe — 항상 동일 메시지)***<br>성공: **200** `{ message, remaining_attempts }` (항상 성공 메시지)<br>실패(형식): **400** / 실패(레이트리밋): **429** (5회/5시간) / 실패(이메일 서비스): **503** | [✅] |
| 3-9 | `POST /auth/find-password` | `/account-recovery` | 비밀번호 찾기 (통합) | ***본인확인(이름+생일+이메일) → 인증코드 발송, Enumeration Safe, OAuth 전용 계정도 동일 응답***<br>성공: **200** `{ message, remaining_attempts }` (항상 동일 메시지)<br>실패(형식): **400** / 실패(레이트리밋): **429** (5회/5시간) | [✅] |
| 3-10 | `POST /auth/mfa/setup` | `/admin/mfa/setup` | MFA 설정 시작 | ***TOTP 비밀키 생성 + QR코드 반환, AES-256-GCM 암호화 저장***<br>성공: **200** `{ secret, qr_code_data_uri, otpauth_uri }`<br>실패(미인증): **401** / 실패(이미 활성화): **409** | [✅] |
| 3-11 | `POST /auth/mfa/verify-setup` | `/admin/mfa/setup` | MFA 설정 확인 | ***TOTP 코드 검증 → MFA 활성화 + 백업코드 10개 생성/반환***<br>성공: **200** `{ enabled: true, backup_codes: [...] }`<br>실패(미인증): **401** / 실패(코드 무효): **401** | [✅] |
| 3-12 | `POST /auth/mfa/login` | `/login` | MFA 2단계 인증 | ***MFA 토큰 + TOTP/백업코드 검증 → 세션 완료***<br>성공: **200** `{ access_token, ... }` + Set-Cookie(refresh_token)<br>실패(토큰 만료): **401** / 실패(코드 무효): **401** / 실패(레이트리밋): **429** (5회/5분) | [✅] |
| 3-13 | `POST /auth/mfa/disable` | (관리자) | MFA 비활성화 | ***HYMN 전용: 대상 사용자의 MFA 해제 + 전체 세션 무효화***<br>성공: **200** `{ disabled: true }`<br>실패(미인증): **401** / 실패(권한 없음): **403** | [✅] |

---

<details>
  <summary>5.3 Phase 3 — auth 시나리오 상세 (5.3-1 ~ 5.3-13)</summary>

#### 공통 정책(5.3-1 ~ 5.3-13)
- **에러 바디(고정)**
  `{ "error": { "http_status": 400|401|403|409|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **로그**: 성공/실패 모두 이벤트 기록
  - `LOGIN`(성공 상태), `LOGIN_LOG`(성공/실패, 원인, IP/UA 등), 사용자 관련 변경은 `USERS_LOG`
- **검증 기준**: **400**=형식·누락·파싱, **422**=도메인 제약(길이·패턴·정책 위반)
- **레이트리밋**: 로그인/비번재설정/아이디찾기엔 **429 + Retry-After**
- **보안**: Enumeration Safe(아이디 찾기/재설정은 결과 노출 없이 동일 응답 문구)

---

#### 5.3-1 : `POST /auth/login` (로그인)
- **성공 → 200 OK(또는 204)**
  - When: `/login`에서 이메일/비밀번호 제출(검증 통과)
  - Then: **200**(또는 **204**), 액세스 토큰·리프레시 토큰 발급(쿠키/헤더), Redis 세션 및 리프레시 키 저장, `LOGIN`/`LOGIN_LOG` 기록
  - 상태축: Auth=pass / Page=`login` init→ready / **Form=`login` pristine→dirty→validating→submitting→success** / Request=`login` pending→success / Data=`login` present / Session=active
- **실패(형식/누락) → 400**
  - 예: 이메일 포맷 불일치, 필수 필드 누락, JSON 파싱 실패
  - 상태축: Form=`login` … → error.client / Request … → error / Data=empty
- **실패(도메인 제약) → 422**
  - 예: 허용되지 않은 로그인 방식, 비밀번호 정책 위반(클라이언트 강화 검증)
- **실패(자격증명 불일치) → 401**
  - 예: 이메일 존재하지만 비밀번호 불일치, 계정 없음
  - 상태축: Auth=stop / Form error.client / Data error
- **실패(계정 상태/차단) → 403(또는 423)**
  - 예: user_state≠'on', 임시 잠금(여러 실패 시도 후)
- **실패(레이트리밋) → 429**
  - 헤더: `Retry-After: <seconds>`
- **실패(소셜 전용 계정) → 401** (별도 에러 코드)
  - When: 이메일/비밀번호 로그인 시도, 해당 이메일이 소셜 로그인 전용 계정인 경우
  - Then: **401**, `{ "error": { "code": "UNAUTHORIZED", "message": "AUTH_401_SOCIAL_ONLY_ACCOUNT:google" } }`
  - 프론트엔드 처리: 소셜 로그인 유도 UI 표시 (amber 색상 안내 박스 + Google 로그인 버튼)
  - 상태축: Auth=stop / Form error.client / Data error (socialOnlyError)
- **실패(이메일 미인증) → 403** (별도 에러 코드)
  - When: 이메일/비밀번호 검증 성공했으나, `user_check_email=false`인 경우
  - Then: **403**, `{ "error": { "code": "FORBIDDEN", "message": "AUTH_403_EMAIL_NOT_VERIFIED:user@example.com" } }`
  - 프론트엔드 처리: `/verify-email` 페이지로 이동 (state에 email 전달), 재발송 버튼 사용 가능
  - 상태축: Auth=stop / Form error.client / Data error (emailNotVerifiedError)
  - **OAuth 자동 인증**: 미인증 이메일로 OAuth 로그인 시 `user_check_email=true` 자동 업데이트

---

#### 5.3-2 : `POST /auth/logout` (로그아웃)
- **성공 → 204 No Content(또는 200)**
  - When: 사용자가 로그아웃 트리거
  - Then: **204**, Redis의 세션/리프레시 키 제거, `LOGIN_LOG`(logout 이벤트) 기록
  - 상태축: Auth=pass / Page=`logout` ready / Request=`logout` pending→success / Data=`logout` present / Session=expired
- **실패(미인증/세션 없음) → 401**
  - 예: 유효한 세션/토큰 없이 호출

---

#### 5.3-3 : `POST /auth/refresh` (토큰 재발급)
- **성공 → 200 OK**
  - When: 백그라운드 토큰 만료 임박/만료 후 리프레시 제출
  - Then: **200**, 새 액세스/리프레시 발급(로테이션), Redis: `ak:refresh:<hash> -> <new_session_id>` 갱신, rotate 로그 기록
  - 상태축: Auth=pass / Page=app ready / Request=`refresh` pending→success / Data=`refresh` present / Session=active
- **실패(형식/누락) → 400**
  - 예: 리프레시 토큰 헤더/쿠키 누락
- **실패(도메인 제약) → 422**
  - 예: 허용되지 않은 클라이언트/디바이스 조합
- **실패(무효/만료) → 401**
  - 예: 만료·폐기된 리프레시, 서명 검증 실패
- **실패(재사용탐지/위조) → 409(또는 403)**
  - 정책: 재사용 탐지 시 기존 세션 무효화 + 알림/로그인 강제

---

#### 5.3-4 : `POST /auth/find_id` (회원 아이디 찾기)
- 성공 → **200**
  - When: `/find-id`에서 식별 정보(이름 + 이메일)를 입력하고 제출한다
  - Then: **200**, "일치 시 등록된 이메일로 안내가 발송되었습니다" **같은 문구**로 항상 응답(Enumeration Safe), `USERS_LOG` 기록
  - 상태축: Auth=pass / Page=`find_id` init→ready / Form=`find_id` pristine→dirty→validating→submitting→success / Request=`find_id` pending→success / Data=`find_id` present
- 실패(형식/누락) → **400**
  - 예: 필수 입력 누락, 형식 불일치(글자/숫자/이메일 패턴 등), JSON 파싱 오류
  - 상태축: Auth=pass / Page=`find_id` init→ready / Form=`find_id` … → error.client / Request=`find_id` pending→error / Data=`find_id` empty
- 실패(레이트리밋) → **429**
  - 조건: 과도한 시도 감지 시
  - 헤더: `Retry-After: <seconds>`
  - 상태축: Auth=pass / Page=`find_id` ready / Form=`find_id` error.client / Request=`find_id` pending→error / Data=`find_id` error

---

#### 5.3-5 : `POST /auth/reset_pw` (회원 비밀번호 재설정)
- **성공(재설정 완료) → 200 OK(또는 204)**
  - When: `/reset-password`에서 토큰/코드 + 새 비밀번호 제출
  - Then: **200**(또는 **204**), 비밀번호 해시 갱신, 관련 세션 전부 무효화(보안), `USERS_LOG` 기록
  - 상태축: Auth=pass / Page=`reset_pw` init→ready / **Form=`reset_pw` pristine→dirty→validating→submitting→success** / Request=`reset_pw` pending→success / Data=`reset_pw` present / Session=rotating→active
- **실패(형식/누락) → 400**, **실패(도메인 제약) → 422**
  - 예: 비밀번호 규칙 위반(길이/복잡성), 필수 누락
- **실패(토큰/코드 무효·만료) → 401**
  - 예: 만료 코드, 위조 토큰
- **실패(레이트리밋) → 429**

---

#### 5.3-6 : `GET /auth/google` & `GET /auth/google/callback` (Google OAuth 로그인)

> **개요**: Google OAuth 2.0 Authorization Code Flow를 통한 소셜 로그인. 기존 이메일 계정 자동 연결, 신규 사용자 자동 가입 지원.

**엔드포인트 구성**:
| 엔드포인트 | 설명 |
|-----------|------|
| `GET /auth/google` | OAuth 인증 URL 반환 (state/nonce 포함) |
| `GET /auth/google/callback` | Google 콜백 처리 → 토큰 발급 → 프론트엔드 리다이렉트 |

**DB 테이블**:
- `USER_OAUTH`: OAuth Provider 연결 정보 (user_id, provider, subject, email, name, picture)
- `LOGIN` / `LOGIN_LOG`: 로그인 세션 및 이력 기록 (login_method = 'google')

**보안 정책**:
- **State 파라미터**: Redis에 저장, 일회용 (CSRF 방지)
- **Nonce**: ID Token에 포함, Replay Attack 방지
- **JWKS 서명 검증**: Google JWKS 공개키로 RS256 서명 검증 (kid 매칭)
- **Audience 검증**: ID Token의 aud가 client_id와 일치해야 함
- **Issuer 검증**: `accounts.google.com` 확인

---

##### OAuth 시작 (`GET /auth/google`)
- **성공 → 200 OK**
  - When: 프론트엔드가 "Google로 로그인" 버튼 클릭 시 호출
  - Then: **200**, `{ auth_url: "https://accounts.google.com/o/oauth2/v2/auth?..." }` 반환
  - 처리: State/Nonce 생성 → Redis 저장 (TTL: 300초) → auth_url 구성
  - 상태축: Auth=pass / Page=`login` ready / Request=`google` pending→success / Data=`google_auth_url` present

- **실패(OAuth 설정 누락) → 500**
  - 예: GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, GOOGLE_REDIRECT_URI 환경변수 미설정
  - 상태축: Request=`google` pending→error / Data=`google` error

##### OAuth 콜백 (`GET /auth/google/callback`)
- **성공(로그인/가입 완료) → 302 Redirect**
  - When: Google 인증 완료 후 콜백 도착 (`?code=xxx&state=xxx`)
  - Then: **302**, 프론트엔드 `/login`으로 리다이렉트 (`?login=success&user_id=xxx&is_new_user=true|false`)
  - 처리 순서:
    1. State 검증 (Redis 조회 → 삭제)
    2. Authorization Code → Token 교환 (Google API)
    3. ID Token 디코딩 및 검증 (JWKS RS256 서명, nonce, aud, iss, exp)
    4. 사용자 조회/생성:
       - OAuth subject로 기존 연결 조회 → 있으면 로그인 (`is_new_user=false`)
       - 없으면 이메일로 기존 계정 조회 → 있으면 자동 연결 (`is_new_user=false`)
       - 없으면 신규 계정 생성 (`is_new_user=true`)
    5. 세션 생성 (JWT + Refresh Cookie)
    6. `LOGIN`, `LOGIN_LOG` 기록
  - **신규 OAuth 사용자 기본값**:
    | 필드 | 기본값 | 비고 |
    |------|--------|------|
    | `user_birthday` | `CURRENT_DATE` | 가입일 (미설정 표시용) |
    | `user_gender` | `none` | 미설정 |
    | `user_country` | `Unknown` | 미설정 |
    | `user_language` | `ko` | 한국어 (서비스 기본) |
    | `user_check_email` | `true` | Google 이메일 인증됨 |
    | `user_password` | `NULL` | 소셜 전용 계정 |
  - 상태축: Auth=pass / Page=`login` redirect→ready / Request=`callback` pending→success / Data=`login` present / Session=active

- **실패(State 검증 실패) → 302 Redirect (에러)**
  - 예: 만료된 state, 위조된 state (CSRF 시도)
  - Then: 프론트엔드로 리다이렉트 (`?error=oauth_failed&error_description=AUTH_401_INVALID_OAUTH_STATE`)
  - 상태축: Auth=stop / Request=`callback` pending→error

- **실패(Nonce 검증 실패) → 302 Redirect (에러)**
  - 예: ID Token의 nonce가 저장된 값과 불일치 (Replay Attack)
  - Then: 프론트엔드로 리다이렉트 (`?error=oauth_failed&error_description=AUTH_401_INVALID_NONCE`)

- **실패(사용자 취소) → 302 Redirect (에러)**
  - When: Google 동의 화면에서 사용자가 취소
  - Then: 프론트엔드로 리다이렉트 (`?error=oauth_error&error_description=access_denied: ...`)

##### 응답 스키마

**GoogleAuthUrlRes (OAuth 시작 응답)**
```json
{
  "auth_url": "https://accounts.google.com/o/oauth2/v2/auth?client_id=...&redirect_uri=...&response_type=code&scope=openid+email+profile&state=...&nonce=...&access_type=offline&prompt=consent"
}
```

**OAuth 콜백 성공 시 리다이렉트**
```
302 Found
Location: http://localhost:5173/login?login=success&user_id=123&is_new_user=true
Set-Cookie: ak_refresh=...; Path=/; HttpOnly; ...
```

| 파라미터 | 값 | 설명 |
|----------|-----|------|
| `login` | `success` | 로그인/가입 성공 |
| `user_id` | `123` | 사용자 ID |
| `is_new_user` | `true` / `false` | 신규 가입 여부 |

**프론트엔드 리다이렉트 분기**:
- `is_new_user=true` → `/user/me?welcome=true` (마이페이지 + 환영 메시지)
- `is_new_user=false` → `/about` (소개 페이지)

**OAuth 콜백 실패 시 리다이렉트**
```
302 Found
Location: http://localhost:5173/login?error=oauth_failed&error_description=...
```

---

##### 프론트엔드 OAuth 콜백 처리

**Hook**: `useOAuthCallback` (`frontend/src/category/auth/hook/use_oauth_callback.ts`)

**처리 흐름**:
1. LoginPage 마운트 시 URL 파라미터 확인 (`login`, `is_new_user`, `error`)
2. 에러 파라미터 있으면 → 토스트 에러 메시지 표시
3. 성공 파라미터 있으면:
   - `refreshToken()` 호출하여 access_token 획득
   - `useAuthStore.login()` 호출하여 로그인 상태 저장
   - `is_new_user` 값에 따라 적절한 페이지로 리다이렉트

**경쟁 조건(Race Condition) 처리**:
- axios interceptor와 OAuth 콜백 처리가 동시에 `refreshToken()`을 호출할 수 있음
- Refresh Token Rotation으로 인해 후자가 409 Conflict 발생 가능
- 해결: `refreshToken()` 실패 시 `isLoggedIn` 상태 확인 → true면 리다이렉트 진행

---

#### 5.3-7 : `POST /auth/verify-email` (이메일 인증코드 확인)

> **개요**: 회원가입 시 발송된 이메일 인증코드를 검증하여 `user_check_email=true`로 업데이트

- **성공 → 200 OK**
  - When: `/verify-email` 페이지에서 6자리 인증코드 입력
  - Then: **200**, `{ message, verified: true }`, `user_check_email=true` 업데이트
  - 보안: HMAC-SHA256 해시 비교 (constant-time), Redis 일회용 코드 삭제
- **실패(코드 무효/만료) → 401**
  - 예: 잘못된 코드, Redis TTL 만료 (10분), 이미 사용된 코드
- **실패(형식/누락) → 400**
  - 예: 이메일 형식 불일치, 코드 길이 불일치
- **실패(레이트리밋) → 429**
  - 조건: 10회/시간 초과

---

#### 5.3-8 : `POST /auth/resend-verification` (이메일 인증코드 재발송)

> **개요**: 미인증 사용자에게 새 이메일 인증코드 발송 (Enumeration Safe)

- **성공 → 200 OK**
  - When: `/verify-email` 페이지에서 "재전송" 버튼 클릭
  - Then: **200**, `{ message, remaining_attempts }` (이메일 존재 여부와 무관하게 항상 동일 메시지)
  - 동작: 미인증 사용자만 실제 이메일 발송, 이미 인증된/미존재 이메일은 발송 없이 성공 응답
- **실패(레이트리밋) → 429**
  - 조건: 5회/5시간 초과 (`RATE_LIMIT_EMAIL_WINDOW_SEC`, `RATE_LIMIT_EMAIL_MAX`)
- **실패(이메일 서비스) → 503**
  - 예: 이메일 프로바이더 연결 실패

---

#### 5.3-9 : `POST /auth/find-password` (비밀번호 찾기 — 통합 계정 복구)

> **개요**: 본인확인(이름+생일+이메일) 후 비밀번호 재설정 인증코드 발송. `/account-recovery` 페이지의 "비밀번호 찾기" 탭에서 사용.

- **성공 → 200 OK**
  - When: `/account-recovery` "비밀번호 찾기" 탭에서 이름, 생일, 이메일 입력
  - Then: **200**, `{ message, remaining_attempts }` (항상 동일 메시지, Enumeration Safe)
  - 본인확인: 이름(blind index) + 생일 + 이메일(blind index) 3중 매칭
  - OAuth 전용 계정(`user_password=NULL`): 동일 성공 응답 반환, 이메일 미발송
  - 매칭 실패: 동일 성공 응답 반환, 이메일 미발송 (타이밍 공격 방지)
- **실패(형식/누락) → 400**
  - 예: 필수 필드 누락, 이메일 형식 불일치
- **실패(레이트리밋) → 429**
  - 조건: 5회/5시간 초과 (IP 기반)

##### 프론트엔드 처리
- `/account-recovery` 탭 UI: "아이디 찾기" / "비밀번호 찾기"
- 비밀번호 찾기 탭에 OAuth 경고 문구 표시 (warning 스타일)
- Step 1(본인확인) → Step 2(인증코드 입력) → `POST /auth/verify-reset` → `/reset-password?token=xxx`
- 잔여 발송 횟수 표시, 한도 도달 시 재전송 버튼 비활성화

---

#### 5.3-10 : `POST /auth/mfa/setup` (MFA 설정 시작)
- **인증 필요**: Bearer 토큰 (AuthUser)
- **성공 → 200 OK**
  - TOTP 비밀키 생성 (`totp-rs` gen_secret)
  - AES-256-GCM 암호화 후 `users.user_mfa_secret`에 임시 저장 (enabled=false 상태)
  - QR 코드 data URI 생성 (`totp-rs` qr feature)
  - 응답: `{ secret: "BASE32...", qr_code_data_uri: "data:image/png;base64,...", otpauth_uri: "otpauth://totp/AmazingKorean:email?..." }`
- **실패(이미 활성화) → 409 Conflict**
- **실패(미인증) → 401 Unauthorized**

#### 5.3-11 : `POST /auth/mfa/verify-setup` (MFA 설정 확인)
- **인증 필요**: Bearer 토큰 (AuthUser)
- **요청**: `{ code: "123456" }` (6자리 TOTP)
- **성공 → 200 OK**
  - TOTP 코드 검증 (±1 step, 90초 허용)
  - 백업 코드 10개 생성 (8자 영숫자)
  - 백업 코드 SHA-256 해시 → JSON → AES-256-GCM 암호화 → DB 저장
  - `user_mfa_enabled=true`, `user_mfa_enabled_at=now()` 업데이트
  - 응답: `{ enabled: true, backup_codes: ["ABC12345", ...] }` (1회만 노출)
- **실패(코드 무효) → 401 Unauthorized**

#### 5.3-12 : `POST /auth/mfa/login` (MFA 2단계 인증)
- **인증 불필요** (mfa_token으로 인증)
- **요청**: `{ mfa_token: "uuid", code: "123456" }` (TOTP 6자리 또는 백업 코드 8자리)
- **플로우**:
  1. Redis `ak:mfa_pending:{mfa_token}` 조회 + 삭제 (일회용)
  2. Rate limit 확인: `rl:mfa:{user_id}:{ip}` (5회/5분)
  3. TOTP 코드 검증 시도 (6자리 숫자)
  4. TOTP 실패 시 백업 코드 검증 시도 (SHA-256 비교)
  5. 백업 코드 사용 시 해당 해시 목록에서 제거 + DB 갱신
  6. 성공 → 세션 생성 (기존 login 후반부 로직 재사용)
- **성공 → 200 OK**: `{ access_token, user_id, ... }` + Set-Cookie(refresh_token)
- **실패(토큰 만료/무효) → 401** `MFA_TOKEN_EXPIRED`
- **실패(코드 무효) → 401** `MFA_INVALID_CODE`
- **실패(레이트리밋) → 429**

#### 5.3-13 : `POST /auth/mfa/disable` (MFA 비활성화)
- **인증 필요**: Bearer 토큰 (AuthUser, HYMN 역할만)
- **요청**: `{ target_user_id: 123 }`
- **성공 → 200 OK**
  - 대상 사용자의 MFA 컬럼 초기화 (secret=NULL, enabled=false, backup_codes=NULL)
  - 대상 사용자의 모든 세션 무효화 (보안)
  - 응답: `{ disabled: true, user_id: 123 }`
- **실패(HYMN 아닌 경우) → 403 Forbidden**

##### MFA 로그인 흐름 (이메일/비밀번호)
1. `POST /auth/login` → 이메일/비밀번호 검증 통과
2. MFA 활성화 사용자 → `{ mfa_required: true, mfa_token: "uuid", user_id: 123 }` (세션 미생성)
3. `POST /auth/mfa/login` → TOTP/백업 코드 검증 → 세션 생성 완료

##### MFA 로그인 흐름 (Google OAuth)
1. `GET /auth/google/callback` → OAuth 인증 완료
2. MFA 활성화 사용자 → 프론트 리다이렉트: `/login?mfa_required=true&mfa_token=uuid&user_id=123`
3. `POST /auth/mfa/login` → TOTP/백업 코드 검증 → 세션 생성 완료

##### AdminRoute MFA 가드
- Admin/HYMN 역할 사용자가 MFA 미설정 시 `/admin/mfa/setup`으로 강제 이동
- MFA 설정 완료 후 관리자 페이지 접근 가능

##### Redis 키 패턴 (MFA)
| 키 | 타입 | TTL | 용도 |
|----|------|-----|------|
| `ak:mfa_pending:{mfa_token}` | STRING (JSON) | 300초 | MFA 인증 대기 (로그인 1단계 후) |
| `rl:mfa:{user_id}:{ip}` | STRING (counter) | 300초 | MFA 코드 검증 Rate Limit |

##### DB 컬럼 추가 (users 테이블)
| 컬럼 | 타입 | 설명 |
|------|------|------|
| `user_mfa_secret` | TEXT | TOTP 비밀키 (AES-256-GCM 암호화) |
| `user_mfa_enabled` | BOOLEAN DEFAULT false | MFA 활성화 여부 |
| `user_mfa_backup_codes` | TEXT | 백업 코드 (SHA-256 해시 JSON, AES-256-GCM 암호화) |
| `user_mfa_enabled_at` | TIMESTAMPTZ | MFA 최초 활성화 시각 |

</details>

---

[⬆️ AMK_API_MASTER.md로 돌아가기](./AMK_API_MASTER.md)
