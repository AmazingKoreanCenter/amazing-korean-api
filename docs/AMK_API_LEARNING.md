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
  - When: 상세 진입, 존재하는 영상 id
  - Then: **200**, 본문에 메타(제목, 설명, 길이, `video_url_vimeo`, **VIDEO_TAG 배열**)
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
| 5-11 | `GET /studies/writing/practice` | `/studies/writing/:level/:type` | 자유 연습 시드 컨텐츠 | ***level+practice_type 필터, seq 오름차순, 기본 20 / 최대 100, 비인증 허용. `writing_practice_seed` 테이블에서 prompt/answer/hint 반환*** | [✅🆗] |

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
  - Then: **200**, 문제 본문/보기/메타(난이도/분류) → **STUDY_TASK_LOG** `view` 업데이트
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
    4. study_task_writing : 한글 자판 타이핑 → **STUDY_TASK_LOG** `start` 업데이트 → 제출 → **STUDY_TASK_LOG** `answer` 업데이트 (세션 단위 통계는 P4 `writing_practice_session` API로 별도 집계)
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
  - Then: **200**,`{ explain_title, explain_text, explain_media_url }` → **STUDY_TASK_LOG** `explain` 업데이트
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
  - `writing_practice_seed` 테이블에서 `(level, practice_type)` 필터로 `seq` 오름차순 조회
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
- **번역 방식**: Claude Code에서 직접 번역 수행 (관리자 검수 → 승인)

**지원 언어 (21개, 아랍어 RTL 별도)**

| 그룹 | 언어 코드 |
|------|-----------|
| 핵심 5개 (Phase 2) | `en`, `ja`, `zh-CN`, `zh-TW`, `vi` |
| 동남아시아 | `id`, `th`, `my`, `km` |
| 중앙/북아시아 | `mn`, `ru`, `uz`, `kk`, `tg` |
| 남아시아 | `ne`, `si`, `hi` |
| 유럽/기타 | `es`, `pt`, `fr`, `de` |

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
      "field_name": "title",
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
  "field_name": "title",
  "lang": "en",
  "translated_text": "Introduction to Korean Alphabet"
}
```

**응답 (성공 201)**
```json
{
  "translation_id": 1,
  "content_type": "lesson",
  "content_id": 42,
  "field_name": "title",
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
    { "content_type": "lesson", "content_id": 42, "field_name": "title", "lang": "en", "translated_text": "Introduction to Korean Alphabet" },
    { "content_type": "lesson", "content_id": 42, "field_name": "description", "lang": "en", "translated_text": "Learn Hangul basics" },
    { "content_type": "lesson", "content_id": 42, "field_name": "title", "lang": "ja", "translated_text": "韓国語アルファベット入門" }
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
| `content_type` | string | ✅ | 콘텐츠 유형 (video, lesson, study, study_task_choice, study_task_typing, study_task_voice, study_task_writing, study_task_explain) |

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
  "stats": { ... }
}
```

- **실패(미인증)**: **401**
- **실패(권한 없음)**: **403**

---

#### 기존 콘텐츠 API `?lang=` 쿼리 파라미터 확장 (⬜ 미구현)

> 모든 기존 콘텐츠 조회 API(lessons, courses, studies, videos)에 `?lang=` 쿼리 파라미터가 추가된다.
> **현재 상태**: 스펙 확정, 미구현. 관리자 번역 CRUD(9-1~9-12)는 완료되었으나, Consumer API에서 번역을 소비하는 `?lang=` 파라미터와 `_translated` 접미사 필드는 아직 구현되지 않음.

| 기존 엔드포인트 | 확장 예시 | 동작 |
|----------------|-----------|------|
| `GET /courses` | `GET /courses?lang=en` | 코스 목록에 영어 번역 포함 |
| `GET /courses/{id}` | `GET /courses/{id}?lang=ja` | 코스 상세에 일본어 번역 포함 |
| `GET /lessons/{id}` | `GET /lessons/{id}?lang=vi` | 레슨 상세에 베트남어 번역 포함 |
| `GET /studies/tasks/{id}` | `GET /studies/tasks/{id}?lang=zh-CN` | 학습 Task에 중국어(간체) 번역 포함 |

**Fallback 동작**:
1. 요청된 `lang`의 `approved` 번역이 존재하면 → 번역된 텍스트 반환
2. 요청된 `lang`의 번역이 없으면 → `en` (영어) `approved` 번역 시도
3. `en` 번역도 없으면 → `ko` (한국어 원본) 반환

**응답 확장 필드**: `?lang=` 지정 시 응답에 `_translated` 접미사 필드가 추가된다.
```json
{
  "lesson_id": 42,
  "lesson_title": "한글 소개",
  "lesson_title_translated": "Introduction to Korean Alphabet",
  "lesson_description": "한글 기초를 배워보세요",
  "lesson_description_translated": "Learn Hangul basics",
  "translation_lang": "en",
  "translation_coverage": { "title": true, "description": true }
}
```

**다국어 확장 병목 포인트**:
1. **번역 품질** — AI 번역 70-80% 정확도, 네이티브 검수 필수 (특히 문법 설명 텍스트)
2. **RTL 테스트** — 아랍어 추가 시 전체 UI 양방향 테스트 필요 (현재 LTR 전용)
3. **콘텐츠 규모** — 비디오 100개 × 21언어 × 3필드 = 6,300+ 레코드 관리

</details>

---

[⬆️ AMK_API_MASTER.md로 돌아가기](./AMK_API_MASTER.md)
