# AMK_DEBTS — 미해결 부채 카탈로그

> **목적**: amazing-korean-api 의 미해결 부채를 한 곳에 정리. 부채 발견 시 본 문서에 즉시 등재. 작업 우선순위 결정 + 조회 시 진입점.
> **작성 원칙**: 사실 기반. 가정·해석 배제. 위치(파일:줄/명령어/공식 ID) + 영향 범위 + 처리 시점 명시. **라인 번호는 작성 시점 (HEAD) 기준 — commit 후 stale 가능, 사용 시 grep 재확인**.
> **작성일**: 2026-05-04 (PR #205 진행 중 일괄 조사 + 5 agent 정합성 검증 + 경로 2 추가 조사 통합)
> **검증 2회차**: 2026-05-04 (저녁, HEAD `3cad9a3`) — 6 agent 분담 병렬, M-007 사고 후 라인/카운트 stale 정정. A2-1 라인 9곳 / B4 unwrap 358→397, 1123→1396 / B5 47→48 / B6 11~12→50 / C6 233→237 / H1 72→41 / J 카운트 3건 정정.
> **검증 3회차**: 2026-05-06 — B5 카운트 48 → 52 정정 (PR #212~#218 기간 추가) + 위험도 분류 종결 (🟢 45 / 🟡 7 / 🔴 0). 처리 우선순위 낮음 확정.
> **갱신 규칙**: 부채 신규 발생 시 해당 카테고리에 추가. 처리 완료 시 행 시작에 `~~취소선~~` + 완료일/PR 명시. 본 문서 직접 갱신 (메모리 미동기화 — 메모리는 본 문서 참조 포인터만).
> **참조 SSoT**:
> - production 인시던트 (INC-NNN): `AMK_CHANGELOG.md` + `feedback_deploy_env_sync.md`
> - AI 작업 사고 (M-NNN): `AMK_AI_MISTAKES.md`
> - 본 문서: 그 외 모든 부채 (기능/품질/보안/인프라/자동화)

---

## 0. 요약 (카테고리별 카운트, 2026-05-04 정합성 검증 후)

| 카테고리 | 미해결 건수 | 비고 |
|---------|:---:|------|
| A. 운영/배포 부채 | ~~10~~ → ~~9~~ → **7** | KYB 의존 4 + 인프라 이전 3. ~~A4-3/A4-5/A4-6/A4-7/A4-8~~ ✅ + ~~A4-4~~ 🟡 부분 (2026-05-06) + ~~A4-1/A4-2~~ ✅ **Phase B 완료 (2026-05-07: HTTPS + Let's Encrypt + Cloudflare Full Strict + 자동 갱신)** |
| 🟡 B. 보안 부채 (취약점) | ~~1~~ → **0** | Rust **1** (rsa Marvin Attack, no upgrade) — 🟡 수용 결정 (2026-05-06, compile-time only + PostgreSQL only = production 영향 0). ~~npm 3건~~ ✅ 해결 (2026-05-04). rustls-webpki 3건 ✅ 해결 (2026-05-04) |
| 🟡 B. 보안 부채 (unsound/unmaintained) | ~~7~~ → **0** | 🟡 모두 수용 결정 (2026-05-06). core2/paste = unmaintained warning 만 + transitive. imageproc 3 = 텍스트 오버레이 영향 낮음. rand 2 = custom logger 미사용으로 영향 0 |
| ~~B. 보안 부채 (panic 위험)~~ | ~~2~~ → **0** | ~~unwrap 잠재 위험 2건~~ ✅ B4 해결 (2026-05-04, commit `ad239ed`) |
| B. 보안 부채 (외부 통신) | **1** | B6 ipgeo HTTP-only. ~~B7 Paddle amount~~ ✅ 해결 (2026-05-04, commit `c744efc`) |
| C. 코드 품질 부채 | **2** | C1 ESLint 27 + C2 lint:ui 9. ~~C3/C4/C5/C6/C7/C8~C13~~ 처리/수용 (2026-05-04~05). C7 ✅ commit `2641766` (bundle 모니터링). B5/B6 = B 카테고리로 재분류 |
| D. 인프라 부채 | 4 | RDS 이전 묶음 (A2 와 중복) |
| E. 기능 부채 (보류/조건부) | **11** | 9 (보류 8 + STATUS #11 이메일 수신 ✅) + **신규 3** (콘텐츠 시딩, SpeechSuper, 번들 최적화) |
| F. 모바일/데스크탑 앱 부채 | 5 | 외부 리포 SSoT |
| G. 자동 검증 부재 (CI 부채) | **5** | ~~G3/G4/G5/G6/G7/G11/G13/G14~~ ✅ 해결 또는 🟡 수용 (2026-05-05). 잔여 = G1/G2 (보류 cargo test/playwright) + G8 branch protection (보류) + G10 src 테스트 부족 + G12 cargo-geiger (보류) |
| H. 문서/메모리 부채 | **0** | ~~H1 메모리 stale~~ 🟡 + ~~H2 docs↔코드 자동 도구~~ 🟡 = 수용 결정 (2026-05-05) |
| I. AI 작업 사고 | **7** | `AMK_AI_MISTAKES.md` SSoT (M-006 → 신규 M-007 = 라인 번호 복사 시 미검증) |
| J. 환경변수/Secrets 정합성 | **0** | ~~J1/J2/J3~~ ✅ + ~~J4~~ 🟡 (2026-05-05 모두 처리/수용). J3 도구 발견 신규 차이 14건 → .env.example/deploy.yml 추가 (commit `7aae36a`) = 사실상 정합성 정착. 도구 보강 (docker-compose.prod.yml union + 주석 인식) = 별도 후속 |

**총 미해결 부채 = 42건** (카테고리 합산: A 7 + B 1 (B6) + C 2 + D 4 + E 11 + F 5 + G 5 + H 0 + I 7 + J 0. 2026-05-07 Phase B 완료로 A4-1/A4-2 ✅ → -2건 = 44 → 42. 카테고리 중복 미배제, 단순 카운트).

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
| A2-1 | E-book fs::read 9곳 — service.rs 8 + watermark.rs 1 | `src/api/ebook/service.rs:62, 402, 651, 665, 679, 762, 777, 791` + `src/api/ebook/watermark.rs:13` | CRITICAL | RDS 이전 시 S3 SDK 전환 (Q9) |
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
| ~~A4-1~~ | ~~nginx HTTPS 미활성 (HTTP-only)~~ ✅ **해결 (2026-05-07 Phase B)** | nginx HTTPS 블록 활성 (TLS 1.2+1.3 / Mozilla Intermediate cipher / OCSP stapling / HSTS / SSL session cache). Cloudflare SSL 모드 = **Full (Strict)** = end-to-end 암호화. origin Let's Encrypt cert (만료 2026-08-05) | — |
| ~~A4-2~~ | ~~Let's Encrypt + certbot 자동 갱신 정책 부재~~ ✅ **해결 (2026-05-07 Phase B)** | certbot 12h renew loop + host crontab 매일 03:00 nginx reload. `renew --dry-run` 통과 검증. 자동 갱신 정착 | — |
| ~~A4-3~~ | ~~EC2 디스크 모니터링 자동화 부재~~ ✅ 해결 (2026-05-05, commit `693dc2a`) | `AMK_DEPLOY_OPS §6` 안에 모니터링 절차 (df -h / docker system df / 임계 70/85/95% / 정리 명령) 추가. 향후 자동화 후속 (GitHub Action SSH) | — |
| ~~A4-4~~ | ~~DB / Redis 백업 정책 부재 (DR 0)~~ 🟡 **부분 해결 (2026-05-06)** | `AMK_DEPLOY_OPS §6` 안에 수동 백업·복구 절차 추가 (PostgreSQL pg_dump + Redis BGSAVE/RDB cp + 권장 정책 표 + 자동화 후속). 자동화는 사용자 정책 결정 후 별도 후속 (cron / S3 / 보관 기간) | — |
| ~~A4-5~~ | ~~Docker log 로테이션 미설정~~ ✅ 해결 (2026-05-04, commit `7e86592`) | YAML anchor `x-logging` + 5 서비스 (api/db/redis/nginx/certbot) 일괄 적용 (max-size 10m × max-file 3 = 서비스당 최대 30MB) | — |
| ~~A4-6~~ | ~~Cloudflare DNS / Email Routing 운영 정책 미문서화~~ ✅ 해결 (2026-05-05, commit `6e7b006`) | `AMK_DEPLOY_OPS.md §7.6` 통합 SSoT 신규 (DNS/Pages/SSL/WAF/Email Routing + 변경 절차 + 비상 시 절차) | — |
| ~~A4-7~~ | ~~nginx Rate Limiting 모니터링 부재~~ ✅ 해결 (2026-05-05, commit `f82dd0d`) | `AMK_DEPLOY_OPS.md §6` 안에 모니터링 절차 (docker logs grep + 대응 정책) 추가 | — |
| ~~A4-8~~ | ~~Docker base image 자동 업데이트 정책 부재~~ ✅ 해결 (2026-05-05, commit `9367f72`) | `.github/dependabot.yml` 신규 (Cargo/npm/Docker/GitHub Actions 자동 PR, 주간/월간 스케줄). G6 동시 해결 | — |

---

## B. 보안 부채

### 🟡 B1. Rust 의존성 보안 취약점 (4 vulnerabilities, cargo audit 2026-05-04) — 잔여 1건 수용 결정 (2026-05-06)

| ID | Crate | Version | Severity | Title | 처리 가능성 |
|:--:|:-----:|:-------:|:--------:|-------|----------|
| 🟡 RUSTSEC-2023-0071 | rsa | 0.9.10 | medium 5.9 | Marvin Attack: timing sidechannel key recovery | **수용 (2026-05-06)** — 사유 아래 |
| ~~RUSTSEC-2026-0099~~ | ~~rustls-webpki~~ | ~~0.103.10~~ | — | ~~Name constraints accepted for wildcard certificates~~ | ✅ **2026-05-04 해결** (rustls-webpki 0.103.10 → 0.103.13, `cargo update`) |
| ~~RUSTSEC-2026-0104~~ | ~~rustls-webpki~~ | ~~0.103.10~~ | — | ~~Reachable panic in CRL parsing~~ | ✅ **2026-05-04 해결** (동일) |
| ~~RUSTSEC-2026-0098~~ | ~~rustls-webpki~~ | ~~0.103.10~~ | — | ~~Name constraints for URI names incorrectly accepted~~ | ✅ **2026-05-04 해결** (동일) |

**B1 rsa 수용 사유 (2026-05-06)**:
- `cargo audit` 결과 = "No fixed upgrade is available!" (RustSec 명시)
- 의존 트리: `rsa → sqlx-mysql → sqlx-macros → sqlx → amazing-korean-api`
- sqlx-macros = **컴파일 타임 macro 라이브러리** (모든 DB driver 컴파일). 런타임 binary 에 mysql 코드 미포함.
- 우리 시스템 = PostgreSQL only (Cargo.toml line 33 = `features = ["postgres", ...]`, mysql feature X)
- → **production runtime 영향 = 0** (rsa 코드 자체가 binary 에 포함되지 X)
- upstream sqlx 가 sqlx-macros 의 mysql driver 를 분리할 때까지 대기 (현재 의존성 구조상 회피 불가)

### 🟡 B2. Rust 의존성 unsound/unmaintained (7건) — 모두 수용 결정 (2026-05-06)

| ID | Crate | Version | Warning | 영향 분석 |
|:--:|:-----:|:-------:|---------|----------|
| 🟡 RUSTSEC-2026-0105 | core2 | 0.4.0 | unmaintained, all versions yanked | transitive 의존 (image → ravif → rav1e → bitstream-io → core2). upstream image crate fix 대기 |
| 🟡 RUSTSEC-2024-0436 | paste | 1.0.15 | no longer maintained (보안 X) | macro 라이브러리, unmaintained 만, 보안 취약점 X. transitive (imageproc/rav1e 경유) |
| 🟡 RUSTSEC-2026-0116 | imageproc | 0.25.0 | unsound — improper invariant check | 우리 사용 = `src/api/ebook/watermark.rs` 텍스트 오버레이 경로. unsound 3건 모두 기하학 변환/샘플링 = 텍스트 오버레이 영향 낮음 (검증 명시) |
| 🟡 RUSTSEC-2026-0117 | imageproc | 0.25.0 | unsound — fragile bounds check (sampling) | 동일 |
| 🟡 RUSTSEC-2026-0115 | imageproc | 0.25.0 | unsound — fragile bounds check (sampling) | 동일 |
| 🟡 RUSTSEC-2026-0097 | rand 0.8.5 | unsound — custom logger using `rand::rng()` | 우리 시스템 = `tracing-subscriber` 사용 (custom logger 미사용, `grep set_logger src/` = 결과 0). 영향 = 0 |
| 🟡 RUSTSEC-2026-0097 | rand 0.9.2 | 동일 | 동일 (transitive: totp-rs / rav1e) |

**B2 7건 수용 사유 (2026-05-06)**:
- 4건 (imageproc 3 + rand 2 = 5건) = unsound but 영향 분석 결과 production 영향 낮음 / 0
- 2건 (paste / core2) = unmaintained warning 만, 보안 취약점 X
- 처리 옵션 = upstream image / imageproc / rav1e 의 fix 또는 fork. 현재 본 리포 직접 회피 불가.
- 재평가 트리거 = upstream fix release 또는 새 unsound advisory 시.

### ~~B3. npm 의존성 보안 취약점~~ ✅ 해결 (2026-05-04, commit `ee68c7c`)

| Severity | Package | 상세 |
|:--------:|:-------:|------|
| ~~moderate~~ | ~~postcss <8.5.10~~ | ~~XSS via Unescaped `</style>` (GHSA-qx2v-qp2m-jg93)~~ ✅ |
| ~~moderate~~ | ~~follow-redirects ≤1.15.11~~ | ~~Custom Auth Header leak~~ ✅ |
| ~~**HIGH**~~ | ~~basic-ftp ≤5.2.2~~ | ~~DoS via unbounded memory~~ ✅ |

**처리 완료**: `npm audit fix` 자동 처리 (lock 만 갱신, package.json 무변경 = semver 호환). `npm audit` 0 vulnerabilities + `npm run build` 통과.

### ~~B4. panic 위험 잠재 — `unwrap()` (9건 중 2건 위험)~~ ✅ 해결 (2026-05-04, commit `ad239ed`)

| 위치 | 코드 | 위험 | 비고 |
|------|------|:--:|------|
| `src/error.rs` | `to_string().parse().unwrap()` | 안전 | round-trip |
| `src/api/user/service.rs` (3곳) | `NaiveDate::from_ymd_opt(1900,1,1).unwrap()` / Argon2 `Params::new` | 안전 | 정적 값 |
| ~~`src/api/auth/service.rs:397`~~ | ~~`Some(user) => PasswordHash::new(user.user_password.as_ref().unwrap())`~~ | ✅ 해결 | `ok_or_else(\|\| AppError::Internal(...))?` 매핑 |
| ~~`src/api/auth/service.rs:1396`~~ | ~~`let user_info = user.unwrap()`~~ | ✅ 해결 | `ok_or_else(\|\| AppError::Internal(...))?` 매핑 |
| `src/api/ebook/watermark.rs:170` | `hash[..8].try_into().unwrap()` | 안전 | 길이 검증 후 |
| `src/api/admin/user/service.rs` (2곳) | `NaiveDate::from_ymd_opt(1900,1,1).unwrap()` | 안전 | 정적 값 |

**처리 완료**: 위험 잠재 2건 모두 `AppError::Internal` 명시 매핑으로 교체 (commit `ad239ed`). anti-enumeration 유지.

### B5. `expect()` 52건 — 위험도 분류 완료 (2026-05-06)

> 처리는 별도 트랙. 본 항목 = 위험도 라벨링 + 처리 우선순위 판단.
>
> **카운트 정정**: 48 → 52 (PR #212~#218 기간 코드 추가분 4건 반영, 실측 grep `\.expect(`).

**파일별 카운트**

| 파일 | 건수 | 분류 |
|------|:--:|------|
| `src/config.rs` | 37 | 🟢 안전 (부팅 시 환경변수 파싱 panic = production safety gate) |
| `src/main.rs` | 6 | 🟢 안전 (부팅 시 Redis pool / API key / Paddle client 초기화) |
| `src/api/auth/service.rs` | 2 | 🟢 1 (dummy hash, 정적 입력) + 🟡 1 (line 447 invariant 의존) |
| `src/api/user/service.rs` | 1 | 🟢 안전 (`hmac_key: &[u8; 32]` 타입 보장) |
| `src/external/email.rs` | 1 | 🟡 cold init (reqwest builder, ResendEmailSender::new) |
| `src/external/apple.rs` | 1 | 🟡 cold init (reqwest builder) |
| `src/external/google.rs` | 1 | 🟡 cold init (reqwest builder) |
| `src/external/vimeo.rs` | 1 | 🟡 cold init (reqwest builder) |
| `src/external/revenuecat.rs` | 1 | 🟡 cold init (reqwest builder) |
| `src/external/ipgeo.rs` | 1 | 🟡 cold init (reqwest builder) |

**분류 합계**

| 분류 | 건수 | 의미 |
|------|:--:|------|
| 🟢 안전 | 45 | 부팅 시점 fail-fast 또는 타입/정적 invariant 로 panic 불가능 |
| 🟡 회색 | 7 | cold init (reqwest builder 6) + 논리 invariant 1 (auth:447) |
| 🔴 위험 | 0 | hot path runtime panic 가능 expect 없음 |

**🟡 회색 7건 상세**

| 위치 | 패턴 | panic 트리거 가능성 |
|------|------|-----|
| `src/external/{apple,email,google,ipgeo,revenuecat,vimeo}.rs` | `reqwest::Client::builder()…build().expect("…")` | 극히 드묾 (TLS native roots 로드 실패 등). reqwest builder API 가 사실상 무결. 객체 생성 1회성 |
| `src/api/auth/service.rs:447` | `user_info.expect("checked above")` 로그인 hot path | 위 분기 (`if user_info.is_none() return Err`) 깨지면 panic. 코드 구조 변경 시 invariant 깨질 위험 |

**처리 권고 (별도 트랙)**

- **🟢 안전 45건** = 처리 불요. 의도된 fail-fast 또는 타입 보장.
- **🟡 reqwest builder 6건** = 수용 권고. `unwrap_or_else` 로 fallback 만들기 어려움 (Client 가 있어야 외부 호출 가능). OnceCell 화 검토 가치 ≪ 비용.
- **🟡 auth:447 1건** = `let-else` 또는 `match` 리팩터 권고 (defense-in-depth, 시간 5m). hot path 이지만 현재 invariant 안전. 우선순위 낮음.

**결론**: 🔴 0건 = production 운영 중 unexpected panic 위험 expect 호출은 0. B5 = 위험도 분류 종결, 후속 처리는 우선순위 낮음 (선택적).

### 🟡 B8. SSL Labs B → A+ 강화 (2026-05-07 신규 발견)

| 위치 | 사실 |
|------|------|
| https://www.ssllabs.com/ssltest/analyze.html?d=api.amazingkorean.net | **B 등급** (4 IP 모두). Cloudflare edge default 영향 |
| origin nginx | 자체 A+ 수준 설정 (TLS 1.2+1.3 / Mozilla Intermediate / HSTS / OCSP). origin 측 영향 X |
| 원인 | Cloudflare edge 가 구식 클라이언트 호환성 위해 weak cipher 일부 활성 |

**처리 옵션 (사용자 결정)**:
- Cloudflare 대시보드 → SSL/TLS → Edge Certificates → Minimum TLS Version = 1.2 이상 + TLS 1.3 활성 (Free 플랜 가능)
- Cloudflare Pro+ 플랜 = Modern cipher suite 옵션 (월 비용 발생)
- 우선순위 = 낮음 (보안 갭 X, 사용자 인증서 정상 동작. 외부 grade 만 영향)

### B6. ipgeo HTTP-only (2026-05-04 신규 발견)

| 위치 | 사실 |
|------|------|
| `src/external/ipgeo.rs:50` | ip-api.com 무료 이용권 = HTTP only (HTTPS 는 유료). IP 기반 위치 조회 시 평문 전송 → 중간자 공격 위험 |

**처리**: ip-api 유료 전환 또는 MaxMind GeoLite2 로컬 DB 전환 (E-9 와 통합 가능).

### ~~B7. Paddle 웹훅 amount defense-in-depth 결여~~ ✅ 해결 (2026-05-04, commit `c744efc`)

| 위치 | 사실 |
|------|------|
| `src/api/payment/service.rs:552` | `let amount_cents = txn.details.totals.total.parse::<i32>().unwrap_or(0);` |
| `src/api/payment/service.rs:553` | `let tax_cents = txn.details.totals.tax.parse::<i32>().unwrap_or(0);` |

**처리 완료**: subscription 매핑 후 `create_transaction` 전에 `amount_cents != billing_interval.price_cents()` 검증 추가. 불일치 시 `tracing::error` + `AppError::Internal` → 500 응답 (Paddle 자동 재시도). DB 저장 차단 = fail-closed semantics. (commit `c744efc`)

---

## C. 코드 품질 부채

### C1. Frontend ESLint baseline (Q16) — 27 errors + 13 warnings

> 카테고리 분류 정정 (2026-05-04 agent 검증):
> - `react-hooks/incompatible-library` **10건** (errors)
> - `react-hooks/static-components` **9건** (errors)
> - `react-refresh/only-export-components` **7건** (errors)
> - 기타 errors (prefer-const, no-empty 등) 1건씩
> - **warnings 13건 카테고리 (검증 2회차 추가 식별)**: `react-hooks/exhaustive-deps` 3 / `react-hooks/refs` 5 / `react-hooks/set-state-in-effect` 2 / `react-hooks/use-memo` 1 / 기타 2

**처리**: shadcn 컴포넌트 파일 분할 + react-hooks 위반 fix + prefer-const fix. 시간 1-2일.

### C2. Frontend lint:ui baseline (Q16) — 9 errors

| 위치 | 카운트 | 색상 |
|------|:--:|------|
| `frontend/src/category/textbook/page/textbook_order_page.tsx` | 2 | emerald, amber (정정: 이전 4 → 2) |
| `frontend/src/category/textbook/receipt_parts.tsx` | 1 | red |
| `frontend/src/category/book/page/book_hub_page.tsx` | 4 | emerald, amber, rose, teal |
| `frontend/src/category/study/component/writing/HangulKeyboardKey.tsx` | 1 | amber |

**합계**: 9 (검증 2회차 = 표 합산 2+1+4+1=8 vs 실측 9. lint:ui 출력 카운트 9 정확).

**처리**: 디자인 토큰 결정 + 9곳 교체. 시간 0.5-1일.

### ~~C3. Rust rustfmt baseline~~ ✅ 해결 (2026-05-04 밤, commit 후속)

> 95 파일 cleanup 완료. `cargo fmt --check --all` exit=0. C4 trailing whitespace 수동 fix 후 cargo fmt --all 재실행 성공.

### ~~C4. `src/docs.rs:92, 94` trailing whitespace~~ ✅ 해결 (2026-05-04 밤)

> 수동 제거 완료. rustfmt internal error 해소. C3 cleanup 진행 가능해짐.

### ~~C5. enum sqlx::Type derive 미전환~~ ✅ 해결됨

> 2026-05-04 검증 결과 = `src/types.rs` 의 enum 들은 이미 `#[sqlx(type_name = "...", rename_all = "...")]` 패턴 적용 완료. `AMK_STATUS.md §8.2 보류 #13` 도 정정 필요.

### 🟡 C6. TODO/FIXME 주석 — 1건 — 수용 결정 (2026-05-05)

| 위치 | 내용 |
|------|------|
| `src/api/video/repo.rs:237` | `video_last_ip_log는 현재 항상 NULL. IP 수집 시 암호화 필수 (Phase 3 참조)` |

**결정**: 수용. Phase 3 (IP 수집 활성화) 트리거 시점에 같이 처리 = 의도된 미래 작업 마커.

### ~~C7. Frontend bundle 사이즈 모니터링 부재~~ ✅ 해결 (2026-05-05, commit `2641766`)

`rollup-plugin-visualizer` 추가 + `frontend/vite.config.ts` plugin 등록. `npm run build` 시 `dist/bundle-stats.html` 자동 생성 (gzip/brotli 사이즈 트리맵 시각화).

| 큰 파일 (top 5, 본 세션 빌드) | 사이즈 |
|---|---:|
| `vendor-react-D3yu5mlF.js` | 231KB (gzip 74KB) |
| `index-DkVKf_Ig.js` | 213KB (gzip 62KB) |
| `vendor-BqtW7tHd.js` | 186KB (gzip 65KB) |
| `bn-D_X29MvY.js` | 115KB (gzip 27KB) |
| `lo-BknZrytl.js` | 112KB (gzip 26KB) |

향후 후속: 임계 기준 (예: index.js > 250KB warning) CI 통합.

### 🟡 C8-C13. Rust/TS 룰 회피 카운트 — 수용 결정 (2026-05-05)

| ID | 항목 | 카운트 | 비고 |
|:--:|------|:--:|------|
| C8 | Rust `#[allow(dead_code)]` | **33건** | 죽은 코드 회피 (의도된 향후 사용 / 외부 trait impl) |
| C9 | Rust `#[allow(clippy::*)]` | **11건** | 특정 clippy 룰 회피 (의도된 패턴) |
| C10 | Rust `#[allow(unused_imports)]` | **8건** | 미사용 import 회피 (의도된 trait re-export) |
| C11 | Rust `#[allow(unused_assignments)]` | 1건 | 미사용 할당 회피 |
| C12 | TypeScript `any` 사용 | **3건** | ebook DRM 의도 (N-3 동일 영역 = 수용) |
| C13 | TypeScript `eslint-disable` 인라인 | **11건** | mount-once / DRM / devtools_detect 의도 (N-5 동일 정책) |

**결정**: 수용. 각 룰 회피 = 의도된 사용 (Rust unsafe 0건 + TS @ts-ignore 0건 = 안전 보장). Q16 baseline cleanup 트랙에서 개별 평가 (C8 dead_code 위주).

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
| ~~G3~~ | ~~`cargo audit` 자동 실행~~ ✅ 해결 (2026-05-05, commit `766c1ce`) | `.github/workflows/security-audit.yml` 신규 (cargo-deny-action@v2 사용, deny.toml 정책). 매주 월 09:00 KST + 수동 |
| ~~G4~~ | ~~`npm audit` 자동 실행~~ ✅ 해결 (2026-05-05, commit `766c1ce`) | 같은 workflow 의 npm-audit job (`--audit-level=high`). dependabot 보안 PR 과 별개 즉시 fail |
| ~~G5~~ | ~~`cargo outdated` / `npm outdated`~~ 🟡 수용 (2026-05-05) | dependabot 자동 PR (commit `9367f72`) 과 중복 = 별도 outdated 검사 불필요 |
| ~~G6~~ | ~~dependabot 자동 PR~~ ✅ 해결 (2026-05-05, commit `9367f72`) | `.github/dependabot.yml` 신규 (Cargo/npm/Docker/Actions). A4-8 동시 해결 |
| 🟡 G7 | secret scanning / GitHub Advanced Security 🟡 수용 (2026-05-05) | private repo + GHAS 라이선스 비용 평가 별도. 1인 환경 + 기존 anti-pattern (config.rs hardcoded secret 0건) = 위험 작음. 향후 plan 결정 시 활성 |
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
| ~~G11~~ | ~~`cargo-deny` 미설치 (라이선스 호환성 / 의존성 정책 검증)~~ ✅ 해결 (2026-05-05, commit `ced50c4`) | `deny.toml` 신규 (advisory ignore 7건 / 라이선스 13종 허용 / multi-version warn / sources 정책). `cargo install cargo-deny` 후 `cargo deny check` 사용. CI 자동 통합 = 후속 |
| G12 | `cargo-geiger` 미설치 (unsafe 코드 분석) | unsafe 0건이라 우선순위 낮음 |
| ~~G13~~ | ~~`.github/CODEOWNERS` 미존재~~ ✅ 해결 (2026-05-05, commit 본 세션) | `.github/CODEOWNERS` 신규 (도메인별 owner = `@AmazingKoreanCenter`) |
| ~~G14~~ | ~~PR template / issue template 미존재~~ ✅ 해결 (2026-05-05, commit 본 세션) | `.github/PULL_REQUEST_TEMPLATE.md` (변경/부채/검증/SSoT/모니터링 체크리스트) + `.github/ISSUE_TEMPLATE/bug_report.md` + `feature_request.md` 신규 |

---

## H. 문서/메모리 부채

| ID | 항목 | 사실 |
|:--:|------|------|
| 🟡 H1 | 메모리 stale 위험 — 수용 결정 (2026-05-05) | 정책상 메모리 = 수동 갱신 (자동 도구 도입 = 메모리 시스템 정책 변경 필요 = 별도 결정). 본 세션 자체 갱신 = 패턴 정착 |
| 🟡 H2 | docs ↔ 코드 일관성 자동 검증 — 수용 결정 (2026-05-05) | J3 패턴 (env 정합성 자동 도구) 처럼 docs↔코드 자동 도구 = 작업 큼 + 가치 분산. AMK_API_*.md 의 enum 카운트 / N-NNN 라인 등 = 수동 grep 검증 (M-007 사고 후 정착). 향후 별도 트랙 |

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
| `.env.example` 정의 | **57건** | `grep -cE '^[A-Z_]+=' .env.example` (검증 2회차 정정: 65 → 57) |
| `.github/workflows/deploy.yml` 안 secrets 사용 | **22건** | `secrets.X` 호출 |
| `deploy.yml` heredoc env 변수 | **33건** | secrets + hardcoded (APP_ENV=production 등) |
| `src/config.rs` `env::var()` 호출 | **82건** | env::var + ENCRYPTION_KEY 레거시 폴백 (검증 2회차 정정: 78 → 82) |

### J1-J4. 정합성 문제 (신규 발견)

| ID | 항목 | 위험 |
|:--:|------|------|
| ~~**J1**~~ | ~~`RATE_LIMIT_TEXTBOOK_WINDOW_SEC` / `RATE_LIMIT_TEXTBOOK_MAX`~~ ✅ 해결 (2026-05-05, commit `7aae36a`) | `.env.example` 추가 + `deploy.yml` heredoc 추가. config.rs default ("3600"/"5") 명시적 |
| ~~J2~~ | ~~`APPLE_CLIENT_ID` / `APPLE_TEAM_ID`~~ ✅ 해결 (2026-05-05, commit `7aae36a`) | `.env.example` 추가. Apple OAuth 미구현 시 비활성 (Option) |
| ~~J3~~ | ~~정합성 검증 자동 도구 X~~ ✅ 해결 (2026-05-05, commit `697dbae`) | `scripts/check_env_consistency.sh` 신규 (3중 동기화 검증, exit 1 시 차이 발견). 사용 = `bash scripts/check_env_consistency.sh`. PR 자동 통합 = 후속 (J1/J2 등 기존 차이 처리 후) |
| 🟡 J4 | `panic` 게이트 동기화 룰 강제 X — 수용 결정 (2026-05-05) | 사용자 결정 정책 (룰 추가 X = 무한 루프 회피). M-008 등재 패턴 = 사고 기록 + 사전 참조. INC-001 학습 = `feedback_deploy_env_sync.md` 인라인 룰 (강제 X 의도) |

---

## 우선순위 매트릭스 (2026-05-04 정정)

### 즉시 처리 권장 (위험 vs 비용 = 위험 우세)

| 우선 | 항목 | 사유 |
|:-:|------|------|
| 1 | **B1 rustls-webpki 3건 upgrade** | `cargo update` 1 명령 |
| ~~2~~ | ~~**B3 npm postcss + follow-redirects + basic-ftp HIGH**~~ | ✅ 해결 2026-05-04 (commit `ee68c7c`) |
| ~~3~~ | ~~**J1 RATE_LIMIT_TEXTBOOK_* 동기화**~~ | ✅ 해결 2026-05-05 (commit `7aae36a`) |
| ~~4~~ | ~~**B4 unwrap 위험 2건 (auth/service.rs:397, 1396)**~~ | ✅ 해결 2026-05-04 (commit `ad239ed`) |
| 5 | **C3+C4 rustfmt baseline** | 본 PR 결정 대기 |
| ~~6~~ | ~~**G6 dependabot 도입**~~ | ✅ 해결 2026-05-05 (commit `9367f72`, A4-8 동시) |
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
| B4 unwrap 위험 | 본 문서 직접 진입 | `src/api/auth/service.rs:397, 1396` |
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
