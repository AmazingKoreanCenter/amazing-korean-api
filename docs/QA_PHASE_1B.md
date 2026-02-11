# Phase 1B QA 테스트 가이드라인 — 프론트엔드 다국어 기반 구축

## 개요

Phase 1B는 프론트엔드에서 21개 언어를 지원하기 위한 기반 인프라를 구축한 작업입니다.
Phase 1A(DB + 백엔드)는 이미 완료 + QA 통과 상태이며, 이번 QA는 **프론트엔드 변경사항**에 집중합니다.

**빌드 상태**: `npm run build` 통과 확인 완료 (tsc + vite, 2026-02-11)

---

## 변경 범위 요약

### 수정된 프론트엔드 파일 (15개)

| 파일 | 변경 내용 |
|------|-----------|
| `frontend/index.html` | Pretendard 폰트 CDN 링크 추가 |
| `frontend/tailwind.config.js` | fontFamily.sans에 Pretendard 추가 |
| `frontend/src/i18n/index.ts` | 21개 언어 동적 로딩, async changeLanguage, SUPPORTED_LANGUAGES 상수 |
| `frontend/src/components/layout/header.tsx` | Globe 토글 → DropdownMenu 드롭다운 (21개 언어) |
| `frontend/src/category/user/page/settings_page.tsx` | 언어 선택 2개 → 21개 |
| `frontend/src/hooks/use_language_sync.ts` | async changeLanguage 대응 (void 추가) |
| `frontend/src/category/lesson/lesson_api.ts` | `lang` 파라미터 추가 |
| `frontend/src/category/lesson/hook/use_lesson_list.ts` | queryKey에 lang 포함 |
| `frontend/src/category/lesson/hook/use_lesson_detail.ts` | queryKey에 lang 포함 |
| `frontend/src/category/video/video_api.ts` | `lang` 파라미터 추가 |
| `frontend/src/category/video/hook/use_video_list.ts` | queryKey에 lang 포함 |
| `frontend/src/category/video/hook/use_video_detail.ts` | queryKey에 lang 포함 |
| `frontend/src/category/study/study_api.ts` | `lang` 파라미터 추가 |
| `frontend/src/category/study/hook/use_study_list.ts` | queryKey에 lang 포함 |
| `frontend/src/category/study/hook/use_study_detail.ts` | queryKey에 lang 포함 |

### 신규 프론트엔드 파일 (26개)

| 파일 | 설명 |
|------|------|
| `frontend/src/utils/font_loader.ts` | 언어별 폰트 동적 로딩 유틸리티 |
| `frontend/src/utils/content_lang.ts` | 콘텐츠 API용 lang 파라미터 유틸리티 |
| `frontend/src/i18n/locales/{19개}.json` | 빈 locale JSON (ja, zh-CN, zh-TW, vi, th, id, my, mn, ru, es, pt, fr, de, hi, ne, si, km, uz, kk, tg) |
| `frontend/src/category/admin/translation/types.ts` | 번역 관리 Zod 스키마 + 타입 |
| `frontend/src/category/admin/hook/use_translations.ts` | 번역 목록/상세 useQuery 훅 |
| `frontend/src/category/admin/hook/use_translation_mutations.ts` | 번역 CRUD useMutation 훅 |
| `frontend/src/category/admin/page/admin_translations_page.tsx` | 번역 목록 페이지 |
| `frontend/src/category/admin/page/admin_translation_edit.tsx` | 번역 생성/수정 페이지 |

### 수정된 라우팅/레이아웃 (2개)

| 파일 | 변경 |
|------|------|
| `frontend/src/app/routes.tsx` | Admin 라우트에 `translations`, `translations/new`, `translations/:id/edit` 추가 |
| `frontend/src/category/admin/page/admin_layout.tsx` | 사이드바에 "Translations" 메뉴 추가 (Languages 아이콘) |

---

## QA 테스트 항목

### 1. 폰트 적용 확인

| ID | 테스트 | 검증 방법 | 예상 결과 |
|----|--------|-----------|-----------|
| F-1 | Pretendard 폰트 로드 | DevTools > Network에서 `pretendardvariable` 요청 확인 | CDN에서 폰트 파일 정상 로드 (200) |
| F-2 | 한국어 텍스트 렌더링 | 메인 페이지에서 한국어 텍스트 확인 | Pretendard 폰트로 깔끔하게 렌더링 |
| F-3 | 영어 텍스트 렌더링 | 언어를 English로 변경 후 텍스트 확인 | Pretendard 폰트 유지 (라틴 문자 커버) |
| F-4 | 일본어 폰트 동적 로드 | 언어를 日本語로 변경 | DevTools Network에서 `Noto+Sans+JP` 폰트 요청 발생 |
| F-5 | 중국어(간체) 폰트 동적 로드 | 언어를 中文(简体)으로 변경 | DevTools Network에서 `Noto+Sans+SC` 폰트 요청 발생 |
| F-6 | 중국어(번체) 폰트 동적 로드 | 언어를 中文(繁體)으로 변경 | DevTools Network에서 `Noto+Sans+TC` 폰트 요청 발생 |
| F-7 | 태국어 폰트 동적 로드 | 언어를 ภาษาไทย로 변경 | DevTools Network에서 `Noto+Sans+Thai` 폰트 요청 발생 |
| F-8 | 폰트 중복 로드 방지 | 같은 언어를 두 번 선택 | `<link>` 태그가 한 번만 삽입됨 (Elements 탭에서 확인) |
| F-9 | 라틴/키릴 계열 폰트 | 언어를 Русский, Español 등으로 변경 | 별도 폰트 요청 없음 (Pretendard가 커버) |

### 2. 언어 드롭다운 UI

| ID | 테스트 | 검증 방법 | 예상 결과 |
|----|--------|-----------|-----------|
| L-1 | 데스크톱 드롭다운 표시 | 헤더의 Globe 버튼 클릭 | 21개 언어가 드롭다운으로 표시, Tier별 구분선 (5번째/11번째 뒤) |
| L-2 | 현재 언어 체크 표시 | 드롭다운 열기 | 현재 선택된 언어에 체크(✓) 아이콘 표시 |
| L-3 | 언어 선택 시 UI 변경 | 드롭다운에서 English 선택 | 헤더 텍스트가 영어로 변경, Globe 버튼 텍스트가 "English"로 변경 |
| L-4 | 모바일 드롭다운 | 모바일 뷰(햄버거 메뉴)에서 언어 버튼 클릭 | 동일한 21개 언어 드롭다운 표시 |
| L-5 | 드롭다운 스크롤 | 21개 언어 중 하단 언어 확인 | `max-h-80` (데스크톱) / `max-h-60` (모바일) 내에서 스크롤 가능 |
| L-6 | 비로그인 상태 언어 변경 | 로그아웃 후 언어 변경 | i18n + localStorage 저장만 (API 호출 없음) |
| L-7 | 로그인 상태 언어 변경 | 로그인 후 언어 변경 | i18n + localStorage + `PATCH /users/settings` API 호출 |

### 3. i18next 동적 로딩

| ID | 테스트 | 검증 방법 | 예상 결과 |
|----|--------|-----------|-----------|
| I-1 | 한국어 즉시 로드 | 페이지 첫 로드 (ko) | ko.json이 메인 번들에 포함, 별도 네트워크 요청 없음 |
| I-2 | 영어 즉시 로드 | 언어를 English로 변경 | en.json이 메인 번들에 포함, 별도 네트워크 요청 없음 |
| I-3 | 일본어 동적 로드 | 언어를 日本語로 변경 | DevTools Network에서 `ja-*.js` chunk 요청 발생 |
| I-4 | Fallback 동작 | 일본어 선택 시 UI 텍스트 | 빈 JSON이므로 en fallback → en 텍스트 표시 (ko가 아닌 en) |
| I-5 | localStorage 저장 | 언어 변경 후 DevTools > Application > localStorage | `language` 키에 선택한 언어 코드 저장 |
| I-6 | localStorage 복원 | 페이지 새로고침 | localStorage에 저장된 언어로 자동 복원 |
| I-7 | html lang 속성 | 언어 변경 후 Elements 탭에서 `<html>` 확인 | `lang="ja"` 등으로 업데이트 |
| I-8 | 초기 로드 기본값 | localStorage 비운 후 페이지 로드 | `lang="ko"` (한국어 기본) |

### 4. 설정 페이지 (Settings)

| ID | 테스트 | 검증 방법 | 예상 결과 |
|----|--------|-----------|-----------|
| S-1 | 21개 언어 표시 | `/user/settings` 접근 (로그인 필요) | Language 드롭다운에 21개 언어 표시 (`nativeName (name)` 형식) |
| S-2 | 현재 언어 선택 상태 | 설정 페이지 접근 | 현재 i18n 언어가 선택된 상태로 표시 |
| S-3 | 헤더 언어 변경 → 설정 연동 | 헤더에서 언어 변경 후 설정 페이지 확인 | 설정 페이지의 Language 값도 변경됨 (dirty 상태 아님) |
| S-4 | 설정 저장 후 언어 반영 | 설정에서 언어 변경 후 Save | 성공 후 changeLanguage 호출, UI 전체 변경 |
| S-5 | 모든 언어 선택 가능 | 21개 언어 순서대로 선택 | 이전 ko/en 제한 없이 모두 선택 가능 |

### 5. 콘텐츠 API ?lang= 전달

| ID | 테스트 | 검증 방법 | 예상 결과 |
|----|--------|-----------|-----------|
| C-1 | 한국어 시 lang 미전달 | ko 상태에서 `/videos` 접근, DevTools Network | `GET /videos` 요청에 `lang` 파라미터 없음 |
| C-2 | 영어 시 lang 전달 | en 상태에서 `/videos` 접근 | `GET /videos?lang=en` |
| C-3 | 일본어 시 lang 전달 | ja 상태에서 `/videos` 접근 | `GET /videos?lang=ja` |
| C-4 | 비디오 상세 lang 전달 | en 상태에서 `/videos/1` 접근 | `GET /videos/1?lang=en` |
| C-5 | 레슨 목록 lang 전달 | en 상태에서 `/lessons` 접근 | `GET /lessons?lang=en` |
| C-6 | 레슨 상세 lang 전달 | en 상태에서 `/lessons/1` 접근 | `GET /lessons/1?lang=en` |
| C-7 | 스터디 목록 lang 전달 | en 상태에서 `/studies` 접근 | `GET /studies?lang=en` |
| C-8 | 스터디 상세 lang 전달 | en 상태에서 `/studies/1` 접근 | `GET /studies/1?lang=en` |
| C-9 | 언어 변경 시 자동 refetch | `/videos`에서 en → ja 변경 | queryKey 변경으로 자동 refetch 발생 (Network에서 새 요청 확인) |
| C-10 | ko 전환 시 lang 제거 | ja → ko 변경 | `lang` 파라미터 없이 요청 |

### 6. 관리자 번역 관리 UI

> **전제조건**: admin 또는 HYMN 권한 계정으로 로그인 필요

| ID | 테스트 | 검증 방법 | 예상 결과 |
|----|--------|-----------|-----------|
| A-1 | 사이드바 메뉴 표시 | `/admin` 접근 | "Translations" 메뉴 항목 표시 (Languages 아이콘) |
| A-2 | 번역 목록 페이지 접근 | `/admin/translations` 접근 | 번역 목록 테이블 표시 (빈 목록 시 "No translations found.") |
| A-3 | Content Type 필터 | 드롭다운에서 "Video" 선택 | content_type=video로 필터링된 결과 |
| A-4 | Language 필터 | 드롭다운에서 "English (en)" 선택 | lang=en으로 필터링된 결과 |
| A-5 | Status 필터 | 드롭다운에서 "Draft" 선택 | status=draft로 필터링된 결과 |
| A-6 | New Translation 버튼 | 클릭 | `/admin/translations/new`로 이동 |
| A-7 | 번역 생성 폼 | 모든 필드 입력 후 Create 클릭 | `POST /admin/translations` 호출, 성공 시 목록으로 이동 + 토스트 |
| A-8 | 번역 생성 — 필수 필드 검증 | field_name 비워두고 제출 | Zod 검증 에러 표시 |
| A-9 | 번역 수정 페이지 접근 | 목록에서 "Edit" 클릭 | `/admin/translations/:id/edit`로 이동, 기존 데이터 로드 |
| A-10 | 번역 수정 — 읽기 전용 메타 | 수정 페이지 확인 | Type, Content ID, Field, Language는 읽기 전용 표시 |
| A-11 | 번역 수정 — 텍스트 변경 | 텍스트 수정 후 Update 클릭 | `PUT /admin/translations/:id` 호출, 성공 시 목록으로 이동 |
| A-12 | 번역 상태 변경 (목록에서) | 목록의 Status 드롭다운 변경 | `PATCH /admin/translations/:id/status` 호출, 즉시 반영 |
| A-13 | 번역 삭제 | 목록에서 "Delete" 클릭 | 확인 다이얼로그 → `DELETE /admin/translations/:id` 호출 |
| A-14 | 페이지네이션 | 20개 초과 번역 생성 후 확인 | 하단 페이지네이션 표시, 페이지 이동 가능 |
| A-15 | Back 버튼 | 생성/수정 페이지에서 Back 클릭 | `/admin/translations`로 이동 |

### 7. 언어 동기화 (Language Sync)

| ID | 테스트 | 검증 방법 | 예상 결과 |
|----|--------|-----------|-----------|
| Y-1 | 로그인 시 DB 언어 적용 | DB에 `user_set_language=en` 저장 후 로그인 | 자동으로 English UI로 변경 |
| Y-2 | 로그아웃 시 언어 유지 | en 상태에서 로그아웃 | localStorage의 언어 유지 (en 유지) |
| Y-3 | 재로그인 시 중복 적용 방지 | 로그인 → 언어 변경 → 페이지 이동 | DB 언어가 한 번만 적용됨 (appliedRef로 중복 방지) |

### 8. 기존 기능 회귀 테스트

| ID | 테스트 | 검증 방법 | 예상 결과 |
|----|--------|-----------|-----------|
| R-1 | 로그인 정상 동작 | 이메일/비밀번호 로그인 | 정상 로그인, 리다이렉트 |
| R-2 | 회원가입 정상 동작 | 신규 회원가입 플로우 | 정상 회원가입 |
| R-3 | 비디오 목록/상세 | `/videos`, `/videos/:id` 접근 | 정상 렌더링 (데이터 로드) |
| R-4 | 스터디 목록/상세 | `/studies`, `/studies/:id` 접근 | 정상 렌더링 |
| R-5 | 레슨 목록/상세 | `/lessons`, `/lessons/:id` 접근 | 정상 렌더링 |
| R-6 | 마이페이지 접근 | `/user/me` 접근 | 정상 렌더링 |
| R-7 | 관리자 대시보드 | `/admin` 접근 | 정상 렌더링 |
| R-8 | 관리자 기존 CRUD | 유저/비디오/스터디/레슨 관리 | 기존 기능 정상 동작 |

---

## 테스트 환경 설정

### 개발 서버 실행

```bash
# 프론트엔드 개발 서버
cd frontend && npm run dev

# 백엔드 (필요 시)
cargo run
```

### 브라우저 DevTools 활용

- **Network 탭**: 폰트 로드, locale chunk 로드, API ?lang= 파라미터 확인
- **Application > localStorage**: `language` 키 확인
- **Elements 탭**: `<html lang="...">` 속성, `<link id="font-*">` 태그 확인
- **Console**: 에러/경고 확인

### 21개 지원 언어 목록

| Tier | 언어 코드 | 네이티브명 | 폰트 |
|------|-----------|-----------|------|
| **Tier 1** | ko | 한국어 | Pretendard |
| | en | English | Pretendard |
| | ja | 日本語 | Noto Sans JP (동적) |
| | zh-CN | 中文(简体) | Noto Sans SC (동적) |
| | zh-TW | 中文(繁體) | Noto Sans TC (동적) |
| **Tier 2** | vi | Tiếng Việt | Pretendard |
| | th | ภาษาไทย | Noto Sans Thai (동적) |
| | id | Bahasa Indonesia | Pretendard |
| | my | မြန်မာဘာသာ | Noto Sans Myanmar (동적) |
| | mn | Монгол хэл | Pretendard |
| | ru | Русский | Pretendard |
| **Tier 3** | es | Español | Pretendard |
| | pt | Português | Pretendard |
| | fr | Français | Pretendard |
| | de | Deutsch | Pretendard |
| | hi | हिन्दी | Noto Sans Devanagari (동적) |
| | ne | नेपाली | Noto Sans Devanagari (동적) |
| | si | සිංහල | Noto Sans Sinhala (동적) |
| | km | ភាសាខ្មែរ | Noto Sans Khmer (동적) |
| | uz | Oʻzbekcha | Pretendard |
| | kk | Қазақ тілі | Pretendard |
| | tg | Тоҷикӣ | Pretendard |

---

## 알려진 제한사항 (Phase 1B 범위)

1. **번역 텍스트 없음**: 19개 언어의 locale JSON은 빈 파일 (`{}`) — UI 텍스트는 en fallback으로 표시됨. 실제 번역은 Phase 2에서 진행.
2. **콘텐츠 번역 없음**: `content_translations` 테이블에 데이터가 없으면 원본(ko) 표시. 번역 데이터는 관리자 UI에서 수동 입력하거나 Phase 2에서 자동 생성.
3. **Vite 빌드 경고**: ko.json/en.json이 정적+동적 import 양쪽에 있어 "dynamic import will not move module into another chunk" 경고 발생 — 동작에 영향 없음.
4. **chunk size 경고**: 메인 번들이 500KB 초과 — Phase 2 이후 code splitting 최적화 예정.

---

## 심각도 분류

- **H (High)**: 기능 불가 — 언어 변경 불가, 페이지 크래시, API 호출 실패
- **M (Medium)**: 기능 저하 — 폰트 미로드, 잘못된 fallback, UI 깨짐
- **L (Low)**: 미관/편의 — 스크롤 동작, 구분선 위치, 텍스트 truncate
