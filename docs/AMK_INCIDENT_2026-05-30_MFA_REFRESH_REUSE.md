# AMK 사건 — MFA 세션 limit 초과 (2026-05-30)

> **목적**: 다음 세션 패치 작업의 단일 SoT. 원인·현상·결과·패치 계획 분류.
> **상태**: 즉시 조치 완료(Redis 수동 정리), **영구 패치 미실행 → 다음 세션 진입점**.
> **재발 위험**: 🔴 패치 미실행 시 7일 이내 재발 보장 (TTL=7일).

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

## 부록 A — 관련 메모리·문서

- 메모리 `project_mfa_session_limit_patch.md` (이 사건 + 패치 계획 포인터)
- `feedback_security_patterns.md` (fail-closed 원칙)
- `feedback_work_rules.md` (verify-before-assert, M-013 교훈)
- `AMK_AI_MISTAKES.md M-013` (스키마 객체명 SoT 교훈 — 동일 정신: 카운팅 SoT는 코드/DB 원본)
- `AMK_DEPLOY_OPS.md` (INC 패턴; 본 사건은 prod 영향 범위 1명/45분으로 INC 등재 여부는 다음 세션 판단)
