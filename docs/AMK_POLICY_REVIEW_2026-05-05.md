# 정책 검증 통합 결정 — N-31 / N-32 / N-35 / N-36 (2026-05-05)

> **대상**: amazing-korean-api `docs/AMK_AUDIT_2026-05-04.md` 의 정책 결정 필요 부채 4건
> **검증 방법**: 사용자 권고 + 2개 LLM 독립 검증 (Codex GPT-5.2 + Gemini 2.0 Flash)
> **결과**: **4건 모두 옵션 일치** (만장일치 또는 조건부 동의)

---

## 0. 요약

| 부채 | 사용자 권고 | Codex | Gemini | **합치 결정** |
|:--:|:--:|:--:|:--:|:--:|
| N-31 HSTS | A 또는 B | A (⚠️ 조건부) | A (✅) | **A** (HTTPS 선행 필수) |
| N-32 CSP | A | A (✅) | A (✅) | **A** (Report-Only 시작) |
| N-35 remaining_attempts | D 또는 A | D (✅) | D (✅) | **D** (1회 남음 시만 경고) |
| N-36 Validation 에러 | D | D (✅) | D (✅) | **D** (인증/비밀번호만 generic) |

**일치도 = 4/4건 (100%)**. 사용자 권고가 두 LLM 독립 검증에서 모두 통과.

---

## 1. 부채별 통합 결정

### 1.1 N-31 HSTS — 옵션 A (조건부)

**채택**: HTTPS (SSL+certbot, A4-1/A4-2) 활성 후 max-age 300s → 점진 증가 → preload 단계.

**선행 조건**:
- A4-1 nginx HTTPS 활성 (현재 SSL 블록 주석 처리)
- A4-2 Let's Encrypt + certbot 자동 갱신
- 모든 서브도메인 HTTPS 보장 (preload 전제)

**Cloudflare 처리 위치 결정 필요** (Gemini 추가 고려): edge 에서 HSTS 적용 vs origin 에서 적용. Cloudflare 사용 중이므로 edge 적용이 일반적이나 origin 도 함께 보내는 다중 layer 가 안전.

**작업 시점**: A4-1/A4-2 처리 후 (당분간 보류).

### 1.2 N-32 CSP — 옵션 A

**채택**: `Content-Security-Policy-Report-Only` 헤더 도입. 위반 로깅만, 차단 X. 1-2주 데이터 수집 후 enforce 결정.

**선결 사항** (Codex/Gemini 합치):
- report-uri / report-to endpoint 결정 (외부 서비스 vs 내부 endpoint)
- 결제 (Paddle) 플로우 + 로그인 (Google OAuth) 플로우 + 영상 (Vimeo) 재생 모두 테스트
- unsafe-inline 제거는 중장기 목표 (인라인 스크립트 외부화 또는 nonce 도입)

**작업 시점**: 즉시 가능 (외부 의존성 X).

### 1.3 N-35 remaining_attempts — 옵션 D

**채택**: 마지막 시도 직전 (1회 남음 시) 만 노출. 그 외 응답에서는 필드 제거 또는 generic 메시지.

**선결 사항** (Codex/Gemini 합치):
- 존재/미존재 계정 분기에서 동일 응답 형식 (anti-enumeration 정합)
- 429 응답의 `Retry-After` 헤더와 정합 (Gemini 지적)

**작업 시점**: 즉시 가능 (auth/dto.rs + service.rs 수정).

### 1.4 N-36 Validation 에러 — 옵션 D

**채택**: 인증/비밀번호 endpoint 만 generic, 나머지 details 유지.

**선결 사항** (Codex/Gemini 합치):
- 전역 `AppError::Validation` 만으로는 endpoint 구분 불가 → endpoint 별 매핑 기준 정의 필요 (Codex)
- validator 크레이트 에러 메시지 i18n 처리 로드맵 (Gemini, 중장기)

**작업 시점**: endpoint별 매핑 정의 후 (반나절 ~ 1일).

---

## 2. 진행 순서 (통합)

Codex 와 Gemini 의 진행 순서가 서로 다름:

- **Codex**: N-31 → N-32 → N-35 → N-36 (HTTPS 선행 강조)
- **Gemini**: N-32 → N-36 → N-35 → N-31 (즉시 가능 작업 우선)

**Gemini 순서가 본 환경에 더 적합** (HTTPS 활성 = A4-1/A4-2 의 외부 작업 부담 큼, 그동안 다른 3건 진행 가능):

| 순 | 부채 | 시간 | 의존 |
|:--:|------|:--:|------|
| 1 | **N-32** CSP Report-Only | 1-2h | 없음 |
| 2 | **N-36** Validation generic (인증 endpoint) | 0.5-1d | endpoint 매핑 정의 |
| 3 | **N-35** remaining_attempts 1회 남음 시만 | 0.5-1h | 없음 |
| 4 | **N-31** HSTS | — | A4-1/A4-2 선행 |

---

## 3. 추가 부채 후보 (LLM 제안, 본 검증 외)

Codex 가 종합 의견에서 추가 점검 권고:

- **쿠키 secure / sameSite / httpOnly** 정책 확인 (현재 `REFRESH_COOKIE_SECURE=true`, `REFRESH_COOKIE_SAMESITE=Lax` 설정 = OK 추정)
- **CSRF** 보호 (Refresh Cookie + state 패턴 = 현재 OAuth 만)
- **운영 로그의 개인정보 (PII) 노출** — 현재 `RUST_LOG=info` 전환됨 (N-37 처리), 추가 점검 가치

→ 본 부채 처리 후 별도 audit 트랙 등재 검토.

---

## 4. 출처 문서

- `docs/AMK_POLICY_REVIEW_2026-05-05_PROMPT.md` — 검증 입력 prompt
- `docs/AMK_POLICY_REVIEW_2026-05-05_CODEX.md` — Codex 답변 원본
- `docs/AMK_POLICY_REVIEW_2026-05-05_GEMINI.md` — Gemini 답변 원본
- `docs/AMK_AUDIT_2026-05-04.md` — 부채 N-NNN SSoT
- `docs/AMK_DEBTS.md` — 부채 카탈로그 SSoT
