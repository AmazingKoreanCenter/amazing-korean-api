# AMK_API_LEARNING — 학습 도메인 API 스펙

> Health, 영상, 학습, 레슨, 코스, 번역 엔드포인트 통합.
> 공통 규칙(인증, 에러, 페이징): [AMK_API_MASTER.md §3](./AMK_API_MASTER.md)
> DB 스키마: [AMK_SCHEMA_PATCHED.md](./AMK_SCHEMA_PATCHED.md)
> 코드 패턴: [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md)

---

### 5.1 Phase 1 — health ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 1-1 | `GET /healthz`<br>`GET /health` | `/health` | 라이브 헬스 | ***서버 작동 여부 확인. `/health`는 `/healthz`의 별칭(동일 핸들러)***<br>**성공:** Auth pass / Page : healthz init→ready / Request : healthz pending→success / Data : healthz present → **200**<br>**실패:** Auth pass / Page : healthz init→ready / Request : healthz pending→error / Data : healthz error → **500** | [✅🆗] |
| 1-1a | `GET /ready` | - | 레디니스 프로브 | ***DB/Redis 연결 상태 확인***<br>**성공(정상):** → **200**<br>**실패(의존성 불가):** → **503** | [✅] |
| 1-2 | `GET /docs` | `/docs` | API 문서 | ***Swagger 태그 순서 고정(health → auth → user → videos → study → lesson → admin)***<br>**성공:** Auth pass / Page : docs init→ready / Request : docs pending→success / Data : docs present → **200**<br>**실패(스키마 집계 실패):** Auth pass / Page : docs init→ready / Request : docs pending→error / Data : docs error → **500**<br>**실패(정적 경로 누락):** Auth pass / Page : docs init→ready / Request : docs pending→error / Data : docs error → **404** | [✅] |

---

<details>
  <summary>Phase 1 — health 시나리오</summary>

#### 5.1-1 : `GET /healthz` | `GET /health` 시나리오
> `GET /health`는 `GET /healthz`의 별칭(alias)으로, 동일한 핸들러를 공유한다.

- **성공**
  - When: 클라이언트가 `GET /healthz` 또는 `GET /health` 호출
  - Then: `200 OK`, JSON 바디 `{"status":"live","uptime_ms":..., "version":"v1.0.0"}`
  - **PROD-5**: `APP_ENV=production`이면 `version` 필드 생략 (`Option<String>`, `skip_serializing_if`)
  - 상태축: Auth=pass / Page=init→ready / Request=pending→success / Data=present
- **실패**
  - When: 헬스 핸들러 내부 예외
  - Then: `500 Internal Server Error`, 에러 바디 `{"error":{"http_status":500,"code":"HEALTH_INTERNAL"}}`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→error / Data=error

---

#### 5.1-1a : `GET /ready` (레디니스 프로브)
- **성공(정상)**
  - When: 클라이언트가 `GET /ready` 호출
  - Then: `200 OK`, DB 및 Redis 연결이 정상임을 확인
- **실패(의존성 불가)**
  - When: DB 또는 Redis 연결 실패
  - Then: `503 Service Unavailable`

---

#### 5.1-2 : `GET /docs` 시나리오
- **PROD-6**: `ENABLE_DOCS=false` (프로덕션 기본)이면 Swagger UI 비활성화 → 404 반환
- **성공** (`ENABLE_DOCS=true`일 때)
  - When: 클라이언트가 `GET /docs` 호출, Swagger에서만 실행
  - Then: `200 OK`, Swagger UI 렌더링, **태그 순서가 user→auth→videos→lesson→admin→health**로 보임
  - 상태축: Auth=pass / Page=init→ready / Request=pending→success / Data=present
- **실패(스키마 집계 실패)**
  - When: OpenAPI 집계/리소스 로드 오류
  - Then: `500 Internal Server Error`, `{"error":{"http_status":500,"code":"DOCS_BUILD_FAIL"}}`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→error / Data=error
- **실패(정적 경로 누락)**
  - When: 리버스 프록시/정적 경로 미설정
  - Then: `404 Not Found`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→error / Data=error
</details>

---

### 5.4 Phase 4 — video ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 4-1 | `GET /videos` | `/videos` | 비디오 목록 | ***`video_url_vimeo` 불러오기, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page videos init→ready / Request videos pending→success / Data videos present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page videos init→ready / Request videos pending→success / Data videos empty → **200**<br>실패(형식/누락): Auth pass 또는 stop / Page videos init→ready / Request videos pending→error / Data videos error → **400**<br>실패(도메인 제약): Auth pass 또는 stop / Page videos init→ready / Request videos pending→error / Data videos error → **422** | [✅🆗] |
| 4-2 | `GET /videos/{id}` | `/videos/{videos_id}` | 비디오 상세 | ***VIDEO_TAG 조회, 시청 로그 트리거(클라이언트 재생 시)***<br>성공: Auth pass 또는 stop / Page video init→ready / Request video pending→success / Data video present → **200**<br>실패(없는 영상): Auth pass 또는 stop / Page video init→ready / Request video pending→error / Data video error → **404** | [✅🆗] |
| 4-3 | `GET /videos/{id}/progress` | `/videos/{videos_id}` | 진행도 조회 | ***VIDEO_LOG: `progress_percent`, `last_watched_at` 조회***<br>성공: Auth pass / Page video init→ready / Request progress pending→success / Data progress present(또는 empty=기록없음, 0%) → **200**<br>실패(미인증): Auth stop / Page video init→ready / Request progress pending→error / Data progress error → **401**<br>실패(없는 영상): Auth pass / Page video init→ready / Request progress pending→error / Data progress error → **404** | [✅🆗] |
| 4-4 | `POST /videos/{id}/progress` | `/videos/{videos_id}` | 진행도 갱신 | ***0~100 고정(멱등연산) → VIDEO_LOG 저장(`progress_percent`, `last_watched_at`)***<br>성공:<br> Auth pass / Page video init→ready / Form progress pristine→dirty→validating→submitting→success /<br> Request progress pending→success / Data progress present → **200**(또는 **204**)<br>실패(형식/누락):<br> Auth pass / Page video init→ready / Form progress pristine→dirty→validating→error.client / Request progress pending→error / Data progress empty → **400**<br>실패(도메인 제약: 범위/증감 규칙):<br> Auth pass / Page video init→ready / Form progress pristine→dirty→validating→error.client / Request progress pending→error / Data progress error → **422**<br>실패(미인증): Auth stop / Page video init→ready / Request progress pending→error / Data progress error → **401**<br>실패(없는 영상): Auth pass / Page video init→ready / Request progress pending→error / Data progress error → **404** | [✅🆗] |

---

<details>
  <summary>5.4 Phase 4 — video 시나리오 상세 (5.4-1 ~ 5.4-4)</summary>

#### 공통 정책(5.4-1 ~ 5.4-4)
- **에러 바디(고정)**
  `{ "error": { "http_status": 400|401|404|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **검증 기준**
  - **400** = 형식 오류/필수 누락/파싱 실패(예: page, per_page 숫자 아님)
  - **422** = 도메인 제약 위반(예: progress 0~100 범위 위반, 증가/감소 규칙 위반을 둘 경우)
- **진행도 규칙**
  - 멱등: 동일 값 재전송은 상태 변화 없이 성공
  - `last_watched_at`는 서버 시각으로 갱신
  - 기록 없음(progress 미생성)은 **200 + empty(0%)**로 응답(오류 아님)

---

#### 응답 스키마

**VideoListRes (목록 응답)**
```json
{
  "meta": {
    "total_count": 100,
    "total_pages": 5,
    "current_page": 1,
    "per_page": 20
  },
  "data": [VideoListItem, ...]
}
```

**VideoListItem (목록 아이템)**
| 필드 | 타입 | 설명 |
|------|------|------|
| `video_id` | `i64` | 비디오 고유 ID |
| `video_idx` | `string` | 비즈니스 식별 코드 (예: VID-001) |
| `title` | `string?` | 영상 제목 (video_tag에서 가져옴) |
| `subtitle` | `string?` | 영상 설명 (video_tag에서 가져옴) |
| `duration_seconds` | `i32?` | 영상 길이 (초, Vimeo 동기화) |
| `language` | `string?` | 언어 코드 |
| `thumbnail_url` | `string?` | 썸네일 URL (Vimeo 동기화) |
| `state` | `string` | 상태 (draft, published, archived) |
| `access` | `string` | 접근권한 (public, private, restricted) |
| `tags` | `string[]` | 태그 문자열 배열 |
| `has_captions` | `bool` | 자막 유무 |
| `created_at` | `datetime` | 생성일시 |

**VideoDetailRes (상세 응답)**
| 필드 | 타입 | 설명 |
|------|------|------|
| `video_id` | `i64` | 비디오 고유 ID |
| `video_url_vimeo` | `string` | Vimeo 영상 URL |
| `video_state` | `string` | 상태 (draft, published, archived) |
| `tags` | `VideoTagDetail[]` | 태그 상세 배열 |
| `created_at` | `datetime` | 생성일시 |

**VideoTagDetail (태그 상세)**
| 필드 | 타입 | 설명 |
|------|------|------|
| `key` | `string?` | 태그 키 |
| `title` | `string?` | 태그 제목 |
| `subtitle` | `string?` | 태그 설명 |

**VideoProgressRes (진행도 응답)**
| 필드 | 타입 | 설명 |
|------|------|------|
| `video_id` | `i64` | 비디오 고유 ID |
| `progress_rate` | `i32` | 진행률 (0~100) |
| `is_completed` | `bool` | 완료 여부 |
| `last_watched_at` | `datetime?` | 마지막 시청 시각 |

---

#### 5.4-1 : `GET /videos` (비디오 목록)
- **로그인 안해도 접근 가능**
- **성공(데이터 있음) → 200**
  - When: `/videos` 진입, `page/per_page/sort`가 유효
  - Then: **200**, 목록 + 페이지 메타, 각 항목에 `video_url_vimeo` 포함
  - 상태축: Auth=pass 또는 stop / Page=`videos` init→ready / Request=`videos` pending→success / Data=`videos` present

- **성공(데이터 없음) → 200**
  - Then: **200**, 빈 배열 + 페이지 메타
  - 상태축: Data=`videos` empty

- **실패(형식/누락) → 400**
  - 예: `page=abc`(숫자 아님), `per_page=foo`(숫자 아님), `sort=` 값 파싱 불가(쉼표/형식 오류)
  - 상태축: Auth=pass 또는 stop / Page=`videos` init→ready / Request=`videos` pending→error / Data=`videos` error

- **실패(도메인 제약) → 422**
  - 예: `page<1`, `per_page<1` 또는 허용 상한 초과(예: `per_page>100`), `sort` 값이 허용 목록 외, `lang` 필터가 허용되지 않은 언어코드
  - 상태축: Auth=pass 또는 stop / Page=`videos` init→ready / Request=`videos` pending→error / Data=`videos` error


---

#### 5.4-2 : `GET /videos/{id}` (비디오 상세)
- **성공 → 200 OK**
  - When: 상세 진입, 존재하는 영상 id. 옵션 `?lang=<언어>` 지정 시 번역 오버라이드.
  - Then: **200**, 본문에 `video_id`, `video_url_vimeo`, `video_state`, `title`, `subtitle`, `tags[]`, `created_at`. `title`/`subtitle` 은 `video_tag_title`/`video_tag_subtitle` MAX 집계 → `?lang=` 있을 때 `content_translations` 의 `video_title`/`video_subtitle` 로 오버라이드 (fallback: lang → en → 원본).
  - 상태축: Auth=pass 또는 stop / Page=`video` init→ready / Request=`video` pending→success / **Data=`video` present**
- **실패(없는 영상) → 404 Not Found**
  - When: 잘못된 id
  - 상태축: Request … → error / **Data=`video` error**

> 메모: 실제 시청(재생 시작/완료 등)은 클라이언트에서 비메오 플레이어 이벤트로 잡고, 별도 **progress API**(3-4)를 호출해 **VIDEO_LOG**를 적재.

---

#### 5.4-3 : `GET /videos/{id}/progress` (진행도 조회)
- **성공(기록 있음) → 200 OK**
  - When: 인증된 사용자가 자신의 진행도 조회
  - Then: **200**, `{ progress_percent, last_watched_at }`
  - 상태축: Auth=pass / Page=`video` init→ready / Request=`progress` pending→success / **Data=`progress` present**
- **성공(기록 없음) → 200 OK**
  - Then: **200**, `{ progress_percent: 0, last_watched_at: null }`
  - 상태축: Data=`progress` **empty**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
  - 상태축: Auth=stop / Request … → error / Data=`progress` error
- **실패(없는 영상) → 404 Not Found**

---

#### 5.4-4 : `POST /videos/{id}/progress` (진행도 갱신)
- **성공 → 200 OK(또는 204 No Content)**
  - When: 클라이언트가 재생 이벤트 동안 진행도(0~100)를 전송
  - Then: **200**(업데이트 후 스냅샷 반환) **혹은 204**, 서버는 `progress_percent`(클램프 0~100)와 `last_watched_at` 갱신, **VIDEO_LOG upsert**
  - 상태축: Auth=pass / Page=`video` init→ready / **Form=`progress` pristine→dirty→validating→submitting→success** / Request=`progress` pending→success / **Data=`progress` present**
- **실패(형식/누락) → 400 Bad Request**
  - 예: `progress_percent`가 숫자 아님, 바디 누락
  - 상태축: **Form=`progress` … → error.client** / Request … → error / **Data=`progress` empty**
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: 범위(0~100) 위반, (정책 선택 시) 지나친 감소 등 규칙 위반
  - 상태축: **Form=`progress` … → error.client** / Request … → error / **Data=`progress` error**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
- **실패(없는 영상) → 404 Not Found**
</details>

---

### 5.5 Phase 5 — study ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 5-1 | `GET /studies` | `/studies` | 학습 문제 목록 | ***`study_program_enum` 기준 조회, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page studies init→ready / Request studies pending→success / Data studies present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page studies init→ready / Request studies pending→success / Data studies empty → **200**<br>실패(형식/누락): Auth pass 또는 stop / Page studies init→ready / Request studies pending→error / Data studies error → **400**<br>실패(도메인 제약): Auth pass 또는 stop / Page studies init→ready / Request studies pending→error / Data studies error → **422** | [✅🆗] |
| 5-2 | `GET /studies/{id}` | `/studies/{study_id}` | Study 상세 (Task 목록) | ***STUDY 상세 + 해당 Study의 STUDY_TASK 목록 조회, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page study init→ready / Request study pending→success / Data study present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page study init→ready / Request study pending→success / Data study empty → **200** (Task 없음)<br>실패(없는 Study): Auth pass 또는 stop / Page study init→ready / Request study pending→error / Data study error → **404** | [✅🆗] |
| 5-3 | `GET /studies/tasks/{id}` | `/studies/tasks/{task_id}` | 학습 문제 상세 | ***STUDY_TASK 조회, 보기(풀이 전)→ STUDY_TASK_LOG 저장(view)***<br>성공: Auth pass 또는 stop / Page task init→ready / Request task pending→success / Data task present → **200**<br>실패(없는 문항): Auth pass 또는 stop / Page task init→ready / Request task pending→error / Data task error → **404** | [✅🆗] |
| 5-4 | `POST /studies/tasks/{id}/answer` | `/studies/tasks/{task_id}` | 정답 제출/채점 | ***STUDY_TASK_STATUS 업데이트 → STUDY_TASK_LOG 저장(채점 포함)***<br>성공:<br> Auth pass / Page task init→ready / Form answer pristine→dirty→validating→submitting→success /<br> Request answer pending→success / Data answer present → **200**<br>실패(형식/누락):<br> Auth pass / Page task init→ready / Form answer pristine→dirty→validating→error.client / Request answer pending→error / Data answer empty → **400**<br>실패(도메인 제약: 선택지 범위/중복 허용 규칙 등):<br> Auth pass / Page task init→ready / Form answer pristine→dirty→validating→error.client / Request answer pending→error / Data answer error → **422**<br>실패(미인증): Auth stop / Page task init→ready / Request answer pending→error / Data answer error → **401**<br>실패(없는 문항): Auth pass / Page task init→ready / Request answer pending→error / Data answer error → **404** | [✅🆗] |
| 5-5 | `GET /studies/tasks/{id}/status` | `/studies/tasks/{task_id}` | 내 시도/기록 | ***내 최신 STATUS(progress/score/attempts) 조회***<br>성공: Auth pass / Page task init→ready / Request status pending→success / Data status present(또는 empty=기록없음) → **200**<br>실패(미인증): Auth stop / Page task init→ready / Request status pending→error / Data status error → **401**<br>실패(없는 문항): Auth pass / Page task init→ready / Request status pending→error / Data status error → **404** | [✅🆗] |
| 5-6 | `GET /studies/tasks/{id}/explain` | `/studies/tasks/{task_id}/explain` | 해설 보기 | ***STUDY_EXPLAIN 문항별 해설/미디어***<br>성공: Auth pass 또는 stop / Page explain init→ready / Request explain pending→success / Data explain present → **200**<br>실패(없는 문항/해설 없음): Auth pass 또는 stop / Page explain init→ready / Request explain pending→error / Data explain error → **404**<br>실패(도메인 정책: 시도 전 열람 금지 설정 시): Auth pass 또는 stop / Page explain ready / Request explain pending→error / Data explain error → **403** | [✅🆗] |
| 5-7 | `POST /studies/writing/sessions` | `/study/writing/:level/:type/:taskId` | 한글 자판 연습 세션 시작 | ***WRITING_PRACTICE_SESSION insert (started_at, writing_level, writing_practice_type, study_task_id)*** | [✅🆗] |
| 5-8 | `PATCH /studies/writing/sessions/{id}` | `/study/writing/:level/:type/:taskId` | 한글 자판 연습 세션 완료 | ***client 측정 total_chars/correct_chars/duration_ms/mistakes → 서버에서 accuracy_rate/CPM 계산 후 finished_at 저장***<br>400: total_chars/correct_chars/duration_ms 음수<br>422: correct_chars > total_chars<br>404: 타인 세션 또는 미존재 | [✅🆗] |
| 5-9 | `GET /studies/writing/sessions` | `/study/writing/history` | 내 세션 목록 | ***user_id 기반 페이지네이션, level/finished_only 필터*** | [✅🆗] |
| 5-10 | `GET /studies/writing/stats` | `/study/writing/stats` | 통계 대시보드 | ***days 파라미터(기본 30, 최대 365) 범위에서 total/avg_accuracy/avg_cpm + 레벨별 + 일별 추이 + 취약 글자 Top 10*** | [✅🆗] |
| 5-11 | `GET /studies/writing/practice` | `/studies/writing/:level/:type` | 자유 연습 시드 컨텐츠 | ***level+practice_type 필터, seq 오름차순, 기본 20 / 최대 100, 비인증 허용. `study_writing_practice_seed` 테이블에서 prompt/answer/hint 반환*** | [✅🆗] |

---

<details>
  <summary>5.5 Phase 5 — study 시나리오 상세 (5.5-1 ~ 5.5-10)</summary>

#### 공통 정책(5.5-1 ~ 5.5-10)
- **에러 바디(고정)**
  `{ "error": { "http_status": 400|401|403|404|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **검증 기준**
  - **400** = 형식/누락/파싱 실패(예: `page=abc`, `program=` 빈값)
  - **422** = 도메인 제약 위반(예: `study_program_enum`에 없는 값, `per_page` 상한 초과, 보기 규칙 위반)
- **로그**
  - 문제 조회(5-3): **STUDY_TASK_LOG**에 study_task_action_log 컬럼 study_task_log_action_enum 바탕으로 `view` 업데이트
  - 정답 제출(5-4)
    1. **STUDY_TASK_STATUS**에 업데이트 : 시도횟수(`study_task_status_try`), 최고점(`study_task_status_best`), 완료여부(`study_task_status_completed`)
    2. **STUDY_TASK_LOG**에 업데이트 : 학습행동(`study_task_action_log`), 시도횟수(`study_task_try_no_log`), 점수기록(`study_task_score_log`), 완료여부(`study_task_is_correct_log`), 풀이기록(`study_task_payload_log`),
  - 상태 조회(5-5): **STUDY_TASK_LOG**에 study_task_action_log 컬럼 study_task_log_action_enum 바탕으로 `status` 업데이트
- **레이트리밋(선택)**
  - 과도한 채점/새로고침 방지 → **429 + Retry-After**
- **권한/공개 정책**
  - 목록/상세/해설은 서비스 정책에 따라 공개/비공개를 조절 가능(기본: 공개 열람 가능, 정답 제출·내 기록 조회는 인증 필요)

---

#### 5.5-1 : `GET /studies` (학습 문제 목록)
- **로그인 안해도 접근 가능**
- **성공(데이터 있음) → 200**
  - When: `/studies` 진입, `program/page/per_page/sort` 유효
  - Then: **200**, 목록 + 페이지 메타, `study_program_enum` 필터 반영
  - 상태축: Auth=pass 또는 stop / Page=`studies` init→ready / Request=`studies` pending→success / Data=`studies` present
- **성공(데이터 없음) → 200**
  - 빈 배열 + 페이지 메타 / Data=`studies` empty
- **실패(형식/누락) → 400**
  - 예: `page`/`per_page` 숫자 아님, `program` 파라미터 형식 오류
- **실패(도메인 제약) → 422**
  - 예: `program`이 enum에 없음, `per_page` 상한 초과, 허용되지 않은 `sort` 필드

---

#### 5.5-2 : `GET /studies/{id}` (Study 상세 + Task 목록)
- **로그인 안해도 접근 가능**
- **성공(데이터 있음) → 200**
  - When: `/studies/{study_id}` 진입, `page/per_page` 유효
  - Then: **200**, Study 정보 + 해당 Study의 Task 목록 + 페이지 메타
  - 응답 예시:
    ```json
    {
      "study_id": 1,
      "study_idx": "test-1",
      "program": "basic_word",
      "title": "한글 자음 연습",
      "subtitle": "\"ㅏ\"로 자음 연습 하기",
      "state": "open",
      "tasks": [
        { "task_id": 1, "kind": "choice", "seq": 1 },
        { "task_id": 2, "kind": "typing", "seq": 2 }
      ],
      "meta": { "total_count": 2, "total_pages": 1, "page": 1, "per_page": 10 }
    }
    ```
  - 상태축: Auth=pass 또는 stop / Page=`study` init→ready / Request=`study` pending→success / Data=`study` present
- **성공(Task 없음) → 200**
  - Study는 존재하지만 Task가 없는 경우 빈 배열 반환
  - Data=`study` present, `tasks` empty
- **실패(없는 Study) → 404**
  - 잘못된 `{id}`
- **실패(형식/누락) → 400**
  - 예: `page`/`per_page` 숫자 아님

---

#### 5.5-3 : `GET /studies/tasks/{id}` (학습 문제 상세)
- 성공 → **200**
  - When: 상세 진입. 옵션 `?lang=<언어>` 지정 시 payload 번역 오버라이드.
  - Then: **200**, 문제 본문/보기/메타(난이도/분류) → **STUDY_TASK_LOG** `view` 업데이트. `?lang=` 있을 때 task kind 별로 `content_translations` 조회 → choice(`study_task_choice_question`/`_1~4`), typing(`study_task_typing_question`), voice(`study_task_voice_question`), writing(`study_task_writing_prompt`/`_answer`/`_hint`) 필드 오버라이드.
  - 상태축: Auth=pass 또는 stop / Page=`task` init→ready / Request=`task` pending→success / Data=`task` present
- 실패(없는 문항) → **404**
  - 잘못된 `{id}`

---

#### 5.5-4 : `POST /studies/tasks/{id}/answer` (정답 제출/채점)
- 성공 → **200**
  - When: 인증 사용자,
    1. study_task_typing : 타이핑 시도 → **STUDY_TASK_LOG** `start` 업데이트 → 타이핑 완료 → **STUDY_TASK_LOG** `answer` 업데이트
    2. study_task_choice : 선택지 클릭 → **STUDY_TASK_LOG** `answer` 업데이트
    3. study_task_voice : 녹음 버튼 클릭 → **STUDY_TASK_LOG** `start` 업데이트 → 녹음 버튼 재클릭 → **STUDY_TASK_LOG** `answer` 업데이트
    4. study_task_writing : 한글 자판 타이핑 → **STUDY_TASK_LOG** `start` 업데이트 → 제출 → **STUDY_TASK_LOG** `answer` 업데이트 (세션 단위 통계는 P4 `study_writing_practice_session` API로 별도 집계)
  - Then: **200**,
    1. study_task_typing : 채점 → **STUDY_TASK_TYPING** `study_task_typing_answer` 대조 → **STUDY_TASK_STATUS** 결과 업데이트 → **STUDY_TASK_LOG** `finish` 업데이트
    2. study_task_choice : 채점 → **STUDY_TASK_CHOICE** `study_task_choice_answer` 대조 → **STUDY_TASK_STATUS** 결과 업데이트 → **STUDY_TASK_LOG** `finish` 업데이트
    3. study_task_voice : 채점 →  **STUDY_TASK_VOICE** `study_task_voice_answer` 대조 → **STUDY_TASK_STATUS** 결과 업데이트 → **STUDY_TASK_LOG** `finish` 업데이트
    4. study_task_writing : 채점 → **STUDY_TASK_WRITING** `study_task_writing_answer` 대조 → **STUDY_TASK_STATUS** 결과 업데이트 → **STUDY_TASK_LOG** `finish` 업데이트. 초급 레벨은 `answer`가 payload에 포함되어 클라이언트에서 실시간 글자별 피드백을 렌더링.
  - 상태축: Auth=pass / Page=`task` init→ready / Form=`answer` pristine→dirty→validating→submitting→success / Request=`answer` pending→success / Data=`answer` present
- 실패(형식/누락) → **400**
  - 예: 바디 없음, 선택지 배열 스키마 불일치, 서술형 빈 문자열 금지 등
  - 상태축: Form=`answer` … → error.client / Request=`answer` pending→error / Data=`answer` empty
- 실패(도메인 제약) → **422**
  - 예: 단일선택 문항에 다중 선택 제출, 범위를 벗어난 보기 인덱스, 이미 종료된 시도에 재제출 금지 정책 등
  - 상태축: Form=`answer` … → error.client / Request=`answer` pending→error / Data=`answer` error
- 실패(미인증) → **401**
  - 토큰 없음/만료
- 실패(없는 문항) → **404**
  - 잘못된 `{id}`
- 실패(레이트리밋, 선택) → **429**
  - 과도한 제출/채점 요청

---

#### 5.5-5 : `GET /studies/tasks/{id}/status` (내 시도/기록)
- 성공 → **200**
  - Then: **200**, `{ study_task_status_try_count, study_task_status_is_solved, study_task_status_last_attempt_at }` → **STUDY_TASK_LOG** `status` 업데이트
  - 상태축: Auth=pass / Page=`task` init→ready / Request=`status` pending→success / Data=`status` present(또는 empty)
- 실패(미인증) → **401**
  - 토큰 없음/만료
- 실패(없는 문항) → **404**

---

#### 5.5-6 : `GET /studies/tasks/{id}/explain` (해설 보기)
- 성공 → **200**
  - When: 최소 1회 시도 후. 옵션 `?lang=<언어>` 지정 시 번역 오버라이드.
  - Then: **200**,`{ title, explanation, resources[] }` → **STUDY_TASK_LOG** `explain` 업데이트. `?lang=` 있을 때 `content_translations` 의 `explain_title`/`explain_text` 로 오버라이드 (fallback: lang → en → 원본).
  - 상태축: Auth=pass 또는 stop / Page=`explain` init→ready / Request=`explain` pending→success / Data=`explain` present
- 실패(해설 없음/없는 문항) → **404**
  - 자료 미제공 또는 잘못된 `{id}`
- 실패(정책상 제한) → **403**
  - 예: "최소 1회 시도 후 열람" 정책을 켠 경우, 시도 전 접근 차단

---

#### 5.5-7 : `POST /studies/writing/sessions` (한글 자판 연습 세션 시작)
- **요청 (JSON Body)**
  ```json
  {
    "study_task_id": 123,                 // 관리자 태스크 기반이면 설정, 자유 연습이면 null
    "writing_level": "beginner",          // beginner | intermediate | advanced
    "writing_practice_type": "syllable"   // jamo | syllable | word | sentence | paragraph
  }
  ```
- 성공 → **200**
  - Then: **WRITING_PRACTICE_SESSION** insert (`started_at=NOW()`, `finished_at=NULL`, `total_chars/correct_chars=0`, `accuracy_rate/chars_per_minute=0`), 생성된 session_id + 메타 반환
- 실패(미인증) → **401**
- 실패(존재하지 않는 writing 태스크) → **404**
  - `study_task_id`가 주어졌지만 `study_task_writing` 서브레코드가 없거나 study가 `open`이 아님

---

#### 5.5-8 : `PATCH /studies/writing/sessions/{id}` (세션 완료 / 결과 저장)
- **요청 (JSON Body)** — 클라이언트 측정값만 전달하고 서버가 정확도/CPM을 계산한다
  ```json
  {
    "total_chars": 120,
    "correct_chars": 114,
    "duration_ms": 45000,
    "mistakes": [
      { "position": 7, "expected": "ㅓ", "actual": "ㅏ" }
    ]
  }
  ```
- **서버 계산**
  - `accuracy_rate = correct_chars / total_chars * 100` (0~100, 소수점 2자리)
  - `chars_per_minute = total_chars * 60000 / duration_ms` (0~99999.99)
  - `mistakes` 배열은 JSONB로 그대로 저장 (취약 글자 통계에 사용)
  - `finished_at = NOW()`
- 성공 → **200**
  - UPDATE ... WHERE `session_id=$id` AND `user_id=auth.sub` 로 소유권 검증
- 실패(형식/누락) → **400**
  - `total_chars<0`, `correct_chars<0`, `duration_ms<0`
- 실패(도메인 제약) → **422**
  - `correct_chars > total_chars`
- 실패(미인증) → **401**
- 실패(타인 세션/미존재) → **404**

---

#### 5.5-9 : `GET /studies/writing/sessions` (세션 목록)
- **Query Params**: `page` (기본 1), `per_page` (기본 20, 최대 100), `level?`, `finished_only?`
- 성공 → **200**
  - 내 세션만 반환 (`user_id = auth.sub`). `started_at DESC` 정렬
- 실패(형식/누락) → **400** (`page=0`, `per_page=0`)
- 실패(도메인 제약) → **422** (`per_page>100`)
- 실패(미인증) → **401**

---

#### 5.5-10 : `GET /studies/writing/stats` (통계 대시보드)
- **Query Params**: `days` (기본 30, 최대 365)
- 성공 → **200**
  ```json
  {
    "total_sessions": 42,
    "avg_accuracy": 95.12,
    "avg_cpm": 180.30,
    "level_breakdown": [
      { "writing_level": "beginner", "sessions": 20, "avg_accuracy": 98.1, "avg_cpm": 90.0 }
    ],
    "recent_trend": [
      { "day": "2026-04-01", "sessions": 3, "avg_accuracy": 96.5, "avg_cpm": 175.2 }
    ],
    "weak_chars": [
      { "expected": "ㅓ", "miss_count": 17 }
    ]
  }
  ```
  - `finished_at IS NOT NULL` 인 세션만 집계
  - `recent_trend`: 일별(`DATE_TRUNC('day', started_at)`) GROUP BY, 시간순 정렬
  - `weak_chars`: `jsonb_array_elements(mistakes)` 로 펼쳐서 `expected` 기준 COUNT Top 10
- 실패(형식/누락) → **400** (`days=0`)
- 실패(도메인 제약) → **422** (`days>365`)
- 실패(미인증) → **401**

#### 5.5-11 : `GET /studies/writing/practice` (자유 연습 시드 컨텐츠)
- **비인증** 허용. `study_task` 수강권과 무관한 드릴 컨텐츠 제공.
- **Query Params**
  - `level` (필수) — `beginner` | `intermediate` | `advanced`
  - `practice_type` (필수) — `jamo` | `syllable` | `word` | `sentence` | `paragraph`
  - `limit` (선택, 기본 20, 최대 100)
- 성공 → **200**
  ```json
  {
    "level": "beginner",
    "practice_type": "jamo",
    "items": [
      { "seed_id": 1, "seq": 1, "prompt": "ㄱ", "answer": "ㄱ", "hint": "giyeok" }
    ]
  }
  ```
  - `study_writing_practice_seed` 테이블에서 `(level, practice_type)` 필터로 `seq` 오름차순 조회
  - `prompt` = 화면에 표시할 텍스트, `answer` = 학습자가 입력해야 할 정답 (대부분 동일)
  - `hint`는 optional (초급 jamo만 로마자 표기 포함)
- 실패(형식/누락) → **400** (`limit=0`)
- 실패(도메인 제약) → **422** (`limit>100`, 잘못된 level/practice_type)

</details>

---

### 5.6 Phase 6 — lesson ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 6-1 | `GET /lessons` | `/lessons` | 수업 전체 목록 | ***`lesson_idx` 기준 조회, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→success / Data lessons present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→success / Data lessons empty → **200**<br>실패(형식/누락): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→error / Data lessons error → **400**<br>실패(도메인 제약): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→error / Data lessons error → **422** | [✅🆗] |
| 6-2 | `GET /lessons/{id}` | `/lessons/{lesson_id}` | 수업 상세 | ***`video_tag_id` + `study_task_id` 기반 목록 조회, 페이지네이션***<br>성공: Auth pass 또는 stop / Page lesson init→ready / Request lesson pending→success / Data lesson present → **200**<br>실패(없는 수업): Auth pass 또는 stop / Page lesson init→ready / Request lesson pending→error / Data lesson error → **404** | [✅🆗] |
| 6-3 | `GET /lessons/{id}/items` | `/lessons/{lesson_id}/items` | 수업 학습 | ***`lesson_item_seq` 기준 조회, 학습 화면 로드(풀이/진행은 별도 API)***<br>성공: Auth pass 또는 stop / Page lesson_items init→ready / Request lesson_items pending→success / Data lesson_items present → **200**<br>실패(없는 수업/항목): Auth pass 또는 stop / Page lesson_items init→ready / Request lesson_items pending→error / Data lesson_items error → **404**<br>실패(정책상 제한: 수강권 필요): Auth forbid / Page lesson_items ready / Request lesson_items pending→error / Data lesson_items error → **403**<br>실패(형식/누락·도메인): Auth pass 또는 stop / Page lesson_items init→ready / Request lesson_items pending→error / Data lesson_items error → **400**/**422** | [✅🆗] |
| 6-4 | `GET /lessons/{id}/progress` | `/lessons/{lesson_id}` | 수업 진행 조회 | ***LESSON_PROGRESS 최신 값 조회(없으면 0%)***<br>성공: Auth pass / Page lesson init→ready / Request lesson_progress pending→success / Data lesson_progress present(또는 empty=0%) → **200**<br>실패(미인증): Auth stop / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **401**<br>실패(없는 수업): Auth pass / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **404** | [✅🆗] |
| 6-5 | `POST /lessons/{id}/progress` | `/lessons/{lesson_id}` | 수업 진행 갱신 | ***LESSON_PROGRESS 컬럼 업데이트(0~100 고정, 멱등)***<br>성공:<br> Auth pass / Page lesson init→ready / Form lesson_progress pristine→dirty→validating→submitting→success /<br> Request lesson_progress pending→success / Data lesson_progress present → **200**(또는 **204**)<br>실패(형식/누락):<br> Auth pass / Page lesson init→ready / Form lesson_progress pristine→dirty→validating→error.client /<br> Request lesson_progress pending→error / Data lesson_progress empty → **400**<br>실패(도메인 제약: 범위/증감 규칙):<br> Auth pass / Page lesson init→ready / Form lesson_progress pristine→dirty→validating→error.client /<br> Request lesson_progress pending→error / Data lesson_progress error → **422**<br>실패(미인증): Auth stop / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **401**<br>실패(없는 수업): Auth pass / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **404**<br>실패(정책상 제한: 수강권 필요): Auth forbid / Page lesson ready / Request lesson_progress pending→error / Data lesson_progress error → **403** | [✅🆗] |

---

<details>
  <summary>5.6 Phase 6 — lesson 시나리오 상세 (5.6-1 ~ 5.6-5)</summary>

#### 공통 정책(5.6-1 ~ 5.6-5)
- **에러 바디(고정)**
  `{ "error": { "http_status": 400|401|403|404|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **검증 기준**
  - **400** = 형식/누락/파싱 실패(예: `page=abc`, `per_page=foo`)
  - **422** = 도메인 제약 위반(예: `per_page` 상한, 허용되지 않은 `sort`, 진행도 0~100 범위 위반 등)
- **권한/수강권**
  - 수업 목록/상세/아이템은 서비스 정책에 따라 공개 가능하되, **수강권 필수 정책을 켜면 403** 적용
  - 진행도 조회/갱신은 **인증 필수**
- **진행도 규칙**
  - 멱등: 동일 값 재전송은 상태 변화 없이 성공
  - 기록 없음은 **200 + empty(0%)**로 응답(오류 아님)
- **로그**
  - 진행도 갱신(5-5): LESSON_PROGRESS 업데이트 시 서버시각으로 갱신, 필요 시 LESSON_PROGRESS_LOG(선택)

---

#### 5.6-1 : `GET /lessons` (수업 전체 목록)
- **로그인 안해도 접근 가능**
- 성공(데이터 있음) → **200**
  - When: `/lessons` 진입, `page/per_page/sort` 유효
  - Then: **200**, 목록 + 페이지 메타(`lesson_idx` 기준 정렬)
  - 상태축: Auth=pass 또는 stop / Page=`lessons` init→ready / Request=`lessons` pending→success / Data=`lessons` present
- 성공(데이터 없음) → **200**
  - 빈 배열 + 페이지 메타 / Data=`lessons` empty
- 실패(형식/누락) → **400**
  - 예: 숫자 아님, 음수/0 페이지
- 실패(도메인 제약) → **422**
  - 예: `per_page` 상한 초과, 허용 외 정렬 키

---

#### 5.6-2 : `GET /lessons/{id}` (수업 상세)
- 성공 → **200**
  - Then: **200**, 수업 메타 + 연계 목록(영상 태그/학습 과제 id 집합) 페이지네이션
  - 상태축: Auth=pass 또는 stop / Page=`lesson` init→ready / Request=`lesson` pending→success / Data=`lesson` present
- 실패(없는 수업) → **404**

---

#### 5.6-3 : `GET /lessons/{id}/items` (수업 학습)
- 성공 → **200**
  - Then: **200**, `lesson_item_seq` 기준 아이템 목록(문항/비디오/자료 등), 학습 화면 로드
  - 상태축: Auth=pass 또는 stop / Page=`lesson_items` init→ready / Request=`lesson_items` pending→success / Data=`lesson_items` present
- 실패(없는 수업/항목) → **404**
- 실패(정책상 제한: 수강권 필요) → **403**
- 실패(형식/누락 → 400 / 도메인 제약 → 422)**

---

#### 5.6-4 : `GET /lessons/{id}/progress` (수업 진행 조회)
- 성공 → **200**
  - Then: **200**, `{ progress_percent, last_updated_at }` (없으면 `{0, null}`)
  - 상태축: Auth=pass / Page=`lesson` init→ready / Request=`lesson_progress` pending→success / Data=`lesson_progress` present(또는 empty)
- 실패(미인증) → **401**
- 실패(없는 수업) → **404**

---

#### 5.6-5 : `POST /lessons/{id}/progress` (수업 진행 갱신)
- 성공 → **200**(또는 **204**)
  - When: 학습 중간/완료 시 진행도를 제출(0~100), 멱등 업데이트
  - Then: **200**(업데이트 후 스냅샷) 또는 **204**, 서버는 LESSON_PROGRESS 갱신
  - 상태축: Auth=pass / Page=`lesson` init→ready / Form=`lesson_progress` pristine→dirty→validating→submitting→success / Request=`lesson_progress` pending→success / Data=`lesson_progress` present
- 실패(형식/누락) → **400**
  - 예: 바디 누락, 숫자 아님
- 실패(도메인 제약) → **422**
  - 예: 0~100 범위 위반, (정책 선택 시) 역진행 금지
- 실패(미인증) → **401**
- 실패(없는 수업) → **404**
- 실패(정책상 제한: 수강권 필요) → **403**

</details>

---

### 5.8 Phase 8 — course ✅
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 8-1 | `GET /courses` | `/courses` | 코스 목록 조회 | ***페이지네이션, 접근 권한 체크***<br>응답에 `course_subtitle` 필드 포함<br>DTO: `CourseListQuery`(IntoParams), `CourseListItem`(ToSchema)<br>성공: **200** | [✅] |
| 8-2 | `POST /courses` | `/admin/courses/new` | 코스 생성 | ***ADMIN_COURSE_LOG, RBAC***<br>DTO: `CreateCourseReq`(ToSchema)<br>성공: **201**<br>실패: **401/403/400/422** | [✅] |
| 8-3 | `GET /courses/{id}` | `/courses/{id}` | 코스 상세 조회 | ***코스 정보 + 레슨 목록, `?lang=` 쿼리 파라미터 지원***<br>성공: **200**<br>실패: **404** | [✅] |

---

### 비고
- 모든 Phase는 "**백엔드 엔드포인트 구현 → 프론트 1화면 연동 → 스모크(성공+대표 에러)**" 순으로 완료 표시.

---

### 5.9 Phase 9 — translation (i18n)
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 9-1 | `GET /admin/translations` | `/admin/translations?page=&size=&content_type=&content_types=&content_id=&lang=&status=` | 번역 목록 조회 | ***필터(content_type/content_types, content_id, lang, status) + 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅] |
| 9-2 | `POST /admin/translations` | `/admin/translations/new` | 번역 단건 생성 (UPSERT) | ***content_type+content_id+field_name+lang 기준 UPSERT, 텍스트 변경 시에만 status 리셋, RBAC***<br>성공: **201**<br>실패: **401/403/400/422** | [✅] |
| 9-3 | `POST /admin/translations/bulk` | `/admin/translations/bulk` | 번역 벌크 생성 | ***부분 성공, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422** | [✅] |
| 9-4 | `GET /admin/translations/{id}` | `/admin/translations/{translation_id}` | 번역 상세 조회 | ***RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅] |
| 9-5 | `PATCH /admin/translations/{id}` | `/admin/translations/{translation_id}/edit` | 번역 수정 (텍스트/상태) | ***translated_text, status 부분 수정, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422** | [✅] |
| 9-6 | `PATCH /admin/translations/{id}/status` | `/admin/translations/{translation_id}` | 번역 상태만 변경 | ***draft → reviewed → approved 상태 전이, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422** | [✅] |
| 9-7 | `DELETE /admin/translations/{id}` | `/admin/translations/{translation_id}` | 번역 삭제 | ***RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅] |
| ~~9-8~~ | ~~`POST /admin/translations/auto`~~ | — | ~~자동 번역 (GCP)~~ | **삭제됨** (2026-03-24, Google Translate API 해지) | — |
| 9-9 | `GET /admin/translations/content-records` | - | 콘텐츠 목록 조회 (드롭다운용) | ***content_type별 레코드 목록 반환, RBAC***<br>성공: **200**<br>실패: **401/403/400** | [✅] |
| 9-10 | `GET /admin/translations/source-fields` | - | 원본 텍스트 조회 | ***content_type+content_id로 한국어 원본 필드 조회, RBAC***<br>성공: **200**<br>실패: **401/403/400** | [✅] |
| ~~9-11~~ | ~~`POST /admin/translations/auto-bulk`~~ | — | ~~벌크 자동 번역~~ | **삭제됨** (2026-03-24, Google Translate API 해지) | — |
| 9-12 | `GET /admin/translations/search` | - | 번역 검색 (재사용) | ***lang으로 최근 approved/reviewed 번역 조회, RBAC***<br>성공: **200**<br>실패: **401/403** | [✅] |
| 9-13 | `GET /admin/translations/stats` | - | 번역 통계 조회 | ***번역 현황 통계 반환, RBAC***<br>성공: **200**<br>실패: **401/403** | [✅] |

---

<details>
  <summary>5.9 Phase 9 — translation (i18n) 상세</summary>

#### 다국어 콘텐츠 번역 시스템 개요

> 모든 학습 콘텐츠의 번역을 `content_translations` 테이블에서 통합 관리한다. 관리자가 번역을 생성/검수/승인하며, 승인된(approved) 번역만 최종 사용자에게 제공된다.

**핵심 정책**
- **Fallback 순서**: 사용자 언어(`?lang=`) → `en` → `ko` (한국어 원본)
- **공개 조건**: `status = 'approved'` 인 번역만 콘텐츠 API에서 제공
- **기존 콘텐츠 API 확장**: 레슨, 코스, 학습, 비디오 등 기존 API에 `?lang=` 쿼리 파라미터 추가
- **번역 인프라 (2026-05-18 정정)**: `content_translations` 의 콘텐츠 번역 = **Mac Mini 파이프라인이 정본** (구 amazing-korean-books seed SQL 방식은 폐기 — books `scripts/guide-v2/seed_output/*_translations.sql` 무시). books 는 콘텐츠 **본문(ko 원문)** 만 시드(study/task 테이블 컬럼), 다국어 overlay 는 본문 시드 후 Mac Mini 가 후속 일괄(explanation 선례와 동형). **frontend UI locale (`frontend/src/i18n/locales/{lang}.json`) 번역**도 `amazing-korean-ai` 리포의 Mac Mini Wave 1 파이프라인이 SSoT — Ollama `gemma4:26b` + 검증 4종 (E1/M01/Q-prefix/orthography) + 외부 LLM 합의. 자체 도구로 번역 금지. 상세: `amazing-korean-ai/docs/AMK_AI_TRANSLATION_HANDOFF.md`

**지원 언어 (36개, `ko` 원본 제외, 아랍어 RTL 별도)**

| 그룹 | 언어 코드 |
|------|-----------|
| 핵심 5개 (Phase 2) | `en`, `ja`, `zh-CN`, `zh-TW`, `vi` |
| 동남아시아 | `id`, `th`, `my`, `km`, `tl`, `lo` |
| 중앙/북아시아 | `mn`, `ru`, `uz`, `kk`, `tg`, `ky` |
| 남아시아 | `ne`, `si`, `hi`, `bn`, `ur` |
| 유럽 | `es`, `es-ES`, `pt`, `pt-PT`, `fr`, `de`, `it`, `pl`, `uk`, `tr` |
| 중동/아프리카 | `ar`, `fa`, `sw`, `am` |

> 2026-04-21 (+13: `tl`/`tr`/`bn`/`ar`/`ur`/`fa`/`lo`/`ky`/`it`/`sw`/`uk`/`am`/`pl`), 2026-04-28 (+`es-ES`/`pt-PT` 유럽 variant — "pt_pt → pt 병합" 정책 번복).

**번역 상태 전이**

```
draft → reviewed → approved
  ↑        ↓
  └────────┘  (검수 반려 시 draft로 되돌림)
```

---

#### 9-1 : `GET /admin/translations` (번역 목록 조회)

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `page` | i64 | N | 페이지 번호 (기본 1) |
| `size` | i64 | N | 페이지 크기 (기본 20, max 100) |
| `content_type` | string | N | 콘텐츠 유형 필터 단일 (course, lesson, video, video_tag, study, ...) |
| `content_types` | string | N | 콘텐츠 유형 필터 복수 (쉼표 구분, content_type보다 우선. e.g. `study,study_task_choice,study_task_typing`) |
| `content_id` | i64 | N | 콘텐츠 ID 필터 |
| `lang` | string | N | 언어 코드 필터 (en, ja, zh-CN, ...) |
| `status` | string | N | 상태 필터 (draft, reviewed, approved) |

**응답 (성공 200)**
```json
{
  "data": [
    {
      "translation_id": 1,
      "content_type": "lesson",
      "content_id": 42,
      "field_name": "lesson_title",
      "lang": "en",
      "translated_text": "Introduction to Korean Alphabet",
      "status": "approved",
      "created_at": "2026-02-10T12:00:00Z",
      "updated_at": "2026-02-10T14:30:00Z"
    }
  ],
  "total": 150,
  "page": 1,
  "size": 20
}
```

---

#### 9-2 : `POST /admin/translations` (번역 단건 생성 — UPSERT)

**요청 (TranslationCreateReq)**
```json
{
  "content_type": "lesson",
  "content_id": 42,
  "field_name": "lesson_title",
  "lang": "en",
  "translated_text": "Introduction to Korean Alphabet"
}
```

> **field_name 규약**: `{table}_{column}` 긴 이름 형식 준수 (예: `lesson_title`, `study_task_choice_question`). 2026-04-21 정합 확정. [plans/translation-field-name-alignment.md](../../.claude/plans/translation-field-name-alignment.md) §2.1 참조.

**응답 (성공 201)**
```json
{
  "translation_id": 1,
  "content_type": "lesson",
  "content_id": 42,
  "field_name": "lesson_title",
  "lang": "en",
  "translated_text": "Introduction to Korean Alphabet",
  "status": "draft",
  "created_at": "2026-02-10T12:00:00Z",
  "updated_at": "2026-02-10T12:00:00Z"
}
```

> **UPSERT 동작**: `(content_type, content_id, field_name, lang)` 조합이 이미 존재하면 `translated_text`와 `updated_at`을 갱신한다. `status`는 `translated_text`가 실제로 변경된 경우에만 `draft`로 리셋하며, 동일한 텍스트를 다시 제출하면 기존 `status`를 유지한다.

---

#### 9-3 : `POST /admin/translations/bulk` (번역 벌크 생성)

**요청**
```json
{
  "translations": [
    { "content_type": "lesson", "content_id": 42, "field_name": "lesson_title", "lang": "en", "translated_text": "Introduction to Korean Alphabet" },
    { "content_type": "lesson", "content_id": 42, "field_name": "lesson_description", "lang": "en", "translated_text": "Learn Hangul basics" },
    { "content_type": "lesson", "content_id": 42, "field_name": "lesson_title", "lang": "ja", "translated_text": "韓国語アルファベット入門" }
  ]
}
```

**응답 (부분 성공 207 / 전체 성공 201)**
```json
{
  "results": [
    { "index": 0, "status": "created", "translation_id": 1 },
    { "index": 1, "status": "created", "translation_id": 2 },
    { "index": 2, "status": "error", "error": "Invalid content_id" }
  ],
  "total": 3,
  "success": 2,
  "failed": 1
}
```

---

#### 9-5 : `PATCH /admin/translations/{id}` (번역 수정)

**요청**
```json
{
  "translated_text": "Introduction to the Korean Alphabet (Hangul)",
  "status": "reviewed"
}
```

**응답 (성공 200)**: TranslationRes 전체 반환

---

#### 9-6 : `PATCH /admin/translations/{id}/status` (번역 상태만 변경)

**요청**
```json
{
  "status": "approved"
}
```

**응답 (성공 200)**: TranslationRes 전체 반환

> **상태 전이 규칙**: `draft → reviewed → approved` 순서만 허용. 검수 반려 시 `reviewed → draft` 또는 `approved → draft`로 되돌림 가능.

---

#### ~~9-8 : `POST /admin/translations/auto` (자동 번역)~~ — 삭제됨 (2026-03-24)

> Google Cloud Translation API 해지로 인해 삭제됨. 번역은 Claude Code에서 직접 수행.

---

#### 9-9 : `GET /admin/translations/content-records` (콘텐츠 목록 조회)

> content_type별로 번역 가능한 레코드 목록을 반환한다. 관리자가 번역 대상 콘텐츠를 드롭다운에서 선택할 때 사용.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `content_type` | string | ✅ | 콘텐츠 유형 (video, lesson, study, study_task_choice, study_task_typing, study_task_voice, study_task_explain, course, study_task_writing). **주의**: `video_tag` 는 직접 선택하지 않음 (Video 선택 시 연결된 tag 가 §9-10 source-fields 응답에 함께 포함됨) |

**응답 (성공 200)**
```json
{
  "items": [
    { "id": 1, "label": "VID-001", "detail": "발음 기초" },
    { "id": 2, "label": "VID-002", "detail": "문법 기초" }
  ]
}
```

---

#### 9-10 : `GET /admin/translations/source-fields` (원본 텍스트 조회)

> content_type + content_id로 해당 레코드의 번역 가능 필드와 한국어 원본 텍스트를 반환한다. Video 선택 시 연결된 video_tag 필드도 함께 반환.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `content_type` | string | ✅ | 콘텐츠 유형 |
| `content_id` | i64 | ✅ | 콘텐츠 ID |

**응답 (성공 200)**
```json
{
  "fields": [
    { "content_type": "video", "content_id": 1, "field_name": "video_idx", "source_text": "VID-001" },
    { "content_type": "video_tag", "content_id": 10, "field_name": "video_tag_title", "source_text": "발음 연습" }
  ]
}
```

---

#### ~~9-11 : `POST /admin/translations/auto-bulk` (벌크 자동 번역)~~ — 삭제됨 (2026-03-24)

> Google Cloud Translation API 해지로 인해 삭제됨.

---

#### 9-12 : `GET /admin/translations/search` (번역 검색)

> 최근 approved/reviewed 상태의 번역을 조회한다. 언어별 필터 가능.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `lang` | string | N | 언어 코드 필터 (없으면 전체 언어) |

**응답 (성공 200)**
```json
{
  "items": [
    { "translation_id": 42, "content_type": "video", "content_id": 1, "field_name": "video_idx", "lang": "en", "translated_text": "VID-001", "status": "approved" }
  ]
}
```

---

#### 9-13 : `GET /admin/translations/stats` (번역 통계 조회)

> 번역 현황 통계를 반환한다. 콘텐츠 타입별, 언어별, 상태별 번역 커버리지 등을 확인할 수 있다.

**응답 (성공 200)**
```json
{
  "items": [
    { "content_type": "lesson", "lang": "en", "status": "approved", "count": 1 },
    { "content_type": "lesson", "lang": "en", "status": "draft", "count": 3 },
    { "content_type": "video_tag", "lang": "en", "status": "draft", "count": 3 }
  ],
  "total_translations": 8
}
```

- **실패(미인증)**: **401**
- **실패(권한 없음)**: **403**

---

#### 기존 콘텐츠 API `?lang=` 쿼리 파라미터 확장 (🟡 부분 구현, 편차 있음)

> **2026-04-21 정합 조사 결과** — 기존 "⬜ 미구현" 표기는 사실과 불일치. Consumer service 4 도메인 중 6 엔드포인트가 이미 번역 주입 로직 보유. 단, (1) 스펙과 다른 "덮어쓰기" 방식으로 구현됐고, (2) `field_name` 불일치 잠복 버그로 실제 번역이 반환되지 않는 상태였음. 상세는 [plans/translation-field-name-alignment.md](../../.claude/plans/translation-field-name-alignment.md) 참조.

##### 🟢 이미 구현된 부분 (덮어쓰기 방식, Q1a+Q1b 완료 2026-04-21)

| 엔드포인트 | 번역 주입 필드 (`field_name`) | 코드 위치 |
|-----------|-------------------------------|-----------|
| `GET /courses` | `course_title`, `course_subtitle` | [src/api/course/service.rs](../src/api/course/service.rs) |
| `GET /courses/{id}` | `course_title`, `course_subtitle` | 동일 |
| `GET /lessons` | `lesson_title`, `lesson_description` | [src/api/lesson/service.rs](../src/api/lesson/service.rs) |
| `GET /lessons/{id}` | `lesson_title`, `lesson_description` | 동일 |
| `GET /videos` (목록) | `video_title`, `video_subtitle` | [src/api/video/service.rs](../src/api/video/service.rs) |
| `GET /videos/{id}` **Q1b** | `video_title`, `video_subtitle` | 동일. VideoDetailRes 에 `title`/`subtitle` 필드 신규 추가 (video_tag_title MAX 집계) |
| `GET /studies` | `study_title`, `study_subtitle` | [src/api/study/service.rs](../src/api/study/service.rs) |
| `GET /studies/{id}` | `study_title`, `study_subtitle` | 동일 |
| `GET /studies/tasks/{id}` **Q1b** | task_kind 별 분기 — choice: `study_task_choice_question`, `study_task_choice_1~4` / typing: `study_task_typing_question` / voice: `study_task_voice_question` / writing: `study_task_writing_prompt`, `study_task_writing_answer`, `study_task_writing_hint` | 동일 |
| `GET /studies/tasks/{id}/explain` **Q1b** | `explain_title`, `explain_text` | 동일 |

##### 🟢 Q1a 잠복 버그 해소 완료 (field_name 규약 불일치 수정, 2026-04-21)

**수정 전**: Consumer service 4곳이 짧은 이름 (`"title"`, `"description"`) 으로 조회. 프로덕션 `content_translations` 실 데이터는 긴 이름 (`lesson_title` 등) 로 저장되어 있어 `?lang=en` 호출 시 절대 반환되지 않음. 2026-04-19 #76 (`study_task_explain`) 와 동종 패턴.

**수정 내용**:
- Consumer service 4곳 `field_name` 조회 키를 긴 이름 (`{table}_{column}`) 으로 치환 — course/lesson/study/video service.rs
- admin `find_content_records` 에 Course, StudyTaskWriting 매핑 추가
- admin `find_source_fields` 에 Course, StudyTaskWriting 매핑 추가. Video 에 `video_title`/`video_subtitle` 필드 노출 (단, `video` 테이블에 물리 컬럼 부재 → `source_text=None`. 관리자가 비디오 레벨 오버라이드 번역을 입력할 수 있도록 필드명만 노출)

**검증**: 마이그레이션 불필요 (실 데이터 8 row 전부 이미 긴 이름). `lesson_title = 'approved'` 1건이 정상 반환되기 시작 = 의도된 동작 복원.

##### 🟢 Q1b 미구현분 구현 완료 (2026-04-21)

- ✅ `GET /videos/{id}` — `?lang=` 파라미터 추가, VideoDetailRes 에 `title`/`subtitle` 필드 신규.
- ✅ `GET /studies/tasks/{id}` — `?lang=` 파라미터 추가. task kind 별 `ContentType::StudyTask*` 로 분기하여 payload 필드 오버라이드 (choice 5필드, typing 1필드, voice 1필드, writing 3필드).
- ✅ `GET /studies/tasks/{id}/explain` — `?lang=` 파라미터 추가. `study_task_explain` → `explain_title`/`explain_text` 오버라이드.

##### 🟢 Q1c 응답 스키마 최종 정렬 완료 (2026-04-21, 사용자 결정 3건 채택)

**결정 A — 덮어쓰기 유지 + 루트 메타 2필드 추가**:
- 10 Consumer 엔드포인트 응답 루트에 `translation_meta: TranslationMeta` 신규 필드.
- `TranslationMeta { translation_lang: Option<SupportedLanguage>, translation_coverage: TranslationCoverage }`.
- `TranslationCoverage` enum: `NotRequested` (`?lang=` 미요청) / `Full` (전 필드 user_lang) / `Partial` (일부 user_lang + 일부 fallback/none) / `None` (0 필드 번역).
- 코드: [src/api/admin/translation/dto.rs](../src/api/admin/translation/dto.rs) TranslationMeta + TranslationCoverage.

**결정 B — Video 테이블에 title/subtitle 물리 컬럼 추가**:
- 마이그레이션 `migrations/20260422_video_title_subtitle.sql`: `ALTER TABLE video ADD video_title VARCHAR(150) NOT NULL, ADD video_subtitle VARCHAR(250)`. 기존 `MAX(video_tag_title)`/`MAX(video_tag_subtitle)` 집계 결과로 백필 후 DEFAULT 제거.
- Repo `list_videos`/`get_video_detail` SQL: `MAX(video_tag_title) as title` → `v.video_title as title`.
- admin `find_source_fields` Video 의 `source_text=None` stub 제거 → 실 컬럼 매핑.
- admin Create/Update API 에 `video_title`/`video_subtitle` 입력 필드 추가 (backward-compat: 미제공 시 `video_tag_title`/`video_tag_subtitle` 폴백).

**결정 C — video_tag 번역 주입 (`VideoTagDetail.id` 노출)**:
- `VideoTagDetail` 구조체에 `id: i64` 필드 신규.
- Repo `get_video_detail` SQL 의 `jsonb_build_object` 에 `'id', vt.video_tag_id` 포함.
- Service `get_video_detail` 에서 tags[] 의 `id` 수집 → `content_type=VideoTag content_id IN (ids)` 로 번역 일괄 조회 → `video_tag_title`/`video_tag_subtitle` 오버라이드.
- `GET /videos` (목록) 의 `tags: Vec<String>` (tag_key 만) 은 그대로 유지 — 목록에선 분류 키만 사용.

##### Fallback 동작 (현재 구현 기준)

1. 요청된 `lang`의 `approved` 번역이 존재하면 → 번역된 텍스트 반환 (`translation_coverage=full` 또는 `partial`)
2. 요청된 `lang`의 번역이 없으면 → `en` (영어) `approved` 번역 시도 (`translation_coverage=partial`)
3. `en` 번역도 없으면 → `ko` (한국어 원본) 반환 (`translation_coverage=none` 또는 `partial`)
4. `lang=ko` 요청 시 번역 조회 스킵 (원본 반환, `translation_coverage=full` — 원본이 곧 번역)
5. `?lang=` 미요청 시 (`translation_coverage=not_requested`)


##### 확장 병목

1. **번역 품질** — AI 번역 70-80% 정확도, 네이티브 검수 필수 (특히 문법 설명 텍스트)
2. **RTL 테스트** — 아랍어 추가 시 전체 UI 양방향 테스트 필요 (현재 LTR 전용)
3. **콘텐츠 규모** — 비디오 100개 × 36 언어 × 3 필드 = 10,800+ 레코드 관리
4. **번역 입력 경로** (2026-04-21) — Google Translate API 해지 후 **서버 사이드 번역 스크립트** (amazing-korean-books 또는 amazing-korean-ai) 를 통한 bulk 생성으로 전환 예정. admin 웹 UI 는 검수(draft → reviewed → approved) 전용.

</details>

---

### 5.10 Phase 10 — explanation (해설 콘텐츠) 🟡 설계 중

> 출처: books→api 인계 (2026-05-17). 계약 문서 = `amazing-korean-books/docs/guide/explanation_handoff_to_api.md` + `explanation_content_model.md`. 전달물 = `scripts/guide-v2/data/explanation_export.json` (568 Unit = pattern_guide 68 + sentence_explain 500, 1,317 block).

**배경**: 해설집을 온라인 교육 콘텐츠로 정립. books가 api-무관 중립 모델(Unit/Block/LocalizedText)로 정리 완료. 연습문제(인터랙티브)는 별도 트랙(재구조화 B) — 범위 밖.

#### 결정된 사항 (2026-05-17)

| # | 결정 | 근거 |
|---|------|------|
| D1 | `study_explain` 재사용 ❌ | task·lang당 1행, `explain_title varchar(120)`, 블록 구조 없음 — 독립 교육 본문 불가 ([migrations/20260208_AMK_V1.sql:313](../migrations/20260208_AMK_V1.sql)) |
| D2 | `lesson_item kind=explanation` ❌ (시기상조) | `lesson_item_kind_enum`=video\|task, lesson_item은 video_id/study_task_id 2컬럼. 568 Unit은 study/task 연결이지 lesson 시퀀스 종속 아님 → 불필요 결합 |
| D3 | **전용 테이블 신설** ✅ `explanation_unit` + `explanation_block` | Unit/Block 모델 1:1 매핑. block의 structured rows/table/diagram = JSONB(과정규화 금지) |
| D4 | **서버 저장 + API 서빙** ✅ (정적 에셋 ❌) | 번역(5,117키×35언어)이 이미 content_translations DB 파이프라인 + status 워크플로 + 서버사이드 user_lang 오버레이([study/service.rs:88](../src/api/study/service.rs)). 콘텐츠만 정적화하면 번역과 소스 분리 = 두 출처·캐시 불일치. study_access 접근 제어 일관성도 서버 필요 |
| — | 연결키 | `study.study_idx`(pattern_guide 68) / `study_task.study_task_idx`=amk500-sent-NNN(sentence_explain 500, 2026-04-18 [migrations/20260418_study_task_idx.sql](../migrations/20260418_study_task_idx.sql)에서 해설집 시딩 목적 명시 도입) → hard FK 대신 논리 참조 + 시드 후 정합 검증 |

#### i18n 조인 결정 = **B 확정** (2026-05-17)

books `i18n_key`(평문 네임스페이스)와 api `content_translations` 튜플 조인키 임피던스 불일치 → **B 채택**: `content_translations` **무변경**. 설명 번역도 기존 10종과 동일하게 `(content_type, content_id, field_name, lang)` 튜플로 적재. 변환은 books 시드 생성기 + 로더가 담당.

- A(공유 표에 i18n_key 컬럼 추가) 기각 사유: 운영 위험은 낮으나 한 표에 식별 방식 2개 공존 = 전 학습 도메인 인지/유지보수 부담. EAV 약점(고아 번역)은 저심각도(위생, 비악화) → 이번 인계와 분리한 별도 백로그 저순위 항목으로만 기록(지금 작업 아님).

#### 확정 스키마 (마이그레이션 적용)

- 마이그레이션: `20260517_explanation_content_type_values.sql` (content_type_enum += explanation_unit/explanation_block, 단독) + `20260518_explanation_tables.sql` (3 enum + 2 table)
- src/types.rs: `ContentType` += ExplanationUnit/ExplanationBlock, 신규 `ExplanationUnitKind`/`ExplanationSource`/`ExplanationBlockType`

**`explanation_unit`** — `explanation_unit_id PK` / `unit_idx`(UNIQUE, books unit_id=재시딩 멱등키) / `unit_seq`(순서) / `unit_kind` / `unit_source` / 연결키 `study_idx`·`study_task_idx`·`sentence_num`·`section_id`(논리 참조, FK 아님) + `link_meta jsonb` / title·subtitle = `*_ko`·`*_en`·`*_lang_invariant` **평면화(방식 ㉠)** (ko nullable).

**`explanation_block`** — `explanation_block_id PK` / `explanation_unit_id`(hard FK, ON DELETE CASCADE) / `block_seq` / `block_type` / `block_level` / `text_ko`·`text_en`·`text_lang_invariant` / `raw`(lang-invariant 원형) / `structured jsonb`(lang-invariant 골격) / UNIQUE(unit_id, block_seq).

**연결키 = 논리 참조** (강제 안 함): tense_v1/josa_v1 무링크, av_307_313 갭, 시딩 순서 독립 → FK 대신 시드 후 정합 검증 쿼리.

**structured 경계**: 번역 안 타는 골격(role/form, concept_card 메타, qword_card 표 구조) = `structured` JSONB 통째(index 보존) / 번역 타는 텍스트(row en·explanation, header, note, **concept_card desc, qword_card header**) = content_translations 행 분리. row/item/header index ↔ field_name `{i}` 불변식. lang-invariant는 번역 파이프라인 미진입. inherit row = explanation만 미진입(en은 진입).

#### 적재 로더 (2026-05-17) — `src/bin/seed_explanation.rs`

Rust 시드 바이너리 (선례 = `rekey_encryption`). `cargo run --bin seed_explanation -- --input <explanation_seed.json>` (또는 env `EXPLANATION_SEED_PATH`). 단일 트랜잭션 멱등 적재:

1. `explanation_unit` upsert `ON CONFLICT (unit_idx)` → PK 맵
2. `explanation_block` upsert `ON CONFLICT (explanation_unit_id, block_seq)` → PK 맵
3. 산출 B → `content_translations` upsert `ON CONFLICT (content_type, content_id, field_name, lang)`. `unit_idx`(+`block_seq`)로 PK 해소. **`lang='en'` + `status='approved'`** (en=권위 텍스트, 서빙 필터 `status='approved'` 통과 위해. 맥미니 35언어는 별도 `upsert_one` 경로 draft→review). 미해소 시 fail-loud.
4. 연결키 정합 검증 **내장** (작업 #2 흡수): `study_idx`→study / `study_task_idx`→study_task 미해소 count 리포트 (논리 참조 = 경고만, 시드 순서 독립).

**스키마 정정 (마이그 20260518)**: `explanation_unit.updated_by_user_id` `NOT NULL` → **nullable + FK→users(user_id)** = lesson/study 컨벤션 일치(시스템 시드 콘텐츠 = NULL updater). 마이그 미적용·미머지(KKRYOUN) 상태라 정정 안전.

> **실행 환경 주의**: 본 세션은 DB 미접속 — `cargo check`/`clippy`/`fmt` 정적 검증만 통과. 실제 시드 실행 + 연결키 검증 수치는 DB 환경(로컬/배포) 실행 시점.

#### 조회 API (2026-05-17) — `src/api/explanation/`

신규 도메인 `explanation` (dto→repo→service→handler→router, `/explanations` nest). **공개 읽기**(접근 제어 컬럼 없음 — D3 설계).

- `GET /explanations/{unit_idx}?lang=` → 단위 + 블록 (unit_idx 예: `guide67:pr_105_114`, `sent:300`)
- `GET /explanations?study_idx=&study_task_idx=&lang=` → 연결키 조회 (둘 다 없으면 400)

**서빙 모델** = structured **골격 + i18n 해소 맵**. 단순 텍스트 블록(paragraph/heading/subtitle/step) = `text` 해소. structured/concept/qword = `structured`(골격 그대로) + `i18n`(field_name→해소 텍스트). 프론트가 §index 불변식으로 재조립.

- **폴백 체인**: 요청 lang → tr(user/en) → en → ko 중 첫 비어있지 않은 값. ko/none = ko원본 우선(없으면 en). en = en 우선. 기타 = tr(user|en) 우선.
- **inherit 계승**: structured_explain `rows[i].inherit=true` → 직전 비-inherit row 의 `explanation_block_row_{j}_explanation` 을 i18n 에 채움 (서버 해소).
- **번역 조회**: explanation 전용 `find_translations` (admin 공유 코드 무수정). `content_translations` `lang IN (user, 'en') AND status='approved'`, ko 단락 회피(설명 structured 는 ko 원본 없음 → en 폴백). jsonb 는 `::text` 캐스트 fetch 후 파싱 (sqlx json feature 미사용).
- OpenAPI: docs.rs paths/schemas/tags 등록. 회귀 테스트 `openapi_paths_match_router_handlers` 포함 7/7 통과.

#### 로컬 DB 런타임 검증 (2026-05-17)

로컬 dev DB(amk-pg)에 우리 마이그(20260517/20260518)만 직접 적용 → `seed_explanation` 실행:

- 적재: unit 568 / block 1317 / translation 4362, **멱등 재실행 동일**(ON CONFLICT), content_translations 전부 `lang=en status=approved` ✓
- repo SQL 실측(psql): `find_unit_by_idx`(enum ::text 캐스트) / `find_translations`(en, sent:300 = header·row_0_en·row_0_explanation·row_1_en·row_2_en) / `find_units_by_link`(study_task_idx=amk500-sent-300 → sent:300, OR/NULL 가드 동작) / 갭1 `explanation_block_card_{i}_desc` end-to-end 적재 ✓
- 연결키 정합: study_idx 566 / study_task_idx 500 미해소 = **정상**(로컬 study 시드 없음, 논리 참조 경고)

> **검증 한계 (정직)**: 라이브 HTTP 경로(service.rs i18n 맵 조립·inherit 계승·폴백)는 미검증 — 서버 부팅이 **dev DB 사전 이력 분기(`20260419` 체크섬, 우리 코드 무관)**에 막힘(`sqlx::migrate!` 부팅 차단, `sqlx migrate run`도 동일). 강제 우회는 feedback_migration_safety 상 미실시. 해당 in-memory 변환은 compile/clippy/코드리뷰 clean + 입력 데이터 실측 정확 = 결정적 동작이나 실 HTTP 응답 미확인. **프로덕션/정상 마이그 환경에서 재검증 필요.**

#### 번역 트랙 적재 계약 (2026-05-17 확정, 구현 대기)

맥미니 Phase C 35언어 산출 → api 적재 경로. books 후속 확인 회답으로 확정 (SSoT = `explanation_seed_contract_from_api.md` §📬 회답):

- **파일** = lang별 분리 `explanation_translations.{lang}.json`, 행 = en 산출 B 와 동일 5-튜플 `(unit_idx, block_seq|null, field_name, lang, translated_text)`. field_name 집합 = en 과 동일(inherit row 도 en 패턴 미러).
- **적재** = `seed_explanation` **신규 `--translations <path>` 모드** (별도 바이너리 X). 산출 A 스킵 + `unit_idx`(+`block_seq`)→PK **DB 조회** 해소 + content_translations upsert. **구현 시점 = 맥미니 산출 도착 시** (현재 계약만, YAGNI).
- **멱등 키** = `(content_type, content_id, field_name, lang)` 튜플 유지 (lang 파일 재실행 = 해당 lang 만 upsert).
- **status = `approved`** (맥미니 검증 4종 통과분, en 일관, 서빙 필터 통과 — 5,117×35 수동검수 비현실).
- 시드 재생성 통지 프로토콜: 트리거=원문 변경 시만, 통지 주체=books(+구조 변경 포함 여부 명시 요청 → api full vs `--translations` 분기), 평시 무동작.

#### 시드 검증 결과 (2026-05-17) — books `explanation_seed.json` PASS·채택

api 독립 전수 검증(meta.self_check 비신뢰, 직접 재계산): 산출 A unit 568(pattern_guide 68+sentence_explain 500)/block 1,317 + 산출 B en 전용 4,362행. unit_idx·(unit_idx,block_seq) UNIQUE / enum·av_307_313·연습누출·lang-invariant누출·PK해소·study_task_idx(amk500-sent-NNN 500/500)·field_name 9종(갭1 포함) 전 항목 ✅. **계약 정정 1건(api 귀책)**: §2 inherit 문구 모호(row 전체 vs explanation 한정) → explanation 한정으로 정정, books 시드 정상·재작업 불필요. **다음**: 적재 로더(산출 A→PK 확정→산출 B PK 해소→content_translations) + 연결키 정합 검증 + 조회 API.

#### B 시드 계약 (api ↔ books)

1. 시드 순서: ① api가 explanation_unit/block 시딩(PK 확정) → ② 로더가 `unit_idx`+`block_seq`로 PK 해소해 content_translations 적재.
2. 결정적 field_name 규약(varchar(100) 내): unit `explanation_unit_title`/`explanation_unit_subtitle` / block `explanation_block_text` / structured `explanation_block_row_{i}_explanation`·`_en`·`explanation_block_header`·`_note` / **concept_card `explanation_block_card_{i}_desc` / qword_card `explanation_block_qword_{i}_header`** (2026-05-17 books 회신 갭1 → (a) 채택, structured 경계 동일 규칙 확장).
3. 번역 대상 한정: `lang_invariant != true` 인 LocalizedText만 content_translations 행 생성. **inherit row (`inherit:true`)** = structured jsonb 슬롯 유지(index 보존). inherit 범위 = **explanation 한정** → `_explanation` 행 없음(렌더 시 직전 비-inherit row의 explanation 계승), 단 `_en` 토큰은 실 콘텐츠라 `_en` 행 **정상 생성·번역** (확인1 정정).
4. books 산출 형태: `(unit_idx, block_seq|null, field_name, lang, text)` → 로더가 idx→id 해소. **산출 B = `lang='en'` 행만**(en=권위). ko는 산출 A `text_ko`/`title_ko`(도메인 테이블)가 원본 → 서빙 시 `lang=ko`=원본 반환(content_translations 미조회, 기존 study 패턴). 35언어=맥미니 Phase C 후속 append. self-check §4-1=en 기준 (확인2).

#### 점검 결과 (2026-05-17, 커밋 `5897cc8` 전수 대조)

계획 대비 구현 = **누락·이탈 0** (B 무변경 / 전용 2테이블 / enum 정합 / 평면화㉠ / 논리참조FK 없음 / CASCADE / 순서·멱등 / structured JSONB / cargo check / 마이그 네이밍 정책). books Block·Unit 모델 필드 커버리지 전수 대조 완료.

**정직 고지 (오류 아님 — 후속 작업 시 감안할 의도적 선택)**:

1. `20260518_*`은 오늘(05-17) 기준 미래 날짜 파일명 — `migrations/README.md §1` "같은 날 다중 = 다음 날짜" 명시 관례 준수 (날짜=정렬 서수).
2. `title_en`/`title_ko` **둘 다 nullable** (계획은 ko만 합의). export 샘플상 subtitle.en/ko null 케이스 존재 → 과제약 회피 의도. **권위 텍스트 en NOT NULL 강제 안 함** (사용자 확인 후 현행 유지).
3. `study_task_idx`는 books `link` 객체에 직접 없는 파생값 — handoff §3 기준 books 시드 생성기가 `amk500-sent-NNN` emit (api 임의 생성 아님).
4. enum 값 DB↔Rust는 **정적 정합만 확인** — 런타임 실 DB 대조는 시드/통합 시점 (Guide67 명시 rename + snake_case 매핑 코드상 일치).
5. i18n_key 테이블 미저장 = B 의도 (미사용 컬럼 회피). 런타임 조인키 = 튜플.

#### 알려진 제약 (books handoff §4)

- guide_67 `ko` = 원본 HTML strip 최선치(없으면 null), **권위 텍스트 = en + i18n_key** → 스키마는 ko nullable 허용
- `av_307_313`(부사어 307~313) 제외 = v9.77 미마이그레이션 legacy (68 = 67 − av_307_313)
- `sentence_explain.title` = 해설 대상 예문(참조용 컨텍스트, 연습 누출 아님)
- `lang_invariant:true`(한국어 활용형·표·예문) = 번역 안 함, 전 언어 동일 노출

#### 다음 흐름

api 스키마 확정(미결정 i18n 1건 해소 후) → books가 그 형식으로 시드 생성기 변환 추가 → 맥미니 번역 i18n_key 기준 content_translations 적재.

---

### 5.11 Phase 11 — study / study_task 콘텐츠 시딩 트랙 🟢 prod 적용·검증 완료 / 공개 flip 대기 (2026-05-18)

> **prod 적용 완료 (2026-05-18, main `e9568e7` 배포 후 수동 1회)**: `seed_hymn_account` → HYMN `user_id=8` state=false/pw=NULL. studies `INSERT 0 67` → `study=67 ready=67 open=0`. tasks → `study_task=500 typing=500` / `study_task_typing=500` / task→study FK 500 OK. 라이브 `/studies` 노출 **0**(전부 `ready` → 서빙 `study_state='open'` 필터가 차단) = 숨김 staged rollout prod 강제 확인. 로컬 dry-run 결과와 완전 일치. **남은 것 = 공개 flip(검증+Mac Mini 번역 후 사용자 결정 게이트) / Mac Mini content_translations / choice·voice·writing·어휘 books 확장 / lesson·course — 전부 별건·외부·미래.**


> 보안 §4 종결 후 사용자 명시 차기 트랙. explanation(§5.10, prod 완료) 외 학습 본문 콘텐츠 시딩. SSoT 메모리 = `project_content_seeding_track`.

**스코프**: study / study_task **텍스트 콘텐츠**. video 제외(TTS·자막 = Phase 16 별건, 사용자 결정). lesson/course = study/task 정착 후 후속.

**소스 (실재 — 별도 생성 불필요)**: `amazing-korean-books/scripts/guide-v2/seed_output/`
- `20260504_seed_textbook_studies.sql` — study 행
- `20260505_seed_textbook_tasks.sql` (448KB) — study_task 행
- `20260506_seed_textbook_translations.sql` — **무시** (번역=Mac Mini 정본, §5.9 정정)
- books `놀라운 한국어 500 해설집_완성/*.html`(76개) = 사람용 렌더링 뷰(어휘·플래시카드·연습 인터랙티브 포함). **실제 구조 데이터 SSoT = `scripts/guide-v2/` 파이프라인** (HTML 직접 파싱 아님).

**번역**: 본문(ko 원문)은 study/study_task 테이블 컬럼에 직접 저장. 다국어 overlay = 본문 시드 후 Mac Mini 후속 일괄(explanation 선례 동형). books `_translations.sql` 정본 아님.

**검증된 패턴 (explanation 선례, §5.10)**: api 스키마 확정 → books export(계약 `amazing-korean-books/docs/guide/explanation_seed_contract_from_api.md`) → `JSON + src/bin/seed_*.rs 멱등 로더 + 연결키/FK 검증`. study/task 도 동형 계약 필요할 수 있음.

**api 인프라 상태**: study/study_task/course = 스키마·서빙 API·i18n 폴백 완성(§5.5/§5.8/§5.9). lesson = i18n 폴백 미연결 가능성(소규모, 구현 시 실측). study/task 테이블 = 테스트 데이터(~2/19건)만, 본 콘텐츠 비어있음.

**구현 완료 (2026-05-18, 커밋 `eff6994`)** — 결정: HYMN=귀속전용 / 숨김 시딩 / 기존 books 파일(20260504/05, 바이트동일·스키마 수동검증) + 로컬 dry-run / 번역=Mac Mini.

- **HYMN 선행 계정**: `src/bin/seed_hymn_account.rs` (신규). raw SQL 불가 — `user_email`/`user_name`/`user_birthday` = 앱 KeyRing AES-256-GCM + HMAC blind index 컬럼 → 앱 `CryptoService`(AAD `users.user_email` 등 read 경로 일치)로 암호화. `user_state=false`·`user_password=NULL`(로그인 차단, FK 귀속 전용). 멱등(`user_auth='HYMN'` skip + `ON CONFLICT(user_email_idx) DO NOTHING`). 신원 `system@amazingkorean.net`/`HYMN`/KR. Dockerfile 3곳(더미+touch+런타임 COPY).
- **시드 파일**: `seeds/20260518_seed_textbook_studies.sql`(books 20260504 복사, **유일 차이 `study_state` open→ready 67행** = 숨김 시딩) + `seeds/20260518_seed_textbook_tasks.sql`(books 20260505 무변경, study_task 500 typing + study_task_typing 500). `seeds/` 는 Dockerfile `COPY seeds`로 이미지 반입.
- **숨김 근거**: 서빙 전 경로 `study_state='open'` 필터(`src/api/study/repo.rs:169/216/290/442` 등) → `ready`=전 사용자 비노출. `study_task` 가시성=부모 study JOIN(290/442)→자동 숨김. studies UPSERT `DO UPDATE SET`=title/subtitle/desc 만(study_state 미포함) → 후속 flip 한 `open` 을 재시딩이 안 되돌림(footgun 없음).
- **검증**: cargo check/clippy/fmt + 로컬 전체체인 dry-run(현 마이그 30 → hymn → studies → tasks): HYMN state=false/pw=NULL·멱등skip / study 67 전부 ready / task 500 typing / FK·updated_by=HYMN 전부 / 멱등 재시딩 불변 / flip 보존 전부 PASS. (dry-run 이 로컬 .env 빈 crypto 키 적발 — 버그 아님, prod 실 키.)

**prod 적용 런북 (머지·배포 후, 수동 1회, 멱등)**:
```bash
# 1) HYMN 선행 (없으면 books SQL abort)
docker exec amk-api /app/seed_hymn_account
docker exec -i amk-pg psql -U postgres -d amazing_korean_db -c \
  "SELECT user_id,user_auth,user_state FROM users WHERE user_auth='HYMN';"   # 1행, state=f
# 2) studies → tasks (이미지 내 seeds → db 컨테이너로 파이프, 멱등)
docker exec amk-api cat /app/seeds/20260518_seed_textbook_studies.sql | \
  docker exec -i amk-pg psql -U postgres -d amazing_korean_db -v ON_ERROR_STOP=1
docker exec amk-api cat /app/seeds/20260518_seed_textbook_tasks.sql | \
  docker exec -i amk-pg psql -U postgres -d amazing_korean_db -v ON_ERROR_STOP=1
# 3) 검증: study 67(전부 ready) / study_task 500 / typing 500 / study_task_idx ↔ live explanation
```
- **공개 flip (검증 + Mac Mini 번역 도착 후, 별도 결정)**: `UPDATE study SET study_state='open' WHERE study_idx LIKE 'amk500-%';`
- **후속(별건)**: Mac Mini content_translations overlay / choice·voice·writing·어휘 books 파이프라인 확장 / lesson·course.

---

[⬆️ AMK_API_MASTER.md로 돌아가기](./AMK_API_MASTER.md)
