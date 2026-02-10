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

## 필수 참조 문서

작업 시작 전 관련 문서를 반드시 읽고 시작할 것.

| 문서 | 역할 | 언제 읽나 |
|------|------|-----------|
| `docs/AMK_API_MASTER.md` | API 스펙, 엔드포인트, DTO, 보안 정책 **(SSoT)** | 모든 작업 |
| `docs/AMK_CODE_PATTERNS.md` | 백엔드/프론트엔드 코드 패턴, 컨벤션 | 새 기능 구현 시 |
| `docs/AMK_DEPLOY_OPS.md` | 배포, 운영, CI/CD, 인프라 | 배포/환경 관련 작업 시 |
| `docs/AMK_SCHEMA_PATCHED.md` | DB 스키마 (테이블, 인덱스, 관계) | DB 관련 작업 시 |

## 디렉토리 구조

```
src/
├── main.rs                    # 부트스트랩
├── config.rs                  # 환경변수 설정 SSoT
├── state.rs                   # AppState 의존성 컨테이너
├── error.rs                   # 전역 에러 타입 (AppError → HTTP 응답)
├── types.rs                   # DB enum ↔ Rust enum ↔ JSON 매핑
├── crypto/                    # 암호화 (cipher, service, blind_index)
├── external/email.rs          # EmailSender trait + ResendEmailSender
├── bin/rekey_encryption.rs    # 키 로테이션 CLI
└── api/
    ├── auth/                  # 인증 (로그인, OAuth, 비밀번호 재설정, 이메일 인증)
    ├── user/                  # 사용자 관리
    ├── admin/                 # 관리자 기능
    ├── course/                # 코스
    ├── lesson/                # 레슨
    ├── study/                 # 학습
    ├── video/                 # 영상
    └── health/                # 헬스체크

frontend/src/
├── category/                  # 도메인별 모듈
│   ├── auth/                  # 인증 (page, hook, types, auth_api)
│   ├── user/                  # 사용자
│   ├── admin/                 # 관리자
│   ├── lesson/                # 레슨
│   ├── study/                 # 학습
│   └── ...
├── i18n/locales/              # 다국어 (ko.json, en.json)
├── hooks/                     # 전역 훅 (use_auth_store 등)
├── api/client.ts              # Axios 인스턴스 + ApiError
└── app/routes.tsx             # 라우팅
```

## 백엔드 모듈 패턴 (각 도메인 공통)

```
api/{domain}/
├── dto.rs       # 요청/응답 DTO (Deserialize, Validate, ToSchema)
├── repo.rs      # SQLx 쿼리 (DB 접근만, 비즈니스 로직 없음)
├── service.rs   # 비즈니스 로직 (트랜잭션, 검증, 외부 호출)
├── handler.rs   # HTTP 핸들러 (파싱 → 서비스 호출 → 응답)
└── router.rs    # 라우트 정의 (미들웨어 바인딩)
```

## 프론트엔드 모듈 패턴 (각 도메인 공통)

```
category/{domain}/
├── types.ts      # Zod 스키마 + TypeScript 타입
├── {domain}_api.ts  # API 함수 (Axios 호출)
├── hook/         # TanStack Query 훅 (useQuery, useMutation)
└── page/         # 페이지 컴포넌트
```

## 핵심 규칙

### 코드 변경 시
- **문서 동기화**: 코드 변경 시 `docs/AMK_API_MASTER.md`도 반드시 함께 업데이트
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

## 검증 체크리스트

코드 변경 후 반드시 확인:

```bash
# 백엔드 컴파일 확인
cargo check

# 프론트엔드 빌드 확인 (tsc + vite)
cd frontend && npm run build
```

## 커밋 컨벤션

```
Phase V1-2 : <작업 내용 요약>
```
