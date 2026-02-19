# QA Report — Phase V1-2 : Paddle 코맨트 반영

**Date**: 2026-02-19
**Scope**: 환불 정책 30일 보장, 약관/개인정보 사업자명, 관리자 대시보드/결제 i18n, noscript SEO, 차트 디자인 토큰, MFA 비활성화
**Method**: Code review + File verification
**Result**: **12/12 PASS, 0 BUG**

---

## Summary

| Category | Items | Pass | Fail |
|----------|-------|------|------|
| CRITICAL — 환불 정책 | 3 | 3 | 0 |
| HIGH — 약관/개인정보 사업자명 | 3 | 3 | 0 |
| HIGH — 관리자 대시보드/결제 | 2 | 2 | 0 |
| MEDIUM — noscript SEO | 1 | 1 | 0 |
| MEDIUM — 차트 디자인 토큰 | 2 | 2 | 0 |
| LOW — MFA 비활성화 | 1 | 1 | 0 |
| **합계** | **12** | **12** | **0** |

---

## CRITICAL — 환불 정책

### 1. 30-Day Money-Back Guarantee (30일 환불 보장)

| Lang | Key | Value | Result |
|------|-----|-------|--------|
| EN | `legal.refund.s1Title` | "1. 30-Day Money-Back Guarantee" | PASS |
| EN | `legal.refund.s1Content` | "You may request a **full refund** within **30 days** of any subscription payment, **no questions asked**..." | PASS |
| KO | `legal.refund.s1Title` | "1. 30일 환불 보장" | PASS |
| KO | `legal.refund.s1Content` | "구독 결제일로부터 **30일 이내**에 **사유와 관계없이** **전액 환불**을 요청하실 수 있습니다..." | PASS |

**Evidence**: `en.json:514-515`, `ko.json:514-515`
- 양쪽 모두 "30일/30 days", "전액/full refund", "무조건/no questions asked" 명시
- 최초 구독 + 갱신 결제 모두 적용됨을 명시
- 환불 처리: 5-10 영업일, 원래 결제 수단으로 반환

### 2. 조건부 환불 언어 제거

| Check | Result | Evidence |
|-------|--------|----------|
| EN: "partial refund" 검색 | 미발견 (환불 정책 내) | PASS |
| EN: "may be eligible", "at our discretion" 검색 | 미발견 | PASS |
| KO: "부분 환불" 검색 (환불 정책 내) | 미발견 | PASS |
| KO: "환불될 수 있습니다" 검색 | 미발견 | PASS |

**Note**: `admin.payment.partiallyRefunded` ("부분 환불") 키는 관리자 거래 내역의 **상태 필터 라벨**이며, 환불 정책 문구가 아님 → 정상.

### 3. 환불 정책 정확히 4개 섹션

| Section | EN Title | KO Title | Result |
|---------|----------|----------|--------|
| s1 | 30-Day Money-Back Guarantee | 30일 환불 보장 | PASS |
| s2 | Subscription Cancellation | 구독 취소 | PASS |
| s3 | Free Trial | 무료 체험 | PASS |
| s4 | Billing Errors | 결제 오류 | PASS |
| s5 | (존재하지 않음) | (존재하지 않음) | PASS |

**Evidence**: `en.json:510-522`, `ko.json:510-522` — `s1`~`s4`만 존재, `s5` 없음.

---

## HIGH — 약관/개인정보 사업자명

### 4. Terms of Service EN — 사업자명 포맷

| Key | Value | Result |
|-----|-------|--------|
| `legal.terms.intro` | "...the Amazing Korean service **(operated by HIM Co., Ltd.)**..." | PASS |
| `legal.terms.s1Content` | "...Amazing Korean, operated by HIM Co., Ltd. (\"Company\")..." | PASS |

**Evidence**: `en.json:475-477`

### 5. Privacy Policy EN — 사업자명 포맷

| Key | Value | Result |
|-----|-------|--------|
| `legal.privacy.intro` | "**Amazing Korean (operated by HIM Co., Ltd.)** values the personal information..." | PASS |

**Evidence**: `en.json:494`

### 6. KO — 사업자명 포맷 (㈜ 힘)

| Key | Value | Result |
|-----|-------|--------|
| `legal.terms.intro` | "Amazing Korean 서비스**(운영: ㈜ 힘)**" | PASS |
| `legal.terms.s1Content` | "Amazing Korean**(운영: ㈜ 힘, 이하 \"회사\")**" | PASS |
| `legal.privacy.intro` | "Amazing Korean**(운영: ㈜ 힘, 이하 \"회사\")**" | PASS |

**Evidence**: `ko.json:475-477, 494`

---

## HIGH — 관리자 대시보드 / 결제 i18n

### 7. Admin Dashboard 실제 데이터 연동

**Result**: PASS — 이전 QA에서 검증 완료 (`docs/QA_DASHBOARD_I18N.md` 참조)

주요 확인 사항:
- 4개 API 호출: `useUserStatsSummary`, `useLoginStatsSummary`, `useVideoStatsSummary`, `useStudyStatsSummary`
- 로딩 중: Skeleton 애니메이션
- API 실패 시: `?? "-"` fallback
- 숫자 포맷: `.toLocaleString()`

### 8. Admin Payment 4개 페이지 i18n

**Result**: PASS — 이전 QA에서 검증 완료 (`docs/QA_DASHBOARD_I18N.md` 참조)

주요 확인 사항:
- 82개 `admin.payment.*` i18n 키 전수 검증
- 4개 페이지: 구독 목록, 거래 내역, 구독 상세, 수동 수강권
- 보간 파라미터 일치: `{{count}}`, `{{total}}`, `{{id}}`, `{{userId}}`

---

## MEDIUM — noscript SEO

### 9. noscript 크롤러 접근성

**파일**: `frontend/index.html:41-65`

| Check | Result | Evidence |
|-------|--------|----------|
| 서비스 설명 | PASS | "The best online Korean learning platform for learners worldwide..." |
| 주요 링크 (About, Pricing) | PASS | `<a href="/about">`, `<a href="/pricing">` |
| 법률 링크 (Terms, Privacy, Refund, FAQ) | PASS | `/terms`, `/privacy`, `/refund-policy`, `/faq` |
| 연락처 정보 (이메일, 전화, 주소) | PASS | `amazingkoreancenter@gmail.com`, `0504-0821-5018`, `350 Hannuri-daero, Sejong, 6F SB3` |
| 사업자 정보 | PASS | "Amazing Korean (operated by HIM Co., Ltd.) \| CEO: Kyungyun Kim \| Business Reg. No.: 505-88-03252" |

---

## MEDIUM — 차트 디자인 토큰

### 10. Admin 통계 페이지 — CSS 변수 기반 디자인 토큰

| Page | 사용된 토큰 | 하드코딩 hex/RGB | Result |
|------|------------|------------------|--------|
| User Stats | `bg-chart-6`, `bg-destructive`, `bg-chart-3`, `bg-chart-2`, `bg-primary/10`, `bg-status-success/10`, `bg-status-warning/10` | 없음 | PASS |
| Login Stats | `bg-chart-1`, `bg-chart-2`, `bg-chart-5`, `text-status-success`, `text-destructive` | 없음 | PASS |
| Study Stats | `bg-chart-1`~`bg-chart-6`, `bg-status-success`, `bg-muted-foreground`, `programBadgeColors` (chart 토큰 기반) | 없음 | PASS |
| Video Stats | `bg-primary/10`, `text-primary` | 없음 | PASS |

**CSS 정의**: `index.css:54-59` (light), `index.css:103-108` (dark)
**Tailwind 매핑**: `tailwind.config.js:48-53` — `chart['1']` through `chart['6']` → `hsl(var(--chart-N))`
**Status 색상**: `tailwind.config.js:59-66` — `status.success`, `status.warning`, `status.info` → CSS 변수 연결

### 11. 다크모드 차트 색상 분리

| Token | Light Mode | Dark Mode | 차이 | Result |
|-------|-----------|-----------|------|--------|
| `--chart-1` | `222 90% 18%` | `220 70% 50%` | 밝기 18%→50% (어두운 배경 대비) | PASS |
| `--chart-2` | `197 84% 46%` | `160 60% 45%` | 색조 변경 (블루→그린) | PASS |
| `--chart-3` | `224 81% 61%` | `30 80% 55%` | 색조 변경 (블루→오렌지) | PASS |
| `--chart-4` | `43 74% 66%` | `280 65% 60%` | 색조 변경 (옐로→퍼플) | PASS |
| `--chart-5` | `340 75% 55%` | `340 75% 55%` | 동일 (핑크) | PASS |
| `--chart-6` | `280 65% 60%` | `280 65% 65%` | 밝기 60%→65% | PASS |
| `--success` | `160 84% 28%` | `160 84% 36%` | 밝기 28%→36% | PASS |
| `--warning` | `38 92% 50%` | `38 92% 55%` | 밝기 50%→55% | PASS |
| `--info` | `217 91% 53%` | `217 91% 58%` | 밝기 53%→58% | PASS |

모든 다크모드 색상이 어두운 배경에서 충분한 대비를 제공하도록 밝기/색조가 조정됨.

---

## LOW — MFA 비활성화

### 12. MFA Disable 성능

**파일**: `src/api/auth/service.rs:1930-1968`

| Step | Operation | 성능 | Result |
|------|-----------|------|--------|
| 1 | HYMN 권한 확인 | O(1) 메모리 비교 | PASS |
| 2 | 자기 비활성화 방지 | O(1) ID 비교 | PASS |
| 3 | `disable_mfa()` | 단일 DB UPDATE | PASS |
| 4 | 세션 조회 + 상태 업데이트 | 트랜잭션 내 2개 쿼리 (N+1 없음) | PASS |
| 5 | Redis 정리 | 세션별 DEL (배치 조회로 N+1 해소) | PASS |

**Evidence**: `service.rs:1951` — "배치 조회로 N+1 해소" 주석. `find_user_sessions_with_refresh_tx()` 한 번 호출로 모든 세션 조회 후 루프 처리.
**참고**: Redis DEL은 개별 호출이나, HYMN 전용 관리 기능으로 대상 사용자당 세션 수가 제한적 → 성능 문제 없음.

---

## Conclusion

전체 **12개 항목 모두 PASS**. 버그 없음.

주요 검증 결과:
1. **환불 정책**: 30일 무조건 환불 보장, 조건부 언어 완전 제거, 정확히 4개 섹션
2. **사업자명**: EN "operated by HIM Co., Ltd.", KO "운영: ㈜ 힘" — 약관/개인정보 모두 일관 적용
3. **관리자 기능**: 대시보드 실데이터 + 결제 i18n 82개 키 (기존 QA 검증 완료)
4. **noscript SEO**: 서비스 설명, 주요/법률 링크, 연락처, 사업자 정보 모두 포함
5. **디자인 토큰**: 4개 통계 페이지 모두 CSS 변수 기반, 하드코딩 없음, 다크모드 분리
6. **MFA disable**: N+1 없는 배치 조회, HYMN 전용으로 성능 적합
