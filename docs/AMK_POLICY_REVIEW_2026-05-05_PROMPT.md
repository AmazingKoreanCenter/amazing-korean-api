# amazing-korean-api 부채 정책 검증 요청

## 배경

- 온라인 한국어 학습 서비스 (Rust + Axum 백엔드 / React + Vite 프론트엔드)
- 프로덕션 = AWS EC2 단일 + Cloudflare + Resend (이메일) + Paddle (결제)
- 현재 HTTP-only (HTTPS / HSTS 미활성)
- 1인 CEO 겸 풀스택, production 영향 큰 변경은 보수적

## 요청

아래 4건 부채의 정책 옵션 + 내 권고에 대한 **독립적 두번째 의견**.

**출력 형식**: 응답 전체를 **하나의 완전한 markdown 문서** 로. 사용자가 그대로 `docs/` 에 저장할 수 있게 아래 양식 정확히 따라줘. 양식 외 인사말/사족/소제목 추가 X.

---

## 검증 대상 4건

### N-31 HSTS (Strict-Transport-Security)
- 위치: `src/main.rs` security_headers()
- 의존성: HTTPS 활성 (현재 HTTP-only)
- 옵션:
  - A. HTTPS (SSL+certbot) 활성 후 max-age 300s → 점진 증가 → preload
  - B. 지금 보류 (HTTPS 활성 후속)
  - C. HTTP-only 상태에서 HSTS 추가
- 내 권고: A 또는 B. SSL/HTTPS + certbot 선행 필수.

### N-32 CSP (Content-Security-Policy)
- 위치: `src/main.rs` security_headers() 또는 `frontend/index.html`
- 외부 의존성: Cloudflare Pages, Paddle.js, Google OAuth, Vimeo, 인라인 스타일
- 옵션:
  - A. Content-Security-Policy-Report-Only 헤더 (위반 로깅, 차단 X)
  - B. 보수적 enforce (`'self' 'unsafe-inline' 'unsafe-eval'`)
  - C. 엄격 nonce 기반 enforce
- 내 권고: A (report-only 부터). 1-2주 데이터 후 enforce.

### N-35 remaining_attempts 응답 노출
- 위치: `auth/dto.rs` FindPasswordRes / RequestResetRes / ResendVerificationRes
- 사실: rate limit 횟수를 사용자 응답에 직접 노출
- 옵션:
  - A. 필드 제거 (보안↑, UX↓)
  - B. 그대로 유지 (UX↑, 정보 노출)
  - C. bucket ("3회 이상" / "1-2회")
  - D. 마지막 시도 직전에만 (1회 남음 시)
- 내 권고: D 또는 A. anti-enumeration 정책 강하므로 D 가 정합.

### N-36 Validation 에러 노출
- 위치: `src/error.rs` AppError::Validation → details: { errors: e.to_string() }
- 사실: 모든 endpoint 의 422 응답에 필드명+룰 노출 (예: "email: invalid format, password: length must be >= 8")
- 옵션:
  - A. 그대로 유지 (UX↑, 룰 노출)
  - B. 필드명만 (룰 X)
  - C. generic 메시지
  - D. 인증/비밀번호 endpoint 만 generic, 나머지 details 유지
- 내 권고: D. 균형.

---

## 출력 양식 (아래 markdown 그대로 채워서 답변)

# 정책 검증 결과 — [LLM 이름 / 모델명] (2026-05-05)

> 대상: amazing-korean-api 부채 N-31 / N-32 / N-35 / N-36
> 검증자: [LLM 이름 / 모델명]
> 일자: 2026-05-05

---

## N-31 HSTS

| 항목 | 내용 |
|------|------|
| 추천 옵션 | A / B / C 중 1개 |
| 결론 | ✅ 동의 / ⚠️ 조건부 / 🟥 반대 (사용자 권고 대비) |
| 사유 (100자 이내) | ... |
| 추가 고려사항 | (선택, 100자 이내) |

## N-32 CSP

| 항목 | 내용 |
|------|------|
| 추천 옵션 | A / B / C |
| 결론 | ✅ / ⚠️ / 🟥 |
| 사유 | ... |
| 추가 고려사항 | ... |

## N-35 remaining_attempts

| 항목 | 내용 |
|------|------|
| 추천 옵션 | A / B / C / D |
| 결론 | ✅ / ⚠️ / 🟥 |
| 사유 | ... |
| 추가 고려사항 | ... |

## N-36 Validation 에러

| 항목 | 내용 |
|------|------|
| 추천 옵션 | A / B / C / D |
| 결론 | ✅ / ⚠️ / 🟥 |
| 사유 | ... |
| 추가 고려사항 | ... |

---

## 종합 의견 (200자 이내)

[전체 의견 / 우선순위 / 누락된 부채 카테고리]

---

## 진행 권고 순서

1. [N-NN — 옵션 X — 사유]
2. [N-NN — 옵션 X — 사유]
3. [N-NN — 옵션 X — 사유]
4. [N-NN — 옵션 X — 사유]

---

답변 = 위 markdown 양식 그대로 (백틱 또는 코드 블록 없이 본문 그대로). 다른 출력 X.
