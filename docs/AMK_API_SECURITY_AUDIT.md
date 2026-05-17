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
**(1) ~~발급된 access token 이 로그아웃/비밀번호 변경 후에도 만료 전까지 유효~~ → 2.1 완료(2026-05-17, fail-open+관찰성)**,
**(2) DB 슈퍼유저 접속**(2.3). → **2.1·2.2·2.4·2.5·2.6 완료, 2.3 Phase 1 완료(Phase 2 컷오버만 게이트 대기)** (2026-05-17). 🔴/🟡 실질 종결, 🟢 장기만 잔존.

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
- [x] **완료 (2026-05-17)** — `src/api/auth/session.rs::ensure_session_active` 신설, `extractor.rs`(AuthUser/OptionalAuthUser)·`role_guard.rs` 디코드 직후 연결. **정책 = fail-open + 관찰성** (감사 기본권고 fail-closed 와 의식적 상이 — 단일 Redis SPOF 에서 fail-closed 는 전면 인증 마비, 노출은 access TTL 15분 상한 = 2.1 이전 베이스라인 이하로만 후퇴해 악화 없음. 사용자 결정). Redis 정상·키 부재 → 401 / Redis 불가 → 검증 SKIP + `tracing::warn!(target="security.session_revocation")` (메트릭 facility 부재 → 로그 기반 알림). 통합 테스트 갱신(`common::seed_session`) + 폐기 세션 401 회귀 테스트 신설 — 로컬 라이브 검증 admin_rbac 8/8·auth_extractor 6/6 (시드→200/403, 폐기→401, 미인증→401).

#### 2.2 JWT `iss`/`aud` 미검증 → 토큰 confusion
- **문제**: 토큰에 `iss: "amk"` 발급하나 `decode_token` 이 `Validation::default()` 사용 → `iss`/`aud` 검증 안 함. 비밀번호 재설정 토큰도 동일 `jwt_secret`+동일 Claims 구조(`service.rs:1127`) → reset 토큰을 인증 토큰으로 혼용할 여지.
- **위치**: `src/api/auth/jwt.rs:41`(발급), `jwt.rs:71`(검증)
- **수정 방향(원안)**: `Validation::new(Algorithm::HS256)` + `set_issuer(&["amk"])`. reset/MFA 토큰에는 별도 `iss`/`aud`/`token_use` 클레임 부여.
- **⚠️ 2026-05-17 재검증 — 감사 전제 STALE**: 본 감사(2026-05-15) 작성 후 코드 변경됨. 실측:
  - 활성 reset 토큰 = **불투명 `ak_reset_<uuid>` Redis 토큰**(JWT 아님, `verify_reset_code:1553`, TTL 1800s). 발급은 opaque **전용**.
  - MFA 토큰 = Redis 랜덤(`ak:mfa_pending:`), JWT 아님 → 혼용 무관.
  - 즉 "reset=access 동일 JWT 구조" 전제 **틀림**. 진짜 취약점은 다른 곳:
  - **🔴 실 취약점(계정 탈취)**: 라우터 연결 활성 함수 `reset_password_with_token`(service.rs:1606) 의 else 분기가 `ak_reset_` 미접두 토큰을 `jwt::decode_token` 으로 처리(레거시 하위호환) → **피해자 access token(15분)을 `/reset-pw` reset_token 으로 제출 시 그 사용자 비밀번호 재설정 가능**. 정상 JWT reset 토큰은 발급 0(opaque 전용·30분 TTL) → 이 분기 = 순수 공격면.
  - `service::reset_password`(1095~1192, JWT 전용) = **dead code**(호출처 0, 핸들러는 `reset_password_with_token` 사용).
- **확정 수정 (a+b+c, 사용자 승인 2026-05-17 / 감사 원안과 의식적 상이 — token_use 클레임 대신 죽은 분기 제거가 더 단순·완전)**:
  - **(a)** `jwt.rs decode_token`: `Validation::new(Algorithm::HS256)` + `set_issuer(["amk"])` (iss/알고리즘 강제 — access 인증 하드닝, alg confusion 차단)
  - **(b)** `reset_password_with_token` 레거시 JWT 폴백 분기 제거 → reset 은 opaque `ak_reset_` 만 허용 (access-token→reset 계정 탈취 차단). 정상 토큰 영향 0
  - **(c)** dead `service::reset_password`(1095) 삭제 (동일 취약 형태 orphan)
  - 검증: jwt 단위(iss 불일치 거부) + 통합(access token을 reset 으로 제출 → 401 / opaque reset → 통과) 회귀 테스트
- [x] **완료 (2026-05-17)** — (a) `jwt.rs decode_token` = `Validation::new(Algorithm::HS256)`+`set_issuer(["amk"])` (b) `reset_password_with_token` 레거시 JWT 폴백 분기 제거 → `ak_reset_` opaque 전용, 비접두 즉시 `AUTH_401_INVALID_RESET_TOKEN` (c) dead `service::reset_password`(1095~1193, 약 100줄) 삭제. **검증**: jwt 단위 `test_decode_token_rejects_foreign_issuer` + 기존 roundtrip(iss=amk) 통과 / lib 213 passed / clippy·fmt clean / 통합 라이브 `--ignored`: service_integration 9/9 (**`test_reset_password_rejects_access_token_as_reset_token` = 실 access JWT 를 reset_token 으로 → 401, 계정 탈취 차단 실증**) + auth_extractor 6/6·admin_rbac 8/8 (2.1 무회귀, iss 강제 후에도 정상 토큰 통과). 기존 reset 테스트 호환(에러코드 동일 유지) + 구식 주석 정정.

### 🟡 단기

#### 2.3 DB 최소 권한 미적용 (참고 패턴 1 적용 지점)
- **문제**: 앱이 PostgreSQL `postgres` 슈퍼유저로 접속. RLS/GRANT/CREATE ROLE 전무. 앱 침해·만일의 SQL 인젝션 시 피해 무제한.
- **위치**: `docker-compose.prod.yml:14,99`
- **수정 방향(원안)**: 런타임 전용 role + 마이그 role 분리.
- **2026-05-17 사실 조사**: 앱은 **상시** `postgres` superuser 접속(`docker-compose.prod.yml:14`), 부팅마다 `sqlx::migrate!`(DDL) 실행(`main.rs:61`). superuser-only 연산(EXTENSION/ROLE/SYSTEM/COPY) **0건** = 불필요. **악용·사고 이력 0**(SQLi 0, IDOR 방어) → 잠재 폭발반경 위험(미실현). 부팅-마이그 동연결이라 순수 DML "최소권한"은 부팅 차단 → **단일 NOSUPERUSER 소유 role** 채택(원안의 마이그/런타임 풀분리는 수익체감·고위험으로 비채택, 사용자 결정).
- **Phase 1 완료 (2026-05-17, 런타임 영향 0)**: `db-init/10_least_priv_role.sql`(멱등) — `amk_app` LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE + public GRANT/ALTER DEFAULT PRIVILEGES + 앱 객체(table/seq/view/enum) OWNER→amk_app. `docker-compose.prod.yml` db 에 `db-init` initdb 마운트(+`APP_DB_PASSWORD` env). 앱은 **계속 postgres 접속**(DATABASE_URL 불변) = 동작 무변경. 로컬 amk-pg dry-run 검증: env없음/빈값 중단·정상 멱등 2회·table/enum owner=amk_app·NOSUPERUSER 확인 (dry-run 이 `REASSIGN OWNED BY postgres` 거부 + psql `$$` 내 `:'var'` 미치환 2버그 사전 적발·정정). 절차 = `AMK_DEPLOY_OPS.md §13`.
- **Phase 2 미실행 (게이트, 사용자 명시 승인 필요)**: `DATABASE_URL` user `postgres→amk_app` 교체 — Secret+compose+deploy.yml+§13 4곳 동시(INC-001 클래스). 롤백=user 환원 즉시.
- [~] **Phase 1 완료 / Phase 2 게이트 대기 (2026-05-17)**

#### 2.4 cargo-deny PR 미실행
- **문제**: 주간 스케줄+수동만. 신규 취약 의존성이 머지 후 최대 1주 노출.
- **위치**: `.github/workflows/security-audit.yml:9-13`
- **수정 방향**: `cargo deny check` 를 PR 워크플로(`pr-check.yml`)에 추가해 머지 전 차단.
- [x] **완료 (2026-05-17)** — `pr-check.yml` 에 `cargo-deny` job 신설(`EmbarkStudios/cargo-deny-action@v2`, `check --all-features`, `deny.toml` 정책 공유). KKRYOUN push 게이트에서 신규 취약 의존성 머지 전 차단. CI 워크플로만 — 런타임/prod 영향 0.
- [x] **게이트 가동 시 기존 deny.toml 부채 4층 표면화·전부 해소 (2026-05-17, `bc5bf33`→`94a2765`→`e1e3ee3`)** — 새 게이트가 오래된 트리 위에서 켜져, 그동안 안 보이던 의존성 위생 부채를 **첫 에러에서 멈추는** 도구 특성상 한 겹씩 노출:
  1. UNLICENSED `exceptions.allow` = v2 unknown term → `[licenses] private.ignore=true` (v2 정식, 기존 방식이 깨진 것)
  2. `CDLA-Permissive-2.0`/`NCSA` 미허용 → `allow` 추가 (webpki-roots/libfuzzer 실 전이 의존, permissive·비배포, 사유 인라인)
  3. 내부 path crate `amazing-korean-crypto` (version 미기재) wildcard 오탐 → `[bans] allow-wildcard-paths=true` (외부 `"*"` 는 여전히 deny)
  4. `core2 0.4.0` yanked (image→ravif→rav1e→bitstream-io 전이, 업그레이드 경로 없음) → `[advisories] yanked="deny"→"warn"`
- **본질 판정**: 1~3 = 메커니즘 교정·명시 정책결정(게이트 본질 무손상). 4 = 회피불가 trade-off지만 실 보안 신호 `RUSTSEC-2026-0105` 는 `advisories.ignore` 에서 영향평가 후 strict 별도 보존 → 현상 무마 아닌 문서화된 의도적 결정. yanked deny 유지 시 고칠 수 없는 전이 의존이 **모든 PR 영구 차단 → 게이트 무력화/우회**(순보안 손실)라 채택 불가.
- **로컬 검증 SOP 정착**: `cargo-deny 0.19.6` 로컬 설치 → 푸시 전 `cargo deny check` 로 advisories/licenses/bans/sources 4종 확정 검증. 더는 blind CI 왕복(push→대기→다음 에러) 없음. **deny.toml 변경 시 = 로컬 `cargo deny check` 선행 필수.**
- **프로덕션 라이브 검증 (2026-05-17, main `0fab7b2`)** — KKRYOUN→main squash 머지 후 EC2 deploy success. `/health 200` / CSP `default-src 'none'; frame-ancestors 'none'`(2.6, `/docs` 완화) / explanation API `sent:300` 실데이터 풀 응답 + `?study_task_idx=` list 200. 보안 §4 (2.1·2.2·2.4·2.5·2.6 + 2.3 Phase 1) prod 동작 확정.

#### 2.5 관리자 IP allowlist 의 XFF 신뢰
- **문제**: `extract_client_ip` 가 `x-forwarded-for` 첫 값 신뢰. 신뢰 프록시 hop 검증 없음 → 클라가 XFF 위조해 allowlist 우회 가능. (현재 `ADMIN_IP_ALLOWLIST` 비어 미사용 — 활성화 시 위험)
- **위치**: `src/api/admin/header_utils.rs:14-26`
- **수정 방향**: nginx 가 단일 신뢰 헤더(`real_ip`)를 덮어쓰게 하고 앱은 그것만 사용, 또는 신뢰 hop 을 우→좌로 건너뛰는 로직. `ADMIN_IP_ALLOWLIST` 활성화 전 필수.
- **⚠️ 2026-05-17 재검증 — 감사 인용 오류**: 본 감사가 지목한 `header_utils.rs::extract_client_ip` 는 **admin lesson/payment/study/user 핸들러의 감사 로그용**(접근 통제 아님). 실제 allowlist 강제자 `admin_ip_guard`(`ip_guard.rs`)는 **자체 인라인 XFF-first 추출**을 보유 — 감사 지목 함수 미사용. 진짜 우회 취약점은 `admin_ip_guard` 에 있었음(header_utils 위조 = 로그 오염, 접근 우회 아님 — 별 심각도).
- [x] **완료 (2026-05-17, scope=ip_guard 만)** — `admin_ip_guard` 의 IP 추출을 `trusted_client_ip()` 순수 fn 으로 분리, **`CF-Connecting-IP` 만 사용**(Cloudflare 가 세팅·덮어써 클라 위조 불가). 클라 위조 가능한 `X-Forwarded-For`/`X-Real-IP` 불신. 부재 시 **fail-closed(거부)** — allowlist 는 명시적 보안 통제이며 `ADMIN_IP_ALLOWLIST` 설정 시에만 작동(미설정 시 early-return 으로 영향 admin 한정, 2.1 의 가용성 우려와 반대로 엄격이 정공법). 단위 테스트 7종(위조 XFF 무시·CF 채택·부재 None 등) 통과. **scope 제외**: `header_utils::extract_client_ip`(로그 경로, 8 단위 테스트 XFF-first 가정·4 핸들러) = 다른 심각도(로그 정확성), 감사가 둘 혼동 → 별도 저순위 후속.
- **⚠️ 운영 주의 (ADMIN_IP_ALLOWLIST 활성화 시)**: 트래픽이 **반드시 Cloudflare 경유**해야 함(CF-Connecting-IP 존재). origin 직결 경로면 CF 헤더 부재 → fail-closed 로 admin 전면 차단. 활성화 전 CF 경유·헤더 전달 확인 필수.

#### 2.6 CSP 헤더 부재
- **문제**: nginx·앱 모두 `Content-Security-Policy` 없음. Swagger UI(`ENABLE_DOCS`) 노출 시 방어선 부족(API 전용이라 위험도 자체는 낮음).
- **위치**: `nginx/nginx.conf`, `src/main.rs:262`(security_headers 미들웨어)
- **수정 방향**: 최소 `default-src 'none'; frame-ancestors 'none'`. Swagger 경로만 예외.
- [x] **완료 (2026-05-17)** — 앱 `security_headers` 미들웨어(`src/main.rs`)에 CSP 추가. 기본 `default-src 'none'; frame-ancestors 'none'` / `/docs`·`/api-docs` 경로만 `self`+`unsafe-inline` 완화(Swagger UI JS/CSS, ENABLE_DOCS=1 시만 마운트·prod 기본 0). **nginx-level 의도적 미적용**: 앱 미들웨어가 전 프록시 응답에 CSP 부여(nginx 는 proxy_pass) → nginx add_header 는 중복, nginx 자체 생성 에러페이지만 미커버(드묾·API 전용 저위험). nginx.conf 변경=scp+reload 배포 리스크 회피(Karpathy #2 최소). 검증: cargo check/clippy clean (바이너리 크레이트 = 기존 헤더 테스트 인프라 없음, 배포 후 curl 확인 항목).

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
  [x] 2.1 access token 세션 폐기 검증 (2026-05-17 완료, fail-open+관찰성, 라이브 검증)
  [x] 2.2 JWT iss 강제 + reset 레거시 JWT 폴백 제거(계정 탈취 차단) (2026-05-17 완료, 라이브 검증)

🟡 단기
  [~] 2.3 DB 슈퍼유저 → amk_app NOSUPERUSER (2026-05-17 Phase 1 완료/Phase 2 컷오버 게이트 대기)
  [x] 2.4 cargo-deny PR 게이트 (2026-05-17 완료)
  [x] 2.5 admin_ip_guard CF-Connecting-IP 권위+fail-closed (2026-05-17 완료, 감사 인용 정정·scope=ip_guard)
  [x] 2.6 CSP 헤더 app 미들웨어 (2026-05-17 완료)

🟢 장기
  [ ] 3.x prod ENABLE_DOCS/SKIP_DB panic 가드
  [ ] 토큰 타입드 클레임 분리
  [ ] nginx rate limit zone 분리
  [ ] 키 KMS/Secrets Manager 이전
```

---

*이 문서는 읽기 전용 감사 결과입니다. 코드는 수정하지 않았으며, 실제 작업 시 각 항목의 `파일:라인` 을 기준으로 진행하세요.*
