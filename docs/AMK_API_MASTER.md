---
title: AMK_API_MASTER — Amazing Korean API  Master Spec
updated: 2026-03-09
owner: HYMN Co., Ltd. (Amazing Korean)
audience: server / database / backend / frontend / lead / AI agent
---

## ※ AMK_API_MASTER — Amazing Korean API Master Spec ※

> 이 문서는 **Amazing Korean server / database / backend / frontend / web&app 전체 스펙·규칙·로드맵의 단일 기준(Single Source of Truth)** 이다.

> 과거 문서들(`AMK_Feature_Roadmap.md`, `AMK_PROJECT_JOURNAL.md`, `AMK_ENGINEERING_GUIDE.md`, `AMK_API_OVERVIEW_FULL.md`, `README_for_assistant.md`)에 흩어져 있던 내용을 통합·정리한 버전

> **이 문서와 다른 문서가 충돌할 경우 이 문서를 정답으로 간주한다.**

---

## 📑 목차 (Table of Contents)

- [0. 문서 메타 & 사용 방법](#0-문서-메타--사용-방법)
  - [0.1 목적](#01-목적)
  - [0.2 사용 원칙](#02-사용-원칙)
  - [0.3 관련 파일](#03-관련-파일)
  - [0.4 AI 에이전트 협업 규칙](#04-ai-에이전트-협업-규칙)

- [1. 프로젝트 개요 & 목표](#1-프로젝트-개요--목표)
  - [1.1 서비스 개요](#11-서비스-개요)
  - [1.2 비즈니스 흐름 (Business Logic)](#12-비즈니스-흐름-business-logic)

- [2. 시스템 & 개발 환경 개요](#2-시스템--개발-환경-개요)
  - [2.1 런타임 / 스택](#21-런타임--스택)
  - [2.2 라우팅 & OpenAPI](#22-라우팅--openapi)
  - [2.3 로컬 개발 & 실행](#23-로컬-개발--실행)
  - [2.4 외부 서비스 연동](#24-외부-서비스-연동)
  - [2.5 User-Agent 서버사이드 파싱](#25-user-agent-서버사이드-파싱-woothee)

- [3. 공통 규칙 (전역 컨벤션)](#3-공통-규칙-전역-컨벤션)
  - [3.1 시간/타임존](#31-시간타임존)
  - [3.2 네이밍 & 스키마 규칙 (요약)](#32-네이밍--스키마-규칙-요약)
  - [3.3 공통 헤더 & 인증](#33-공통-헤더--인증)
  - [3.4 에러 응답 표준](#34-에러-응답-표준)
  - [3.5 페이징 & 검색 표준](#35-페이징--검색-표준)
  - [3.6 응답 래퍼 정책](#36-응답-래퍼-정책)
  - [3.7 인증 & 세션 관리 (통합)](#37-인증--세션-관리-통합)

- [4. 데이터 모델 개요 (요약)](#4-데이터-모델-개요-요약)
  - [4.1 사용자 도메인 (USERS)](#41-사용자-도메인-users)
  - [4.2 인증 로그인 도메인 (AUTH LOGIN)](#42-인증-로그인-도메인-auth-login)
  - [4.3 비디오 도메인 (VIDEOS)](#43-비디오-도메인-videos)
  - [4.4 학습 도메인 (STUDY)](#44-학습-도메인-study)
  - [4.5 수업 구성 도메인 (LESSON)](#45-수업-구성-도메인-lesson)
  - [4.6 코스 도메인 (COURSE)](#46-코스-도메인-course--구현-완료)
  - [4.7 향후 업데이트 도메인](#47-향후-업데이트-도메인)
  - [4.8 번역 도메인 (TRANSLATION)](#48-번역-도메인-translation)
  - [4.9 결제 도메인 (PAYMENT)](#49-결제-도메인-payment)
  - [4.10 교재 주문 도메인 (TEXTBOOK)](#410-교재-주문-도메인-textbook)
  - [4.11 E-book 도메인 (EBOOK)](#411-e-book-도메인-ebook)

- [5. 기능 & API 로드맵 (도메인 인덱스)](#5-기능--api-로드맵-도메인-인덱스)
  - [5.0 Phase 로드맵 체크박스 범례](#50-phase-로드맵-체크박스-범례)
  - [상태축 (State Axis) 정의](#상태축-state-axis-정의)
  - [5.1 도메인별 상세 문서 인덱스](#51-도메인별-상세-문서-인덱스)

- [6. 프론트엔드 구조 & 규칙](#6-프론트엔드-구조--규칙) → [`AMK_FRONTEND.md`](./AMK_FRONTEND.md)

- [7. 엔지니어링 가이드](#7-엔지니어링-가이드) → [`AMK_CODE_PATTERNS.md`](./AMK_CODE_PATTERNS.md)

- [8. 작업 현황](#8-작업-현황) → [`AMK_STATUS.md`](./AMK_STATUS.md)

- [9. 변경 이력](#9-변경-이력)

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 0. 문서 메타 & 사용 방법

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

### 0.1 목적

- Amazing Korean server / database / backend / frontend / web&app 대한:
  - **기능 & API 로드맵 (Phase / 화면 / 엔드포인트 / 완료 상태)**
  - **공통 규칙 (에러 / 시간 / 인증 / 페이징 / 응답 래퍼 등)**
  - **개발 / 작업 방식 (엔지니어링 가이드)**
  - **AI 에이전트 협업 규칙**
  - **작업 현황 (완료/진행 예정/세부 검토)**
- 을 한 파일에서 관리하기 위함.

### 0.2 사용 원칙

- **스펙 / 기능 / 엔드포인트를 변경할 때는 항상 이 파일을 먼저 수정**한다.
- 코드/마이그레이션/테스트를 변경한 뒤에는, 여기의 관련 섹션(Phase 표, 규칙, TODO)을 반드시 갱신한다.
- 과거 md 문서들은 모두 **참고용 아카이브**이며, 새로운 정보는 **여기에만 적는다**.

### 0.3 관련 파일

- **데이터베이스 스키마**: [`AMK_SCHEMA_PATCHED.md`](./AMK_SCHEMA_PATCHED.md) - 전체 DDL 정의
- **코드 패턴 & 엔지니어링**: [`AMK_CODE_PATTERNS.md`](./AMK_CODE_PATTERNS.md) - 엔지니어링 원칙 + 백엔드/프론트엔드 코드 패턴
- **배포 & 운영**: [`AMK_DEPLOY_OPS.md`](./AMK_DEPLOY_OPS.md) - 빌드, 배포, CI/CD, 유지보수
- **변경 이력**: [`AMK_CHANGELOG.md`](./AMK_CHANGELOG.md) - 시간 역순 변경 이력
- **API 도메인 문서** (§5에서 분할):
  - [`AMK_API_AUTH.md`](./AMK_API_AUTH.md) — 인증 (로그인, OAuth, MFA, 비밀번호, 이메일 인증)
  - [`AMK_API_USER.md`](./AMK_API_USER.md) — 사용자 CRUD + 관리자 사용자 관리
  - [`AMK_API_LEARNING.md`](./AMK_API_LEARNING.md) — health, video, study, lesson, course, translation
  - [`AMK_API_PAYMENT.md`](./AMK_API_PAYMENT.md) — Paddle 결제, 구독, 웹훅
  - [`AMK_API_TEXTBOOK.md`](./AMK_API_TEXTBOOK.md) — 교재 주문
  - [`AMK_API_EBOOK.md`](./AMK_API_EBOOK.md) — E-book 웹 뷰어
  - [`AMK_API_FUTURE.md`](./AMK_API_FUTURE.md) — 미구현 (시딩, 발음, 조음, TTS)
- **프론트엔드**: [`AMK_FRONTEND.md`](./AMK_FRONTEND.md) — 구조, 라우팅, 상태관리, UI/UX
- **작업 현황**: [`AMK_STATUS.md`](./AMK_STATUS.md) — 완료/예정 항목, 로드맵, 체크리스트
- **파이프라인**: [`AMK_PIPELINE.md`](./AMK_PIPELINE.md) - 멀티 AI 오케스트레이션, 작업 흐름
- **시장 분석**: [`AMK_MARKET_ANALYSIS.md`](./AMK_MARKET_ANALYSIS.md) - 경쟁사 분석, 결제 전략

### 0.4 AI 에이전트 협업 규칙

> Claude Code, Gemini (OpenClaw), Codex 등 코딩 에이전트 공통 규칙

1. **SSOT 우선순위**: 이 문서(`AMK_API_MASTER.md`)가 최우선 참조. 코드와 문서가 다를 경우 이 문서 기준으로 코드 수정
2. **문서 구조**: 스펙/규칙은 이 파일, 코드 예시는 [`AMK_CODE_PATTERNS.md`](./AMK_CODE_PATTERNS.md), 배포/운영은 [`AMK_DEPLOY_OPS.md`](./AMK_DEPLOY_OPS.md), 작업 흐름/역할 분리는 [`AMK_PIPELINE.md`](./AMK_PIPELINE.md)
3. **네이밍/패턴 준수**: Section 3 규칙 + `AMK_CODE_PATTERNS.md` 패턴을 따를 것
4. **에러 처리 필수**: Silent Failure 금지, 사용자에게 명확한 피드백 제공 (toast, 에러 페이지 등)
5. **타입 일관성**: 프론트엔드는 `types.ts` 정의 타입만 사용, 백엔드 DTO는 DB 스키마와 일치

---

## 1. 프로젝트 개요 & 목표

### 1.1 서비스 개요

- **Brand Identity**: **Amazing Korean** (Global Korean Language LMS)
- **Target Audience**:
  - **EPS-TOPIK & TOPIK 준비생**: 한국 취업 및 유학을 목표로 하는 해외 학습자
  - **수준별 학습**:
    - **기초(Foundation)**: 500문장 패턴 습득을 통한 문법/회화 기초 완성
    - **급수별 과정**: 초급(TOPIK 1~2), 중급(TOPIK 3~4), 고급(TOPIK 5~6) 맞춤형 커리큘럼
- **Core Value (차별점)**:
  - **습득(Acquisition) 중심**: 암기가 아닌, 실제 한국인의 언어 사용 패턴(Context) 기반 자연적 습득 유도
  - **압도적 효율성**: 데이터 기반 커리큘럼으로 기존 대비 **1/3 학습 시간**으로 목표 등급 달성
  - **이중 언어 학습(Bilingual)**: 학습자의 모국어와 한국어를 매핑하여 이해도 극대화 (DB 다국어 지원 설계)
- **Platform Channels**:
  - **Web/App**: `https://amazingkorean.net` (반응형 웹 및 하이브리드 앱, 준비중)
  - **Core Features**: LMS(학습 관리), VOD 스트리밍, CBT(Computer Based Test), 결제 및 멤버십

### 1.2 비즈니스 흐름 (Business Logic)

- **학습자 (User Journey)**
  1. **접근 및 가입**: 소셜/이메일 회원가입 (User/Auth)
  2. **과정 탐색**: 레벨/목적에 맞는 강좌(Course) 및 무료 샘플 강의(Lesson) 체험
  3. **결제 및 권한 획득**:
     - PG 결제 또는 B2B 바우처 등록 (Payment/Ticket)
     - 멤버십 기간 동안 해당 콘텐츠 접근 권한(Access Control) 획득
  4. **학습 진행 (Learning Loop)**:
     - **VOD 학습**: Vimeo 연동 영상 시청 및 진도율 자동 저장 (Video Log)
     - **Practice**: 문장/단어 퀴즈 및 따라 하기 (Study Log)
     - **Test**: 단원 평가 및 모의고사 응시 (Exam Result)
  5. **성과 관리**: 나의 진도율 확인, 수료증 발급, 오답 노트 복습

- **관리자 (Admin & Operation)**
  - **콘텐츠 관리**: 비디오/태그 메타데이터 등록, 강좌/강의 커리큘럼 구성 (CMS)
  - **학습자 관리**: 회원 정보 조회, 수강 이력 모니터링, 악성 유저 제재
  - **매출/통계**: 기간별 결제 내역 확인, 인기 강좌 및 이탈률 분석

- **Business Model (BM)**
  - **B2C (개인)**: 월/년 단위 구독 또는 단과 강좌 구매
  - **B2B (기관/대학)**:
    - 기업/학교 대상 대량 수강권(Voucher) 발급 및 관리
    - 기관 전용 대시보드 및 학습자 리포트 제공 (컨설팅)

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 2. 시스템 & 개발 환경 개요

### 2.1 런타임 / 스택

#### **Frontend**
  - **Core & Build**
    - **Vite**: 빠른 개발 서버 및 번들링
    - **React (v18)**: UI 라이브러리
    - **TypeScript**: 정적 타입 언어

  - **UI & Styling**
    - **Tailwind CSS**: 유틸리티 퍼스트 CSS 프레임워크 (`darkMode: ["class"]`)
    - **Shadcn/ui**: 재사용 가능한 컴포넌트 라이브러리 (Radix UI 기반)
    - **next-themes**: 다크모드 테마 전환 (Light/Dark/System, localStorage 유지)
    - **Lucide React**: 아이콘 팩
    - **class-variance-authority (cva)**: 컴포넌트 변형(Variant) 관리

  - **State Management**
    - **TanStack Query (React Query)**: 서버 상태 관리 (Caching, Fetching, Synchronization)
    - **Zustand**: 클라이언트 전역 상태 관리 (Auth, Session 등)

  - **Routing & Network**
    - **React Router DOM**: SPA 라우팅
    - **Axios**: HTTP 클라이언트 (Interceptor를 통한 토큰/에러 처리)

  - **Form & Validation**
    - **React Hook Form**: 폼 상태 관리 및 성능 최적화
    - **Zod**: 스키마 기반 데이터 검증 (TypeScript 타입 추론 연동)

  - **Media & Features**
    - **@vimeo/player**: Vimeo 영상 제어 및 이벤트 핸들링 (SDK)

#### **Backend**
  - **Language & Framework**
    - **Rust**: 메모리 안전성 및 고성능 보장
    - **Axum (0.8)**: Tokio 기반 비동기 웹 프레임워크
  - **Data & API**
    - **SQLx**: 컴파일 타임 쿼리 검증 및 비동기 PostgreSQL 드라이버
    - **utoipa (v5)**: 코드 기반 OpenAPI(Swagger) 문서 자동화 (`/docs`)
  - **Auth & Security**
    - **JWT**: HS256 알고리즘 기반 Stateless Access Token
    - **Argon2**: 안전한 비밀번호 해싱
    - **Redis**: Refresh Token 저장 및 세션 관리
  - **Encryption**
    - **AES-256-GCM**: 애플리케이션 레벨 PII 필드 암호화 (`src/crypto/cipher.rs`)
    - **HMAC-SHA256**: Blind Index 기반 암호화된 필드 검색 (`src/crypto/blind_index.rs`)
    - **CryptoService**: 암/복호화 + 평문 호환 처리 (`src/crypto/service.rs`)
    - **Key Rotation**: 다중 키 지원 (`ENCRYPTION_KEY_V{n}` 패턴, `src/bin/rekey_encryption.rs`)

#### **Database**
  - **PostgreSQL**
    - 도커 컨테이너명: `amk-pg`
    - 기본 포트: `5432`
    - 표준: 모든 시간 컬럼 `TIMESTAMPTZ` (UTC 기준), Default `now()`
  - **Redis**
    - 도커 컨테이너명: `amk-redis`
    - 용도: 인증 토큰 관리 및 임시 데이터 캐싱

#### **Infrastructure & Environment**
  - **Development (Local)**
    - **OS**: Windows (Host) + **WSL2** (Ubuntu Subsystem)
    - **Runtime**: Docker Desktop / Docker Compose (WSL Integration)
  - **Dev Tools & AI**
    - **IDE**: VS Code (Remote - WSL)
    - **AI Agent**: Codex CLI
    - **MCP (Model Context Protocol)**:
      - `filesystem`: 프로젝트 파일 시스템 접근 및 제어
      - `sequential-thinking`: 단계적 사고 및 문제 해결
      - `brave-search`: 실시간 웹 정보 검색 및 검증
  - **Production (Hybrid Architecture)**
    - **Frontend**: Cloudflare Pages
      - 글로벌 CDN으로 정적 자원 배포
      - 자동 SSL, DDoS 방어
      - Git 연동 자동 배포
    - **Backend**: AWS EC2 (Ubuntu 24.04 LTS)
      - Nginx (Reverse Proxy: 80/443 → App Server)
      - Docker Compose: 컨테이너 기반 오케스트레이션
    - **Database/Cache**: AWS EC2 내 Docker 또는 관리형 서비스 (RDS/ElastiCache)

### 2.2 라우팅 & OpenAPI

- `Router<AppState>` + `.with_state(state)` 패턴
- 인증 추출:
  - Axum 0.8 `FromRequestParts<S>` 기반 `Claims` 추출
  - `Claims.sub` = `user_id` (i64)
- OpenAPI 루트:
  - `src/docs.rs` (예: `ApiDoc`)
  - Swagger UI: `GET /docs` — **`ENABLE_DOCS=true`일 때만 활성화** (PROD-6, 프로덕션 기본 비활성화)
  - 태그/표시 순서 **고정**: `health → auth → user → videos → study → lesson → admin` (필요 시 추가 리소스는 뒤에)

### 2.3 로컬 개발 & 실행

- DB 마이그레이션:
  - `sqlx migrate run`
- 기본 빌드/실행:
  - `cargo check`
  - `cargo fmt -- --check`
  - `cargo clippy -- -D warnings`
  - `cargo run`
- Swagger 문서 확인:
  - 브라우저에서 `http://localhost:3000/docs`

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

### 2.4 외부 서비스 연동

#### 2.4.1 이메일 발송 (EmailSender trait 추상화)

> Transactional Email 전용. 마케팅 이메일 미사용.
> `EMAIL_PROVIDER` 환경변수로 Provider 설정. 현재 Resend 사용.

**Provider 설정**
| Provider | 환경변수 | 설명 |
|----------|----------|------|
| `resend` | `RESEND_API_KEY` | Resend API (기본, 즉시 사용 가능, 무료 3,000통/월) |
| `none` | — | 이메일 미발송 (로컬 개발용, 프로덕션에서 사용 시 서버 부팅 실패) |

**공통 설정**
| 항목 | 값 |
|------|-----|
| 인증된 도메인 | `amazingkorean.net` |
| 발신 주소 | `noreply@amazingkorean.net` |

**환경변수**
```env
EMAIL_PROVIDER=resend          # resend | none
RESEND_API_KEY=re_xxx          # Resend 사용 시 필수
EMAIL_FROM_ADDRESS=noreply@amazingkorean.net  # 발신 주소
```

**코드 구조**
- `src/external/email.rs`: `EmailSender` trait + `ResendEmailSender` 구현
- `src/state.rs`: `AppState.email: Option<Arc<dyn EmailSender>>`
- `src/config.rs`: `email_provider`, `resend_api_key` + 프로덕션 fail-fast 검증

**EmailTemplate 종류**
| 템플릿 | 용도 | 사용처 |
|--------|------|--------|
| `PasswordResetCode` | 비밀번호 재설정 인증코드 (6자리) | Phase 3 - `POST /auth/request-reset` |
| `EmailVerification` | 이메일 인증 코드 (회원가입 시) | Phase 2 - `POST /users` ✅ |
| `Welcome` | 가입 환영 이메일 | Phase 2 - 회원가입 완료 시 |
| `AdminInvite` | 관리자 초대 코드 + URL | Phase 7 - `POST /admin/upgrade` |

**이메일 발송 제한**
- Rate Limit: 이메일당 5회/5시간 (기본값, 환경변수로 조정 가능)
  - 환경변수: `RATE_LIMIT_EMAIL_WINDOW_SEC` (기본: 18000초=5시간), `RATE_LIMIT_EMAIL_MAX` (기본: 5, **1 이상 필수** — 0 이하 시 서버 부팅 실패)
  - 적용 대상: 비밀번호 재설정 요청, 비밀번호 찾기, 이메일 인증코드 재발송
  - 응답에 `remaining_attempts` 포함 (잔여 발송 횟수, 프론트엔드 표시)
  - 이메일 발송 실패 시 rate limit 카운터 자동 롤백 (`DECR`) — 사용자 시도 낭비 방지
- TTL: 인증코드 10분 만료
- 프로덕션 fail-fast:
  - `APP_ENV=production` + `EMAIL_PROVIDER=none` → 서버 부팅 실패
  - `APP_ENV=production` + `REVENUECAT_API_KEY` 미설정 → 서버 부팅 실패 (IAP 영수증 검증 우회 방지)

#### 2.4.2 Google OAuth

> Google OAuth 2.0 Authorization Code Flow

**환경변수**
```env
GOOGLE_CLIENT_ID=xxx.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=xxx
GOOGLE_REDIRECT_URI=http://localhost:3000/auth/google/callback
```

**관련 엔드포인트**: Phase 3 - `GET /auth/google`, `GET /auth/google/callback`

**ID Token 서명 검증 (JWKS)**
- Google JWKS 엔드포인트(`https://www.googleapis.com/oauth2/v3/certs`)에서 RSA 공개키 조회
- JWT 헤더의 `kid`로 매칭되는 키 선택 → `DecodingKey::from_rsa_components(n, e)` 생성
- 검증 항목: RS256 서명, Issuer (`accounts.google.com`), Audience (`client_id`), 만료시간

#### 2.4.2a Google OAuth (모바일)

> 모바일 앱에서 `google_sign_in` 플러그인으로 ID token 직접 획득 후 서버 검증.

**환경변수**
```env
GOOGLE_MOBILE_CLIENT_ID=xxx.apps.googleusercontent.com  # 모바일 전용 (Android/iOS)
```

**관련 엔드포인트**: `POST /auth/google-mobile` (AMK_API_AUTH.md 5.3-16)

#### 2.4.2b Apple OAuth (모바일 Sign in with Apple)

> Apple 정책상 소셜 로그인 제공 시 필수. Apple JWKS(`https://appleid.apple.com/auth/keys`)로 RS256 검증.

**환경변수**
```env
APPLE_CLIENT_ID=net.amazingkorean.app   # Apple Bundle ID
APPLE_TEAM_ID=XXXXXXXXXX                # Apple Team ID
```

**관련 엔드포인트**: `POST /auth/apple-mobile` (AMK_API_AUTH.md 5.3-17)
**코드**: `src/external/apple.rs`

**특이사항**: Apple은 최초 인증에만 email 제공. subject로 기존 유저 못 찾고 email도 없으면 계정 생성 불가 → 재인증 요청 에러.

#### 2.4.2c RevenueCat (모바일 IAP)

> Apple/Google IAP 영수증 검증을 RevenueCat REST API로 통합.

**환경변수**
```env
REVENUECAT_API_KEY=xxx                  # RevenueCat 서버 API 키
REVENUECAT_WEBHOOK_AUTH_TOKEN=xxx       # RevenueCat 웹훅 Bearer 토큰
```

**관련 엔드포인트**: `POST /ebook/purchase/iap` (AMK_API_EBOOK.md 12.5-2.5), `POST /payment/webhook/revenuecat` (AMK_API_PAYMENT.md 11-4)
**코드**: `src/external/revenuecat.rs`

#### 2.4.3 Vimeo (동영상 스트리밍)

> 동영상 호스팅 및 스트리밍

**코드 구조**
- `src/external/vimeo.rs`: VimeoClient 구현 (메타데이터 조회, tus 업로드 티켓)
- `src/state.rs`: AppState에 `Option<VimeoClient>` 포함

**환경변수**
```env
VIMEO_ACCESS_TOKEN=xxx
```

**관련 엔드포인트**: Phase 7 - `GET /admin/videos/vimeo/preview`, `POST /admin/videos/vimeo/upload-ticket`

#### 2.4.4 IP Geolocation (ip-api.com)

> 로그인 시 IP 기반 지리정보 자동 조회

**서비스**: [ip-api.com](http://ip-api.com) (무료 티어: 45 req/min)

**코드 구조**
- `src/external/ipgeo.rs`: IpGeoClient 구현
- `src/state.rs`: AppState에 `Arc<IpGeoClient>` 포함

**조회 데이터**
| 필드 | DB 컬럼 | 설명 | 예시 |
|------|---------|------|------|
| `countryCode` | `login_country` | ISO 3166-1 alpha-2 국가 코드 | "KR", "US" |
| `as` | `login_asn` | AS 번호 (Autonomous System Number) | 4766 |
| `org` | `login_org` | ISP/조직명 | "Korea Telecom" |

**적용 범위**
- `login` 테이블: 활성 세션 정보
- `login_log` 테이블: 로그인 이력 (감사 로그)

**Private IP 처리**
- `std::net::IpAddr` 파싱 후 표준 라이브러리 메서드로 판별
  - IPv4: `is_private()` || `is_loopback()` (127.x, 10.x, 192.168.x, 172.16-31.x)
  - IPv6: `is_loopback()`
  - 파싱 실패 시: `"localhost"` 문자열 매칭
- 사설 IP는 외부 API 조회 skip, 기본값: `country='LC'` (Local), `asn=0`, `org='local'`

#### 2.4.5 Paddle Billing (결제)

> Paddle Billing (Merchant of Record) — 구독 기반 결제. Paddle이 세금/규정 처리.

**Provider 설정**
| Provider | 환경변수 | 설명 |
|----------|----------|------|
| `paddle` | 아래 9개 | Paddle Billing API (Sandbox/Production) |

**환경변수**
```env
PADDLE_API_KEY=apikey_xxx            # Paddle API Key
PADDLE_CLIENT_TOKEN=test_xxx         # 프론트엔드 Paddle.js 초기화용
PADDLE_SANDBOX=true                  # true(Sandbox) / false(Production)
PADDLE_WEBHOOK_SECRET=pdl_xxx        # Webhook 서명 검증용 Secret Key
PADDLE_PRICE_MONTH_1=pri_xxx         # 1개월 구독 Price ID ($10)
PADDLE_PRICE_MONTH_3=pri_xxx         # 3개월 구독 Price ID ($30, 정가)
PADDLE_PRICE_MONTH_6=pri_xxx         # 6개월 구독 Price ID ($60, 정가)
PADDLE_PRICE_MONTH_12=pri_xxx        # 12개월 구독 Price ID ($120, 정가)
PADDLE_PRICE_EBOOK=pri_xxx           # E-book 일회성 Price ID ($10 USD)
```

**코드 구조**
- `src/external/payment.rs`: `PaymentProvider` trait + `PaddleProvider` 구현 (paddle-rust-sdk)
- `src/state.rs`: `AppState.payment: Option<Arc<dyn PaymentProvider>>`
- `src/config.rs`: Paddle 환경변수 9개 + `billing_interval_for_price()` 매핑
- `src/api/payment/`: 사용자 결제 API (plans, subscription, webhook)
- `src/api/admin/payment/`: 관리자 결제 관리 API
- `src/api/textbook/`: 교재 주문 API (catalog, orders — 비회원 접근 가능)
- `src/api/admin/textbook/`: 관리자 교재 주문 관리 API

**비즈니스 모델**
| 항목 | 값 |
|------|-----|
| 결제 모델 | 구독 (자동 갱신) |
| 통화 | USD |
| 무료 체험 | 1일 |
| 1개월 | $10 |
| 3개월 | $25 |
| 6개월 | $50 |
| 12개월 | $100 |

**Webhook 이벤트 처리**
| 이벤트 | 처리 내용 |
|--------|-----------|
| `subscription.created` | 구독 레코드 생성 |
| `subscription.activated` | 상태 active 전환 + 수강권 부여 |
| `subscription.updated` | 기간/가격 업데이트 |
| `subscription.canceled` | 상태 canceled + 수강권 만료일 설정 |
| `subscription.paused` | 상태 paused + 수강권 비활성화 |
| `subscription.resumed` | 상태 active + 수강권 재활성화 |
| `subscription.trialing` | 상태 trialing + 수강권 부여 |
| `subscription.past_due` | 상태 past_due |
| `transaction.completed` | 트랜잭션 기록 저장 |

**Webhook 보안**
- 서명 검증: `Paddle::unmarshal()` (HMAC-SHA256, 300초 MaximumVariance)
- 멱등성: `webhook_events` 테이블 UNIQUE(payment_provider, provider_event_id)

### 2.5 User-Agent 서버사이드 파싱 (woothee)

로그인/회원가입 시 HTTP `User-Agent` 헤더를 서버에서 파싱하여 `login_os`, `login_browser`, `login_device`를 자동으로 채운다.

**라이브러리**: `woothee` (Cargo.toml)

**파싱 매핑**
| woothee 필드 | DB 컬럼 | 설명 | 예시 |
|-------------|---------|------|------|
| `os` | `login_os` | 운영체제 | "Windows 10", "Mac OS X", "Linux" |
| `name` | `login_browser` | 브라우저 | "Chrome", "Firefox", "Safari" |
| `category` | `login_device` | 기기 유형 매핑 | "pc"→desktop, "smartphone"→mobile |

**기기 유형 매핑 규칙**
- `pc` → `desktop`
- `smartphone`, `mobilephone` → `mobile`
- 그 외 (`crawler`, `appliance`, `misc`, `UNKNOWN`) → `other`

**적용 범위**: 로그인, 회원가입, OAuth 콜백 (프론트엔드에서 device/browser/os를 전송하지 않음)

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 3. 공통 규칙 (전역 컨벤션)

### 3.1 시간/타임존

- DB의 시간 컬럼(특히 로그/이력)은:
  - 타입: `TIMESTAMPTZ`
  - 기본값: `DEFAULT now()` (UTC)
- 클라이언트(웹/앱)에선 KST or 로컬 타임존으로 변환하여 표시.

#### 사용자 타임존 (`user_set_timezone`) 정책

> 목적: 알람, 학습 리마인더, 콘텐츠 예고 등 시간 기반 서비스를 위한 사용자별 시간대 관리

- **자동 감지**: 회원가입 또는 로그인 시 브라우저/기기에서 `Intl.DateTimeFormat().resolvedOptions().timeZone`으로 자동 감지하여 DB에 저장
- **수동 변경 허용**: 사용자가 설정 페이지에서 직접 타임존을 변경할 수 있도록 지원 (VPN/여행 등으로 감지값이 실제 생활 시간대와 다를 수 있음)
- **자동 갱신 안 함**: 로그인 시 감지된 값으로 자동 덮어쓰지 않음 (사용자가 설정한 값을 존중)
  - 최초 가입 시에만 자동 저장, 이후에는 사용자가 직접 변경해야 함
- **활용 예정**: 알람/푸시 알림 발송 시간, 학습 리마인더, 콘텐츠 공개 시각 표시 등

### 3.2 네이밍 & 스키마 규칙 (요약)

> 최상위 원칙

- **외부 인터페이스(DB 스키마, API 경로, JSON 필드 이름)** 은 **snake_case**를 기준으로 한다.
- **각 레이어의 코드 레벨 네이밍**은 해당 언어/프레임워크의 관습을 따른다.
  - 백엔드: Rust 관례
  - 프론트엔드: TypeScript/React 관례

---

#### 3.2.1 Database

> Naming Convention : snake_case  
> 논리명(문서/ERD)은 **대문자 SNAKE_CASE**, 실제 DB 스키마/컬럼은 **소문자 snake_case**를 기본으로 한다.

- **table 명**
  - 형식: `<도메인(단수형, 대문자)>_<의미 1(존재 시 대문자)>_<의미 2(존재 시 대문자)>...`
  - 예시:
    - `USERS` (PostgreSQL에 `USER` 예약어가 있어 복수형 사용)
    - `VIDEO_TAG`, `VIDEO_TAG_MAP`
    - `USERS_LOG`, `ADMIN_USERS_LOG`, `STUDY_TASK_LOG`

- **enum 명**
  - 형식: `<도메인(단수형, 소문자)>_<의미 1(소문자)>_<의미 2(소문자)>..._enum`
  - 예시:
    - `user_auth_enum`, `user_set_language_enum`
    - `study_task_kind_enum`, `lesson_item_kind_enum`

- **log 테이블/컬럼**
  - 테이블:
    - 형식: `<도메인(복수형, 대문자)>_<의미 1(대문자)>_<의미 2(대문자)>..._LOG`
    - 예시: `STUDY_TASK_LOG`, `LOGIN_LOG`, `USERS_LOG`
  - 로그용 컬럼:
    - 형식: `<도메인(단수형, 소문자)>_<의미 1(소문자)>_<의미 2(소문자)>..._log`
    - 예시: `user_nickname_log`, `video_last_user_agent_log`, `study_task_score_log`

- **admin 계열**
  - 테이블:
    - 형식: `ADMIN_<도메인(복수형, 대문자)>_<의미 1(대문자)>_<의미 2(대문자)>...`
    - 예시: `ADMIN_USERS_LOG`, `ADMIN_VIDEO_LOG`, `ADMIN_STUDY_LOG`
  - 컬럼:
    - 형식: `admin_<도메인(단수형, 소문자)>_<의미 1(소문자)>_<의미 2(소문자)>...`
    - 예시: `admin_pick_study_id`, `admin_user_action`, `admin_study_log_id`

---

#### 3.2.2 API 경로 & JSON 필드

- **API 경로**
  - 경로 표기: **소문자 + 케밥케이스**  
    - 예시: `/users`, `/auth/login`, `/admin/videos`
  - 리소스 이름:
    - 기본: **명사(복수형)** 사용 (`/users`, `/videos`, `/studies` 등)
    - 예외: `/auth` 계열은 기능 중심 (`/auth/login`, `/auth/refresh` 등)
  - 액션 표현:
    - **HTTP 메서드**로 표현  
      - 예시: `GET /users`, `POST /users`, `POST /users/me`, `PATCH /admin/users/{id}`

- **리소스 / ID 경로 패턴**
  - 단일 리소스:
    - `/users/{user_id}`, `/videos/{video_id}`
  - 하위 리소스:
    - `/videos/{video_id}/captions`
    - `/videos/{video_id}/progress`
    - `/studies/tasks/{task_id}/explain`

- **사용 예시**
  - 조회:
    - `GET /users/me/settings`, `GET /videos`
  - 생성/업데이트:
    - `POST /videos/{video_id}/progress`
    - `POST /studies/tasks/{task_id}/answer`
    - `POST /users/me`, `POST /users/me/settings`

- **JSON 필드**
  - API 요청/응답의 필드 이름은 **DB 컬럼과 동일한 snake_case**를 사용한다.
    - 예시: `user_id`, `video_title`, `created_at`, `user_state`

---

#### 3.2.3 백엔드(Rust) 네이밍 & 역할

> 기본 원칙  
> - DB 스키마·API·JSON 필드 = **snake_case**  
> - 코드 레벨 네이밍은 **Rust 관례**를 따른다.

- **모듈/파일명**
  - 도메인별 디렉터리 구조(예: `src/api/user/`):
    - `dto.rs`, `handler.rs`, `repo.rs`, `router.rs`, `service.rs`, `mod.rs` 고정
    - 예: `src/api/user/dto.rs`, `src/api/user/service.rs`, `src/api/user/repo.rs`
  - 그 외 보조 파일은 필요 시 **소문자 + snake_case**로 추가
    - 예: `token_utils.rs`, `validator.rs` 등

- **함수/변수명 (Naming Convention)**
  - **기본 규칙**: `snake_case` (소문자 + 언더스코어)
  - **계층 간 통일 (Feature Parity)**:
    - 하나의 기능(Feature)에 대해 Handler, Service, Repo 계층의 **메인 함수명은 반드시 통일**한다.
    - 코드 추적성(Traceability) 향상을 위함.
    - **패턴**: `[도메인]_[행위]_[대상]` (필요 시 도메인 생략 가능)
    - **예시 (관리자 유저 생성)**:
      - Handler: `admin_create_user(...)`
      - Service: `admin_create_user(...)`
      - Repo: `admin_create_user(...)`
  - **Repo 보조 함수 (Helpers)**:
    - 메인 로직 외의 단순 조회, 검증, 로그 기록 등은 기능에 맞는 이름 사용 가능.
    - 예: `exists_email`, `create_audit_log`, `find_by_id`
  - **타입(Struct/Enum/DTO)**: **PascalCase** (대문자 카멜 표기)
    - 예: `SignupReq`, `AdminUserRes`, `VideoProgressLog`, `UserAuth`

- **DTO/필드명**
  - DB/JSON과 매핑되는 필드 이름은 **snake_case**로 작성
    - 예:
      ```rust
      pub struct UserMeRes {
          pub user_id: i64,
          pub user_email: String,
          pub user_state: String,
      }
      ```
  - 필요 시 `#[serde(rename = "...")]`, `#[sqlx(rename = "...")]` 로 DB/JSON 필드와의 정렬성을 명시적으로 유지

- **도메인별 repo 역할**
  - 각 도메인(`user`, `video`, `study`, `lesson`, `admin` 등)의 `repo.rs`는  
    **그 도메인의 단일 DB 진입점(single entry point)** 역할을 한다.
  - 다른 도메인에서 해당 도메인의 데이터를 다뤄야 할 때,
    - 가능한 한 **그 도메인의 service 레이어**를 경유해서 접근한다.
    - 예: admin이 유저를 생성할 때 → `user::service::create_by_admin(...)` 호출

- **유즈케이스 단위 함수 이름 규칙**
  - 하나의 유즈케이스(예: `/users` 회원가입, `/auth/login`, `/users/me/settings` 수정 등)에 대해서는  
    도메인별 `handler.rs` / `service.rs` / `repo.rs`에서 **가능하면 동일한 함수명**을 사용한다.
    - 예:
      - `handler::signup`
      - `service::signup`
      - `repo::signup`
  - Rust 모듈 네임스페이스를 활용해,
    - `handler::signup` → `service::signup` → `repo::signup` 흐름이 한눈에 보이도록 맞춘다.
  - 예시:
    ```rust
    // handler.rs
    pub async fn signup(...) -> AppResult<Json<SignupRes>> {
        let res = service::signup(...).await?;
        Ok(Json(res))
    }

    // service.rs
    pub async fn signup(...) -> AppResult<SignupRes> {
        let user = repo::signup(...).await?;
        Ok(SignupRes::from(user))
    }

    // repo.rs
    pub async fn signup(...) -> AppResult<UserRow> {
        // INSERT INTO users ...
    }
    ```

- **감사 로그 (`admin_action_log`) 값 규칙**
  - `action_type`: **대문자 SNAKE_CASE** — 예: `"CREATE_VIDEO"`, `"BULK_UPDATE_USERS"`, `"LIST_LESSONS"`
  - `target_table`: **소문자 snake_case** (실제 DB 테이블명과 일치) — 예: `"video"`, `"study_task"`, `"lesson_item"`, `"users"`, `"subscriptions"`
  - `target_id`: 단건 조회/수정/삭제 시 `Some(id)`, 목록/벌크 작업 시 `None`
  - `ip_address`: AES-256-GCM 암호화 저장 (평문 금지)
  - `details`: 변경 내역 JSON (`before`/`after` 또는 요약)

- **공통 repo 함수 (여러 유즈케이스에서 공유할 때)**
  - 여러 유즈케이스에서 동일한 DB 동작을 사용하는 경우,
    - repo 내부에서 **좀 더 일반적인 이름**으로 공통 함수를 분리한다.
    - 예:
      - `insert_user`, `get_user_by_email`, `update_user_state` 등
  - service 계층에서는 유즈케이스 이름을 유지한다.
    - 예:
      ```rust
      // repo.rs
      pub async fn insert_user(...) -> AppResult<UserRow> { ... }
      pub async fn get_user_by_email(...) -> AppResult<Option<UserRow>> { ... }

      // service.rs
      pub async fn signup(...) -> AppResult<SignupRes> {
          if repo::get_user_by_email(&req.email).await?.is_some() {
              return Err(AppError::Conflict(...));
          }
          let user = repo::insert_user(...).await?;
          Ok(SignupRes::from(user))
      }

      pub async fn admin_create_user(...) -> AppResult<AdminUserRes> {
          let user = repo::insert_user(...).await?;
          Ok(AdminUserRes::from(user))
      }
      ```
  - 이때 **쿼리 자체를 별도 “쿼리 전용 모듈”로 빼지 않고**,  
    각 도메인 repo(`user::repo`, `video::repo` 등)가 그 도메인의 쿼리 단일 소스 역할을 맡는다.
  - 정말 cross-domain으로 공유해야 하는 복잡한 패턴(예: 통합 통계 뷰 등)은  
    PostgreSQL의 **VIEW/FUNCTION**으로 추상화하는 것을 우선 검토한다.

> 정리:  
> - **유즈케이스 이름은 handler/service/repo에서 최대한 동일하게**,  
> - **쿼리 중복 제거와 스키마 변경 대응은 도메인별 repo에서 책임**,  
> - DB 레벨 공통화가 필요하면 VIEW/FUNCTION으로 해결하는 것을 기본 전략으로 한다.

---

#### 3.2.4 프론트엔드(TypeScript + React) 네이밍

- **React 컴포넌트**
  - 파일명: PascalCase
    - 예시: `LoginPage.tsx`, `VideoListPage.tsx`, `UserSettingsForm.tsx`
  - 컴포넌트 이름 & JSX:
    - 예시: `function LoginPage() { ... }`, `<LoginPage />`

- **기타 TS 파일 (hook / api / lib / util 등)**
  - 파일명: 소문자 + snake_case
    - 예시: `video_api.ts`, `auth_api.ts`, `use_auth.ts`, `date_format.ts`
  - 함수/변수명: camelCase
    - 예시: `fetchVideos`, `loginUser`, `formatDate`

- **API DTO 인터페이스**
  - 인터페이스 이름: PascalCase
    - 예시: `interface VideoRes { ... }`
  - 필드 이름: **snake_case** (백엔드/DB와 동일)
    - 예시:
      ```ts
      export interface VideoRes {
        video_id: number;
        video_title: string;
        created_at: string;
      }
      ```

---

> 자세한 컬럼 구조와 실제 타입 정의는 `amk_schema_patched.sql` 및 각 도메인별 Rust/TS DTO를 기준으로 하며, 이 문서에는 **책임과 역할, 규칙 위주로 요약**한다.

### 3.3 공통 헤더 & 인증

- **보안 응답 헤더** (PROD-4, 모든 응답에 자동 적용):
  - `X-Content-Type-Options: nosniff` — MIME 타입 스니핑 방지
  - `X-Frame-Options: DENY` — 클릭재킹 방지 (iframe 삽입 차단)
  - `X-XSS-Protection: 0` — 브라우저 XSS 필터 비활성화 (CSP로 대체 권장)
  - `Permissions-Policy: camera=(), microphone=(), geolocation=()` — 민감 API 사용 제한
  - 구현: `src/main.rs` → `security_headers` 미들웨어 (가장 바깥 레이어)
- HTTP 요청 헤더:
  - `Authorization: Bearer <ACCESS_TOKEN>`
    - 인증 필요한 모든 엔드포인트에 필수
  - `Content-Type: application/json`
    - 요청 본문이 JSON일 때
  - `Accept: application/json`
- **Guard 응답 형식** (PROD-7):
  - Admin IP Guard (`ip_guard.rs`): 403 → `AppError::Forbidden` JSON 응답
  - Admin Role Guard (`role_guard.rs`): 401/403 → `AppError::Unauthorized/Forbidden` JSON 응답
  - 모든 에러 응답은 Section 3.4 에러 응답 표준 형식 준수
- 인증 플로우(기본):
  - `POST /auth/login` → 액세스 토큰(헤더), 리프레시 토큰(쿠키) 발급
  - 만료 시 `POST /auth/refresh`로 재발급 (리프레시 회전/검증/로그 기록)
- 리프레시 쿠키:
  - SameSite/Domain/Secure 설정은 서버 환경설정에 따르되,
    배포 환경에서 **HTTPS + Secure**를 기본으로 가정.

### 3.4 에러 응답 표준

- 공통 에러 바디 예시:

```json
{
  "error": {
    "code": "invalid_argument",
    "http_status": 400,
    "message": "video_state must be one of: ready,open,close",
    "details": null,
    "trace_id": "..."
  }
}
```

- 필드 의미:
  - `code`: 내부/클라이언트 공통으로 식별 가능한 에러 코드 문자열
  - `http_status`: 실제 HTTP status 코드 (예: 400, 401, 403, 404, 409, 500…)
  - `message`: 사용자가 이해할 수 있는 메시지(영문/다국어는 이후 확장)
  - `details`: 필드별 검증 에러 등 구조화된 정보 (없으면 `null`)
  - `trace_id`: 로깅/트레이싱용 ID

- 대표 매핑 예:
  - 400: 검증 실패, 잘못된 요청 파라미터
  - 401: 인증 실패(토큰 없음/만료/위조)
  - 403: 권한 부족 (`user_state != on`, RBAC 불일치 등)
  - 404: 리소스 없음
  - 409: 무결성 위반 (예: 이메일 중복, UNIQUE 제약)
  - 500/503: 서버 내부 오류, 일시적인 외부 의존성 장애

### 3.5 페이징 & 검색 표준

- 기본 규칙:
  - **페이지 기반(page/size) 페이징**을 기본으로 사용
  - 쿼리 파라미터:
    - `page`: 1 기반 페이지 번호
    - `size`: 페이지 당 개수(기본값/상한은 엔드포인트별 정의)
    - `sort`: 정렬 컬럼 (예: `created_at`, `video_title`)
    - `order`: 정렬 방향 (`asc` / `desc`)
- 페이징 응답 래퍼 예시:

```json
{
  "items": [ /* 결과 배열 */ ],
  "page": 1,
  "size": 20,
  "total": 57
}
```

- 기존에 커서 기반 등의 다른 방식이 있다면:
  - 새로 추가되는 목록형 API는 위 표준을 우선 적용
  - 단건 조회/소규모 목록은 굳이 래퍼 없이 배열/객체 반환 허용

### 3.6 응답 래퍼 정책

- 성공 응답:
  - 별도 상위 래퍼 없이 **직접 JSON 객체/배열** 반환을 기본으로 한다.
  - 페이징이 필요한 경우에만 `items/page/size/total` 래퍼 사용.
- 실패 응답:
  - 위의 **공통 에러 바디**를 사용한다.
- PUT/DELETE:
  - 일반적으로 `200` 또는 `204 No Content` 사용
  - 필요한 경우 `200 + 수정 결과 객체` 허용

### 3.7 인증 & 세션 관리 (통합)

> 이 섹션은 인증 관련 산재된 내용을 통합하여 정리함
> - 기존 Section 3.3 (공통 헤더 & 인증)
> - Phase 5.2-3 (POST /auth/refresh)
> - Section 6.4.1 (프론트 인증 상태 관리)
> - Section 7.1 (보안 작업 원칙)

#### 토큰 종류 & 수명

- **액세스 토큰 (Access Token)**:
  - 형식: JWT (HS256 알고리즘)
  - 수명: **15분** (900초, `config.rs` `JWT_ACCESS_TTL_MIN` 기본값)
  - 전송 방식: `Authorization: Bearer <ACCESS_TOKEN>` 헤더
  - 페이로드 구조:
    ```json
    {
      "sub": "<user_id>",       // i64 - 사용자 ID
      "role": "<user_auth>",    // "HYMN" | "admin" | "manager" | "learner"
      "session_id": "<uuid>",   // 세션 식별자 (로그아웃 시 무효화용)
      "iss": "amazing-korean",  // 발급자 식별
      "exp": 1234567890,        // Unix timestamp (15분 후)
      "iat": 1234564290         // 발급 시각
    }
    ```

- **리프레시 토큰 (Refresh Token)**:
  - 형식: Opaque Token (UUID 기반 해시)
  - 수명: **역할별 TTL 적용** (Role-based TTL):
    | 역할 | TTL | 설명 |
    |------|-----|------|
    | HYMN | 1일 (86400초) | 최고 권한 - 보안상 짧은 세션 |
    | admin | 7일 (604800초) | 관리자 - 일반 보안 수준 |
    | manager | 7일 (604800초) | 매니저 - 일반 보안 수준 |
    | learner | 30일 (2592000초) | 학습자 - 편의성 우선 |
  - 전송 방식: **httpOnly 쿠키** (`ak_refresh`)
  - 저장소: **Redis** (`ak:refresh:<hash>` → `<session_id>`)
  - 쿠키 옵션:
    - `HttpOnly`: true (JavaScript 접근 차단)
    - `SameSite`: Lax (CSRF 보호)
    - `Secure`: true (HTTPS 환경에서만 전송, 프로덕션 필수)
    - `Domain`: 환경별 설정 (예: `.amazingkorean.net`)

#### 인증 플로우

**1. 로그인 (`POST /auth/login`)**:
- 요청:
  ```json
  {
    "user_email": "user@example.com",
    "user_password": "password123"
  }
  ```
- 성공 응답 (200 OK):
  ```json
  {
    "access_token": "eyJhbGc...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "user": {
      "user_id": 123,
      "user_email": "user@example.com",
      "user_auth": "learner"
    }
  }
  ```
  - **+ Set-Cookie 헤더**: `ak_refresh=<refresh_token>; HttpOnly; SameSite=Lax; Secure; Max-Age=604800`
- 동작:
  1. 이메일/비밀번호 검증 (Argon2 해싱)
  2. 액세스 토큰 생성 (JWT, 15분)
  3. 리프레시 토큰 생성 (UUID 해시)
  4. Redis에 세션 저장: `ak:refresh:<hash>` → `<session_id>` (TTL 7일)
  5. `users_login_log` 테이블에 로그인 기록

**2. 토큰 재발급 (`POST /auth/refresh`)**:
- 요청:
  - **쿠키**: `ak_refresh=<refresh_token>` (자동 전송)
  - **바디**: 없음 (쿠키에서 자동 추출)
- 성공 응답 (200 OK):
  ```json
  {
    "access_token": "eyJhbGc...",
    "token_type": "Bearer",
    "expires_in": 3600
  }
  ```
  - **+ Set-Cookie 헤더**: `ak_refresh=<new_refresh_token>; HttpOnly; SameSite=Lax; Secure; Max-Age=604800`
- 동작 (Rotate-on-Use 전략):
  1. 쿠키에서 리프레시 토큰 추출
  2. Redis에서 세션 검증 (`ak:refresh:<hash>` 존재 여부)
  3. **새 액세스 토큰 생성** (JWT, 15분)
  4. **새 리프레시 토큰 생성** (UUID 해시)
  5. Redis에서 **기존 리프레시 토큰 삭제**
  6. Redis에 **새 리프레시 토큰 저장**: `ak:refresh:<new_hash>` → `<session_id>` (TTL 7일)
  7. `users_login_log` 테이블에 rotate 로그 기록

**3. 로그아웃 (`POST /auth/logout`)**:
- 요청:
  - **헤더**: `Authorization: Bearer <ACCESS_TOKEN>`
  - **쿠키**: `ak_refresh=<refresh_token>`
- 성공 응답 (204 No Content)
- 동작:
  1. Claims에서 `user_id` 추출
  2. Redis에서 리프레시 토큰 삭제 (`DEL ak:refresh:<hash>`)
  3. `users_login_log` 테이블에 로그아웃 기록
  4. 쿠키 삭제: `Set-Cookie: ak_refresh=; Max-Age=0`

#### Redis 키 패턴 & TTL

| 키 패턴 | 값 | TTL | 용도 |
|---------|-----|-----|------|
| `ak:session:{session_id}` | user_id (i64) | 15분 | 액세스 토큰 유효성 빠른 확인 |
| `ak:refresh:{refresh_hash}` | session_id (UUID) | 역할별 (1/7/30일) | 리프레시 토큰 검증 |
| `ak:user_sessions:{user_id}` | Set\<session_id\> | - | 전체 로그아웃 + 동시 세션 수 제한 (SCARD) |
| `rl:login:{email}:{ip}` | 시도 횟수 (i64) | 15분 | 로그인 Rate Limiting (10회/15분) |
| `rl:find_id:{ip}` | 시도 횟수 (i64) | 15분 | 아이디 찾기 Rate Limiting |
| `rl:reset_pw:{ip}` | 시도 횟수 (i64) | 15분 | 비밀번호 재설정 Rate Limiting |

> **참고**: `ak:session`, `ak:refresh` TTL은 `config.rs`의 `jwt_access_ttl_min`, 역할별 `refresh_ttl_secs` 값 기준

#### 동시 세션 수 제한

로그인 시 `enforce_session_limit()`가 활성 세션 수를 검증한다.

| 역할 | 최대 세션 | 초과 시 정책 | 환경변수 |
|------|:---------:|-------------|---------|
| HYMN | 2 | 로그인 거부 (403) | `MAX_SESSIONS_HYMN` |
| Admin | 2 | 로그인 거부 (403) | `MAX_SESSIONS_ADMIN` |
| Manager | 3 | 로그인 거부 (403) | `MAX_SESSIONS_MANAGER` |
| Learner | 5 | 가장 오래된 세션 자동 퇴장 (FIFO) | `MAX_SESSIONS_LEARNER` |

- **유령 세션 정리**: `SMEMBERS` + `EXISTS` 체크 후 만료된 세션 자동 제거.
  - refresh_hash 는 `find_login_refresh_hashes_by_session_ids` 로 **배치 조회** (N+1 제거).
  - DB 상태 업데이트는 `update_login_states_by_sessions` 로 **배치 UPDATE**.
- **FIFO 퇴장**: `find_active_sessions_oldest` 가 `(session_id, refresh_hash)` 튜플을
  반환해 eviction 루프에서 추가 DB 조회 없이 Redis 키를 즉시 정리.
- **Fail-closed**: `enforce_session_limit` 의 모든 Redis 호출은 `unwrap_or` 가 아닌
  `?` 로 에러 전파. 일시적 Redis 장애가 유효 세션을 "만료" 로 오판해 강제 로그아웃
  시키는 것을 방지.
- **FIFO 보호**: DB 가 빈 목록을 반환하면 Redis SET 은 무순서라 FIFO 를 보장할 수 없다.
  조용히 랜덤 eviction 하는 대신 에러 로그 + 요청 실패로 중단 (`500`).
- **적용 지점**: `login()` (비MFA), `create_oauth_session()` (OAuth/MFA)
- **거부 에러**: `403 AUTH_403_SESSION_LIMIT:{max_sessions}`

#### 에러 케이스 & HTTP 상태 코드

| 시나리오 | HTTP 상태 | 설명 |
|---------|----------|------|
| 로그인 성공 | 200 OK | 액세스 + 리프레시 토큰 발급 |
| 로그인 실패 (이메일/비밀번호 불일치) | 401 Unauthorized | `{ "code": "invalid_credentials", "message": "..." }` |
| 로그인 실패 (계정 비활성화) | 403 Forbidden | `{ "code": "account_disabled", "message": "..." }` |
| 리프레시 성공 | 200 OK | 새 액세스 + 리프레시 토큰 발급 |
| 리프레시 실패 (토큰 만료/없음) | 401 Unauthorized | 재로그인 필요 |
| 리프레시 실패 (토큰 위조/Redis 없음) | 401 Unauthorized | 재로그인 필요 |
| 로그아웃 성공 | 204 No Content | 세션 삭제 완료 |
| 로그아웃 실패 (미인증) | 401 Unauthorized | 세션 없음 |
| 보호된 엔드포인트 (토큰 없음) | 401 Unauthorized | `Authorization` 헤더 누락 |
| 보호된 엔드포인트 (토큰 만료) | 401 Unauthorized | 리프레시 필요 |
| 보호된 엔드포인트 (권한 부족) | 403 Forbidden | RBAC 불일치 (예: learner가 admin 경로 접근) |

#### 프론트엔드 연동 (Section 6.4.1 참조)

**인증 상태 관리 (Zustand + TanStack Query)**:
- **전역 상태 (Zustand)**:
  ```typescript
  interface AuthState {
    authStatus: "pass" | "stop" | "forbid";
    user: UserDto | null;
    setAuth: (status: "pass" | "stop" | "forbid", user?: UserDto) => void;
  }
  ```

- **TanStack Query 훅 예시**:
  ```typescript
  // 로그인
  const loginMutation = useMutation({
    mutationFn: (dto: LoginDto) => apiClient.post('/auth/login', dto),
    onSuccess: (data) => {
      setAuth("pass", data.user);
      // 액세스 토큰은 Axios Interceptor에서 자동 관리
      // 리프레시 토큰은 쿠키로 자동 전송
    },
    onError: (error) => {
      if (error.status === 401) toast.error("이메일 또는 비밀번호가 잘못되었습니다");
      if (error.status === 403) toast.error("계정이 비활성화되었습니다");
    }
  });

  // 리프레시
  const refreshMutation = useMutation({
    mutationFn: () => apiClient.post('/auth/refresh'),
    onSuccess: (data) => {
      // 새 액세스 토큰은 Interceptor에서 자동 저장
      // 새 리프레시 토큰은 쿠키로 자동 수신
    },
    onError: () => {
      setAuth("stop");
      router.push("/login");
    }
  });

  // 로그아웃
  const logoutMutation = useMutation({
    mutationFn: () => apiClient.post('/auth/logout'),
    onSuccess: () => {
      setAuth("stop", null);
      router.push("/login");
    }
  });
  ```

- **Axios Interceptor (자동 토큰 관리)**:
  ```typescript
  // Request Interceptor: 액세스 토큰 자동 추가
  apiClient.interceptors.request.use((config) => {
    const token = localStorage.getItem('access_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  });

  // Response Interceptor: 401 에러 시 자동 리프레시
  apiClient.interceptors.response.use(
    (response) => response,
    async (error) => {
      if (error.response?.status === 401 && !error.config._retry) {
        error.config._retry = true;
        try {
          const { data } = await apiClient.post('/auth/refresh');
          localStorage.setItem('access_token', data.access_token);
          error.config.headers.Authorization = `Bearer ${data.access_token}`;
          return apiClient(error.config);
        } catch (refreshError) {
          // 리프레시 실패 → 로그인 페이지로
          setAuth("stop");
          router.push("/login");
          return Promise.reject(refreshError);
        }
      }
      return Promise.reject(error);
    }
  );
  ```

#### 보안 원칙 (Section 7.1 참조)

1. **리프레시 토큰 Rotate-on-Use**:
   - 매번 리프레시 시 새 토큰 발급 + 기존 토큰 즉시 무효화
   - 토큰 재사용 공격 방어

2. **Redis 세션 TTL 관리**:
   - 리프레시 토큰: 7일 TTL
   - 로그아웃 시 즉시 삭제

3. **쿠키 보안 옵션**:
   - `HttpOnly`: XSS 공격 방어
   - `SameSite=Lax`: CSRF 공격 방어
   - `Secure`: HTTPS 전송 강제 (프로덕션)

4. **액세스 토큰 저장 위치**:
   - 프론트엔드: `localStorage` (빠른 접근, XSS 리스크 있으나 httpOnly 쿠키로 리프레시 보호)
   - 대안: `sessionStorage` (탭 닫으면 자동 삭제)

5. **JWT 서명 검증**:
   - 백엔드에서 HS256 알고리즘으로 검증
   - 위조 토큰 자동 거부 (401 응답)

#### 백엔드 구현 참조

- **코드 위치**:
  - 백엔드: `src/api/auth/` (handler, service, repo)
    - `handler.rs`: 엔드포인트 정의 (login, refresh, logout)
    - `service.rs`: 비즈니스 로직 (토큰 생성, 검증, rotate)
    - `repo.rs`: DB/Redis 접근 (세션 저장, 로그 기록)
    - `jwt.rs`: JWT 인코딩/디코딩
    - `token_utils.rs`: 리프레시 토큰 생성/검증
  - 프론트엔드: `frontend/src/category/auth/` (api, hooks, types)
    - `api.ts`: API 클라이언트 함수
    - `hooks/useAuth.ts`: TanStack Query 훅
    - `types.ts`: DTO 타입 정의 (ReadOnly)

#### 상태축 매핑 (프론트엔드 ↔ 백엔드)

| 백엔드 상태 | 프론트엔드 상태 (`authStatus`) | UI 동작 |
|------------|------------------------------|---------|
| 인증 성공 (Claims 추출 성공) | `"pass"` | 보호된 콘텐츠 표시 |
| 인증 실패 (토큰 없음/만료) | `"stop"` | `/login` 리디렉션 + "로그인이 필요합니다" 메시지 |
| 권한 부족 (RBAC 불일치) | `"forbid"` | 403 에러 페이지 + "접근 권한이 없습니다" 메시지 |
| 계정 비활성화 (`user_state=false`) | `"forbid"` | "계정이 비활성화되었습니다" 메시지 |

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 4. 데이터 모델 개요 (요약)

> 전체 DDL/컬럼은 `amk_schema_patched.sql` 기준.
> 여기서는 **주요 도메인과 테이블 역할**만 요약한다.

### 4.1 사용자 도메인 (USERS)

- `users`
  - 회원 정보 (이메일, 비밀번호 해시, 이름, 국가, 언어, 생년월일, 성별 등)
  - `user_auth_enum` ('HYMN', 'admin', 'manager', 'learner') 사용자 권한
  - `user_state` : boolean 타입 (true = on, false = off) 사용자 계정 활성 여부
  - `user_language_enum` ('ko', 'en') 사용자 구사 언어
  - `user_gender_enum` ('none', 'male', 'female', 'other') 사용자 성별
  - **암호화 컬럼** (AES-256-GCM, Phase 2C 이후 평문 제거 완료):
    - `user_email_enc`, `user_email_idx` (blind index) — 이메일
    - `user_name_enc`, `user_name_idx` — 이름
    - `user_birthday_enc` — 생년월일
    - `user_phone_enc`, `user_phone_idx` — 전화번호
  - **MFA 컬럼** (2026-02-14 추가):
    - `user_mfa_secret` (TEXT) — TOTP 비밀키 (AES-256-GCM 암호화)
    - `user_mfa_enabled` (BOOLEAN DEFAULT false) — MFA 활성화 여부
    - `user_mfa_backup_codes` (TEXT) — 백업 코드 (SHA-256 해시 JSON, AES-256-GCM 암호화)
    - `user_mfa_enabled_at` (TIMESTAMPTZ) — MFA 최초 활성화 시각
- `users_log`
  - 회원 정보 활동 기록
  - `user_action_log_enum` ('signup', 'find_id', 'reset_pw', 'update') 사용자 활동 이력
  - `user_auth_enum` ('HYMN', 'admin', 'manager', 'learner') 사용자 권한 이력
  - `user_language_enum` ('ko', 'en') 사용자 구사 언어 이력
  - `user_gender_enum` ('none', 'male', 'female', 'other') 사용자 성별 이력
- `users_setting`
  - 사용자 관련 UI 언어, 타임존, 알림 등 개인 설정
  - `user_set_language_enum` ('ko', 'en') 사용자 설정 언어
  - `user_set_timezone` (VARCHAR) 사용자 타임존 (예: "Asia/Seoul", "America/New_York") — 최초 가입 시 자동 감지, 이후 수동 변경만 허용 (→ 3.1 참고)
- `admin_users_log`
  - 사용자 관련 관리자 활동 기록
  - `admin_action_enum` ('create', 'update', 'banned', 'reorder', 'publish', 'unpublish') 관리자 활동 이력
  - `ip_address` (TEXT) — 관리자 IP 주소 (AES-256-GCM 암호화 저장)
- `user_export_data`
  - 개인정보 내보내기/백업 요청 상태 및 결과 관리(비동기 처리용)

### 4.2 인증 로그인 도메인 (AUTH LOGIN)

- `login`
  - 로그인 정보(지역, 방식, 시간, 상태)
  - `login_device_enum` ('mobile', 'tablet', 'desktop', 'other') 로그인 기기
  - `login_method_enum` ('email', 'google', 'apple') 로그인 방법
  - `login_state_enum` ('active', 'revoked', 'expired', 'logged_out', 'compromised') 로그인 상태
  - `login_os`, `login_browser`, `login_device`: 서버사이드 User-Agent 파싱(`woothee`)으로 자동 채움
  - `login_expire_at`: 로그인 시 `NOW() + refresh_ttl` 기록, 토큰 갱신 시 갱신
  - `login_active_at`: 토큰 갱신(refresh) 시 `NOW()` 업데이트
  - `login_revoked_reason`: 세션 상태 변경 사유 기록 (기본값 `none`, revoke 시: `password_changed`, `security_concern`, `admin_action`, `account_disabled`)
- `login_log`
  - 로그인 정보 활동 이력(로그인 이벤트, 세부 지역, 세부 방식)
  - `login_event_enum` ('login', 'logout', 'refresh', 'rotate', 'fail', 'reuse_detected') 로그인 활동 이력
  - `login_device_enum` ('mobile', 'tablet', 'desktop', 'other') 로그인 기기 이력
  - `login_method_enum` ('email', 'google', 'apple') 로그인 방법 이력
  - `login_access_log` (char(64)): access token SHA-256 해시 (감사 추적용)
  - `login_token_id_log` (varchar): JWT `jti` claim 값 (토큰 식별용)
  - `login_fail_reason_log` (text): 실패 사유 (기본값 `none`, 실패 시: `invalid_credentials`, `account_disabled`, `token_reuse`)
- `redis_session`
  - Key: ak:session:< sid >
  - TTL은 expire_at 기준. 세션 본문은 직렬화(JSON 등)하되, 운영 상 조회 필드는 컬럼으로 문서화.
  - `login_state_enum` ('active', 'revoked', 'expired', 'logged_out', 'compromised') 로그인 상태
- `redis_refresh`
  - Key: ak:refresh:< hash > -> < sid >
  - 로테이션(rotate-on-use) 시 refresh_hash 교체. 재사용 탐지 시 세션 일괄 폐기 정책과 연동.
- `redis_user_sessions`
  - Key: ak:user_sessions:< uid > (set/list 모델을 행 단위로 전개)
  - 실제 Redis에서는 set/list로 보관. dbdiagram 문서화를 위해 행 형태로 표현.
- `user_oauth`
  - OAuth 소셜 로그인 연동 정보 (Google, Apple 등)
  - `login_method_enum` ('email', 'google', 'apple') OAuth 제공자
  - `oauth_subject` — OAuth 제공자의 고유 사용자 ID (sub claim)
  - `oauth_email`, `oauth_name`, `oauth_picture_url` — 제공자로부터 받은 프로필 정보
  - 동일 이메일 기존 계정 자동 연결, 신규 이메일은 자동 회원가입

### 4.3 비디오 도메인 (VIDEOS)

- `video`
  - 동영상 강의 정보(vimeo 링크, 상태, 접근)
  - `video_state_enum` ('ready', 'open', 'close') 강의 상태
  - `video_access_enum` ('public', 'paid', 'private', 'promote') 강의 접근
  - `video_duration` (INT, nullable) — 영상 길이 (초, Vimeo API 동기화)
  - `video_thumbnail` (TEXT, nullable) — 썸네일 URL (Vimeo API 동기화)
- `video_log`
  - 동영상 강의 시청 정보(진행, 완료, 횟수, 접속정보)
- `video_tag`
  - 동영상 강의 메타 정보(제목, 부제목)
- `video_tag_map`
  - 동영상 강의 맵핑 : `video_tag` - `video`
- `video_stat_daily`
  - 동영상 일별 통계 : UTC 기준
- `admin_video_log`
  - 동영상 강의 관련 관리자 활동 기록
  - `admin_action_enum` ('create', 'update', 'banned', 'reorder', 'publish', 'unpublish') 관리자 활동 이력

### 4.4 학습 도메인 (STUDY)

- `study`
  - 학습 문제 정보(상태, 프로그램, 문제 정보)
  - `study_state_enum` ('ready', 'open', 'close') 학습 문제 상태
  - `study_program_enum` ('basic_pronunciation', 'basic_word', 'basic_900', 'topik_read', 'topik_listen', 'topik_write', 'tbc') 학습 프로그램 분류
- `study_task`
  - 학습 문제 세부 정보(종류, 순서)
  - `study_task_kind_enum` ('choice', 'typing', 'voice') 학습 문제 유형
- `study_task_choice`
  - 학습 문제 : 4지 선다 (정답 1~4)
- `study_task_typing`
  - 학습 문제 : 쓰기 / 타이핑
- `study_task_voice`
  - 학습 문제 : 발음
- `study_task_explain`
  - 학습 문제 해설(해설 언어, 해설 내용)
  - `user_set_language_enum` ('ko', 'en') 해설 제공 언어
- `study_task_status`
  - 학습 상태(시도 횟수, 최고점, 완료여부)
- `study_task_log`
  - 학습 문제 풀이 기록(시도 횟수, 최고점, 완료여부, 풀이내용, 접속정보)
  - `study_task_log_action_enum` ('view', 'start', 'answer', 'finish', 'explain', 'status') 학습 행동 이력
- `admin_study_log`
  - 학습 문제 관련 관리자 활동 기록
  - `admin_action_enum` ('create', 'update', 'banned', 'reorder', 'publish', 'unpublish') 관리자 활동 이력

### 4.5 수업 구성 도메인 (LESSON)

- `lesson`
  - 수업 구성 : 동영상 강의 + 학습 문제(내용 설명)
- `lesson_item`
  - 수업 구성 아이템 : 순서 지정(순서, 종류)
  - `lesson_item_kind_enum` ('video', 'task') 수업 구성 종류
- `lesson_progress`
  - 수업 구성 : 학습 진도 사항(진도율, 마지막 아이템)
- `admin_lesson_log`
  - 수업 구성 관련 관리자 세부 정보
  - `admin_action_enum` ('create', 'update', 'banned', 'reorder', 'publish', 'unpublish') 관리자 활동 이력

> 상세 스키마 변경이 필요하면, 항상 이 문서와 `amk_schema_patched.sql`을 함께 업데이트한다.

### 4.6 코스 도메인 (COURSE) ✅ 구현 완료

- `course`
  - 코스/강좌 정보 (제목, 설명, 타입, 상태, 접근 권한)
  - `course_type` ('video', 'study', 'live', 'package')
  - `course_state` ('active', 'inactive', 'deleted')
- `course_lesson`
  - 코스-레슨 맵핑 (순서, 접근 권한)
- `user_course`
  - 사용자별 수강 정보 (구매/체험/만료 상태)
- `admin_course_log`
  - 코스 관련 관리자 활동 기록

### 4.7 향후 업데이트 도메인

- `live`
  - 실시간 강의 : ZOOM API 연동을 통한 실시간 강의 서비스 관련 테이블
  - `live_state` ('ready', 'open', 'close')
- `live_zoom`
  - 줌 연동 정보
  - `live_zoom_state` ('pending', 'registered', 'failed')
- `live_log`
  - 라이브 강의 참여 로그

### 4.8 번역 도메인 (TRANSLATION)

> 다국어 콘텐츠 번역을 관리하는 도메인. 모든 학습 콘텐츠(코스, 레슨, 비디오, 학습 문제 등)의 번역을 단일 테이블로 통합 관리한다.

- `content_translations`
  - 번역 데이터: content_type + content_id + field_name + lang 조합으로 번역 관리
  - `translation_id` (PK, BIGSERIAL)
  - `content_type` (content_type_enum): 번역 대상 콘텐츠 유형
  - `content_id` (BIGINT): 대상 콘텐츠의 PK
  - `field_name` (VARCHAR): 번역 대상 필드명 (예: title, description)
  - `lang` (supported_language_enum): 번역 언어
  - `translated_text` (TEXT): 번역된 텍스트
  - `status` (translation_status_enum): 번역 상태 (draft → reviewed → approved)
  - `created_at`, `updated_at` (TIMESTAMPTZ)
  - **UNIQUE**: (content_type, content_id, field_name, lang)

- **Enums**
  - `content_type_enum`: `'course'`, `'lesson'`, `'video'`, `'video_tag'`, `'study'`, `'study_task_choice'`, `'study_task_typing'`, `'study_task_voice'`, `'study_task_explain'`
    - `'video'` = 비디오 제목/부제 번역, `'video_tag'` = 비디오 태그 번역, `'study_task_explain'` = 학습 해설 번역
  - `translation_status_enum`: `'draft'`, `'reviewed'`, `'approved'`
  - `supported_language_enum`: `'ko'`, `'en'`, `'zh-CN'`, `'zh-TW'`, `'ja'`, `'vi'`, `'id'`, `'th'`, `'my'`, `'km'`, `'mn'`, `'ru'`, `'uz'`, `'kk'`, `'tg'`, `'ne'`, `'si'`, `'hi'`, `'es'`, `'pt'`, `'fr'`, `'de'` (22개, `ko`는 원본 언어, 아랍어 제외 — RTL 별도 대응 필요)

### 4.9 결제 도메인 (PAYMENT)

> Paddle Billing 기반 구독 결제 시스템. 구독, 트랜잭션, Webhook 이벤트를 관리한다.

- `subscriptions`
  - 사용자 구독 정보: Paddle 구독 ID, 상태, 결제 주기, 가격, 기간
  - `subscription_id` (PK, BIGSERIAL)
  - `user_id` (BIGINT, FK → users)
  - `payment_provider` (payment_provider_enum): 결제 제공자
  - `provider_subscription_id` (VARCHAR, UNIQUE): Paddle 구독 ID
  - `provider_customer_id` (VARCHAR): Paddle 고객 ID
  - `status` (subscription_status_enum): 구독 상태
  - `billing_interval` (billing_interval_enum): 결제 주기
  - `current_price_cents` (INT): 현재 가격 (센트 단위)
  - `current_period_start`, `current_period_end` (TIMESTAMPTZ): 현재 구독 기간
  - `trial_ends_at`, `canceled_at`, `paused_at` (TIMESTAMPTZ): 상태 변경 시간
  - `provider_data` (JSONB): Paddle 원본 데이터
  - **UNIQUE**: `provider_subscription_id`

- `transactions`
  - 결제 트랜잭션 기록: Paddle 트랜잭션 ID, 금액, 세금
  - `transaction_id` (PK, BIGSERIAL)
  - `subscription_id` (BIGINT, FK → subscriptions)
  - `user_id` (BIGINT, FK → users)
  - `payment_provider` (payment_provider_enum)
  - `provider_transaction_id` (VARCHAR, UNIQUE): Paddle 트랜잭션 ID
  - `status` (transaction_status_enum): completed/refunded/partially_refunded
  - `amount_cents` (INT): 결제 금액 (센트)
  - `tax_cents` (INT): 세금 (센트)
  - `currency` (VARCHAR): 통화 코드
  - `billing_interval` (billing_interval_enum): 결제 주기
  - `occurred_at` (TIMESTAMPTZ): 결제 발생 시간
  - `provider_data` (JSONB): Paddle 원본 데이터

- `webhook_events`
  - Webhook 이벤트 멱등성 관리: 중복 처리 방지
  - `webhook_event_id` (PK, BIGSERIAL)
  - `payment_provider` (payment_provider_enum)
  - `provider_event_id` (VARCHAR): Paddle 이벤트 ID
  - `event_type` (VARCHAR): 이벤트 유형 (subscription.activated 등)
  - `payload` (JSONB): 원본 페이로드
  - `processed_at` (TIMESTAMPTZ): 처리 시간
  - **UNIQUE**: (payment_provider, provider_event_id)

- **Enums**
  - `payment_provider_enum`: `'paddle'`
  - `subscription_status_enum`: `'trialing'`, `'active'`, `'past_due'`, `'paused'`, `'canceled'`
  - `transaction_status_enum`: `'completed'`, `'refunded'`, `'partially_refunded'`
  - `billing_interval_enum`: `'month_1'`, `'month_3'`, `'month_6'`, `'month_12'`

### 4.10 교재 주문 도메인 (TEXTBOOK)

> 비회원 교재 주문 시스템. 계좌이체 기반, 20개 언어 × 2종(학생용/교사용), ₩25,000/권, 최소 10권.
> 마이그레이션: `migrations/20260226_textbook.sql`, `migrations/20260303_textbook_improvements.sql`

- `textbook`
  - 교재 주문 기본 정보 (신청자, 배송, 결제, 세금계산서, 금액)
  - `order_code` (VARCHAR(20), UNIQUE): 외부 공개용 주문번호 (TB-YYMMDD-NNNN, Advisory Lock으로 중복 방지)
  - `textbook_order_status_enum` ('pending', 'confirmed', 'paid', 'printing', 'shipped', 'delivered', 'canceled')
  - `textbook_payment_method_enum` ('bank_transfer')
  - `tracking_number`, `tracking_provider`: 배송 추적 정보
  - `is_deleted`, `deleted_at`: Soft Delete 지원
  - CHECK: `tax_invoice = false OR tax_biz_number IS NOT NULL`
- `textbook_item`
  - 주문 항목 (1:N, FK RESTRICT)
  - `textbook_language_enum` (20개 언어)
  - `textbook_type_enum` ('student', 'teacher')
  - `quantity` (CHECK ≥ 1), `unit_price` (₩25,000), `subtotal`
- `admin_textbook_log`
  - 관리자 작업 기록 (상태 변경, 배송 추적 수정, 삭제 등, FK RESTRICT)

- **Enums**
  - `textbook_language_enum`: `'ja'`, `'zh_cn'`, `'zh_tw'`, `'vi'`, `'th'`, `'id'`, `'my'`, `'mn'`, `'ru'`, `'es'`, `'pt'`, `'fr'`, `'de'`, `'hi'`, `'ne'`, `'si'`, `'km'`, `'uz'`, `'kk'`, `'tg'`
  - `textbook_type_enum`: `'student'`, `'teacher'`
  - `textbook_order_status_enum`: `'pending'`, `'confirmed'`, `'paid'`, `'printing'`, `'shipped'`, `'delivered'`, `'canceled'`
  - `textbook_payment_method_enum`: `'bank_transfer'` (추후 `'card'`, `'paddle'` 추가)

### 4.11 E-book 도메인 (EBOOK)

> 회원 전용 e-book 웹 뷰어 시스템. 페이지 이미지 기반 (EPUB/PDF 미노출, 다운로드 없음).
> 마이그레이션: `migrations/20260310_ebook.sql`

- `ebook_purchase`
  - e-book 구매 기록 (회원 전용, `user_id` FK)
  - `purchase_code` (VARCHAR(20), UNIQUE): 구매번호 (EB-YYMMDD-NNNN, Advisory Lock)
  - `ebook_edition_enum` ('student', 'teacher')
  - `ebook_purchase_status_enum` ('pending', 'completed', 'refunded')
  - `ebook_payment_method_enum` ('paddle', 'bank_transfer')
  - `paddle_txn_id`: Paddle transaction ID (결제 완료 시 저장)
  - `is_deleted`, `completed_at`, `refunded_at`: Soft Delete + 상태 타임스탬프
  - 가격: 교사용 ₩15,000 / 학생용 ₩12,000 (KRW)
- `ebook_access_log`
  - 페이지 조회 감사 로그 (purchase_id, user_id, page_number, watermark_id, IP, UA)
  - 워터마크 추적 연동 (법적 증거)
- `admin_ebook_log`
  - 관리자 작업 기록 (상태 변경, 삭제 등, FK RESTRICT)

- **Enums**
  - `ebook_edition_enum`: `'student'`, `'teacher'`
  - `ebook_purchase_status_enum`: `'pending'`, `'completed'`, `'refunded'`
  - `ebook_payment_method_enum`: `'paddle'`, `'bank_transfer'`

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 5. 기능 & API 로드맵 (도메인 인덱스)

> 상세 엔드포인트 스펙은 도메인별 문서로 분할됨 (2026-03-17).
> 아래에는 공통 범례와 도메인 문서 인덱스만 유지한다.


### 5.0 Phase 로드맵 체크박스 범례

| 기호 | 의미 | 설명 |
|------|------|------|
| ✅ | 백엔드 완료 | API 엔드포인트, 비즈니스 로직, DB 마이그레이션 완료 |
| 🆗 | 프론트엔드 완료 | 화면 구현, API 연동, 상태 관리 완료 |
| ⚠️ | 부분 완료 | 기본 기능은 동작하나 리팩토링/최적화 필요 |
| ❌ | 미착수 | 아직 구현 시작 안 함 |
| 🔄 | 진행 중 | 현재 작업 중 |

**표기 예시**:
- `[✅]` → 백엔드만 완료
- `[✅🆗]` → 백엔드 + 프론트엔드 모두 완료
- `[✅⚠️]` → 백엔드 완료, 프론트엔드 부분 완료
- `[🔄]` → 백엔드 또는 프론트엔드 작업 진행 중
- `[❌]` → 미착수

---

### 상태축 (State Axis) 정의

| 축 | 상태 값 | 상태 설명 |
|---|---|---|
| **Auth** | `pass` | 인증 완료(접근 허용) |
|  | `stop` | 인증 불가(로그인 필요/토큰 없음·만료) |
|  | `forbid` | 접근 불가(권한 부족·차단 상태) |
| **Session** | `active` | 세션 발급 |
|  | `expiring` | 세션 재발급 필요 임박 |
|  | `rotating` | 세션 리프레시 중 |
|  | `expired` | 세션 만료 |
| **Page** | `init` | 페이지 로딩 중(초기 진입·프리페치) |
|  | `ready` | 페이지 로딩 완료(입력/상호작용 가능) |
| **Data** | `empty` | 데이터 없음(오류 아님, 0개 결과) |
|  | `present` | 데이터 존재(정상 바인딩) |
|  | `error` | 데이터 조회 실패(404/500 등) |
| **Form** | `pristine` | 화면 구성 직후, 입력 전 상태 |
|  | `dirty` | 사용자 입력 발생(변경됨) |
|  | `validating` | 클라이언트 검증 중(형식·범위 확인) |
|  | `submitting` | 서버로 제출 중(중복 제출 차단) |
|  | `success` | 서버 저장 성공(후속 이동/토스트) |
|  | `error.client` | 클라이언트 검증 실패(형식/범위 오류) |
|  | `error.conflict` | 서버 충돌(예: 409 이메일 중복) |
| **Request** | `pending` | 네트워크 요청 진행 중 |
|  | `success` | 네트워크 요청 성공 |
|  | `error` | 네트워크 요청 실패 |
|  | `retryable` | 일시 장애로 재시도 가능(예: 503/네트워크) |
| **Course** | `buy` | 구매 완료, 구매 권한 존재 |
|  | `taster` | 체험판, 체험 권한 존재 |
|  | `buy-not` | 비구매, 구매 권한 없음 |
|  | `checking` | 구매 여부 및 구매 권한 점검 |


---


### 5.1 도메인별 상세 문서 인덱스

> §5.1~§5.16의 상세 엔드포인트 스펙은 아래 도메인 문서로 분할됨 (2026-03-17).
> 각 문서에는 Phase 번호, 엔드포인트, 화면 경로, 기능 명칭, 점검사항, 시나리오가 포함됨.

| 문서 | Phase | 도메인 | 상태 |
|------|-------|--------|------|
| [AMK_API_LEARNING.md](./AMK_API_LEARNING.md) | 1, 4~6, 8~9 | health, video, study, lesson, course, translation | ✅🆗 |
| [AMK_API_USER.md](./AMK_API_USER.md) | 2, 7 | user CRUD, admin 사용자 관리 | ✅🆗 |
| [AMK_API_AUTH.md](./AMK_API_AUTH.md) | 3 | 로그인, OAuth, MFA, 비밀번호, 이메일 인증 | ✅🆗 |
| [AMK_API_PAYMENT.md](./AMK_API_PAYMENT.md) | 10~11 | Paddle 결제, 구독, 웹훅 | ✅🆗 |
| [AMK_API_TEXTBOOK.md](./AMK_API_TEXTBOOK.md) | 12 | 교재 주문 | ✅🆗 |
| [AMK_API_EBOOK.md](./AMK_API_EBOOK.md) | 12.5 | E-book 웹 뷰어 | ✅🆗 |
| [AMK_API_FUTURE.md](./AMK_API_FUTURE.md) | 13~16 | 콘텐츠 시딩, 발음, 조음, TTS (미구현) | ❌ |

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 6. 프론트엔드 구조 & 규칙

> 이 섹션은 [AMK_FRONTEND.md](./AMK_FRONTEND.md)로 분리됨 (2026-03-17).

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 7. 엔지니어링 가이드

> 이 섹션은 [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md) §0 엔지니어링 원칙으로 통합됨 (2026-03-17).

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 8. 작업 현황

> 이 섹션은 [AMK_STATUS.md](./AMK_STATUS.md)로 분리됨 (2026-03-17).

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 9. 변경 이력

> 상세 변경 이력은 [`AMK_CHANGELOG.md`](./AMK_CHANGELOG.md)로 분리됨 (2026-02-17).

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

**문서 끝 (End of Document)**
