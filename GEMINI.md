# Amazing Korean – Backend Context (Axum + SQLx)

## Goal
- Rust + Axum 기반 API 서버의 기능 확장과 리팩터링을 Gemini CLI와 함께 수행.
- 우선 과제: `GET /users/me`, `PUT /users/me` 구현(+ JWT), Swagger(/docs) 정합성.

## Stack
- Rust (Axum 0.8, Tokio, SQLx(PostgreSQL), utoipa/Swagger)
- DB: PostgreSQL (도커 컨테이너: `amk-pg`, 포트 5432)
- 실행: `cargo run` (bind: 0.0.0.0:3000), OpenAPI: `/api-docs/openapi.json`, Swagger UI: `/docs`

## Current APIs
- `GET /healthz` 헬스체크
- `GET /courses`, `POST /courses` (DTO/에러 규칙 준수)

## Conventions
- 에러: `AppError` → 400/401/403/404/409/500 매핑, `IntoResponse`
- 인증: JWT HS256 (`JWT_SECRET`, `JWT_EXPIRE_HOURS`) – **미구현/이번 태스크에서 처리**
- 사용자: `users.email` UNIQUE 위반(23505) → 409
- 문서화: utoipa v5, 보안 스키마는 Bearer

## Files (참조용 앵커)
- @./src  ← 소스 전체
- @./Cargo.toml
- @./openapi/README.md  （있다면）
- @./docs/ERD.png       （있다면）
- @./.env.example

## Tasks for Gemini
1) `/users/me`(GET): Bearer 토큰 파싱 → user_state=="on"만 200(프로필), 아니면 401/403.
2) `/users/me`(PUT): 닉네임/언어/국가/생일/성별 업데이트. DTO-검증-서비스-리포지토리 계층 분리.
3) Swagger(/docs) 스펙 반영 및 예제 응답 추가.
4) 단위 테스트(서비스/리포) & 핸들러 통합 테스트.

## Done = Definition
- 핸들러/서비스/리포/DTO 구현 + 테스트 통과
- OpenAPI 스펙에 엔드포인트/스키마 반영
- 문서: 변경사항 요약, 마이그레이션/ENV 안내 업데이트

