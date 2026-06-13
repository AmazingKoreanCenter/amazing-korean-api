# AMK_AUDIT_2026-06-10 — 조합-결함 심층 감사 (인증·세션 / 결제 웹훅)

> **목적**: 단일 파일만 보면 정상이나 둘 이상 컴포넌트의 상호작용에서만 드러나는 **조합-결함(combination defect)** 을 인증·세션 축과 결제 웹훅 축에서 발견·기록. 인시던트급 잠복 결함 탐지가 목표.
> **조사 방법**: Workflow 멀티에이전트 (87 agent) — 경로 단위 finder 11개(흐름별 가설 생성) → 가설별 3-렌즈 적대적 검증자(제어흐름/동시성/재현가능성, **반증 우선**) → 5필드 게이트 종합. 25 가설 → 적대적 검증 16 생존 → 게이트·병합 후 **확정 10건**.
> **조사 일시**: 2026-06-10
> **기준**: HEAD commit `95a93cf` (KKRYOUN = origin/main)
> **원칙**: 메커니즘은 코드 file:line 직접 추적으로 확정. 영향 규모가 런타임 사실에 좌우되는 건 `런타임 확인 필요`로 명시(verify-before-assert). "테스트 부족"·스타일 류 일반론은 게이트에서 폐기.
> **상태**: **발견·기록 단계. 수정 미착수.** 수정 착수는 별도 결정 사항.

> **관련 SoT**: 인증/세션 = `docs/AMK_INCIDENT_2026-05-30_MFA_REFRESH_REUSE.md`, `docs/AMK_ADMIN_SESSION_MODEL_V2.md` / 결제 = `docs/AMK_API_PAYMENT.md` / 부채 게이트 = `docs/AMK_DEBTS.md` (5필드).

---

## 0. 요약

| 축 | 확정 | 심각도 분포 |
|----|:--:|----|
| 인증·세션 (A) | 4 | HIGH 2 · MEDIUM 2 |
| 결제 웹훅 (P) | 6 | HIGH 5 · MEDIUM 1 |
| **합계** | **10** | **HIGH 7 · MEDIUM 3** |

- **정적 확정(런타임 불요)**: A-1(ban 우회), A-4(MFA pending) — 2건
- **메커니즘 확정·영향규모 런타임 확인 필요**: 나머지 8건 (prod env var / Paddle·RevenueCat 실제 이벤트 행태 / IAP 라이브 여부에 노출 규모 좌우)
- **적대적 검증으로 반증 1건**(pending 재사용 익스플로잇 — TOTP는 POST 바디로만 전송, URL엔 UUID만 → 체인 미성립)
- **busywork 게이트로 폐기 1건**(orphan `ak:refresh` 키 — 메커니즘 real이나 인증 우회 아님·TTL 자연만료·세션 카운트 무오염 → 구체적 prod 장애 시나리오 부재)
- **양호 확인 9건** → §3

---

## 1. 인증·세션 축 (A)

### A-1. OAuth/MFA 세션 발급 경로에 `user_state` 게이트 부재 → 정지(ban) 계정 세션 재발급 — **HIGH** (정적 확정)

- **WHERE**: `src/api/auth/service.rs:2361` (`create_oauth_session`, user_state 검사 전무) / `:2026-2064` (`google_login_callback`: find_or_create_oauth_user→create_oauth_session, 게이트 없음) / `:2682-2790` (`mfa_login`: pending 읽고 user_state 재확인 없이 create_oauth_session 호출) / `:2842` (`invalidate_all_sessions`: `acquire_user_session_lock_tx` 미호출) / `src/api/auth/session.rs:46-57` (`ensure_session_active`: ak:session 존재만 확인) — **대조**: `:577`(비번 login)·`:935`(refresh)는 user_state 게이트 보유.
- **WHAT**: 비번 로그인(577)·refresh(935)는 `user_state=false`면 거부하지만, OAuth 콜백·mfa_login이 공유하는 `create_oauth_session` 경로에는 게이트가 전혀 없다. ban을 수행하는 `invalidate_all_sessions`는 login admission이 잡는 advisory lock(namespace 2)을 잡지 않아 동시 로그인과 직렬화되지 않는다. `ensure_session_active`도 ak:session 키 존재만 보고 user_state를 재검하지 않는다.
- **FAILURE SCENARIO**: (결정론적, 동시성 불필요) HYMN이 user X 정지(user_state=false 커밋 + invalidate_all_sessions revoke) → 정지된 X가 Google OAuth 재인증 → create_oauth_session이 user_state 검사 없이 새 login 행+ak:session(TTL 15분)+access token 발급 → 정지 계정이 정상 세션 획득. (레이스 변형: 정지와 OAuth/mfa_login 동시 진행 시 새 세션 INSERT가 revoke 스냅샷 이후 끼어 무효화 회피.)
- **PROD IMPACT**: 정지/탈퇴 계정(특히 운영 접근권 HYMN/Admin/Manager — MFA 활성 역할)이 정지 후에도 OAuth 재인증으로 새 정상 세션 획득(빈도 무제한) 또는 레이스로 생존(≤access-TTL, 기본 15분). 관리자 의도 즉시 차단이 OAuth/MFA 경로에서 무력화.
- **FIX DIRECTION**: create_oauth_session 진입 직전(find_or_create_oauth_user 반환 직후 / mfa_login의 호출 전)에 비번·refresh와 동일한 user_state 게이트 추가(fail-closed 통일). invalidate_all_sessions가 동일 namespace advisory lock을 같은 tx에서 잡아 admission과 직렬화하면 레이스 윈도 제거. 발급된 access token 잔존(≤access-TTL)은 ban 시 ak:session 전 키 강제 만료 또는 짧은 access TTL로 보완.
- **런타임 확인 필요**: 아니오 (정적 확정).

### A-2. refresh 회전이 `ak:session` TTL 미갱신 → access-TTL(기본 15분) 후 활성 세션 거짓 401 좀비 — **HIGH**

- **WHERE**: `src/api/auth/service.rs:769-776` (login: ak:session set_ex TTL=jwt_access_ttl_min*60) / `:2462-2470` (oauth 동일) / `:987-1000` (refresh Step6: **ak:refresh만 재set, ak:session 미재set** — set_ex는 login/oauth 2곳뿐) / `src/api/auth/session.rs:46-57` (ensure_session_active: ak:session 부재=결정적 401) / `src/api/auth/extractor.rs:47`·`src/api/admin/role_guard.rs:61` (모든 보호요청 게이트 강제) / `frontend/src/api/client.ts:80-102` (재시도 401은 _retry=true라 추가 refresh 차단, bare return으로 catch 미발화).
- **WHAT**: `ak:session:{sid}`는 login/oauth에서만 access TTL로 set되고 session_id는 회전 후에도 불변인데, refresh 회전(Step6)은 ak:refresh만 재set하고 ak:session은 절대 재set하지 않는다 → ak:session 수명이 '최초 로그인+access TTL'에 고정. ensure_session_active는 ak:session 부재를 확정 폐기(401)로 처리. 각 컴포넌트는 정상이나 결합 시 'refresh 토큰은 살아있는데 ak:session만 죽은' 거짓 401.
- **FAILURE SCENARIO**: T0 로그인(ak:session TTL 15분) → T0+14분 정상 refresh(새 JWT exp=T0+29분, 새 ak:refresh) but ak:session은 T0+15분 만료 예정 → T0+15분 ak:session 자연 만료 → 유효 JWT로 요청 시 ensure_session_active EXISTS=false → 401 'Session revoked' → 프론트 1회 refresh 재시도(성공) but ak:session 여전히 미재생성 → 또 401 → _retry=true로 추가 refresh 차단·bare return으로 catch 미발화 → logout/리다이렉트 없이 401 전파. DB 세션 idle 만료/수동 재로그인 전까지 모든 요청 실패.
- **PROD IMPACT**: 활성 사용자가 로그인 ~access-TTL(기본 15분) 후부터 의도된 슬라이딩 세션(Learner 30일/Admin 1h) 내내 거짓 'Session revoked' 401을 받는 좀비 세션. refresh는 계속 성공해 '로그인 상태'로 보이지만 데이터/액션은 전부 실패하는 반쯤 깨진 화면. 관리자 v2 1h sliding 핵심 목표 정면 무력화.
- **⚠️ 교차참조**: finder가 이 결함을 **리포 인시던트 "G4 (API-dead 세션)" 로 이미 명명·관측됨**으로 교차참조. 사실이면 강한 방증이자 이미 트래킹 중일 수 있음 → **incident 로그와 대조 확인 필요(미확정 단정 회피).**
- **FIX DIRECTION**: refresh Step6(`:987-1000`)에서 ak:session:{session_id}를 login/oauth와 동일하게 set_ex 재무장(회전 후 sid 동일=동일 키). 단 ak:session 의미(즉시강퇴 ≤access-TTL 윈도)와 슬라이딩 1h 충돌 고려해 TTL 정책 함께 결정. del 기반 강제퇴장(evict/logout)은 여전히 다음 요청 즉시 401이라 trade-off 없음.
- **런타임 확인 필요**: 예 (prod `JWT_ACCESS_TTL_MIN` 값에 노출 타이밍 좌우).

### A-3. 비번재설정/인증코드 발송 anti-enumeration 응답지연 미동일화 → 계정 존재/유형 타이밍 오라클 — **MEDIUM**

- **WHERE**: `src/api/auth/service.rs:1439·1451` (request_password_reset 조기 return: 미존재/OAuth전용은 set_ex·send 스킵) / `:1478-1496` (실계정만 set_ex+send_templated) / `:1145·1157` (find_password) / `:1849·1853` (resend_verification) / `src/external/email.rs:72-82` (reqwest `.send().await` 동기 HTTP, spawn 아님, timeout 15s) / `:530` (login은 dummy_password_hash로 보정 — 발송 3경로엔 동등 보정 없음).
- **WHAT**: 세 경로 모두 동일 성공 메시지를 반환하지만 분기 비용이 비대칭. 미존재/OAuth전용은 조기 return으로 Redis·이메일 스킵, 실 비번계정만 Resend 동기 HTTP 왕복(수십~수백 ms)을 임계 경로에서 거침. login의 dummy_password_hash 같은 타이밍 보정이 발송 3경로엔 없음.
- **FAILURE SCENARIO**: 공격자가 임의 이메일을 분산/회전 IP(rate-limit 키가 blind_index:IP라 우회 가능)로 반복 제출하며 latency 다회 측정 → 본문/상태코드 동일하나 실 비번계정만 일관되게 느림 → 평균/중앙값 비교로 '비번 기반 실계정'을 OAuth전용/미존재와 통계 분리.
- **PROD IMPACT**: 계정 열거 — 어떤 이메일이 가입·비번 기반인지가 응답 지연으로 노출 → 표적 피싱·크리덴셜 스터핑 선별 악용. "메시지 동일화로 방어됐다"는 거짓 안심. 통계적 사이드채널이라 jitter에 묻힐 수 있어 medium.
- **FIX DIRECTION**: 3경로 이메일 발송을 tokio::spawn fire-and-forget으로 분리(rate-limit 롤백 DECR도 spawn 내부로) → 응답을 분기 독립 즉시 반환. 또는 모든 분기에 발송 평균비용 상응 고정지연 부여(dummy_password_hash 패턴 확장).
- **런타임 확인 필요**: 예 (실제 지연 차이가 jitter 위로 측정 가능한지).

### A-4. MFA pending 1회용 소비 × 5회 rate-limit 설계 충돌 → 오타 1번에 전체 재로그인 강제 — **MEDIUM** (가용성/UX, 보안 아님, 정적 확정)

- **WHERE**: `src/api/auth/service.rs:2695-2705` (mfa_login: ak:mfa_pending GET 직후 코드검증 전 무조건 DEL) / `:2732-2740` (rl:mfa incr, max까지 허용) / `:2767` (MFA_INVALID_CODE — pending 이미 삭제) / `:2700-2702` (재시도 GET=None→MFA_TOKEN_EXPIRED) / `src/config.rs:64` (rate_limit_mfa_max 기본 5).
- **WHAT**: mfa_login은 pending을 GET 직후 코드검증 전 무조건 DEL(일회용). 같은 함수는 rl:mfa를 max(기본 5)까지 허용하는 '다회 재시도' 전제로 설계. pending이 1회 GET에서 삭제되므로 2번째 시도부터 GET=None→MFA_TOKEN_EXPIRED가 incr 전 early-return → 5회 예산은 attempts=2조차 도달 불가한 사문(dead) 코드.
- **FAILURE SCENARIO**: MFA 활성 사용자 로그인 1단계 성공 → 단일 mfa_token 발급 → TOTP 1자리 오타 → GET 성공·DEL·incr=1·검증 실패·MFA_INVALID_CODE(pending 영구 소멸) → 올바른 코드 재전송 → GET=None → MFA_TOKEN_EXPIRED 무한 반복 → 유일 탈출 = 비번(+OAuth)부터 전체 재로그인.
- **PROD IMPACT**: MFA 사용 관리자·OAuth+MFA 사용자가 코드 오타 1회마다 전체 로그인 재수행(OAuth+MFA는 'Back to login'조차 없어 OAuth 전체 재시작). 운영자에게 RATE_LIMIT_MFA_MAX=5는 '5회 허용'으로 보이나 실제 챌린지당 1회만 동작하는 설정-실동작 불일치. 보안 침해 아님(1회용이 brute-force엔 더 엄격).
- **FIX DIRECTION**: pending DEL을 코드검증 성공 이후로 이동(또는 검증 실패 시 동일 token으로 pending set_ex 재기록) → rate-limit 예산과 정합. 그러면 5회 cap·reset·429 분기가 의미를 가짐.
- **런타임 확인 필요**: 아니오 (정적 확정).

---

## 2. 결제 웹훅 축 (P)

### P-1. 웹훅 멱등성 read-then-write 갭 + `provider_transaction_id` UNIQUE 부재 → 동시 중복배달 시 transaction 이중 INSERT — **HIGH**

- **WHERE**: `src/api/payment/service.rs:154` (is_webhook_event_processed read-only SELECT, lock 없음) / `:165-203` (side-effect 블록) / `:208` (record_webhook_event 맨 마지막 INSERT ON CONFLICT DO NOTHING) / `src/api/payment/repo.rs:247-277` (create_transaction: ON CONFLICT 없는 plain INSERT) / `migrations/20260215_payment_system.sql:98` (idx_transactions_provider_txn_id = **비-UNIQUE** 인덱스) / `repo.rs:280-302` (update_transaction_status_by_provider_id: LIMIT 없는 UPDATE).
- **WHAT**: process_webhook_event = (1)존재여부 SELECT → (2)side-effect → (3)맨 마지막 멱등키 INSERT. 셋을 감싸는 DB 트랜잭션도 advisory lock도 없음(payment 모듈 begin/pg_advisory grep=0). 동시 연결 허용이므로 같은 event_id 거의 동시 2회 배달 시 두 핸들러 모두 (1)=false → 둘 다 side-effect 실행. ON CONFLICT DO NOTHING은 (3) 멱등 원장만 1건으로 막을 뿐 이미 실행된 side-effect는 못 되돌림. 결정타: create_transaction이 plain INSERT이고 provider_transaction_id에 UNIQUE 없어 DB가 중복 못 막음.
- **FAILURE SCENARIO**: Paddle이 transaction.completed(evt_X, txn_Y)를 겹치는 창에 2회 배달 → 핸들러 A·B 각각 별도 task+풀 커넥션 → 둘 다 서명 통과 → 둘 다 is_processed=false → 둘 다 create_transaction plain INSERT → payment_transaction에 txn_Y 2건. 멱등 원장은 1건이지만 transaction 원장 2건. 이후 환불 시 LIMIT 없는 UPDATE가 두 행 모두 Refunded.
- **PROD IMPACT**: 동일 결제 1건이 중복 기록되어 매출/세금 리포트·정산 이중 계상(로컬 재무 원장 무결성 손상). 실제 돈 이동은 Paddle이 SoT라 이중 청구 없음, 구독 권한(UPSERT)은 멱등이라 무손상 — 손상은 재무 데이터 국한.
- **FIX DIRECTION**: payment_transaction(payment_provider, provider_transaction_id)에 UNIQUE 추가 + create_transaction을 ON CONFLICT DO NOTHING/RETURNING으로(webhook_events 패턴 통일). 또는 record_webhook_event를 진입부에 먼저 INSERT하는 claim-first 멱등으로 게이트를 side-effect 앞에. 또는 (provider,event_id) 해시로 pg_advisory_xact_lock + 단일 트랜잭션화. (a) UNIQUE가 최소 변경으로 피해 봉쇄.
- **런타임 확인 필요**: 예 (Paddle 동시 중복배달 실제 빈도).

### P-2. 구독 FSM에 occurred_at 단조성 가드·advisory lock 부재 → 순서뒤바뀜/동시인터리빙이 취소·일시정지 구독 좀비 부활 — **HIGH**

- **WHERE**: `src/api/payment/repo.rs:209-240` (update_subscription_status: occurred_at/sequence 비교 없이 무조건 SET status) / `service.rs:377-408` (handle_subscription_updated: Active/Trialing이면 update_course_expiry) / `:340-374` (activated: grant_all_courses 무조건 active=true) / `:446-469` (paused: revoke_all_courses) / `:411-443` (canceled: expire_at=period_end만, 즉시 revoke 안 함) / `src/api/lesson/repo.rs:114-135` (has_course_access: active=true AND expire_at만, status 무시) — payment 모듈 전체 advisory lock/tx 0건.
- **WHAT**: update_subscription_status는 단조성 가드·'Canceled/Paused면 active 전이 무시' 선조건 없이 무조건 status 덮어씀. 핸들러는 tx·lock 없이 raw pool 실행이라 같은 구독의 다른 이벤트 병렬/역순 처리 가능. Paddle은 도착 순서 미보장·재시도 시 과거 상태 이벤트가 나중 도착 가능. 멱등 테이블은 같은 event_id만 막을 뿐 다른 event_id의 순서역전·인터리빙 미직렬화. 접근 게이트는 status 안 보고 active 플래그만 신뢰.
- **FAILURE SCENARIO**: (A 순서역전) 취소 → canceled(status=Canceled). 직전 발생했던(occurred_at 더 과거) updated(status=Active)가 지연/재시도로 canceled 뒤 도착(event_id 달라 멱등 우회) → status를 Active로 되돌리고 expire_at을 미래로 재기록. (B 동시인터리빙) paused(revoke, active=false)와 activated grant(active=true)가 락 없이 병렬 → grant UPSERT가 revoke 뒤 커밋되면 active=true 복구. 두 케이스 모두 has_course_access true 반환.
- **PROD IMPACT**: 취소·일시정지 사용자가 유료 접근 유지(매출 누수) 또는 역인터리빙 시 정상 사용자 거짓 차단(CS). status 필드도 Canceled→Active 영구 오염 → 관리자 UI 오표시·탐지 지연. 자가치유 안 됨(다음 정상 이벤트 와야 정정).
- **FIX DIRECTION**: (1) update_subscription_status에 occurred_at 단조성 가드 또는 'Canceled/Paused→Active 자동복귀는 명시적 activated/resumed로만 허용'하는 전이 가드. (2) payment_webhook_event에 occurred_at 저장 + 구독 단위 워터마크 비교. (3) 구독별 pg_advisory_xact_lock으로 직렬화 + 핸들러당 단일 tx로 status·entitlement 묶기. (4) 선택: has_course_access가 구독 status 교차검증.
- **런타임 확인 필요**: 예 (Paddle 순서역전·동시배달 실제 행태).

### P-3. 구독 환불 webhook(handle_adjustment)이 수강권 미회수 → 환불받고도 콘텐츠 접근 유지 — **HIGH**

- **WHERE**: `src/api/payment/service.rs:648-692` (handle_adjustment: ebook 매칭 실패 시 status=Refunded만 찍고 종료, revoke_all_courses/update_course_expiry 경로 전무) / `repo.rs:336-356` (grant_all_courses: expire_at=period_end, None이면 NULL=무기한) / `src/api/lesson/repo.rs:114-135` (has_course_access: users_course.active=true AND expire_at만, payment_transaction.status 무시) / `src/api/lesson/service.rs:182-192` (Paid 게이트→has_course_access).
- **WHAT**: 환불 처리와 entitlement 회수가 분리돼 서로 호출 안 함. handle_adjustment는 구독 transaction을 Refunded로만 표시하고 끝나며 수강권 회수 경로 전무. 레슨 게이트는 payment_transaction.status 안 보고 users_course active+expire_at만 봄. grant 시 expire_at=period_end(미래)라 환불(기간 중) 후에도 active=true·미래 expire_at 잔존.
- **FAILURE SCENARIO**: 월구독 결제 → activated가 grant(active=true, expire_at=미래). Paddle 환불(refund-only, 취소 미동반) → adjustment.created(Refund, Approved) → handle_adjustment가 ebook 매칭 실패 후 status=Refunded만 기록, users_course 무변경 → has_course_access 계속 true → 환불받은 사용자가 expire_at(구독 종료일)까지 유료 시청. grant 시 period_end=None이었으면 expire_at=NULL=영구.
- **PROD IMPACT**: 환불·차지백 사용자가 환불금 받고도 접근 유지 = 매출 손실 + 차지백 악용(결제→소비→차지백). 운영자가 수동 revoke 안 하면 좀비 권한 유지. refund-only는 어떤 자동회수도 없음.
- **FIX DIRECTION**: handle_adjustment 구독 환불 분기에서 환불 transaction의 subscription_id로 user_id 역추적 → revoke_all_courses(user_id) 또는 update_course_expiry(user_id, now)를 status 업데이트와 동일 tx로 호출. 또는 has_course_access SQL이 최신 status!=Refunded 교차검증. 부분환불 회수 여부는 정책 결정.
- **런타임 확인 필요**: 예 (Paddle refund-only 이벤트 발송 여부).

### P-4. RevenueCat CANCELLATION(환불) 웹훅 stub → IAP e-book 환불 후 영구 열람 (Paddle과 비대칭) — **HIGH**

- **WHERE**: `src/api/payment/service.rs:245-248` (process_revenuecat_webhook CANCELLATION arm: tracing::info! 로그만, '향후 구현' 비어있음) / `src/api/ebook/service.rs:222` (create_iap_purchase: 영수증 검증 후 status='completed' 영구) / `src/api/ebook/repo.rs:151-160` (insert_iap_purchase: status='completed' 하드코딩, paddle_txn_id=NULL) / `src/api/ebook/service.rs:388/630/743` (뷰어/타일/페이지 게이트: status==Completed만, 만료 없음) — **대조**: `payment/service.rs:668`+`ebook/repo.rs:299` (Paddle adjustment→refund_by_paddle_txn으로 회수).
- **WHAT**: IAP e-book은 영수증 검증 후 status='completed'(만료없음) 영구 부여, 게이트는 status==Completed만 확인. RevenueCat CANCELLATION(환불) arm은 로그만 남기고 비어 있어 회수 미호출. IAP는 paddle_txn_id=NULL이라 Paddle 환불 이벤트가 우연히도 IAP 행 회수 불가. RevenueCat→ebook_purchase 회수 경로 0건.
- **FAILURE SCENARIO**: 모바일 IAP 구매 → 영수증 통과 → status='completed' 영구. 스토어 환불 승인 → RevenueCat CANCELLATION 웹훅 → arm이 로그만 남기고 200 반환, ebook_purchase 무변경 → status 여전히 'completed' → 게이트 계속 통과 → 환불받은 사용자가 e-book 영구 열람·다운로드. 동일 자산을 Paddle 결제·환불했다면 차단됨 — 채널 비대칭.
- **PROD IMPACT**: 환불 후 IAP e-book 미회수 → 매출 누수/무단 접근. Paddle은 회수·IAP는 미회수 비대칭 → 정책/회계 불일치. 노출 규모는 (a) RevenueCat이 환불 시 CANCELLATION/REFUND를 실제 발송하는지, (b) IAP e-book 판매 라이브 여부(모바일 앱 출시)에 좌우 — 코드 결함 자체는 확정.
- **FIX DIRECTION**: CANCELLATION(및 환불 관련) arm에서 RevenueCat 페이로드 transaction id로 ebook_purchase를 'completed'→'refunded' 전이하는 repo 함수(refund_by_iap_txn) 추가. `AMK_API_PAYMENT.md:292`가 이미 이 동작을 의도로 명시.
- **런타임 확인 필요**: 예 (RevenueCat 환불 이벤트 발송 + IAP 라이브 여부).

### P-5. 부분 환불을 전액 환불로 처리 → 일부만 환불에 전체 접근 박탈 + 회계 왜곡 (PartiallyRefunded enum 사문화) — **HIGH**

- **WHERE**: `src/api/payment/service.rs:655·659` (handle_adjustment: action==Refund && status==Approved만, 금액/type 미판독) / `:668` (e-book: refund_by_paddle_txn 전체 회수) / `:679` (구독: transaction 전체 Refunded) / `src/api/ebook/repo.rs:299` (status='refunded' 전체 강등) / `src/types.rs:495` (PartiallyRefunded enum 존재하나 미세팅) / `src/api/admin/payment/service.rs:300` (partially_refunded 필터 존재하나 항상 0건).
- **WHAT**: handle_adjustment는 action==Refund && status==Approved만 검사하고 환불 금액(amount/payout_totals)·type(Partial/Full)을 전혀 안 읽음. Paddle 부분 환불도 Refund로 도착하는데 코드는 무조건 transaction 전체 Refunded·e-book 전체 'refunded' 강등. PartiallyRefunded enum·admin 필터는 존재하나 핸들러가 절대 세팅 안 해 영구 dead.
- **FAILURE SCENARIO**: 결제 후 운영자가 Paddle에서 50% 부분 환불 승인 → adjustment(Refund, type=Partial, Approved, totals=부분) → handle_adjustment가 type/totals 미확인 통과 → e-book이면 status='refunded' 전체 회수(이후 모든 page image Forbidden), 구독이면 transaction 전체 Refunded. 부분만 환불됐는데 사용자는 접근 전부 상실, 회계상 전액 환불 집계, PartiallyRefunded 영구 누락.
- **PROD IMPACT**: 부분 환불 고객 접근 부당 100% 박탈(CS 분쟁) + 회계 왜곡(부분→전액 과대계상, partially_refunded 필터 항상 0). 승인된 모든 부분환불에서 100% 재현, 트리거 빈도는 운영자가 부분환불 발행하는지에 의존.
- **FIX DIRECTION**: handle_adjustment에서 adj.r#type(Partial/Full) 또는 totals/payout_totals를 원거래 금액과 비교해 Full↔Partial 구분, Partial일 때 TransactionStatus::PartiallyRefunded(및 e-book 대응 상태) 세팅. e-book 부분환불 시 접근 유지/박탈은 비즈니스 정책 결정(자동 단정 금지).
- **런타임 확인 필요**: 예 (운영자 부분환불 실제 발행 여부).

### P-6. admin 결제목록 행단위 hard decrypt → 복호화 불가 행 1건이 페이지 전체 500 (COUNT 통과로 meta는 정상) — **MEDIUM**

- **WHERE**: `src/api/admin/payment/service.rs:127` (list_subscriptions: 행별 crypto.decrypt(...)?, soft fallback 없음) / `:329` (list_transactions) / `:434` (list_grants) / `:182·187` (get_subscription 상세) / `src/api/admin/user/service.rs:168` (user 목록 동일 패턴) — **대조**: `src/api/admin/ebook/service.rs:131-134` (if let Ok 소프트 폴백 의도적).
- **WHAT**: repo는 COUNT와 SELECT 별도 실행, COUNT는 user_email 미접근이라 항상 성공해 total_count/total_pages 메타 정상. service는 SELECT 각 행 user_email을 soft fallback 없이 hard `crypto.decrypt(...)?`. 한 행이 복호화 불가(키버전 누락/잘못된 AAD/손상 base64)면 그 행이 CryptoError→AppError::Internal→500 → 페이지 전체 500. ebook service는 soft 폴백 쓰면서 사용자 노출 admin 목록엔 hard ? 택한 일관성 결함.
- **FAILURE SCENARIO**: partial rekey 후 운영자가 --verify 통과 전 구버전 ENCRYPTION_KEY 조기 제거(문서화된 rekey→verify→키제거 순서 위반)해 미-rekey 행이 누락 키버전이 되거나 DB 손상으로 enc:v 깨진 행 발생 → 그 user가 payment_subscription 행 보유 → /admin/payment/subscriptions 열 때 COUNT 성공(meta 'N건') → SELECT가 나쁜 행 포함 → `:127` 루프에서 decrypt 실패 → 500. 정렬/필터/페이지로 나쁜 행을 화면 밖으로 밀면 다시 200 → 진단 오도하는 페이지·필터 특이적 거짓 500.
- **PROD IMPACT**: 단일 이상치가 결제 admin 목록 특정 페이지/필터를 사용불가화 → 그 행 구간에서 환불·취소 작업 차단. 메타상 행 존재 표시되는데 페이지만 500이라 진단 오도. 정상 운영(정확 AAD, 양쪽 키 보존 partial rekey)으론 미발생 — 운영 순서 위반/외부 손상 요구하는 잠재 취약성이라 medium.
- **FIX DIRECTION**: admin payment/user 목록 행별 복호화를 try_decrypt_or_plaintext(또는 if let Ok 폴백 후 '<decryption_failed>' placeholder)로 전환 → 단일 손상 행 격리(ebook service에 이미 존재하는 soft idiom 재사용).
- **런타임 확인 필요**: 예 (복호화 불가 행 실제 존재 여부).

---

## 3. 양호 확인 (clean — 검증 결과 정상)

1. **비번 로그인(service.rs:577) user_state 게이트** — 정지 계정 차단 정상(OAuth/MFA 경로만 갭).
2. **refresh(service.rs:935) user_state 재판독** — 정지 계정 토큰 재발급 401 거부, ban 우회를 access-TTL 1윈도로 상한하는 안전장치 정상.
3. **ensure_session_active Redis 장애 fail-open(session.rs:31-43,58-68)** — 감사 2.1 명시 수용 베이스라인(≤access-TTL 잔존), 신규 악화 없음.
4. **evict/logout ak:session del 즉시 강퇴(service.rs:908,1264,1687,2863)** — A-2 결함과 무관하게 폐기 즉시성 정상(다음 요청 즉시 401).
5. **멱등 원장 payment_webhook_event UNIQUE + ON CONFLICT DO NOTHING(repo.rs:411, migration:83)** — 순차 재전송 멱등 정상(동시 재전송만 갭=P-1).
6. **grant_all_courses ON CONFLICT DO UPDATE UPSERT(repo.rs:336-356)** — 동일 이벤트 중복 실행 시 권한 무손상(중복 손상은 transaction 행 국한).
7. **payment_subscription UNIQUE(provider, provider_subscription_id)(migration:93)** — 동시 중복 구독 생성 DB 차단.
8. **subscription.paused revoke_all_courses 즉시 비활성화(service.rs:464)** — 단독 일시정지 경로 정상(동시 인터리빙·환불 미동반에서만 문제=P-2/P-3).

---

## 4. busywork 게이트 폐기 (1건)

**orphan `ak:refresh` 키 (ban/logout × refresh 회전 race)** — 메커니즘은 real이나:
- 인증 우회 아님(후속 refresh가 DB state=revoked/user_state로 401, ak:session 부재로 액세스도 401)
- refresh TTL(≤30일)로 자연 만료, 세션 카운트(DB 권위) 무오염
→ "구체적 prod 장애 시나리오" 부재로 **5필드 게이트 미충족** → 위생 부채(OBSERVATIONS급)로 폐기. (실제 incident 시 또는 카운팅 발산 재관측 시 재평가.)

---

## 5. 다음 단계 (미결정)

- **수정 우선순위 후보**: A-1(정적 확정·동시성 불필요) → P-3/P-4/P-5(환불 미회수 클러스터, 같은 뿌리=환불·entitlement 분리) → P-1/P-2(동시성·순서 가드) → A-2(ak:session 재무장, 단 TTL 정책 동반) → A-3/A-4/P-6.
- **선행 확인 필요(런타임)**: prod `JWT_ACCESS_TTL_MIN`(A-2) / Paddle·RevenueCat 환불 이벤트 실제 발송 행태(P-3/P-4) / IAP e-book 라이브 여부(P-4) / A-2의 incident G4 교차참조 사실 확인.
- 수정 착수 여부·모델·세션은 별도 결정.
