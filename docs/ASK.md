[AMK][P5.0][Step 0] CLI 테스트 : health 파트

1) 대상 섹션:
- AMK_API_MASTER : 5.0 Phase 0 — health

2) 산출물 종류:
- (예시)
  - GEMINI_PATCH_PROMPT: 코드 패치용 프롬프트
  - DESIGN_NOTE: 설계/스펙 정리
  - SQL_MIGRATION: 마이그레이션 SQL

3) 하고 싶은 일:
- 한글로 구체적인 작업 목표 한두 문장
  (예: 로그인 API 전체 스펙을 5.1 기준으로 다시 정리하고,
       dto/handler/router 골격까지 만드는 PATCH 프롬프트 만들어줘)

4) 현재 상태/제약:
- 이미 구현된 부분 있으면 요약
- cargo check 에러 로그 있으면 붙여넣기
- 건드리면 안 되는 부분/파일 있으면 명시

5) 포함할 파일/정보:
- (필요하면) dto.rs, handler.rs, router.rs 전체 내용 붙여넣기
- 또는 “이 파일은 새로 생성” / “이 파일은 전체 교체” 같이 의도 설명