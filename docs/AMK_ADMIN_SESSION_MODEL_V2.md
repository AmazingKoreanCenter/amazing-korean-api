# AMK 관리자 세션 보안 모델 v2 — 설계·구현 핸드오프

> **목적**: 다음 세션이 추측 없이 바로 구현 착수하도록, **왜 / 무엇을 / 어떻게**를 확정 기록.
> **상태**: 설계 확정 + **구현 착수(2026-06-02)**. PR 분할 = **PR #1 프론트 single-flight refresh(분리 선배포·저위험·BE무관)** / **PR #2 BE 세션모델 + 프론트 단일탭 차단**. 둘 다 prod까지(머지=Deploy to EC2 자동).
> **선행(완료·배포됨)**: `docs/AMK_INCIDENT_2026-05-30_MFA_REFRESH_REUSE.md` (Phase 1, PR #317 `fa1ae2e` prod 안착·검증). 본 문서 = **Phase 2**.
> **모든 항목은 코드/이력 실측 기반. 추측은 "추론"으로 명시 표기.**

> ## 0. 구현 진행 상태 (2026-06-02 갱신)
> - **§4.1 refresh TTL 단위 = 확정**: 옵션 ②(초 반환 일반화) 채택. `refresh_ttl_days_for_role` → **`refresh_ttl_secs_for_role(role) -> i64`** 로 교체(Hymn/Admin/Manager = 신규 `REFRESH_TTL_SECS_ADMIN` 기본 3600 / Learner = `refresh_ttl_days * 86400`). 호출 3곳(`service.rs:628`·`:865`·`:2299`)의 `* 24 * 3600` 제거. dead 필드 `refresh_ttl_days_admin/_hymn` 제거. 근거: 하류 `repo.rs:439 make_interval(secs => $3)`가 이미 초 기반 → 변환점 소멸.
> - **§4.2 evict 범위 = 확정**: HYMN/Admin/Manager만 evict 추가. **reject 경로(`enforce_admission_in_tx`)는 유지(dormant)** — `is_session_evict_role` true면 자동 우회, 삭제 안 함(회귀 안전망). 단 evict 경로(`enforce_session_limit`)에 advisory lock 추가 필수.
> - **§4.3 ak:session 즉시강퇴 = ⚠️ 설계 §3.1 #5 STALE 정정 (구현 시 발견 2026-06-02)**: "현재 요청별 검증이 ak:session 을 안 봐서 15분 잔존"은 **이미 해소됨**. **보안 audit 2.1(`AMK_API_SECURITY_AUDIT.md:45` `[x] 완료 2026-05-17`)** 가 `session.rs::ensure_session_active` 를 `extractor.rs`(AuthUser/OptionalAuthUser) + `role_guard.rs:61` 에 전역 wiring → ak:session 부재 시 즉시 401. evict 가 `ak:session` 키를 삭제하므로 퇴장 기기 다음 요청 = 즉시 401 이 **이미 동작**. → **PR #2 무코드**(재구현/축소 안 함. "관리자 한정 축소"는 Learner extractor 검사 제거=보안 다운그레이드라 미채택).
> - **PR #1(완료·배포·prod 라이브)**: `frontend/src/api/client.ts` single-flight(설계 §3.2 ⑥). PR #318 머지(main `ea9bf59`)·Cloudflare Pages success.
> - **PR #2(구현 완료·검증 통과)**: 백엔드 ①②③④(#5는 위 무코드) + 프론트 단일탭 ⑦.
>   - **①②③ config**: `refresh_ttl_secs_for_role`(초)·`REFRESH_TTL_SECS_ADMIN=3600`·`MAX_SESSIONS_*=1`·`is_session_evict_role` 전 역할 true. dead 필드 제거.
>   - **④ 원자 evict (위험 최소화 범위)**: Learner 경로 **무변경**(pre-tx FIFO 유지). HYMN/Admin/Manager 만 `enforce_admission_in_tx` 에서 advisory lock 안에 count→`evict_oldest_sessions_tx`(in-tx UPDATE…RETURNING)→insert 원자화. 퇴장분은 commit 후 `cleanup_evicted_sessions_redis` 로 정리(즉시 강퇴).
>   - **⑦ 단일탭**: `useAdminSingleTab`(BroadcastChannel hello/present) + `AdminLayout` 차단 화면. AdminRoute(admin/HYMN/manager 전용)로만 진입 → 자동 스코프.
>   - **검증**: cargo check/clippy(-D warnings)/fmt + 통합테스트 3/3(신규 `evict_oldest_sessions_tx_revokes_only_oldest_n` 실DB 통과) + npm build/eslint/lint:ui.

---

## 1. 왜 이 작업을 하는가 (배경 — 전부 확정 사실)

### 1.1 Phase 1로 드러난 문제
Phase 1(2026-05-30, PR #317)에서 동시 세션 카운팅을 **Redis SCARD → DB 권위 카운트**로 바꿔 거짓 403을 없앴다. 그 결과 **카운팅이 정확해지면서 기존 "초과 시 거부(reject)" 정책의 결함이 일관되게 드러났다**:
- 관리자(사장님)가 Google OAuth + MFA로 **본인 인증을 완료했는데도**, 본인이 쿠키도 없어 **쓰지도 못하는 옛 세션 2개**가 새 로그인을 막음 (`AUTH_403_SESSION_LIMIT:2`).
- prod 실측: user_1이 active 세션 2개(login_id 81·82, 둘 다 desktop, 미만료) → 한도 2 도달 → 정당하지만 **본인이 락아웃**.

### 1.2 reject 정책의 출처 (git 추적 결과 — 확정)
- 도입 커밋: **`0d85b19` "Phase V1-2 : 동시 세션 수 제한 구현"**, **2026-04-08**, `Co-Authored-By: Claude Opus 4.6`. = **AI가 구현한 결정**(사용자 계정으로 커밋).
- `is_session_evict_role` / `max_sessions_*` / `AUTH_403_SESSION_LIMIT` / `enforce_session_limit` 전부 이 한 커밋에서 같이 도입(git pickaxe `-S` 확인).
- **사용자가 reject를 지시한 기록 없음** — 커밋 메시지·CHANGELOG([line 7005-7011](AMK_CHANGELOG.md))에 "Admin+ 거부(403)"가 **사실로만** 적혀 있고 요청 근거 없음.
- **reject vs evict를 선택한 근거 문서 0** — 커밋 메시지 / `AMK_API_MASTER.md §동시 세션 수 제한(978-1000)` / `AMK_DEPLOY_OPS.md:147-150` / 메모리 전부 **무엇(정책)만 적고 왜(reject 선택 이유)는 한 줄도 없음**(전수 확인).
- **(추론, 기록 아님)**: "고권한=더 엄격(거부)" 관행을 AI가 기본값으로 깐 것으로 보임. 단 MFA 필수 1인 CEO 환경엔 논리가 약함(인증 통과한 본인을 막음).
- **결론**: reject는 사용자가 정한 것도, 문서화된 보안 요구도 아닌 **미문서화 AI 기본값**. → evict로 바꾸는 건 설계 의도 위반이 아니라 근거 없던 기본값을 실상황에 맞게 정정하는 것.

### 1.3 부수 발견 — "몇 시간 뒤/재오픈 시 로그아웃"의 진짜 원인 (코드 근거 — 확정)
사용자 증상: "관리자로 로그인·업무 후 탭 닫고, 몇 시간 뒤 다시 들어가면 재로그인 필요."
- **원인 = 멀티탭이 아니라, 프론트 refresh가 single-flight가 아님.** `frontend/src/api/client.ts:42-89` 401 인터셉터에 **`isRefreshing` 플래그·대기 큐가 없음**.
- 메커니즘: 여러 API 요청이 동시에 401(access token 15분 만료 — 예: 재오픈 시 첫 화면이 호출 다발) → **각 요청이 *따로* `/auth/refresh` 동시 호출** → 서버는 첫 refresh에서 토큰 회전(`repo.rs update_login_refresh_hash_tx`) → 나머지가 옛 토큰 제시 → **reuse 감지(`service.rs:792~`) → 세션 compromised → 409 → 프론트 catch(`client.ts:80-83`) → 로그아웃**.
- **단일 탭에서도 발생**(동시 요청만으로 충분). 멀티탭은 *추가* 트리거.
- prod 증거: incident §1.4의 **compromised 24건(매일)** = 이 현상.
- 이전에 "멀티탭"으로 단정했던 설명은 **부정확 → 정정**(2026-06-02). 진짜 근본 수정 = `client.ts` single-flight화.

---

## 2. 검토한 고려사항과 선택 (각 대안 + 선택 + 이유 — 전부 사용자 합의 2026-06-02)

| # | 항목 | 검토한 대안 | **선택** | 이유 |
|---|---|---|---|---|
| 1 | 초과 시 행동 | reject(거부) vs **evict(기존 퇴출)** | **evict** | MFA 통과한 본인 절대 락아웃 안 됨. 침입(MFA 필요) 시 본인 세션 즉시 끊겨 **탬퍼 신호 가시화**(reject보다 탐지 우수). reject의 이점(2번째 세션 차단)은 이미 비번+MFA 탈취된 상황이라 미미. |
| 2 | 최대 세션 | 1 vs 2 | **1 (단일 기기)** | 사용자가 한 번에 한 기기만 사용. 고권한 최소 공격면. (2기기 동시 필요 시 2로 가능했으나 사용자가 1 확정.) |
| 3 | 세션 수명 | 7일(현행) vs 1시간 절대(은행식+카운트다운+연장버튼) vs **1시간 sliding(idle)** | **1시간 sliding** | 사용 중 자동연장(필요할 때 안 끊김) + 1시간 미사용 시 자동 퇴출(관리 용이). 절대만료/카운트다운/버튼 같은 별도 기계장치 불필요. |
| 4 | 멀티탭 처리 | 즉시 로그아웃 vs **새 탭 차단** vs 크로스탭 single-flight | **새 탭 차단** | 트리거 = "우리 사이트가 이미 열린 상태에서 우리 사이트 2번째 탭이 열릴 때". 그 새 탭만 차단/리다이렉트("이미 다른 탭에서 사용 중") → **기존 작업 탭은 안 끊김**, 보안효과 동일. 무작정 로그아웃 불필요. |
| 5 | 퇴출/로그아웃된 기기의 access token 15분 잔존 | 유지 vs **즉시 무효화** | **즉시 무효화** | 관리자 계정은 끊긴 즉시 강퇴돼야 함. 현재 요청별 검증이 `ak:session` 존재를 안 봐서 15분 잔존(`AMK_API_SECURITY_AUDIT.md:45` 기존 갭). |
| 6 | 동시 로그인 순간 2세션 가능 | 허용(self-heal) vs **정확히 1 보장** | **정확히 1** | evict 경로에도 advisory lock 걸어 동시 로그인 직렬화. |
| 7 | "재오픈 시 로그아웃" 근본 수정 | (프론트) refresh single-flight 도입 | **도입** | §1.3의 근본 원인. 이게 없으면 1시간 sliding(③)도 활동 중 랜덤 로그아웃 지속. ③과 **반드시 묶어서** 진행. |

**적용 범위**: HYMN / Admin / Manager (= "관리자 영역 모든 권한"). **Learner(학습자/고객)는 현행 유지**(max 5 / 30일 / evict).

---

## 3. 확정 설계 (최종 모델)

### 3.1 백엔드 (HYMN / Admin / Manager)
1. **최대 세션 1** — `max_sessions_hymn/admin/manager` = 1.
2. **evict로 전환** — `is_session_evict_role`이 현재 `matches!(Learner)`만 true. → HYMN/Admin/Manager도 evict 대상에 포함. (max=1 + evict = "기존 전부 퇴출 후 새 1개" = 단일 세션·last-login-wins. `enforce_session_limit`의 FIFO 로직 그대로 재사용: `evict_count = active_count - 1 + 1 = active_count` → 전부 퇴출.)
3. **refresh TTL 1시간(sliding)** — 사용/회전 시 `login_expire_at = now()+1h` 갱신(기존 `update_login_refresh_hash_tx` 동작이 이미 sliding) → idle 1시간이면 만료.
4. **동시 로그인 정확히 1** — Phase 1에서 만든 `acquire_user_session_lock_tx`(advisory lock)를 **evict 경로(`enforce_session_limit`)에도** 적용. (현재는 reject 경로 `enforce_admission_in_tx`에만 있음.)
5. **즉시 강퇴(요청별 `ak:session` 검증)** — ⚠️ **STALE(2026-06-02 구현 시 정정, §0 참조)**: 이미 audit 2.1(`[x] 완료 2026-05-17`)가 `ensure_session_active` 를 extractor + role_guard 에 전역 wiring 완료 → 본 항목은 **이미 구현됨, PR #2 무코드**. 아래 "현재 ~ 안 봐서 15분 잔존" 서술은 audit 2.1 이전 기준의 오래된 인용이었다. ~~extractor/`ensure_session_active`가 요청마다 `ak:session:{sid}` 존재 확인 → 퇴출/로그아웃 즉시 access token 무효.~~

### 3.2 프론트엔드
6. **🔑 single-flight refresh** (`src/api/client.ts`) — 동시 401이 와도 `/auth/refresh`는 **1번만** 실행, 나머지 요청은 그 결과를 기다렸다 재시도. (현재 큐/플래그 없음 = 근본 버그.) → "재오픈/몇시간 뒤 로그아웃" 근본 해결.
7. **단일 탭 강제(관리자)** — BroadcastChannel/localStorage로 "우리 사이트가 이미 다른 탭에서 열림" 감지 → **새로 연 탭을 차단/리다이렉트**("이미 다른 탭에서 사용 중입니다"). **기존 작업 탭은 유지**(로그아웃 아님).

---

## 4. 구현 시 정해야 할 사항 (다음 세션 — 현재 미결, 추측 금지)

1. **🔴 refresh TTL 단위 문제 (필수 선결)**: 현재 config는 **일(day) 단위** — `refresh_ttl_days_admin`(기본 7) × 24 × 3600. **1시간은 정수 일로 표현 불가.** → ① admin/hymn/manager용 TTL을 **분/초 단위 필드로 신설/변경**하거나 ② `refresh_ttl_for_role`을 초 반환으로 일반화. **구현 전 이 단위 설계를 먼저 확정할 것.** (Learner의 30일은 유지 필요 → 역할별 단위 혼재 처리 주의.)
2. **evict 적용 역할 범위**: HYMN/Admin/Manager 3개만 evict로 바꿀지, 혹은 `is_session_evict_role`을 전 역할 true로 할지(그러면 reject 경로 `enforce_admission_in_tx`가 완전 미사용 → 제거 검토). **단, evict 경로에 advisory lock(#4)이 들어가야 원자성 유지** — Phase 1에서 advisory lock을 reject 경로에만 넣은 점 주의.
3. **요청별 `ak:session` 검증 범위/비용**: 전 역할 vs 관리자 라우트 한정. Redis 1회/요청 비용 vs 즉시강퇴 필요성 저울질. (사용자 의도 = 관리자 즉시강퇴.)
4. **single-tab 차단 UX**: 차단 화면/문구, "여기서 이어서 작업" 같은 takeover 옵션 여부(YAGNI 가능).
5. **single-flight refresh 큐 구현**: 진행 중 refresh Promise 공유 + 실패 시 일괄 로그아웃 처리 방식.
6. **MFA 일회용 토큰 소비 순서(선택)**: Phase 1 워크플로가 지적한 "403이 토큰을 태워 MFA_TOKEN_EXPIRED로 보임" 문제. **evict 전환 후엔 관리자 403 자체가 사라져 자연 소멸** → 별도 핫픽스 불요(확인). 단 견고성 위해 "성공 시에만 ak:mfa_pending DELETE"로 옮기는 건 선택.

---

## 5. 작업 순서 제안 (다음 세션)
1. **#4.1 TTL 단위 설계 확정** (선결).
2. 백엔드: config(한도1·TTL1h·evict역할확장) → `is_session_evict_role`/`max_sessions`/`refresh_ttl` → enforce 경로 정리(evict에 advisory lock) → 요청별 ak:session 검증.
3. 프론트: `client.ts` single-flight → 관리자 단일탭 차단(BroadcastChannel).
4. 검증: cargo check/clippy/fmt + npm build + 회귀/통합 테스트(아래 §7) + prod 스모크.
5. 문서 동기화(`AMK_API_MASTER §동시세션` 표 갱신·이 문서 상태 갱신·CHANGELOG·STATUS) + 메모리 갱신 + PR(KKRYOUN→main, 머지 후 `git reset --hard origin/main`+force-push).

## 6. 변경 금지 / 주의 (확정)
- Learner 정책(max 5 / 30일 / evict) **유지**.
- `users.user_mfa_secret` AAD 문자열 변경 금지(prod 복호화 깨짐).
- MFA 토큰 TTL 5분 변경은 별건.
- Phase 1 산출물(DB 권위 카운트·reaper·ban 무효화·reuse fail-closed)은 **이미 prod 동작** — 건드리지 말고 위에 얹을 것.
- 머지 후 KKRYOUN 리셋 규칙(`feedback_git_branching`) 필수.

## 7. 검증 기준 (성공 정의)
- Admin이 2번째 기기/로그인 시 **기존 세션 퇴출 + 본인 admit**(403 없음). 동시 로그인 2건 → **최종 active 정확히 1**.
- 퇴출된 기기의 다음 요청 → **즉시 401**(15분 안 기다림).
- 관리자 1시간 미사용 → 세션 만료(재로그인). 사용 중엔 안 끊김.
- 관리자 우리 사이트 2번째 탭 → **차단**(기존 탭 유지).
- 탭 닫고 재오픈 / 동시 다발 요청 → **single-flight로 reuse 안 터짐 → 로그아웃 안 됨**(prod 로그 `reuse detected` 소거 확인).

---

## 부록 — 핵심 코드/이력 참조 (확정)
- reject 도입: 커밋 `0d85b19`(2026-04-08, Claude Opus 4.6). 근거 문서 0.
- 현재 정책 코드: `config.rs` max_sessions(288-305)·refresh_ttl_days(139-150)·`is_session_evict_role`(661-663, Learner만)·`refresh_ttl_days_for_role`(640-648, Admin|Manager→admin 변수=7일/Hymn→1일).
- enforce: `service.rs` `enforce_session_limit`(198-365, evict+ghost) / `enforce_admission_in_tx`(372-391, reject+advisory lock). 호출 login(`:620`,`:655`)/oauth(`:2321`,`:2335`).
- single-flight 버그: `frontend/src/api/client.ts:42-89`(큐/플래그 없음), logout 분기 `:80-83`.
- 요청별 검증 갭: `AMK_API_SECURITY_AUDIT.md:45`.
- access TTL 15분: `jwt_access_ttl_min=15`. ak:session TTL = 15분, ak:refresh/login_expire_at = 역할별 TTL.
