# AMK 문서 구조화 및 원칙 — 결정 사항 정리

> 작성일: 2026-03-15
> 상태: 논의 중 (미결정 사항 존재)
> 근거 문서: [AMK_DOCS_DEVELOP_POINT.md](./AMK_DOCS_DEVELOP_POINT.md) — 분석/리서치 보고서

---

## 1. 확정된 구조: 3단계 문서 체계

```
CLAUDE.md (자동 로드)
  │  1차 분기 → MEMORY.md 링크
  ▼
MEMORY.md (자동 로드)
  │  2차 분기 → AMK_*.md 링크
  ▼
AMK_*.md (필요 시 Read)
  → 각 작업의 세부사항 (SSoT)
```

| 파일 | 역할 | 제한 |
|------|------|------|
| **CLAUDE.md** | 지도 — 프로젝트 개요 + 작업 문서 링크 (→ MEMORY.md로 1차 분기) | 200줄 이하 |
| **MEMORY.md** | 작업 주의사항 + 작업 지침/내역/계획 문서 링크 (→ AMK 문서로 2차 분기) | 200줄 이하 |
| **AMK_*.md** | 각 작업에 대한 세부사항 기록 (SSoT) | 분할하여 관리 |

**탐색 방식:** CLAUDE.md + MEMORY.md는 모두 자동 로드되므로, 실질적으로 **1단계 탐색** (AMK 문서 Read만 하면 작업 가능).

**토픽 파일 계층 폐지:** 기존 `~/.claude/projects/.../memory/` 토픽 파일 30개는 AMK 문서로 승격하거나 삭제. 중간 계층을 없앰.

---

## 2. 확정된 원칙

| # | 원칙 | 설명 |
|---|------|------|
| 1 | **200줄 제한** | CLAUDE.md, MEMORY.md 각각 200줄 이하 엄수 |
| 2 | **1단계 탐색** | 자동 로드(CLAUDE.md + MEMORY.md) → AMK 문서 Read. 3단계 이상 금지 |
| 3 | **작업전/중/후 규칙 분리** | 규칙을 시점별로 분리하여 적용 시점 명확화 |
| 4 | **작업 유형별 분기** | 코드 작업 / 교재 작업 / 파이프라인 작업 각각 다른 진입점 |
| 5 | **복사 대신 포인터** | AMK 문서 내용을 MEMORY.md에 복사 금지, 참조(링크)만 기재 |
| 6 | **구식 문서는 독** | 완료/중복/오류 문서는 즉시 처리 (방치 금지) |
| 7 | **반응적 규칙 추가** | 사전에 규칙을 쌓지 말고, 실제 문제 발생 시에만 추가 |

---

## 3. 확정된 분할 방향

### 3.1 AMK_API_MASTER.md (269KB) → 공통 + 도메인별 7개

| 분할 후 파일 | 원본 섹션 | 내용 |
|-------------|----------|------|
| **AMK_API_MASTER.md** (축소 유지) | §0~4 | 문서 메타, 프로젝트 개요, 공통 규칙, 데이터 모델 개요 + 도메인 문서 인덱스 |
| AMK_API_AUTH.md | §5.3 | 인증 (로그인, OAuth, MFA, 비밀번호 재설정, 이메일 인증) |
| AMK_API_USER.md | §5.2 + §5.7 일부 | 사용자 관리 + 관리자 사용자 관리 |
| AMK_API_PAYMENT.md | §5.10 + §5.11 | 결제/구독 (관리자 + 사용자 Paddle) |
| AMK_API_EBOOK.md | §5.12.5 | E-book 웹 뷰어 |
| AMK_API_TEXTBOOK.md | §5.12 | 교재 주문 |
| AMK_API_LEARNING.md | §5.4 + §5.5 + §5.6 + §5.8 | video + study + lesson + course |
| AMK_API_FUTURE.md | §5.13~16 | 미구현 Phase (콘텐츠 시딩, 발음 평가, 조음, TTS) |

### 3.2 AMK_CODE_PATTERNS.md (131KB) → 2개

| 분할 후 파일 | 내용 |
|-------------|------|
| AMK_CODE_PATTERNS_BE.md | 백엔드 패턴 (Rust, Axum, SQLx) |
| AMK_CODE_PATTERNS_FE.md | 프론트엔드 패턴 (React, TypeScript, shadcn/ui) |

### 3.3 나머지 (30KB 이하) → 현행 유지

AMK_SCHEMA_PATCHED.md, AMK_DEPLOY_OPS.md, AMK_PIPELINE.md 등은 현행 유지.

---

## 4. 확정된 기타 사항

### 4.1 교재 작업

- 교재 전용 SSoT가 현재 **없음** — 별도 생성 필요
- `dev/amazing-korean-book` 별도 폴더 분리 검토 중
- 메모리 정리 완료 후 별도 진행

### 4.2 언어 정책

- AI 간 전달 문서: **영어**
- 사용자 보고: **한국어**

---

## 5. 미결정 사항

### 5.1 토픽 파일 30개의 개별 처리 방안

각 파일을 AMK 문서로 승격 / MEMORY.md에 한줄 흡수 / 삭제 중 어디로 보낼지 개별 판정 필요.

현재 토픽 파일 목록:
```
[장기적 가치 — AMK 문서 승격 후보]
- education_methodology.md      교육 방법론, 비즈니스 전략
- content_design.md             교재→웹 학습 콘텐츠 설계
- ebook_distribution.md         e-book 유통 전략
- ebook_market_research.md      한국어 교재 시장 조사
- pronunciation_ai_research.md  발음 평가 AI 조사
- articulation_animation_research.md  조음 애니메이션 조사
- guide_edition.md              해설용 교재 설계

[작업 기록 — 유지/아카이브 판단 필요]
- ebook_viewer.md               e-book 뷰어 시스템 상세
- ebook_viewer_improvement_plan.md  e-book 개선 계획
- cover_system.md               표지 자동 생성 시스템
- epub_system.md                EPUB3 빌드 파이프라인
- isbn_imprint.md               판권지 제목 통일
- isbn_title.md                 ISBN 학생용 확정
- en_edition.md                 영어본 교재
- student_edition.md            학생용 교재 디자인
- tense_table_update.md         시제 테이블 수정
- textbook_rebuild_lessons.md   교재 재구축 시행착오
- textbook_translation.md       다국어 번역 워크플로우
- textbook_tasks.md             교재 수정 작업 목록
- css_audit_and_fix.md          CSS 전수 감사
- ebook_pdf_fixes.md            PDF 비교 수정 상세
- translation_verification.md   번역 품질 검증

[완료/중복 — 삭제 후보]
- content_protection.md         AMK_API_MASTER §8.6 중복
- ebook_web_viewer.md           ebook_viewer.md 불완전 요약본
- login_table_plan.md           완료된 계획
- db_security_audit.md          완료된 감사
- db_encryption_phase2_plan.md  완료된 계획

[유지]
- debugging.md                  Claude가 겪은 실수 기록
- i18n_multilingual_plan.md     다국어 계획 (ar 오류 수정 필요)
- feedback_product_recommendation.md  Claude 전용 피드백
```

### 5.2 MEMORY.md의 구체적 구조

- 섹션을 어떻게 나눌 것인가 (작업 유형별? 도메인별? 상태별?)
- "작업 주의사항"에 어느 수준까지 넣을 것인가

### 5.3 CLAUDE.md의 구체적 구조

- 작업전/중/후 규칙의 구체적 내용
- 작업 유형별 분기를 어떤 형태로 기재할 것인가

### 5.4 AMK 문서 분할의 세부 범위

- AMK_API_MASTER.md 각 섹션의 정확한 분할 경계
- 분할 시 공통 정책(§0~4)에서 참조 링크를 어떻게 유지할지
- §6(프론트엔드), §7(개발 플로우), §8(보안) 등 공통 섹션의 배치

### 5.5 작업 순서

- MEMORY.md 정리 → CLAUDE.md 재구성 → AMK 분할 순서로 갈지
- 동시에 진행할지

### 5.6 대화 중 새 결정의 저장 방식

- 기존: 토픽 파일로 저장 → 나중에 정리
- 변경 후: 바로 AMK 문서에 반영? MEMORY.md에 한줄 기록? 둘 다?
