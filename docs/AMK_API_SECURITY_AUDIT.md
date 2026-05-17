# AMK API 보안 감사 보고서

> 작성일: 2026-05-15 / **재검증: 2026-05-17** (explanation 도메인 추가 후 doc↔code 대조)
> 범위: `/home/kkryo/dev/amazing-korean-api` (Rust 백엔드 — Cargo workspace, Docker, nginx, AWS, SQL migrations)
> 방식: 읽기 전용 코드/설정 분석. 실제 시크릿 값은 미포함, "노출 위치"만 기록.
> 비교 기준: 사내 다른 프로젝트(workout)에서 검증된 6개 보안 패턴

> **재검증 메모 (2026-05-17)** — 보안 작업 착수 전 doc↔code 간극 점검:
> - **actionable 항목 인용 = 정확** (간극 없음, 그대로 신뢰 가능): 2.1 `extractor.rs:42`(AuthUser decode_token 호출 라인 — 이후 Redis 세션 미검증 지점) + `role_guard.rs:51`(decode) / 2.2 `jwt.rs:41`(iss 발급)·`71`(`Validation::default()`) / 2.4 `pr-check.yml`(트리거=`push: KKRYOUN`+`workflow_dispatch`, cargo-deny 부재) 모두 현재 코드 일치 확인.
> - **신규 커버리지 추가 — explanation 도메인** (감사 작성 후 추가, §1 SQL/IDOR 주장 명시 확장): `src/api/explanation/` 공개 read API (`/explanations`, `/explanations/{unit_idx}`). **인젝션 0** — `repo.rs` 의 `format!` 은 정적 상수 `UNIT_COLS` 만, 사용자 입력(unit_idx/study_idx/study_task_idx/unit_id) 전부 `.bind()`. **인증 = 의도된 공개**(`api/mod.rs:66` nest, 미들웨어 없음 — 읽기 전용 공개 학습 콘텐츠, 사용자 스코프 데이터 없음 → IDOR/우회 무관, 설계 D3). `seed_explanation` 바이너리 = 네트워크 표면 아님. `seeds/explanation_seed.json` = 공개 콘텐츠, 시크릿 없음. → 신규 표면 보안 영향 없음, §2 이슈 목록 불변.

---

## 0. 한 줄 결론

SQL 인젝션·IDOR·시크릿 노출·비밀번호 저장은 **견고**. 가장 시급한 두 가지는
**(1) 발급된 access token 이 로그아웃/비밀번호 변경 후에도 만료 전까지 유효**(세션 revocation 미검증),
**(2) DB 슈퍼유저 접속**.

---

## 1. 현황 — 잘 되어 있는 것

| 영역 | 내용 | 근거 |
|---|---|---|
| 비밀번호 해시 | Argon2id, OWASP 권장(19MB/2iter/1par), per-call 랜덤 salt. 평문 저장 없음 | `src/api/auth/password.rs:13-21` |
| 시크릿 관리 | `.env*` gitignore/dockerignore 차단, `.env.example`만 추적(placeholder). compose 는 `${VAR}` 보간, 하드코딩 없음. `Config` Debug 가 시크릿 `"***"` 마스킹. JWT secret <32B 면 부팅 panic | `.gitignore:15-18`, `docker-compose.prod.yml:14,100`, `src/config.rs:751-888,121-126` |
| 인증 토큰 | JWT HS256 + Refresh rotation + reuse 탐지. Refresh 는 HttpOnly+SameSite+Secure 쿠키. 역할별 차등 TTL/세션수 | `src/api/auth/service.rs:761`, `src/api/auth/handler.rs:159-165` |
| SQL 인젝션 | 취약점 0. `format!` SQL 은 정적 컬럼 상수+placeholder 만, 사용자 입력은 전부 `.bind()`. sort 는 화이트리스트 `match`, LIKE 는 `escape_like()` | `src/api/admin/user/repo.rs:54-60`, `src/api/textbook/repo.rs:647`, `src/api/ebook/repo.rs:482` |
| IDOR 방어 | 사용자 리소스 쿼리가 인증 주체 ID 로 스코프. `WHERE ... AND user_id = $2` | `src/api/ebook/repo.rs:327,212`, `handler.rs:76,142,169` |
| 입력 검증 이중화 | `validator` derive(앱) + DB 제약 272건(CHECK/UNIQUE/FK/NOT NULL) | `migrations/20260208_AMK_V1.sql` |
| 필드 암호화 | AES-256-GCM + 버전드 KeyRing + HMAC-SHA256 blind index, OsRng nonce | `crates/crypto/src/cipher.rs:17` |
| 타이밍 공격 방어 | 미존재 사용자도 dummy argon2 수행(enumeration 방지), 토큰 `subtle::ct_eq` 상수시간 비교 | `src/api/auth/service.rs:128`, `src/api/payment/handler.rs:188` |
| 웹훅 인증 | Paddle HMAC+300s 타임스탬프(리플레이 방어), RevenueCat 상수시간 토큰. prod 키 미설정 시 panic | `src/api/payment/handler.rs:121-131`, `src/config.rs:453-458` |
| 인프라 | Dockerfile 멀티스테이지+non-root(uid 1001)+HEALTHCHECK. nginx TLS1.2/1.3+HSTS+X-Frame-Options+OCSP+rate limit. 앱 레이어 보안헤더+Redis rate limit | `Dockerfile`, `nginx/nginx.conf`, `src/main.rs:260-291` |
| 의존성 | cargo-deny 정책(advisory/yanked/wildcard deny), advisory 영향평가 코멘트와 함께 ignore. CI 주간 자동. `unsafe_code = "deny"` | `deny.toml`, `.github/workflows/security-audit.yml` |

---

## 2. 발견된 이슈 + 개선 권고

### 🔴 즉시

#### 2.1 Access Token 세션 폐기(revocation) 미검증 — **최우선**
- **문제**: `AuthUser` extractor 와 `admin_role_guard` 가 JWT 서명+만료만 검증. Redis `ak:session:` 키는 로그인/refresh/로그아웃에서만 참조되고 **요청별 검증 경로에 없음**. → 로그아웃·강제 퇴장·비밀번호 변경(`service.rs:1147` `revoked`) 후에도 발급된 access token 이 만료(기본 15분)까지 유효. 탈취 토큰/강제 로그아웃 우회 가능.
- **위치**: `src/api/auth/extractor.rs:42` (`AuthUser::from_request_parts`), `src/api/admin/role_guard.rs:51`
- **수정 방향**: 디코드 직후 `claims.session_id` 로 Redis `ak:session:{sid}` 존재(또는 jti 매칭) 확인, 없으면 401. 삭제 인프라는 이미 존재(`service.rs:799,1158`) — 검증만 연결. Redis 장애 시 fail-open/closed 정책 명시(로그아웃 보안 목적이면 fail-closed 권장 + 가용성 trade-off 문서화).
- [ ] 작업 예정

#### 2.2 JWT `iss`/`aud` 미검증 → 토큰 confusion
- **문제**: 토큰에 `iss: "amk"` 발급하나 `decode_token` 이 `Validation::default()` 사용 → `iss`/`aud` 검증 안 함. 비밀번호 재설정 토큰도 동일 `jwt_secret`+동일 Claims 구조(`service.rs:1127`) → reset 토큰을 인증 토큰으로 혼용할 여지.
- **위치**: `src/api/auth/jwt.rs:41`(발급), `jwt.rs:71`(검증)
- **수정 방향**: `Validation::new(Algorithm::HS256)` + `set_issuer(&["amk"])`. reset/MFA 토큰에는 별도 `iss` 또는 `aud`/`token_use` 클레임 부여해 access token 과 구분.
- [ ] 작업 예정

### 🟡 단기

#### 2.3 DB 최소 권한 미적용 (참고 패턴 1 적용 지점)
- **문제**: 앱이 PostgreSQL `postgres` 슈퍼유저로 접속. RLS/GRANT/CREATE ROLE 전무. 앱 침해·만일의 SQL 인젝션 시 피해 무제한.
- **위치**: `docker-compose.prod.yml:14,99`
- **수정 방향**: 런타임 전용 role 생성 → 필요 테이블에 `SELECT/INSERT/UPDATE/DELETE`만 GRANT, `DATABASE_URL` 을 해당 role 로 변경. 마이그레이션 실행 role 과 런타임 role 분리. (RLS 자체는 Rust 백엔드라 필수성 낮음 — 클라가 DB 직결 안 함. 최소권한이 비용 대비 효과 큼)
- [ ] 작업 예정

#### 2.4 cargo-deny PR 미실행
- **문제**: 주간 스케줄+수동만. 신규 취약 의존성이 머지 후 최대 1주 노출.
- **위치**: `.github/workflows/security-audit.yml:9-13`
- **수정 방향**: `cargo deny check` 를 PR 워크플로(`pr-check.yml`)에 추가해 머지 전 차단.
- [ ] 작업 예정

#### 2.5 관리자 IP allowlist 의 XFF 신뢰
- **문제**: `extract_client_ip` 가 `x-forwarded-for` 첫 값 신뢰. 신뢰 프록시 hop 검증 없음 → 클라가 XFF 위조해 allowlist 우회 가능. (현재 `ADMIN_IP_ALLOWLIST` 비어 미사용 — 활성화 시 위험)
- **위치**: `src/api/admin/header_utils.rs:14-26`
- **수정 방향**: nginx 가 단일 신뢰 헤더(`real_ip`)를 덮어쓰게 하고 앱은 그것만 사용, 또는 신뢰 hop 을 우→좌로 건너뛰는 로직. `ADMIN_IP_ALLOWLIST` 활성화 전 필수.
- [ ] 작업 예정

#### 2.6 CSP 헤더 부재
- **문제**: nginx·앱 모두 `Content-Security-Policy` 없음. Swagger UI(`ENABLE_DOCS`) 노출 시 방어선 부족(API 전용이라 위험도 자체는 낮음).
- **위치**: `nginx/nginx.conf`, `src/main.rs:262`(security_headers 미들웨어)
- **수정 방향**: 최소 `default-src 'none'; frame-ancestors 'none'`. Swagger 경로만 예외.
- [ ] 작업 예정

### 🟢 장기

- [ ] production 에서 `ENABLE_DOCS=0`/`SKIP_DB=0` 강제 — `src/config.rs:431` prod 검증부에 true 면 panic 추가
- [ ] 토큰 타입드 클레임(enum `token_use`)으로 access/reset/MFA 분리 + 디코드 지점마다 용도 강제
- [ ] nginx rate limit(`10r/s` 단일 zone)을 인증 엔드포인트/일반 트래픽으로 분리(앱 레이어 Redis 와 다층화)
- [ ] `HMAC_KEY`/`ENCRYPTION_KEY_V*` 를 AWS SSM Parameter Store(SecureString) 또는 Secrets Manager 로 이전(현재 평문 env)

---

## 3. 참고 보안 패턴 적용 가능성

비교 기준 6개 패턴(workout 검증). 스택 차이(Rust 백엔드, 클라가 DB 직결 안 함) 감안.

| # | 패턴 | 현 상태 | 권고 |
|---|---|---|---|
| 1 | DB 행 단위 권한(RLS) | 미사용. Rust 백엔드라 필수성 낮음. 앱 레이어 인가는 촘촘(쿼리마다 user_id 스코프, IDOR 방어 확인) | RLS 대신 **DB role 최소권한**(2.3) 채택 — 🟡 |
| 2 | 권한 키 분리 | 양호. service/admin 키 서버 env 한정, 클라 번들 미포함. Paddle server/client 토큰 분리 | 유지. KMS 이전만 🟢 |
| 3 | bcrypt 해시 | 충족 이상 — Argon2id | 변경 불필요 |
| 4 | 입력 검증 이중화 | 충족 — validator + DB 제약 272건 | 유지 |
| 5 | JWT + HttpOnly 쿠키 | 충족. 단 access token revocation 미검증이 약점 | 🔴 2.1 세션 검증 추가 |
| 6 | 약점 인지/문서화 | 우수 — deny.toml ignore 영향평가 코멘트, config panic 가드 사유 주석 | 세션 revocation·XFF 신뢰 trade-off 도 문서화 |

---

## 4. 작업 우선순위 체크리스트

```
🔴 즉시
  [ ] 2.1 access token 세션 폐기 검증 (extractor.rs:42 / role_guard.rs:51)
  [ ] 2.2 JWT iss/aud 검증 + reset 토큰 분리 (jwt.rs:71)

🟡 단기
  [ ] 2.3 DB 슈퍼유저 → 최소권한 role (docker-compose.prod.yml + migration)
  [ ] 2.4 cargo deny check 를 PR 워크플로에 추가
  [ ] 2.5 관리자 IP guard XFF 강화 (header_utils.rs)
  [ ] 2.6 CSP 헤더 (nginx / main.rs)

🟢 장기
  [ ] 3.x prod ENABLE_DOCS/SKIP_DB panic 가드
  [ ] 토큰 타입드 클레임 분리
  [ ] nginx rate limit zone 분리
  [ ] 키 KMS/Secrets Manager 이전
```

---

*이 문서는 읽기 전용 감사 결과입니다. 코드는 수정하지 않았으며, 실제 작업 시 각 항목의 `파일:라인` 을 기준으로 진행하세요.*
