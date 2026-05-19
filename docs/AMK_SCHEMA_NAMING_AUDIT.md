# AMK 스키마 명명 SSoT 정리 — 전수 감사 (1a + 1b)

> **SSoT Core** = `AMK_API_MASTER.md §3.2.1` (테이블 = `<도메인(단수)>_<의미>...`, 도메인 접두사 필수, `users`만 PG 예약어 예외, admin = `ADMIN_<도메인(복수)>_..._LOG`, enum = `<도메인>_..._enum`).
>
> 본 문서 = 스키마 명명 위반 정리 트랙의 **단일 감사 SoT**. 2단계(정리+마이그) 설계서가 참조한다.
> 작업순서(사용자 확정): ①전수조사+사용처·데이터 파악(1a~1c) → ②정리+마이그 → ③study 테이블 역할조정.
> 진행: 1a 감사(2026-05-18, `7716d03`) ✅ / **1b 전수매핑 + 1c 문서화(2026-05-19) ✅** / 2~3 대기.
> 연계: STATUS #154, 메모리 `project_schema_naming_cleanup`.

---

## 1. 위반 분류 (49 테이블 전수 대비)

| 등급 | 테이블 | 위반 사유 |
|---|---|---|
| 🔴 | `explanation_unit` `explanation_block` | "explanation"=도메인 아님(해설집=study 영역) |
| 🔴 | `subscriptions` `transactions` `webhook_events` | payment 도메인 접두사 없음 + 복수형 (이중 위반) |
| 🔴 | `writing_practice_seed` `writing_practice_session` | "writing_practice"=등록 도메인 아님(study 영역), study_task_writing과 분리 |
| 🟡 | `user_oauth` `user_export_data` `user_course` | `users_`(users_log/users_setting) ↔ `user_`(이 3개) 접두사 혼용 |
| 🟠 | `content_translations` | 폴리모픽 i18n, 도메인 없음 — 규칙 예외 명문화 vs 개명 검토(2단계 결정) |
| 🟠 | `redis_*` | 일관 접두사라 허용 가능(검토 보류) |

**메타부채**: §3.2.1 규칙은 존재하나 신규 마이그 강제 게이트 없음 → 드리프트(explanation_* 가 증거). 2단계에 재발방지(규칙 위반 신규 테이블 차단 CI/스크립트) 포함 검토.

---

## 2. 그룹 1 — explanation 이중 해설 모델 (트랙 발단, 🔴)

이중모델 = 본 트랙의 본질. 통합(study_explain→study_contents 배선) **후** 리네임이 정공법(리네임 선행 시 churn).

### 2.1 데이터 (local dev DB 실측, prod = PR #298 568/1317·study_explain 0)

| 테이블 | local 행수 | prod |
|---|---|---|
| `explanation_unit` | 568 | 568 (PR #298, 2026-05-17) |
| `explanation_block` | 1317 | 1317 |
| `study_explain` | **0 (빈)** | **0 (빈)** |

### 2.2 코드 사용처 (file:line)

**`explanation_unit` / `explanation_block`** (신규 구조화, prod 적재 SoT):
- DDL: `migrations/20260518_explanation_tables.sql:22-56`(unit) / `:59-77`(block, FK unit ON DELETE CASCADE)
- Repo: `src/api/explanation/repo.rs:58`(find_unit_by_idx) `:67`(find_units_by_link by study_idx/study_task_idx) `:85`(find_blocks) `:104`(find_translations content_type)
- Service: `src/api/explanation/service.rs:17`(get_unit) `:24`(list_by_link) `:70-172`(build_unit + inherit/structured_explain 로직)
- Handler/Router: `src/api/explanation/handler.rs:29`(list) `:57`(get) / `router.rs:7-11` / `mod.rs:66`(nest)
- Seed: `src/bin/seed_explanation.rs:123`(unit upsert ON CONFLICT unit_idx) `:163`(block upsert) `:195`(content_translations insert) `:246`(orphan 검증)
- Types: `src/types.rs:200-216`(ContentType enum, content_type_enum) `:420-457`(UnitKind/Source/BlockType)
- 프론트: 직접 타입 없음(study task explain 경로로 흐름)

**`study_explain`** (레거시 평면, 이 콘텐츠 빈):
- DDL: `migrations/20260208_AMK_V1.sql:313-321` (study_task_id FK, explain_lang/title/text/media_url, 암묵 UNIQUE(task,lang))
- **학습자 경로(갭 핵심)**: `src/api/study/repo.rs:522 find_task_explain` → `FROM study_explain WHERE study_task_id=$1 AND explain_lang='ko' AND s.study_state='open'`. **빈 테이블 조회 → 학습자 해설 0건** (실측 확인 2026-05-19). 라우트 `study/router.rs:17 GET /studies/tasks/{id}/explain`
- Admin CRUD: `src/api/admin/study/repo.rs` 6함수(`:238`list `:410`exists `:432`create `:468`find `:496`find_tx `:524`update) / `handler.rs:306,465,507,548,592` / `service.rs:589,761,826,905,1033`(translation audit content_type="study_explain")
- Admin translation JOIN: `src/api/admin/translation/repo.rs:362`(JOIN study_explain)
- 프론트: `frontend/src/category/study/types.ts:214`(taskExplainResSchema) / `study/page/study_task_page.tsx:186`(ExplainCard, **빈 레거시 렌더**) / `study/study_api.ts:56`(getTaskExplain) / `admin/page/admin_study_detail.tsx`(admin CRUD 훅)

### 2.3 연계 enum (study_contents 리네임 시 동반 — INC-004급)

| enum | DDL | 참조 | 리스크 |
|---|---|---|---|
| `content_type_enum` 값 `explanation_unit`/`explanation_block` | `migrations/20260517_explanation_content_type_values.sql:10-11` | **content_translations 4362행** + `types.rs:214-215` + `explanation/repo.rs:104` + `seed_explanation.rs:195` | 🔴 이미 적용 enum 값 변경 = sqlx checksum crash 선례(INC-004). 데이터 마이그 동반 필수 |
| `explanation_unit_kind_enum` | `20260518:11` | `types.rs:420` `seed_explanation.rs:129` | 신규(prod 적재됨), 신중 |
| `explanation_source_enum` | `20260518:13` | `types.rs:429`(`#[sqlx(rename="guide_67")]`) `seed:129` | 동일 |
| `explanation_block_type_enum` | `20260518:15-18` | `types.rs:441` `service.rs:126`(structured_explain 분기) `seed:170` | 동일 |

### 2.4 확정 결정 (사용자 지시, 메모리)

- `explanation_unit`/`explanation_block` → **`study_contents`**(+block 테이블). 충돌 없음 실측(`study_description`=study.study_description 컬럼 충돌로 폐기됨).
- `study_explain` 흡수/폐기 + 학습/관리자 흐름 `study_contents` 배선 + content_type_enum 값(4362행) 데이터 동반 마이그.
- `video_`/`lesson_` 병렬 테이블 신설 = **YAGNI**(콘텐츠 없음).
- 타이밍 = 지금이 적기(프론트 미통합·공개 flip 전·의존 최소).

---

## 3. 그룹 2 — payment (무접두사+복수형, 🔴)

DDL 전부 `migrations/20260215_payment_system.sql`. 배선 규모 최대(8 웹훅 핸들러).

### 3.1 데이터 (local 실측)

`subscriptions`=10 / `transactions`=4 / `webhook_events`=212. (prod 행수 별도 확인 필요 — 2단계 착수 전.)

### 3.2 코드 사용처 + 권장명

| 테이블 | 권장명 | DDL | 인덱스 | 핵심 배선 |
|---|---|---|---|---|
| `subscriptions` | `payment_subscription` | `:34-52` | 3 + 복합 `idx_subscriptions_user_status`(`20260504:14`) | repo 5함수(`payment/repo.rs:98,126,153,179,218`) + service 8 웹훅 핸들러(`service.rs:311~526` created/activated/resumed/updated/canceled/paused/past_due/trialing) + admin(`admin/payment/repo.rs:40,132,281`) + 프론트 admin 3페이지 |
| `transactions` | `payment_transaction` | `:57-71` | 3(`idx_transactions_provider_txn_id`) | repo 2함수(`payment/repo.rs:246 create`,`:280 update_by_provider_id`) + `service.rs:545,679`(completed/refund) + admin list(`admin/payment/repo.rs:171,201`) + 프론트 + **tests/payment_integration.rs:903,1004,1080 raw SQL** |
| `webhook_events` | `payment_webhook_event` | `:76-84` | 1(`idx_webhook_events_type`) | 멱등 전용 `payment/repo.rs:309 is_processed`/`:400 record` + `service.rs:154,208,231,262`(Paddle+RevenueCat). 프론트 0 |

프론트: `frontend/src/category/admin/payment/{types.ts,hook/use_admin_payment.ts,page/*}` + `admin/admin_api.ts:705,711,716,722`. 라우트 `frontend/src/app/routes.tsx:237-240`.

⚠️ 기존 제약(본 트랙 무관, 기록만): `transactions`에 `updated_at` 컬럼 없음 — `payment/repo.rs:286` 주석 명시. 환불 처리 시 갱신시각 미기록.

---

## 4. 그룹 3 — writing_practice (study 영역, 🔴)

| 테이블 | local | DDL | 사용처 | 비고 |
|---|---|---|---|---|
| `writing_practice_seed` | 190 | `20260413_writing_practice_seed.sql:8-21`(+190 INSERT) | `study/repo.rs:975`→`service.rs:791`→`handler.rs:301`→`router.rs:30` **public `GET /studies/writing/practice`**. 프론트 `study/hook/use_writing_practice_seed.ts` | study 도메인 — 리네임 후보 `study_writing_seed`류(2단계 결정) |
| `writing_practice_session` | 25 | `20260412_writing_practice.sql:39-67`(FK user CASCADE, study_task SET NULL) | `study/repo.rs:630`(create) `:692`(finish) `:761`(count) `:788`(list) `:842/869/906/942`(stats 4종). 라우트 `/studies/writing/sessions` + `/stats`. 프론트 `study/hook/use_writing_session.ts`, `writing_stats_page.tsx`, e2e | reset 마이그 삭제순서 #8(task기반 한정, 자유연습 보존) |

전부 study 도메인 라우트 하위. 도메인 = study.

---

## 5. 그룹 4 — user_ vs users_ 혼용 (🟡)

| 테이블 | local | DDL | 사용처 | 판정 |
|---|---|---|---|---|
| `users` `users_log` `users_setting` | — | `20260208_AMK_V1.sql` | — | ✅ 기준(`users` PG 예약어 예외, `_log`/`_setting` 동반) |
| `user_oauth` | 3 | `20260208:83-94`(암호화 PII + blind index) | `auth/repo.rs:649,674,701,975` + `service.rs:2092,2111,2211`(AES-256-GCM AAD) + `bin/rekey_encryption.rs:79` | 🟡 `users_` 정합 검토 |
| `user_export_data` | **0** | `20260208:107-116` | **코드 참조 0건**(src/·frontend/ 전수 grep 빈 결과) | 🟡 + **dead table**. 리네임 자명 or YAGNI 폐기 — 2단계 결정 |
| `user_course` | 0(local) | `20260208:440-453`(UNIQUE user_id,course_id, FK 4) | `payment/repo.rs:343 grant_all`/`:364 revoke_all`/`:385 update_expiry` + `lesson/repo.rs:120 has_course_access` + `admin/payment/repo.rs:280 orphaned_grants` | 🟡 다중 도메인 의존(payment+lesson+admin). 리네임 영향면 큼 |

`users_*`=user 메타(log/setting) vs `user_*`=정션/auth — 암묵 구분이 §3.2.1에 명문화 안 됨 = 드리프트 근원. 2단계에서 규칙 명문화 or 통일 결정.

---

## 6. 부수 발견 (트랙 외 — 기록만, 별건 처리)

| # | 발견 | 위치 | 영향 | 조치 |
|---|---|---|---|---|
| AUD-1 | 🐛 rekey 도구 PK 표기 오류 | `src/bin/rekey_encryption.rs:80` `pk:"oauth_id"` (실제 = `user_oauth_id`) | user_oauth 키 로테이션 실행 시 실패(현재 미실행이라 잠복) | 별건 1줄 수정. 그룹4 리네임 시 동반 가능 |
| AUD-2 | `user_export_data` dead table | 코드 0건 | 기능 미구현 인프라 | 그룹4 정리 시 리네임 vs 폐기 결정 |
| AUD-3 | `transactions.updated_at` 부재 | `payment/repo.rs:286` | 환불 갱신시각 미기록(기존 알려진 제약) | 본 트랙 무관, 기록만 |

---

## 7. 2단계(정리+마이그) 난이도·순서 평가

| 그룹 | 난이도 | 근거 |
|---|---|---|
| explanation→study_contents | 🔴 최상 | enum 4 + content_translations 4362행 데이터 동반 + study_explain 통합 배선 = INC-004급. **본 트랙 본질** |
| payment 3테이블 | 🔴 상 | 8 웹훅 핸들러 + 멱등성 + 통합테스트 raw SQL + 인덱스 8 + 프론트 admin |
| writing_practice 2테이블 | 🟡 중 | study 도메인 라우트 하위, 데이터 소량(190/25) |
| user_* 3테이블 | 🟡 중하 | prefix 통일, user_export_data dead, user_course 다중의존 |

**본질 = study_explain→study_contents 통합 후 리네임** (리네임만 선행 = churn). 순서·범위·마이그 전략 = §9.

---

## 9. 2단계 설계 (정리+마이그) — 확정 (2026-05-19)

### 9.1 사용자 확정 결정

| 결정 | 값 |
|---|---|
| 범위 | **A 순수 리네임 분리** — study_explain 통합·학습/admin 재배선·프론트는 후속 별도 트랙 |
| user_ vs users_ | **`users_` 통일** (§3.2.1 보강: user 도메인 토큰 = `users`, PG 예약어 기인 복수형) |
| writing_practice 명 | **`study_writing_practice_seed` / `study_writing_practice_session`** |
| 리네임 깊이 | **풀 정합** (테이블 + 인덱스 + 제약 + enum 타입명 + 모듈경로 + API라우트) |
| user_export_data | **`users_export_data` 리네임** (폐기 안 함) |
| explanation→study_contents | **이번 2단계 보류** (study_explain 처분 A1/A2 + `/explanations` 라우트 개명 = 그룹 ④ 도달 시 결정) |

### 9.2 스코프·시퀀싱 (이번 2단계 = 3그룹)

| 순서 | 그룹 | 리네임 | 위험 | 계약변경 |
|---|---|---|---|---|
| ① | user_* | `user_oauth`→`users_oauth` / `user_export_data`→`users_export_data` / `user_course`→`users_course` | 🟡 저 (rename+sqlx+test 루프 검증) | 0 |
| ② | writing_practice | `writing_practice_seed`→`study_writing_practice_seed` / `writing_practice_session`→`study_writing_practice_session` | 🟡 중 | 0 |
| ③ | payment | `subscriptions`→`payment_subscription` / `transactions`→`payment_transaction` / `webhook_events`→`payment_webhook_event` | 🔴 상 (8 웹훅 핸들러+멱등) | 0 |
| ④ | explanation→study_contents | **보류** — 그룹 ④ 도달 시 R1(study_explain A1/A2)·R3(/explanations 개명) 결정 후 착수 | 🔴 최상 | (라우트) |

3그룹 모두 공개 API 라우트 개명 없음 → 계약 변경 0 (계약 이슈는 보류한 ④에만).

### 9.3 마이그 방법 (전 그룹 공통)

- **신규 forward 마이그만** (적용된 마이그 절대 수정 금지 = INC-004 차단). 그룹당 마이그 1 + PR 1, 독립 배포·검증.
- `ALTER TABLE ... RENAME TO` + `ALTER INDEX/CONSTRAINT ... RENAME` + (해당 시) `ALTER TYPE ... RENAME TO` — 전부 메타데이터, 데이터 이동 0, 무중단.
- 풀 정합: 테이블·인덱스·제약·enum 타입명 + 코드 SQL 문자열 + 모듈 디렉터리/경로 + DTO/types.rs `#[sqlx(...)]` 동기화.
- enum **값**(content_type_enum) 동반은 ④ 전용 — 3그룹엔 enum 값 변경 없음(타입명 정합만, 해당 시).

### 9.4 로컬 검증 절차 (선결 블로커 없음 확인)

- 로컬 amk-pg = 비-head 손패치(버전 14/8자리 혼재, 20260517/18 미적용). **이 분기는 로컬 한정** — CI/prod는 fresh/기적용이라 신규 forward 마이그 깨끗이 append. 3그룹 테이블(구 마이그)은 로컬 존재.
- 검증 루프(그룹별): ①신규 마이그 amk-pg 수동 적용(explanation 선례 동일, 20260419 분기 강제 우회 **안 함**) → ②전 코드 SQL/경로 치환 → ③`cargo check`(query! 매크로가 리네임 스키마 검증) → ④`cargo sqlx prepare` `.sqlx` 재생성·커밋 → ⑤통합테스트(payment_integration 등) green → ⑥`npm run build`(프론트 영향 그룹) → ⑦도메인 문서+STATUS+CHANGELOG+메모리.

### 9.5 §3.2.1 보강 (메타부채 재발방지) — 그룹 ① PR 동반

- 규칙 보강: "user 도메인 테이블 = `users_<의미>` (PG 예약어 `USER`→복수형 `users`가 도메인 토큰). `user_*` 금지."
- 재발방지 검토: 신규 마이그 테이블명 SSoT 위반 차단(CI 스크립트 or 마이그 린트) — 설계만, 구현은 ③ 이후 별도 판단(YAGNI 경계).

---

## 10. 2단계 검증 패스 결과 (2026-05-19, 커밋 전 — 사용자 요구)

> 사용자: "커밋 전 추측 말고 실제 코드 기반 검증" → "다른 결함 없나" 재압박. 판단 기반 샘플링 → 결정론적 전수 게이트 전환.

### 10.1 적발·정정 결함 2건

| # | 결함 | 영향 | 정정 |
|---|---|---|---|
| 1 | 마이그 3파일 버전 prefix 전부 `20260519` 동일 (`migrations/README.md §1.3` "같은날 복수=연속날짜, HHMMSS 접미사 금지" 위반) | sqlx `_sqlx_migrations` PK = version → 중복 시 **CI/prod 마이그 실행 실패**. 과거 동일날짜 복수 선례 0(우리만) | `20260519`(user) / `20260520`(writing) / `20260521`(payment) — 3 리네임 상호 독립(disjoint 테이블)이라 순서 무관. 코드/문서/메모리 파일명 참조 전부 동기화 |
| 2 | `docs/AMK_APP_ROADMAP.md:134` 구명 `subscriptions` 잔존 | 도메인 문서 sweep 누락 (CLAUDE.md 문서 동기화 필수 위반) | `payment_subscription` 정정 + 전 `docs/` 재sweep 0 확인 |

원인 = 마이그 검증을 `psql` 수동적용(대용물)로 수행(`sqlx migrate run` 실제 경로 미실행) + 문서 sweep 을 AI 판단 범위로 한정. 사고 기록 = `AMK_AI_MISTAKES.md M-012`.

### 10.2 결정론적 게이트 결과 (판단 배제)

- **코드 diff 전수**: 9파일 45/45 완전 대칭, replace_all 오타격 0, 의도 외 변경 0.
- **라이브 DB 무결성**: INVALID FK 0 / 의존 뷰 0 / 트리거 0 / 참조 FK 2건(admin_course_log→users_course, payment_transaction→payment_subscription) 리네임 자동승계 확인.
- **AAD 암호문 6곳 불변** (변경 시 prod oauth 복호화 전면 실패 — 무사).
- **전역 grep**(스코프 무가정): src/tests/seeds/db-init/frontend/.sqlx 구 식별자 0. `.sqlx` 플래그 7건 = 신명 부분문자열·미변경 컬럼 오탐 확정(negative-lookbehind 재검 0).
- **fresh DB `sqlx migrate run`**: 우리 3 마이그 로드·버전검증 통과(중복 0 = 결함#1 수정 유효 입증). 실패는 `20260210000001`(2026-02 legacy 14자리) `content_type_enum does not exist` = **사전존재·문서화 G16 부채**(README §2-3, 우리 무관, 우리 마이그 도달 전). prod 무영향(증분적용, 우리 3개만 append).

### 10.3 잔여 (결함 아님 — 명명된 3개, 기존 배포 게이트로 닫힘)

1. **마이그 checksum/등록 (prod 증분)** — 모든 과거 마이그와 동일 위험 봉투(G16 표준 조건), 본 작업 특수 아님. 닫힘 = 배포 후 마이그 검증 SOP(`AMK_DEPLOY_OPS`).
2. **런타임 `sqlx::query()` 문자열** — 컴파일러 미검사 잔여. 단 리네임은 **정적 문자열 토큰만** 변경(동적 테이블명 생성 경로 0), 전역 grep 0 + 대표 쿼리(q1~6/w1~3/p1~4) 실DB 실행으로 토큰 정합 확인. 完封 = 정상 env 통합테스트.
3. **prod 런타임/상태** (20260504 적용 여부 등) — prod 에만 존재하는 사실. 닫힘 = 배포 후 스모크(explanation 선례).

> 결론: "버그 0" 증명 불가(원리적). 달성한 것 = 미지를 **결정론 게이트 통과 + 명명된 3 잔여(기존 게이트로 닫힘)**로 환원. 앱 통합테스트는 사전존재 env 패닉(`config.rs:421` EBOOK_IMAGE_ENCRYPTION_KEY, 리네임 무관)로 본 환경 미실행 — explanation 선례대로 prod 재검.

### 10.4 PR #314 CI 적발 — 결함 2건 추가 정정 (2026-05-19, 사용자 머지 시도)

검증 패스(§10.2)에서도 못 잡고 **CI(pr-check)가 차단** = 명명 잔여가 아닌 신규 클래스. 사고 = `AMK_AI_MISTAKES M-013`.

| # | 결함 | CI 신호 | 근본원인 | 정정 |
|---|---|---|---|---|
| 3 | 마이그 제약 RENAME 대상에 `user_export_data_user_id_fkey1`(중복 FK) — clean/prod 엔 부재(원본 20260208 FK 1회 정의, `_fkey` 단일) | integration·Playwright `psql:20260519: constraint "..._fkey1" does not exist` | 제약명을 마이그 원본(SoT) 아닌 **손패치 로컬 amk-pg** `pg_constraint` 스캔으로 작성 | 마이그 3개 = **존재 가드 재작성**(`DO`+`pg_constraint` IF EXISTS / `ALTER INDEX IF EXISTS`) → clean/prod/손패치 모두 안전 |
| 4 | `cargo fmt --check` 실패 (리네임으로 길어진 문자열 폭 초과) | backend job `cargo fmt --check --all` FAILURE | 커밋 전 `cargo fmt` 미실행 (M-008 재발) | `cargo fmt --all` 적용 |

**진짜 경로 재현 검증**(대용물 아님): fresh DB 에 **CI 동일 lexicographic `psql` 루프**로 전체 마이그 적용 → 우리 3 무에러 / `_fkey1` 정상 skip / 8테이블 전환·구명 0 / clean DB 단일 `users_export_data_user_id_fkey` 확인. backend = `cargo fmt --check` + `SQLX_OFFLINE cargo check --locked` + `cargo clippy -D warnings` 전부 0. main 머지 충돌(STATUS/CHANGELOG, 코드 0)은 HEAD(상위호환) 채택 해소.

**교훈**: 스키마 객체명 SoT = 마이그 원본이지 런타임 DB 아님(특히 손패치 환경). 마이그는 환경별 객체집합 차이를 가정한 **존재 가드**가 기본값이어야 한다.

