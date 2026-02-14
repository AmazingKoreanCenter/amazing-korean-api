# 수동 테스트 체크리스트 (브라우저 필요)

**작성일**: 2026-02-11
**범위**: Phase 2 (자동 번역 UI) + Phase 3 (Tier 2~3 언어 확장) + 기존 CRUD 회귀

---

## A. 자동 번역 UI (Phase 2)

- [ ] **A-1** AutoTranslateDialog 렌더링 — 관리자 로그인 → 번역 관리 페이지 → "자동 번역" 버튼 클릭 → 다이얼로그 열림 확인
- [ ] **A-2** 5개 필드 입력 — content_type, content_id, field_name, source_text, target_langs 모두 정상 입력 가능한지
- [ ] **A-3** Select All 체크박스 — target_langs "전체 선택" → 20개 언어 일괄 선택/해제
- [ ] **A-4** 번역 실행 + 스피너 — 번역 실행 시 로딩 스피너 표시 → 완료 후 toast 메시지 + 목록 자동 갱신
- [ ] **A-5** Validation 에러 UI — source_text 비우고 제출 → 에러 메시지 표시, target_langs 0개로 제출 → 에러
- [ ] **A-6** 프로덕션 자동 번역 — 프로덕션 관리자 로그인 → 실제 콘텐츠 자동 번역 실행 → DB 반영 확인

## B. 기존 번역 CRUD UI 회귀 (Phase 1A)

- [ ] **B-1** 번역 목록 조회 — 번역 관리 페이지 → 목록 로딩, status 필터(draft/reviewed/approved) 동작
- [ ] **B-2** 번역 수동 수정 — 목록에서 항목 선택 → 텍스트 수정 → 저장 → 반영 확인
- [ ] **B-3** 상태 전이 — draft → reviewed → approved 순서 변경 가능한지
- [ ] **B-4** 번역 삭제 — 항목 삭제 → 목록에서 제거 확인

## C. 언어 전환 + 폰트 렌더링 (Phase 3)

- [ ] **C-1** 언어 드롭다운 전환 — 22개 언어 선택 → UI 텍스트 즉시 전환 + 새로고침 후 유지 (localStorage)
- [ ] **C-2** 특수 문자 체계 렌더링 — 아래 6개 언어에서 폰트 깨짐/tofu(□) 없는지 확인:
  - [ ] 미얀마어 (my) — Myanmar 문자 정상 표시
  - [ ] 크메르어 (km) — Khmer 문자 정상 표시
  - [ ] 싱할라어 (si) — Sinhala 문자 정상 표시
  - [ ] 힌디어 (hi) — Devanagari 정상 표시
  - [ ] 네팔어 (ne) — Devanagari 정상 표시
  - [ ] 태국어 (th) — Thai 문자 정상 표시
- [ ] **C-3** 긴 번역 레이아웃 — 독일어(de), 러시아어(ru)로 전환 → 버튼/메뉴 텍스트 잘림이나 overflow 없는지
- [ ] **C-4** CJK 폰트 — 일본어(ja), 중국어 간체(zh-CN), 중국어 번체(zh-TW) → Pretendard fallback으로 한자 정상 표시
- [ ] **C-5** 동적 로딩 체감 — 첫 언어 전환 시 chunk 로딩 지연이 체감되지 않는지 (네트워크 탭에서 locale chunk 확인)

## D. 콘텐츠 API 다국어 (통합)

- [ ] **D-1** 콘텐츠 번역 표시 — 언어 전환 후 코스/레슨/영상 목록에서 번역된 title/description이 표시되는지 (DB에 번역 데이터가 있는 경우)
- [ ] **D-2** Fallback 동작 — 번역 없는 언어 선택 시 한국어 원문으로 표시되는지

---

## 우선순위

1. **C-2** (특수 문자 렌더링) — Myanmar, Khmer, Sinhala는 시스템 폰트에 없을 수 있어 웹폰트 fallback 확인 필수
2. **A-4** (자동 번역 실행) — GCP v2 API 실제 호출 + DB 반영 E2E 확인
3. **A-6** (프로덕션 자동 번역) — 프로덕션 환경에서 GCP API Key 정상 동작 확인
4. **C-1** (언어 전환) — 22개 언어 전환 + localStorage 영속성
5. 나머지 항목

---

## 자동 QA 통과 현황 (CLI 검증 완료)

| Phase | 항목 수 | 결과 |
|-------|---------|------|
| Phase 2 (Google Translation 연동) | 36/36 | ALL PASS |
| Phase 3 (Tier 2~3 언어 확장) | 전체 | ALL PASS |
| 프로덕션 QA | 7/7 | ALL PASS |

**상세 리포트**: `docs/QA_PHASE2_TRANSLATION.md`
