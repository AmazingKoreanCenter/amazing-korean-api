# AMK 사건 — MFA 세션 limit 초과 (2026-05-30)

> **목적**: 다음 세션 패치 작업의 단일 SoT. 원인·현상·결과·패치 계획 분류.
> **상태**: 즉시 조치 완료(Redis 수동 정리). 1단계 코드 트레이싱 + 전수 정합 감사(§6) + prod 실측(§6.5.1) + 패치 설계(§7) 완료. **영구 패치 6축 구현 완료 — 빌드/회귀테스트 통과, 사용자 검토·배포 대기 (§8).**
> **재발 위험**: 🔴 패치 미실행 시 7일 이내 재발 보장 (TTL=7일).
> **실측 진단 쿼리**: `scripts/diagnostics/` (read-only) — prod 8키 출처 확정 + 전 사용자 at-risk 스캔. EC2에서 실행 후 §6.5 채움.

---

## 1. 현상 (Symptom)

### 1.1 사용자 관점

- **발생 시점**: 2026-05-30 (전일까지 정상, 당일 갑자기)
- **흐름**: 사이트 → "Google로 로그인" → MFA 6자리 입력 화면 → Google Authenticator 코드 입력 → 실패
- **에러 1 (1차 시도, 코드 유효 시간 내)**:
  ```json
  {"error": {"code": "FORBIDDEN", "http_status": 403,
   "message": "AUTH_403_SESSION_LIMIT:2",
   "trace_id": "019e7695-49e2-7df2-b4eb-cfc58c29ebff"}}
  ```
- **에러 2 (즉시 재시도, 같은 인증코드 유효시간 내)**:
  ```json
  {"error": {"code": "UNAUTHORIZED", "http_status": 401,
   "message": "MFA_TOKEN_EXPIRED",
   "trace_id": "019e7695-95cf-7693-a551-5eb8b5e7123d"}}
  ```
- **사용자 측 화면 메시지**: "올바르지 않은 인증 코드입니다." (FE의 fallback 매핑, `frontend/src/category/auth/hook/use_mfa_login.ts:26`)

### 1.2 서버 관점 (api 로그)

- 11일간(2026-05-19 ~ 2026-05-29) 매일 `OAuth login: existing user 1 via google` 정상 로그가 떴음
- 그 중 5/19, 5/21~28에 `WARN: Refresh token reuse detected! Session: <uuid>` 매일 출력
- 2026-05-30 01:15:57 ~ 01:24:01 사이 OAuth 로그인 5번 시도 (사용자가 반복 클릭) → 모두 OAuth 단계는 success → 그 뒤 MFA 검증 실패
- MFA 검증 자체는 로그에 흔적 없음(mfa_login 함수에 명시적 info!/warn! 로깅 없음)

### 1.3 시스템 상태 (Redis, 사건 시점)

| 키 | 개수 | 의미 |
|---|---|---|
| `ak:user_sessions:1` (SET) | 8 (추정, 정리 후 빈 SET) | user_id=1의 session id들 |
| `ak:session:*` | **0** | Access 토큰들은 모두 자연 만료(짧은 TTL) |
| `ak:refresh:*` | **8** | ★ 누적된 refresh 키 — **원인** |
| `ak:mfa_pending:*` | 0 | TTL 5분, 자연 만료 |
| evicted_keys | 0 | LRU 축출 무관 |
| maxmemory_policy | `noeviction` | 정책상 키 사라질 일 없음 |

### 1.4 시스템 상태 (DB `login` 테이블, user_id=1)

```
 login_state | cnt
-------------+-----
 compromised |  24
 logged_out  |  17
 expired     |   6
 active      |   1 (방금 새로 만든 세션)
```

- **24개 `compromised: security_concern`** — 5/19~5/28 사이 거의 매일 1건씩
- 정리 직전엔 active=1+이전 stale 8개 합쳐 카운트가 9였을 가능성 (즉시 조치 후 1로 복귀)

---

## 2. 원인 (Cause)

### 2.1 정책 (배경)

| 역할 | 최대 세션 | 초과 시 |
|---|---|---|
| Learner | 5 | FIFO 자동 evict |
| Manager | 3 | 로그인 거부 (403) |
| **Admin** (해당 사용자) | **2** | **로그인 거부 (403)** |
| HYMN | 2 | 로그인 거부 (403) |

설정 위치: `src/config.rs:68-71`, `docker-compose.prod.yml:56-59` (`MAX_SESSIONS_ADMIN=2`)
거부 에러: `src/api/auth/service.rs:355` `AUTH_403_SESSION_LIMIT:{}` (`:{}` = limit 값)

### 2.2 직접 메커니즘 (확정 사실)

`Self::enforce_session_limit` (`src/api/auth/service.rs:198-290`) 의 카운팅 로직:

1. **`ak:user_sessions:{user_id}` SET** 의 모든 session_ids 조회
2. **1a (유령 1차)**: `ak:session:{sid}` 가 없는 것만 유령 후보
3. **1b**: 후보들의 `login_refresh_hash` 를 DB에서 배치 조회
4. **1c (유령 확정)**: 1차 후보 중 `ak:refresh:{hash}` **도** 없는 것만 진짜 유령
5. **1d**: 진짜 유령만 SET에서 제거 + DB `login_state='expired'`
6. **카운트**: SCARD(`ak:user_sessions:{user_id}`) = active_count
7. `if active_count < max_sessions` 면 통과; 아니면 Learner는 evict, **Admin은 403**

**핵심 (확정)**: `ak:refresh:{hash}` 가 살아있으면 그 세션은 1c 단계에서 유령으로 판정되지 **않음** → 활성 세션 수로 카운트됨.

사건 시점: `ak:refresh:*` **8개 잔존** → `active_count = 8` >> `max_sessions = 2` → **AUTH_403_SESSION_LIMIT:2** 반환.

### 2.3 누적 메커니즘 (가설 — **다음 세션 1단계 조사 필요**)

`ak:refresh` 키가 왜 8개나 누적됐는지가 진짜 근본 원인입니다. **자동 정리 경로가 모두 정상 동작하는 것으로 보이는데도 누적**됐다는 게 미스터리입니다:

#### 정상 동작 확인된 경로

| 경로 | 위치 | Redis 정리 |
|---|---|---|
| Refresh rotate 시 reuse 감지 | `service.rs:759-805` | ✅ `ak:refresh:{hash}` DEL + `ak:session:{sid}` DEL + `ak:user_sessions` SREM 모두 있음 |
| 유령 세션 cleanup | `service.rs:280-282` | ✅ ak:refresh 만료된 것만 정리 (정의상 정상) |
| Refresh rotate 성공 후 | (코드 추가 확인 필요) | ⚠️ 새 rotate 시 이전 hash의 ak:refresh 키 DEL 여부 확인 필요 |
| 새 OAuth 로그인 시 이전 세션 처리 | (코드 추가 확인 필요) | ⚠️ Admin이 새 OAuth 로그인 → 이전 세션의 ak:refresh 어떻게 처리되나? |

#### 미스터리 (조사 항목)

11일간 매일 새 OAuth 로그인이 성공했는데, `max_sessions_admin=2` 정책상 2일째부터 거부됐어야 함. 그런데 [E]의 12개 row(login_id 68~79)는 5/19부터 매일 새 세션 생성 성공을 보여줌. **두 가지 중 하나**:

- **(a) enforce_session_limit 이 OAuth 흐름의 어딘가에서 누락**되어 카운트 체크 없이 새 세션이 생성됨
- **(b) ak:user_sessions SET 이 매번 비워지는 다른 경로**가 있어 카운트가 매일 0에서 시작됨 → 그러나 ak:refresh 키는 SET 과 무관하게 남아있다가 결국 8개에 도달

이 둘 중 무엇인지 + 정확한 코드 경로 추적이 **패치 1단계**.

### 2.4 부수 관찰

매일 reuse 감지(`WARN: Refresh token reuse detected!`)가 발생한 원인 — 클라이언트 측 추정:
- 다중 탭/창이 동시에 자동 refresh 시도 → 한 탭이 새 hash로 rotate → 다른 탭이 old hash 사용
- 또는 브라우저 캐시된 stale refresh 토큰 사용
- frontend 토큰 저장·갱신 로직 점검 필요 (별도 트랙)

---

## 3. 결과 (Result)

### 3.1 영향

- **범위**: user_id=1 (Admin) 한 명
- **시간**: 2026-05-30 01:15 ~ 02:00 (약 45분, 사용자가 반복 시도하며 차단됨)
- **데이터 무결성**: ✅ 영향 없음 (마이그·DB 변경 0, 시드 데이터 0)
- **다른 사용자**: ✅ 영향 없음 (사용자별 SET이라 격리됨)

### 3.2 즉시 조치 (완료 2026-05-30 ~02:00)

```bash
# EC2 SSH 후
RU=$(docker exec amk-api printenv REDIS_URL)
RP=$(echo "$RU" | sed -E 's|redis://[^:]*:([^@]*)@.*|\1|')
RC() { docker exec amk-redis redis-cli -a "$RP" --no-auth-warning "$@"; }
for k in $(RC --scan --pattern 'ak:refresh:*'); do RC DEL "$k"; done
```

**효과**:
- `ak:refresh:*` 8 → 0
- 사용자가 다시 OAuth 로그인 시도 → enforce_session_limit 통과(0 < 2) → MFA 검증 통과 → 세션 생성 정상
- 검증: 정리 직후 `ak:session=1`, `ak:refresh=1` (방금 만든 1세트만), DB `login_state='active'=1` (login_id=79), session TTL=739s/refresh TTL=604,639s 모두 정책치
- 자동 부산물: 다음 OAuth 시 유령 cleanup 로직이 5/29 row(78)를 `expired/session_expired`로 자동 정리한 흔적 [E]에 보임

### 3.3 검증 명령 (재발 모니터링용)

prod EC2에서 주기적으로:
```bash
RC() { docker exec amk-redis redis-cli -a "$RP" --no-auth-warning "$@"; }
echo "user=1 SET 카운트: $(RC scard ak:user_sessions:1)"
echo "ak:refresh 전체: $(RC --scan --pattern 'ak:refresh:*' | wc -l)"
```
SET 카운트가 limit 근처(2)에 다가가면 사전 감지 가능.

---

## 4. 패치 계획 (다음 세션 진입점)

### 4.1 1단계 — 정확한 누적 메커니즘 규명 (선결)

**필수**: 패치 코드 짜기 전에 §2.3의 미스터리를 코드로 확정해야 함. 추측 패치 금지.

- [ ] `enforce_session_limit` 호출 그래프 전수 — OAuth 로그인 흐름(MFA enabled/disabled 각각), refresh rotate, 모든 진입점
  - 현재 알려진 호출 위치: `service.rs:591` (MFA 미사용 로그인), `service.rs:2276` (OAuth MFA 흐름 어딘가)
  - 누락된 호출 경로 있는지 확인
- [ ] 새 OAuth 로그인 시 이전 세션의 `ak:refresh:{hash}` 처리 경로 확인 — 즉시 DEL 하는지, 자연 만료 대기인지
- [ ] `ak:user_sessions:{user_id}` SET 에 sid 가 추가/제거되는 모든 위치 매핑
- [ ] [E]의 24개 compromised + 17개 logged_out + 6개 expired 분포를 시간순으로 추적 → 정상 정리 패턴 vs 누적 패턴 시간선상 비교

### 4.2 2단계 — 수정 방향 (3가지 후보, 1단계 결과로 선택)

**후보 A — 카운팅 SoT를 DB로 통일 (권장 1순위)**
- 현재: `SCARD(ak:user_sessions:{user_id})` (Redis SET)
- 변경: `SELECT COUNT(*) FROM login WHERE user_id=$1 AND login_state='active' AND login_expire_at > NOW()`
- 장점: DB가 SoT. Redis 키 누락/누적과 무관하게 정확한 카운트
- 단점: 매 로그인마다 추가 DB 쿼리 1회 (인덱스 추가 필요할 수 있음)
- 영향 범위: `service.rs:enforce_session_limit` 의 카운트 부분만

**후보 B — 유령 cleanup 강화 (보완)**
- 현재 1c 로직: `ak:refresh` 가 살아있으면 ghost 아님
- 변경: DB의 `login_state` 확인 추가 — state in ('compromised', 'logged_out', 'expired') 면 ak:refresh 살아있어도 ghost 처리 + 키 DEL
- 장점: 기존 구조 유지하며 누적 차단
- 단점: 1단계에서 발견된 진짜 누락 경로를 못 잡을 수 있음

**후보 C — 정기 background 정리 (Redis ↔ DB 동기 보장)**
- 별도 worker/scheduled task가 주기적으로 `ak:refresh:*` 스캔 → 각 hash에 대응하는 DB `login` row의 state 확인 → 'active' 아니면 Redis 키 DEL
- 장점: 누적이 발생해도 최대 N분 이내 자동 정리
- 단점: 새 인프라(스케줄러), 운영 복잡도 증가

**우선순위**: 1단계 결과 보고 결정. 현재 추정으로는 **A + B 동시 적용** 가능성이 가장 높음(A=근본 보장, B=방어선 추가).

### 4.3 3단계 — 회귀 테스트

- [ ] `enforce_session_limit` 단위 테스트: Admin 2개 세션 + 추가 1개 시도 → 403 검증
- [ ] 통합테스트: ak:user_sessions SET 과 DB login_state 불일치 시나리오 → 카운트 정확성 검증
- [ ] reuse detection 후 Redis 키 완전 정리 검증

### 4.4 4단계 — 운영 모니터링 (영구)

- [ ] CloudWatch/로그에 `AUTH_403_SESSION_LIMIT` 발생 시 알람
- [ ] `ak:refresh` 전체 키 수, 사용자별 `ak:user_sessions` SET 크기 메트릭 노출
- [ ] DB `login_state` 분포 일일 집계

### 4.5 부수 — frontend 토큰 갱신 로직 (별도 트랙)

매일 reuse detection이 발생한 원인 = 클라이언트 측 다중 탭/캐시 추정. 별도 PR로 다루기.

---

## 5. 다음 세션 진입점

1. 이 문서 정독 (특히 §2.3 미스터리, §4.1 1단계)
2. **시작점**: `src/api/auth/service.rs:198` `enforce_session_limit` 함수 + 호출 위치(`:591`, `:2276`) 트레이싱
3. **확정 사실 코드 인용**:
   - 세션 limit 거부: `service.rs:355` `AUTH_403_SESSION_LIMIT:{}`
   - reuse detection (정리 포함): `service.rs:759-805`
   - MFA 검증: `service.rs:2556 mfa_login`
   - MFA expired 에러: `service.rs:2575`
4. **prod 재발 모니터링 명령**: §3.3
5. **변경 금지 (스코프 외)**:
   - `users.user_mfa_secret` AAD 문자열 (`"users.user_mfa_secret"`) — 절대 변경 시 prod 복호화 깨짐
   - MFA 토큰 TTL(5분), 세션 limit 정책 값(2/2/3/5) — 정책 조정은 별건

---

## 6. 정합 감사 결과 (2026-05-30, 워크플로 93 에이전트)

> 12개 세션-사망 경로 × {ak:refresh / ak:session / SET 멤버 / DB state} 전수 정합 감사 + 적대적 검증(78 verdict, 39 leak 주장 중 검증 통과만 확정) + 실측 쿼리 생성. 추측 배제, 검증 통과 갭만 기록.

### 6.1 근본원인 (확정)

The ONE structural root cause is that the system maintains session liveness across three stores with no atomic write boundary and no time-driven reconciler: the DB `login.login_state`/`login_expire_at`, the per-key Redis TTLs (`ak:session` ~15min, `ak:refresh` 1–30d by role), and a no-TTL Redis SET `ak:user_sessions` counted by SCARD — and they are only reconciled lazily inside `enforce_session_limit` on the same user's next login. Clean paths (no surviving verified leak after both lenses): `logout_single`, `enforce_evict_learner`, and `password_reset_invalidate` — their orphan claims were refuted because every non-active state transition couples its `ak:refresh` DEL with the SET SREM, login rows are never hard-deleted (so the "no DB row" preconditions are unreachable), and any residue is DB-gated and TTL-bounded. Leaky paths: `mfa_disable_by_hymn`, `natural_ttl_expiry_residue`, `session_create_toctou`, the ban path, plus medium orphans in the two refresh races, reuse-detection, the everywhere-logout, and ghost-cleanup. Stale `login_state='active'` DB rows CAN accumulate indefinitely (a once-logged-in user who never returns, or a half-dead session whose `ak:refresh` outlives `ak:session`) and are never time-reaped — this directly poisons candidate A, so a DB count must filter `login_expire_at > NOW()` rather than `login_state='active'`. TOCTOU is real and confirmed on both `session_create_toctou` (concurrent same-user login overshoots reject-role caps with no eviction and no DB constraint, an accelerant for the documented AUTH_403 self-lock) and `mfa_disable_by_hymn` (a session created mid-disable fully survives); neither has any lock/SETNX/advisory-lock between the SCARD/snapshot check and the SADD/insert. The leak that no path or TTL within an acceptable bound ever cleans is the ban/account-deactivation bypass: `admin_update_user` flips only `users.user_state` while `refresh()` re-validates only `login_record.state=='active'` and never `user.user_state`, so a banned user keeps rotating refresh tokens and re-arming a 7–30d TTL effectively forever — the highest-severity, highest-confidence finding. Confidence is high on the structural mechanism and the ban/natural-expiry/TOCTOU findings (verified directly in code: `refresh` ignores `user_state`, the ban path has zero Redis/login_state calls, `find_login_refresh_hashes_by_session_ids` has no state filter, ghost-cleanup 1c spares live-`ak:refresh` sessions); the refuted false alarms (logout None-branch, learner-eviction orphans, reuse SET-member over-count) were correctly downgraded because their preconditions require a hard-deleted login row or a non-monotonic tri-state Redis failure that the code cannot produce.


### 6.2 정합 매트릭스 (12 경로)

| Path | Trigger | DB state update | ak:refresh | ak:session | SET member | Confirmed leaks |
|------|---------|-----------------|-----------|-----------|------------|-----------------|
| `refresh_rotation_race` | 2 concurrent refresh() with same old token (multi-tab) | converges to `compromised` (correct) | **raced** | cleaned | cleaned | 1 — `ak:refresh:H_new1` (winner's SET vs loser's DEL on same key, unordered → orphan to role TTL) |
| `refresh_reuse_detection` | replayed/rotated-out token, `incoming_hash != DB hash` | `compromised` (correct, committed before Redis) | **partial** | partial | partial | 1 — on swallowed `let _ =` Redis error, `ak:refresh:{current_hash}` + SET member survive (ak:session self-heals ≤15min); SET-member-count claim refuted; incoming_hash refuted |
| `logout_single` | authed POST /auth/logout | `logged_out` (any state, no filter) | **partial** | cleaned | cleaned | 0 — None-branch unreachable (rows never hard-deleted); TOCTOU new-hash inert (DB-gated + SREM); audit-row gap is absence-of-state, not a leak |
| `logout_all_everywhere` | logout-all `everywhere=true` | active-only `logged_out` | **missed** (for non-active members) | cleaned | cleaned | 1 — `ak:refresh:{compromised_hash}` seeded by reuse-branch swallowed error, then orphaned past whole-SET DEL (only role TTL clears). Active-only-orphan + whole-SET-DEL variants refuted (coupled DEL invariant) |
| `enforce_ghost_cleanup` | enforce_session_limit 1a–1d on login/OAuth | `expired` only when both keys gone; TOCTOU may wrongly flip `expired` | **raced** | na | cleaned (SREM `?`-propagated) | 1 — TOCTOU false-ghost: concurrent rotate sets `ak:refresh:H_new` then 1c reads stale H_old absent → SREM+DB `expired`, H_new orphaned. None-branch + SREM-then-UPDATE refuted |
| `enforce_evict_learner` | Learner login, SCARD ≥ 5 → FIFO | `revoked` for evicted active sids (correct) | cleaned (evicted) | cleaned (evicted) | **partial** | 0 — orphan-SET / orphan-keys / abort-branch all refuted (every non-active transition couples ak:refresh DEL + SREM; orphan precondition unreachable; abort self-heals ≤15min) |
| `password_reset_invalidate` | reset-confirm with valid token | active-only `revoked` (DB complete) | **partial** | cleaned | cleaned | 0 — no-DB-row orphan unreachable; rotation-race residue inert (DB-gated) + SET already DEL'd; ak:session/SET no-leak confirmed |
| `mfa_disable_by_hymn` | HYMN mfa_disable(target) | active-only `revoked` | **partial/missed** | partial | cleaned | 3 — (a) non-active-row `ak:refresh` orphan **[1 lens refuted, 1 confirmed → CONFIRMED]**; (b) `ak:session` orphan on swallowed DEL after commit (≤15min, accepted baseline by elsewhere-lens but race-lens confirms); (c) **race: session created mid-disable fully survives** (DB stays active, keys not in snapshot DEL) |
| `natural_ttl_expiry_residue` | no event; keys hit own TTL | **never** flips off `active` (no reaper) | na (TTL) | na (TTL) | **missed** | 4 — stale `active` DB row forever if user never returns; `active` + live ak:refresh half-dead (the 2026-05-30 accumulation); no-TTL SET member; `login_expire_at` populated-but-never-enforced. Inconsistent-definition confirmed; both-keys-gone SET-member refuted; login_expire_at race-lens refuted |
| `session_create_toctou` | concurrent same-user login/OAuth | all inserted `active`, no DB cap | **raced** | raced | raced | 2 — reject-role surplus (`ak:user_sessions` exceeds hard cap, no eviction, persists to refresh TTL = self-lock accelerator); surplus DB `active` row (no partial-unique constraint). Learner surplus + mfa_pending refuted |


### 6.3 확정 갭 (11건, 심각도순)


**G1 [HIGH] `account_deletion_or_ban (refresh_state / extractor)`** — ak:refresh:{hash} + continuously re-minted access tokens (refresh ignores user_state)

- 수정 방향: refresh() at service.rs:817 already fetches the user (user_repo::find_user returns user_state) but only reads user.user_auth. Add `if !user.user_state { return Err(AUTH_401) }` right after Step 4, mirroring the login gate at service.rs:481/513. This is the candidate-B principle applied to the user table: the refresh path must re-validate live account/DB state, not only login_record.state=='active'. Ban (admin_update_user) currently writes only the users table and does no session kill — also wire the ban path to invalidate_all_sessions (DEL ak:refresh/ak:session + whole ak:user_sessions SET + UPDATE login.login_state='revoked'), reusing the existing mfa_disable cleanup at service.rs:2748-2777.


**G2 [HIGH] `account_deletion_or_ban (DB login rows + ensure_session_active)`** — login.login_state stays 'active' for all banned-user sessions; ak:session authorizes until 15min TTL

- 수정 방향: ban path (admin/user/service.rs admin_update_user / bulk) sets users.user_state=false but never touches login or Redis. On deactivation, call the existing session-invalidation routine to flip login_state→'revoked' and DEL the Redis keys + SET for that user (same code path as mfa_disable service.rs:2748-2777). Candidate B: enrich ensure_session_active or add a DB-state cross-check so a deactivated user's still-live ak:session cannot pass for up to 15min on non-(get_me/settings) endpoints.


**G3 [HIGH] `natural_ttl_expiry_residue`** — login.login_state='active' for a physically-dead session (both Redis keys TTL-expired); never reconciled if user never logs in again

- 수정 방향: No process flips login_state off 'active' on time; correction only happens lazily inside enforce_session_limit on that SAME user's next login. Directly impacts candidate A: a DB COUNT WHERE login_state='active' would OVER-count these phantoms (false AUTH_403 for Admin/HYMN max 2). For candidate A, count WHERE login_expire_at > NOW() (the indexed time-truth) NOT login_state='active'. Better: add a time-based reaper that UPDATE login SET login_state='expired' WHERE login_state='active' AND login_expire_at < NOW() — login_expire_at + index_login_expire_at already exist and are unused for this.


**G4 [HIGH] `natural_ttl_expiry_residue / enforce_ghost_cleanup`** — login.login_state='active' + live ak:refresh while ak:session expired → SCARD-counted half-dead session (the 2026-05-30 Admin AUTH_403_SESSION_LIMIT:2 accumulation)

- 수정 방향: Ghost-cleanup 1c (service.rs:259) spares any session whose ak:refresh is still alive even though ak:session expired and the session is API-dead. Candidate B: make the count authoritative on DB, not on the no-TTL Redis SET — derive active_count from COUNT(login WHERE login_state='active' AND login_expire_at > NOW()) instead of SCARD ak:user_sessions, OR have ghost-cleanup consult login_expire_at (1c: if ak:session gone AND login_expire_at < NOW(), confirm ghost regardless of ak:refresh). Either removes the accumulation that requires the documented manual `DEL ak:refresh:*` recovery.


**G5 [HIGH] `session_create_toctou`** — ak:user_sessions:{user_id} surplus member + ak:session + ak:refresh for reject-roles (Admin/Manager/HYMN) — cap silently breached, no eviction

- 수정 방향: Check-then-act gap: SCARD (service.rs:285) → DB tx → SADD (service.rs:688/2361) with no lock. Atomicity fix: reserve the slot atomically before the DB tx — e.g. SADD a placeholder then SCARD (or Lua/atomic INCR with TTL) so concurrent logins see the reservation, or take a per-user pg_advisory_xact_lock(user_id) at the top of enforce_session_limit through commit. Candidate A as a backstop: add a partial-unique/EXCLUDE-style cap or a COUNT(login WHERE user_id=$ AND login_state='active') < max check inside the insert tx so the DB rejects the (max+1)th active row for reject-roles.


**G6 [HIGH] `mfa_disable_by_hymn`** — session created mid-disable fully survives (race): new login row stays login_state='active', its ak:session/ak:refresh are written after the snapshot-built DEL and are never targeted

- 수정 방향: mfa_disable SELECTs active sessions (snapshot), UPDATEs, commits, then DELs only snapshot keys — a concurrent login committed after the snapshot survives with an 'active' DB row. Atomicity: take pg_advisory_xact_lock(target_user_id) spanning the SELECT→UPDATE→Redis-DEL so concurrent session creation for that user serializes; OR re-run the active-session collection + DEL after commit (read-after-write) before returning. Combine with candidate-B user_state check in refresh so a survivor still cannot rotate.


**G7 [MEDIUM] `refresh_rotation_race`** — ak:refresh:H_new1 — rotation winner's SET vs reuse-loser's DEL on the same key, post-commit and unordered (SET-after-DEL orphans it)

- 수정 방향: Two pooled Redis connections, no cross-connection ordering. Make the rotation winner's `DEL old + SET new` and the loser's reuse cleanup atomic relative to each other — gate the post-commit Redis ops under the same per-session lock used for the DB FOR UPDATE (extend its scope past commit), or use a Lua script keyed on the DB-current hash so the loser cannot DEL a key the winner just legitimately set. Self-heals only at role TTL (1d/7d/30d) otherwise; not an auth bypass (state-gate at 812).


**G8 [MEDIUM] `enforce_ghost_cleanup`** — ak:refresh:H_new (TOCTOU false-ghost) — concurrent rotate sets H_new, then 1c reads stale H_old absent → session wrongly flipped 'expired' and H_new orphaned

- 수정 방향: 1a snapshot / 1b DB read / 1c EXISTS are non-atomic across a DB round-trip with no row lock; rotate's plain-pool 'expired' UPDATE (no login_state='active' guard, repo.rs:611) then overwrites a freshly-rotated active row. Candidate B + atomicity: add `AND login_state='active'` (or a recompute) before flipping ghosts to 'expired' so a row a concurrent rotate kept active is not spuriously revoked; or re-read the refresh hash under a per-user lock so 1c sees H_new. Removes both the spurious revocation and the orphaned H_new.


**G9 [MEDIUM] `refresh_reuse_detection`** — ak:refresh:{current_hash} + ak:user_sessions SET member survive a transient Redis error (swallowed `let _ =` at service.rs:795-806) after DB committed 'compromised'

- 수정 방향: The reuse branch swallows Redis errors (`let _ =`) while the rotation/logout blocks 60 lines away fail-closed with `.map_err(...)?`. Make the three reuse-cleanup ops fail-closed (`?`) like the rest of the function, or best-effort-with-retry, so a transient Redis blip cannot leave a compromised session's keys live (ak:session self-heals ≤15min; ak:refresh + SET member ride role TTL and keep the session SCARD-counted, feeding the AUTH_403 accumulation).


**G10 [MEDIUM] `logout_all_everywhere`** — ak:refresh:{compromised_hash} orphaned past whole-SET DEL — seeded by the reuse-branch swallowed error, then unreachable to any cleanup (only role TTL clears it)

- 수정 방향: Root cause is the same swallowed `let _ =` in reuse detection (service.rs:795-806) leaving a non-active SET member with a live ak:refresh; logout_all's active-only SELECT (repo.rs:510) skips it and the whole-SET DEL (service.rs:1268) removes its last anchor. Fix at source (make reuse cleanup fail-closed). Defense-in-depth in the everywhere path: SMEMBERS-then-clean each member's ak:refresh (look up hashes WITHOUT the active filter via find_login_refresh_hashes_by_session_ids) before the whole-SET DEL, matching the ghost-cleanup pattern.


**G11 [MEDIUM] `natural_ttl_expiry_residue`** — login_expire_at populated + indexed but never used as a cleanup/transition predicate (only read by admin stats); active-session definition diverges 3 ways

- 수정 방향: login_expire_at (with index_login_expire_at) is the correct time-truth SoT but no query reconciles login_state against it; admin stats use WHERE login_expire_at > NOW() while enforcement uses SCARD and FIFO/bulk paths use login_state='active'. Unify: pick login_expire_at > NOW() as the single active-session predicate (drives a safe candidate A) and add the time-based reaper described above. This is also the safer source for any DB-count migration since it does not over-count stale 'active' rows.


### 6.4 패치 설계 함의 (감사로 정정된 핵심)

- **후보 A 보정 필수**: `login_state='active'` 단독 COUNT는 **유령 active row 과대계수**(reaper 부재 → 영구 잔존) → 거짓 403 재발. 반드시 `AND login_expire_at > now()` (인덱스 `index_login_expire_at` 기존재, 미사용). natural_ttl 발견이 "A만으로 충분" 반증.

- **후보 B**: ghost-cleanup 1c가 `ak:refresh` 살아있으면 무조건 보존 → 반-죽은 세션 누적(=5/30 사건). 1c에 `login_expire_at < now()` 시 ghost 확정 추가, 또는 카운트 SoT를 DB로.

- **TOCTOU 원자성 (신규, A/B 둘 다 미해결)**: enforce SCARD↔SADD 사이 락 없음 → 동시 로그인이 reject-role 한도 무음 돌파. `pg_advisory_xact_lock(user_id)` 또는 슬롯 원자 예약 필요. session_create + mfa_disable 둘 다 해당.

- **time-based reaper (신규)**: `UPDATE login SET login_state='expired' WHERE login_state='active' AND login_expire_at < now()` 주기 실행 — 유령 row 영구 잔존 근절. 코드 내 background 정합기 전무 확인됨.

- **reuse cleanup fail-closed**: service.rs:795-806 `let _ =` (에러 무시) → 60줄 위 rotation/logout는 `?` fail-closed. 일관성 위해 `?`로. (G9가 G3 logout_all 고아의 source.)

- **🔴 ban/탈퇴 우회 (G1/G2, → 본 패치에 통합. 결정 2026-05-30)**: `admin_update_user`가 `users.user_state`만 변경, login 세션·Redis 무손. `refresh()`는 `login_record.state`만 검사하고 `user.user_state` 무시 → **정지 사용자가 7~30d 영구 토큰 갱신·전 API 접근 유지.** 독립된 최고 심각도 갭이나 세션 정합 패치와 함께 처리. refresh에 user_state 게이트(즉시 출혈 차단) + ban 시 세션 무효화(mfa_disable 정리 재사용) + extractor DB-state 크로스체크.

> **결정 (2026-05-30, §6 감사 후)**: ① ban 갭(G1/G2)은 본 세션 정합 패치에 **통합**. ② 패치 설계는 **prod 실측 결과(§6.5) 확정 후 착수**. 11갭 통합 패치 축 = A(`+login_expire_at`) · B(ghost 1c expire 확인) · TOCTOU 원자성(advisory lock) · time reaper · reuse fail-closed · ban 통합.


### 6.5 실측 진단 (prod, EC2에서 실행 — 결과 기입 대기)

read-only 번들 `scripts/diagnostics/` (전부 SELECT/SCAN, 변경 0):

- `session_divergence_all_users.sql` — 전 사용자 상태분포(Q1) + active_db vs active_live(time-truth) over-limit 플래그(Q2) + 테이블 전체 드리프트(Q3)

- `session_user1_correlation.sql` — user_1 login_id 68~79 시간순(R2) + active+refresh_hash 행(R3) + Redis session_id↔DB 분류 조인(R4)

- `session_orphan_scan.sh` — ak:refresh:* 카운트[1] + SCARD ak:user_sessions:1[2] + 각 키 GET·TTL[3] + 반-죽은 부분집합[5]. SCAN만(KEYS * 금지), DEL 없음.

- **판정**: ORPHAN 행 수 = (8 − LEGIT 행 수) = 8키 누적 정확 출처. Q2 `over_limit_live=true`인 다른 사용자 = 잠복 지뢰.

- **[x] prod 실행 완료 (2026-05-30, 즉시조치 이후 시점)** — 아래 §6.5.1.

#### 6.5.1 prod 실측 결과 (2026-05-30)

**Redis (현재, 즉시조치 후):** `ak:refresh total = 0`, `SCARD ak:user_sessions:1 = 0`, SMEMBERS 빈. → 8키는 즉시조치 DEL로 제거됨 + 02:00 복구 로그인(login_id 79)도 이미 다시 `compromised`로 정리됨. **orphan 분류 쿼리(③)는 moot**(라이브 키 0).

**DB at-risk 스캔 (active>0인 사용자 전체):**

| user | role | acct_active | active_db | **active_live** | **stale_active** | max |
|---|---|---|---|---|---|---|
| 4 | learner | t | 5 | 4 | **1** | 5 |
| 2 | learner | t | 1 | **0** | **1** | 5 |
| 7 | learner | t | 1 | 1 | 0 | 5 |
| 6 | learner | t | 1 | 1 | 0 | 5 |
| 3 | learner | t | 1 | **0** | **1** | 5 |

→ **user_1(Admin)은 목록에 없음 = DB active_db=0.** Redis SCARD 8은 순수 발산. reject-role 초과자 현재 0(불 꺼짐). **G3 stale-active 유령 prod 실증**(user 2·3·4) — `state='active'` 단독은 과대계수. reaper + `login_expire_at` 필터 필수 재확인.

**user_1 타임라인 (login 33~79):** 매일 1세션(거의 00:1x~00:5x UTC = KST 아침) 생성 → **대부분 `compromised: security_concern`**, 일부 `expired: session_expired`(ghost-cleanup 1d), 오래된 것 `logged_out`. 전부 `has_hash=t`(DB가 refresh hash 보존). 사건 시점(05-30 01:1x) 직전 7일 내(login 72~78) ≈ 7~8세션의 `login_expire_at` 미래(refresh-TTL 생존) = **8 ak:refresh 키의 출처**. DB는 compromised인데 Redis 키 생존 = 감사 예측대로의 Redis↔DB 발산. **복구 세션 79도 이미 compromised = 일일 reuse 엔진(프론트 멀티탭, §4.5) 여전히 작동 → 패치 없으면 ~7일 내 재누적·재차단.**

**결정타**: DB가 user_1을 active=0으로 정확히 앎 → **후보 A(`active' AND login_expire_at>now()` COUNT)였다면 0 반환, 절대 차단 안 됨.** 수정 방향 prod 확정.

> **남은 미세 확인(선택, 비차단)**: 8키가 (i) reuse가 DEL 못 한 current-hash인지 (ii) rotation race(G7) 고아 H_new인지의 정확 분해 — `login_log`의 rotate/reuse 이벤트 시퀀스로만 확정 가능(Redis SET 스냅샷은 소멸). 패치는 후보 A로 양쪽 모두 무력화하므로 불요. 운영 이해 목적이면 login_log 딥다이브 가능.


---

## 7. 패치 설계안 (2026-05-30, 워크플로 13 에이전트 · 적대적 검증)

> 6축 코드-grounding 설계 → 축별 적대적 검증(컴파일 적합성/축간 충돌/제약 위반/clean 경로) → 통합 합성. **마이그레이션 0, 코드 변경은 승인 후.** A+C는 분리 불가(하나의 원자 tx).


### 7.1 설계 narrative

## MFA Session-Limit Incident — Unified Patch Plan (2026-05-30)

**Root cause recap.** Session liveness is split across three stores: DB (`login.login_state` / `login_expire_at`), per-key Redis TTLs (`ak:session`, `ak:refresh`), and a **no-TTL Redis SET** `ak:user_sessions` counted by `SCARD`. `enforce_session_limit` (service.rs:198) derives its admission decision from `SCARD` at line 284. Because the 3-stage ghost-cleanup (1a–1d) keeps any sid whose `ak:refresh` still EXISTS (refresh TTL up to 7d for Admin), 8 stale members survived cleanup while the DB had **zero** active rows → `SCARD=8 ≥ max=2` → `AUTH_403_SESSION_LIMIT:2`. There is no time-based reaper, so DB rows with `login_state=active AND login_expire_at < now()` (prod users 2/3/4) also phantom-inflate any naive count.

The fix has six coordinated axes. **A and C are inseparable** and ship as one atomic count+insert transaction under a per-user advisory lock. B and D are decoupled hygiene/correctness. E and F are independently shippable hardening.

---

### Axis A + C (MERGED) — DB-authoritative count, atomic under advisory lock
**WHAT.** Add two repo helpers and move the *admission decision* inside the login/oauth DB transaction:
- `AuthRepo::acquire_user_session_lock_tx(tx, user_id)` → `SELECT pg_advisory_xact_lock(2, $1)` (**two-arg form**, namespace `2`, to avoid keyspace collision with ebook's single-arg `pg_advisory_xact_lock(1701011563)` and textbook's `pg_advisory_xact_lock($1)`).
- `AuthRepo::count_active_sessions_tx(tx, user_id) -> i64` → `count(*) FROM public.login WHERE user_id=$1 AND login_state='active'::login_state_enum AND (login_expire_at IS NULL OR login_expire_at > now())`.
- New private `AuthService::enforce_admission_in_tx(tx, st, user_id, user_auth)` that runs `acquire_lock → count → if active >= max && !evict_role: return Forbidden("AUTH_403_SESSION_LIMIT:{max}")`. The lock is held to commit, so a second concurrent same-user login blocks, re-reads the now-incremented count, and gets a clean 403 — TOCTOU closed.
- In `login()` (tx at service.rs:623) and `create_oauth_session()` (tx at service.rs:2287, the shared OAuth + `mfa_login` path = the literal incident path), call `enforce_admission_in_tx` as the FIRST statement after `begin()`, before `insert_login_record(_oauth)_tx`.

**WHY.** The DB is the only store with time-based truth (`login_expire_at`). `count(*)` with the `> now()` predicate is correct *even before the reaper exists* because it excludes stale_active phantoms at read time. Doing it inside the locked tx makes count+insert atomic, eliminating both the persistent SCARD over-count and the same-user burst over-admission (admin clicking login N times exceeding cap 2).

**CRITICAL DELETION (incident fix).** In the *pre-tx* `enforce_session_limit`, **delete the reject-role 403 branch** (service.rs:353-359) and the SCARD-based early-return gate at 284-290 *as the admission authority*. `enforce_session_limit` is RENAMED-in-comment to "pre-lock hygiene + Learner FIFO eviction pass" — it keeps the ghost-cleanup (now hygiene-only, Axis B) and the Learner eviction branch (293-352), but it NO LONGER 403s anyone. If this branch is left in place, Admin/Manager/HYMN still get the false 403 from the divergent SCARD before ever reaching the tx, and the incident is **not** fixed. The sole admission authority becomes `enforce_admission_in_tx`.

**Ordering.** For Learner (evict role) the pre-lock FIFO pass already trimmed to <max, so `enforce_admission_in_tx` short-circuits on `is_session_evict_role` and never 403s. For reject roles the in-tx count is authoritative.

---

### Axis B — Ghost-cleanup demoted to Redis hygiene only
**WHAT.** Keep the `ak:user_sessions` SET (still consumed by bulk-logout fan-out and login/oauth `sadd`). Demote the 3-stage Redis ghost-cleanup from *load-bearing* to *hygiene*: its only job is to keep the no-TTL SET from accreting dead sids. The DB-side `update_login_states_by_sessions(..,'expired',..)` (274-280) is **removed from the login hot path** — the reaper (Axis D) becomes the single writer of time-based `expired`. SREM hygiene becomes **fail-open** (warn, never block the login hot path), since the count no longer depends on Redis. `smembers` stays **fail-closed** (consistent with the file's documented fail-closed philosophy) — do not relax it.

**WHY.** Once Axis A counts from the DB, the SET is no longer the limit oracle, so its drift cannot over-count. Cleanup narrows to bounding SET growth. **Do NOT delete `find_login_refresh_hashes_by_session_ids`** — confirmed 3 live callers (service.rs:234 enforce, 1242 logout_all, 1556 password_reset); deleting it breaks two clean paths. The 1b/1c Redis-EXISTS stages used inside *enforce* can be simplified to a single DB-truth check (reuse the live-sids the count already computed), but this is optional polish; the load-bearing change is that the SET is no longer the oracle.

**Honest scope note.** Both bulk-logout paths (`mfa_disable` 2750-2777, `logout_all` 1201-1268) fan out by **DB user_id then DEL the whole SET key** — neither iterates SET membership. So the SET is nearly vestigial post-A; hygiene is cheap insurance against unbounded growth, not correctness-critical for any consumer. Stated plainly rather than overclaimed.

---

### Axis D — Background time-based session reaper
**WHAT.** New `src/jobs/mod.rs` + `src/jobs/session_reaper.rs`; `pub mod jobs;` in lib.rs (after `pub mod extract;`, preserving alpha order). New `AuthRepo::reap_expired_sessions(pool) -> AppResult<u64>`: `UPDATE public.login SET login_state='expired', login_revoked_reason='ttl_reaped', login_updated_at=now() WHERE login_state='active' AND login_expire_at IS NOT NULL AND login_expire_at < now()`. A detached `tokio::spawn` interval loop calls it; interval from new `SESSION_REAPER_INTERVAL_SEC` (default 300); `<=0` disables (boot-safe, no panic gate). Spawned in main.rs before `axum::serve` — capture `let reaper_db = app_state.db.clone();` **before** line 226 (where `app_state` is moved into `app_router`), then spawn after router build.

**WHY.** Flips stale_active phantom rows to `expired` so dashboards, the refresh gate, and the Learner-eviction query read truth. **Add `"time"` to tokio features in Cargo.toml** — `tokio::time::interval`/`MissedTickBehavior` currently compile only via transitive feature unification; make it explicit.

**Honest dependency.** The reaper ALONE does not close the incident (the 403 came from Redis SCARD, which the reaper never touches). It must land **with Axis A**. Use the **identical predicate** `login_state='active' AND login_expire_at > now()` in A's count so the ~300s reaper lag cannot cause a count/reaper mismatch.

---

### Axis E — Fail-closed reuse-detection Redis cleanup
**WHAT.** Convert the three swallowed `let _ = redis_conn.del/srem(...).await;` ops at service.rs:795-806 into fail-closed `let _: () = redis_conn.<op>(...).await.map_err(|e| AppError::Internal(...))?;` — the exact template already blessed in `logout_single` (1139-1148). Drop the turbofish; the `let _: ()` binding drives inference. Keep all three ops (incl. SREM). The tx already committed at 792 (session durably `compromised`), so a 500 here causes no double-commit; client retry hits the state gate (812 → 401).

**WHY.** Today a swallowed Redis failure on the reuse path leaves the compromised session's `ak:session`/`ak:refresh` keys alive AND orphans a SET member → contributes to the exact SCARD over-count behind the incident. The 409→500 change only fires when Redis is down (auth already degraded). No existing test asserts the reuse path returns exactly 409 (verified: `AUTH_409_REUSE_DETECTED` has no test/handler key), so nothing breaks.

---

### Axis F — Ban/deactivation session revocation
**WHAT.**
1. **Refresh user_state gate** (service.rs after 819): add `if !user.user_state { return Err(AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into())); }` right after `user_repo::find_user`. Generic 401 (not `ACCOUNT_DISABLED`) for anti-enumeration consistency with sibling refresh errors. Placed AFTER reuse-detection (759-808) and AFTER the state gate (812), so compromise handling is unaffected.
2. **Shared helper** `AuthService::invalidate_all_sessions(st, user_id, reason: &str)` extracted verbatim from the `mfa_disable` cleanup block (2748-2777): own short tx → `find_user_sessions_with_refresh_tx` → `update_login_state_by_user_tx(.., "revoked", Some(reason))` → commit → single Redis `DEL` of all `ak:refresh:{hash}` + `ak:session:{sid}` + the `ak:user_sessions:{user_id}` SET key (best-effort `.unwrap_or(())`). Refactor `mfa_disable` to call it with `reason="mfa_disabled"` (behavior preserved 1:1, only reason parameterized).
3. **Wire into admin deactivation** — the **only** paths that write `user_state=false` are in `src/api/admin/user/` (CONTEXT pointed at `src/api/user/`, which is incorrect — corrected by grep). Single path `admin_update_user` (service.rs:625-640, the existing post-commit transition-detect block): add `if !new_state { if let Err(e) = AuthService::invalidate_all_sessions(st, updated.id, "account_deactivated").await { warn!(...) } }`. Bulk path `admin_update_users_bulk` (inside the async closure, after `tx.commit()` at 756, before `Ok(updated_user)` at 758): `if matches!(item.user_state, Some(false)) && target_user.user_state { ...warn-on-err... }` — gate on the actual `true→false` transition (`target_user` already fetched at 671) to avoid no-op round-trips. Add `use crate::api::auth::service::AuthService;` to the admin service (confirmed not imported).

**WHY.** Closes the window where a deactivated user keeps minting access tokens via a valid refresh cookie (TTL 7d Admin / 30d Learner). The SET `DEL` also pre-empts SCARD over-count for banned users. `login_revoked_reason` is free-text (migration line 136), so `"account_deactivated"` is valid.

**Extractor cross-check — deliberately NOT added.** `ensure_session_active` (session.rs:26) runs in the auth extractor on EVERY request. Adding a per-request DB `user_state` SELECT to defend a ≤15min window (`ak:session` TTL) that helper #2 already closes on the Redis-healthy path is not justified. Residual: if the Redis `DEL` silently fails, a live `ak:session` authorizes a just-deactivated user for ≤15min; the refresh gate prevents re-minting past that. Bounded and documented.

---

### A+C merge — explicit interaction summary
A provides the *time-correct count query*; C provides the *atomicity substrate* (advisory lock + count-inside-tx). They are the **same function pair** and MUST ship together: `count_active_sessions_tx` is meaningless racy without the lock, and the lock is pointless if the count it guards is wrong (SCARD). The merged result: one serialized DB transaction per user — `lock → count → admit/403 → insert → commit` — under which the SCARD divergence and the same-user burst race are both structurally impossible. The pre-tx `enforce_session_limit` retains ONLY Learner FIFO eviction + Redis hygiene (Axis B); its reject-role 403 is deleted.


### 7.2 파일 변경표

| File | Change | New/Edit |
|------|--------|----------|
| `src/api/auth/repo.rs` | ADD `acquire_user_session_lock_tx(tx, user_id)` → `SELECT pg_advisory_xact_lock(2, $1)` (two-arg, namespace 2) | Edit |
| `src/api/auth/repo.rs` | ADD `count_active_sessions_tx(tx, user_id) -> i64` (`login_state='active' AND (login_expire_at IS NULL OR login_expire_at > now())`) | Edit |
| `src/api/auth/repo.rs` | ADD `reap_expired_sessions(pool) -> u64` (single time-based UPDATE → 'expired'/'ttl_reaped') | Edit |
| `src/api/auth/repo.rs` | (Optional B polish) add `AND login_expire_at > now()` to `find_active_sessions_oldest` (565) so live-set == evictable-set; **DO NOT delete** `find_login_refresh_hashes_by_session_ids` (3 callers) | Edit |
| `src/api/auth/service.rs` | DELETE reject-role 403 branch (353-359) + SCARD admission gate (284-290) from `enforce_session_limit`; keep Learner FIFO + demote ghost-cleanup to hygiene (B); remove in-line DB 'expired' UPDATE 274-280 (D owns it) | Edit |
| `src/api/auth/service.rs` | ADD private `enforce_admission_in_tx(tx, st, user_id, user_auth)`; call it first inside `login()` tx (623) and `create_oauth_session()` tx (2287) | Edit |
| `src/api/auth/service.rs` | Convert reuse-detection Redis ops (795-806) to fail-closed `let _: () = ...map_err(Internal)?` (Axis E) | Edit |
| `src/api/auth/service.rs` | ADD refresh `user_state` gate after 819; ADD `invalidate_all_sessions(st, user_id, reason)` (~2713); refactor `mfa_disable` (2748-2777) to call it with `"mfa_disabled"` (Axis F) | Edit |
| `src/api/admin/user/service.rs` | Wire `invalidate_all_sessions(.., "account_deactivated")` into `admin_update_user` transition block (625-640) and bulk path (after commit 756, gated on `Some(false) && target_user.user_state`); add `use crate::api::auth::service::AuthService;` | Edit |
| `src/jobs/mod.rs` | NEW — `pub mod session_reaper;` | New |
| `src/jobs/session_reaper.rs` | NEW — `spawn(db, interval_sec)` interval loop; `<=0` disables | New |
| `src/lib.rs` | ADD `pub mod jobs;` after `pub mod extract;` (preserve alpha order) | Edit |
| `src/main.rs` | Capture `let reaper_db = app_state.db.clone();` before line 226; spawn reaper before `axum::serve` (245) | Edit |
| `src/config.rs` | ADD `session_reaper_interval_sec: i64` field (~71), `from_env` parse default 300 (~303), Self{} init (~585), Debug impl | Edit |
| `Cargo.toml` | ADD `"time"` to tokio features (line 27) | Edit |
| `docker-compose.prod.yml` | ADD `SESSION_REAPER_INTERVAL_SEC: ${SESSION_REAPER_INTERVAL_SEC:-300}` near MAX_SESSIONS_* | Edit |
| `docs/AMK_DEPLOY_OPS.md` | ADD `SESSION_REAPER_INTERVAL_SEC` to env table (default 300, amk_app UPDATE confirmed, no migration) | Edit |
| `docs/AMK_API_AUTH.md` | Document reaper + DB-authoritative count + advisory-lock atomicity + ban-revocation in session lifecycle | Edit |
| `docs/AMK_STATUS.md` / `docs/AMK_CHANGELOG.md` | Per CLAUDE.md: STATUS checkbox + CHANGELOG entry | Edit |
| `docs/AMK_INCIDENT_2026-05-30_MFA_REFRESH_REUSE.md` | Record permanent-patch resolution + extractor-cross-check decision + residual risks | Edit |
| `tests/auth_login_integration.rs` (+ repo/oauth integration tests) | Add TOCTOU, phantom-exclusion, ban-revocation, reaper, mfa_disable-parity tests | Edit |


### 7.3 마이그레이션

NONE — no schema migration required. Confirmed by reading migrations/20260208_AMK_V1.sql against live code:
- `login_expire_at timestamptz` (line 134), populated on insert (repo.rs:142 `NOW()+make_interval`) and rotation (repo.rs:439).
- `login_state login_state_enum NOT NULL DEFAULT 'active'` (135); enum includes 'expired' and 'revoked' (line 13).
- `login_revoked_reason text` (136) — free-text, so 'ttl_reaped' / 'account_deactivated' are valid without enum change.
- `login_updated_at` exists; no UPDATE trigger (only insert DEFAULT now()), so explicit SET is safe.
- Indexes: `index_login_active_by_user(user_id, login_state)` (542), `index_login_expire_at` (541), `index_login_state` (539) — all present.
- login table NOT renamed by 20260519/20520/20521 (confirmed in schema-naming track).

All new SQL is plain SELECT count(*) / UPDATE / pg_advisory_xact_lock — within amk_app NOSUPERUSER grants (amk_app OWNs login and already runs advisory locks via textbook/ebook + the mfa_disable UPDATE in prod).

CONFIRM-BEFORE-IMPLEMENT GATE (per migration_safety memory — schema SoT is the migration file but the env may have drifted): in the target env run `\d login` to re-verify login_expire_at + index_login_active_by_user before relying on the index-backed count(*). No DDL to apply either way.


### 7.4 롤아웃 순서

1. 0. PRE-FLIGHT (in target env): run `\d login` to re-verify login_expire_at + index_login_active_by_user + login_revoked_reason present (schema SoT = migration file, but confirm no drift). No DDL to apply.
2. 1. Cargo.toml: add tokio "time" feature (foundational — D won't compile reliably without it).
3. 2. Axis A+C repo helpers: acquire_user_session_lock_tx (two-arg, ns=2), count_active_sessions_tx, + reap_expired_sessions; optional find_active_sessions_oldest predicate alignment. (Repo-only, no behavior change yet.)
4. 3. Axis A+C service: add enforce_admission_in_tx; wire into login() and create_oauth_session() tx; DELETE reject-role 403 + SCARD admission gate from enforce_session_limit (the incident fix); demote ghost-cleanup to hygiene (B), remove in-line DB 'expired' UPDATE.
5. 4. Axis E: fail-close the reuse-detection Redis ops (independent, tiny).
6. 5. Axis F: refresh user_state gate; extract invalidate_all_sessions; refactor mfa_disable; wire admin deactivation single + bulk paths; add AuthService import.
7. 6. Axis D: jobs module + session_reaper; lib.rs pub mod jobs; main.rs spawn (capture db clone before app_state move); config.rs field + from_env + Self{} + Debug; docker-compose env line.
8. 7. Verify: cargo check + clippy + fmt --check; run the integration test suite (TOCTOU, phantom, ban, reaper, mfa_disable parity); frontend npm run build (per CLAUDE.md flow even if no FE change).
9. 8. Docs (per CLAUDE.md sync): AMK_DEPLOY_OPS env table, AMK_API_AUTH lifecycle, AMK_STATUS checkbox, AMK_CHANGELOG, INCIDENT doc resolution + residual-risk + extractor decision.
10. 9. Deploy to EC2; smoke: /health 200, then targeted prod verification — log in as the previously-stuck Admin (was 403'd, EXPECT admitted now), confirm reaper info-log fires within one interval, confirm a test deactivation revokes sessions. Update memory project_mfa_session_limit_patch.md + git reset --hard origin/main after merge (feedback_git_branching).

### 7.5 사용자 결정사항 (검토 필요)


**D1. Ship order — fast hotfix vs single PR?**
- 옵션: (1) Single PR with all 6 axes. (2) Hotfix PR with Axis A+C+B-deletion (the actual incident fix) first, then a follow-up PR with D/E/F hardening.
- ▶ 권장: Single PR. A+C+the reject-403 deletion are inseparable and constitute the real fix; E and F are small and low-risk; D is independently safe (disabled-by-default if interval<=0). Splitting adds two deploy/verify cycles for marginal safety. The immediate prod symptom is already cleared (manual Redis DEL on 2026-05-30), so there is no live fire forcing a same-day hotfix — a single reviewed PR is safer than a rushed split. Caveat: if you want the refresh user_state gate (Axis F #1) in prod ASAP independent of the count rework, it is a genuine 2-line standalone hotfix candidate.

**D2. Reaper interval (SESSION_REAPER_INTERVAL_SEC default)**
- 옵션: 300s (5min) vs 60s.
- ▶ 권장: 300s. The count's `login_expire_at > now()` predicate already makes admission correct at read time, so the reaper is hygiene; 300s bounds table growth with negligible query load. 60s only tightens the dashboard-staleness window 5x for 5x the query frequency — not worth it at current scale (prod confirmed only a handful of stale rows). Tunable via env without redeploy if needed.

**D3. Redis orphan reconciliation in the reaper (SCAN/DEL of orphaned ak:refresh / ak:user_sessions members)?**
- 옵션: (a) No — reaper touches DB only; leave Redis to per-key TTLs + lazy hygiene. (b) Yes — reaper also SCANs and prunes Redis.
- ▶ 권장: (a) No, defer. ak:session self-expires at 15min TTL; ak:refresh at role TTL; the no-TTL SET is bounded by Axis B hygiene + bulk-logout whole-key DELs. Adding Redis SCAN to the reaper is heavier (per-row hash fetch), needs cursor handling, and the SET is nearly vestigial post-Axis-A. Karpathy #2 (don't pre-abstract). Revisit only if SET memory growth is observed.

**D4. Per-request extractor user_state cross-check (instant ban lockout even under Redis-DEL failure)?**
- 옵션: (a) No DB check in ensure_session_active (rely on the helper's SET DEL + refresh gate; ≤15min residual on Redis-DEL failure). (b) Add a per-request DB SELECT on user_state.
- ▶ 권장: (a) No. Option (b) imposes a DB hit on EVERY authenticated request across the whole API to defend a ≤15min worst-case that only occurs if a Redis DEL silently failed — a cost/benefit loss. The refresh gate (Axis F #1) prevents re-minting past 15min. If a hard requirement for instant lockout under Redis failure emerges, the cheaper path is a Redis MULTI on the DEL, not a per-request DB read. Document the residual in the audit doc.

**D5. Keep ak:user_sessions SET + SCARD as a secondary signal, or remove the SET entirely?**
- 옵션: (a) Keep SET (sadd on login, DEL on bulk-logout), drop SCARD from the decision only. (b) Remove the SET + all sadd/scard.
- ▶ 권장: (a) Keep the SET, remove SCARD from the admission decision. The SET is still the bulk-logout fan-out vehicle (mfa_disable, logout_all DEL the whole key) and removing it is a larger, riskier diff touching login/oauth/logout/mfa_disable/password_reset. Honest caveat: post-Axis-A the SET is nearly vestigial (bulk-logout is DB-driven), so a future cleanup PR could remove it — but that is scope creep here (Karpathy #3). Keep + bound via hygiene now.

**D6. Self-deactivation guard in admin_update_user?**
- 옵션: (a) No guard (current behavior — admin can deactivate self; invalidate_all_sessions then revokes their own live sessions, current request returns 200, next request 401s). (b) Add MFA_CANNOT_DISABLE_SELF-style guard.
- ▶ 권장: (a) No guard under this axis — it is a UX decision, not a security gap, and adding it is out of scope (Karpathy #3). Behavior is acceptable/expected. Flagged for a separate decision if you want the UX safety rail.

**D7. Align find_active_sessions_oldest predicate with the count (add AND login_expire_at > now())?**
- 옵션: (a) Add the predicate so eviction and count operate on the identical set. (b) Leave eviction active-only and rely on the reaper to flip phantoms first.
- ▶ 권장: (a) Add it — one-line change, makes live-set == evictable-set, prevents the Learner eviction from targeting a phantom row that the count didn't include. Low harm either way (evicting a phantom is desirable), but set-consistency is cheap and removes an incoherence. Include in the same PR.

### 7.6 리스크 레지스터

- **R1.** TOCTOU residual is fully closed for same-user logins by the advisory lock, BUT cross-process correctness depends on every new-session creator taking the lock. Verified the only two creators are login() (623) and create_oauth_session() (2287). If a future path inserts a login row without enforce_admission_in_tx, it bypasses the cap. Mitigation: document the invariant in AMK_API_AUTH; the in-tx count is the gate, not the lock alone.
- **R2.** Behavioral flip (intended, user-visible): the previously-stuck Admin (false 403 from SCARD=8) will be ADMITTED post-deploy. Reject-role users who genuinely hold 2 valid sessions and concurrently log in now get a clean 403 instead of silently overshooting. Verify against the prod incident user before/after.
- **R3.** Lock contention: same-user concurrent logins serialize on pg_advisory_xact_lock(2, user_id) held to commit. Per-user only (no cross-user impact); each holder is a short 2-insert tx with geo lookup already pre-tx. Low risk at current scale; if abused (rapid same-user login spam) it self-throttles, which is desirable.
- **R4.** Advisory-lock keyspace: two-arg (2, user_id) fully partitions from ebook's single-arg 1701011563 and textbook's single-arg dynamic key — Postgres treats single-arg and two-arg locks as disjoint namespaces. RESOLVED by the two-arg fix; residual is zero.
- **R5.** Reaper lag: a session can be time-expired up to 300s before the reaper flips its DB row. Admission is unaffected (count predicate excludes it at read time); only dashboards/other queries see ≤300s staleness. Access tokens already die at the 15min ak:session TTL. Bounded, acceptable.
- **R6.** Reaper alone insufficient: it does NOT fix SCARD; it MUST land with Axis A. Hard dependency, flagged in rollout. If Axis A is reverted but D kept, the incident can recur.
- **R7.** Ban revocation under Redis-DEL failure: invalidate_all_sessions DELs Redis best-effort (.unwrap_or). If the DEL fails, the DB rows are still 'revoked' (refresh gate blocks re-mint) but a live ak:session can authorize the banned user for ≤15min. The extractor cross-check was deliberately NOT added (cost). Residual ≤15min, documented.
- **R8.** Bulk deactivation partial failure: if invalidate_all_sessions errors for one item, that user is already committed-deactivated in DB but sessions not yet revoked; we log+continue (matches existing bulk error semantics). Refresh gate + reaper/next-login clean up lazily. Do NOT roll back the bulk item.
- **R9.** find_login_refresh_hashes_by_session_ids accidental deletion would break logout_all (1242) and password_reset (1556). Explicitly KEEP — converted from open-decision to firm constraint.
- **R10.** Reuse-path 409→500 on Redis outage: only fires when Redis is down (auth already degraded); DB state durably compromised so retry → clean 401. No double-commit. Alerting that pages on auth 500 spikes will now page on a Redis blip on this path — desirable (surfaces the orphan-creation event), but note for ops.
- **R11.** Test-harness gap: no mock/toxiproxy/failpoint infra exists — the fail-injection 500 cases (E) and some failure-mode cases are not implementable today without new infrastructure (out of scope). The fail-closed logic is proven by the identical prod-clean logout_single template. Treat those tests as aspirational, not merge gates; the healthy-path and DB-state tests ARE implementable and are the gate.
- **R12.** Config without panic gate: SESSION_REAPER_INTERVAL_SEC has no range/panic guard by design (must default safely; <=0 disables). A typo like 'abc' panics at from_env (matches every sibling parse) — acceptable, consistent. A negative/zero value silently disables the reaper, which is boot-safe but could be an operator footgun; document the disable semantics in DEPLOY_OPS.
- **R13.** Multi-instance future: if scaled to >1 app instance, both run the reaper (idempotent, second reaps 0) and both honor the same advisory lock (Postgres-global). Safe. Single EC2 today; no action needed.

### 7.7 성공 기준 (회귀 테스트)

- **SC1.** INCIDENT REGRESSION (the literal trigger): seed an Admin (max=2) with DB active_db=0 (or any value < 2) AND SADD 8 stale members into ak:user_sessions; drive a login through create_oauth_session (mfa_login path). EXPECT admitted (Ok), because count_active_sessions_tx reads the DB not SCARD. Pre-fix this returned AUTH_403_SESSION_LIMIT:2.
- **SC2.** TOCTOU reject-role: spawn 5 concurrent login() for the same Admin (max=2) starting from 1 active row via JoinSet. EXPECT exactly 1 succeeds (fills slot 2), the other 4 return Forbidden AUTH_403_SESSION_LIMIT:2, and final count_active_sessions_tx == 2 — NEVER 3+ active DB rows.
- **SC3.** Phantom exclusion: insert 3 rows login_state=active with login_expire_at < now() (stale_active) for a max=2 Admin, then login(). EXPECT count_active_sessions_tx returns 0 and login SUCCEEDS (no false 403 from phantoms) — mirrors prod users 2/3/4.
- **SC4.** Admin at exactly max: 2 active+unexpired rows, Admin max=2, new login. EXPECT Forbidden AUTH_403_SESSION_LIMIT:2 (>= is reject for non-evict role).
- **SC5.** Learner FIFO unchanged: 5 active+unexpired rows (Learner max=5), 6th login. EXPECT oldest evicted (find_active_sessions_oldest), session_limit_evicted reason set, final active <= 5, new session admitted, no Internal('eviction aborted').
- **SC6.** Deactivated user refresh: admin sets user_state=false; a still-valid (non-rotated) refresh token for that user. EXPECT refresh returns AUTH_401_INVALID_REFRESH (not a new access token).
- **SC7.** Reuse-before-ban ordering: deactivated user submits a REUSED refresh token. EXPECT AUTH_409_REUSE_DETECTED (reuse branch runs before the user_state gate).
- **SC8.** Ban revocation completeness: admin_update_user sets user_state=false. EXPECT all login rows for that user → login_state='revoked' reason='account_deactivated', AND ak:user_sessions/ak:session/ak:refresh keys gone, AND a subsequent ensure_session_active → 401.
- **SC9.** Bulk no-op skip: admin_update_users_bulk item with user_state=Some(true) or unchanged Some(false) on an already-inactive user → invalidate_all_sessions NOT called; item with true→false transition → called once.
- **SC10.** mfa_disable parity after refactor: mfa_disable still revokes all DB sessions ('revoked') AND clears all Redis keys — identical to pre-refactor (positive cleanup test, since existing tests reject before cleanup).
- **SC11.** Reaper flips stale_active within interval: insert active row with login_expire_at = now()-1h; reap_expired_sessions returns 1, row becomes login_state='expired' reason='ttl_reaped'. Future/NULL expire_at rows untouched (return 0). Terminal-state rows (revoked/expired/logged_out/compromised) untouched.
- **SC12.** Reaper disabled: spawn(db, 0) and (db, -5) log 'disabled' and do NOT start a loop.
- **SC13.** Reuse fail-closed: with Redis up, reuse path returns 409 and deletes keys + SET member. (Failure-injection 500 cases are aspirational — no mock/toxiproxy harness exists; logic is proven by the identical prod-clean logout_single template.)
- **SC14.** Advisory lock auto-release on rollback: force insert_login_record_tx to error inside the tx; a subsequent same-user login does NOT hang (proves xact-scoped lock).
- **SC15.** No cross-user blocking: concurrent logins for two different users do not serialize.
- **SC16.** Keyspace isolation: confirm login lock uses pg_advisory_xact_lock(2, user_id) — a user_id == 1701011563 does NOT collide with ebook's single-arg lock.
- **SC17.** Build gates: cargo check + cargo clippy + cargo fmt --check clean; cd frontend && npm run build (no frontend change expected but per CLAUDE.md flow).


---

## 8. 구현 완료 (2026-05-30) — 빌드/회귀테스트 통과, 검토·배포 대기

§7 설계안의 6축을 모두 구현. **코드 변경은 단일 브랜치 KKRYOUN, 커밋/PR/배포는 사용자 승인 후.**

### 8.1 구현된 변경 (파일별)

| 파일 | 변경 |
|---|---|
| `Cargo.toml` | tokio `"time"` feature 추가 (reaper interval) |
| `src/api/auth/repo.rs` | `count_active_sessions` / `count_active_sessions_tx` (DB 권위 카운트), `acquire_user_session_lock_tx`(`pg_advisory_xact_lock(2,user_id)`), `reap_expired_sessions`; `find_active_sessions_oldest`에 `login_expire_at>now()` 정렬(D7) |
| `src/api/auth/service.rs` | **A+C**: `enforce_session_limit`에서 SCARD 게이트·reject-role 403 제거(DB 카운트로, Learner FIFO만 잔존) + 신규 `enforce_admission_in_tx`(lock+카운트+403)를 `login()`/`create_oauth_session()` tx 안에 wire. **E**: reuse-detection Redis 3 op `let _ =`→`?` fail-closed. **F**: `refresh()` user_state 게이트 + `invalidate_all_sessions` 헬퍼 추출 + `mfa_disable` 리팩터 |
| `src/api/admin/user/service.rs` | **F**: `admin_update_user`/`admin_update_users_bulk` 활성→비활성 전이 시 `invalidate_all_sessions("account_deactivated")` 호출 |
| `src/jobs/mod.rs`, `src/jobs/session_reaper.rs` | **D**: 신규 reaper task (interval, `<=0` 비활성) |
| `src/lib.rs`, `src/main.rs` | `pub mod jobs;` + serve 전 `session_reaper::spawn(db, interval)` |
| `src/config.rs` | `session_reaper_interval_sec` 필드 + from_env(기본 300) + Self + Debug |
| `docker-compose.prod.yml`, `docs/AMK_DEPLOY_OPS.md` | `SESSION_REAPER_INTERVAL_SEC` env (기본 300) |
| `tests/auth_session_limit_integration.rs` | 회귀 테스트 2건 (phantom 제외 카운트 · reaper) |

### 8.2 검증 (통과)
- `cargo check` ✅ / `cargo clippy --all-targets` ✅(0 warn) / `cargo fmt --check` ✅ / `cd frontend && npm run build` ✅
- 회귀 테스트 라이브 로컬 DB(amk-pg) 통과: `count_active_sessions_excludes_phantoms_and_nonactive`(축 A), `reap_expired_sessions_flips_only_stale_active`(축 D). 실행 = `EBOOK_IMAGE_ENCRYPTION_KEY=$(openssl rand -hex 32) cargo test --test auth_session_limit_integration -- --ignored --test-threads=1`.
- **마이그레이션 0** (login_expire_at·인덱스·enum 기존재 — `migrations/20260208_AMK_V1.sql` 실측).

### 8.3 구현 시 설계 대비 판단 (정직 기록)
- **B(ghost-cleanup) 최소 변경 채택**: 설계는 "in-line DB 'expired' UPDATE 제거"를 제안했으나, ghost-cleanup의 DB UPDATE는 진짜 유령(양 키 소멸)을 expired로 표기하는 정상 동작이라 **제거하지 않고 유지**(reaper와 멱등, 제거가 오히려 위험). B의 본질=카운트 오라클이 SCARD→DB로 바뀌는 것(A가 처리). Karpathy "시킨 것만/단순하게" 적용.
- **D4(extractor user_state 크로스체크) 미적용**(사용자 결정): refresh 게이트 + invalidate가 닫으므로 매 요청 DB 비용 회피. 잔여 = Redis DEL 실패 시 ≤15분 live ak:session 통과(R7).

### 8.4 잔여 (사용자 게이트)
- [ ] 사용자 코드 리뷰
- [ ] 커밋 → PR → main 머지(머지 후 `git reset --hard origin/main`+force-push, `feedback_git_branching`) → Deploy to EC2
- [ ] **prod 스모크**: 막혔던 Admin OAuth+MFA 로그인 → 이제 admit 확인 / reaper info-log 1주기 내 발화 확인 / 테스트 비활성화 → 세션 revoke 확인
- [ ] (별도 트랙) 프론트 다중탭 reuse 유발 토큰 갱신 로직 점검 (§4.5)

---

## 부록 A — 관련 메모리·문서

- 메모리 `project_mfa_session_limit_patch.md` (이 사건 + 패치 계획 포인터)
- `feedback_security_patterns.md` (fail-closed 원칙)
- `feedback_work_rules.md` (verify-before-assert, M-013 교훈)
- `AMK_AI_MISTAKES.md M-013` (스키마 객체명 SoT 교훈 — 동일 정신: 카운팅 SoT는 코드/DB 원본)
- `AMK_DEPLOY_OPS.md` (INC 패턴; 본 사건은 prod 영향 범위 1명/45분으로 INC 등재 여부는 다음 세션 판단)
