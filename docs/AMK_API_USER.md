# AMK_API_USER — 사용자 & 관리자 API 스펙

> 공통 규칙(인증, 에러, 페이징): [AMK_API_MASTER.md §3](./AMK_API_MASTER.md)
> DB 스키마: [AMK_SCHEMA_PATCHED.md](./AMK_SCHEMA_PATCHED.md)
> 코드 패턴: [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md)

---

### 5.2 Phase 2 — user ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 2-1 | `POST /users` | `/signup` | 회원가입 | ***USERS, USERS_LOG 저장 + 세션/토큰 발급***<br>성공: Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→submitting→success / Request signup pending→success / Data signup present → **201**<br>실패(형식/누락): Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→error.client / Request signup pending→error / Data signup empty → **400**<br>실패(도메인 제약): Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→error.client / Request signup pending→error / Data signup error → **422**<br>실패(중복/충돌): Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→error.conflict / Request signup pending→error / Data signup error → **409**<br>실패(레이트리밋): Auth pass / Page signup ready / Form signup error.client / Request signup pending→error / Data signup error → **429** | [✅🆗] |
| 2-2 | `GET /users/me` | `/me` | 내 정보 조회 | ***USERS 안전 필드(비밀번호 제외)***<br>성공: Auth pass / Page me init→ready / Request me pending→success / Data me present → **200**<br>실패(미인증): Auth stop / Page me init→ready / Request me pending→error / Data me error → **401**<br>실패(미존재/비활성): Auth pass / Page me init→ready / Request me pending→error / Data me error → **404** | [✅🆗] |
| 2-3 | `POST /users/me` | `/me/edit` | 내 정보 수정 | ***USERS 일부 업데이트 → USERS_LOG 저장***<br>성공: Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→submitting→success / Request me_edit pending→success / Data me_edit present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→error.client / Request me_edit pending→error / Data me_edit empty → **400**<br>실패(도메인 제약): Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→error.client / Request me_edit pending→error / Data me_edit error → **422**<br>실패(미인증): Auth stop / Page me_edit init→ready / Request me_edit pending→error / Data me_edit error → **401**<br>실패(충돌·고유제약): Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→error.conflict / Request me_edit pending→error / Data me_edit error → **409** | [✅🆗] |
| 2-4 | `GET /users/me/settings` | `/settings` | 내 설정 조회 | ***USERS_SETTING 조회***<br>성공: Auth pass / Page settings init→ready / Request settings pending→success / Data settings present → **200**<br>실패(미인증): Auth stop / Page settings init→ready / Request settings pending→error / Data settings error → **401** | [✅🆗] |
| 2-5 | `POST /users/me/settings` | `/settings` | 내 설정 수정 | ***USERS_SETTING 수정 → USERS_LOG 저장***<br>성공: Auth pass / Page settings init→ready / Form settings pristine→dirty→validating→submitting→success / Request settings pending→success / Data settings present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page settings init→ready / Form settings pristine→dirty→validating→error.client / Request settings pending→error / Data settings empty → **400**<br>실패(도메인 제약): Auth pass / Page settings init→ready / Form settings pristine→dirty→validating→error.client / Request settings pending→error / Data settings error → **422**<br>실패(미인증): Auth stop / Page settings init→ready / Request settings pending→error / Data settings error → **401** | [✅🆗] |

---

<details>
  <summary>5.2 Phase 2 — user 시나리오</summary>

#### 공통 정책(1-1 ~ 1-5)
- **응답 에러 스키마(고정)**
  `{ "error": { "http_status": 400|401|404|409|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **로그 정책**: **성공/실패 모두 USERS_LOG 기록**(민감정보 제외, 실패 시 에러코드/사유 포함)
- **검증 기준**: **400**=형식/누락/파싱, **422**=도메인 제약 위반
- **중복 제출 방지**: Form=`submitting` 동안 UI 차단 + 서버 시간/조건 기반 방지
- **레이트리밋(우선 대상: 1-1)**: 과도 시 **429** + `Retry-After`
- **성공 후 페이지 전환**: 성공 시 다음 화면으로 이동하여 **Form 수명주기 종료**

#### 5.2-1 : `POST /users` (회원가입)
- **성공 → 201 Created**
  - When: `/signup` 폼 입력 후 제출이 서버 검증을 통과한다
  - Then: **201**
    - **Body**: `SignupRes { message, requires_verification }`
    - `requires_verification: true` → 이메일 인증코드 발송됨, 프론트엔드에서 `/verify-email` 페이지로 이동
    - `requires_verification: false` → 개발 환경(`EMAIL_PROVIDER=none`) 자동 인증, 즉시 로그인 가능
    - **자동 로그인 제거**: 회원가입 시 토큰/세션 발급 없음 (이메일 인증 후 로그인 필요)
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` pristine→dirty→validating→submitting→success** / Request=`signup` pending→success / Data=`signup` present
  - 로그: USERS insert 후 **USERS_LOG(성공 스냅샷)** 기록(민감정보 제외)
  - **미인증 재가입**: 동일 이메일로 `user_check_email=false`인 기존 레코드 존재 시 비밀번호/프로필 **덮어쓰기** + 새 인증코드 발송 (409 대신)
  - **인증코드 보안**: Redis에 HMAC-SHA256 해시 저장 (평문 저장 금지), blind index 키 사용
- **실패(형식/누락) → 400 Bad Request**
  - 예: 이메일 형식 불일치, 필수 항목 누락, JSON 파싱 오류
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.client** / Request=`signup` pending→error / **Data=`signup` empty**
  - 에러 바디: `{ "error": { "http_status": 400, "code": "BAD_REQUEST", "message": "...", "trace_id": "..." } }`
  - 로그: **USERS_LOG(실패 이벤트)** 기록(에러코드/사유, 민감값 마스킹)
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: birthday 범위 위반, 금지값, 정책 규칙 위반, 약한 비밀번호
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.client** / Request=`signup` pending→error / **Data=`signup` error**
  - 에러 바디: `http_status:422, code:"UNPROCESSABLE_ENTITY"`
  - 로그: 실패 이벤트 기록
- **실패(중복/충돌) → 409 Conflict**
  - 예: 이메일 UNIQUE 충돌 (인증 완료된 기존 계정)
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.conflict** / Request=`signup` pending→error / **Data=`signup` error**
  - 에러 바디: `http_status:409, code:"CONFLICT"`
  - 로그: 실패 이벤트 기록
- **실패(레이트리밋) → 429 Too Many Requests**
  - 조건: 과도한 가입 시도
  - 상태축: Auth=pass / Page=`signup` ready / **Form=`signup` error.client** / Request=`signup` pending→error / **Data=`signup` error**
  - 헤더: `Retry-After: <seconds>`
  - 로그: 실패 이벤트 + 차단 지표
- **중복 제출 방지(정책)**
  - 프론트: **Form=submitting** 동안 버튼/Enter 비활성
  - 백엔드: 미인증 재가입 시 덮어쓰기 + 새 코드 발송, 인증 완료 계정은 409

---

#### 5.2-2 : `GET /users/me` (내 정보 조회)
- **성공 → 200 OK**
  - When: 인증된 사용자가 `/me` 화면에서 자기 정보를 조회한다
  - Then: **200**, 안전 필드만 반환(비밀번호·해시·토큰 제외)
  - 상태축: Auth=pass / Page=`me` init→ready / Request=`me` pending→success / **Data=`me` present**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료/서명 오류
  - Then: **401**, `WWW-Authenticate: Bearer ...`
  - 상태축: **Auth=stop** / Page=`me` init→ready / Request=`me` pending→error / **Data=`me` error**
- **실패(미존재/비활성) → 404 Not Found**
  - When: 토큰은 유효하나 사용자 계정이 비활성/삭제 처리되어 조회 불가
  - Then: **404**
  - 상태축: Auth=pass / Page=`me` init→ready / Request=`me` pending→error / **Data=`me` error**

---

#### 5.2-3 : `POST /users/me` (내 정보 수정)
- **성공 → 200 OK(또는 204)**
  - When: `/me/edit` 폼 입력 후 제출이 서버 검증을 통과한다
  - Then: **200**(변경 후 스냅샷 바디) **또는 204**, USERS 업데이트 후 **USERS_LOG(성공 스냅샷)** 기록
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` pristine→dirty→validating→submitting→success** / Request=`me_edit` pending→success / **Data=`me_edit` present**
- **실패(형식/누락) → 400 Bad Request**
  - 예: 이메일 포맷 오류, 필수 필드 누락, JSON 파싱 실패
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` … → error.client** / Request=`me_edit` pending→error / **Data=`me_edit` empty**
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: birthday 범위 위반, 허용되지 않은 locale 값 등
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` … → error.client** / Request=`me_edit` pending→error / **Data=`me_edit` error**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
  - 상태축: **Auth=stop** / Page=`me_edit` init→ready / Request=`me_edit` pending→error / **Data=`me_edit` error**
- **실패(충돌/고유제약) → 409 Conflict**
  - 예: 닉네임/이메일 고유 제약 충돌 정책을 허용하는 경우
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` … → error.conflict** / Request=`me_edit` pending→error / **Data=`me_edit` error**

---

#### 5.2-4 : `GET /users/me/settings` (내 설정 조회)
- **성공 → 200 OK**
  - When: 인증된 사용자가 `/settings`에서 설정을 조회한다
  - Then: **200**, USERS_SETTING 반환
  - 상태축: Auth=pass / Page=`settings` init→ready / Request=`settings` pending→success / **Data=`settings` present**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
  - Then: **401**
  - 상태축: **Auth=stop** / Page=`settings` init→ready / Request=`settings` pending→error / **Data=`settings` error**

---

#### 5.2-5 : `POST /users/me/settings` (내 설정 수정)
- **성공 → 200 OK(또는 204)**
  - When: `/settings` 폼 입력 후 제출이 서버 검증을 통과한다
  - Then: **200**(최신 설정 반환) **또는 204**, USERS_SETTING 수정 후 **USERS_LOG(성공 스냅샷)** 기록
  - 상태축: Auth=pass / Page=`settings` init→ready / **Form=`settings` pristine→dirty→validating→submitting→success** / Request=`settings` pending→success / **Data=`settings` present**
- **실패(형식/누락) → 400 Bad Request**
  - 예: 필수 설정 누락, JSON 파싱 실패
  - 상태축: Auth=pass / Page=`settings` init→ready / **Form=`settings` … → error.client** / Request=`settings` pending→error / **Data=`settings` empty**
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: 언어 코드 범위/우선순위 규칙 위반(선호 언어 배열 유효성)
  - 상태축: Auth=pass / Page=`settings` init→ready / **Form=`settings` … → error.client** / Request=`settings` pending→error / **Data=`settings` error**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
  - Then: **401**
  - 상태축: **Auth=stop** / Page=`settings` init→ready / Request=`settings` pending→error / **Data=`settings` error**
</details>

---

### 5.7 Phase 7 — admin ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 7-1 | `GET /admin/users` | `/admin/users?page=&size=&q=&sort=&order=` | 사용자 조회 | ***검색/정렬/페이지네이션, RBAC(admin)***<br>성공(데이터 있음/없음): → **200**<br>실패(미인증): **401** / RBAC: **403** / 형식: **400** / 도메인: **422** | [✅🆗] |
| 7-2 | `GET /admin/users/{id}/admin-logs` | `/admin/users/{user_id}?tab=admin-logs&page=&size=` | 관리자 사용자 변경 로그 조회 | ***페이지네이션, RBAC***<br>성공: → **200**<br>실패: **401/403/404/400/422** | [✅🆗] |
| 7-3 | `GET /admin/users/{id}/user-logs` | `/admin/users/{user_id}?tab=user-logs&page=&size=` | 사용자 자체 변경 로그 조회 | ***페이지네이션, RBAC***<br>성공: → **200**<br>실패: **401/403/404/400/422** | [✅🆗] |
| 7-4 | `POST /admin/users` | `/admin/users/new` | 사용자 단건 생성 | ***ADMIN_USERS_LOG 저장, RBAC***<br>성공: → **201**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-5 | `POST /admin/users/bulk` | `/admin/users/bulk` | 사용자 다중 생성 | ***부분 성공, ADMIN_USERS_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-6 | `PATCH /admin/users/{id}` | `/admin/users/{user_id}/edit` | 사용자 단건 수정 | ***ADMIN_USERS_LOG 저장, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-7 | `PATCH /admin/users/bulk` | `/admin/users/bulk` | 사용자 다중 수정 | ***부분 성공, ADMIN_USERS_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |

| 7-8 | `GET /admin/videos` | `/admin/videos?page=&size=&q=&sort=&order=` | 비디오 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-9 | `GET /admin/videos/{id}` | `/admin/videos/{video_id}` | 비디오 상세 조회 | ***RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-10 | `GET /admin/videos/vimeo/preview` | `/admin/videos/new` | Vimeo 메타데이터 미리보기 | ***Vimeo API 연동, RBAC***<br>query: `url`<br>성공: **200**<br>실패: **401/403/400** | [✅🆗] |
| 7-11 | `POST /admin/videos/vimeo/upload-ticket` | `/admin/videos/new` | Vimeo 업로드 티켓 생성 | ***Vimeo tus upload, RBAC***<br>성공: **200**<br>실패: **401/403/400** | [✅🆗] |
| 7-12 | `POST /admin/videos` | `/admin/videos/new` | 비디오 단건 생성 | ***ADMIN_VIDEO_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-13 | `POST /admin/videos/bulk` | `/admin/videos/bulk` | 비디오 다중 생성 | ***부분 성공, ADMIN_VIDEO_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-14 | `PATCH /admin/videos/{id}` | `/admin/videos/{video_id}/edit` | 비디오 단건 수정 | ***ADMIN_VIDEO_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-15 | `PATCH /admin/videos/bulk` | `/admin/videos/bulk` | 비디오 다중 수정 | ***부분 성공, ADMIN_VIDEO_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-16 | `PATCH /admin/videos/{id}/tags` | `/admin/videos/{video_id}/tags` | 비디오 태그 단건 수정 | ***태그 검증, ADMIN_VIDEO_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-17 | `PATCH /admin/videos/bulk/tags` | `/admin/videos/bulk/tags` | 비디오 태그 다중 수정 | ***부분 성공, ADMIN_VIDEO_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-18 | `GET /admin/videos/stats/summary` | `/admin/videos/stats?from=&to=` | 비디오 통계 요약 | ***총 조회수/완료수/활성비디오수, 기간 검증(max 366일), RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-19 | `GET /admin/videos/stats/top` | `/admin/videos/stats?from=&to=&limit=&sort_by=` | TOP 비디오 조회 | ***조회수/완료수 정렬, limit 1-50, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-20 | `GET /admin/videos/stats/daily` | `/admin/videos/stats?from=&to=` | 비디오 일별 통계 | ***전체 집계, 제로필, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-21 | `GET /admin/videos/{id}/stats/daily` | `/admin/videos/{video_id}/stats?from=&to=` | 비디오별 일별 통계 | ***VIDEO_STAT_DAILY 조회, 제로필, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422** | [✅🆗] |

| 7-22 | `GET /admin/studies` | `/admin/studies?page=&size=&q=&sort=&order=` | 학습 문제 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-23 | `GET /admin/studies/{id}` | `/admin/studies/{study_id}` | 학습 문제 상세 조회 | ***tasks 포함, RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-24 | `POST /admin/studies` | `/admin/studies/new` | 학습 문제 단건 생성 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-25 | `POST /admin/studies/bulk` | `/admin/studies/bulk` | 학습 문제 다중 생성 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-26 | `PATCH /admin/studies/{id}` | `/admin/studies/{study_id}/edit` | 학습 문제 단건 수정 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-27 | `PATCH /admin/studies/bulk` | `/admin/studies/bulk` | 학습 문제 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-28 | `GET /admin/studies/tasks` | `/admin/studies/tasks?study_id=&page=&size=` | 학습 Task 조회 | ***study_id 필수, 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422/404** | [✅🆗] |
| 7-29 | `GET /admin/studies/tasks/{id}` | `/admin/studies/tasks/{task_id}` | 학습 Task 상세 조회 | ***RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-30 | `POST /admin/studies/tasks` | `/admin/studies/tasks/new` | 학습 Task 단건 생성 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/404/409** | [✅🆗] |
| 7-31 | `POST /admin/studies/tasks/bulk` | `/admin/studies/tasks/bulk` | 학습 Task 다중 생성 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/404/409** | [✅🆗] |
| 7-32 | `PATCH /admin/studies/tasks/{id}` | `/admin/studies/tasks/{task_id}/edit` | 학습 Task 단건 수정 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-33 | `PATCH /admin/studies/tasks/bulk` | `/admin/studies/tasks/bulk` | 학습 Task 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-34 | `GET /admin/studies/tasks/explain` | `/admin/studies/tasks/explain?task_id=&page=&size=` | 학습 해설 조회 | ***task_id 검증, 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422/404** | [✅🆗] |
| 7-35 | `POST /admin/studies/tasks/{id}/explain` | `/admin/studies/tasks/{task_id}/explain/new` | 학습 해설 단건 생성 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/404/409** | [✅🆗] |
| 7-36 | `POST /admin/studies/tasks/bulk/explain` | `/admin/studies/tasks/bulk/explain` | 학습 해설 다중 생성 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/404/409** | [✅🆗] |
| 7-37 | `PATCH /admin/studies/tasks/{id}/explain` | `/admin/studies/tasks/{task_id}/explain/edit` | 학습 해설 단건 수정 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-38 | `PATCH /admin/studies/tasks/bulk/explain` | `/admin/studies/tasks/bulk/explain` | 학습 해설 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409/404** | [✅🆗] |
| 7-39 | `GET /admin/studies/tasks/status` | `/admin/studies/tasks/status?task_id=&page=&size=` | 학습 상태 조회 | ***task_id 검증, 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422/404** | [✅🆗] |
| 7-40 | `PATCH /admin/studies/tasks/{id}/status` | `/admin/studies/tasks/{task_id}/status/edit` | 학습 상태 단건 수정 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-41 | `PATCH /admin/studies/tasks/bulk/status` | `/admin/studies/tasks/bulk/status` | 학습 상태 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409/404** | [✅🆗] |
| 7-42 | `GET /admin/studies/stats/summary` | `/admin/studies/stats?from=&to=` | 학습 통계 요약 | ***총 학습수/Task수/시도수/해결수/해결률, Program별/State별 분포, 기간 검증(max 366일), RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-43 | `GET /admin/studies/stats/top` | `/admin/studies/stats?from=&to=&limit=&sort_by=` | TOP 학습 조회 | ***시도수/해결수/해결률 정렬, limit 1-50, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-44 | `GET /admin/studies/stats/daily` | `/admin/studies/stats?from=&to=` | 학습 일별 통계 | ***일별 시도수/해결수/활성사용자, 제로필, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |

| 7-45 | `GET /admin/lessons` | `/admin/lessons?page=&size=&q=&sort=&order=` | 수업 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-46 | `GET /admin/lessons/{id}` | `/admin/lessons/{lesson_id}` | 수업 상세 조회 | ***lesson_id로 단건 조회, RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-47 | `POST /admin/lessons` | `/admin/lessons/new` | 수업 단건 생성 | ***ADMIN_LESSON_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-48 | `POST /admin/lessons/bulk` | `/admin/lessons/bulk-create` | 수업 다중 생성 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-49 | `PATCH /admin/lessons/{id}` | `/admin/lessons/{lesson_id}` | 수업 단건 수정 | ***ADMIN_LESSON_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-50 | `PATCH /admin/lessons/bulk` | `/admin/lessons` | 수업 다중 수정 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-51 | `GET /admin/lessons/items` | `/admin/lessons/items?page=&size=&lesson_id=` | 수업 아이템 조회 | ***lesson_id 필터, 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-52 | `GET /admin/lessons/items/{id}` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 상세 조회 | ***lesson_id로 아이템 목록+상세 조회 (video/task 정보 포함), RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-53 | `POST /admin/lessons/{id}/items` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 생성 | ***insert_mode(error/shift), ADMIN_LESSON_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-54 | `POST /admin/lessons/bulk/items` | `/admin/lessons/bulk-create` | 수업 아이템 다중 생성 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-55 | `PATCH /admin/lessons/{id}/items/{seq}` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 단건 수정 | ***seq로 아이템 지정, 순서 규칙 검증, ADMIN_LESSON_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-56 | `PATCH /admin/lessons/bulk/items` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 다중 수정 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409/404** | [✅🆗] |
| 7-57 | `DELETE /admin/lessons/{id}/items/{seq}` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 단건 삭제 | ***seq로 아이템 지정, ADMIN_LESSON_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-58 | `DELETE /admin/lessons/bulk/items` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 다중 삭제 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/404** | [✅🆗] |
| 7-59 | `GET /admin/lessons/progress` | `/admin/lessons/progress?page=&size=&lesson_id=&user_id=` | 수업 진행 조회 | ***lesson_id/user_id 필터, 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-60 | `GET /admin/lessons/progress/{id}` | `/admin/lessons/{lesson_id}` (Progress 탭) | 수업 진행 상세 조회 | ***lesson_id로 사용자별 진행현황 목록 조회 (current_item 포함), RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-61 | `PATCH /admin/lessons/{id}/progress` | `/admin/lessons/{lesson_id}` (Progress 탭) | 수업 진행 단건 수정 | ***user_id 지정, percent/last_item_seq 수정, ADMIN_LESSON_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-62 | `PATCH /admin/lessons/bulk/progress` | `/admin/lessons/{lesson_id}` (Progress 탭) | 수업 진행 다중 수정 | ***부분 성공, 다중 사용자 진행 수정, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409/404** | [✅🆗] |

| 7-63 | `GET /admin/users/stats/summary` | `/admin/users/stats?from=&to=` | 사용자 요약 통계 | ***총 사용자수/신규/활성/비활성, 역할별 집계, 기간 검증(max 366일), RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-64 | `GET /admin/users/stats/signups` | `/admin/users/stats?from=&to=` | 일별 가입 통계 | ***일별 가입수, 역할별 집계, 제로필, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-65 | `GET /admin/logins/stats/summary` | `/admin/logins/stats?from=&to=` | 로그인 요약 통계 | ***총 로그인/성공/실패/고유사용자/활성세션, 기간 검증(max 366일), RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-66 | `GET /admin/logins/stats/daily` | `/admin/logins/stats?from=&to=` | 일별 로그인 통계 | ***일별 성공/실패/고유사용자, 제로필, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-67 | `GET /admin/logins/stats/devices` | `/admin/logins/stats?from=&to=` | 디바이스별 로그인 통계 | ***디바이스별 성공횟수/비율, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |

| 7-71 | `POST /admin/email/test` | (관리자 전용) | 테스트 이메일 발송 | ***이메일 설정 검증용, RBAC(HYMN/Admin)***<br>성공: **200**<br>실패: **401/403/500** | [✅] |

| 7-68 | `POST /admin/upgrade` | `/admin/upgrade` | 관리자 초대 | ***초대 코드 생성 + 이메일 발송, RBAC(HYMN→Admin/Manager, Admin→Manager), Redis TTL 10분***<br>성공: **200**<br>실패: **401/403/400/422/409**(이미 가입된 이메일) | [✅🆗] |
| 7-69 | `GET /admin/upgrade/verify` | `/admin/upgrade/join?code=xxx` | 초대 코드 검증 | ***Public, 코드 유효성 검증, 이메일/역할 정보 반환***<br>성공: **200**<br>실패: **400/401**(만료/무효 코드) | [✅🆗] |
| 7-70 | `POST /admin/upgrade/accept` | `/admin/upgrade/join?code=xxx` | 관리자 계정 생성 | ***Public(코드 필수), 관리자 계정 생성(OAuth 불가), 코드 삭제***<br>성공: **201**<br>실패: **400/401/409/422** | [✅🆗] |

---

<details>
  <summary>5.7 Phase 7 — admin 관리자 초대 시나리오 (7-68 ~ 7-70)</summary>

#### 관리자 초대 시스템 개요

> 관리자 계정은 **오직 초대를 통해서만** 생성 가능. 일반 회원가입 후 승격 불가.

**보안 정책**
- 관리자 계정: OAuth 로그인 비허용 (이메일/비밀번호만)
- 초대 코드: Redis 저장, TTL 10분, 일회용
- 기존 이메일로 초대 시: 거부 (이미 가입된 이메일)
- 권한별 초대 가능 범위:
  | 요청자 | 초대 가능 권한 |
  |--------|---------------|
  | HYMN | Admin, Manager |
  | Admin | Manager |
  | Manager | 불가 (403) |

---

#### 7-68: `POST /admin/upgrade` (관리자 초대)

**요청**
```json
{
  "email": "new-admin@example.com",
  "role": "admin"  // admin | manager
}
```

**응답 (성공 200)**
```json
{
  "message": "Invitation sent successfully",
  "expires_at": "2026-02-04T12:10:00Z"
}
```

**처리 흐름**
1. 요청자 권한 검증 (HYMN/Admin만)
2. 초대 가능 role 검증 (HYMN→Admin/Manager, Admin→Manager)
3. 이메일 중복 체크 (기존 가입자면 409)
4. 초대 코드 생성: `ak_upgrade_{uuid}`
5. Redis 저장: `ak:upgrade:{code}` → `{email, role, invited_by, created_at}`, TTL 10분
6. 이메일 발송 (Resend)
7. 초대 로그 기록

**실패 케이스**
- **401**: 미인증
- **403**: 권한 부족 (Manager가 초대 시도, Admin이 Admin 초대 시도)
- **409**: 이미 가입된 이메일
- **422**: 유효하지 않은 role

---

#### 7-69: `GET /admin/upgrade/verify` (초대 코드 검증)

**요청**: `GET /admin/upgrade/verify?code=ak_upgrade_xxx`

**응답 (성공 200)**
```json
{
  "email": "new-admin@example.com",
  "role": "admin",
  "invited_by": "hymn@amazingkorean.net",
  "expires_at": "2026-02-04T12:10:00Z"
}
```

**실패 케이스**
- **400**: 코드 파라미터 누락
- **401**: 만료/무효 코드

---

#### 7-70: `POST /admin/upgrade/accept` (관리자 계정 생성)

**요청**
```json
{
  "code": "ak_upgrade_xxx",
  "password": "SecureP@ss123",
  "name": "홍길동",
  "nickname": "admin_hong",
  "country": "KR",
  "birthday": "1990-01-01",
  "gender": "male",
  "language": "ko"
}
```

**응답 (성공 201)**
```json
{
  "user_id": 123,
  "email": "new-admin@example.com",
  "user_auth": "admin",
  "message": "Admin account created successfully"
}
```

**처리 흐름**
1. 코드 검증 (Redis 조회)
2. 비밀번호 해싱 (Argon2id)
3. 사용자 생성 (user_auth = 초대 시 지정된 role)
4. 초대 코드 삭제 (일회용)
5. 초대 수락 로그 기록
6. (선택) 자동 로그인 토큰 발급

**실패 케이스**
- **400**: 필수 필드 누락, 형식 오류
- **401**: 만료/무효 코드
- **409**: 코드 이미 사용됨
- **422**: 비밀번호 정책 위반, 닉네임 중복

</details>

---

<details>
  <summary>5.7 Phase 7 — admin 공통 정책 & 시나리오 템플릿</summary>

#### 공통 보안/권한
- 미인증: Auth=stop → **401**
- 권한 부족(RBAC): Auth=forbid → **403**
- 리소스 은닉 전략(선택): 민감 리소스는 **404**로 은닉 가능

#### 에러 스키마(고정)
`{ "error": { "http_status": 400|401|403|404|409|422|429|500, "code": "...", "message": "...", "details": {}, "trace_id": "..." } }`

#### 검증 기준
- **400**: 형식/누락/파싱 실패(예: page=abc, size<1, 잘못된 정렬문법)
- **422**: 도메인 제약 위반(허용되지 않은 sort 필드, size 상한 초과, 비즈 규칙 위반)
- **409**: 고유제약/상태충돌(중복 이메일/태그, 삭제된 리소스 수정 금지 등)
- **429**: 대량/연속 작업 차단(선택, Retry-After 포함)

#### 로깅(필수)
- 모든 Admin 엔드포인트: 성공/실패 모두 `admin_*_log` 기록(요청 요약, actor user_id, 대상/개수, 결과코드, trace_id). 민감값은 마스킹.

---

#### 목록/조회 공통 시나리오(예: GET /admin/videos)
- 성공(데이터 있음/없음) → **200**
  Auth pass / Page init→ready / Request pending→success / Data present|empty
- 실패(미인증/권한) → **401**/**403**
- 실패(형식/도메인) → **400**/**422**

---

#### 단건 생성 템플릿(예: POST /admin/videos)
- 성공 → **201**
  Page init→ready / Form pristine→dirty→validating→submitting→success / Request pending→success / Data present
  헤더: `Location: /admin/videos/{id}`
- 실패(형식/도메인/중복/권한) → **400**/**422**/**409**/**401**/**403**

---

#### 다중 생성(벌크) 템플릿
- 성공(전량) → **201**
- 성공(부분) → **207**
- 실패 항목은 배열로 에러 사유 제공(예: 400/422/409)

---

#### 단건 수정 템플릿(예: PATCH /admin/lessons/{id})
- 성공 → **200** 또는 **204**
- 실패 → **401**/**403**/**404**/**400**/**422**/**409**

---

#### 벌크 수정 템플릿
- 성공(전량) → **200** 또는 **204**
- 성공(부분) → **207**
- 실패 항목별 에러 사유 포함

---

#### 통계 조회(예: GET /admin/videos/{id}/stats)
- 성공 → **200** (빈 구간도 **200**)
- 실패 → **401**/**403**/**404**/**400**/**422**
  (기간(from≤to)·그라뉼러리티 검증 포함)

</details>

---

[⬆️ AMK_API_MASTER.md로 돌아가기](./AMK_API_MASTER.md)
