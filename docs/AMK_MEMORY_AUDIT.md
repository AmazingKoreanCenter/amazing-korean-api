# AMK Memory Audit Report

> **기준 문서**: `docs/AMK_API_MASTER.md` (SSoT)
> **비교 대상**: `~/.claude/projects/.../memory/` (30개 토픽 파일 + MEMORY.md 인덱스)
> **작성일**: 2026-03-12

---

## 1. 삭제 권장 (API_MASTER와 완전 중복)

| 파일 | 이유 |
|------|------|
| `content_protection.md` | API_MASTER §8.6의 복사본. 파일 자체에 "AMK_API_MASTER.md §8.6에 정식 문서화 완료"라고 명시됨 |
| `ebook_web_viewer.md` | `ebook_viewer.md`의 불완전 요약판. 동일 Phase 12.5 내용이며 ebook_viewer.md가 더 상세 |

---

## 2. 모순 발견 (수정 필요)

| 파일 | 모순 내용 | 올바른 정보 (API_MASTER 기준) |
|------|----------|----------------------------|
| `i18n_multilingual_plan.md` | 아랍어(ar) 포함 22개 언어로 기재 | 실제 구현: 아랍어 없이 21개 언어 + EN = 22언어. TextbookLanguage enum에 아랍어 없음 |
| `textbook_tasks.md` | ISBN 번호 오래된 정보 (VN 교사용 979-11-997727-0-0 등) | `isbn_imprint.md`/`cover_system.md` 확정본과 불일치 (0-0=영어 학생용, 5-5=베트남어 학생용) |

---

## 3. 부분 중복 (PARTIAL) — 고유 정보 포함, 유지 가치 있음

| 파일 | API_MASTER 겹침 | 고유 정보 |
|------|----------------|----------|
| `MEMORY.md` | §8.1/8.2/8.3 결제/e-book/교재 주문 | 법인 행정 작업, Mac Mini AI 인프라, Figma 캡처 현황, CEO 영문 이름, Filipino(tl) 미적용 상세 |
| `articulation_animation_research.md` | §5.15 Phase 15 한 줄 | 15-17개 다이어그램 상세, CC0 SVG 출처, GSAP MorphSVG 기술 스택, 경쟁 도구 분석 |
| `content_design.md` | §5.13 Phase 13 | JSON→DB 계층 매핑, Study Task 종류, 오답 생성 전략, 영상 제작 전략, 발음 평가 3단계 |
| `ebook_distribution.md` | §8.6 콘텐츠 보호 일부 | 플랫폼별 수수료/정산, Amazon KDP 한국어 전략, DRM 방식 비교 |
| `ebook_viewer.md` | §5.12 E-book 뷰어 Phase | 파일 구조, API 엔드포인트 상세, Paddle 연동, Config 환경변수, QA 피드백 11건 |
| `education_methodology.md` | §8.6 교육 방법론 일부 | 500문장 근거, 200시간 TOPIK 달성 이유, 22개 언어 확장 설계 원칙, 판매 채널 전략 |
| `i18n_multilingual_plan.md` | §4.4 content_translations | 22개 언어별 스크립트 분류, Noto Sans 동적 로딩, RTL UI 설계, 번역 파이프라인 |
| `login_table_plan.md` | §4.1 login/login_log 테이블 | 미사용 컬럼 6개 분석, 구현 6단계 계획, woothee 라이브러리 (상태: 구현 완료) |
| `pronunciation_ai_research.md` | §5.14 Phase 14 한 줄 | ETRI/SpeechSuper/Azure 비교, 비용 시뮬레이션, wav2vec2, AIHub 데이터셋, L2 오류 패턴 |

---

## 4. 완전 고유 (API_MASTER에 없는 정보)

대부분 **교재 빌드/출판 파이프라인** 관련. API_MASTER는 "웹 서비스 API 스펙" 중심이라 이 도메인을 포함하지 않음.

### 교재 빌드 시스템 (14개)
| 파일 | 내용 |
|------|------|
| `cover_system.md` | 표지 자동 생성 (22언어x2에디션=44개, 스프레드 규격, 국기 SVG, ISBN 바코드+QR) |
| `css_audit_and_fix.md` | Phase 5 Puppeteer CSS 감사 (getComputedStyle 비교, 387→1 diff) |
| `en_edition.md` | EN 에디션 전용 CSS/컴포넌트 (buildEnEditionCSS, body class, 6개 컴포넌트) |
| `epub_system.md` | EPUB3 FXL 빌드 파이프라인 (viewport 1190x1684, CSS 후처리 10단계, 44개 EPUB) |
| `guide_edition.md` | 해설용 교재 설계 (3개국어 해설, 300-400p, guide_filter.js) |
| `isbn_imprint.md` | 판권지 제목 규칙, 학생용 ISBN 10개 확정 |
| `isbn_title.md` | ISBN 심사 규칙, 반려 원인, 플랫폼별 제목 규칙 (Amazon KDP/Google/Apple) |
| `student_edition.md` | 학생용 에디션 필터 (student_filter.js 7개 필터, 전용 CSS) |
| `tense_table_update.md` | 시제 테이블 번호/단어 순서 변경 (p26-p35, 80단어) |
| `textbook_rebuild_lessons.md` | Phase별 시행착오 플레이북, 반복 근본 원인 4패턴, 검증 체크리스트 |
| `textbook_tasks.md` | 페이지별 구조(45 wrapper, 120p), 7파트 색상, 인쇄 사양 |
| `textbook_translation.md` | Phase 7 번역 워크플로우 (추출→번역→검수→병합, 16언어x923항목) |
| `translation_verification.md` | 역번역 기반 검증 (20개 언어 평균 4.72/5.0, Critical 3건) |
| `ebook_pdf_fixes.md` | Phase 5-6 PDF 비교 (8개 체계적 패턴, 120페이지 전수 비교) |

### 향후 계획 (2개)
| 파일 | 내용 |
|------|------|
| `ebook_viewer_improvement_plan.md` | 구매코드 형식 변경, 4중 비가시적 워터마크, Cache-Control/프리페치 |
| `ebook_market_research.md` | 한국어 교재 시장 분석, TOPIK 통계 55만명, EPS-TOPIK 쿼터, ROI 분석 |

### 리서치 (2개)
| 파일 | 내용 |
|------|------|
| `articulation_animation_research.md` | 조음 애니메이션 다이어그램/기술 조사 (부분 중복이지만 고유 비율 높음) |
| `pronunciation_ai_research.md` | 발음 평가 AI 플랫폼 비교/비용 분석 (부분 중복이지만 고유 비율 높음) |

### 기타 (2개)
| 파일 | 내용 |
|------|------|
| `db_encryption_phase2_plan.md` | 암호화 Phase 2 상세 계획, 8개 버그, 백필 전략 |
| `db_security_audit.md` | 보안 감사 결과, 암호화 방식 결정 과정, Blind Index 설계 근거 |
| `debugging.md` | Docker 환경변수 동기화, SQLx 타입 매칭 등 운영 지식 |

---

## 5. MEMORY.md 인덱스 상태

| 항목 | 현재 | 권장 |
|------|------|------|
| 줄 수 | 295줄 | 150줄 이하 (시스템 200줄 제한, 하단 95줄 잘림) |
| 법인 행정 작업 | 35줄 차지 | 별도 파일로 분리 또는 제거 |
| 완료된 작업 changelog | ~25줄 | 제거 (git history에 존재) |
| 프로젝트 구조 | ~10줄 | 제거 (CLAUDE.md와 중복) |
| 빌드 명령어/디렉토리 트리 | ~30줄 | 토픽 파일로 이동 |

---

## 6. 권장 조치 요약

| 우선순위 | 조치 | 대상 |
|:--------:|------|------|
| **즉시** | 삭제 | `content_protection.md`, `ebook_web_viewer.md` |
| **즉시** | 모순 수정 | `i18n_multilingual_plan.md` (아랍어 제거), `textbook_tasks.md` (ISBN 업데이트) |
| **높음** | MEMORY.md 축소 | 150줄 이하로 (법인 행정/changelog/중복 구조 제거) |
| **보통** | 완료된 계획 정리 | `login_table_plan.md`, `db_security_audit.md`, `db_encryption_phase2_plan.md` (아카이브 표시 또는 삭제) |
| **낮음** | 교재 빌드 메모리 통합 | 14개 교재 관련 파일 중 유사 주제 병합 검토 |
