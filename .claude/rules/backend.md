---
paths:
  - "src/**/*.rs"
---

# 백엔드 규칙 (Rust / Axum)

## 모듈 구조

각 도메인(`src/api/{domain}/`)은 동일한 파일 구조를 따른다:
- `dto.rs` — 요청/응답 DTO (Deserialize, Validate, ToSchema)
- `repo.rs` — SQLx 쿼리 (DB 접근만, 비즈니스 로직 없음)
- `service.rs` — 비즈니스 로직 (트랜잭션, 검증, 외부 호출)
- `handler.rs` — HTTP 핸들러 (파싱 → 서비스 호출 → 응답)
- `router.rs` — 라우트 정의 (미들웨어 바인딩)

## 핵심 파일

- `config.rs` — 환경변수 설정 SSoT, 환경변수 추가/변경 시 여기서 시작
- `state.rs` — AppState 의존성 컨테이너
- `error.rs` — 전역 에러 타입 (AppError → HTTP 응답 매핑)
- `types.rs` — DB enum ↔ Rust enum ↔ JSON 매핑
- `crypto/` — AES-256-GCM 암호화 + HMAC-SHA256 Blind Index
- `external/email.rs` — EmailSender trait + ResendEmailSender
- `external/payment.rs` — PaymentProvider trait + PaddleProvider

## 규칙

- 새 엔드포인트: dto → repo → service → handler → router 순서
- PII 필드: AES-256-GCM 암호화 저장 + Blind Index로 검색
- 인증코드: HMAC-SHA256 해시로 Redis 저장 (평문 금지)
- Anti-enumeration: 인증 응답에서 존재 여부 노출 금지
- Rate Limiting: 이메일 발송 실패 시 카운터 DECR 롤백
- 상세 패턴: `docs/AMK_CODE_PATTERNS.md` 참조
