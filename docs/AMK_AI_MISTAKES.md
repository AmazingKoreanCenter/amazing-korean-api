# AMK_AI_MISTAKES — AI (Claude) 작업 실수 기록

> **목적**: AI 작업 중 발생한 실수를 사실 기반으로 기록. 작업 시 사전 참조하여 같은 실수 회피.
> **작성 원칙**: 사실만 기재. 가정·해석·"왜 그랬을까" 류 추측 배제. 원인 = 직접 관찰 가능한 행위. 결과 = 직접 관찰 가능한 영향.
> **참조 시점**: 모든 작업 시작 전 본 문서 확인. 메모리 `feedback_ai_mistakes_reference.md` 가 본 문서를 참조하라고 지시.
> **production 인시던트 (INC-NNN)**: 별도 — `AMK_DEPLOY_OPS.md` + `AMK_CHANGELOG.md` 참조. 본 문서는 **AI 실수만**.

---

## 사고 카탈로그

### M-001 — stale 메모리만 보고 origin 상태 미확인

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오전, 세션 시작 직후) |
| AI 행위 | 메모리 `project_status.md` (2026-04-30 기준) 만 읽고 "unblocked 작업 없음" 결론 출력 |
| 사용자 응답 | "내가 잘 이해가 안가. 무슨 의도로 이렇게 답변한거야??" |
| 누락 행위 | `git fetch` + `gh pr list --state all` 미실행. origin 의 PR #194~#199 진행 미인지 |
| 결과 | 사용자가 직접 정정 요청. 추가 조사 후 재답변 |

---

### M-002 — books 핸드오프 프롬프트에 도메인 혼재

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오전) |
| AI 행위 | books 세션용 핸드오프 프롬프트 작성 시 "UI 용어 번역 (518 keys 갭)" 과 "학습 콘텐츠 작업" 을 같은 섹션에 기재 |
| 사용자 응답 | "UI 용어 번역이야 아니면 학습콘텐츠 번역 내용이야?" |
| 누락 행위 | 메모리 `project_q13_briefing.md` 의 "content_translations(학습 콘텐츠) ≠ i18n locale(UI)" 명시를 사전 참조 안 함 |
| 결과 | 사용자가 도메인 분리 명시 지시. `feedback_translation_domain_separation.md` 신규 메모리 + 핸드오프 재작성 |

---

### M-003 — git reset --hard 시 미커밋 변경 폐기

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오전) |
| AI 행위 | INC-005 fix 작업 진입 시 KKRYOUN 동기화 절차로 `git reset --hard origin/main` 실행 |
| 누락 행위 | reset 전 `git status` 확인 안 함. 환경 정보의 세션 시작 git status (`M docs/AMK_CHANGELOG.md`, `M docs/AMK_EBOOK_SECURITY.md`, `M docs/AMK_STATUS.md`) 무시 |
| 결과 | 미커밋 변경 3개 파일 working tree 에서 폐기. `AMK_EBOOK_SECURITY.md` = 사용자 e-book 보안 연구 노트 (TOONRADER 벤치마크) |
| 복구 경로 | git reflog/fsck = 미커밋 추적 안 함, 복구 X. VSCode local history = 검색 결과 없음. 다른 세션이 메모리 `reference_toonrader.md` (생존) + 대화 컨텍스트 기반으로 재작성 → 본 세션 즉시 commit (`00a7ec0`) 으로 보호 |
| 사용자 응답 | "AMK_EBOOK_SECURITY.md말고 유실된게 더 있어?" / "왜 유실된거야??" |

---

### M-004 — pr-check workflow 도입 시 main baseline 미검증

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오후) |
| AI 행위 | `.github/workflows/pr-check.yml` 신규 작성 후 push. push 가 self-test 트리거 |
| 누락 행위 | 워크플로 도입 전 main HEAD 의 `cargo clippy --lib --bins --locked -- -D warnings` + `cd frontend && npm run lint` 통과 여부 사전 검증 안 함 |
| 결과 | self-test 첫 실행 = backend (cargo clippy) FAILURE 1건 (`useless_conversion`, `src/api/auth/service.rs:192`. Rust 1.95 신규 룰) + frontend (npm run lint) FAILURE 27 errors (대부분 `react-refresh/only-export-components`, shadcn/ui 컴포넌트 + variants 같은 파일 export 패턴) |
| 사용자 응답 | "이건 뭐야 지금??" |
| 후속 | clippy 1줄 fix + lint 단계 `continue-on-error: true` 임시 적용 + Q16 신규 (baseline cleanup 큐) |

---

### M-006 — `cargo fmt --check --all` 결과 의미 잘못 해석

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오후, pr-check.yml 도입 검증 시) |
| AI 행위 | 로컬 `cargo fmt --check --all` 실행 후 출력에 ANSI 색 코드 잔여물 + diff 라인 (`}` + `+`) 이 있었으나 `exit=0` 만 보고 "통과" 로 사용자 보고 |
| 누락 행위 | exit code + 출력 둘 다 검증 X. ANSI 색 잔여물로 잘못 해석 |
| 결과 | pr-check.yml 도입 후 첫 self-test 시 `cargo fmt --check --all` FAILURE. CI 환경에서 crates/crypto 다수 파일 unformatted 검출 |
| 사용자 응답 | (PR #205 두 번째 fail 후 보고) |
| 후속 | M-005 와 같은 카테고리 (추정 단정) 의 새 발현 — 검증 결과의 의미 잘못 읽음 |

### M-007 — 다른 문서 라인 번호 직접 검증 X

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (저녁, AMK_DEBTS.md 작성 시) |
| AI 행위 | `docs/AMK_DEBTS.md` 작성 시 `AMK_STATUS.md §8.2 검증된 리스크 표` + 어제 grep 결과의 라인 번호를 **직접 grep 검증 없이 복사** |
| 누락 행위 | 본 문서 작성 시점 HEAD 기준 grep 으로 라인 번호 사실 확인 X. 원본 `AMK_STATUS.md` 도 stale 였음 |
| 결과 | 5 agent 정합성 검증 시 라인 번호 다수 stale 발견 (deploy.yml:87-98 → 92-103 / ebook/service.rs 8곳 모두 stale / config.rs:97,101,325 모두 stale / auth/service.rs:397,1396 → 358,1123 / video/repo.rs:237 → 233 등). 부채 카운트 자체 (B3 npm 2 → 3) 도 정정 필요 |
| 사용자 응답 | "현재 프로젝트 상태와 비교해 다시 검증" |
| 후속 | 본 문서 라인 번호 정정 + AMK_STATUS §8.2 정정 + AMK_DEBTS 의 라인 번호 사용 정책 명시 ("HEAD 기준, 사용 시 grep 재확인") |

### M-005 — lint:ui 결과 추정으로 단정

| 항목 | 내용 |
|------|------|
| 발생일 | 2026-05-04 (오후, M-004 직후) |
| AI 행위 | M-004 의 frontend job 분석 시 "lint:ui (하드코딩 색상 검사) 는 PASS" 라고 사용자에게 출력 |
| 누락 행위 | 실제 데이터 = lint step (이전 step) 이 fail 해서 후속 step (lint:ui) 실행 자체가 안 됨 (set -e). lint:ui 결과는 unknown 이었으나 "PASS" 로 단정 |
| 결과 | M-004 fix 후 재 self-test 시 lint:ui 가 실제로 fail. 9 곳 하드코딩 색상 검출 (`textbook_order_page.tsx` 4곳 / `receipt_parts.tsx` 1곳 / `book_hub_page.tsx` 4곳 / `HangulKeyboardKey.tsx` 1곳). frontend job FAILURE 재발 |
| 사용자 응답 | "왜 자꾸 실수하지?? 원인을 제시해" / "이런 현상이 계속 발생하는 근본적인 원인이 뭐야?" |

---

## 카테고리 분포 (M-001 ~ M-007)

| 카테고리 | 사고 ID | 공통 행위 |
|----------|---------|----------|
| 사전 상태 미확인 | M-001, M-003, M-004 | 작업 시작 전 현재 상태 (origin / git status / baseline) 확인 누락 |
| 추정을 사실로 단정 | M-002, M-005, M-006, M-007 | 데이터 X 또는 검증 X 인 항목을 결과 단정 형태로 출력 (M-006 = 검증 결과의 의미 잘못 해석, M-007 = 다른 문서 라인 번호 복사) |

---

## 갱신 규칙

- 신규 사고 발생 시 본 문서에 새 M-NNN 엔트리 추가
- 엔트리 내용 = 사실만. 가정·해석·"왜 발생했나" 류 추측 배제
- 원인 = "AI 가 무엇을 했는가 / 무엇을 안 했는가" 행위 단위
- 결과 = 직접 관찰된 영향 (사용자 응답 / 데이터 손실 / fail 등)
- 본 문서 갱신 시 메모리 갱신 불필요 (메모리는 본 문서를 참조하라는 포인터만)
