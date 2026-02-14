# QA 리포트: 관리자 번역 기능 추가 수정 (2026-02-14)

> 초기 QA (30/30 PASS) 이후 추가 변경된 파일들에 대한 QA 테스트 결과

## 변경 범위

### 백엔드
| 파일 | 변경 내용 |
|------|-----------|
| `dto.rs` | `TranslationListReq.content_types` 필드 추가, `TranslationSearchReq` 간소화 (lang만), `TranslationStatItem` + `TranslationStatsRes` 신규 |
| `repo.rs` | `count_all`/`find_all`에 `content_types_csv` 파라미터 추가, `search_translations` 간소화, `find_translation_stats` 신규 |
| `service.rs` | `list_translations`에 content_types 우선순위 로직, `get_translation_stats` 신규 |
| `handler.rs` | `admin_get_translation_stats` 핸들러 추가, utoipa `status = 500` → `502` 수정 |
| `router.rs` | `/stats` GET 라우트 추가 |
| `docs.rs` | **누락 발견 → 수정 완료** (5개 핸들러 + 15개 DTO 등록) |

### 프론트엔드
| 파일 | 변경 내용 |
|------|-----------|
| `types.ts` | `ko` 추가, `content_types` 필드, `TranslationStatItem`, `TranslationStatsRes` |
| `admin_api.ts` | `searchTranslations` 간소화, `getTranslationStats` 신규 |
| `use_translations.ts` | `useSearchTranslations` 간소화, `useTranslationStats` 신규 |
| `admin_translations_page.tsx` | Dashboard 버튼, TIER_BREAK_INDICES 구분선, `content_types` 서버사이드 필터링 |
| `admin_translation_dashboard.tsx` | **신규 파일** — content_type × lang 매트릭스 히트맵 |
| `routes.tsx` | `/admin/translations/dashboard` 라우트 추가 |

---

## 테스트 결과 (19/19 PASS)

### 빌드 검증 (2/2)

| ID | 테스트 | 결과 |
|----|--------|------|
| B-1 | `cargo check` | **PASS** |
| B-2 | `npm run build` (tsc + vite) | **PASS** (22 locale 청크) |

### 코드 리뷰 (6/6)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| C-1 | `TranslationStatItem` — `FromRow` derive + 4필드(content_type, lang, status, count) | **PASS** | |
| C-2 | `find_translation_stats` — GROUP BY + ORDER BY 정확 | **PASS** | |
| C-3 | `content_types_csv` SQL — `string_to_array` + `ANY` 매칭 | **PASS** | CASE/WHEN으로 우선순위 처리 |
| C-4 | `search_translations` — lang 옵셔널, `approved`/`reviewed` 필터, LIMIT 50 | **PASS** | |
| C-5 | Dashboard 페이지 — `useMemo` 매트릭스, `getStatusColor`, TIER_BREAK_INDICES | **PASS** | |
| C-6 | 라우트 등록 — `/admin/translations/dashboard` | **PASS** | |

### API 런타임 테스트 (11/11)

| ID | 테스트 | 기대 | 결과 |
|----|--------|------|------|
| T-1 | `GET /admin/translations/stats` (관리자 JWT) | 200 + `items[]` + `total_translations` | **PASS** — 4개 항목, total=8 |
| T-2 | `GET /admin/translations/stats` (인증 없음) | 401 | **PASS** |
| T-3 | `GET /admin/translations/search` (lang 없이) | 200 + approved/reviewed 목록 | **PASS** — 1건 (vi/approved) |
| T-4 | `GET /admin/translations/search?lang=vi` | 200 + vi만 필터 | **PASS** |
| T-5 | `GET /admin/translations/search?lang=en` | 200 + 빈 배열 | **PASS** |
| T-6 | `?content_types=study,...` (Study 전체) | 200 + Study 계열만 | **PASS** — 0건 (데이터 없음, 필터 정상) |
| T-7 | `?content_type=lesson` (단수 필터) | 200 + lesson만 | **PASS** — 4건 |
| T-8 | `?content_types=video&content_type=lesson` (우선순위) | content_types 우선 → video만 | **PASS** — 1건 (video) |
| T-9 | 필터 없이 전체 조회 | 200 + 모든 타입 포함 | **PASS** — total=8 |
| T-10 | `?content_types=video,video_tag` (복수 타입) | video + video_tag | **PASS** — 4건 |
| T-11 | `?content_types=lesson,video` | lesson + video | **PASS** — 5건 |

---

## 발견 이슈 및 수정

### M-1: docs.rs Swagger 등록 누락 (수정 완료)

- **심각도**: Medium
- **내용**: 5개 신규 핸들러 + 15개 신규 DTO가 `src/docs.rs`에 등록되지 않아 Swagger UI에 미표시
- **수정**: paths에 5개 핸들러, schemas에 15개 DTO 추가
- **검증**: `cargo check` 통과

**추가된 핸들러 (paths)**:
1. `admin_list_content_records`
2. `admin_get_source_fields`
3. `admin_auto_translate_bulk`
4. `admin_search_translations`
5. `admin_get_translation_stats`

**추가된 DTO (schemas)**:
1. `ContentRecordsReq`, `ContentRecordItem`, `ContentRecordsRes`
2. `SourceFieldsReq`, `SourceFieldItem`, `SourceFieldsRes`
3. `AutoTranslateBulkItem`, `AutoTranslateBulkReq`, `AutoTranslateBulkItemResult`, `AutoTranslateBulkRes`
4. `TranslationSearchReq`, `TranslationSearchItem`, `TranslationSearchRes`
5. `TranslationStatItem`, `TranslationStatsRes`

---

## 최종 결과

| 카테고리 | 항목 수 | PASS | FAIL |
|----------|---------|------|------|
| 빌드 검증 | 2 | 2 | 0 |
| 코드 리뷰 | 6 | 6 | 0 |
| API 런타임 | 11 | 11 | 0 |
| **합계** | **19** | **19** | **0** |

**이슈**: M-1 (docs.rs 누락) → **수정 완료**

---

## 브라우저 수동 테스트 권장 항목

1. `/admin/translations` → Dashboard 버튼 클릭 → 대시보드 페이지 로드 확인
2. 대시보드에서 content_type × lang 매트릭스 셀 클릭 → 해당 필터 목록 페이지 이동
3. 번역 목록에서 Study 카테고리 선택 → 하위 5종 서버사이드 필터링 동작
4. 언어 드롭다운에서 Tier 구분선 표시 확인
