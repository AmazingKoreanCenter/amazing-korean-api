# Q11 pt footer — API 팀 follow-up (2026-04-23)

> **발신**: `amazing-korean-ai/scripts/qa` (Mac Mini QA)
> **수신**: `amazing-korean-api` 팀
> **맥락**: 2026-04-22 handoff §2.3 에서 "별도 PR" 로 큐에 남긴 Q11 (pt footer 오버랩) 의 **후속 판단 요청**
> **근거 run**: `tests/qa-results/2026-04-22T06-39-53Z/` (full 22 lang × 전 라우트 × A+B+C)
> **이전 문서**: `docs/qa/api-team-sync-2026-04-22.md`, `docs/AMK_AI_QA_HANDOFF_2026-04-22.md`

---

## 0. 한 줄 요약

Q11 은 **진짜 버그인지 Gemma false positive 인지 불분명**한 상태. API 팀 판단이 필요합니다 — "고친다 / 안 고친다 / QA 쪽 prompt 로 흡수한다" 중 방향 확정 부탁.

---

## 1. 전체 검증 상황 (2026-04-22 full run 기준)

2026-04-22 handoff 에서 API 팀이 PR #182 로 수정한 3건 (2.1/2.2/2.4) 은 **전부 효과 확인**:

| 지표 | PR #182 전 | PR #182 후 overnight run | 변화 |
|---|---:|---:|---|
| Gemma 총 flag | 32 | **1** | -97% |
| Playwright fail | 87 (reporter 버그 포함) | 19 | 실제는 어제 기준 critical_path 12 (QA 쪽 fix 완료) + flaky 7 |
| Fuzz unhandled 5xx | 0 | 0 | ✅ |
| JWT 만료로 인한 bridge fail | 70 | 2 | Q12 (JWT_ACCESS_TTL_MIN=360) 효과 |

**남은 Gemma flag 1건 = 이 Q11**. 다른 모든 지표는 목표 달성.

---

## 2. Q11 증거 상세

### 2.1 Gemma 응답 원문 — 부분 hallucination 포함

`tests/qa-results/2026-04-22T06-39-53Z/ai_checks/pt/root_desktop_text_overflow.json`:

```json
{
  "flagged": true,
  "confidence": 0.95,
  "reason": "Text is overlapping/rendered behind another element at the bottom:
    '© 2016 Amazing Korean. Todos os direitos reservados.
     Termos de Uso Política de Privacidade'
    is overlapping with 'You are a web UI quality reviewer...'",
  "region": "bottom",
  "model": "gemma4:26b"
}
```

🚨 **중요**: reason 끝의 `"You are a web UI quality reviewer..."` 는 Gemma 에게 전달한 **프롬프트 첫 문장** (`ollama_check/prompts/text_overflow.md:1`). Gemma 가 자기 지시문을 "페이지 텍스트" 로 착각 — 해당 overlap 관계는 팩트상 존재 불가.

즉 reason 의 **절반은 hallucination**. 남은 주장은 "pt 페이지 bottom 영역의 copyright/legal 텍스트가 overlap 되어 보인다" 뿐이고, 이것만 검증 대상.

### 2.2 스크린샷 관찰

`tests/qa-results/2026-04-22T06-39-53Z/screenshots/pt/root_desktop.png` — 1440×desktop.

- 페이지 맨 아래 두 개의 bar:
  - 상단 bar: 회사정보 (HYMN Co. | CEO 등)
  - 하단 bar: `© 2026 Amazing Korean. Todos os direitos reservados.` (좌) + `Termos de Uso` + `Política de Privacidade` (우)
- 1440px 기준 좌·우 사이 **충분한 gap** 이 보여, **시각적으로 overlap 은 확인되지 않음**.
- 다른 locale (en/ja/ko 등) 의 같은 페이지 screenshot 과 비교해도 pt 에서만 특별히 크램프된 양상 없음.

### 2.3 관련 프론트 코드

`frontend/src/components/layout/footer.tsx:169-187` (Bottom Bar):

```tsx
<div className="flex flex-col md:flex-row justify-between items-center gap-4">
  <p className="text-footer-foreground/50 text-sm">
    {t("footer.copyright", { year: currentYear })}
  </p>
  <div className="flex items-center gap-6">
    <Link to="/terms">{t("footer.terms")}</Link>
    <Link to="/privacy">{t("footer.privacy")}</Link>
  </div>
</div>
```

- `md` (≥ 768px) 이상: 가로 배치, `justify-between` + `gap-4`
- `md` 미만: 세로 stack
- QA 는 1440 (desktop) + 375 (mobile) 캡처. 둘 다 "문제 없는 구간"
- **잠재적으로 문제 있을 구간**: **768–1023px (`md` 와 `lg` 사이)** — pt 카피라이트 문구가 길어서 링크와 거리가 좁아질 수 있음. 하지만 우리 viewport 매트릭스엔 이 구간이 없음.

즉 handoff 의 fix 후보 (a) `md:flex-row → lg:flex-row` 는 이 **768–1023px 구간을 세로 stack 으로 밀어 안전영역 확보** 하는 의도.

---

## 3. 판단 방향 — 3 시나리오

### 시나리오 A — 고친다 (handoff §2.3 후보 중 선택)

| # | fix | 영향도 |
|---|---|---|
| (a) | `md:flex-row` → `lg:flex-row` | 768–1023px 구간에서 footer bottom bar 가 세로 stack. md 뷰포트 (tablet) 데스크톱-like 경험이 약간 축소. 22 locale 전부 영향 |
| (b) | `flex-wrap gap-x-8 gap-y-3` 추가 | wrap 전략. `justify-between` 과의 상호작용 — pt 처럼 긴 텍스트는 자연 줄바꿈, 짧은 텍스트는 가로 유지. 브라우저별 wrap 처리 차이 가능 |
| (c) | pt 카피라이트 문구를 축약 | 법적 요건 / 브랜드 정책 검토 필요. 다른 long-copyright locale (fr/es?) 도 점검 필요 |

- QA 후속: `./run_qa.sh --skip-bringup --category A --lang pt` 로 pt 단독 ~3 분 재검증. 선택적으로 22 lang full run (2.5h).
- **고른 fix 에 따라 QA 매트릭스 viewport 추가 여부** 결정 필요:
  - (a) 채택 시 **tablet viewport (약 900px) 추가 권장** — 안 그러면 이번 fix 의 대상 구간이 QA 커버리지 밖이 됨.
  - (b)/(c) 는 1440 단독으로 검증 가능.

### 시나리오 B — 안 고친다 (QA 쪽 prompt 로 흡수)

- 이유: "스크린샷상 overlap 이 확인 안 되고 Gemma reason 절반이 hallucination. 실 사용자 영향 미미. 전역 footer 건드릴 리스크 대비 가치 낮음."
- QA 쪽 조치:
  1. `ollama_check/prompts/text_overflow.md` 보강 — "footer copyright + legal links 는 정상적인 조밀 배치일 수 있음. 명확히 가려지거나 잘린 경우만 flag" 류 guard 문구.
  2. 또는 `check_runner.py` 에 path+check 단위 whitelist (예: `pt/root_desktop/text_overflow` 또는 region="bottom" 조합) 지원 추가.
- **리스크**: 이후 실제 footer 회귀가 나도 Gemma 가 반응을 못 할 확률 ↑. "boy who cried wolf" 효과.

### 시나리오 C — 당분간 보류

- 현 상태 유지. 매 full run 에 pt 1 flag 재현. 무시.
- 장기적으로는 A 나 B 로 수렴해야 하지만, 이번 스프린트는 우선순위 낮음.

---

## 4. 판단에 필요한 맥락 (QA 쪽 참고정보)

- **Gemma 신뢰도 운영 실적**: overnight run 에서 Gemma 3515건 중 이 1건만 flag (flag rate 0.03%). 다른 flag 가 전부 PR #182 로 사라진 상태라, 남은 1건의 시그널 가치 비중이 상대적으로 큼.
- **Gemma false positive 누적 데이터**: 아직 충분치 않음 (MVP run 1회). prompt 보강으로 FP 억제는 정성적 개선이지, 수치적 근거는 없음.
- **viewport 매트릭스 공백**: 768–1023px 범위가 현재 QA 캡처에 없음. Handoff 에서 이 구간을 "Phase 2 tablet viewport" 로 남겨둔 상태. 시나리오 A-(a) 를 선택하시면 QA 쪽에서 tablet viewport 추가 작업이 동시에 붙습니다.

---

## 5. 체크박스 응답 요청

- [x] **시나리오 선택**: **(B) QA prompt 보강 + 조건부 whitelist** 채택
- [ ] ~~(A 선택 시) fix PR 머지 예정일~~ — 해당 없음 (B 선택)
- [ ] ~~(A-a 선택 시) QA 쪽 tablet viewport 추가~~ — 해당 없음, 다만 B 선택 후에도 tablet viewport 추가는 **권장** (§6.3 참조)
- [x] **(B) prompt 보강 / whitelist 선호 방식**: **prompt 보강 우선**, whitelist 는 보조. 상세 §6.2.

답 주시는 대로 QA 쪽 action item 진행하고 완료 통보드리겠습니다.

---

## 6. API 팀 답변 (2026-04-23)

### 6.1 판단 — 시나리오 B 채택

**결론**: footer 코드 **수정 안 함**. QA prompt 보강으로 흡수.

**근거**:
1. **실 버그 증거 부재** — 1440px 스크린샷에 overlap 시각적으로 확인되지 않고, 잠재 문제 구간 (768-1023px) 은 QA 뷰포트 매트릭스 밖이라 측정된 적이 없음. 존재·확인 모두 안 된 버그를 위해 전역 footer (22 locale × 모든 페이지) 를 건드리는 건 과잉.
2. **Gemma reason hallucination 실증** — 이 1건의 reason 절반이 `"You are a web UI quality reviewer..."` 프롬프트 첫 문장을 페이지 텍스트로 착각한 것. 구조적 FP 신호로 볼 수 있음. prompt 레벨 가이드가 적절.
3. **전역 footer 수정 리스크 > 가치** — A-a (`lg:flex-row`) 는 22 locale × 전 페이지 레이아웃 변경. A-b (`flex-wrap`) 는 `justify-between` 과의 상호작용이 브라우저별 차이 유발 가능. A-c (pt 문구 축약) 는 법적·브랜드 검토 선행 필요.
4. **Gemma FP 누적 데이터 부족** (MVP 1회) → prompt 개선은 정성적이지만 즉시 착수 가능한 저리스크 개선.

### 6.2 QA 쪽 권장 조치 (B 구현 방향)

**우선 — prompt 보강** (권장):
- `ollama_check/prompts/text_overflow.md` 에 가드 문구 추가:
  > "Footer copyright + legal links 가 조밀하게 배치된 경우는 정상적인 디자인 의도일 수 있습니다. 텍스트가 **명확히 가려지거나 잘린** 경우 (예: 한 요소가 다른 요소를 물리적으로 덮음, 글자가 컨테이너 밖으로 벗어남) 만 flag 하십시오. 단순히 요소들이 가깝게 있거나 줄바꿈된 것은 flag 하지 마십시오."
- footer 외 다른 영역에 대한 Gemma 감지력에는 영향 최소화 목표.

**보조 — whitelist** (필요 시):
- `check_runner.py` 에 path+check 단위 whitelist 지원 추가.
- 포맷 예: `{"path": "*/root_desktop", "check": "text_overflow", "region": "bottom"}` 매칭 시 경고로 격하.
- 초기에는 prompt 만 적용하고, 2-3 full run 동안 효과 관찰 후 필요 시 whitelist 도입.

### 6.3 Tablet viewport (768-1023px) 추가 권장

B 채택하더라도 **tablet viewport 추가는 별도로 권장**합니다:
- 768-1023 구간은 Gemma 감지 커버리지의 **구조적 공백**. footer 외에도 회귀 잠복 가능한 범위.
- A-a 선택 시에만 필요한 작업이 아니라, 전반적인 QA 품질 향상 목적으로 독립 진행 가치.
- B 와 별개 트랙으로 QA 쪽 착수 판단 부탁드립니다.

### 6.4 "양치기 소년" 리스크 대응 (Gemma 감지력 저하 관찰 plan)

Prompt 보강 후 2-3 full run 동안 다음을 관찰하여 효과/부작용 검증:

| 관찰 항목 | 기대 | 대응 |
|---|---|---|
| pt footer flag 소멸 | ✅ 기대 | 성공 |
| 다른 영역 (subtitle / carousel / text overlap) 의 실 회귀 발생 시 Gemma 가 여전히 flag 하는가 | 변함없이 감지 | 만약 감지력 떨어졌다면 prompt 재조정 or A-a 재고 |
| 신규 footer 회귀 (실제 CSS 버그) 발생 시 Gemma 가 감지하는가 | 감지 기대 (guard 문구가 "가려지거나 잘린 경우만" 이라 실 회귀는 통과) | 감지 실패 시 prompt 재조정 |

### 6.5 후속 확인

- QA 쪽에서 prompt 보강 + 다음 full run 결과 공유 부탁드립니다.
- 다음 run 에서 pt footer flag 가 소멸하고 다른 회귀 감지가 유지되면 B 성공으로 종결.
- 맥미니 → API 핸드오프 채널은 기존 방식 유지 (`docs/QA_결과.md` 갱신 또는 신규 follow-up 문서).

---

---

## 부록 — 지금까지 정리된 Q11 결정 히스토리

- **2026-04-22 handoff 생성 시점**: "footer breakpoint 또는 wrap 전략 변경은 전 언어 디자인 영향 → 별도 PR 검토" + "Gemma flag 유지돼도 무방" (API 팀 §2.3).
- **2026-04-22 PR #182**: 2.1 (ebook subtitle), 2.2 (textbook subtitle), 2.4 (캐러셀 aria-label) 만 포함. 2.3 (Q11) 제외.
- **2026-04-22 overnight full run**: Q11 만 잔존. 다른 flag 전부 해소.
- **2026-04-23 (본 문서)**: Gemma reason 의 hallucination 특성 확인 후 "진짜 버그 여부" 가 애매해져 판단 요청.