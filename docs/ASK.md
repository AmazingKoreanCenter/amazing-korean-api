[AMK][P5.0][Step 0] CLI 테스트 : health 파트

1) 대상 섹션:
- AMK_API_MASTER : 5.0 Phase 0 — health

2) 산출물 종류:
- LLM_PATCH_TEMPLATE

3) 하고 싶은 일:
- gemini cli, codex 작동 방식 테스트 및 비교
- 5.0 Phase 0 — health을 기준으로 새롭게 코드 작성

4) 현재 상태/제약:
- 이미 구현된 부분과 상관 없이 다시 코딩
- cargo check 에러 로그 있으면 붙여넣기
- src/api/health 폴더에 있는 파일만 수정
- 기존 구조 파일 그대로 사용 : src/api/mod.rs, src/config.rs, docs.rs, error.rs, main.rs, state.rs, types.rs

5) 포함할 파일/정보:
- 없음