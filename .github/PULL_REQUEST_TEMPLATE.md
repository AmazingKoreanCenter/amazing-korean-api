# PR 템플릿

## 변경 사항

<!-- 1-3 줄 요약. "왜" 중심 (무엇은 diff 가 보여줌) -->

## 부채 / 트랙

<!-- 해당 항목 체크 + N-NNN / X-N 표기 -->

- [ ] AMK_AUDIT N-NN 처리: 
- [ ] AMK_DEBTS X-N 처리: 
- [ ] AI 사고 등재 (M-NNN): 
- [ ] 기능 / 개선 / 버그 fix (부채 외): 

## 검증

<!-- 백엔드 / 프론트엔드 / 인프라 각 해당 항목 체크 -->

**백엔드 (Rust)**:
- [ ] `cargo check --workspace` exit=0
- [ ] `cargo fmt --check --all` exit=0 (M-006/M-008 회피)
- [ ] `cargo clippy --lib --bins --locked -- -D warnings` exit=0
- [ ] (마이그 시) `cargo sqlx prepare --check` exit=0

**프론트엔드 (frontend 변경 시)**:
- [ ] `cd frontend && npm run build` 통과 (tsc + vite)
- [ ] 응답 schema 변경 시 백엔드 dto.rs 와 frontend types.ts 동시 변경 (Zod parse 호환성)

**인프라 / 배포 (해당 시)**:
- [ ] `deploy.yml` heredoc + `.env.example` + `config.rs` 3중 동기화 (INC-001 패턴 회피)
- [ ] production 영향 (자동 배포 / 마이그 적용 / nginx 재시작 / 등) 명시

## SSoT 갱신

- [ ] `docs/AMK_DEBTS.md` 처리 마킹 + 카운트 (해당 시)
- [ ] `docs/AMK_AUDIT_2026-05-04.md` N-NNN 마킹 (해당 시)
- [ ] `docs/AMK_AI_MISTAKES.md` M-NNN 등재 (사고 발생 시)
- [ ] `docs/AMK_CHANGELOG.md` entry 추가
- [ ] 메모리 `project_status.md` 갱신 (필요 시)

## 머지 후 모니터링

<!-- 필요 시 — 예: dependabot PR / Cloudflare 배포 / DB 마이그 / 외부 서비스 영향 -->
