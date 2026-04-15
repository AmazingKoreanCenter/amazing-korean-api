# Amazing Korean API - Claude Code 프로젝트 가이드

## 프로젝트 개요

온라인 한국어 학습 서비스 **Amazing Korean**의 풀스택 애플리케이션.

- **백엔드**: Rust 2021 + Axum 0.8 + SQLx + PostgreSQL + Redis
- **프론트엔드**: React + Vite + TypeScript + shadcn/ui + TanStack Query
- **이메일**: Resend (AWS SES 폐기됨 - 프로덕션 승인 3회 거절)
- **인증**: JWT 액세스/리프레시 토큰 + Google OAuth 2.0
- **암호화**: AES-256-GCM + HMAC-SHA256 Blind Index
- **배포**: AWS EC2 (백엔드) + Cloudflare Pages (프론트엔드)
- **도메인**: amazingkorean.net (프론트), api.amazingkorean.net (백엔드)

## 참조 문서

작업 시작 전 관련 문서를 반드시 읽고 시작할 것.

### 핵심 문서 (항상 참조)

| 문서 | 역할 | 언제 읽나 |
|------|------|-----------|
| `docs/AMK_API_MASTER.md` | 공통 규칙, 시스템 환경, 데이터 모델 **(SSoT Core)** | 모든 작업 |
| `docs/AMK_CODE_PATTERNS.md` | 엔지니어링 원칙 + 백엔드/프론트엔드 코드 패턴 | 새 기능 구현 시 |
| `docs/AMK_DEPLOY_OPS.md` | 배포, 운영, CI/CD, 인프라 | 배포/환경 관련 작업 시 |
| `docs/AMK_SCHEMA_PATCHED.md` | DB 스키마 (테이블, 인덱스, 관계) | DB 관련 작업 시 |

### API 도메인 문서 (해당 도메인 작업 시)

| 문서 | 도메인 |
|------|--------|
| `docs/AMK_API_AUTH.md` | 인증 (로그인, OAuth, MFA, 비밀번호, 이메일 인증) |
| `docs/AMK_API_USER.md` | 사용자 CRUD + 관리자 사용자 관리 |
| `docs/AMK_API_LEARNING.md` | health, video, study, lesson, course, translation |
| `docs/AMK_API_PAYMENT.md` | Paddle 결제, 구독, 웹훅 |
| `docs/AMK_API_TEXTBOOK.md` | 교재 주문 |
| `docs/AMK_API_EBOOK.md` | E-book 웹 뷰어 |
| `docs/AMK_API_FUTURE.md` | 미구현 (시딩, 발음, 조음, TTS) |

### 기타 도메인 문서

| 문서 | 역할 |
|------|------|
| `docs/AMK_EBOOK_SECURITY.md` | E-book 보안 전략, DRM 조사, 플랫폼별 보안 역량 |
| `docs/AMK_APP_ROADMAP.md` | 모바일(Flutter)/데스크탑(Tauri) 앱 개발 로드맵, 전체 우선순위 |
| `docs/AMK_FRONTEND.md` | 프론트엔드 구조, 라우팅, 상태관리, UI/UX |
| `docs/AMK_STATUS.md` | 작업 현황, 로드맵, 체크리스트 |
| `docs/AMK_MARKET_ANALYSIS.md` | 시장 분석, 교육 방법론, 비즈니스 전략 |
| `docs/AMK_DESIGN_SYSTEM.md` | Figma 디자인 시스템, UI 컴포넌트 |
| `docs/AMK_CHANGELOG.md` | 변경 이력 |

> **이관된 문서**: `docs/AMK_PIPELINE.md` → `amazing-korean-ai/docs/AMK_AI_PIPELINE.md`, `docs/AMK_MACMINI_SETUP.md` → `amazing-korean-ai/docs/AMK_AI_MACMINI.md` (amazing-korean-ai 리포 참조).

## 핵심 파일

- `src/config.rs` — 환경변수 설정 SSoT
- `src/error.rs` — 전역 에러 타입 (AppError → HTTP 응답)
- `src/types.rs` — DB enum ↔ Rust enum ↔ JSON 매핑
- `src/api/{domain}/` — 백엔드 도메인별 모듈 (dto → repo → service → handler → router)
- `frontend/src/category/{domain}/` — 프론트엔드 도메인별 모듈 (types → api → hook → page)

## 핵심 규칙

### 코드 변경 시
- **문서 동기화**: 코드 변경 시 해당 도메인 문서(`docs/AMK_API_*.md`)도 반드시 함께 업데이트
- **환경변수 변경**: `config.rs` + `docker-compose.prod.yml` + `docs/` 동시 반영
- **새 엔드포인트 추가**: dto → repo → service → handler → router 순서로 구현

### 보안
- **Anti-enumeration**: 인증 관련 응답은 존재 여부 노출 금지 (항상 동일 메시지)
- **이메일**: Resend만 사용 (AWS SES 폐기됨)
- **암호화**: PII는 AES-256-GCM 암호화 + Blind Index로 검색
- **인증코드**: HMAC-SHA256 해시로 Redis 저장 (평문 저장 금지)
- **Rate Limiting**: 이메일 발송 실패 시 카운터 자동 롤백 (DECR)

### 프로덕션 안전장치
- `EMAIL_PROVIDER=none` + `APP_ENV=production` → 서버 부팅 실패 (panic)
- `RATE_LIMIT_EMAIL_MAX < 1` → 서버 부팅 실패 (panic)

## 작업 수행 플로우

코드 변경이 포함된 모든 작업에 아래 순서를 따를 것.

1. 해당 도메인 문서 읽기 (`docs/AMK_API_*.md`)
2. 코드 작업 수행
3. 검증 (`cargo check` + `cd frontend && npm run build`)
4. 변경된 도메인 문서 업데이트 (`docs/AMK_API_*.md`)
5. `docs/AMK_STATUS.md` 체크박스 갱신
6. `docs/AMK_CHANGELOG.md` 변경 이력 추가
7. 관련 메모리 날짜 갱신

## 커밋 컨벤션

```
Phase V1-2 : <작업 내용 요약>
```
