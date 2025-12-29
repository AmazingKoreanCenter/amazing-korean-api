# GEMINI 작업 가이드 (GEMINI.md)

> 업데이트: 2025-12-05
> 이 문서는 **docs/AMK_API_MASTER.md**와 **amk_schema_patched.sql**을 기반으로  
> `GEMINI_PROMPT_TEMPLATE.md` 프롬프트를 작성할 때 사용할 **요약 컨텍스트 & 작업 가이드**이다.

## 1. 이 문서의 역할

- **정식/최신 스펙의 단일 기준(Single Source of Truth)** 은 항상  
  `docs/AMK_API_MASTER.md` 와 `amk_schema_patched.sql` 이다.
- 이 파일(GEMINI.md)은 다음을 위해 존재한다:
  - LLM(Gemini)이 이해해야 할 **프로젝트 공통 규칙**을 압축 정리
  - 도메인별(USER, SETTINGS, ADMIN, VIDEO, STUDY, LIVE …)로  
    `GEMINI_PROMPT_TEMPLATE.md`의 **CONTEXT/CONTRACT를 채우기 위한 재료**를 제공
  - “어떤 섹션/Phase를 프롬프트에 포함해야 하는지”를 빠르게 찾는 **인덱스 역할**

## 2. 상위 문서와의 관계

- `docs/AMK_API_MASTER.md`
  - 엔드포인트/Phase/에러 정책/도메인 설명 등 **모든 스펙의 공식 문서**
  - 이 문서와 GEMINI.md, 코드/마이그레이션이 충돌할 경우 **AMK_API_MASTER.md를 우선**으로 한다.
- `amk_schema_patched.sql`
  - PostgreSQL 테이블/컬럼/ENUM/CHECK/UNIQUE/FK 등 **실제 DB 스키마 정의**
  - 마이그레이션 설계 시 항상 이 파일과 일치해야 하며, 변경 시 새 마이그레이션으로만 반영한다.
- `GEMINI_PROMPT_TEMPLATE.md`
  - 실제 패치 프롬프트의 **폼(form)** 을 정의하는 템플릿  
    (ROLE / OBJECTIVE / CONTEXT / CONTRACT / PATCH RULES / ACCEPTANCE / FILE PATCHES)
  - 이 GEMINI.md에 정리된 내용을 바탕으로, 각 섹션을 **채워 넣는 용도**로 사용한다.

## 3. 이 문서를 사용하는 기본 흐름

1. 수정/구현할 API/도메인(예: VIDEO A3, USER /users/me)을 정한다.
2. `docs/AMK_API_MASTER.md`에서 해당 Phase/섹션의 **정식 스펙**을 확인한다.
3. 이 GEMINI.md에서 해당 도메인 섹션의 **요약/표(Phase, HTTP, 검증, DB, 보안, 마이그레이션)** 를 참고한다.
4. `GEMINI_PROMPT_TEMPLATE.md`를 열어 CONTEXT/CONTRACT를
   - 공통 규칙(시간/에러/네이밍/마이그레이션 원칙) +  
   - 이 파일의 도메인별 요약 +  
   - 필요 시 AMK_API_MASTER의 일부 인용  
   으로 채운다.
5. FILE PATCHES에 코드/마이그레이션/문서(AMK_API_MASTER, amk_schema_patched.sql 포함 가능)를 나열한 뒤,  
   LLM 응답을 그대로 복붙하여 적용하고 `cargo fmt / clippy / check / sqlx migrate run`으로 검증한다.

## 4. **Writer 모드 선언**
   - “Gemini는 코드 생성/패치 담당(Writer)”
   - “범위 밖 변경/개선 제안은 금지(필요하면 코멘트만)”

## 5. **고정 실행 루프(매 Step 동일)**
   - 입력: ASK.md + AMK_API_MASTER 섹션 + LLM_PATCH_TEMPLATE
   - 출력: `LLM_PATCH_TEMPLATE` 형식의 PATCH 문서 1개
   - 적용 → fmt/check/clippy → curl → 종료

## 6. **가드레일**
   - allowed_paths 밖 파일 수정 금지
   - Change Budget 초과 금지
   - 재시도 max 2회

## 7. **“초단문 Writer 프롬프트” 블록(복붙용)**
   - 매번 긴 프롬프트를 쓰지 않도록,
   - GEMINI.md에 **짧은 고정 프롬프트(10~20줄)**를 넣어두기
   - 예시 구성:
     - “너는 Writer다”
     - “이 Step은 이 파일만”
     - “LLM_PATCH_TEMPLATE 형식으로만 출력”
     - “ACCEPTANCE 통과해야 함”

## 8. **로그 처리 규칙**
   - 실패하면:
     - “에러 로그 전체를 그대로 붙여넣고 1회 수정”
     - 2회 실패 시 중단 후 사람이 Step 분할