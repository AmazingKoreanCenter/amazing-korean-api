# Phase 2 — Google Cloud Translation 연동 + 핵심 5개 언어 실제 번역

## QA 테스트 가이드

**작성일**: 2026-02-11
**범위**: Phase 2 (Google Cloud Translation v2 Basic 백엔드 연동, 관리자 자동 번역 UI, 핵심 5개 언어 UI locale 번역)

---

## 1. 구현 현황 요약

### 1.1 백엔드 변경사항

| 파일 | 변경 내용 |
|------|----------|
| `src/external/translator.rs` | **[신규]** TranslationProvider trait + GoogleCloudTranslator 구현 (GCP v2 Basic REST API) |
| `src/external/mod.rs` | `pub mod translator;` 추가 |
| `src/types.rs` | `SupportedLanguage::to_gcp_code()` 메서드 추가 (21개 언어 → GCP 코드 매핑) |
| `src/config.rs` | 3개 환경변수 추가: `TRANSLATE_PROVIDER`, `GOOGLE_TRANSLATE_API_KEY`, `GOOGLE_TRANSLATE_PROJECT_ID` |
| `src/state.rs` | `AppState.translator: Option<Arc<dyn TranslationProvider>>` 추가 |
| `src/main.rs` | translator 초기화 로직 추가 (email 패턴 동일) |
| `src/api/admin/translation/dto.rs` | `AutoTranslateReq`, `AutoTranslateRes`, `AutoTranslateItemResult` DTO 추가 |
| `src/api/admin/translation/service.rs` | `auto_translate()` 메서드 추가 |
| `src/api/admin/translation/handler.rs` | `admin_auto_translate` 핸들러 추가 + `AppError` import |
| `src/api/admin/translation/router.rs` | `.route("/auto", post(admin_auto_translate))` 추가 |
| `src/docs.rs` | Swagger에 신규 핸들러 + 3개 DTO 등록 |

### 1.2 프론트엔드 변경사항

| 파일 | 변경 내용 |
|------|----------|
| `frontend/src/category/admin/translation/types.ts` | `AutoTranslateReq`, `AutoTranslateRes`, `AutoTranslateItemResult` 타입 추가 |
| `frontend/src/category/admin/admin_api.ts` | `autoTranslateContent()` API 함수 추가 |
| `frontend/src/category/admin/hook/use_translation_mutations.ts` | `useAutoTranslate` hook 추가 |
| `frontend/src/category/admin/page/admin_translations_page.tsx` | Auto Translate 버튼 + Dialog 컴포넌트 추가 |

### 1.3 UI Locale 번역 (4개 언어)

| 파일 | 변경 내용 |
|------|----------|
| `frontend/src/i18n/locales/ja.json` | 일본어 전체 번역 (322개 키) |
| `frontend/src/i18n/locales/zh-CN.json` | 중국어 간체 전체 번역 (322개 키) |
| `frontend/src/i18n/locales/zh-TW.json` | 중국어 번체 전체 번역 (322개 키) |
| `frontend/src/i18n/locales/vi.json` | 베트남어 전체 번역 (322개 키) |

> 기존 `ko.json` (한국어), `en.json` (영어)은 변경 없음.

### 1.4 인프라/문서 변경

| 파일 | 변경 내용 |
|------|----------|
| `docker-compose.prod.yml` | 환경변수 3개 추가 (TRANSLATE_PROVIDER, GOOGLE_TRANSLATE_API_KEY, GOOGLE_TRANSLATE_PROJECT_ID) |
| `docs/AMK_API_MASTER.md` | 9-8 엔드포인트 문서 추가, Phase 2 완료 표기 |
| `docs/AMK_DEPLOY_OPS.md` | .env.prod에 Translation 섹션 추가, GitHub Secrets 테이블 2행 추가 |

### 1.5 신규 환경변수

| 변수 | 기본값 | 프로덕션 필수 | 설명 |
|------|--------|:---:|------|
| `TRANSLATE_PROVIDER` | `none` | ❌ | `"google"` 또는 `"none"`. 프로덕션에서도 `none` 허용 (번역은 선택적) |
| `GOOGLE_TRANSLATE_API_KEY` | 없음 | ⚠️ | `TRANSLATE_PROVIDER=google`일 때 필수 |
| `GOOGLE_TRANSLATE_PROJECT_ID` | 없음 | ⚠️ | `TRANSLATE_PROVIDER=google`일 때 필수 |

> **이메일과의 차이점**: 이메일은 프로덕션에서 `EMAIL_PROVIDER=none` → panic. 번역은 프로덕션에서도 `TRANSLATE_PROVIDER=none` 허용 (번역 자동화는 선택적 기능).

---

## 2. 테스트 항목

### 2.1 서버 부팅 테스트

#### B-1: `TRANSLATE_PROVIDER=none`으로 서버 부팅 (기본값)
- **방법**: `.env`에 `TRANSLATE_PROVIDER` 미설정 또는 `none`으로 설정 후 서버 실행
- **기대**: 서버 정상 부팅, 로그에 `"Translation provider disabled (TRANSLATE_PROVIDER=none)"` 출력
- **확인**: `GET /health` 정상 응답

#### B-2: `TRANSLATE_PROVIDER=google`로 서버 부팅 (API Key 미설정)
- **방법**: `TRANSLATE_PROVIDER=google` 설정, `GOOGLE_TRANSLATE_API_KEY` 미설정
- **기대**: 서버 **panic** (`GOOGLE_TRANSLATE_API_KEY required when TRANSLATE_PROVIDER=google`)

#### B-3: `TRANSLATE_PROVIDER=google`로 서버 부팅 (정상)
- **방법**: `TRANSLATE_PROVIDER=google`, `GOOGLE_TRANSLATE_API_KEY=<valid-key>`, `GOOGLE_TRANSLATE_PROJECT_ID=<valid-id>` 설정
- **기대**: 서버 정상 부팅, 로그에 `"Translation provider enabled: Google Cloud Translation v2"` 출력

#### B-4: `TRANSLATE_PROVIDER=invalid`로 서버 부팅
- **방법**: `TRANSLATE_PROVIDER=invalid` 설정
- **기대**: 서버 **panic** (`Unknown TRANSLATE_PROVIDER 'invalid'`)

---

### 2.2 자동 번역 API 테스트 (TRANSLATE_PROVIDER=none)

#### A-1: 자동 번역 요청 시 503 응답
- **엔드포인트**: `POST /admin/translations/auto`
- **헤더**: 관리자 JWT Bearer 토큰
- **Body**:
```json
{
  "content_type": "video",
  "content_id": 1,
  "field_name": "title",
  "source_text": "테스트 텍스트",
  "target_langs": ["en"]
}
```
- **기대**: `502` (AppError::External → BAD_GATEWAY) JSON 응답, `"Translation provider not configured. Set TRANSLATE_PROVIDER=google."` 메시지

#### A-2: 비인증 요청 시 401 응답
- **엔드포인트**: `POST /admin/translations/auto`
- **헤더**: JWT 토큰 없음
- **기대**: `401 Unauthorized` JSON 응답

#### A-3: 일반 사용자(learner) 요청 시 403 응답
- **엔드포인트**: `POST /admin/translations/auto`
- **헤더**: 일반 사용자 JWT
- **기대**: `403 Forbidden` JSON 응답

---

### 2.3 자동 번역 API 테스트 (TRANSLATE_PROVIDER=google)

> ⚠️ 이 테스트는 유효한 Google Cloud Translation API Key가 필요합니다.

#### G-1: 단일 언어 자동 번역
- **엔드포인트**: `POST /admin/translations/auto`
- **Body**:
```json
{
  "content_type": "video",
  "content_id": 1,
  "field_name": "title",
  "source_text": "한국어 초급 과정",
  "target_langs": ["en"]
}
```
- **기대 응답** (200):
```json
{
  "total": 1,
  "success_count": 1,
  "results": [
    {
      "lang": "en",
      "success": true,
      "translation_id": <number>,
      "translated_text": "<영어 번역 결과>",
      "error": null
    }
  ]
}
```
- **DB 확인**: `content_translations` 테이블에 `content_type=video, content_id=1, field_name=title, lang=en, status=draft` 행 존재

#### G-2: 다중 언어 자동 번역
- **Body**:
```json
{
  "content_type": "course",
  "content_id": 1,
  "field_name": "description",
  "source_text": "초보자를 위한 한국어 학습 과정입니다.",
  "target_langs": ["en", "ja", "zh-CN", "zh-TW", "vi"]
}
```
- **기대**: `total: 5`, `success_count: 5`, 각 언어별 번역 결과 포함
- **DB 확인**: 5개 행이 모두 `draft` 상태로 저장됨

#### G-3: 동일 콘텐츠 재번역 시 UPSERT 동작
- **사전조건**: G-1에서 생성된 `video/1/title/en` 번역이 존재
- **Body**: G-1과 동일하되 `source_text`를 "한국어 중급 과정"으로 변경
- **기대**: 기존 행이 UPDATE됨 (새로운 행 생성 안 됨), `translated_text`가 변경되고 `status`가 다시 `draft`로 리셋

#### G-4: 이미 approved 상태인 번역 재번역 시
- **사전조건**: `video/1/title/en` 번역을 `approved`로 변경 후
- **Body**: G-1과 동일 (source_text 변경)
- **기대**: `translated_text` 변경 + `status`가 `draft`로 자동 리셋 (UPSERT 정책)

#### G-5: 빈 target_langs 배열
- **Body**: `target_langs: []`
- **기대**: `400 Bad Request` (validation 실패, min=1)

#### G-6: 20개 초과 target_langs
- **Body**: `target_langs`에 21개 이상 언어 전달
- **기대**: `400 Bad Request` (validation 실패, max=20)

#### G-7: source_text가 빈 문자열
- **Body**: `source_text: ""`
- **기대**: `400 Bad Request` (validation 실패, min=1)

---

### 2.4 기존 번역 CRUD API 회귀 테스트

> Phase 1A에서 구현된 기존 7개 엔드포인트가 정상 동작하는지 확인

#### R-1: `GET /admin/translations` 목록 조회
- 자동 번역으로 생성된 항목들이 목록에 정상 표시되는지 확인
- `?status=draft` 필터 동작 확인

#### R-2: `GET /admin/translations/{id}` 상세 조회
- 자동 번역으로 생성된 항목 ID로 상세 조회

#### R-3: `PUT /admin/translations/{id}` 수정
- 자동 번역 결과를 수동으로 수정 가능한지 확인

#### R-4: `PATCH /admin/translations/{id}/status` 상태 변경
- `draft → reviewed → approved` 상태 전이 정상 동작

#### R-5: `DELETE /admin/translations/{id}` 삭제
- 자동 번역 항목 삭제 가능

---

### 2.5 프론트엔드 — 관리자 번역 페이지

#### F-1: 번역 목록 페이지 진입
- **경로**: `/admin/translations`
- **기대**: 기존 번역 목록 + "New Translation" 버튼 + **"Auto Translate" 버튼** 표시

#### F-2: Auto Translate 버튼 클릭 → Dialog 열림
- **기대**: Dialog에 다음 필드 표시:
  - Content Type 드롭다운 (course, lesson, video, video_tag, study)
  - Content ID 입력 필드
  - Field Name 입력 필드
  - Source Text 텍스트영역
  - Target Languages 체크박스 그룹 (21개 언어) + "Select All" 체크박스
  - "Run Auto Translate" 버튼

#### F-3: Auto Translate Dialog — Select All 동작
- **기대**: "Select All" 체크 시 모든 21개 언어 체크, 해제 시 모두 해제
- **기대**: 개별 언어 해제 시 "Select All" 자동 해제

#### F-4: Auto Translate 실행 (TRANSLATE_PROVIDER=none)
- **방법**: 모든 필드 입력 후 "Run Auto Translate" 클릭
- **기대**: 에러 토스트 표시 (Translation provider not configured)

#### F-5: Auto Translate 실행 (TRANSLATE_PROVIDER=google)
- **방법**: 유효한 데이터 입력 후 실행
- **기대**:
  - 버튼이 "Translating..." + 로딩 스피너로 변경
  - 성공 시 토스트: "자동 번역 완료: X/Y 성공"
  - 번역 목록 자동 갱신 (invalidateQueries)

#### F-6: Auto Translate Dialog — 필수 필드 누락
- **방법**: 일부 필드 비워두고 실행
- **기대**: 폼 제출 차단 (required 속성) 또는 validation 에러

#### F-7: 기존 번역 CRUD UI 회귀
- **기대**: "New Translation" Dialog, Edit, Delete, Status 변경 등 기존 UI 정상 동작

---

### 2.6 UI Locale 번역 검증 (5개 핵심 언어)

> 개발 서버에서 언어를 전환하여 UI 텍스트가 올바르게 표시되는지 확인

#### L-1: 일본어 (ja) UI 전환
- **방법**: 설정 페이지에서 언어를 일본어로 변경
- **확인 항목**:
  - 네비게이션: 紹介, 動画, 学習, レッスン, マイページ, ログイン, 会員登録, ログアウト
  - 홈페이지 Hero: "韓国語学習の新しい始まり"
  - 로그인 페이지: "Amazing Koreanへようこそ", "メールでログイン"
  - Footer: "世界中の韓国語学習者のための最高のオンライン学習プラットフォーム"

#### L-2: 중국어 간체 (zh-CN) UI 전환
- **확인 항목**:
  - 네비게이션: 介绍, 视频, 学习, 课程, 我的页面, 登录, 注册, 退出登录
  - 홈페이지 Hero: "韩语学习的全新开始"
  - 로그인: "欢迎来到Amazing Korean", "使用邮箱登录"
  - Footer: "面向全球韩语学习者的最佳在线学习平台"

#### L-3: 중국어 번체 (zh-TW) UI 전환
- **확인 항목**:
  - 네비게이션: 介紹, 影片, 學習, 課程, 我的頁面, 登入, 註冊, 登出
  - **zh-CN과 구분 확인**: "视频" vs "影片", "登录" vs "登入", "注册" vs "註冊"
  - Footer: "面向全球韓語學習者的最佳線上學習平台"

#### L-4: 베트남어 (vi) UI 전환
- **확인 항목**:
  - 네비게이션: Giới thiệu, Video, Học tập, Bài học, Trang cá nhân, Đăng nhập, Đăng ký, Đăng xuất
  - 홈페이지 Hero: "Khởi đầu mới cho việc học tiếng Hàn"
  - 로그인: "Chào mừng đến với Amazing Korean"
  - Footer: "Nền tảng học trực tuyến tốt nhất dành cho người học tiếng Hàn trên toàn thế giới"

#### L-5: 영어 (en) 회귀 확인
- **기대**: 기존 영어 번역 정상 동작 (변경 없음)

#### L-6: 한국어 (ko) 회귀 확인
- **기대**: 기존 한국어 원본 정상 동작 (변경 없음)

#### L-7: 미번역 언어 fallback 확인
- **방법**: 태국어 (th) 등 아직 번역되지 않은 언어로 전환
- **기대**: fallback 순서에 따라 ko (한국어) 표시

---

### 2.7 Swagger 문서 검증

#### S-1: Swagger UI에서 신규 엔드포인트 확인 (개발 환경, ENABLE_DOCS=1)
- **경로**: `/docs`
- **기대**: `POST /admin/translations/auto` 엔드포인트 표시
- **기대**: `AutoTranslateReq`, `AutoTranslateRes`, `AutoTranslateItemResult` 스키마 표시

#### S-2: 프로덕션에서 Swagger 비활성 (ENABLE_DOCS=0)
- **기대**: `/docs` 접근 불가

---

### 2.8 docker-compose.prod.yml 검증

#### D-1: 환경변수 확인
- `docker-compose.prod.yml`의 `api` 서비스에 다음 3개 환경변수 존재 확인:
```yaml
TRANSLATE_PROVIDER: ${TRANSLATE_PROVIDER:-none}
GOOGLE_TRANSLATE_API_KEY: ${GOOGLE_TRANSLATE_API_KEY:-}
GOOGLE_TRANSLATE_PROJECT_ID: ${GOOGLE_TRANSLATE_PROJECT_ID:-}
```

---

## 3. 테스트 우선순위

| 우선순위 | 범주 | 항목 |
|:---:|------|------|
| **HIGH** | 서버 부팅 | B-1, B-2, B-3, B-4 |
| **HIGH** | API 보안 | A-1, A-2, A-3 |
| **HIGH** | 기존 기능 회귀 | R-1~R-5, F-7, L-5, L-6 |
| **MEDIUM** | 자동 번역 핵심 | G-1, G-2, G-3, G-4 |
| **MEDIUM** | 프론트엔드 UI | F-1~F-6 |
| **MEDIUM** | Locale 번역 | L-1~L-4, L-7 |
| **LOW** | Validation | G-5, G-6, G-7 |
| **LOW** | Swagger / Infra | S-1, S-2, D-1 |

---

## 4. 테스트 환경 설정

### 4.1 TRANSLATE_PROVIDER=none (기본, API Key 불필요)
```env
# .env에 TRANSLATE_PROVIDER 미설정 또는:
TRANSLATE_PROVIDER=none
```
- B-1, A-1~A-3, F-4, R-1~R-5, F-1~F-3, F-6~F-7, L-1~L-7, S-1~S-2, D-1 테스트 가능

### 4.2 TRANSLATE_PROVIDER=google (API Key 필요)
```env
TRANSLATE_PROVIDER=google
GOOGLE_TRANSLATE_API_KEY=<your-gcp-api-key>
GOOGLE_TRANSLATE_PROJECT_ID=<your-gcp-project-id>
```
- B-3, G-1~G-7, F-5 테스트 가능

> **GCP API Key 발급**: Google Cloud Console → APIs & Services → Credentials → API Key 생성 → Cloud Translation API 활성화 필요

---

## 5. 알려진 제한사항

1. **경고 1건**: `cargo check` 시 `GcpError.code` 필드 미사용 경고 (dead_code) — GCP 에러 응답에서 `code` 필드를 역직렬화하지만 현재 직접 참조하지 않음. 향후 에러 핸들링 고도화 시 활용 예정.
2. **번역 품질**: AI 자동 번역(locale 파일 포함)은 초안 수준이므로 수동 검수 필요.
3. **번역 요금**: Google Cloud Translation API는 문자 수 기반 과금. 대량 번역 시 비용 발생 가능.
