---
title: AMK_API_MASTER — Amazing Korean API  Master Spec
updated: 2026-02-16
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

- [5. 기능 & API 로드맵 (Phase / 화면 / 엔드포인트 / 상태 / DoD)](#5-기능--api-로드맵-phase--화면--엔드포인트--상태--dod)
  - [5.0 Phase 로드맵 체크박스 범례](#50-phase-로드맵-체크박스-범례)
  - [5.1 Phase 1 — health](#51-phase-1--health-)
  - [5.2 Phase 2 — user](#52-phase-2--user-)
  - [5.3 Phase 3 — auth](#53-phase-3--auth-)
  - [5.4 Phase 4 — video](#54-phase-4--video-)
  - [5.5 Phase 5 — study](#55-phase-5--study-)
  - [5.6 Phase 6 — lesson](#56-phase-6--lesson-)
  - [5.7 Phase 7 — admin](#57-phase-7--admin-)
  - [5.8 Phase 8 — course](#58-phase-8--course-)
  - [5.9 Phase 9 — translation (i18n)](#59-phase-9--translation-i18n)
  - [5.10 Phase 10 — 관리자 결제/구독 관리](#510-phase-10--관리자-결제구독-관리--수동-수강권-)
  - [5.11 Phase 11 — 사용자 결제 (Paddle Billing)](#511-phase-11--사용자-결제-paddle-billing-)

- [6. 프론트엔드 구조 & 규칙](#6-프론트엔드-구조--규칙)
  - [6.1 프론트엔드 스택 & 기본 원칙](#61-프론트엔드-스택--기본-원칙)
  - [6.2 프론트 디렉터리 구조 & 컴포넌트 계층](#62-프론트-디렉터리-구조--컴포넌트-계층)
    - [6.2.4 다국어(i18n) 아키텍처](#624-다국어i18n-아키텍처)
  - [6.3 라우팅 & 접근 제어](#63-라우팅--접근-제어)
  - [6.4 상태 관리 & API 연동 패턴](#64-상태-관리--api-연동-패턴)
  - [6.5 UI/UX & Tailwind 규칙 (shadcn/ui System)](#65-uiux--tailwind-규칙-shadcnui-system)
  - [6.6 프론트 테스트 & 로컬 개발 (요약)](#66-프론트-테스트--로컬-개발-요약)

- [7. 작업 방식 / 엔지니어링 가이드 (요약)](#7-작업-방식--엔지니어링-가이드-요약)
  - [7.1 작업 원칙](#71-작업-원칙)
  - [7.2 개발 플로우](#72-개발-플로우)
  - [7.3 DTO/검증 규칙 (요약)](#73-dto검증-규칙-요약)
  - [7.4 서비스 계층 및 파일 구조](#74-서비스-계층-및-파일-구조)
  - [7.5 트랜잭션 패턴](#75-트랜잭션-패턴)
  - [7.6 테스트 & 자동화](#76-테스트--자동화)

- [8. Open Questions & 설계 TODO](#8-open-questions--설계-todo)
  - [8.1 RBAC / 관리자 권한](#81-rbac--관리자-권한)
  - [8.2 Admin action log actor 연결](#82-admin-action-log-actor-연결)
  - [8.3 페이징 고도화 (Keyset vs Page)](#83-페이징-고도화-keyset-vs-page)
  - [8.4 테스트 전략](#84-테스트-전략)
  - [8.5 보안/운영 (후순위 계획)](#85-보안운영-후순위-계획)
  - [8.6 코드 일관성 (Technical Debt)](#86-코드-일관성-technical-debt)
  - [8.7 작업 로드맵](#87-작업-로드맵)
  - [8.8 데이터 모니터링 & 접근](#88-데이터-모니터링--접근)
  - [8.9 디자인 & UI](#89-디자인--ui)
  - [8.10 마케팅 & 데이터 분석](#810-마케팅--데이터-분석)
  - [8.11 한국어 발음 교정 AI (Pronunciation Coaching AI)](#811-한국어-발음-교정-ai-pronunciation-coaching-ai)

- [9. 변경 이력 (요약)](#9-변경-이력-요약)

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
  - **Open Questions & 설계 TODO**
- 을 한 파일에서 관리하기 위함.

### 0.2 사용 원칙

- **스펙 / 기능 / 엔드포인트를 변경할 때는 항상 이 파일을 먼저 수정**한다.
- 코드/마이그레이션/테스트를 변경한 뒤에는, 여기의 관련 섹션(Phase 표, 규칙, TODO)을 반드시 갱신한다.
- 과거 md 문서들은 모두 **참고용 아카이브**이며, 새로운 정보는 **여기에만 적는다**.

### 0.3 관련 파일

- **데이터베이스 스키마**: [`docs/AMK_SCHEMA_PATCHED.md`](./AMK_SCHEMA_PATCHED.md) - 전체 DDL 정의
- **코드 예시 (Best Practices)**: [`docs/AMK_CODE_PATTERNS.md`](./AMK_CODE_PATTERNS.md) - 백엔드/프론트엔드 검증된 코드 패턴
- **배포 & 운영 가이드**: [`docs/AMK_DEPLOY_OPS.md`](./AMK_DEPLOY_OPS.md) - 빌드, 배포, CI/CD, 유지보수
- **개발 파이프라인**: [`docs/AMK_PIPELINE.md`](./AMK_PIPELINE.md) - 멀티 AI 오케스트레이션, 작업 흐름, 역할 분리
- 이 문서는 레포 내 `docs/AMK_API_MASTER.md` 경로에 위치하는 것을 기본으로 한다.

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
    - **기초(Foundation)**: 900문장 패턴 습득을 통한 문법/회화 기초 완성
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
    - **Tailwind CSS**: 유틸리티 퍼스트 CSS 프레임워크
    - **Shadcn/ui**: 재사용 가능한 컴포넌트 라이브러리 (Radix UI 기반)
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
- 프로덕션 fail-fast: `APP_ENV=production` + `EMAIL_PROVIDER=none` → 서버 부팅 실패

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
PADDLE_PRODUCT_ID=pro_xxx            # 상품 ID
PADDLE_PRICE_MONTH_1=pri_xxx         # 1개월 구독 Price ID ($10)
PADDLE_PRICE_MONTH_3=pri_xxx         # 3개월 구독 Price ID ($25)
PADDLE_PRICE_MONTH_6=pri_xxx         # 6개월 구독 Price ID ($50)
PADDLE_PRICE_MONTH_12=pri_xxx        # 12개월 구독 Price ID ($100)
```

**코드 구조**
- `src/external/payment.rs`: `PaymentProvider` trait + `PaddleProvider` 구현 (paddle-rust-sdk)
- `src/state.rs`: `AppState.payment: Option<Arc<dyn PaymentProvider>>`
- `src/config.rs`: Paddle 환경변수 9개 + `billing_interval_for_price()` 매핑
- `src/api/payment/`: 사용자 결제 API (plans, subscription, webhook)
- `src/api/admin/payment/`: 관리자 결제 관리 API

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
| `ak:user_sessions:{user_id}` | Set\<session_id\> | - | 전체 로그아웃 시 세션 목록 |
| `rl:login:{email}:{ip}` | 시도 횟수 (i64) | 15분 | 로그인 Rate Limiting (10회/15분) |
| `rl:find_id:{ip}` | 시도 횟수 (i64) | 15분 | 아이디 찾기 Rate Limiting |
| `rl:reset_pw:{ip}` | 시도 횟수 (i64) | 15분 | 비밀번호 재설정 Rate Limiting |

> **참고**: `ak:session`, `ak:refresh` TTL은 `config.rs`의 `jwt_access_ttl_min`, 역할별 `refresh_ttl_secs` 값 기준

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
  - `currency` (VARCHAR): 통화 코드 (USD)
  - `current_period_start`, `current_period_end` (TIMESTAMPTZ): 현재 구독 기간
  - `trial_ends_at`, `canceled_at`, `paused_at` (TIMESTAMPTZ): 상태 변경 시간
  - `provider_meta` (JSONB): Paddle 원본 데이터
  - **UNIQUE**: `provider_subscription_id`

- `transactions`
  - 결제 트랜잭션 기록: Paddle 트랜잭션 ID, 금액, 세금
  - `transaction_id` (PK, BIGSERIAL)
  - `subscription_id` (BIGINT, FK → subscriptions)
  - `user_id` (BIGINT, FK → users)
  - `payment_provider` (payment_provider_enum)
  - `provider_transaction_id` (VARCHAR, UNIQUE): Paddle 트랜잭션 ID
  - `status` (transaction_status_enum): completed/refunded
  - `amount_cents` (INT): 결제 금액 (센트)
  - `tax_cents` (INT): 세금 (센트)
  - `currency` (VARCHAR): 통화 코드
  - `billing_interval` (billing_interval_enum): 결제 주기
  - `occurred_at` (TIMESTAMPTZ): 결제 발생 시간
  - `provider_meta` (JSONB): Paddle 원본 데이터

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
  - `transaction_status_enum`: `'completed'`, `'refunded'`
  - `billing_interval_enum`: `'month_1'`, `'month_3'`, `'month_6'`, `'month_12'`

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 5. 기능 & API 로드맵 (Phase / 화면 / 엔드포인트 / 상태 / DoD)

> 이 섹션은 **기존 `AMK_Feature_Roadmap.md`의 내용을 기준으로 한다.**
> 아래 표들은 _Phase / 엔드포인트 / 화면 경로 / 기능 명칭 / 점검사항 / UX 규칙 / 기능 완료_ 를 나타내며,
> 마지막 열의 체크박스는 구현 완료 여부를 의미한다.


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

### 5.1 Phase 1 — health ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 | 
|---|---|---|---|---|---|
| 1-1 | `GET /healthz` | `/health` | 라이브 헬스 | ***서버 작동 여부 확인***<br>**성공:** Auth pass / Page : healthz init→ready / Request : healthz pending→success / Data : healthz present → **200**<br>**실패:** Auth pass / Page : healthz init→ready / Request : healthz pending→error / Data : healthz error → **500** | [✅🆗] |
| 1-2 | `GET /docs` | `/docs` | API 문서 | ***Swagger 태그 순서 고정(health → auth → user → videos → study → lesson → admin)***<br>**성공:** Auth pass / Page : docs init→ready / Request : docs pending→success / Data : docs present → **200**<br>**실패(스키마 집계 실패):** Auth pass / Page : docs init→ready / Request : docs pending→error / Data : docs error → **500**<br>**실패(정적 경로 누락):** Auth pass / Page : docs init→ready / Request : docs pending→error / Data : docs error → **404** | [✅] |

---

<details>
  <summary>Phase 1 — health 시나리오</summary>
  
#### 5.1-1 : `GET /healthz` 시나리오
- **성공**
  - When: 클라이언트가 `GET /healthz` 호출, Swagger에서만 실행
  - Then: `200 OK`, JSON 바디 `{"status":"live","uptime_ms":..., "version":"v1.0.0"}`
  - **PROD-5**: `APP_ENV=production`이면 `version` 필드 생략 (`Option<String>`, `skip_serializing_if`)
  - 상태축: Auth=pass / Page=init→ready / Request=pending→success / Data=present
- **실패**
  - When: 헬스 핸들러 내부 예외
  - Then: `500 Internal Server Error`, 에러 바디 `{"error":{"http_status":500,"code":"HEALTH_INTERNAL"}}`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→error / Data=error

---

#### 5.1-2 : `GET /docs` 시나리오
- **PROD-6**: `ENABLE_DOCS=false` (프로덕션 기본)이면 Swagger UI 비활성화 → 404 반환
- **성공** (`ENABLE_DOCS=true`일 때)
  - When: 클라이언트가 `GET /docs` 호출, Swagger에서만 실행
  - Then: `200 OK`, Swagger UI 렌더링, **태그 순서가 user→auth→videos→lesson→admin→health**로 보임
  - 상태축: Auth=pass / Page=init→ready / Request=pending→success / Data=present
- **실패(스키마 집계 실패)**
  - When: OpenAPI 집계/리소스 로드 오류
  - Then: `500 Internal Server Error`, `{"error":{"http_status":500,"code":"DOCS_BUILD_FAIL"}}`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→error / Data=error
- **실패(정적 경로 누락)**
  - When: 리버스 프록시/정적 경로 미설정
  - Then: `404 Not Found`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→error / Data=error
</details>

---

### 5.2 Phase 2 — user ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 | 
|---|---|---|---|---|---|
| 2-1 | `POST /users` | `/signup` | 회원가입 | ***USERS, USERS_LOG 저장 + 세션/토큰 발급***<br>성공: Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→submitting→success / Request signup pending→success / Data signup present → **201**<br>실패(형식/누락): Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→error.client / Request signup pending→error / Data signup empty → **400**<br>실패(도메인 제약): Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→error.client / Request signup pending→error / Data signup error → **422**<br>실패(중복/충돌): Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→error.conflict / Request signup pending→error / Data signup error → **409**<br>실패(레이트리밋): Auth pass / Page signup ready / Form signup error.client / Request signup pending→error / Data signup error → **429** | [✅🆗] |
| 2-2 | `GET /users/me` | `/me` | 내 정보 조회 | ***USERS 안전 필드(비밀번호 제외)***<br>성공: Auth pass / Page me init→ready / Request me pending→success / Data me present → **200**<br>실패(미인증): Auth stop / Page me init→ready / Request me pending→error / Data me error → **401**<br>실패(미존재/비활성): Auth pass / Page me init→ready / Request me pending→error / Data me error → **404** | [✅🆗] |
| 2-3 | `POST /users/me` | `/me/edit` | 내 정보 수정 | ***USERS 일부 업데이트 → USERS_LOG 저장***<br>성공: Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→submitting→success / Request me_edit pending→success / Data me_edit present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→error.client / Request me_edit pending→error / Data me_edit empty → **400**<br>실패(도메인 제약): Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→error.client / Request me_edit pending→error / Data me_edit error → **422**<br>실패(미인증): Auth stop / Page me_edit init→ready / Request me_edit pending→error / Data me_edit error → **401**<br>실패(충돌·고유제약): Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→error.conflict / Request me_edit pending→error / Data me_edit error → **409** | [✅🆗] |
| 2-4 | `GET /users/me/settings` | `/settings` | 내 설정 조회 | ***USERS_SETTING 조회***<br>성공: Auth pass / Page settings init→ready / Request settings pending→success / Data settings present → **200**<br>실패(미인증): Auth stop / Page settings init→ready / Request settings pending→error / Data settings error → **401** | [✅🆗] |
| 2-5 | `POST /users/me/settings` | `/settings` | 내 설정 수정 | ***USERS_SETTING 수정 → USERS_LOG 저장***<br>성공: Auth pass / Page settings init→ready / Form settings pristine→dirty→validating→submitting→success / Request settings pending→success / Data settings present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page settings init→ready / Form settings pristine→dirty→validating→error.client / Request settings pending→error / Data settings empty → **400**<br>실패(도메인 제약): Auth pass / Page settings init→ready / Form settings pristine→dirty→validating→error.client / Request settings pending→error / Data settings error → **422**<br>실패(미인증): Auth stop / Page settings init→ready / Request settings pending→error / Data settings error → **401** | [✅🆗] |

---

<details>
  <summary>5.2 Phase 2 — user 시나리오</summary>

#### 공통 정책(1-1 ~ 1-5)
- **응답 에러 스키마(고정)**  
  `{ "error": { "http_status": 400|401|404|409|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **로그 정책**: **성공/실패 모두 USERS_LOG 기록**(민감정보 제외, 실패 시 에러코드/사유 포함)
- **검증 기준**: **400**=형식/누락/파싱, **422**=도메인 제약 위반
- **중복 제출 방지**: Form=`submitting` 동안 UI 차단 + 서버 시간/조건 기반 방지
- **레이트리밋(우선 대상: 1-1)**: 과도 시 **429** + `Retry-After`
- **성공 후 페이지 전환**: 성공 시 다음 화면으로 이동하여 **Form 수명주기 종료**

#### 5.2-1 : `POST /users` (회원가입)
- **성공 → 201 Created**
  - When: `/signup` 폼 입력 후 제출이 서버 검증을 통과한다
  - Then: **201**
    - **Body**: `SignupRes { message, requires_verification }`
    - `requires_verification: true` → 이메일 인증코드 발송됨, 프론트엔드에서 `/verify-email` 페이지로 이동
    - `requires_verification: false` → 개발 환경(`EMAIL_PROVIDER=none`) 자동 인증, 즉시 로그인 가능
    - **자동 로그인 제거**: 회원가입 시 토큰/세션 발급 없음 (이메일 인증 후 로그인 필요)
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` pristine→dirty→validating→submitting→success** / Request=`signup` pending→success / Data=`signup` present
  - 로그: USERS insert 후 **USERS_LOG(성공 스냅샷)** 기록(민감정보 제외)
  - **미인증 재가입**: 동일 이메일로 `user_check_email=false`인 기존 레코드 존재 시 비밀번호/프로필 **덮어쓰기** + 새 인증코드 발송 (409 대신)
  - **인증코드 보안**: Redis에 HMAC-SHA256 해시 저장 (평문 저장 금지), blind index 키 사용
- **실패(형식/누락) → 400 Bad Request**
  - 예: 이메일 형식 불일치, 필수 항목 누락, JSON 파싱 오류
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.client** / Request=`signup` pending→error / **Data=`signup` empty**
  - 에러 바디: `{ "error": { "http_status": 400, "code": "BAD_REQUEST", "message": "...", "trace_id": "..." } }`
  - 로그: **USERS_LOG(실패 이벤트)** 기록(에러코드/사유, 민감값 마스킹)
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: birthday 범위 위반, 금지값, 정책 규칙 위반, 약한 비밀번호
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.client** / Request=`signup` pending→error / **Data=`signup` error**
  - 에러 바디: `http_status:422, code:"UNPROCESSABLE_ENTITY"`
  - 로그: 실패 이벤트 기록
- **실패(중복/충돌) → 409 Conflict**
  - 예: 이메일 UNIQUE 충돌 (인증 완료된 기존 계정)
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.conflict** / Request=`signup` pending→error / **Data=`signup` error**
  - 에러 바디: `http_status:409, code:"CONFLICT"`
  - 로그: 실패 이벤트 기록
- **실패(레이트리밋) → 429 Too Many Requests**
  - 조건: 과도한 가입 시도
  - 상태축: Auth=pass / Page=`signup` ready / **Form=`signup` error.client** / Request=`signup` pending→error / **Data=`signup` error**
  - 헤더: `Retry-After: <seconds>`
  - 로그: 실패 이벤트 + 차단 지표
- **중복 제출 방지(정책)**
  - 프론트: **Form=submitting** 동안 버튼/Enter 비활성
  - 백엔드: 미인증 재가입 시 덮어쓰기 + 새 코드 발송, 인증 완료 계정은 409

---

#### 5.2-2 : `GET /users/me` (내 정보 조회)
- **성공 → 200 OK**
  - When: 인증된 사용자가 `/me` 화면에서 자기 정보를 조회한다
  - Then: **200**, 안전 필드만 반환(비밀번호·해시·토큰 제외)
  - 상태축: Auth=pass / Page=`me` init→ready / Request=`me` pending→success / **Data=`me` present**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료/서명 오류
  - Then: **401**, `WWW-Authenticate: Bearer ...`
  - 상태축: **Auth=stop** / Page=`me` init→ready / Request=`me` pending→error / **Data=`me` error**
- **실패(미존재/비활성) → 404 Not Found**
  - When: 토큰은 유효하나 사용자 계정이 비활성/삭제 처리되어 조회 불가
  - Then: **404**
  - 상태축: Auth=pass / Page=`me` init→ready / Request=`me` pending→error / **Data=`me` error**

---

#### 5.2-3 : `POST /users/me` (내 정보 수정)
- **성공 → 200 OK(또는 204)**
  - When: `/me/edit` 폼 입력 후 제출이 서버 검증을 통과한다
  - Then: **200**(변경 후 스냅샷 바디) **또는 204**, USERS 업데이트 후 **USERS_LOG(성공 스냅샷)** 기록
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` pristine→dirty→validating→submitting→success** / Request=`me_edit` pending→success / **Data=`me_edit` present**
- **실패(형식/누락) → 400 Bad Request**
  - 예: 이메일 포맷 오류, 필수 필드 누락, JSON 파싱 실패
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` … → error.client** / Request=`me_edit` pending→error / **Data=`me_edit` empty**
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: birthday 범위 위반, 허용되지 않은 locale 값 등
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` … → error.client** / Request=`me_edit` pending→error / **Data=`me_edit` error**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
  - 상태축: **Auth=stop** / Page=`me_edit` init→ready / Request=`me_edit` pending→error / **Data=`me_edit` error**
- **실패(충돌/고유제약) → 409 Conflict**
  - 예: 닉네임/이메일 고유 제약 충돌 정책을 허용하는 경우
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` … → error.conflict** / Request=`me_edit` pending→error / **Data=`me_edit` error**

---

#### 5.2-4 : `GET /users/me/settings` (내 설정 조회)
- **성공 → 200 OK**
  - When: 인증된 사용자가 `/settings`에서 설정을 조회한다
  - Then: **200**, USERS_SETTING 반환
  - 상태축: Auth=pass / Page=`settings` init→ready / Request=`settings` pending→success / **Data=`settings` present**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
  - Then: **401**
  - 상태축: **Auth=stop** / Page=`settings` init→ready / Request=`settings` pending→error / **Data=`settings` error**

---

#### 5.2-5 : `POST /users/me/settings` (내 설정 수정)
- **성공 → 200 OK(또는 204)**
  - When: `/settings` 폼 입력 후 제출이 서버 검증을 통과한다
  - Then: **200**(최신 설정 반환) **또는 204**, USERS_SETTING 수정 후 **USERS_LOG(성공 스냅샷)** 기록
  - 상태축: Auth=pass / Page=`settings` init→ready / **Form=`settings` pristine→dirty→validating→submitting→success** / Request=`settings` pending→success / **Data=`settings` present**
- **실패(형식/누락) → 400 Bad Request**
  - 예: 필수 설정 누락, JSON 파싱 실패
  - 상태축: Auth=pass / Page=`settings` init→ready / **Form=`settings` … → error.client** / Request=`settings` pending→error / **Data=`settings` empty**
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: 언어 코드 범위/우선순위 규칙 위반(선호 언어 배열 유효성)
  - 상태축: Auth=pass / Page=`settings` init→ready / **Form=`settings` … → error.client** / Request=`settings` pending→error / **Data=`settings` error**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
  - Then: **401**
  - 상태축: **Auth=stop** / Page=`settings` init→ready / Request=`settings` pending→error / **Data=`settings` error**
</details>

---

### 5.3 Phase 3 — auth ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 | 
|---|---|---|---|---|---|
| 3-1 | `POST /auth/login` | `/login` | 로그인 | ***LOGIN/LOGIN_LOG 저장 + Redis 세션/리프레시 발급***<br>성공: Auth pass / Page login init→ready / Form login pristine→dirty→validating→submitting→success / Request login pending→success / Data login present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page login init→ready / Form login pristine→dirty→validating→error.client / Request login pending→error / Data login empty → **400**<br>실패(도메인 제약): Auth pass / Page login init→ready / Form login pristine→dirty→validating→error.client / Request login pending→error / Data login error → **422**<br>실패(자격증명 불일치): Auth stop / Page login ready / Form login error.client / Request login pending→error / Data login error → **401**<br>실패(계정 상태/차단): Auth forbid / Page login ready / Form login error.client / Request login pending→error / Data login error → **403**(또는 **423**)<br>실패(레이트리밋): Auth pass / Page login ready / Form login error.client / Request login pending→error / Data login error → **429** | [✅🆗] | 
| 3-2 | `POST /auth/logout` | `/logout` | 로그아웃 | ***세션/리프레시 키 제거, LOGIN_LOG 저장***<br>성공: Auth pass / Page logout ready / Request logout pending→success / Data logout present → **204**(또는 **200**)<br>실패(미인증/세션 없음): Auth stop / Page logout ready / Request logout pending→error / Data logout error → **401** | [✅🆗] |
| 3-2a | `POST /auth/logout/all` | (전역처리) | 전체 로그아웃 | ***사용자의 모든 세션/리프레시 키 일괄 제거, LOGIN_LOG 저장***<br>성공: Auth pass / Request logout_all pending→success → **204**<br>실패(미인증): Auth stop → **401** | [✅] |
| 3-3 | `POST /auth/refresh` | (전역처리) | 토큰 재발급 | ***리프레시 로테이션/검증/재사용탐지 + 로그(rotate)***<br>성공: Auth pass / Page app ready / Request refresh pending→success / Data refresh present → **200**<br>실패(형식/누락): Auth pass / Page app ready / Request refresh pending→error / Data refresh empty → **400**<br>실패(도메인 제약): Auth pass / Page app ready / Request refresh pending→error / Data refresh error → **422**<br>실패(리프레시 무효/만료): Auth stop / Page app ready / Request refresh pending→error / Data refresh error → **401**<br>실패(재사용탐지/위조): Auth forbid / Page app ready / Request refresh pending→error / Data refresh error → **409**(또는 **403**) | [✅🆗] |
| 3-4 | `POST /auth/find-id` | `/find-id` | 회원 아이디 찾기 | ***개인정보 보호: 결과 폭로 금지(Enumeration Safe), USERS_LOG 저장***<br>성공(요청 수락/존재 여부와 무관):<br> Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→submitting→success / Request find_id pending→success / Data find_id present → **200**(항상 동일 메시지)<br>실패(형식/누락): Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→error.client / Request find_id pending→error / Data find_id empty → **400**<br>실패(도메인 제약): Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→error.client / Request find_id pending→error / Data find_id error → **422**<br>실패(레이트리밋): Auth pass / Page find_id ready / Form find_id error.client / Request find_id pending→error / Data find_id error → **429** | [✅🆗] |
| 3-5a | `POST /auth/request-reset` | `/reset-password` | 비밀번호 재설정 요청 | ***이메일 기반 인증코드 발송 (Resend), Redis 코드 저장 (TTL 10분)***<br>성공(항상 동일 응답): Auth pass / Request pending→success → **200** `{ message, remaining_attempts }`<br>실패(형식/누락): **400** / 실패(레이트리밋): **429** | [✅🆗] |
| 3-5b | `POST /auth/verify-reset` | `/reset-password` | 비밀번호 재설정 검증 | ***인증코드 검증 + 새 비밀번호 설정, 관련 세션 전부 무효화***<br>성공: Auth pass / Request pending→success → **200**<br>실패(코드 만료/무효): **401** / 실패(형식): **400** / 실패(레이트리밋): **429** | [✅🆗] |
| 3-5 | `POST /auth/reset-pw` | `/reset-password` | 회원 비밀번호 재설정 (legacy) | ***요청→검증→재설정의 단일 엔드포인트(토큰/코드 포함), USERS_LOG 저장***<br>성공(재설정 완료):<br> Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→submitting→success / Request reset_pw pending→success / Data reset_pw present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→error.client / Request reset_pw pending→error / Data reset_pw empty → **400**<br>실패(도메인 제약): Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→error.client / Request reset_pw pending→error / Data reset_pw error → **422**<br>실패(토큰/코드 무효·만료): Auth stop / Page reset_pw ready / Form reset_pw error.client / Request reset_pw pending→error / Data reset_pw error → **401**<br>실패(레이트리밋): Auth pass / Page reset_pw ready / Form reset_pw error.client / Request reset_pw pending→error / Data reset_pw error → **429** | [✅🆗] |
| 3-6 | `GET /auth/google`<br>`GET /auth/google/callback` | `/login` | Google OAuth 로그인 | ***Google OAuth 2.0 Authorization Code Flow, 자동 계정 연결/생성, USER_OAUTH/LOGIN/LOGIN_LOG 저장***<br>성공(OAuth 시작): Auth pass / Page login ready / Request google pending→success / Data google_auth_url present → **200**<br>성공(OAuth 콜백): Auth pass / Page login redirect→ready / Request callback pending→success / Data login present → **302**(프론트엔드 리다이렉트)<br>실패(OAuth 설정 누락): Auth pass / Page login ready / Request google pending→error / Data google error → **500**<br>실패(State 검증 실패/CSRF): Auth stop / Page login ready / Request callback pending→error / Data callback error → **401**<br>실패(사용자 취소): Auth pass / Page login ready / Request callback pending→error / Data callback error → **302**(에러 정보와 함께 리다이렉트) | [✅🆗] |
| 3-7 | `POST /auth/verify-email` | `/verify-email` | 이메일 인증코드 확인 | ***회원가입 이메일 인증, HMAC-SHA256 해시 비교 (constant-time), user_check_email=true 업데이트***<br>성공: **200** `{ message, verified: true }`<br>실패(코드 무효/만료): **401** / 실패(형식): **400** / 실패(레이트리밋): **429** (10회/시간) | [✅] |
| 3-8 | `POST /auth/resend-verification` | `/verify-email` | 이메일 인증코드 재발송 | ***미인증 사용자에게 새 인증코드 발송 (Enumeration Safe — 항상 동일 메시지)***<br>성공: **200** `{ message, remaining_attempts }` (항상 성공 메시지)<br>실패(형식): **400** / 실패(레이트리밋): **429** (5회/5시간) / 실패(이메일 서비스): **503** | [✅] |
| 3-9 | `POST /auth/find-password` | `/account-recovery` | 비밀번호 찾기 (통합) | ***본인확인(이름+생일+이메일) → 인증코드 발송, Enumeration Safe, OAuth 전용 계정도 동일 응답***<br>성공: **200** `{ message, remaining_attempts }` (항상 동일 메시지)<br>실패(형식): **400** / 실패(레이트리밋): **429** (5회/5시간) | [✅] |
| 3-10 | `POST /auth/mfa/setup` | `/admin/mfa/setup` | MFA 설정 시작 | ***TOTP 비밀키 생성 + QR코드 반환, AES-256-GCM 암호화 저장***<br>성공: **200** `{ secret, qr_code_data_uri, otpauth_uri }`<br>실패(미인증): **401** / 실패(이미 활성화): **409** | [✅] |
| 3-11 | `POST /auth/mfa/verify-setup` | `/admin/mfa/setup` | MFA 설정 확인 | ***TOTP 코드 검증 → MFA 활성화 + 백업코드 10개 생성/반환***<br>성공: **200** `{ enabled: true, backup_codes: [...] }`<br>실패(미인증): **401** / 실패(코드 무효): **401** | [✅] |
| 3-12 | `POST /auth/mfa/login` | `/login` | MFA 2단계 인증 | ***MFA 토큰 + TOTP/백업코드 검증 → 세션 완료***<br>성공: **200** `{ access_token, ... }` + Set-Cookie(refresh_token)<br>실패(토큰 만료): **401** / 실패(코드 무효): **401** / 실패(레이트리밋): **429** (5회/5분) | [✅] |
| 3-13 | `POST /auth/mfa/disable` | (관리자) | MFA 비활성화 | ***HYMN 전용: 대상 사용자의 MFA 해제 + 전체 세션 무효화***<br>성공: **200** `{ disabled: true }`<br>실패(미인증): **401** / 실패(권한 없음): **403** | [✅] |

---

<details>
  <summary>5.3 Phase 3 — auth 시나리오 상세 (5.3-1 ~ 5.3-6)</summary>

#### 공통 정책(5.3-1 ~ 5.3-6)
- **에러 바디(고정)**  
  `{ "error": { "http_status": 400|401|403|409|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **로그**: 성공/실패 모두 이벤트 기록  
  - `LOGIN`(성공 상태), `LOGIN_LOG`(성공/실패, 원인, IP/UA 등), 사용자 관련 변경은 `USERS_LOG`  
- **검증 기준**: **400**=형식·누락·파싱, **422**=도메인 제약(길이·패턴·정책 위반)  
- **레이트리밋**: 로그인/비번재설정/아이디찾기엔 **429 + Retry-After**  
- **보안**: Enumeration Safe(아이디 찾기/재설정은 결과 노출 없이 동일 응답 문구)

---

#### 5.3-1 : `POST /auth/login` (로그인)
- **성공 → 200 OK(또는 204)**  
  - When: `/login`에서 이메일/비밀번호 제출(검증 통과)  
  - Then: **200**(또는 **204**), 액세스 토큰·리프레시 토큰 발급(쿠키/헤더), Redis 세션 및 리프레시 키 저장, `LOGIN`/`LOGIN_LOG` 기록  
  - 상태축: Auth=pass / Page=`login` init→ready / **Form=`login` pristine→dirty→validating→submitting→success** / Request=`login` pending→success / Data=`login` present / Session=active
- **실패(형식/누락) → 400**  
  - 예: 이메일 포맷 불일치, 필수 필드 누락, JSON 파싱 실패  
  - 상태축: Form=`login` … → error.client / Request … → error / Data=empty
- **실패(도메인 제약) → 422**  
  - 예: 허용되지 않은 로그인 방식, 비밀번호 정책 위반(클라이언트 강화 검증)  
- **실패(자격증명 불일치) → 401**  
  - 예: 이메일 존재하지만 비밀번호 불일치, 계정 없음  
  - 상태축: Auth=stop / Form error.client / Data error  
- **실패(계정 상태/차단) → 403(또는 423)**  
  - 예: user_state≠'on', 임시 잠금(여러 실패 시도 후)  
- **실패(레이트리밋) → 429**
  - 헤더: `Retry-After: <seconds>`
- **실패(소셜 전용 계정) → 401** (별도 에러 코드)
  - When: 이메일/비밀번호 로그인 시도, 해당 이메일이 소셜 로그인 전용 계정인 경우
  - Then: **401**, `{ "error": { "code": "UNAUTHORIZED", "message": "AUTH_401_SOCIAL_ONLY_ACCOUNT:google" } }`
  - 프론트엔드 처리: 소셜 로그인 유도 UI 표시 (amber 색상 안내 박스 + Google 로그인 버튼)
  - 상태축: Auth=stop / Form error.client / Data error (socialOnlyError)
- **실패(이메일 미인증) → 403** (별도 에러 코드)
  - When: 이메일/비밀번호 검증 성공했으나, `user_check_email=false`인 경우
  - Then: **403**, `{ "error": { "code": "FORBIDDEN", "message": "AUTH_403_EMAIL_NOT_VERIFIED:user@example.com" } }`
  - 프론트엔드 처리: `/verify-email` 페이지로 이동 (state에 email 전달), 재발송 버튼 사용 가능
  - 상태축: Auth=stop / Form error.client / Data error (emailNotVerifiedError)
  - **OAuth 자동 인증**: 미인증 이메일로 OAuth 로그인 시 `user_check_email=true` 자동 업데이트

---

#### 5.3-2 : `POST /auth/logout` (로그아웃)
- **성공 → 204 No Content(또는 200)**  
  - When: 사용자가 로그아웃 트리거  
  - Then: **204**, Redis의 세션/리프레시 키 제거, `LOGIN_LOG`(logout 이벤트) 기록  
  - 상태축: Auth=pass / Page=`logout` ready / Request=`logout` pending→success / Data=`logout` present / Session=expired
- **실패(미인증/세션 없음) → 401**  
  - 예: 유효한 세션/토큰 없이 호출

---

#### 5.3-3 : `POST /auth/refresh` (토큰 재발급)
- **성공 → 200 OK**  
  - When: 백그라운드 토큰 만료 임박/만료 후 리프레시 제출  
  - Then: **200**, 새 액세스/리프레시 발급(로테이션), Redis: `ak:refresh:<hash> -> <new_session_id>` 갱신, rotate 로그 기록  
  - 상태축: Auth=pass / Page=app ready / Request=`refresh` pending→success / Data=`refresh` present / Session=active
- **실패(형식/누락) → 400**  
  - 예: 리프레시 토큰 헤더/쿠키 누락  
- **실패(도메인 제약) → 422**  
  - 예: 허용되지 않은 클라이언트/디바이스 조합  
- **실패(무효/만료) → 401**  
  - 예: 만료·폐기된 리프레시, 서명 검증 실패  
- **실패(재사용탐지/위조) → 409(또는 403)**  
  - 정책: 재사용 탐지 시 기존 세션 무효화 + 알림/로그인 강제

---

#### 5.3-4 : `POST /auth/find_id` (회원 아이디 찾기)
- 성공 → **200**
  - When: `/find-id`에서 식별 정보(이름 + 이메일)를 입력하고 제출한다
  - Then: **200**, “일치 시 등록된 이메일로 안내가 발송되었습니다” **같은 문구**로 항상 응답(Enumeration Safe), `USERS_LOG` 기록
  - 상태축: Auth=pass / Page=`find_id` init→ready / Form=`find_id` pristine→dirty→validating→submitting→success / Request=`find_id` pending→success / Data=`find_id` present
- 실패(형식/누락) → **400**
  - 예: 필수 입력 누락, 형식 불일치(글자/숫자/이메일 패턴 등), JSON 파싱 오류
  - 상태축: Auth=pass / Page=`find_id` init→ready / Form=`find_id` … → error.client / Request=`find_id` pending→error / Data=`find_id` empty
- 실패(레이트리밋) → **429**
  - 조건: 과도한 시도 감지 시
  - 헤더: `Retry-After: <seconds>`
  - 상태축: Auth=pass / Page=`find_id` ready / Form=`find_id` error.client / Request=`find_id` pending→error / Data=`find_id` error

---

#### 5.3-5 : `POST /auth/reset_pw` (회원 비밀번호 재설정)
- **성공(재설정 완료) → 200 OK(또는 204)**
  - When: `/reset-password`에서 토큰/코드 + 새 비밀번호 제출
  - Then: **200**(또는 **204**), 비밀번호 해시 갱신, 관련 세션 전부 무효화(보안), `USERS_LOG` 기록
  - 상태축: Auth=pass / Page=`reset_pw` init→ready / **Form=`reset_pw` pristine→dirty→validating→submitting→success** / Request=`reset_pw` pending→success / Data=`reset_pw` present / Session=rotating→active
- **실패(형식/누락) → 400**, **실패(도메인 제약) → 422**
  - 예: 비밀번호 규칙 위반(길이/복잡성), 필수 누락
- **실패(토큰/코드 무효·만료) → 401**
  - 예: 만료 코드, 위조 토큰
- **실패(레이트리밋) → 429**

---

#### 5.3-6 : `GET /auth/google` & `GET /auth/google/callback` (Google OAuth 로그인)

> **개요**: Google OAuth 2.0 Authorization Code Flow를 통한 소셜 로그인. 기존 이메일 계정 자동 연결, 신규 사용자 자동 가입 지원.

**엔드포인트 구성**:
| 엔드포인트 | 설명 |
|-----------|------|
| `GET /auth/google` | OAuth 인증 URL 반환 (state/nonce 포함) |
| `GET /auth/google/callback` | Google 콜백 처리 → 토큰 발급 → 프론트엔드 리다이렉트 |

**DB 테이블**:
- `USER_OAUTH`: OAuth Provider 연결 정보 (user_id, provider, subject, email, name, picture)
- `LOGIN` / `LOGIN_LOG`: 로그인 세션 및 이력 기록 (login_method = 'google')

**보안 정책**:
- **State 파라미터**: Redis에 저장, 일회용 (CSRF 방지)
- **Nonce**: ID Token에 포함, Replay Attack 방지
- **JWKS 서명 검증**: Google JWKS 공개키로 RS256 서명 검증 (kid 매칭)
- **Audience 검증**: ID Token의 aud가 client_id와 일치해야 함
- **Issuer 검증**: `accounts.google.com` 확인

---

##### OAuth 시작 (`GET /auth/google`)
- **성공 → 200 OK**
  - When: 프론트엔드가 "Google로 로그인" 버튼 클릭 시 호출
  - Then: **200**, `{ auth_url: "https://accounts.google.com/o/oauth2/v2/auth?..." }` 반환
  - 처리: State/Nonce 생성 → Redis 저장 (TTL: 300초) → auth_url 구성
  - 상태축: Auth=pass / Page=`login` ready / Request=`google` pending→success / Data=`google_auth_url` present

- **실패(OAuth 설정 누락) → 500**
  - 예: GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, GOOGLE_REDIRECT_URI 환경변수 미설정
  - 상태축: Request=`google` pending→error / Data=`google` error

##### OAuth 콜백 (`GET /auth/google/callback`)
- **성공(로그인/가입 완료) → 302 Redirect**
  - When: Google 인증 완료 후 콜백 도착 (`?code=xxx&state=xxx`)
  - Then: **302**, 프론트엔드 `/login`으로 리다이렉트 (`?login=success&user_id=xxx&is_new_user=true|false`)
  - 처리 순서:
    1. State 검증 (Redis 조회 → 삭제)
    2. Authorization Code → Token 교환 (Google API)
    3. ID Token 디코딩 및 검증 (JWKS RS256 서명, nonce, aud, iss, exp)
    4. 사용자 조회/생성:
       - OAuth subject로 기존 연결 조회 → 있으면 로그인 (`is_new_user=false`)
       - 없으면 이메일로 기존 계정 조회 → 있으면 자동 연결 (`is_new_user=false`)
       - 없으면 신규 계정 생성 (`is_new_user=true`)
    5. 세션 생성 (JWT + Refresh Cookie)
    6. `LOGIN`, `LOGIN_LOG` 기록
  - **신규 OAuth 사용자 기본값**:
    | 필드 | 기본값 | 비고 |
    |------|--------|------|
    | `user_birthday` | `CURRENT_DATE` | 가입일 (미설정 표시용) |
    | `user_gender` | `none` | 미설정 |
    | `user_country` | `Unknown` | 미설정 |
    | `user_language` | `ko` | 한국어 (서비스 기본) |
    | `user_check_email` | `true` | Google 이메일 인증됨 |
    | `user_password` | `NULL` | 소셜 전용 계정 |
  - 상태축: Auth=pass / Page=`login` redirect→ready / Request=`callback` pending→success / Data=`login` present / Session=active

- **실패(State 검증 실패) → 302 Redirect (에러)**
  - 예: 만료된 state, 위조된 state (CSRF 시도)
  - Then: 프론트엔드로 리다이렉트 (`?error=oauth_failed&error_description=AUTH_401_INVALID_OAUTH_STATE`)
  - 상태축: Auth=stop / Request=`callback` pending→error

- **실패(Nonce 검증 실패) → 302 Redirect (에러)**
  - 예: ID Token의 nonce가 저장된 값과 불일치 (Replay Attack)
  - Then: 프론트엔드로 리다이렉트 (`?error=oauth_failed&error_description=AUTH_401_INVALID_NONCE`)

- **실패(사용자 취소) → 302 Redirect (에러)**
  - When: Google 동의 화면에서 사용자가 취소
  - Then: 프론트엔드로 리다이렉트 (`?error=oauth_error&error_description=access_denied: ...`)

##### 응답 스키마

**GoogleAuthUrlRes (OAuth 시작 응답)**
```json
{
  "auth_url": "https://accounts.google.com/o/oauth2/v2/auth?client_id=...&redirect_uri=...&response_type=code&scope=openid+email+profile&state=...&nonce=...&access_type=offline&prompt=consent"
}
```

**OAuth 콜백 성공 시 리다이렉트**
```
302 Found
Location: http://localhost:5173/login?login=success&user_id=123&is_new_user=true
Set-Cookie: ak_refresh=...; Path=/; HttpOnly; ...
```

| 파라미터 | 값 | 설명 |
|----------|-----|------|
| `login` | `success` | 로그인/가입 성공 |
| `user_id` | `123` | 사용자 ID |
| `is_new_user` | `true` / `false` | 신규 가입 여부 |

**프론트엔드 리다이렉트 분기**:
- `is_new_user=true` → `/user/me?welcome=true` (마이페이지 + 환영 메시지)
- `is_new_user=false` → `/about` (소개 페이지)

**OAuth 콜백 실패 시 리다이렉트**
```
302 Found
Location: http://localhost:5173/login?error=oauth_failed&error_description=...
```

---

##### 프론트엔드 OAuth 콜백 처리

**Hook**: `useOAuthCallback` (`frontend/src/category/auth/hook/use_oauth_callback.ts`)

**처리 흐름**:
1. LoginPage 마운트 시 URL 파라미터 확인 (`login`, `is_new_user`, `error`)
2. 에러 파라미터 있으면 → 토스트 에러 메시지 표시
3. 성공 파라미터 있으면:
   - `refreshToken()` 호출하여 access_token 획득
   - `useAuthStore.login()` 호출하여 로그인 상태 저장
   - `is_new_user` 값에 따라 적절한 페이지로 리다이렉트

**경쟁 조건(Race Condition) 처리**:
- axios interceptor와 OAuth 콜백 처리가 동시에 `refreshToken()`을 호출할 수 있음
- Refresh Token Rotation으로 인해 후자가 409 Conflict 발생 가능
- 해결: `refreshToken()` 실패 시 `isLoggedIn` 상태 확인 → true면 리다이렉트 진행

---

#### 5.3-7 : `POST /auth/verify-email` (이메일 인증코드 확인)

> **개요**: 회원가입 시 발송된 이메일 인증코드를 검증하여 `user_check_email=true`로 업데이트

- **성공 → 200 OK**
  - When: `/verify-email` 페이지에서 6자리 인증코드 입력
  - Then: **200**, `{ message, verified: true }`, `user_check_email=true` 업데이트
  - 보안: HMAC-SHA256 해시 비교 (constant-time), Redis 일회용 코드 삭제
- **실패(코드 무효/만료) → 401**
  - 예: 잘못된 코드, Redis TTL 만료 (10분), 이미 사용된 코드
- **실패(형식/누락) → 400**
  - 예: 이메일 형식 불일치, 코드 길이 불일치
- **실패(레이트리밋) → 429**
  - 조건: 10회/시간 초과

---

#### 5.3-8 : `POST /auth/resend-verification` (이메일 인증코드 재발송)

> **개요**: 미인증 사용자에게 새 이메일 인증코드 발송 (Enumeration Safe)

- **성공 → 200 OK**
  - When: `/verify-email` 페이지에서 "재전송" 버튼 클릭
  - Then: **200**, `{ message, remaining_attempts }` (이메일 존재 여부와 무관하게 항상 동일 메시지)
  - 동작: 미인증 사용자만 실제 이메일 발송, 이미 인증된/미존재 이메일은 발송 없이 성공 응답
- **실패(레이트리밋) → 429**
  - 조건: 5회/5시간 초과 (`RATE_LIMIT_EMAIL_WINDOW_SEC`, `RATE_LIMIT_EMAIL_MAX`)
- **실패(이메일 서비스) → 503**
  - 예: 이메일 프로바이더 연결 실패

---

#### 5.3-9 : `POST /auth/find-password` (비밀번호 찾기 — 통합 계정 복구)

> **개요**: 본인확인(이름+생일+이메일) 후 비밀번호 재설정 인증코드 발송. `/account-recovery` 페이지의 "비밀번호 찾기" 탭에서 사용.

- **성공 → 200 OK**
  - When: `/account-recovery` "비밀번호 찾기" 탭에서 이름, 생일, 이메일 입력
  - Then: **200**, `{ message, remaining_attempts }` (항상 동일 메시지, Enumeration Safe)
  - 본인확인: 이름(blind index) + 생일 + 이메일(blind index) 3중 매칭
  - OAuth 전용 계정(`user_password=NULL`): 동일 성공 응답 반환, 이메일 미발송
  - 매칭 실패: 동일 성공 응답 반환, 이메일 미발송 (타이밍 공격 방지)
- **실패(형식/누락) → 400**
  - 예: 필수 필드 누락, 이메일 형식 불일치
- **실패(레이트리밋) → 429**
  - 조건: 5회/5시간 초과 (IP 기반)

##### 프론트엔드 처리
- `/account-recovery` 탭 UI: "아이디 찾기" / "비밀번호 찾기"
- 비밀번호 찾기 탭에 OAuth 경고 문구 표시 (warning 스타일)
- Step 1(본인확인) → Step 2(인증코드 입력) → `POST /auth/verify-reset` → `/reset-password?token=xxx`
- 잔여 발송 횟수 표시, 한도 도달 시 재전송 버튼 비활성화

---

#### 5.3-10 : `POST /auth/mfa/setup` (MFA 설정 시작)
- **인증 필요**: Bearer 토큰 (AuthUser)
- **성공 → 200 OK**
  - TOTP 비밀키 생성 (`totp-rs` gen_secret)
  - AES-256-GCM 암호화 후 `users.user_mfa_secret`에 임시 저장 (enabled=false 상태)
  - QR 코드 data URI 생성 (`totp-rs` qr feature)
  - 응답: `{ secret: "BASE32...", qr_code_data_uri: "data:image/png;base64,...", otpauth_uri: "otpauth://totp/AmazingKorean:email?..." }`
- **실패(이미 활성화) → 409 Conflict**
- **실패(미인증) → 401 Unauthorized**

#### 5.3-11 : `POST /auth/mfa/verify-setup` (MFA 설정 확인)
- **인증 필요**: Bearer 토큰 (AuthUser)
- **요청**: `{ code: "123456" }` (6자리 TOTP)
- **성공 → 200 OK**
  - TOTP 코드 검증 (±1 step, 90초 허용)
  - 백업 코드 10개 생성 (8자 영숫자)
  - 백업 코드 SHA-256 해시 → JSON → AES-256-GCM 암호화 → DB 저장
  - `user_mfa_enabled=true`, `user_mfa_enabled_at=now()` 업데이트
  - 응답: `{ enabled: true, backup_codes: ["ABC12345", ...] }` (1회만 노출)
- **실패(코드 무효) → 401 Unauthorized**

#### 5.3-12 : `POST /auth/mfa/login` (MFA 2단계 인증)
- **인증 불필요** (mfa_token으로 인증)
- **요청**: `{ mfa_token: "uuid", code: "123456" }` (TOTP 6자리 또는 백업 코드 8자리)
- **플로우**:
  1. Redis `ak:mfa_pending:{mfa_token}` 조회 + 삭제 (일회용)
  2. Rate limit 확인: `rl:mfa:{user_id}:{ip}` (5회/5분)
  3. TOTP 코드 검증 시도 (6자리 숫자)
  4. TOTP 실패 시 백업 코드 검증 시도 (SHA-256 비교)
  5. 백업 코드 사용 시 해당 해시 목록에서 제거 + DB 갱신
  6. 성공 → 세션 생성 (기존 login 후반부 로직 재사용)
- **성공 → 200 OK**: `{ access_token, user_id, ... }` + Set-Cookie(refresh_token)
- **실패(토큰 만료/무효) → 401** `MFA_TOKEN_EXPIRED`
- **실패(코드 무효) → 401** `MFA_INVALID_CODE`
- **실패(레이트리밋) → 429**

#### 5.3-13 : `POST /auth/mfa/disable` (MFA 비활성화)
- **인증 필요**: Bearer 토큰 (AuthUser, HYMN 역할만)
- **요청**: `{ target_user_id: 123 }`
- **성공 → 200 OK**
  - 대상 사용자의 MFA 컬럼 초기화 (secret=NULL, enabled=false, backup_codes=NULL)
  - 대상 사용자의 모든 세션 무효화 (보안)
  - 응답: `{ disabled: true, user_id: 123 }`
- **실패(HYMN 아닌 경우) → 403 Forbidden**

##### MFA 로그인 흐름 (이메일/비밀번호)
1. `POST /auth/login` → 이메일/비밀번호 검증 통과
2. MFA 활성화 사용자 → `{ mfa_required: true, mfa_token: "uuid", user_id: 123 }` (세션 미생성)
3. `POST /auth/mfa/login` → TOTP/백업 코드 검증 → 세션 생성 완료

##### MFA 로그인 흐름 (Google OAuth)
1. `GET /auth/google/callback` → OAuth 인증 완료
2. MFA 활성화 사용자 → 프론트 리다이렉트: `/login?mfa_required=true&mfa_token=uuid&user_id=123`
3. `POST /auth/mfa/login` → TOTP/백업 코드 검증 → 세션 생성 완료

##### AdminRoute MFA 가드
- Admin/HYMN 역할 사용자가 MFA 미설정 시 `/admin/mfa/setup`으로 강제 이동
- MFA 설정 완료 후 관리자 페이지 접근 가능

##### Redis 키 패턴 (MFA)
| 키 | 타입 | TTL | 용도 |
|----|------|-----|------|
| `ak:mfa_pending:{mfa_token}` | STRING (JSON) | 300초 | MFA 인증 대기 (로그인 1단계 후) |
| `rl:mfa:{user_id}:{ip}` | STRING (counter) | 300초 | MFA 코드 검증 Rate Limit |

##### DB 컬럼 추가 (users 테이블)
| 컬럼 | 타입 | 설명 |
|------|------|------|
| `user_mfa_secret` | TEXT | TOTP 비밀키 (AES-256-GCM 암호화) |
| `user_mfa_enabled` | BOOLEAN DEFAULT false | MFA 활성화 여부 |
| `user_mfa_backup_codes` | TEXT | 백업 코드 (SHA-256 해시 JSON, AES-256-GCM 암호화) |
| `user_mfa_enabled_at` | TIMESTAMPTZ | MFA 최초 활성화 시각 |

</details>

---

### 5.4 Phase 4 — video ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 4-1 | `GET /videos` | `/videos` | 비디오 목록 | ***`video_url_vimeo` 불러오기, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page videos init→ready / Request videos pending→success / Data videos present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page videos init→ready / Request videos pending→success / Data videos empty → **200**<br>실패(형식/누락): Auth pass 또는 stop / Page videos init→ready / Request videos pending→error / Data videos error → **400**<br>실패(도메인 제약): Auth pass 또는 stop / Page videos init→ready / Request videos pending→error / Data videos error → **422** | [✅🆗] |
| 4-2 | `GET /videos/{id}` | `/videos/{videos_id}` | 비디오 상세 | ***VIDEO_TAG 조회, 시청 로그 트리거(클라이언트 재생 시)***<br>성공: Auth pass 또는 stop / Page video init→ready / Request video pending→success / Data video present → **200**<br>실패(없는 영상): Auth pass 또는 stop / Page video init→ready / Request video pending→error / Data video error → **404** | [✅🆗] |
| 4-3 | `GET /videos/{id}/progress` | `/videos/{videos_id}` | 진행도 조회 | ***VIDEO_LOG: `progress_percent`, `last_watched_at` 조회***<br>성공: Auth pass / Page video init→ready / Request progress pending→success / Data progress present(또는 empty=기록없음, 0%) → **200**<br>실패(미인증): Auth stop / Page video init→ready / Request progress pending→error / Data progress error → **401**<br>실패(없는 영상): Auth pass / Page video init→ready / Request progress pending→error / Data progress error → **404** | [✅🆗] |
| 4-4 | `POST /videos/{id}/progress` | `/videos/{videos_id}` | 진행도 갱신 | ***0~100 고정(멱등연산) → VIDEO_LOG 저장(`progress_percent`, `last_watched_at`)***<br>성공:<br> Auth pass / Page video init→ready / Form progress pristine→dirty→validating→submitting→success /<br> Request progress pending→success / Data progress present → **200**(또는 **204**)<br>실패(형식/누락):<br> Auth pass / Page video init→ready / Form progress pristine→dirty→validating→error.client / Request progress pending→error / Data progress empty → **400**<br>실패(도메인 제약: 범위/증감 규칙):<br> Auth pass / Page video init→ready / Form progress pristine→dirty→validating→error.client / Request progress pending→error / Data progress error → **422**<br>실패(미인증): Auth stop / Page video init→ready / Request progress pending→error / Data progress error → **401**<br>실패(없는 영상): Auth pass / Page video init→ready / Request progress pending→error / Data progress error → **404** | [✅🆗] |

---

<details>
  <summary>5.4 Phase 4 — video 시나리오 상세 (5.4-1 ~ 5.4-4)</summary>

#### 공통 정책(5.4-1 ~ 5.4-4)
- **에러 바디(고정)**
  `{ "error": { "http_status": 400|401|404|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **검증 기준**
  - **400** = 형식 오류/필수 누락/파싱 실패(예: page, per_page 숫자 아님)
  - **422** = 도메인 제약 위반(예: progress 0~100 범위 위반, 증가/감소 규칙 위반을 둘 경우)
- **진행도 규칙**
  - 멱등: 동일 값 재전송은 상태 변화 없이 성공
  - `last_watched_at`는 서버 시각으로 갱신
  - 기록 없음(progress 미생성)은 **200 + empty(0%)**로 응답(오류 아님)

---

#### 응답 스키마

**VideoListRes (목록 응답)**
```json
{
  "meta": {
    "total_count": 100,
    "total_pages": 5,
    "current_page": 1,
    "per_page": 20
  },
  "data": [VideoListItem, ...]
}
```

**VideoListItem (목록 아이템)**
| 필드 | 타입 | 설명 |
|------|------|------|
| `video_id` | `i64` | 비디오 고유 ID |
| `video_idx` | `string` | 비즈니스 식별 코드 (예: VID-001) |
| `title` | `string?` | 영상 제목 (video_tag에서 가져옴) |
| `subtitle` | `string?` | 영상 설명 (video_tag에서 가져옴) |
| `duration_seconds` | `i32?` | 영상 길이 (초, Vimeo 동기화) |
| `language` | `string?` | 언어 코드 |
| `thumbnail_url` | `string?` | 썸네일 URL (Vimeo 동기화) |
| `state` | `string` | 상태 (draft, published, archived) |
| `access` | `string` | 접근권한 (public, private, restricted) |
| `tags` | `string[]` | 태그 문자열 배열 |
| `has_captions` | `bool` | 자막 유무 |
| `created_at` | `datetime` | 생성일시 |

**VideoDetailRes (상세 응답)**
| 필드 | 타입 | 설명 |
|------|------|------|
| `video_id` | `i64` | 비디오 고유 ID |
| `video_url_vimeo` | `string` | Vimeo 영상 URL |
| `video_state` | `string` | 상태 (draft, published, archived) |
| `tags` | `VideoTagDetail[]` | 태그 상세 배열 |
| `created_at` | `datetime` | 생성일시 |

**VideoTagDetail (태그 상세)**
| 필드 | 타입 | 설명 |
|------|------|------|
| `key` | `string?` | 태그 키 |
| `title` | `string?` | 태그 제목 |
| `subtitle` | `string?` | 태그 설명 |

**VideoProgressRes (진행도 응답)**
| 필드 | 타입 | 설명 |
|------|------|------|
| `video_id` | `i64` | 비디오 고유 ID |
| `progress_rate` | `i32` | 진행률 (0~100) |
| `is_completed` | `bool` | 완료 여부 |
| `last_watched_at` | `datetime?` | 마지막 시청 시각 |

---

#### 5.4-1 : `GET /videos` (비디오 목록)
- **로그인 안해도 접근 가능**
- **성공(데이터 있음) → 200**
  - When: `/videos` 진입, `page/per_page/sort`가 유효
  - Then: **200**, 목록 + 페이지 메타, 각 항목에 `video_url_vimeo` 포함
  - 상태축: Auth=pass 또는 stop / Page=`videos` init→ready / Request=`videos` pending→success / Data=`videos` present

- **성공(데이터 없음) → 200**
  - Then: **200**, 빈 배열 + 페이지 메타
  - 상태축: Data=`videos` empty

- **실패(형식/누락) → 400**
  - 예: `page=abc`(숫자 아님), `per_page=foo`(숫자 아님), `sort=` 값 파싱 불가(쉼표/형식 오류)
  - 상태축: Auth=pass 또는 stop / Page=`videos` init→ready / Request=`videos` pending→error / Data=`videos` error

- **실패(도메인 제약) → 422**
  - 예: `page<1`, `per_page<1` 또는 허용 상한 초과(예: `per_page>100`), `sort` 값이 허용 목록 외, `lang` 필터가 허용되지 않은 언어코드
  - 상태축: Auth=pass 또는 stop / Page=`videos` init→ready / Request=`videos` pending→error / Data=`videos` error


---

#### 5.4-2 : `GET /videos/{id}` (비디오 상세)
- **성공 → 200 OK**  
  - When: 상세 진입, 존재하는 영상 id  
  - Then: **200**, 본문에 메타(제목, 설명, 길이, `video_url_vimeo`, **VIDEO_TAG 배열**)  
  - 상태축: Auth=pass 또는 stop / Page=`video` init→ready / Request=`video` pending→success / **Data=`video` present**
- **실패(없는 영상) → 404 Not Found**  
  - When: 잘못된 id  
  - 상태축: Request … → error / **Data=`video` error**

> 메모: 실제 시청(재생 시작/완료 등)은 클라이언트에서 비메오 플레이어 이벤트로 잡고, 별도 **progress API**(3-4)를 호출해 **VIDEO_LOG**를 적재.

---

#### 5.4-3 : `GET /videos/{id}/progress` (진행도 조회)
- **성공(기록 있음) → 200 OK**  
  - When: 인증된 사용자가 자신의 진행도 조회  
  - Then: **200**, `{ progress_percent, last_watched_at }`  
  - 상태축: Auth=pass / Page=`video` init→ready / Request=`progress` pending→success / **Data=`progress` present**
- **성공(기록 없음) → 200 OK**  
  - Then: **200**, `{ progress_percent: 0, last_watched_at: null }`  
  - 상태축: Data=`progress` **empty**
- **실패(미인증) → 401 Unauthorized**  
  - When: 토큰 없음/만료  
  - 상태축: Auth=stop / Request … → error / Data=`progress` error
- **실패(없는 영상) → 404 Not Found**

---

#### 5.4-4 : `POST /videos/{id}/progress` (진행도 갱신)
- **성공 → 200 OK(또는 204 No Content)**  
  - When: 클라이언트가 재생 이벤트 동안 진행도(0~100)를 전송  
  - Then: **200**(업데이트 후 스냅샷 반환) **혹은 204**, 서버는 `progress_percent`(클램프 0~100)와 `last_watched_at` 갱신, **VIDEO_LOG upsert**  
  - 상태축: Auth=pass / Page=`video` init→ready / **Form=`progress` pristine→dirty→validating→submitting→success** / Request=`progress` pending→success / **Data=`progress` present**
- **실패(형식/누락) → 400 Bad Request**  
  - 예: `progress_percent`가 숫자 아님, 바디 누락  
  - 상태축: **Form=`progress` … → error.client** / Request … → error / **Data=`progress` empty**
- **실패(도메인 제약) → 422 Unprocessable Entity**  
  - 예: 범위(0~100) 위반, (정책 선택 시) 지나친 감소 등 규칙 위반  
  - 상태축: **Form=`progress` … → error.client** / Request … → error / **Data=`progress` error**
- **실패(미인증) → 401 Unauthorized**  
  - When: 토큰 없음/만료  
- **실패(없는 영상) → 404 Not Found**
</details>

---

### 5.5 Phase 5 — study ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 5-1 | `GET /studies` | `/studies` | 학습 문제 목록 | ***`study_program_enum` 기준 조회, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page studies init→ready / Request studies pending→success / Data studies present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page studies init→ready / Request studies pending→success / Data studies empty → **200**<br>실패(형식/누락): Auth pass 또는 stop / Page studies init→ready / Request studies pending→error / Data studies error → **400**<br>실패(도메인 제약): Auth pass 또는 stop / Page studies init→ready / Request studies pending→error / Data studies error → **422** | [✅🆗] |
| 5-2 | `GET /studies/{id}` | `/studies/{study_id}` | Study 상세 (Task 목록) | ***STUDY 상세 + 해당 Study의 STUDY_TASK 목록 조회, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page study init→ready / Request study pending→success / Data study present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page study init→ready / Request study pending→success / Data study empty → **200** (Task 없음)<br>실패(없는 Study): Auth pass 또는 stop / Page study init→ready / Request study pending→error / Data study error → **404** | [✅🆗] |
| 5-3 | `GET /studies/tasks/{id}` | `/studies/tasks/{task_id}` | 학습 문제 상세 | ***STUDY_TASK 조회, 보기(풀이 전)→ STUDY_TASK_LOG 저장(view)***<br>성공: Auth pass 또는 stop / Page task init→ready / Request task pending→success / Data task present → **200**<br>실패(없는 문항): Auth pass 또는 stop / Page task init→ready / Request task pending→error / Data task error → **404** | [✅🆗] |
| 5-4 | `POST /studies/tasks/{id}/answer` | `/studies/tasks/{task_id}` | 정답 제출/채점 | ***STUDY_TASK_STATUS 업데이트 → STUDY_TASK_LOG 저장(채점 포함)***<br>성공:<br> Auth pass / Page task init→ready / Form answer pristine→dirty→validating→submitting→success /<br> Request answer pending→success / Data answer present → **200**<br>실패(형식/누락):<br> Auth pass / Page task init→ready / Form answer pristine→dirty→validating→error.client / Request answer pending→error / Data answer empty → **400**<br>실패(도메인 제약: 선택지 범위/중복 허용 규칙 등):<br> Auth pass / Page task init→ready / Form answer pristine→dirty→validating→error.client / Request answer pending→error / Data answer error → **422**<br>실패(미인증): Auth stop / Page task init→ready / Request answer pending→error / Data answer error → **401**<br>실패(없는 문항): Auth pass / Page task init→ready / Request answer pending→error / Data answer error → **404** | [✅🆗] |
| 5-5 | `GET /studies/tasks/{id}/status` | `/studies/tasks/{task_id}` | 내 시도/기록 | ***내 최신 STATUS(progress/score/attempts) 조회***<br>성공: Auth pass / Page task init→ready / Request status pending→success / Data status present(또는 empty=기록없음) → **200**<br>실패(미인증): Auth stop / Page task init→ready / Request status pending→error / Data status error → **401**<br>실패(없는 문항): Auth pass / Page task init→ready / Request status pending→error / Data status error → **404** | [✅🆗] |
| 5-6 | `GET /studies/tasks/{id}/explain` | `/studies/tasks/{task_id}/explain` | 해설 보기 | ***STUDY_EXPLAIN 문항별 해설/미디어***<br>성공: Auth pass 또는 stop / Page explain init→ready / Request explain pending→success / Data explain present → **200**<br>실패(없는 문항/해설 없음): Auth pass 또는 stop / Page explain init→ready / Request explain pending→error / Data explain error → **404**<br>실패(도메인 정책: 시도 전 열람 금지 설정 시): Auth pass 또는 stop / Page explain ready / Request explain pending→error / Data explain error → **403** | [✅🆗] |

---

<details>
  <summary>5.5 Phase 5 — study 시나리오 상세 (5.5-1 ~ 5.5-5)</summary>

#### 공통 정책(5.5-1 ~ 5.5-5)
- **에러 바디(고정)**  
  `{ "error": { "http_status": 400|401|403|404|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **검증 기준**  
  - **400** = 형식/누락/파싱 실패(예: `page=abc`, `program=` 빈값)
  - **422** = 도메인 제약 위반(예: `study_program_enum`에 없는 값, `per_page` 상한 초과, 보기 규칙 위반)
- **로그**
  - 문제 조회(4-2): **STUDY_TASK_LOG**에 study_task_action_log 컬럼 study_task_log_action_enum 바탕으로 `view` 업데이트
  - 정답 제출(4-3)
    1. **STUDY_TASK_STATUS**에 업데이트 : 시도횟수(`study_task_status_try`), 최고점(`study_task_status_best`), 완료여부(`study_task_status_completed`)
    2. **STUDY_TASK_LOG**에 업데이트 : 학습행동(`study_task_action_log`), 시도횟수(`study_task_try_no_log`), 점수기록(`study_task_score_log`), 완료여부(`study_task_is_correct_log`), 풀이기록(`study_task_payload_log`), 
  - 상태 조회(4-4): **STUDY_TASK_LOG**에 study_task_action_log 컬럼 study_task_log_action_enum 바탕으로 `status` 업데이트
- **레이트리밋(선택)**  
  - 과도한 채점/새로고침 방지 → **429 + Retry-After**
- **권한/공개 정책**  
  - 목록/상세/해설은 서비스 정책에 따라 공개/비공개를 조절 가능(기본: 공개 열람 가능, 정답 제출·내 기록 조회는 인증 필요)

---

#### 5.5-1 : `GET /studies` (학습 문제 목록)
- **로그인 안해도 접근 가능**
- **성공(데이터 있음) → 200**  
  - When: `/studies` 진입, `program/page/per_page/sort` 유효
  - Then: **200**, 목록 + 페이지 메타, `study_program_enum` 필터 반영
  - 상태축: Auth=pass 또는 stop / Page=`studies` init→ready / Request=`studies` pending→success / Data=`studies` present
- **성공(데이터 없음) → 200**  
  - 빈 배열 + 페이지 메타 / Data=`studies` empty
- **실패(형식/누락) → 400**  
  - 예: `page`/`per_page` 숫자 아님, `program` 파라미터 형식 오류
- **실패(도메인 제약) → 422**  
  - 예: `program`이 enum에 없음, `per_page` 상한 초과, 허용되지 않은 `sort` 필드

---

#### 5.5-2 : `GET /studies/{id}` (Study 상세 + Task 목록)
- **로그인 안해도 접근 가능**
- **성공(데이터 있음) → 200**
  - When: `/studies/{study_id}` 진입, `page/per_page` 유효
  - Then: **200**, Study 정보 + 해당 Study의 Task 목록 + 페이지 메타
  - 응답 예시:
    ```json
    {
      "study_id": 1,
      "study_idx": "test-1",
      "program": "basic_word",
      "title": "한글 자음 연습",
      "subtitle": "\"ㅏ\"로 자음 연습 하기",
      "state": "open",
      "tasks": [
        { "task_id": 1, "kind": "choice", "seq": 1 },
        { "task_id": 2, "kind": "typing", "seq": 2 }
      ],
      "meta": { "total_count": 2, "total_pages": 1, "page": 1, "per_page": 10 }
    }
    ```
  - 상태축: Auth=pass 또는 stop / Page=`study` init→ready / Request=`study` pending→success / Data=`study` present
- **성공(Task 없음) → 200**
  - Study는 존재하지만 Task가 없는 경우 빈 배열 반환
  - Data=`study` present, `tasks` empty
- **실패(없는 Study) → 404**
  - 잘못된 `{id}`
- **실패(형식/누락) → 400**
  - 예: `page`/`per_page` 숫자 아님

---

#### 5.5-3 : `GET /studies/tasks/{id}` (학습 문제 상세)
- 성공 → **200**  
  - Then: **200**, 문제 본문/보기/메타(난이도/분류) → **STUDY_TASK_LOG** `view` 업데이트
  - 상태축: Auth=pass 또는 stop / Page=`task` init→ready / Request=`task` pending→success / Data=`task` present
- 실패(없는 문항) → **404**  
  - 잘못된 `{id}`

---

#### 5.5-4 : `POST /studies/tasks/{id}/answer` (정답 제출/채점)
- 성공 → **200**  
  - When: 인증 사용자,
    1. study_task_typing : 타이핑 시도 → **STUDY_TASK_LOG** `start` 업데이트 → 타이핑 완료 → **STUDY_TASK_LOG** `answer` 업데이트
    2. study_task_choice : 선택지 클릭 → **STUDY_TASK_LOG** `answer` 업데이트
    3. study_task_voice : 녹음 버튼 클릭 → **STUDY_TASK_LOG** `start` 업데이트 → 녹음 버튼 재클릭 → **STUDY_TASK_LOG** `answer` 업데이트
  - Then: **200**, 
    1. study_task_typing : 채점 → **STUDY_TASK_TYPING** `study_task_typing_answer` 대조 → **STUDY_TASK_STATUS** 결과 업데이트 → **STUDY_TASK_LOG** `finish` 업데이트
    2. study_task_choice : 채점 → **STUDY_TASK_CHOICE** `study_task_choice_answer` 대조 → **STUDY_TASK_STATUS** 결과 업데이트 → **STUDY_TASK_LOG** `finish` 업데이트
    3. study_task_voice : 채점 →  **STUDY_TASK_VOICE** `study_task_voice_answer` 대조 → **STUDY_TASK_STATUS** 결과 업데이트 → **STUDY_TASK_LOG** `finish` 업데이트
  - 상태축: Auth=pass / Page=`task` init→ready / Form=`answer` pristine→dirty→validating→submitting→success / Request=`answer` pending→success / Data=`answer` present
- 실패(형식/누락) → **400**  
  - 예: 바디 없음, 선택지 배열 스키마 불일치, 서술형 빈 문자열 금지 등
  - 상태축: Form=`answer` … → error.client / Request=`answer` pending→error / Data=`answer` empty
- 실패(도메인 제약) → **422**  
  - 예: 단일선택 문항에 다중 선택 제출, 범위를 벗어난 보기 인덱스, 이미 종료된 시도에 재제출 금지 정책 등
  - 상태축: Form=`answer` … → error.client / Request=`answer` pending→error / Data=`answer` error
- 실패(미인증) → **401**
  - 토큰 없음/만료
- 실패(없는 문항) → **404**  
  - 잘못된 `{id}`
- 실패(레이트리밋, 선택) → **429**
  - 과도한 제출/채점 요청

---

#### 5.5-5 : `GET /studies/tasks/{id}/status` (내 시도/기록)
- 성공 → **200**  
  - Then: **200**, `{ study_task_status_try_count, study_task_status_is_solved, study_task_status_last_attempt_at }` → **STUDY_TASK_LOG** `status` 업데이트
  - 상태축: Auth=pass / Page=`task` init→ready / Request=`status` pending→success / Data=`status` present(또는 empty)
- 실패(미인증) → **401**
  - 토큰 없음/만료
- 실패(없는 문항) → **404**

---

#### 5.5-6 : `GET /studies/tasks/{id}/explain` (해설 보기)
- 성공 → **200**  
  - Then: **200**,`{ explain_title, explain_text, explain_media_url }` → **STUDY_TASK_LOG** `explain` 업데이트
  - 상태축: Auth=pass 또는 stop / Page=`explain` init→ready / Request=`explain` pending→success / Data=`explain` present
- 실패(해설 없음/없는 문항) → **404**
  - 자료 미제공 또는 잘못된 `{id}`
- 실패(정책상 제한) → **403**
  - 예: “최소 1회 시도 후 열람” 정책을 켠 경우, 시도 전 접근 차단

</details>

---

### 5.6 Phase 6 — lesson ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 6-1 | `GET /lessons` | `/lessons` | 수업 전체 목록 | ***`lesson_idx` 기준 조회, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→success / Data lessons present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→success / Data lessons empty → **200**<br>실패(형식/누락): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→error / Data lessons error → **400**<br>실패(도메인 제약): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→error / Data lessons error → **422** | [✅🆗] |
| 6-2 | `GET /lessons/{id}` | `/lessons/{lesson_id}` | 수업 상세 | ***`video_tag_id` + `study_task_id` 기반 목록 조회, 페이지네이션***<br>성공: Auth pass 또는 stop / Page lesson init→ready / Request lesson pending→success / Data lesson present → **200**<br>실패(없는 수업): Auth pass 또는 stop / Page lesson init→ready / Request lesson pending→error / Data lesson error → **404** | [✅🆗] |
| 6-3 | `GET /lessons/{id}/items` | `/lessons/{lesson_id}/items` | 수업 학습 | ***`lesson_item_seq` 기준 조회, 학습 화면 로드(풀이/진행은 별도 API)***<br>성공: Auth pass 또는 stop / Page lesson_items init→ready / Request lesson_items pending→success / Data lesson_items present → **200**<br>실패(없는 수업/항목): Auth pass 또는 stop / Page lesson_items init→ready / Request lesson_items pending→error / Data lesson_items error → **404**<br>실패(정책상 제한: 수강권 필요): Auth forbid / Page lesson_items ready / Request lesson_items pending→error / Data lesson_items error → **403**<br>실패(형식/누락·도메인): Auth pass 또는 stop / Page lesson_items init→ready / Request lesson_items pending→error / Data lesson_items error → **400**/**422** | [✅🆗] |
| 6-4 | `GET /lessons/{id}/progress` | `/lessons/{lesson_id}` | 수업 진행 조회 | ***LESSON_PROGRESS 최신 값 조회(없으면 0%)***<br>성공: Auth pass / Page lesson init→ready / Request lesson_progress pending→success / Data lesson_progress present(또는 empty=0%) → **200**<br>실패(미인증): Auth stop / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **401**<br>실패(없는 수업): Auth pass / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **404** | [✅🆗] |
| 6-5 | `POST /lessons/{id}/progress` | `/lessons/{lesson_id}` | 수업 진행 갱신 | ***LESSON_PROGRESS 컬럼 업데이트(0~100 고정, 멱등)***<br>성공:<br> Auth pass / Page lesson init→ready / Form lesson_progress pristine→dirty→validating→submitting→success /<br> Request lesson_progress pending→success / Data lesson_progress present → **200**(또는 **204**)<br>실패(형식/누락):<br> Auth pass / Page lesson init→ready / Form lesson_progress pristine→dirty→validating→error.client /<br> Request lesson_progress pending→error / Data lesson_progress empty → **400**<br>실패(도메인 제약: 범위/증감 규칙):<br> Auth pass / Page lesson init→ready / Form lesson_progress pristine→dirty→validating→error.client /<br> Request lesson_progress pending→error / Data lesson_progress error → **422**<br>실패(미인증): Auth stop / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **401**<br>실패(없는 수업): Auth pass / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **404**<br>실패(정책상 제한: 수강권 필요): Auth forbid / Page lesson ready / Request lesson_progress pending→error / Data lesson_progress error → **403** | [✅🆗] |

---

<details>
  <summary>5.6 Phase 6 — lesson 시나리오 상세 (5.6-1 ~ 5.6-5)</summary>

#### 공통 정책(5.6-1 ~ 5.6-5)
- **에러 바디(고정)**  
  `{ "error": { "http_status": 400|401|403|404|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **검증 기준**  
  - **400** = 형식/누락/파싱 실패(예: `page=abc`, `per_page=foo`)  
  - **422** = 도메인 제약 위반(예: `per_page` 상한, 허용되지 않은 `sort`, 진행도 0~100 범위 위반 등)
- **권한/수강권**  
  - 수업 목록/상세/아이템은 서비스 정책에 따라 공개 가능하되, **수강권 필수 정책을 켜면 403** 적용  
  - 진행도 조회/갱신은 **인증 필수**
- **진행도 규칙**  
  - 멱등: 동일 값 재전송은 상태 변화 없이 성공  
  - 기록 없음은 **200 + empty(0%)**로 응답(오류 아님)
- **로그**  
  - 진행도 갱신(5-5): LESSON_PROGRESS 업데이트 시 서버시각으로 갱신, 필요 시 LESSON_PROGRESS_LOG(선택)

---

#### 5.6-1 : `GET /lessons` (수업 전체 목록)
- **로그인 안해도 접근 가능**
- 성공(데이터 있음) → **200**  
  - When: `/lessons` 진입, `page/per_page/sort` 유효  
  - Then: **200**, 목록 + 페이지 메타(`lesson_idx` 기준 정렬)
  - 상태축: Auth=pass 또는 stop / Page=`lessons` init→ready / Request=`lessons` pending→success / Data=`lessons` present
- 성공(데이터 없음) → **200**  
  - 빈 배열 + 페이지 메타 / Data=`lessons` empty
- 실패(형식/누락) → **400**  
  - 예: 숫자 아님, 음수/0 페이지
- 실패(도메인 제약) → **422**  
  - 예: `per_page` 상한 초과, 허용 외 정렬 키

---

#### 5.6-2 : `GET /lessons/{id}` (수업 상세)
- 성공 → **200**  
  - Then: **200**, 수업 메타 + 연계 목록(영상 태그/학습 과제 id 집합) 페이지네이션
  - 상태축: Auth=pass 또는 stop / Page=`lesson` init→ready / Request=`lesson` pending→success / Data=`lesson` present
- 실패(없는 수업) → **404**

---

#### 5.6-3 : `GET /lessons/{id}/items` (수업 학습)
- 성공 → **200**  
  - Then: **200**, `lesson_item_seq` 기준 아이템 목록(문항/비디오/자료 등), 학습 화면 로드
  - 상태축: Auth=pass 또는 stop / Page=`lesson_items` init→ready / Request=`lesson_items` pending→success / Data=`lesson_items` present
- 실패(없는 수업/항목) → **404**
- 실패(정책상 제한: 수강권 필요) → **403**
- 실패(형식/누락 → 400 / 도메인 제약 → 422)**

---

#### 5.6-4 : `GET /lessons/{id}/progress` (수업 진행 조회)
- 성공 → **200**  
  - Then: **200**, `{ progress_percent, last_updated_at }` (없으면 `{0, null}`)
  - 상태축: Auth=pass / Page=`lesson` init→ready / Request=`lesson_progress` pending→success / Data=`lesson_progress` present(또는 empty)
- 실패(미인증) → **401**
- 실패(없는 수업) → **404**

---

#### 5.6-5 : `POST /lessons/{id}/progress` (수업 진행 갱신)
- 성공 → **200**(또는 **204**)  
  - When: 학습 중간/완료 시 진행도를 제출(0~100), 멱등 업데이트
  - Then: **200**(업데이트 후 스냅샷) 또는 **204**, 서버는 LESSON_PROGRESS 갱신
  - 상태축: Auth=pass / Page=`lesson` init→ready / Form=`lesson_progress` pristine→dirty→validating→submitting→success / Request=`lesson_progress` pending→success / Data=`lesson_progress` present
- 실패(형식/누락) → **400**
  - 예: 바디 누락, 숫자 아님
- 실패(도메인 제약) → **422**
  - 예: 0~100 범위 위반, (정책 선택 시) 역진행 금지
- 실패(미인증) → **401**
- 실패(없는 수업) → **404**
- 실패(정책상 제한: 수강권 필요) → **403**

</details>

---

### 5.7 Phase 7 — admin ✅🆗
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 7-1 | `GET /admin/users` | `/admin/users?page=&size=&q=&sort=&order=` | 사용자 조회 | ***검색/정렬/페이지네이션, RBAC(admin)***<br>성공(데이터 있음/없음): → **200**<br>실패(미인증): **401** / RBAC: **403** / 형식: **400** / 도메인: **422** | [✅🆗] |
| 7-2 | `GET /admin/users/{id}/admin-logs` | `/admin/users/{user_id}?tab=admin-logs&page=&size=` | 관리자 사용자 변경 로그 조회 | ***페이지네이션, RBAC***<br>성공: → **200**<br>실패: **401/403/404/400/422** | [✅🆗] |
| 7-3 | `GET /admin/users/{id}/user-logs` | `/admin/users/{user_id}?tab=user-logs&page=&size=` | 사용자 자체 변경 로그 조회 | ***페이지네이션, RBAC***<br>성공: → **200**<br>실패: **401/403/404/400/422** | [✅🆗] |
| 7-4 | `POST /admin/users` | `/admin/users/new` | 사용자 단건 생성 | ***ADMIN_USERS_LOG 저장, RBAC***<br>성공: → **201**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-5 | `POST /admin/users/bulk` | `/admin/users/bulk` | 사용자 다중 생성 | ***부분 성공, ADMIN_USERS_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-6 | `PATCH /admin/users/{id}` | `/admin/users/{user_id}/edit` | 사용자 단건 수정 | ***ADMIN_USERS_LOG 저장, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-7 | `PATCH /admin/users/bulk` | `/admin/users/bulk` | 사용자 다중 수정 | ***부분 성공, ADMIN_USERS_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |

| 7-8 | `GET /admin/videos` | `/admin/videos?page=&size=&q=&sort=&order=` | 비디오 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-9 | `GET /admin/videos/{id}` | `/admin/videos/{video_id}` | 비디오 상세 조회 | ***RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-10 | `GET /admin/videos/vimeo/preview` | `/admin/videos/new` | Vimeo 메타데이터 미리보기 | ***Vimeo API 연동, RBAC***<br>query: `url`<br>성공: **200**<br>실패: **401/403/400** | [✅🆗] |
| 7-11 | `POST /admin/videos/vimeo/upload-ticket` | `/admin/videos/new` | Vimeo 업로드 티켓 생성 | ***Vimeo tus upload, RBAC***<br>성공: **200**<br>실패: **401/403/400** | [✅🆗] |
| 7-12 | `POST /admin/videos` | `/admin/videos/new` | 비디오 단건 생성 | ***ADMIN_VIDEO_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-13 | `POST /admin/videos/bulk` | `/admin/videos/bulk` | 비디오 다중 생성 | ***부분 성공, ADMIN_VIDEO_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-14 | `PATCH /admin/videos/{id}` | `/admin/videos/{video_id}/edit` | 비디오 단건 수정 | ***ADMIN_VIDEO_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-15 | `PATCH /admin/videos/bulk` | `/admin/videos/bulk` | 비디오 다중 수정 | ***부분 성공, ADMIN_VIDEO_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-16 | `PATCH /admin/videos/{id}/tags` | `/admin/videos/{video_id}/tags` | 비디오 태그 단건 수정 | ***태그 검증, ADMIN_VIDEO_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-17 | `PATCH /admin/videos/bulk/tags` | `/admin/videos/bulk/tags` | 비디오 태그 다중 수정 | ***부분 성공, ADMIN_VIDEO_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-18 | `GET /admin/videos/stats/summary` | `/admin/videos/stats?from=&to=` | 비디오 통계 요약 | ***총 조회수/완료수/활성비디오수, 기간 검증(max 366일), RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-19 | `GET /admin/videos/stats/top` | `/admin/videos/stats?from=&to=&limit=&sort_by=` | TOP 비디오 조회 | ***조회수/완료수 정렬, limit 1-50, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-20 | `GET /admin/videos/stats/daily` | `/admin/videos/stats?from=&to=` | 비디오 일별 통계 | ***전체 집계, 제로필, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-21 | `GET /admin/videos/{id}/stats/daily` | `/admin/videos/{video_id}/stats?from=&to=` | 비디오별 일별 통계 | ***VIDEO_STAT_DAILY 조회, 제로필, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422** | [✅🆗] |

| 7-22 | `GET /admin/studies` | `/admin/studies?page=&size=&q=&sort=&order=` | 학습 문제 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-23 | `GET /admin/studies/{id}` | `/admin/studies/{study_id}` | 학습 문제 상세 조회 | ***tasks 포함, RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-24 | `POST /admin/studies` | `/admin/studies/new` | 학습 문제 단건 생성 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-25 | `POST /admin/studies/bulk` | `/admin/studies/bulk` | 학습 문제 다중 생성 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-26 | `PATCH /admin/studies/{id}` | `/admin/studies/{study_id}/edit` | 학습 문제 단건 수정 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-27 | `PATCH /admin/studies/bulk` | `/admin/studies/bulk` | 학습 문제 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-28 | `GET /admin/studies/tasks` | `/admin/studies/tasks?study_id=&page=&size=` | 학습 Task 조회 | ***study_id 필수, 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422/404** | [✅🆗] |
| 7-29 | `GET /admin/studies/tasks/{id}` | `/admin/studies/tasks/{task_id}` | 학습 Task 상세 조회 | ***RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-30 | `POST /admin/studies/tasks` | `/admin/studies/tasks/new` | 학습 Task 단건 생성 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/404/409** | [✅🆗] |
| 7-31 | `POST /admin/studies/tasks/bulk` | `/admin/studies/tasks/bulk` | 학습 Task 다중 생성 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/404/409** | [✅🆗] |
| 7-32 | `PATCH /admin/studies/tasks/{id}` | `/admin/studies/tasks/{task_id}/edit` | 학습 Task 단건 수정 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-33 | `PATCH /admin/studies/tasks/bulk` | `/admin/studies/tasks/bulk` | 학습 Task 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-34 | `GET /admin/studies/tasks/explain` | `/admin/studies/tasks/explain?task_id=&page=&size=` | 학습 해설 조회 | ***task_id 검증, 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422/404** | [✅🆗] |
| 7-35 | `POST /admin/studies/tasks/{id}/explain` | `/admin/studies/tasks/{task_id}/explain/new` | 학습 해설 단건 생성 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/404/409** | [✅🆗] |
| 7-36 | `POST /admin/studies/tasks/bulk/explain` | `/admin/studies/tasks/bulk/explain` | 학습 해설 다중 생성 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/404/409** | [✅🆗] |
| 7-37 | `PATCH /admin/studies/tasks/{id}/explain` | `/admin/studies/tasks/{task_id}/explain/edit` | 학습 해설 단건 수정 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-38 | `PATCH /admin/studies/tasks/bulk/explain` | `/admin/studies/tasks/bulk/explain` | 학습 해설 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409/404** | [✅🆗] |
| 7-39 | `GET /admin/studies/tasks/status` | `/admin/studies/tasks/status?task_id=&page=&size=` | 학습 상태 조회 | ***task_id 검증, 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422/404** | [✅🆗] |
| 7-40 | `PATCH /admin/studies/tasks/{id}/status` | `/admin/studies/tasks/{task_id}/status/edit` | 학습 상태 단건 수정 | ***ADMIN_STUDY_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-41 | `PATCH /admin/studies/tasks/bulk/status` | `/admin/studies/tasks/bulk/status` | 학습 상태 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409/404** | [✅🆗] |
| 7-42 | `GET /admin/studies/stats/summary` | `/admin/studies/stats?from=&to=` | 학습 통계 요약 | ***총 학습수/Task수/시도수/해결수/해결률, Program별/State별 분포, 기간 검증(max 366일), RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-43 | `GET /admin/studies/stats/top` | `/admin/studies/stats?from=&to=&limit=&sort_by=` | TOP 학습 조회 | ***시도수/해결수/해결률 정렬, limit 1-50, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-44 | `GET /admin/studies/stats/daily` | `/admin/studies/stats?from=&to=` | 학습 일별 통계 | ***일별 시도수/해결수/활성사용자, 제로필, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |

| 7-45 | `GET /admin/lessons` | `/admin/lessons?page=&size=&q=&sort=&order=` | 수업 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-46 | `GET /admin/lessons/{id}` | `/admin/lessons/{lesson_id}` | 수업 상세 조회 | ***lesson_id로 단건 조회, RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-47 | `POST /admin/lessons` | `/admin/lessons/new` | 수업 단건 생성 | ***ADMIN_LESSON_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-48 | `POST /admin/lessons/bulk` | `/admin/lessons/bulk-create` | 수업 다중 생성 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-49 | `PATCH /admin/lessons/{id}` | `/admin/lessons/{lesson_id}` | 수업 단건 수정 | ***ADMIN_LESSON_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-50 | `PATCH /admin/lessons/bulk` | `/admin/lessons` | 수업 다중 수정 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-51 | `GET /admin/lessons/items` | `/admin/lessons/items?page=&size=&lesson_id=` | 수업 아이템 조회 | ***lesson_id 필터, 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-52 | `GET /admin/lessons/items/{id}` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 상세 조회 | ***lesson_id로 아이템 목록+상세 조회 (video/task 정보 포함), RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-53 | `POST /admin/lessons/{id}/items` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 생성 | ***insert_mode(error/shift), ADMIN_LESSON_LOG, RBAC***<br>성공: **201**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-54 | `POST /admin/lessons/bulk/items` | `/admin/lessons/bulk-create` | 수업 아이템 다중 생성 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422/409** | [✅🆗] |
| 7-55 | `PATCH /admin/lessons/{id}/items/{seq}` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 단건 수정 | ***seq로 아이템 지정, 순서 규칙 검증, ADMIN_LESSON_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-56 | `PATCH /admin/lessons/bulk/items` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 다중 수정 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409/404** | [✅🆗] |
| 7-57 | `DELETE /admin/lessons/{id}/items/{seq}` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 단건 삭제 | ***seq로 아이템 지정, ADMIN_LESSON_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-58 | `DELETE /admin/lessons/bulk/items` | `/admin/lessons/{lesson_id}` (Items 탭) | 수업 아이템 다중 삭제 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/404** | [✅🆗] |
| 7-59 | `GET /admin/lessons/progress` | `/admin/lessons/progress?page=&size=&lesson_id=&user_id=` | 수업 진행 조회 | ***lesson_id/user_id 필터, 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-60 | `GET /admin/lessons/progress/{id}` | `/admin/lessons/{lesson_id}` (Progress 탭) | 수업 진행 상세 조회 | ***lesson_id로 사용자별 진행현황 목록 조회 (current_item 포함), RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅🆗] |
| 7-61 | `PATCH /admin/lessons/{id}/progress` | `/admin/lessons/{lesson_id}` (Progress 탭) | 수업 진행 단건 수정 | ***user_id 지정, percent/last_item_seq 수정, ADMIN_LESSON_LOG, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422/409** | [✅🆗] |
| 7-62 | `PATCH /admin/lessons/bulk/progress` | `/admin/lessons/{lesson_id}` (Progress 탭) | 수업 진행 다중 수정 | ***부분 성공, 다중 사용자 진행 수정, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207**<br>실패: **401/403/400/422/409/404** | [✅🆗] |

| 7-63 | `GET /admin/users/stats/summary` | `/admin/users/stats?from=&to=` | 사용자 요약 통계 | ***총 사용자수/신규/활성/비활성, 역할별 집계, 기간 검증(max 366일), RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-64 | `GET /admin/users/stats/signups` | `/admin/users/stats?from=&to=` | 일별 가입 통계 | ***일별 가입수, 역할별 집계, 제로필, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-65 | `GET /admin/logins/stats/summary` | `/admin/logins/stats?from=&to=` | 로그인 요약 통계 | ***총 로그인/성공/실패/고유사용자/활성세션, 기간 검증(max 366일), RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-66 | `GET /admin/logins/stats/daily` | `/admin/logins/stats?from=&to=` | 일별 로그인 통계 | ***일별 성공/실패/고유사용자, 제로필, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |
| 7-67 | `GET /admin/logins/stats/devices` | `/admin/logins/stats?from=&to=` | 디바이스별 로그인 통계 | ***디바이스별 성공횟수/비율, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅🆗] |

| 7-71 | `POST /admin/email/test` | (관리자 전용) | 테스트 이메일 발송 | ***이메일 설정 검증용, RBAC(HYMN/Admin)***<br>성공: **200**<br>실패: **401/403/500** | [✅] |

| 7-68 | `POST /admin/upgrade` | `/admin/upgrade` | 관리자 초대 | ***초대 코드 생성 + 이메일 발송, RBAC(HYMN→Admin/Manager, Admin→Manager), Redis TTL 10분***<br>성공: **200**<br>실패: **401/403/400/422/409**(이미 가입된 이메일) | [✅🆗] |
| 7-69 | `GET /admin/upgrade/verify` | `/admin/upgrade/join?code=xxx` | 초대 코드 검증 | ***Public, 코드 유효성 검증, 이메일/역할 정보 반환***<br>성공: **200**<br>실패: **400/401**(만료/무효 코드) | [✅🆗] |
| 7-70 | `POST /admin/upgrade/accept` | `/admin/upgrade/join?code=xxx` | 관리자 계정 생성 | ***Public(코드 필수), 관리자 계정 생성(OAuth 불가), 코드 삭제***<br>성공: **201**<br>실패: **400/401/409/422** | [✅🆗] |

---

<details>
  <summary>5.7 Phase 7 — admin 관리자 초대 시나리오 (7-68 ~ 7-70)</summary>

#### 관리자 초대 시스템 개요

> 관리자 계정은 **오직 초대를 통해서만** 생성 가능. 일반 회원가입 후 승격 불가.

**보안 정책**
- 관리자 계정: OAuth 로그인 비허용 (이메일/비밀번호만)
- 초대 코드: Redis 저장, TTL 10분, 일회용
- 기존 이메일로 초대 시: 거부 (이미 가입된 이메일)
- 권한별 초대 가능 범위:
  | 요청자 | 초대 가능 권한 |
  |--------|---------------|
  | HYMN | Admin, Manager |
  | Admin | Manager |
  | Manager | 불가 (403) |

---

#### 7-68: `POST /admin/upgrade` (관리자 초대)

**요청**
```json
{
  "email": "new-admin@example.com",
  "role": "admin"  // admin | manager
}
```

**응답 (성공 200)**
```json
{
  "message": "Invitation sent successfully",
  "expires_at": "2026-02-04T12:10:00Z"
}
```

**처리 흐름**
1. 요청자 권한 검증 (HYMN/Admin만)
2. 초대 가능 role 검증 (HYMN→Admin/Manager, Admin→Manager)
3. 이메일 중복 체크 (기존 가입자면 409)
4. 초대 코드 생성: `ak_upgrade_{uuid}`
5. Redis 저장: `ak:upgrade:{code}` → `{email, role, invited_by, created_at}`, TTL 10분
6. 이메일 발송 (Resend)
7. 초대 로그 기록

**실패 케이스**
- **401**: 미인증
- **403**: 권한 부족 (Manager가 초대 시도, Admin이 Admin 초대 시도)
- **409**: 이미 가입된 이메일
- **422**: 유효하지 않은 role

---

#### 7-69: `GET /admin/upgrade/verify` (초대 코드 검증)

**요청**: `GET /admin/upgrade/verify?code=ak_upgrade_xxx`

**응답 (성공 200)**
```json
{
  "email": "new-admin@example.com",
  "role": "admin",
  "invited_by": "hymn@amazingkorean.net",
  "expires_at": "2026-02-04T12:10:00Z"
}
```

**실패 케이스**
- **400**: 코드 파라미터 누락
- **401**: 만료/무효 코드

---

#### 7-70: `POST /admin/upgrade/accept` (관리자 계정 생성)

**요청**
```json
{
  "code": "ak_upgrade_xxx",
  "password": "SecureP@ss123",
  "name": "홍길동",
  "nickname": "admin_hong",
  "country": "KR",
  "birthday": "1990-01-01",
  "gender": "male",
  "language": "ko"
}
```

**응답 (성공 201)**
```json
{
  "user_id": 123,
  "email": "new-admin@example.com",
  "user_auth": "admin",
  "message": "Admin account created successfully"
}
```

**처리 흐름**
1. 코드 검증 (Redis 조회)
2. 비밀번호 해싱 (Argon2id)
3. 사용자 생성 (user_auth = 초대 시 지정된 role)
4. 초대 코드 삭제 (일회용)
5. 초대 수락 로그 기록
6. (선택) 자동 로그인 토큰 발급

**실패 케이스**
- **400**: 필수 필드 누락, 형식 오류
- **401**: 만료/무효 코드
- **409**: 코드 이미 사용됨
- **422**: 비밀번호 정책 위반, 닉네임 중복

</details>

---

<details>
  <summary>5.7 Phase 7 — admin 공통 정책 & 시나리오 템플릿</summary>

#### 공통 보안/권한
- 미인증: Auth=stop → **401**
- 권한 부족(RBAC): Auth=forbid → **403**
- 리소스 은닉 전략(선택): 민감 리소스는 **404**로 은닉 가능

#### 에러 스키마(고정)
`{ "error": { "http_status": 400|401|403|404|409|422|429|500, "code": "...", "message": "...", "details": {}, "trace_id": "..." } }`

#### 검증 기준
- **400**: 형식/누락/파싱 실패(예: page=abc, size<1, 잘못된 정렬문법)
- **422**: 도메인 제약 위반(허용되지 않은 sort 필드, size 상한 초과, 비즈 규칙 위반)
- **409**: 고유제약/상태충돌(중복 이메일/태그, 삭제된 리소스 수정 금지 등)
- **429**: 대량/연속 작업 차단(선택, Retry-After 포함)

#### 로깅(필수)
- 모든 Admin 엔드포인트: 성공/실패 모두 `admin_*_log` 기록(요청 요약, actor user_id, 대상/개수, 결과코드, trace_id). 민감값은 마스킹.

---

#### 목록/조회 공통 시나리오(예: GET /admin/videos)
- 성공(데이터 있음/없음) → **200**  
  Auth pass / Page init→ready / Request pending→success / Data present|empty
- 실패(미인증/권한) → **401**/**403**
- 실패(형식/도메인) → **400**/**422**

---

#### 단건 생성 템플릿(예: POST /admin/videos)
- 성공 → **201**  
  Page init→ready / Form pristine→dirty→validating→submitting→success / Request pending→success / Data present  
  헤더: `Location: /admin/videos/{id}`
- 실패(형식/도메인/중복/권한) → **400**/**422**/**409**/**401**/**403**

---

#### 다중 생성(벌크) 템플릿
- 성공(전량) → **201**
- 성공(부분) → **207**
- 실패 항목은 배열로 에러 사유 제공(예: 400/422/409)

---

#### 단건 수정 템플릿(예: PATCH /admin/lessons/{id})
- 성공 → **200** 또는 **204**
- 실패 → **401**/**403**/**404**/**400**/**422**/**409**

---

#### 벌크 수정 템플릿
- 성공(전량) → **200** 또는 **204**
- 성공(부분) → **207**
- 실패 항목별 에러 사유 포함

---

#### 통계 조회(예: GET /admin/videos/{id}/stats)
- 성공 → **200** (빈 구간도 **200**)
- 실패 → **401**/**403**/**404**/**400**/**422**  
  (기간(from≤to)·그라뉼러리티 검증 포함)

</details>

---

### 5.8 Phase 8 — course ✅
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 8-1 | `GET /courses` | `/courses` | 코스 목록 조회 | ***페이지네이션, 접근 권한 체크***<br>응답에 `course_subtitle` 필드 포함<br>DTO: `CourseListQuery`(IntoParams), `CourseListItem`(ToSchema)<br>성공: **200** | [✅] |
| 8-2 | `POST /courses` | `/admin/courses/new` | 코스 생성 | ***ADMIN_COURSE_LOG, RBAC***<br>DTO: `CreateCourseReq`(ToSchema)<br>성공: **201**<br>실패: **401/403/400/422** | [✅] |
| 8-3 | `GET /courses/{id}` | `/courses/{id}` | 코스 상세 조회 | ***코스 정보 + 레슨 목록, `?lang=` 쿼리 파라미터 지원***<br>성공: **200**<br>실패: **404** | [✅] |

---

### 비고
- 모든 Phase는 "**백엔드 엔드포인트 구현 → 프론트 1화면 연동 → 스모크(성공+대표 에러)**" 순으로 완료 표시.

---

### 5.9 Phase 9 — translation (i18n)
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 9-1 | `GET /admin/translations` | `/admin/translations?page=&size=&content_type=&content_types=&content_id=&lang=&status=` | 번역 목록 조회 | ***필터(content_type/content_types, content_id, lang, status) + 페이지네이션, RBAC***<br>성공: **200**<br>실패: **401/403/400/422** | [✅] |
| 9-2 | `POST /admin/translations` | `/admin/translations/new` | 번역 단건 생성 (UPSERT) | ***content_type+content_id+field_name+lang 기준 UPSERT, 텍스트 변경 시에만 status 리셋, RBAC***<br>성공: **201**<br>실패: **401/403/400/422** | [✅] |
| 9-3 | `POST /admin/translations/bulk` | `/admin/translations/bulk` | 번역 벌크 생성 | ***부분 성공, RBAC***<br>성공: **201** / 부분: **207**<br>실패: **401/403/400/422** | [✅] |
| 9-4 | `GET /admin/translations/{id}` | `/admin/translations/{translation_id}` | 번역 상세 조회 | ***RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅] |
| 9-5 | `PATCH /admin/translations/{id}` | `/admin/translations/{translation_id}/edit` | 번역 수정 (텍스트/상태) | ***translated_text, status 부분 수정, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422** | [✅] |
| 9-6 | `PATCH /admin/translations/{id}/status` | `/admin/translations/{translation_id}` | 번역 상태만 변경 | ***draft → reviewed → approved 상태 전이, RBAC***<br>성공: **200**<br>실패: **401/403/404/400/422** | [✅] |
| 9-7 | `DELETE /admin/translations/{id}` | `/admin/translations/{translation_id}` | 번역 삭제 | ***RBAC***<br>성공: **200**<br>실패: **401/403/404** | [✅] |
| 9-8 | `POST /admin/translations/auto` | `/admin/translations` | 자동 번역 (GCP) | ***Google Cloud Translation v2 Basic 연동, 원본 텍스트를 대상 언어로 자동 번역 후 draft 상태로 UPSERT, TRANSLATE_PROVIDER=none이면 503, RBAC***<br>성공: **200**<br>실패: **401/403/400/422/503** | [✅] |
| 9-9 | `GET /admin/translations/content-records` | - | 콘텐츠 목록 조회 (드롭다운용) | ***content_type별 레코드 목록 반환, RBAC***<br>성공: **200**<br>실패: **401/403/400** | [✅] |
| 9-10 | `GET /admin/translations/source-fields` | - | 원본 텍스트 조회 | ***content_type+content_id로 한국어 원본 필드 조회, RBAC***<br>성공: **200**<br>실패: **401/403/400** | [✅] |
| 9-11 | `POST /admin/translations/auto-bulk` | `/admin/translations/new` | 벌크 자동 번역 | ***복수 필드 × 복수 언어 일괄 자동 번역, 숫자 값 스킵, RBAC***<br>성공: **200**<br>실패: **401/403/400/422/503** | [✅] |
| 9-12 | `GET /admin/translations/search` | - | 번역 검색 (재사용) | ***lang으로 최근 approved/reviewed 번역 조회, RBAC***<br>성공: **200**<br>실패: **401/403** | [✅] |

---

<details>
  <summary>5.9 Phase 9 — translation (i18n) 상세</summary>

#### 다국어 콘텐츠 번역 시스템 개요

> 모든 학습 콘텐츠의 번역을 `content_translations` 테이블에서 통합 관리한다. 관리자가 번역을 생성/검수/승인하며, 승인된(approved) 번역만 최종 사용자에게 제공된다.

**핵심 정책**
- **Fallback 순서**: 사용자 언어(`?lang=`) → `en` → `ko` (한국어 원본)
- **공개 조건**: `status = 'approved'` 인 번역만 콘텐츠 API에서 제공
- **기존 콘텐츠 API 확장**: 레슨, 코스, 학습, 비디오 등 기존 API에 `?lang=` 쿼리 파라미터 추가
- **번역 API**: Google Cloud Translation v2 Basic 연동 완료 (AI 자동 초안 → 관리자 검수 → 승인)

**지원 언어 (21개, 아랍어 RTL 별도)**

| 그룹 | 언어 코드 |
|------|-----------|
| 핵심 5개 (Phase 2) | `en`, `ja`, `zh-CN`, `zh-TW`, `vi` |
| 동남아시아 | `id`, `th`, `my`, `km` |
| 중앙/북아시아 | `mn`, `ru`, `uz`, `kk`, `tg` |
| 남아시아 | `ne`, `si`, `hi` |
| 유럽/기타 | `es`, `pt`, `fr`, `de` |

**번역 상태 전이**

```
draft → reviewed → approved
  ↑        ↓
  └────────┘  (검수 반려 시 draft로 되돌림)
```

---

#### 9-1 : `GET /admin/translations` (번역 목록 조회)

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `page` | i64 | N | 페이지 번호 (기본 1) |
| `size` | i64 | N | 페이지 크기 (기본 20, max 100) |
| `content_type` | string | N | 콘텐츠 유형 필터 단일 (course, lesson, video, video_tag, study, ...) |
| `content_types` | string | N | 콘텐츠 유형 필터 복수 (쉼표 구분, content_type보다 우선. e.g. `study,study_task_choice,study_task_typing`) |
| `content_id` | i64 | N | 콘텐츠 ID 필터 |
| `lang` | string | N | 언어 코드 필터 (en, ja, zh-CN, ...) |
| `status` | string | N | 상태 필터 (draft, reviewed, approved) |

**응답 (성공 200)**
```json
{
  "data": [
    {
      "translation_id": 1,
      "content_type": "lesson",
      "content_id": 42,
      "field_name": "title",
      "lang": "en",
      "translated_text": "Introduction to Korean Alphabet",
      "status": "approved",
      "created_at": "2026-02-10T12:00:00Z",
      "updated_at": "2026-02-10T14:30:00Z"
    }
  ],
  "total": 150,
  "page": 1,
  "size": 20
}
```

---

#### 9-2 : `POST /admin/translations` (번역 단건 생성 — UPSERT)

**요청 (TranslationCreateReq)**
```json
{
  "content_type": "lesson",
  "content_id": 42,
  "field_name": "title",
  "lang": "en",
  "translated_text": "Introduction to Korean Alphabet"
}
```

**응답 (성공 201)**
```json
{
  "translation_id": 1,
  "content_type": "lesson",
  "content_id": 42,
  "field_name": "title",
  "lang": "en",
  "translated_text": "Introduction to Korean Alphabet",
  "status": "draft",
  "created_at": "2026-02-10T12:00:00Z",
  "updated_at": "2026-02-10T12:00:00Z"
}
```

> **UPSERT 동작**: `(content_type, content_id, field_name, lang)` 조합이 이미 존재하면 `translated_text`와 `updated_at`을 갱신한다. `status`는 `translated_text`가 실제로 변경된 경우에만 `draft`로 리셋하며, 동일한 텍스트를 다시 제출하면 기존 `status`를 유지한다.

---

#### 9-3 : `POST /admin/translations/bulk` (번역 벌크 생성)

**요청**
```json
{
  "translations": [
    { "content_type": "lesson", "content_id": 42, "field_name": "title", "lang": "en", "translated_text": "Introduction to Korean Alphabet" },
    { "content_type": "lesson", "content_id": 42, "field_name": "description", "lang": "en", "translated_text": "Learn Hangul basics" },
    { "content_type": "lesson", "content_id": 42, "field_name": "title", "lang": "ja", "translated_text": "韓国語アルファベット入門" }
  ]
}
```

**응답 (부분 성공 207 / 전체 성공 201)**
```json
{
  "results": [
    { "index": 0, "status": "created", "translation_id": 1 },
    { "index": 1, "status": "created", "translation_id": 2 },
    { "index": 2, "status": "error", "error": "Invalid content_id" }
  ],
  "total": 3,
  "success": 2,
  "failed": 1
}
```

---

#### 9-5 : `PATCH /admin/translations/{id}` (번역 수정)

**요청**
```json
{
  "translated_text": "Introduction to the Korean Alphabet (Hangul)",
  "status": "reviewed"
}
```

**응답 (성공 200)**: TranslationRes 전체 반환

---

#### 9-6 : `PATCH /admin/translations/{id}/status` (번역 상태만 변경)

**요청**
```json
{
  "status": "approved"
}
```

**응답 (성공 200)**: TranslationRes 전체 반환

> **상태 전이 규칙**: `draft → reviewed → approved` 순서만 허용. 검수 반려 시 `reviewed → draft` 또는 `approved → draft`로 되돌림 가능.

---

#### 9-8 : `POST /admin/translations/auto` (자동 번역)

> Google Cloud Translation v2 Basic를 사용하여 원본 텍스트를 지정 언어로 자동 번역한다. 번역 결과는 `draft` 상태로 `content_translations`에 UPSERT된다.

**요청 Body (JSON)**

| 필드 | 타입 | 필수 | 설명 |
|------|------|------|------|
| `content_type` | string | ✅ | `course`, `lesson`, `video`, `video_tag`, `study` |
| `content_id` | integer | ✅ | 콘텐츠 ID |
| `field_name` | string | ✅ | 번역 대상 필드명 (예: `title`, `description`) |
| `source_text` | string | ✅ | 원본 텍스트 (한국어) |
| `target_langs` | string[] | ✅ | 대상 언어 코드 배열 (최대 20개, 예: `["en", "ja", "zh-CN"]`) |

```json
{
  "content_type": "video",
  "content_id": 1,
  "field_name": "title",
  "source_text": "한국어 초급 과정",
  "target_langs": ["en", "ja", "zh-CN", "zh-TW", "vi"]
}
```

**응답 (성공 200)**

```json
{
  "total": 5,
  "success_count": 5,
  "results": [
    {
      "lang": "en",
      "success": true,
      "translation_id": 42,
      "translated_text": "Korean Beginner Course",
      "error": null
    }
  ]
}
```

> **주의사항**:
> - `TRANSLATE_PROVIDER=none`이면 `503 Service Unavailable` (Translation provider not configured) 반환
> - 개별 언어 번역 실패 시 해당 항목만 `success: false` + `error` 메시지, 나머지는 정상 처리
> - 번역 결과는 `draft` 상태로 UPSERT → 관리자가 검수(reviewed) → 승인(approved) 후 사용자에게 제공
> - 환경변수: `TRANSLATE_PROVIDER=google`, `GOOGLE_TRANSLATE_API_KEY`, `GOOGLE_TRANSLATE_PROJECT_ID` 필요

---

#### 9-9 : `GET /admin/translations/content-records` (콘텐츠 목록 조회)

> content_type별로 번역 가능한 레코드 목록을 반환한다. 관리자가 번역 대상 콘텐츠를 드롭다운에서 선택할 때 사용.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `content_type` | string | ✅ | 콘텐츠 유형 (video, lesson, study, study_task_choice, study_task_typing, study_task_voice, study_task_explain) |

**응답 (성공 200)**
```json
{
  "items": [
    { "id": 1, "label": "VID-001", "detail": "발음 기초" },
    { "id": 2, "label": "VID-002", "detail": "문법 기초" }
  ]
}
```

---

#### 9-10 : `GET /admin/translations/source-fields` (원본 텍스트 조회)

> content_type + content_id로 해당 레코드의 번역 가능 필드와 한국어 원본 텍스트를 반환한다. Video 선택 시 연결된 video_tag 필드도 함께 반환.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `content_type` | string | ✅ | 콘텐츠 유형 |
| `content_id` | i64 | ✅ | 콘텐츠 ID |

**응답 (성공 200)**
```json
{
  "fields": [
    { "content_type": "video", "content_id": 1, "field_name": "video_idx", "source_text": "VID-001" },
    { "content_type": "video_tag", "content_id": 10, "field_name": "video_tag_title", "source_text": "발음 연습" }
  ]
}
```

---

#### 9-11 : `POST /admin/translations/auto-bulk` (벌크 자동 번역)

> 복수 필드 × 복수 언어를 일괄 자동 번역한다. 순수 숫자 source_text는 번역 API 호출 없이 그대로 UPSERT.

**요청 Body (JSON)**
| 필드 | 타입 | 필수 | 설명 |
|------|------|------|------|
| `items` | array | ✅ | 번역 대상 필드 목록 (content_type, content_id, field_name, source_text) |
| `target_langs` | string[] | ✅ | 대상 언어 코드 배열 |

```json
{
  "items": [
    { "content_type": "video", "content_id": 1, "field_name": "video_idx", "source_text": "VID-001" },
    { "content_type": "video_tag", "content_id": 10, "field_name": "video_tag_title", "source_text": "발음 연습" }
  ],
  "target_langs": ["en", "ja", "vi"]
}
```

**응답 (성공 200)**
```json
{
  "total": 6,
  "success_count": 6,
  "fail_count": 0,
  "results": [
    { "content_type": "video", "content_id": 1, "field_name": "video_idx", "lang": "en", "success": true, "translation_id": 42, "translated_text": "VID-001" }
  ]
}
```

---

#### 9-12 : `GET /admin/translations/search` (번역 검색)

> 최근 approved/reviewed 상태의 번역을 조회한다. 언어별 필터 가능.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `lang` | string | N | 언어 코드 필터 (없으면 전체 언어) |

**응답 (성공 200)**
```json
{
  "items": [
    { "translation_id": 42, "content_type": "video", "content_id": 1, "field_name": "video_idx", "lang": "en", "translated_text": "VID-001", "status": "approved" }
  ]
}
```

---

#### 기존 콘텐츠 API `?lang=` 쿼리 파라미터 확장

> 모든 기존 콘텐츠 조회 API(lessons, courses, studies, videos)에 `?lang=` 쿼리 파라미터가 추가된다.

| 기존 엔드포인트 | 확장 예시 | 동작 |
|----------------|-----------|------|
| `GET /courses` | `GET /courses?lang=en` | 코스 목록에 영어 번역 포함 |
| `GET /courses/{id}` | `GET /courses/{id}?lang=ja` | 코스 상세에 일본어 번역 포함 |
| `GET /lessons/{id}` | `GET /lessons/{id}?lang=vi` | 레슨 상세에 베트남어 번역 포함 |
| `GET /studies/tasks/{id}` | `GET /studies/tasks/{id}?lang=zh-CN` | 학습 Task에 중국어(간체) 번역 포함 |

**Fallback 동작**:
1. 요청된 `lang`의 `approved` 번역이 존재하면 → 번역된 텍스트 반환
2. 요청된 `lang`의 번역이 없으면 → `en` (영어) `approved` 번역 시도
3. `en` 번역도 없으면 → `ko` (한국어 원본) 반환

**응답 확장 필드**: `?lang=` 지정 시 응답에 `_translated` 접미사 필드가 추가된다.
```json
{
  "lesson_id": 42,
  "lesson_title": "한글 소개",
  "lesson_title_translated": "Introduction to Korean Alphabet",
  "lesson_description": "한글 기초를 배워보세요",
  "lesson_description_translated": "Learn Hangul basics",
  "translation_lang": "en",
  "translation_coverage": { "title": true, "description": true }
}
```

</details>

---

<details>
<summary><strong>5.10 Phase 10 — 관리자 결제/구독 관리 + 수동 수강권 ✅</strong></summary>

> 관리자가 구독/트랜잭션을 조회·관리하고, Paddle 없이 수동으로 수강권을 부여/회수할 수 있는 어드민 기능.

#### 10-1 : `GET /admin/payment/subscriptions` (구독 목록)

> 관리자가 전체 구독 목록을 조회한다. 이메일 검색, 상태 필터, 정렬, 페이지네이션 지원.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `page` | i64 | N | 페이지 번호 (기본 1) |
| `size` | i64 | N | 페이지 크기 (기본 20, 최대 100) |
| `q` | string | N | 이메일/닉네임 검색 |
| `status` | string | N | 상태 필터 (trialing/active/past_due/paused/canceled) |
| `sort` | string | N | 정렬 기준 (id/created_at/status/billing_interval/price) |
| `order` | string | N | 정렬 방향 (asc/desc) |

**응답 (성공 200)**
```json
{
  "items": [
    {
      "subscription_id": 1,
      "user_id": 42,
      "user_email": "user@example.com",
      "status": "active",
      "billing_interval": "month_3",
      "current_price_cents": 2500,
      "current_period_end": "2026-05-15T00:00:00Z",
      "created_at": "2026-02-15T00:00:00Z"
    }
  ],
  "meta": { "page": 1, "size": 20, "total_count": 50, "total_pages": 3 }
}
```

---

#### 10-2 : `GET /admin/payment/subscriptions/{id}` (구독 상세)

> 구독 상세 정보 + 사용자 정보 + 관련 트랜잭션 내역을 함께 반환.

**응답 (성공 200)**
```json
{
  "subscription": {
    "subscription_id": 1,
    "user_id": 42,
    "provider": "paddle",
    "provider_subscription_id": "sub_01...",
    "provider_customer_id": "ctm_01...",
    "status": "active",
    "billing_interval": "month_3",
    "current_price_cents": 2500,
    "currency": "USD",
    "current_period_start": "2026-02-15T00:00:00Z",
    "current_period_end": "2026-05-15T00:00:00Z",
    "trial_ends_at": null,
    "canceled_at": null,
    "paused_at": null,
    "created_at": "2026-02-15T00:00:00Z",
    "updated_at": "2026-02-15T00:00:00Z"
  },
  "user": {
    "user_id": 42,
    "email": "user@example.com",
    "nickname": "korean_learner",
    "user_auth": "LEARNER"
  },
  "transactions": [
    {
      "transaction_id": 1,
      "status": "completed",
      "amount_cents": 2500,
      "tax_cents": 250,
      "currency": "USD",
      "occurred_at": "2026-02-15T00:00:00Z"
    }
  ]
}
```

---

#### 10-3 : `POST /admin/payment/subscriptions/{id}/cancel` (관리자 구독 취소)

> 관리자가 사용자의 구독을 취소한다. Paddle API 호출 후 감사 로그 기록.

**요청 Body**
```json
{ "immediately": true }
```

- `immediately: true` → 즉시 취소
- `immediately: false` → 다음 결제일에 취소

**응답**: `200 OK` (빈 JSON)

---

#### 10-4 : `POST /admin/payment/subscriptions/{id}/pause` (관리자 구독 일시정지)

> 활성 상태인 구독만 일시정지 가능.

**응답**: `200 OK` (빈 JSON)

---

#### 10-5 : `POST /admin/payment/subscriptions/{id}/resume` (관리자 구독 재개)

> 일시정지 상태인 구독만 재개 가능.

**응답**: `200 OK` (빈 JSON)

---

#### 10-6 : `GET /admin/payment/transactions` (트랜잭션 목록)

> 전체 트랜잭션 목록 조회. 이메일 검색, 상태 필터, 정렬, 페이지네이션.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `page` | i64 | N | 페이지 번호 (기본 1) |
| `size` | i64 | N | 페이지 크기 (기본 20, 최대 100) |
| `q` | string | N | 이메일/닉네임 검색 |
| `status` | string | N | 상태 필터 (completed/refunded) |
| `sort` | string | N | 정렬 기준 (id/occurred_at/status/amount) |
| `order` | string | N | 정렬 방향 (asc/desc) |

**응답 (성공 200)**
```json
{
  "items": [
    {
      "transaction_id": 1,
      "subscription_id": 1,
      "user_id": 42,
      "user_email": "user@example.com",
      "status": "completed",
      "amount_cents": 2500,
      "tax_cents": 250,
      "currency": "USD",
      "billing_interval": "month_3",
      "occurred_at": "2026-02-15T00:00:00Z"
    }
  ],
  "meta": { "page": 1, "size": 20, "total_count": 10, "total_pages": 1 }
}
```

---

#### 10-7 : `POST /admin/payment/grants` (수동 수강권 부여)

> Paddle 구독 없이 관리자가 직접 사용자에게 수강권을 부여한다 (VIP, CS 대응, 이벤트 등).

**요청 Body**
```json
{
  "user_id": 42,
  "expire_at": "2026-12-31T23:59:59Z",
  "reason": "VIP 사용자 수동 부여"
}
```

- `expire_at`: 선택. null이면 무기한.
- `reason`: 필수. 감사 로그에 기록.

**응답 (성공 201)**
```json
{
  "user_id": 42,
  "courses_granted": 5,
  "expire_at": "2026-12-31T23:59:59Z",
  "granted_by": 1,
  "reason": "VIP 사용자 수동 부여",
  "created_at": "2026-02-16T00:00:00Z"
}
```

---

#### 10-8 : `GET /admin/payment/grants` (수동 부여 내역 조회)

> 구독 없이 수강권이 활성화된 사용자 목록 조회.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `page` | i64 | N | 페이지 번호 (기본 1) |
| `size` | i64 | N | 페이지 크기 (기본 20, 최대 100) |
| `q` | string | N | 이메일/닉네임 검색 |

**응답 (성공 200)**
```json
{
  "items": [
    {
      "user_id": 42,
      "user_email": "user@example.com",
      "user_nickname": "korean_learner",
      "active_courses": 5,
      "earliest_enrolled": "2026-01-01T00:00:00Z",
      "latest_expire": "2026-12-31T23:59:59Z"
    }
  ],
  "meta": { "page": 1, "size": 20, "total_count": 3, "total_pages": 1 }
}
```

---

#### 10-9 : `DELETE /admin/payment/grants/{userId}` (수동 수강권 회수)

> 사용자의 모든 수강권을 회수한다.

**응답**: `204 No Content`

</details>

---

<details>
<summary><strong>5.11 Phase 11 — 사용자 결제 (Paddle Billing) ✅</strong></summary>

> Paddle Billing 기반 구독 결제. 플랜 조회, 구독 상태 확인, Webhook 수신.

#### 11-1 : `GET /payment/plans` (플랜 목록)

> 공개 엔드포인트. 구독 플랜 목록 + Paddle Client Token 반환.

**인증**: 불필요 (공개)

**응답 (성공 200)**
```json
{
  "plans": [
    {
      "price_id": "pri_01khg4rcvq9ewz1n1rs9zd59rp",
      "interval": "month_1",
      "price_cents": 1000,
      "currency": "USD",
      "label": "1 Month"
    }
  ],
  "client_token": "test_53998ff59a87110b9c389e35880",
  "sandbox": true
}
```

---

#### 11-2 : `GET /payment/subscription` (내 구독 상태)

> 인증된 사용자의 현재 구독 정보 조회. 구독이 없으면 404.

**인증**: Bearer Token (필수)

**응답 (성공 200)**
```json
{
  "subscription_id": 1,
  "status": "active",
  "billing_interval": "month_3",
  "current_price_cents": 2500,
  "currency": "USD",
  "current_period_start": "2026-02-15T00:00:00Z",
  "current_period_end": "2026-05-15T00:00:00Z",
  "trial_ends_at": null,
  "canceled_at": null,
  "paused_at": null,
  "created_at": "2026-02-15T00:00:00Z",
  "management_urls": {
    "cancel": "https://...",
    "update_payment_method": "https://..."
  }
}
```

---

#### 11-3 : `POST /payment/webhook` (Paddle Webhook)

> Paddle에서 호출하는 Webhook 엔드포인트. 서명 검증 후 이벤트 처리.

**인증**: Paddle HMAC-SHA256 서명 검증 (Paddle-Signature 헤더)

**처리 이벤트**: subscription.created/activated/updated/canceled/paused/resumed/trialing/past_due, transaction.completed

**응답**: `200 OK` (항상)

</details>

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 6. 프론트엔드 구조 & 규칙

> 목적: AMK 백엔드(API)와 일관되게 동작하는 **Vite + React + Tailwind** 기반 프론트엔드의 공통 규칙을 정의한다.  
> 이 섹션은 **웹(반응형, 앱까지 고려)** 을 기준으로 한다.

---

### 6.1 프론트엔드 스택 & 기본 원칙

> 목적: AMK 백엔드(API)와 일관되게 동작하며, **한국어 학습자 환경(저사양/데이터 절약)**에 최적화된 **"Lightweight React"** 아키텍처를 정의한다.

- **기술 스택 (Strict)**
  - **Core**: Vite + React + TypeScript
    - *Create React App(CRA) 및 Next.js 사용 금지 (SPA 모드 유지)*
  - **Styling**: Tailwind CSS
  - **UI Library**: **shadcn/ui** (Radix UI 기반 Headless)
    - *MUI, AntD 등 번들 사이즈가 큰 UI 프레임워크 반입 금지*
  - **State Management**:
    - **Server State**: **TanStack Query (React Query)** (API 캐싱 및 로딩 상태 관리)
    - **Global Client State**: **Zustand** (로그인 세션 등 최소한의 전역 상태)
    - **Form**: **React Hook Form** + **Zod** (렌더링 최적화 및 스키마 검증)
  - **Routing**: React Router (v6)
  - **i18n (다국어)**: **react-i18next** + **i18next** (ko/en 지원, 수동 전환 방식)
  - **HTTP**: `fetch` API 래퍼 (Axios 사용 지양, `src/api/client.ts`로 통일)

- **설계 기본 원칙**
  1. **단일 소스 오브 트루스 (SSOT)**
     - 백엔드 스펙/엔드포인트/상태코드/에러 정책은 **항상 AMK_API_MASTER.md** 를 기준으로 한다.
  
  2. **성능 및 데이터 최적화 (Data Saver First)**
     - **목표**: 인터넷 환경이 좋지 않은 국가의 학습자를 위해 초기 로딩 속도와 데이터 소모를 최소화한다.
     - **Code Splitting**: 모든 페이지 라우트는 `React.lazy`와 `Suspense`를 통해 동적으로 로딩한다.
     - **Asset Lazy Loading**: 이미지와 비디오(Vimeo Player SDK 포함)는 뷰포트에 들어오거나 사용자가 상호작용(클릭)하기 전까지 절대 미리 로드하지 않는다.
     - **No Heavy Libs**: Gzip 기준 **10kb**를 초과하는 외부 라이브러리 추가 시, 반드시 대체재(직접 구현 또는 경량 라이브러리)를 검토한다.

  3. **모바일 퍼스트 & 앱 확장성 (Mobile First Architecture)**
     - **반응형**: 모든 UI는 모바일(`sm`) 기준으로 먼저 설계하고, 태블릿(`md`) 및 데스크톱(`lg`)으로 확장한다.
     - **로직 분리 (Hook Separation)**:
       - 향후 **React Native 모바일 앱** 확장을 고려하여, 비즈니스 로직은 컴포넌트(UI) 내부에 작성하지 않는다.
       - 반드시 **Custom Hook** (`useAuth`, `useVideoPlayer` 등)으로 추출하여 UI와 로직을 100% 분리한다.

  4. **도메인(Category) 주도 구조**
     - 백엔드와 동일하게 `auth / user / video / study / lesson / admin` 도메인 기준으로 폴더와 로직을 격리한다.
     - 페이지 안에서 "즉석 컴포넌트"를 만들지 않고, `common/ui`의 디자인 시스템을 조립하여 사용한다.

---

### 6.2 프론트 디렉터리 구조 & 컴포넌트 계층

> 목적: **도메인 주도(Domain-Driven)** 구조를 기반으로, shadcn/ui 표준과 React Hook 패턴을 결합하여 유지보수성과 확장성을 극대화한다.

#### 6.2.1 디렉터리 구조 (Strict)

- 기준 경로: `frontend/src/`

```text
src/
  app/
    router.tsx           # 라우트 정의 (React Router v6)
    layout_root.tsx      # 최상위 레이아웃 (AppShell)
    providers.tsx        # 전역 Provider 모음 (QueryClient, AuthProvider 등)
  
  api/
    client.ts            # fetch 래퍼 (Axios 지양), Interceptor (토큰/에러)
    # 도메인별 API 호출 함수 (fetcher)
    auth.ts
    user.ts
    video.ts
    study.ts
    lesson.ts
    admin.ts

  category/              # ★ 핵심: 도메인별 기능 격리 (Vertical Slicing)
    auth/
      page/              # 페이지 컴포넌트 (Route와 1:1 매핑)
      component/         # 해당 도메인 전용 UI 조각
      hook/              # 비즈니스 로직 & Custom Hook (UI 분리 원칙)
      types.ts           # 해당 도메인 전용 Request/Response DTO 타입
    user/
      page/
      component/
      hook/
      types.ts
    video/
      # ... (동일 구조)
    study/
      # ... (동일 구조)
    lesson/
      # ... (동일 구조)
    admin/
      # ... (동일 구조)

  components/            # 공용 컴포넌트 (Horizontal Slicing)
    ui/                  # ★ shadcn/ui 설치 경로 (Button, Dialog 등)
    layout/              # Header, Footer, Sidebar 등 레이아웃 조각
    shared/              # 도메인에 종속되지 않는 재사용 컴포넌트 (LoadingSpinner 등)

  i18n/                  # ★ 다국어(i18n) 모듈
    index.ts             # i18next 초기화, changeLanguage/getSavedLanguage 헬퍼
    locales/
      ko.json            # 한국어 번역 (기본 언어)
      en.json            # 영어 번역

  hooks/                 # 전역 Custom Hook
    use_auth.ts          # 인증 상태 관리 (Zustand + Logic)
    use_language_sync.ts # DB 언어 설정 ↔ i18n 동기화
    use_toast.ts         # 알림 UI 제어
    use_mobile.ts        # 모바일 감지 및 반응형 처리

  lib/
    utils.ts             # cn() 등 shadcn/ui 필수 유틸
    constants.ts         # 전역 상수
    format.ts            # 날짜/시간/통화 포맷터
```

> **네이밍 규칙 (Strict)**
> - **Files**:
>   - React 컴포넌트 (`.tsx`): **PascalCase** (예: `LoginPage.tsx`, `VideoCard.tsx`)
>   - 그 외 TS 파일 (`.ts`): **snake_case** (예: `video_api.ts`, `use_auth.ts`, `utils.ts`)
> - **Code**:
>   - 컴포넌트/인터페이스/타입명: **PascalCase**
>   - 변수/함수명: **camelCase**
>   - **API DTO 필드명**: 백엔드 DB 컬럼명과 100% 일치하는 **snake_case** (예: `video_id`, `is_completed`)
>     - *프론트엔드에서 camelCase로 변환하지 않고 그대로 사용한다.*

#### 6.2.2 컴포넌트 3단계 계층

1. **Page 컴포넌트 (`category/*/page/`)**
   - **역할**: 라우팅의 종착점. 데이터 페칭(`useQuery`)과 레이아웃 조립만 담당.
   - **규칙**:
     - `useEffect` 등 복잡한 로직을 직접 포함하지 않는다. (Hook으로 위임)
     - 스타일링(Tailwind)을 최소화하고, `component`들을 배치하는 데 집중한다.
     - 파일명 예시: `VideoListPage.tsx`, `SignupPage.tsx`

2. **도메인 컴포넌트 (`category/*/component/`)**
   - **역할**: 특정 도메인 기능(비디오 플레이어, 문제 풀이 폼)을 수행하는 UI 블록.
   - **규칙**:
     - 해당 도메인(`category`) 내에서만 사용된다.
     - 비즈니스 로직이 필요한 경우, 상위 Page에서 Props로 받거나 전용 Hook을 사용한다.
     - 파일명 예시: `VideoPlayer.tsx`, `AnswerForm.tsx`

3. **공용 UI 컴포넌트 (`components/ui/`)**
   - **역할**: 디자인 시스템의 원자(Atom). (`shadcn/ui` 컴포넌트들)
   - **규칙**:
     - **도메인 로직(비즈니스)을 절대 포함하지 않는다.**
     - `className` prop을 통해 외부에서 스타일 확장이 가능해야 한다.
     - 파일명 예시: `Button.tsx`, `Dialog.tsx`

#### 6.2.3 훅(Hook) & API 레이어 설계

- **API Layer (`src/api/*.ts`)**
  - 순수 함수(Pure Function)로 구성된 `fetch` 호출부.
  - React 의존성(State, Hook)이 전혀 없어야 한다.
  - `client.ts`를 import하여 사용한다.

- **Query Hook (`category/*/hook/`)**
  - **TanStack Query**를 래핑하여 데이터 상태(`isLoading`, `data`, `error`)를 제공하는 훅.
  - 예: `useVideoListQuery`, `useVideoProgressMutation`
  - 이 계층에서 **API 응답 타입(DTO)**과 **프론트엔드 뷰 모델** 간의 변환이 필요하다면 수행한다. (단, 기본적으로는 DTO 구조를 그대로 사용하는 것을 권장)

- **Logic Hook (`category/*/hook/`)**
  - UI 상태(Form, Modal open/close)와 사용자 인터랙션 핸들러를 캡슐화.
  - Page 컴포넌트가 "Controller" 역할을 하지 않도록 로직을 분리해내는 핵심 계층.
  - 예: `useSignupForm`, `useVideoPlayerController`

#### 6.2.4 다국어(i18n) 아키텍처

> 목적: 한국어(ko)와 영어(en)를 지원하며, **사용자 수동 전환** 방식으로 동작한다. 브라우저 언어 자동 감지는 사용하지 않는다.

##### 지원 언어 & 기본값

| 코드 | 언어 | 비고 |
|------|------|------|
| `ko` | 한국어 | **기본 언어 (fallback)** |
| `en` | English | |

##### 언어 결정 우선순위

```
1. DB user_set_language (로그인 상태)
2. localStorage "language" 키
3. 기본값 "ko"
```

- **로그인 시**: `useLanguageSync` 훅이 DB의 `user_set_language`를 가져와 i18n + localStorage에 적용 (최초 1회)
- **비로그인 시**: localStorage에 저장된 언어를 유지
- **로그아웃 시**: 마지막 선택한 언어를 localStorage에서 유지

##### 번역 파일 구조

- 경로: `src/i18n/locales/{ko,en}.json`
- 네임스페이스 구조 (플랫 JSON, 도메인별 prefix):

```json
{
  "common": { "loading": "...", "save": "..." },
  "nav":    { "about": "...", "login": "..." },
  "footer": { "brandDescription": "...", "copyright": "..." },
  "auth":   { "loginTitle": "...", "signupButton": "..." },
  "user":   { "myPageTitle": "...", "settingsTitle": "..." },
  "home":   { "heroTitle": "...", "ctaStart": "..." },
  "about":  { "badge": "...", "missionTitle": "..." },
  "study":  { "listTitle": "...", "kindChoice": "..." },
  "lesson": { "listTitle": "...", "accessPaid": "..." },
  "video":  { "listTitle": "...", "emptyTitle": "..." },
  "error":  { "notFoundTitle": "...", "accessDeniedTitle": "..." }
}
```

- **규칙**: ko.json과 en.json의 키 구조는 **반드시 1:1 일치**해야 한다.
- **보간(Interpolation)**: `{{variable}}` 문법 사용 (예: `"총 {{count}}개"`)

##### 코드 사용 패턴

| 컨텍스트 | 패턴 | 예시 |
|----------|------|------|
| React 컴포넌트 내부 | `useTranslation` 훅 | `const { t } = useTranslation();` → `t("auth.loginTitle")` |
| React 컴포넌트 외부 (Hook, Zod 스키마 등) | `i18n.t()` 직접 호출 | `import i18n from "@/i18n";` → `i18n.t("common.requestFailed")` |
| 언어 변경 | `changeLanguage` 헬퍼 | `import { changeLanguage } from "@/i18n";` → `changeLanguage("en")` |

##### 언어 전환 UI & 동기화

- **헤더 토글**: Globe 아이콘 버튼으로 ko↔en 전환
  - 데스크톱: `"EN"` / `"KO"` 약어 표시
  - 모바일: `"English"` / `"한국어"` 전체 표시 (전환 대상 언어를 해당 언어로 표기)
  - 로그인 상태일 경우 `useUpdateSettings`로 DB에도 저장
- **설정 페이지**: Select 드롭다운으로 언어 선택 → 저장 시 DB + i18n 동시 적용
- **동기화**: 헤더 토글 변경 시 `i18n.language` 변경 감지를 통해 설정 페이지 form에 즉시 반영

##### 적용 범위

| 대상 | i18n 적용 | 비고 |
|------|-----------|------|
| 사용자 대면 페이지 (홈, 로그인, 학습 등) | O | 모든 UI 텍스트 `t()` 처리 |
| 레이아웃 (헤더, 푸터) | O | |
| 에러 페이지 (404, 403, 500) | O | |
| 관리자(Admin) 페이지 | X | 한국어 전용 (관리자가 한국어 사용자) |
| Zod 유효성 검증 메시지 | O | `i18n.t()` 패턴 사용 |
| Toast 알림 메시지 | O | Hook 내에서 `i18n.t()` 사용 |

---

### 6.3 라우팅 & 접근 제어

> 목적: 5. 기능 & API 로드맵의 “화면 경로”를 기준으로, **Code Splitting이 적용된 React Router 트리**와 **엄격한 접근 제어(Auth/Admin Guard)**를 정의한다.

#### 6.3.1 라우트 매핑 원칙 (Lazy Loading 필수)

- **라우트 정의 위치**
  - `src/app/router.tsx` 에서 **전체 라우트 트리**를 정의한다.
  - **성능 원칙**: 모든 페이지 컴포넌트는 `React.lazy`로 import하여, 초기 번들 사이즈를 최소화해야 한다.

- **파일명 패턴 (예시)**
  - `/` → `category/home/page/HomePage.tsx` (홈)
  - `/about` → `category/about/page/AboutPage.tsx` (소개)
  - `/login` → `category/auth/page/LoginPage.tsx`
  - `/videos/:video_id` → `category/video/page/VideoDetailPage.tsx`
  - `/admin/users` → `category/admin/page/AdminUserListPage.tsx`
  - *파일명은 PascalCase를 따른다.*

- **라우트 구성 예시 (Strict Code Splitting)**

```tsx
// app/router.tsx
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Suspense, lazy } from "react";
import { AppShell } from "@/components/layout/AppShell"; // layout 경로 수정됨
import { RequireAuth } from "./route_guard_auth";
import { RequireAdmin } from "./route_guard_admin";
import { LoadingSpinner } from "@/components/shared/LoadingSpinner";

// ★ 핵심: 모든 페이지는 Lazy Load 처리
const HomePage = lazy(() => import("@/category/home/page/HomePage"));
const AboutPage = lazy(() => import("@/category/about/page/AboutPage"));
const LoginPage = lazy(() => import("@/category/auth/page/LoginPage"));
const SignupPage = lazy(() => import("@/category/auth/page/SignupPage"));
const VideoListPage = lazy(() => import("@/category/video/page/VideoListPage"));
const VideoDetailPage = lazy(() => import("@/category/video/page/VideoDetailPage"));
const StudyListPage = lazy(() => import("@/category/study/page/StudyListPage"));
const LessonListPage = lazy(() => import("@/category/lesson/page/LessonListPage"));
const MePage = lazy(() => import("@/category/user/page/MePage"));
const AdminUserListPage = lazy(() => import("@/category/admin/page/AdminUserListPage"));

export function AppRouter() {
  return (
    <BrowserRouter>
      {/* Suspense: Lazy Loading 중 보여줄 Fallback UI */}
      <Suspense fallback={<LoadingSpinner fullScreen />}>
        <AppShell>
          <Routes>
            {/* Public Routes */}
            <Route path="/" element={<HomePage />} />
            <Route path="/about" element={<AboutPage />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/signup" element={<SignupPage />} />
            <Route path="/account-recovery" element={<AccountRecoveryPage />} />
            <Route path="/verify-email" element={<VerifyEmailPage />} />
            <Route path="/videos" element={<VideoListPage />} />
            <Route path="/videos/:video_id" element={<VideoDetailPage />} />
            <Route path="/studies" element={<StudyListPage />} />
            <Route path="/lessons" element={<LessonListPage />} />

            {/* Protected Routes (Member) */}
            <Route element={<RequireAuth />}>
              <Route path="/me" element={<MePage />} />
            </Route>

            {/* Admin Routes (RBAC) */}
            <Route element={<RequireAdmin />}>
              <Route path="/admin/users" element={<AdminUserListPage />} />
              {/* ... other admin routes */}
            </Route>
            
            {/* 404 Handling */}
            <Route path="*" element={<div>Page Not Found</div>} />
          </Routes>
        </AppShell>
      </Suspense>
    </BrowserRouter>
  );
}
```

> 실제 구현 시 파일명/컴포넌트명은 이 문서의 **네이밍 규칙(3.2.4 프론트엔드 네이밍)** 을 따른다.

#### 6.3.2 접근 제어 패턴 (Auth / Admin 가드)

- **공통 개념**
  - 백엔드의 상태축을 프론트에서 `useAuth()` 훅을 통해 `pass / stop / forbid` 상태로 해석한다.
  - **권한 확인 로직은 `hooks/use_auth.ts`에 중앙화한다.**

- **`RequireAuth` (사용자 로그인 필수)**
  - **로직**:
    - `authStatus === "pass"` (토큰 유효) AND `user_state === "on"` (계정 활성)
  - **실패 시 처리**:
    - `authStatus === "stop"` (미로그인/토큰만료) → 로그인 페이지로 이동 (`state: { from: location }` 전달)
    - `user_state !== "on"` (정지/탈퇴) → "계정 비활성화" 안내 페이지로 이동.

- **`RequireAdmin` (관리자 RBAC)** ✅ 구현 완료 (2026-02-01)
  - **로직**:
    - `RequireAuth` 통과 AND `user_auth_enum` IN `['HYMN', 'admin']`
    - ⚠️ `manager` 역할은 **admin 접근 불가** (향후 class 기반 접근 권한으로 별도 구현 예정)
  - **실패 시 처리**:
    - 인증은 되었으나 권한 부족 → `/403` 페이지로 리다이렉트
    - *절대 로그인 페이지로 튕겨내지 않는다 (무한 루프 방지).*
  - **백엔드 미들웨어** (`src/api/admin/role_guard.rs`):
    - HYMN/admin → 200 통과
    - manager → 403 "Access denied: Manager role requires class-based access"
    - learner → 403 "Access denied: Insufficient permissions for admin access"

- **에러 페이지** ✅ 구현 완료 (2026-02-01)
  - 위치: `frontend/src/category/error/page/`
  - 페이지 목록:
    | 라우트 | 컴포넌트 | 설명 |
    |--------|----------|------|
    | `/403` | `AccessDeniedPage` | 권한 없음 (ShieldX 아이콘) |
    | `/error` | `ErrorPage` | 서버 에러 (ServerCrash 아이콘, 재시도 버튼) |
    | `*` | `NotFoundPage` | 404 페이지 없음 (FileQuestion 아이콘) |

- **Redirect 정책 (Guest Guard)**
  - 로그인 상태(`pass`)인 사용자가 `/login` 또는 `/signup` 접근 시:
    - 일반 사용자 → `/videos` (메인)으로 리다이렉트
    - 관리자 → `/admin/dashboard` 등으로 리다이렉트 (선택 사항)

---

### 6.4 상태 관리 & API 연동 패턴

> 목적: **TanStack Query(Server State)**와 **Zustand(Client State)**를 중심으로, 백엔드 API와 프론트엔드 UI를 **선언적(Declarative)**으로 연결한다.

#### 6.4.1 인증 상태 관리 (Zustand + AuthProvider)

- **토큰/세션 보관 전략 (Strict)**
  - **Access Token**: 메모리(Zustand Store) 또는 React Query 캐시에만 보관. (LocalStorage 저장 금지 - XSS 취약)
  - **Refresh Token**: `httpOnly` 쿠키로 백엔드가 설정. (JS 접근 불가)

- **Auth Store 구조 (`hooks/use_auth.ts`)**
  - `Zustand`를 사용하여 전역 인증 상태를 관리한다.
  - **State**:
    - `user`: User DTO | null
    - `authStatus`: `"pass"`(인증됨) | `"stop"`(미인증/만료) | `"forbid"`(권한부족)
    - `isAdmin`: boolean (Helper Getter)
  - **Actions**:
    - `login(token, user)`: 상태 업데이트 및 토큰 메모리 저장
    - `logout()`: 상태 초기화 및 `/auth/logout` API 호출
    - `refresh()`: 앱 초기 진입 시 `/auth/refresh` 호출하여 세션 복구

#### 6.4.2 공통 API 클라이언트 (`src/api/client.ts`)

- **역할**
  - `fetch` API 기반의 Singleton 인스턴스.
  - **Interceptor**: 요청 시 헤더에 `Authorization: Bearer {token}` 자동 주입.
  - **Error Handling**: HTTP 에러를 `AppError` 객체로 변환하여 throw.

- **네이밍 규칙 (Strict)**
  - **Request/Response DTO는 백엔드와 동일하게 `snake_case`를 사용한다.**
  - 프론트엔드에서 `camelCase`로 변환하지 않는다. (불필요한 연산 및 매핑 오버헤드 제거)

- **에러 매핑 규칙 (Global Error Boundary)**
  - `401 Unauthorized` → `authStatus`를 `"stop"`으로 변경하고 로그인 모달/페이지 유도.
  - `403 Forbidden` → `authStatus`를 `"forbid"`로 변경.
  - `5xx Server Error` → Toast 메시지로 "잠시 후 다시 시도해주세요" 출력.

#### 6.4.3 도메인별 훅 패턴 (React Query & Custom Hooks)

> **원칙**: UI 컴포넌트는 `useEffect`를 사용하지 않고, 아래 훅을 통해 데이터를 구독한다.

- **Query Hook (Data Fetching)**
  - **TanStack Query**를 사용하여 서버 상태를 관리한다.
  - 파일 위치: `category/*/hook/use[Domain]Query.ts`
  - 예시:
    ```typescript
    // useVideoListQuery.ts
    export const useVideoListQuery = (params) => {
      return useQuery({
        queryKey: ["videos", params],
        queryFn: () => fetchVideos(params), // api/video.ts 호출
        staleTime: 1000 * 60 * 5, // 5분간 캐시 유지 (데이터 절약)
      });
    };
    ```

- **Mutation Hook (Data Update)**
  - 데이터 변경(POST/PUT/DELETE)을 담당한다.
  - 예시:
    ```typescript
    // useVideoProgressMutation.ts
    export const useVideoProgressMutation = () => {
      const queryClient = useQueryClient();
      return useMutation({
        mutationFn: updateVideoProgress,
        onSuccess: () => {
          queryClient.invalidateQueries(["videos"]); // 목록 갱신
        }
      });
    };
    ```

- **Controller Hook (UI Logic)**
  - 폼 핸들링, 모달 제어 등 순수 클라이언트 로직.
  - `useForm`(React Hook Form)과 `zod` 스키마를 결합하여 사용한다.
  - 예: `useSignupForm`, `useVideoPlayerController`

#### 6.4.4 상태축과 UI 상태 매핑

> **5. 기능 & API 로드맵**의 상태축을 프론트엔드 변수로 변환하는 규칙이다.

- **Request 상태 (React Query 상태 매핑)**
  - `pending` → `isLoading` (스피너 표시)
  - `error` → `isError` (에러 메시지/재시도 버튼 표시)
  - `success` → `data` (콘텐츠 렌더링)
  - `retryable` → React Query의 `retry` 옵션으로 자동 처리

- **Course 상태 (접근 권한 계산)**
  - `/videos/{id}` 등 유료 콘텐츠 접근 시 `Course` 축(`buy/taster/buy-not`)을 계산하는 로직은 **Selector** 또는 **Helper Hook**으로 분리한다.
  - 예: `useCourseAccess(videoId)`
    - Return: `{ canPlay: boolean, showPaywall: boolean }`
    - 로직: 내 수강권 목록과 해당 비디오의 `is_free` 여부를 대조.

- **Form 상태**
  - React Hook Form의 `formState`를 그대로 활용한다.
  - `isSubmitting` (전송 중), `isValid` (유효성 검증 통과), `errors` (필드별 에러)

### 6.5 UI/UX & Tailwind 규칙 (shadcn/ui System)

> 목적: **shadcn/ui** 디자인 시스템을 기반으로, 모바일 퍼스트 및 의미론적(Semantic) 스타일링 규칙을 정의하여 일관성과 생산성을 확보한다.

#### 6.5.1 디자인 시스템 철학 (Shadcn First)

- **Mobile First**: 모든 레이아웃은 모바일(`sm`)에서 시작하여 태블릿(`md`), 데스크톱(`lg`)으로 확장한다.
- **Semantic Styling**: 색상 코드를 직접 사용하지 않고, 역할에 따른 변수를 사용한다.
  - ❌ Bad: `bg-blue-600`, `text-gray-500`
  - ⭕ Good: `bg-primary`, `text-muted-foreground`
- **Atomic Components**:
  - 버튼, 인풋 등을 처음부터 만들지 않는다.
  - `components/ui/`에 설치된 **shadcn 컴포넌트**(`<Button>`, `<Input>`, `<Card>`)를 조립하여 화면을 구성한다.

#### 6.5.2 레이아웃 & 그리드

- **AppShell (`components/layout/RootLayout.tsx`)**
  - 앱의 최상위 껍데기.
  - 구성:
    - **Header**: 로고 + 햄버거 메뉴(모바일) / 네비게이션(데스크톱) + 로그인/로그아웃 버튼
    - **Main**: `max-w-screen-xl mx-auto px-4` (콘텐츠 중앙 정렬 및 가로 여백 확보)
    - **Footer**: 회사 정보, 연락처, 이용약관/개인정보처리방침 링크

- **Header 네비게이션 구조**
  ```
  ┌─────────────────────────────────────────────────────────────────┐
  │ [Amazing Korean]    [소개] [영상] [학습] [수업]     [로그인/로그아웃] │
  │      (Logo)           (Navigation)                  (Auth)       │
  └─────────────────────────────────────────────────────────────────┘
  ```
  - **왼쪽 (Logo)**: "Amazing Korean" 텍스트 로고 (클릭 시 `/` 홈으로 이동)
  - **가운데 (Navigation)**: 메인 메뉴
    | 메뉴명 | 라우트 | 설명 |
    |--------|--------|------|
    | 소개 | `/about` | 서비스 소개 |
    | 영상 | `/videos` | 영상 목록 |
    | 학습 | `/studies` | 학습 목록 |
    | 수업 | `/lessons` | 수업 목록 |
  - **오른쪽 (Auth)**: 인증 상태에 따른 조건부 렌더링
    - 비로그인: `[로그인]` `[회원가입]` 버튼
    - 로그인: `[마이페이지]` `[로그아웃]` 버튼

- **반응형 전략**
  - **Grid**: `grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6` 패턴을 기본으로 한다.
  - **Spacing**: 모바일에서는 `gap-4`, 데스크톱에서는 `gap-6` 이상을 사용하여 시원한 느낌을 준다.

#### 6.5.3 Tailwind & Color System (Theme)

- **색상 토큰 (globals.css 기반)**
  - `primary`: 브랜드 메인 컬러 (Amazing Korean Blue) → 주요 액션 버튼
  - `secondary`: 보조 컬러 → 취소/서브 버튼
  - `destructive`: 위험/삭제 → `bg-red-600` 계열
  - `muted`: 비활성/배경 → `bg-gray-100` 계열
  - `accent`: 강조 포인트 → 학습 완료 체크 등

- **타이포그래피**
  - `h1` (Page Title): `text-2xl font-bold tracking-tight md:text-3xl`
  - `h2` (Section): `text-xl font-semibold tracking-tight`
  - `p` (Body): `leading-7 [&:not(:first-child)]:mt-6`
  - `small` (Caption): `text-sm font-medium leading-none`
  - `muted` (Subtext): `text-sm text-muted-foreground`

- **유틸리티 함수 (`cn`)**
  - Tailwind 클래스 병합을 위해 `lib/utils.ts`의 `cn()` 함수를 적극 활용한다.
  - 예: `<div className={cn("flex items-center", isMobile && "flex-col")}>`

#### 6.5.4 주요 UI 패턴 가이드

- **Card Pattern (목록 아이템)**
  - `Card`, `CardHeader`, `CardContent`, `CardFooter` 컴포넌트 조합 사용.
  - 썸네일(이미지/비디오)은 **`aspect-video` (16:9 비율)** 클래스를 사용하여 레이아웃 이동(CLS)을 방지한다.

- **Form Pattern (로그인/입력)**
  - **React Hook Form** + **zod** + **shadcn Form** 조합 필수.
  - `<Form>` 감싸기 → `<FormField>` → `<FormItem>` → `<FormControl>` 구조 준수.
  - 에러 메시지는 `<FormMessage />` 컴포넌트로 자동 노출.

- **Feedback (Toast)**
  - 사용자 액션 결과는 `alert()` 대신 **Toast** (`hooks/use-toast.ts`)를 사용한다.
  - 성공: `toast({ title: "저장되었습니다.", variant: "default" })`
  - 에러: `toast({ title: "오류 발생", variant: "destructive" })`

#### 6.5.5 미디어 & 데이터 최적화 (UX)

- **이미지 (Image)**
  - 포맷: `WebP` 사용 권장.
  - 로딩: `loading="lazy"` 속성 필수.
  - 플레이스홀더: 이미지가 로드되기 전 `bg-muted` 영역을 미리 잡아준다.

- **비디오 (Video)**
  - 목록 화면에서는 무거운 `Vimeo Player` 대신 **가벼운 썸네일 이미지**만 보여준다.
  - 사용자가 "재생" 버튼을 클릭했을 때만 플레이어 SDK를 로드한다 (Lazy Interaction).

---

### 6.6 프론트 테스트 & 로컬 개발 (요약)

> 목적: Vite + React 환경에서 **Type Safety**를 보장하며, 빌드된 정적 자원(`dist/`)을 운영 환경에 일관되게 배포하는 파이프라인을 정의한다.

#### 6.6.1 로컬 개발 플로우

- **패키지 관리**
  - `npm`을 표준 패키지 매니저로 사용한다. (`package-lock.json` 공유)
  - 설치: `npm install`
  - shadcn 컴포넌트 추가: `npx shadcn@latest add [component-name]`

- **환경 변수 (.env)**
  - `.env.local` (로컬 전용, gitignore 대상)
  - `.env.production` (운영 전용)
  - 필수 변수:
    - `VITE_API_BASE_URL`: 백엔드 API 주소 (예: `http://localhost:8080` 또는 `https://api.amazingkorean.net`)
    - *Client 코드에서는 `import.meta.env.VITE_API_BASE_URL`로 접근.*

- **개발 서버 실행**
  - `npm run dev` (기본 포트: 5173)

> 빌드, 배포, CI/CD, EC2 유지보수 등은 [`AMK_DEPLOY_OPS.md`](./AMK_DEPLOY_OPS.md) 참조

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 7. 작업 방식 / 엔지니어링 가이드 (요약)

> 기존 `AMK_ENGINEERING_GUIDE.md` + `README_for_assistant.md` + `AMK_PROJECT_JOURNAL.md`의 “결정/규칙” 부분을 통합한 섹션.

### 7.1 작업 원칙

1. **문서 우선**
   - 스펙/기능/규칙은 항상 이 문서를 기준으로 한다.
2. **파일 전체 교체본**
   - LLM/Gemini에게 코드 패치를 요청할 때는 **항상 “파일 전체 교체본”**으로 요청/응답한다.
3. **정적 가드 필수**
   - `cargo fmt -- --check`
   - `cargo clippy -- -D warnings`
   - `cargo check`
   - 위 3개를 모두 통과해야 PR/머지 가능.
4. **마이그레이션 규칙**
   - 이미 적용된 마이그레이션 파일은 **수정/이름 변경 금지**.
   - 변경이 필요하면 항상 **새 마이그레이션 추가**.
   - SQLx 마커는 `--! up` / `--! down`만 사용 (ASCII 하이픈).
   - 적용 순서 : 1) USERS → 2) LOGIN → 3) VIDEO → 4) STUDY → 5) LESSON
5. **로그/감사**
   - 도메인별 변경 사항 기록 : `USERS_LOG`, `LOGIN_LOG`, `VIDEO_LOG`, `STUDY_TASK_LOG`
   - 관리자 활동 사항 기록 : `ADMIN_USERS_LOG`, `ADMIN_VIDEO_LOG`, `ADMIN_STUDY_LOG`, `ADMIN_LESSON_LOG`
6. **보안 (기본)**  
   - 계정 상태
     - `user_state == 'on'` 인 사용자만 로그인/액세스 허용.
     - 로그인 시점 + 모든 인증 보호 엔드포인트에서 `user_state`를 다시 검증한다.
   - 인증 토큰
     - 모든 보호 엔드포인트는 `Authorization: Bearer <ACCESS_TOKEN>`를 요구한다.
     - 토큰 안의 `sub`(user_id)는 **유일한 신뢰 가능한 사용자 식별자**로 사용하고,  
       요청 바디/쿼리로 들어오는 user_id는 신뢰하지 않는다.
   - 비밀번호 및 민감정보
     - 비밀번호는 Argon2 등 안전한 해시로만 저장하고, **원문은 절대 저장/로그에 남기지 않는다.**
     - USERS_LOG, ADMIN_*_LOG 등 어떤 로그에도 비밀번호/토큰/쿠키 값은 남기지 않는다.
   - 세션/리프레시 토큰
     - 세션/리프레시는 Redis 키(`ak:session:*`, `ak:refresh:*`)를 사용한다.
     - 리프레시는 **사용 시 로테이션(rotate-on-use)** 하고,  
       재사용이 감지되면 해당 세션/사용자의 관련 세션을 일괄 폐기하는 정책과 연동한다.
   - 관리자 RBAC
     - 관리자 롤은 `HYMN / admin / manager` 를 기준으로 한다.
     - `/admin/**` 경로는 기본적으로 **“허용된 롤만 접근 가능”**(default deny) 원칙을 따른다.
     - 롤별 세부 권한 매트릭스는 **Section 8.1 (Open Questions)**에서 정의/업데이트 한다.
   - 통신
     - 운영 환경에서는 반드시 HTTPS를 사용하고, 토큰/세션 ID를 URL(query string)에 노출하지 않는다.

### 7.2 개발 플로우

1. 문서 확인 (**AMK_API_MASTER.md** + 관련 파일)
2. 1) 기존 개발 사항 : 문서 확인 및 참조 후 해당 개발 사항 작업 진행
   2) 신규 개발 사항 : 신규 API 명시 → 문서 확인 및 참조 → 문서 형식으로 업데이트 → 해당 개발 사항 작업 진행
3. 코드/마이그레이션 생성 ([`AMK_CODE_PATTERNS.md`](./AMK_CODE_PATTERNS.md) 패턴 참조)
4. 정적 가드 (`cargo check` / `cargo fmt -- --check` / `cargo clippy -- -D warnings`) + 스모크 테스트
5. 로드맵 체크박스 업데이트 + 문서 동기화

### 7.3 DTO/검증 규칙 (요약)

- 공통 원칙
  - HTTP 경계에서는 항상 **DTO(struct)** 를 사용하고,  
    내부 도메인 타입과 분리한다.
  - 필수/옵션 필드, 기본값, 검증 규칙은 **DTO에 명시**한다.

- 문자열 필드
  - `trim` 후 검증을 기준으로 한다.
  - 길이 제한을 명시한다. (예: 이메일/닉네임 등은 최소/최대 길이 지정)
  - 공백만 있는 문자열은 “빈 값”으로 처리하고, 필요한 경우 400으로 반환한다.

- 이메일
  - RFC 이메일 형식 검증 (예: `validator` 크레이트).
  - 대소문자는 구분하지 않는 것을 기본 가정으로 한다.

- 비밀번호
  - 최소 길이/복잡도는 프로젝트 정책으로 정의 (예: 최소 8자 이상).
  - DTO에서 문자열 길이만 검증하고, **해시는 service 계층에서 수행**한다.
  - 비밀번호 원문은 절대 로그/이력에 남기지 않는다.

- 날짜
  - DTO에서는 `chrono::NaiveDate` 사용.
  - DB에는 `DATE` 또는 `TIMESTAMPTZ`로 캐스팅한다.
  - 잘못된 날짜 형식은 400 + `invalid_argument` 로 응답한다.

- Enum 필드
  - DTO에서는 enum 타입을 사용하거나, 문자열 입력을 enum으로 매핑한다.
  - 정의되지 않은 값이 들어오면 400 + `invalid_argument`.
  - enum 값은 **이 문서 4. 데이터 모델 개요의 enum 정의**를 기준으로 한다.

- ID / 페이징
  - ID는 음수가 아닌 정수로 검증한다. (0 또는 음수는 400)
  - 페이징 파라미터
    - `page >= 1`, `1 <= size <= 최대값(예: 100)`
    - 위반 시 400 + `invalid_argument`.

### 7.4 서비스 계층 및 파일 구조

> 기준 경로: `src/` (예: `\\wsl.localhost\Ubuntu\home\kkryo\dev\amazing-korean-api\src`)

#### 7.4.1 디렉터리 구조(요약)

- `src/api`
  - `admin/{lesson,study,user,video}/`
    - `dto.rs`, `handler.rs`, `repo.rs`, `router.rs`, `service.rs`, `mod.rs`
  - `auth/`
    - `dto.rs`, `extractor.rs`, `handler.rs`, `jwt.rs`, `repo.rs`, `router.rs`, `service.rs`, `token_utils.rs`, `mod.rs`
  - `health/`
    - `handler.rs`, `mod.rs`
  - `lesson/`, `study/`, `user/`, `video/`
    - 각 도메인별 `dto.rs`, `handler.rs`, `repo.rs`, `router.rs`, `service.rs`, `mod.rs`
  - `scripts/`
    - `db_fastcheck.sh`
  - `mod.rs` (api 루트 모듈)

- 루트 파일
  - `config.rs`  : 환경 변수/설정 로딩
  - `docs.rs`    : OpenAPI/Swagger 정의
  - `error.rs`   : 공통 에러 타입(AppError 등)
  - `main.rs`    : 엔트리 포인트(서버 부트스트랩)
  - `state.rs`   : `AppState` 정의(DB 풀, Redis, 설정 등)
  - `types.rs`   : 공용 타입/별칭

#### 7.4.2 계층별 역할

- `dto.rs`
  - 요청/응답 DTO 정의
  - `serde`/`validator`/`utoipa::ToSchema` 등을 사용
  - **핵심:** HTTP 경계에서만 쓰이는 타입(내부 도메인 모델과 분리)

- `handler.rs`
  - Axum 핸들러 함수(라우트별 엔드포인트 구현)
  - 역할:
    - Path/Query/Json 등 요청 파라미터 파싱
    - DTO 검증 결과 처리
    - `Claims`/`AppState` 추출
    - **비즈니스 로직은 직접 수행하지 않고 `service`를 호출**

- `service.rs`
  - 도메인 비즈니스 로직의 중심 계층
  - 역할:
    - 유즈케이스 단위 메서드 (예: `signup_user`, `update_video`, `submit_answer`)
    - 여러 `repo` 호출을 묶어 **트랜잭션 경계**를 형성
    - 검증/권한 체크/상태 전이 규칙을 여기서 처리
  - 원칙:
    - HTTP/프레임워크 의존성 없음 (가능한 한 순수 로직 유지)
    - handler는 얇게, service는 두껍게

- `repo.rs`
  - DB 접근 전담 계층(sqlx 쿼리)
  - 역할:
    - SELECT/INSERT/UPDATE/DELETE 및 저장 프로시저/함수 호출
    - 입력/출력을 struct로 매핑
  - 원칙:
    - 비즈니스 규칙은 넣지 않는다(검증/권한/상태 전이는 service 담당)
    - 필요 시 트랜잭션 핸들러(`&mut Transaction<'_, Postgres>`)를 인자로 받아 사용

- `router.rs`
  - 도메인별 서브 라우터 정의
  - 역할:
    - 각 HTTP 메서드 + 경로에 `handler`를 매핑
    - 도메인 공통 미들웨어(예: 관리자 인증, 로깅) 부착
  - 반환 타입:
    - `Router<AppState>` (상위 `api::mod.rs`에서 `.nest("/videos", video::router())` 형태로 사용)

- `mod.rs`
  - 각 도메인 모듈의 루트
  - 역할:
    - `pub mod dto; pub mod handler; ...` 선언
    - `pub fn router() -> Router<AppState>` 같은 진입 함수 노출
    - 상위 모듈에서 사용할 공개 타입/함수 re-export

#### 7.4.3 특수 모듈(auth, health, scripts)

- `api/auth/`
  - `extractor.rs` : `Claims` 등 인증 관련 Axum extractor
  - `jwt.rs`       : JWT 인코딩/디코딩, 키 관리
  - `token_utils.rs`: 액세스/리프레시 토큰 생성·검증 유틸
  - 나머지(`dto/handler/repo/service/router`)는 일반 도메인과 동일 패턴

- `api/health/`
  - `handler.rs`: `/health/live`, `/health/ready` 등 헬스체크 엔드포인트
  - `mod.rs`: 헬스 라우터 노출

- `api/scripts/db_fastcheck.sh`
  - 로컬/CI용 DB 빠른 연결 확인 스크립트
  - `sqlx` 마이그레이션 실행 전 DB 준비 상태 점검 등에 사용

### 7.5 트랜잭션 패턴

> 목표: **여러 DB 작업을 한 덩어리(원자 단위)로 처리**해서  
> 중간에 에러가 나면 전부 되돌리고, 성공하면 전부 반영되도록 한다.

#### 7.5.1 언제 트랜잭션을 쓰는가

- 대표 사용 사례
  - `USERS` + `USERS_LOG` 같이 **본 테이블 + 로그 테이블**을 함께 갱신할 때
  - `VIDEO` / `STUDY` / `LESSON` 데이터를 수정하면서 **관련 `ADMIN_*_LOG`까지 함께 기록**할 때
  - 한 HTTP 요청 안에서 **여러 테이블을 연속으로 변경**해야 할 때
  - 결제/수강권 등 **비즈니스 일관성이 특히 중요한 작업** (PAY + COURSE/COURSE_PROGRESS 등)
- 원칙
  - “이 중 하나만 반영되고 나머지는 실패하면 안 되는 작업”이면 **트랜잭션을 쓴다.**
  - “로그/통계가 약간 늦게 쌓여도 되느냐?”가 아니라  
    **“이 변경과 로그/통계가 항상 같이 있어야 하느냐”** 기준으로 판단한다.
  - 현재 AMK의 기본 방향:
    - **핵심 비즈니스 상태 + 그에 대한 로그**는 한 트랜잭션 안에서 함께 처리한다.

#### 7.5.2 어디에서 트랜잭션을 여는가

- handler 계층
  - 트랜잭션을 **직접 열지 않는다.**
  - 역할:
    - HTTP 요청 파싱 (path/query/body)
    - 인증/인가 정보 추출 (Claims 등)
    - 기본 수준의 유효성 검사
    - → 이후 **service** 함수 호출
- service 계층
  - **트랜잭션 시작/커밋/롤백의 책임을 가진다.**
  - 하나의 유즈케이스(예: `/users` signup, `/users/me` update 등)에 필요하다면  
    service 함수에서 트랜잭션을 열고, 도메인 repo들을 호출한다.
  - 패턴 예시:

    ```rust
    pub async fn update_user_and_log(
        state: &AppState,
        input: UpdateUserInput,
        actor_id: i64,
    ) -> AppResult<User> {
        // 1) 트랜잭션 시작
        let mut tx = state.db.begin().await?;

        // 2) 메인 상태 변경
        let user = user_repo::update_user(&mut tx, &input).await?;

        // 3) 로그 기록
        user_repo::insert_users_log(&mut tx, &user, actor_id, "update_profile").await?;

        // 4) (필요 시) 통계/기타 파생 데이터 갱신

        // 5) 전부 성공했으면 커밋
        tx.commit().await?;

        Ok(user)
    }
    ```
    
  - 중간에 에러가 나면 `commit()`에 도달하지 못하고,
    트랜잭션 객체가 drop되면서 전체 작업이 **롤백**된다고 보는 것을 기본 전제로 한다.
- repo 계층
  - “어떤 실행 컨텍스트(DB 연결 or 트랜잭션)를 받을지”만 신경 쓴다.
  - 트랜잭션을 시작/커밋하지 않고, **넘겨받은 executor 안에서만 쿼리 실행**한다.
    ```rust
    pub async fn update_user<'e, E>(
        executor: E,
        input: &UpdateUserInput,
    ) -> AppResult<UserRow>
    where
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    {
        // UPDATE users SET ... WHERE user_id = ...
        // ...
    }
    ```
  - 같은 repo 함수가 트랜잭션 안/밖 양쪽에서 재사용될 수 있도록  
    **`&PgPool` / `&mut Transaction<'_, Postgres>` 모두를 받을 수 있는 제네릭 executor 패턴**을 사용한다.

#### 7.5.3 API upsert 패턴 (예: 비디오 진도 저장)

- 비디오 진도 API는 DB 함수 형태로 upsert를 처리하는 것을 기본 패턴으로 한다.
  - 예: `api_upsert_video_progress(user_id, video_id, progress, ...)`
- 이 함수 안에서:
  - 새 기록이면 `INSERT`
  - 기존 기록이면 `UPDATE`
  - 필요한 경우 `VIDEO_LOG` / `VIDEO_STAT_DAILY` 등 연관 정보까지 함께 갱신
  - → 를 **한 번에 처리**하도록 설계한다. (DB 함수 내부가 하나의 트랜잭션 역할)
- 서비스 계층에서는:
  - 1) 입력 검증 (0~100 범위, 사용자 권한, 소유권 등)
  - 2) `api_upsert_video_progress(...)`를 **한 번 호출하는 것**을  
       “이 작업의 트랜잭션 단위”로 본다.
- 한 HTTP 요청에서
  - “진도 upsert + 다른 테이블 변경”이 함께 필요하다면,
  - 7.5.2 패턴대로 **서비스에서 트랜잭션을 열고**, 그 안에서
    - `api_upsert_video_progress(...)`
    - + 기타 repo 함수
    - 를 함께 호출한다.

#### 7.5.4 트랜잭션 내부 순서 패턴

> 기본 순서: **검증 → 메인 변경 → 로그 → (통계/파생) → 커밋**

- 1) 검증 / 현재 상태 조회
  - 예:
    - 대상 레코드 존재 여부 확인
    - 소유권/권한 체크
    - 중복 여부(이메일 중복 등)
  - 주로 `SELECT ... FOR UPDATE` 또는 단순 `SELECT` 로 처리
- 2) 메인 상태 변경
  - 비즈니스에 직접적인 영향을 주는 테이블 변경
    - 예: `USERS`, `USERS_SETTING`, `STUDY_TASK_STATUS`, `LESSON_PROGRESS` 등
  - `INSERT` / `UPDATE` / `DELETE` 중심
- 3) 로그/감사 기록
  - `USERS_LOG`, `LOGIN_LOG`, `VIDEO_LOG`, `STUDY_TASK_LOG`, `ADMIN_*_LOG` 등
  - 가능하면 **before/after 스냅샷, actor, action**을 함께 저장
- 4) 통계/파생 데이터(선택)
  - 집계/통계용 테이블 (`VIDEO_STAT_DAILY` 등)
  - 필요 시에만 갱신, 너무 복잡해지면 추후 비동기/배치로 분리 검토
- 5) 커밋
  - 위 단계들(1~4)이 모두 성공한 경우에만 `commit()`
  - 중간에 하나라도 실패하면 → **전체 롤백**  
    → 실제 데이터와 로그/통계 간 **일관성 유지**

- 예시 (USERS + USERS_LOG):

  ```rust
  pub async fn update_profile(
      state: &AppState,
      req: UpdateProfileReq,
      actor_id: i64,
  ) -> AppResult<UserMeRes> {
      let mut tx = state.db.begin().await?;

      // 1) 현재 상태 조회 (검증)
      let before = user_repo::find_user_for_update(&mut tx, req.user_id).await?;

      // 2) 메인 상태 변경
      let after = user_repo::update_user_profile(&mut tx, &req).await?;

      // 3) 로그 기록
      user_repo::insert_users_log(&mut tx, &before, &after, actor_id, "update_profile").await?;

      // 4) (필요 시) 통계/파생 데이터 갱신

      // 5) 커밋
      tx.commit().await?;

      Ok(UserMeRes::from(after))
  }

> **요약**  
> 하나의 유즈케이스 안에서 여러 DB 작업이 필요하면 **service 레이어에서 트랜잭션을 열고**,  
> **검증 → 메인 변경 → 로그 → (통계) → 커밋** 순서로 실행한다.  
> 이렇게 하면 사용자 입장에서는 “요청 한 번”이,  
> 관리/운영 입장에서는 **일관성 있는 상태 + 신뢰할 수 있는 로그**로 남는다.

### 7.6 테스트 & 자동화

> 목표: **“사람이 실수로 빼먹지 않게” 최소한의 테스트를 자동으로 돌리는 것**  
> (처음에는 가볍게 시작하고, 점진적으로 확장한다.)

#### 7.6.1 최소 정적 가드 (로컬 + CI 공통)

- 항상 돌려야 하는 기본 가드:
  - `cargo fmt -- --check`  : 포맷 일관성
  - `cargo clippy -- -D warnings` : 잠재 버그/나쁜 패턴 차단
  - `cargo check` : 타입/빌드 오류 사전 검출
- 사용 방식:
  - **로컬**: 기능 개발 후 커밋 전에 수동 실행
  - **CI**: PR 생성/업데이트 시 자동 실행 (향후 GitHub Actions 등으로 구성 예정)

#### 7.6.2 스모크 테스트 (기능 단위 확인)

- 목적:
  - “서버가 뜨고, 대표적인 API 몇 개는 정상 응답을 준다”를 빠르게 확인하기 위함.
- 대상:
  - 대표 엔드포인트
    - health: `/health`
    - auth: `/auth/login`, `/auth/refresh`
    - user: `/users`, `/users/me`
    - videos: `/videos`, `/videos/{id}`, `/videos/{id}/progress`
    - admin: `/admin/videos`, `/admin/studies` 등
- 형태:
  - `scripts/` 폴더에 cURL 기반 스모크 스크립트를 둔다.
    - 예: `scripts/smoke_health.sh`, `scripts/smoke_auth.sh`, `scripts/smoke_videos.sh`
  - 각 스크립트는 **성공 케이스 + 대표 에러 케이스 1개 정도**를 포함한다.
    - 예: 토큰 없이 `/users/me` 호출 → 401 확인
- 실행 타이밍:
  - **로컬**: 큰 변경(예: 도메인 추가, 마이그레이션 변경) 후 수동 실행
  - **CI (향후)**: main 브랜치에 머지되기 전 1회 실행을 목표로 한다.

#### 7.6.3 자동화 레벨 (초기 방침)

- 1단계: 로컬 스크립트
  - 개발자는 다음을 수동으로 실행한다.
    - `./scripts/db_fastcheck.sh` (DB 준비 상태 점검)
    - `cargo fmt -- --check`
    - `cargo clippy -- -D warnings`
    - `cargo check`
    - 필요 시 `./scripts/smoke_*.sh`
- 2단계: CI 연계 (향후)
  - PR 생성/업데이트 시:
    - `fmt` / `clippy` / `check` 자동 실행
  - main 브랜치 머지 전:
    - 최소 한 개 이상의 스모크 스크립트 실행 (예: `smoke_health.sh`, `smoke_auth.sh`)
- 3단계: 부하/성능 테스트 (K6, 향후)
  - K6 스크립트를 `scripts/k6/` 아래에 두고,
  - 주요 시나리오(로그인 + 비디오 조회 + 진도 저장 등)를 기준으로 부하 테스트를 구성한다.
  - CI/CD 파이프라인에서 주기적으로 또는 수동 트리거로 실행하는 것을 목표로 한다.

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

> 코드 예시(Best Practices)는 [`AMK_CODE_PATTERNS.md`](./AMK_CODE_PATTERNS.md) 참조

## 8. Open Questions & 설계 TODO

> 기존 `AMK_PROJECT_JOURNAL.md`의 Open Questions + Engineering Guide의 “다음 단계 로드맵”에서 정책 수준만 정리.

### 8.1 RBAC / 관리자 권한 ✅ 구현 완료 (2026-02-01)

- ~~임시 가드(모든 요청 허용)를 실제 RBAC로 교체해야 함.~~ → **완료**
- 롤별 접근 권한:
  | 역할 | Admin 접근 | 데이터 범위 | 비고 |
  |------|----------|------------|------|
  | **HYMN** | ✅ 가능 | 전체 | 모든 기능 + 시스템 설정 |
  | **admin** | ✅ 가능 | 전체 | 읽기/쓰기 모든 기능 |
  | **manager** | ❌ 불가 | 담당 class | 향후 class 기반 접근 구현 예정 |
  | **learner** | ❌ 불가 | 자신만 | 일반 사용자 |
- 구현 내역:
  - 백엔드: `src/api/admin/role_guard.rs` - 미들웨어 RBAC
  - 백엔드: Admin IP Allowlist (`admin_ip_guard.rs`)
  - 프론트: `AdminRoute` 컴포넌트 - 역할 확인 후 `/403` 리다이렉트
  - 프론트: 에러 페이지 (`/403`, `/error`, `*`)
- 향후 TODO:
  - manager 역할: class 테이블 구현 후 담당 학습자 범위 내 접근 권한 부여
  - 세분화된 권한 (예: admin이 일부 민감 기능 제한)

### 8.2 Admin action log actor 연결 ✅ 구현 완료 (2026-02-02)

- ~~`ADMIN_USERS_LOG` 및 비디오/스터디/레슨 admin 로그에 **actor user id** 채우기~~ → **완료**
  - `AuthUser` extractor에서 JWT Claims의 `sub` (user_id) 추출
  - 모든 Admin handler → service → repo까지 `actor_user_id` 전달
  - `create_audit_log()`에서 `admin_id`로 정상 저장
- 향후 검토: 역할별 로그 조회 범위 제한 (manager는 담당 class만 조회 등)

### 8.3 페이징 고도화 (Keyset vs Page)

- 현재 표준은 page/size 기반
- **트리거**: 테이블 데이터 **1만 건 이상** 시 Keyset pagination 검토
- 대상 테이블: `video_log`, `study_task_log`, `login_log`
- 기존 API와 호환성 유지 (page/size 파라미터 병행)

### 8.4 테스트 전략

**목표 성능 (K6 부하 테스트 기준)**:

| 엔드포인트 | 목표 RPS | P95 응답시간 |
|----------|---------|-------------|
| 인증 (login/refresh) | 100 | < 200ms |
| 목록 조회 (videos/studies) | 200 | < 100ms |
| 상세 조회 | 300 | < 50ms |
| 진도 저장 (progress) | 100 | < 150ms |

**대표 시나리오**: 회원가입 → 로그인 → 비디오 조회 → 시청 → 진도 저장 → 학습 문제 풀이

### 8.5 보안/운영 (후순위 계획)

**✅ 완료 항목 (2026-02-01):**
- ~~세션/리프레시 토큰 정책 강화: 역할별 TTL~~ → **완료** (HYMN: 1일, admin/manager: 7일, learner: 30일)
- ~~접근 제어: 관리자 IP allowlist~~ → **완료** (`admin_ip_guard.rs`, CIDR 지원)
- ~~RBAC 미들웨어~~ → **완료** (`role_guard.rs`, HYMN/admin만 admin 접근 허용)

**✅ 완료 항목 (2026-02-14):**
- ~~관리자 MFA 도입 (HYMN/admin 계정)~~ → **완료** (TOTP MFA, Google Authenticator, 백업 코드 10개, AdminRoute 강제 설정 가드)
- ~~토큰 재사용 탐지 (Refresh Token Replay Attack 방지)~~ → **완료** (service.rs:380-410, 409 Conflict + 전체 세션 무효화)

**📋 남은 항목** → [8.7 향후 작업 계획](#87-작업-로드맵)으로 통합됨

### 8.6 코드 일관성 (Technical Debt) ✅

> **완료됨** (2026-02-02). 모든 항목 정리 완료.

| 이슈 | 상태 | 변경 내용 |
|------|:----:|----------|
| Refresh Token 포맷 | ✅ | user/service.rs → `session_id:uuid` 포맷으로 통일 |
| LessonService 구조 | ✅ | Stateless 패턴 적용 (`struct LessonService;`) |
| Lesson 에러 타입 | ✅ | `AppResult<T>` 래핑 적용 |
| login SADD 추가 | ✅ | auth/service.rs 로그인 시 `ak:user_sessions` SADD 추가 |
| set_domain 중복 | ✅ | auth/service.rs 중복 호출 제거 |
| Handler `_handler` 접미사 | ✅ | `create_video_handler` → `admin_create_video` 등 통일 |
| Admin 함수 prefix | ✅ | `get_user_self_logs` → `admin_get_user_self_logs` 등 통일 |
| Video repo 함수명 | ✅ | `find_*` → `get_*/list_*` 패턴 통일 |

### 8.7 작업 로드맵

> 내부 DB 작업 → 외부 API 연결 순서로 진행

#### 내부 DB 작업 ✅

| 순서 | 항목 | 상태 | 설명 |
|------|------|------|------|
| 1 | Redis 인증 설정 | ✅ | `REDIS_PASSWORD` 환경변수 추가, docker-compose 수정 |
| 2 | Redis 포트 바인딩 | ✅ | 개발환경 127.0.0.1:16379로 제한 |
| 3 | 영상 실제 시청 시간 | ✅ | `video_log`에 `video_watch_duration_sec` 컬럼 추가 |
| 4 | Study 레이트리밋 | ✅ | `rl:study_submit:{user_id}` 키로 30회/분 제한 |
| 5 | Course 도메인 추가 | ✅ | `20260202_ADD_COURSE_DOMAIN.sql` 마이그레이션 생성 |
| 6 | 수강권 정책 적용 | ✅ | `lesson_access` 기반 403 Forbidden 검증 로직 (lesson/service.rs) |

#### 외부 API 연결

| 순서 | 항목 | 상태 | 설명 |
|------|------|:----:|------|
| 1-1 | Google OAuth | ✅ | Google OAuth 2.0 Authorization Code Flow 구현 완료 |
| 1-2 | Apple OAuth | 보류 | 개발 환경 및 비용 문제로 보류 |
| 2 | 이메일 발송 (Resend) | ✅ | `EmailSender` trait 추상화 + Resend 구현 (2026-02-09), `EMAIL_PROVIDER` 환경변수로 전환, 회원가입 이메일 인증 플로우 완료. 프로덕션 설정 완료 (2026-02-10): API Key (GitHub Secrets), 도메인 검증(DKIM/SPF), SES 코드 완전 제거. ~~AWS SES → 프로덕션 승인 3회 거절로 폐기~~ |
| 3 | 결제 시스템 | 📋 | Stripe, Polar 연동 (수강권과 연계) |
| 4 | RDS/ElastiCache 이전 | 📋 | EC2 → AWS RDS + ElastiCache (TLS, maxmemory 자동 적용) |
| 5 | 다중 서버 구성 (HA) | 📋 | 단계적 확장: ①nginx+컨테이너 복제(비용0) → ②ALB+EC2 다중화+RDS → ③ECS Fargate+Auto Scaling |
| 6 | GeoIP 서비스 전환 | 보류 | ip-api.com(HTTP) → MaxMind GeoLite2(로컬 DB) 또는 HTTPS 지원 서비스, 트래픽 증가 시 |
| 7 | 이메일 수신 | 검토 | `support@amazingkorean.net` 등 수신 필요 시 — Cloudflare Email Routing(무료, 개인 메일 전달) 또는 Google Workspace 검토 |

#### 보안 & 데이터 보호

| 순서 | 항목 | 상태 | 설명 |
|------|------|:----:|------|
| 1 | DB 필드 암호화 | ✅ | AES-256-GCM + Blind Index (HMAC-SHA256), Phase 1~2C 완료 (2026-02-07) |
| 2 | 암호화 모듈 구현 | ✅ | `src/crypto/` (cipher.rs, blind_index.rs, service.rs) |
| 3 | 기존 데이터 마이그레이션 | ✅ | backfill + 평문 컬럼 제거 완료 (Phase 2B~2C) |
| 4 | 키 로테이션 인프라 | ✅ | KeyRing 다중 키 지원, `src/bin/rekey_encryption.rs` (Phase 2D, 2026-02-08) |
| 5 | admin_action_log IP 암호화 | ✅ | INET→TEXT 변환 + 55+ call sites 암호화 적용 (Phase 3, 2026-02-08) |
| 6 | 프로덕션 클린 배포 | ✅ | 통합 마이그레이션 + 시드 데이터 + Dockerfile 멀티바이너리 + 암호화 검증 (2026-02-08) |

> **암호화 대상**: `user_email`, `user_name`, `user_birthday`, `user_phone`, `oauth_email`, `oauth_subject`, `login_ip`, `admin_action_log.ip_address` 등 PII
> **키 관리**: `ENCRYPTION_KEY_V{n}` (AES-256, 다중 버전) + `HMAC_KEY` (blind index), 환경변수, AppState KeyRing 로드
> **보안 로드맵**: ~~1단계 앱 레벨 AES~~ ✅ → 2단계 AWS KMS envelope → 3단계 HSM

#### 프로덕션 하드닝

| 순서 | 항목 | 상태 | 설명 |
|------|------|:----:|------|
| PROD-4 | 보안 응답 헤더 | ✅ | `X-Content-Type-Options`, `X-Frame-Options`, `X-XSS-Protection`, `Permissions-Policy` (2026-02-10) |
| PROD-5 | Health version 숨김 | ✅ | `APP_ENV=production`이면 `version` 필드 생략 (2026-02-10) |
| PROD-6 | Swagger UI 비활성화 | ✅ | `ENABLE_DOCS=false`(기본)이면 SwaggerUI 비활성화 (2026-02-10) |
| PROD-7 | Guard JSON 통일 | ✅ | `ip_guard.rs`, `role_guard.rs` plain text → `AppError` JSON (2026-02-10) |
| PROD-8 | 404 Fallback | ✅ | 존재하지 않는 라우트에 JSON `AppError::NotFound` 반환 (2026-02-10) |

#### 다국어 콘텐츠 확장

> API 엔드포인트 상세는 [5.9 Phase 9 — translation (i18n)](#59-phase-9--translation-i18n), DB 스키마는 [4.8 번역 도메인 (TRANSLATION)](#48-번역-도메인-translation) 참조

| 순서 | 항목 | 상태 | 설명 |
|------|------|:----:|------|
| 1 | 번역 테이블 설계 | ✅ | `content_translations` 테이블, 21개 언어 enum, `content_type_enum`에 `video` 추가 (Phase 1A, 2026-02-10) |
| 2 | Admin 번역 CRUD API | ✅ | 7개 엔드포인트 구현 완료, UPSERT 조건부 status 리셋 (Phase 1A, 2026-02-10) — [5.9 참조](#59-phase-9--translation-i18n) |
| 3 | 기존 콘텐츠 API `?lang=` 확장 | ✅ | courses, lessons, videos, studies에 `?lang=` 쿼리 파라미터 + fallback 주입 (Phase 1A, 2026-02-10) |
| 4 | 프론트엔드 다국어 기반 | ✅ | Pretendard 폰트, i18next 21개 언어 동적 로딩, 언어 드롭다운 UI, 관리자 번역 위저드 UI (Phase 1B, 2026-02-12) |
| 5 | RTL 지원 | 제외 | 아랍어(RTL) 제외 확정 — 지원 언어 21개 (LTR만) |
| 6 | 번역 API 연동 | ✅ | GoogleCloudTranslator 구현 완료, `TRANSLATE_PROVIDER` 환경변수로 활성화 (Phase 2, 2026-02-12) |
| 7 | 핵심 5개 언어 locale | ✅ | en, ja, zh-CN, zh-TW, vi locale 파일 생성 완료 (Phase 2, 2026-02-14) |
| 8 | 나머지 16개 언어 locale | ✅ | id, th, my, km, mn, ru, uz, kk, tg, ne, si, hi, es, pt, fr, de locale 파일 생성 완료 (Phase 3, 2026-02-14) |
| 9 | i18n 동적 로딩 + async | ✅ | Vite dynamic import + async changeLanguage 구현 완료 (Phase 1B, 2026-02-12) |

> **지원 언어 (21개, 아랍어 제외)**: en, zh-CN, zh-TW, ja, vi, id, th, my, km, mn, ru, uz, kk, tg, ne, si, hi, es, pt, fr, de
> **번역 대상**: video title/description, category name, study_task title/description, achievement (UI 메타데이터만, 학습 본문 제외)
> **Fallback**: 사용자 언어 → en → ko (한국어 원본)
> **공개 조건**: `status = 'approved'` 번역만 콘텐츠 API에서 제공
> ~~DB 확정 후 리셋해서 서버 배포 진행 필요~~ → **완료** (2026-02-08 프로덕션 클린 배포)

#### 향후 작업 계획 (우선순위 순)

> 각 섹션(8.5 보안, 8.7 외부 API 등)에 분산되어 있던 📋 항목을 통합 정리

| 순서 | 항목 | 카테고리 | 설명 | 출처 |
|:----:|------|---------|------|------|
| 1 | 결제 시스템 | 외부 API | Stripe 연동, 수강권 결제, subscriptions/payments 테이블 | 8.7 외부 API #3 |
| 2 | 동시 세션 수 제한 | 보안 | RDS 이전 후 진행 | 8.5 보안 |
| 3 | RDS/ElastiCache 이전 | 인프라 | EC2 → AWS RDS + ElastiCache (TLS, maxmemory 자동 적용) | 8.7 외부 API #4 |
| 4 | 다중 서버 구성 (HA) | 인프라 | ①nginx+컨테이너 복제 → ②ALB+EC2 다중화 → ③ECS Fargate | 8.7 외부 API #5 |

**보류/조건부 항목:**

| 항목 | 조건 | 설명 |
|------|------|------|
| Apple OAuth | 비용 | 개발 환경 및 비용 문제로 보류 |
| GeoIP 서비스 전환 | 트래픽 | ip-api.com → MaxMind GeoLite2, 트래픽 증가 시 |
| step-up MFA | 필요 시 | MFA 도입 완료, 민감한 작업 시 추가 인증 확장 |
| 이메일 수신 | 검토 | Cloudflare Email Routing 또는 Google Workspace |
| 토큰 재발급 Redis 캐싱 | 10K+ | 동시 접속자 10K+ 시 재검토 (캐시 무효화 복잡도 고려) |
| enum sqlx::Type 매핑 전환 | 결제 후 | 수동 match → `#[sqlx(type_name)]` 전환 |

#### 보류/낮음 우선순위 (기능)

| 항목 | 상태 | 설명 |
|------|:----:|------|
| 학습 문제 동적 생성/전달 | 보류 | 커리큘럼 데이터 완비 후, 사용자 요구 시 구현 |
| Lesson 통계 기능 | 보류 | `/admin/lessons/stats` — 기본 progress 데이터 있음, 추후 구현 예정 |
| Login/Login_log 테이블 개선 | ✅ | UA 서버파싱(woothee), expire_at/active_at, revoked_reason, login_log 감사 컬럼, JWT jti, geo 기본값(LC/local/none) |
| 통계 비동기/배치 분리 | 보류 | 집계/통계 복잡해지면 검토 |
| URL/함수명 통일 | ✅ | 2026-02-02 완료 — handler/service/repo 네이밍 패턴 통일 |
| OAuth repo/service 중복 통합 | 보류 | Apple OAuth 등 세 번째 인증 수단 추가 시 리팩토링 |

### 8.8 데이터 모니터링 & 접근

**현재 상태**: SSH 터널 + DB 클라이언트로 운영 데이터 접근 가능, Admin 통계 API 구현 완료

#### 9.8.1 SSH 터널 접속

```bash
# SSH 터널 → DBeaver/pgAdmin 접속
ssh -i your-key.pem -L 5433:localhost:5432 ec2-user@43.200.180.110
# Host: localhost, Port: 5433, DB: amazing_korean_db
```

#### 9.8.2 Admin 통계 API

- ✅ `/admin/users/stats`, `/admin/logins/stats`, `/admin/studies/stats`, `/admin/videos/stats`
- 🔄 시스템 상태 모니터링 (DB/Redis) — 미구현

### 8.9 디자인 & UI

**현재 상태**: shadcn/ui + Tailwind 사용, 디자인 시스템 미정립

**TODO**: 브랜딩, 타이포그래피, 반응형 점검

#### 다국어 UI 대응 (21개 언어, LTR 전용)

| 항목 | 설명 |
|------|------|
| **폰트** | Noto Sans 패밀리 동적 로딩 (Latin/Cyrillic/CJK/Thai/Myanmar/Khmer/Sinhala/Devanagari) |
| **RTL** | 아랍어(ar) 제외 확정 — 전체 LTR만 지원 |
| **텍스트 길이** | 독일어 등 60%+ 길어질 수 있음 → 고정 폭 금지, flex/grid 사용, `text-overflow: ellipsis` |
| **줄 높이** | Thai/Myanmar/Khmer/Sinhala 결합 문자 → `line-height: 1.6~1.8` |
| **레이아웃** | 모든 스크립트 공통 대응 가능한 유연한 컴포넌트 설계 |

### 8.10 마케팅 & 데이터 분석

**현재 상태**: login_log, video_log, study_task_log로 기본 데이터 수집 중

**TODO**: 사용자 세그먼트 정의, 리텐션 분석, 마케팅 자동화 연동

### 8.11 한국어 발음 교정 AI (Pronunciation Coaching AI)

**현재 상태**: 설계 단계 (2026-02-16)

**문제 정의**: 한국어 학습자의 발음 교정은 1:1 원어민 교사 없이는 사실상 불가능하다. 한국어는 비성조 언어로 발화 시 피치가 일정한 특성이 있으나, 학습자는 모국어의 음성적 특성(영어 강세, 중국어 성조, 일본어 고저 악센트)을 한국어에 투영한다. 기존 서비스는 "맞았다/틀렸다" 수준의 피드백만 제공하며, "왜 틀렸고 어떻게 고쳐야 하는지"를 기술적으로 제공하는 솔루션은 없다.

#### 4대 핵심 기능

**① 발음 인식 및 음소 단위 평가**

| 항목 | 내용 |
|------|------|
| **목표** | 학습자 발음을 음소(phoneme) 단위로 인식하고, 기준 발음과 비교하여 교정 피드백 제공 |
| **기술 후보** | SpeechSuper API (한국어 음소 평가 지원), Whisper 파인튜닝 (비원어민 한국어 PER 3.22%), WhisperKit (온디바이스) |
| **출력** | 음소별 정확도 점수, 오류 음소 식별, 오류 유형 (대치/삽입/탈락) |
| **온디바이스** | WhisperKit/Apple Speech (기본 STT) → 오프라인 가능. 정밀 평가는 서버 API |

**② 발화 피치 분석 (F0 기본 주파수)**

| 항목 | 내용 |
|------|------|
| **목표** | 한국어 표준 발화의 평탄한 피치 패턴을 기준으로, 학습자의 모국어 피치 간섭을 감지하고 교정 |
| **기술 후보** | CREPE (신경망 F0 추출, 10ms 단위, SOTA), librosa pYIN (경량 F0 추출), ProsodyAI (프로소디 분석 API) |
| **출력** | 기준 F0 컨투어 vs 사용자 F0 컨투어 오버레이, 피치 편차 구간 식별, 시각적 피드백 |
| **온디바이스** | CREPE (TF Lite) 또는 pYIN → 완전 온디바이스 가능 (모델 수 MB, CPU 실시간 처리) |
| **핵심 차별점** | 시장에 한국어 발화 피치 분석을 제공하는 학습 서비스가 없음 |

**③ 조음 가이드 (입모양 + 혀위치)**

| 항목 | 내용 |
|------|------|
| **목표** | 음소 오류 감지 시 해당 음소의 올바른 입모양/혀위치를 시각적으로 안내 (카메라 불필요) |
| **원리** | 한글 자음은 발음 기관의 모양을 본떠 설계됨 → 각 음소의 조음 위치가 음성학적으로 확정 → 음소 오류 유형에서 교정 방법이 결정론적으로 매핑됨 |
| **구현** | 조음 데이터베이스 (19자음+21모음, 조음 위치/입모양/혀위치 정의) + SVG/Lottie 다이어그램 + 모국어별 공통 오류 패턴 사전 |
| **출력** | 입 단면도 다이어그램, 혀위치 애니메이션, 자연어 교정 지침 (BitNet 보조) |
| **온디바이스** | 정적 데이터 (룩업 테이블) → 완전 오프라인 |

**④ 단음절 정밀 발음 (기존 TTS의 한계 극복)**

| 항목 | 내용 |
|------|------|
| **문제** | 기존 TTS는 "가", "나", "다" 같은 단음절을 명확하게 발음하지 못함 (문맥 없이 어색하거나 너무 짧게 끊김) |
| **해결** | 하이브리드 접근: 전문 성우 녹음 (핵심 음절 ~2,000개 x 3속도 = ~6,000 파일, ~300MB) + 동일 화자 기반 커스텀 TTS (단어/문장 수준) |
| **기능** | 음절 분해 재생 ("안녕하세요" → "안"+"녕"+"하"+"세"+"요" 개별 재생), 속도 조절 (보통/느림/강조) |
| **온디바이스** | 녹음 파일 앱 번들 포함 → 완전 오프라인 |

#### 기술 스택 요약

| 기능 | 클라우드 API | 온디바이스 | 비고 |
|------|------------|-----------|------|
| 음소 평가 | SpeechSuper (정밀) | WhisperKit / Apple Speech (기본) | 하이브리드 |
| 피치 분석 | ProsodyAI | CREPE / pYIN (완전 로컬) | 온디바이스 주력 |
| 조음 가이드 | - | 룩업 테이블 + SVG (완전 로컬) | 온디바이스 전용 |
| 단음절 발음 | - | 전문 녹음 파일 (완전 로컬) | 온디바이스 전용 |

#### 통합 사용자 흐름

```
사용자가 "안녕하세요"를 발음
  ├─ ① 음소 인식 → "ㄴ 발음이 ㄷ에 가까움" (오류 식별)
  ├─ ② 피치 분석 → "두 번째 음절에서 피치 상승 감지" (F0 비교)
  ├─ ③ 조음 가이드 → "혀끝을 윗잇몸에 대세요 (ㄴ)" (교정 지침)
  └─ ④ 모범 발음 → "ㄴ" 단독 재생 → "안" 재생 → 전체 재생
      → 다시 따라하기 → ①로 반복
```

#### 준비 데이터

| 데이터 | 내용 | 예상 규모 |
|--------|------|----------|
| 표준 발음 F0 프로필 | 서울 표준어 화자 음소/음절별 F0 범위 | 측정 + DB 구축 |
| 조음 데이터베이스 | 19자음+21모음 입모양/혀위치/입술모양 정의 | 정적 데이터 (1회 구축) |
| 모국어별 오류 패턴 | 영어/중국어/일본어/베트남어 화자 공통 오류 | 음성학 문헌 기반 |
| 전문 성우 녹음 | 핵심 음절 ~2,000개 x 3속도 | ~300MB (오디오 파일) |
| 비원어민 발화 데이터 | Whisper 파인튜닝용 학습 데이터 | AIHub 공개 데이터 활용 |

> **오케스트레이션**: 이 기능의 멀티 AI 개발 전략은 [`AMK_PIPELINE.md §11.9`](./AMK_PIPELINE.md#119-한국어-발음-교정-ai-오케스트레이션) 참조

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 9. 변경 이력 (요약)

- **2026-02-16 — 결제 시스템 (Paddle Billing) 전체 구현 + 프로덕션 배포**
  - **데이터 모델**: Section 4.9 결제 도메인 추가 — 4 ENUMs + 3 Tables (subscriptions, transactions, webhook_events)
  - **외부 서비스**: Section 2.4.5 Paddle Billing 연동 추가
  - **Phase 11** (사용자 결제): `GET /payment/plans` (공개), `GET /payment/subscription` (인증), `POST /payment/webhook` (Paddle)
  - **Phase 10** (관리자 결제): 구독 CRUD 6개 + 수동 수강권 3개 = 총 9개 엔드포인트
  - **Webhook**: 8 subscription + 1 transaction 이벤트 처리, HMAC-SHA256 서명 검증, 멱등성 보장
  - **user_course 연동**: 구독 활성화 시 수강권 자동 부여, 취소/일시정지 시 자동 회수
  - **프론트엔드**: Pricing 페이지 (Paddle.js overlay checkout), 프로모 코드 입력, 관리자 결제 관리 UI
  - **프로덕션 배포**: DB 마이그레이션 + Paddle Sandbox Webhook 연동 완료

- **2026-02-15 — 문서 정리 (코드-문서 동기화)**
  - Section 8.7 다국어 콘텐츠 확장: 항목 4,6,7,8,9 📋→✅ (Phase 1B/2/3 완료 반영)
  - Section 8.7 "향후 작업 계획" 통합 섹션 추가: 8.5 보안, 8.7 외부 API 분산 📋 항목을 한 곳으로 정리
  - Section 8.5 남은 항목 → 8.7 향후 작업 계획 참조로 통합
  - Section 8.9 다국어 UI 대응: 22개 언어 → 21개 언어 (아랍어 RTL 제외 확정 반영)

- **2026-02-14 — Admin MFA (TOTP 2단계 인증) 구현 + QA 완료**
  - **백엔드 (Rust/Axum)**
    - DB 마이그레이션: `users` 테이블에 MFA 컬럼 4개 추가 (`user_mfa_secret`, `user_mfa_enabled`, `user_mfa_backup_codes`, `user_mfa_enabled_at`)
    - `Cargo.toml`: `totp-rs = { version = "5", features = ["qr", "gen_secret"] }` 의존성 추가
    - `src/api/auth/dto.rs`: MFA DTO 7개 (MfaChallengeRes, MfaLoginReq, MfaSetupRes, MfaVerifySetupReq, MfaVerifySetupRes, MfaDisableReq, MfaDisableRes)
    - `src/api/auth/repo.rs`: `UserLoginInfo`에 `user_mfa_enabled` 추가 + MFA repo 함수 7개
    - `src/api/auth/service.rs`: `LoginOutcome`/`OAuthLoginOutcome` enum, `login()`/`google_auth_callback()` MFA 분기, MFA 메서드 4개 (mfa_setup, mfa_verify_setup, mfa_login, mfa_disable)
    - `src/api/auth/handler.rs`: MFA 핸들러 4개 + login/OAuth 핸들러 반환 타입 변경 (`impl IntoResponse`)
    - `src/api/auth/router.rs`: `/mfa/setup`, `/mfa/verify-setup`, `/mfa/login`, `/mfa/disable` 라우트 추가
    - `src/config.rs`: MFA 환경변수 3개 (MFA_TOKEN_TTL_SEC=300, RATE_LIMIT_MFA_MAX=5, RATE_LIMIT_MFA_WINDOW_SEC=300)
    - `src/api/user/dto.rs` + `repo.rs`: `ProfileRes`에 `mfa_enabled: bool` 추가
    - `src/docs.rs`: MFA 핸들러 4개 + DTO 7개 Swagger 등록
  - **프론트엔드 (React/TypeScript)**
    - `auth/types.ts`: MfaChallengeRes, MfaLoginReq(zod), MfaSetupRes, MfaVerifySetupRes
    - `auth/auth_api.ts`: mfaLogin, mfaSetup, mfaVerifySetup API 함수
    - `auth/hook/use_login.ts`: MFA 챌린지 감지 (`isMfaChallenge` 타입가드) + `mfaPending` 상태
    - `auth/hook/use_oauth_callback.ts`: OAuth MFA 리다이렉트 파라미터 처리
    - `auth/page/login_page.tsx`: MFA 코드 입력 UI (6~8자 TOTP/백업코드)
    - `admin/page/admin_mfa_setup_page.tsx`: 3단계 위저드 (QR스캔→코드확인→백업코드)
    - `routes/admin_route.tsx`: MFA 강제 설정 가드 (`!mfa_enabled` → `/admin/mfa/setup`)
    - `app/routes.tsx`: `/admin/mfa/setup` 라우트 추가 (AdminLayout 밖, AdminRoute 안)
    - `user/types.ts`: `mfa_enabled: z.boolean().optional()` 추가
    - i18n: MFA 관련 키 추가 (ko.json, en.json + 20개 언어)
  - **보안**
    - TOTP 비밀키: AES-256-GCM 암호화 (AAD: `users.user_mfa_secret`)
    - 백업 코드: SHA-256 해시 → JSON → AES-256-GCM 암호화
    - MFA 토큰: Redis UUID (5분 TTL, 일회용)
    - Rate Limit: `rl:mfa:{user_id}:{ip}` (5회/5분)
    - MFA 비활성화: HYMN 전용, 자기 자신 비활성화 불가, 대상 전체 세션 무효화
  - **QA (39/39 PASS)**
    - H-1 수정: `login_method: "login"` → `"email"` (login_method_enum 불일치)
    - M-1 수정: docs.rs에 MFA 핸들러/스키마 Swagger 등록 누락
  - **프로덕션 배포 완료** (2026-02-14)
    - DB 마이그레이션 수동 실행 (EC2 SSH → psql)
    - Admin/HYMN MFA 설정 정상 작동 확인

- **2026-02-10 — Phase 1A 다국어 인프라 + QA 수정 + 프로덕션 QA**
  - **Phase 1A 다국어 인프라 (백엔드)**
    - `content_translations` 테이블 + 21개 언어 enum (`SupportedLanguage`) 구현
    - Admin 번역 CRUD API 7개 엔드포인트 (목록/생성UPSERT/벌크/상세/수정/상태변경/삭제)
    - 기존 콘텐츠 API `?lang=` 확장: courses, lessons, videos, studies에 번역 fallback 주입
    - Fallback 순서: 사용자 언어 → en → ko (서비스 계층 post-fetch merge)
  - **Phase 1A QA 수정 (10개 이슈)**
    - H-1: Course `GET /courses/{id}` 번역 지원 — handler→service 리팩토링, `?lang=` 파라미터 추가
    - H-2: `ContentType::Video` 추가 — video title/subtitle 번역과 video_tag 번역 의미 분리, migration 추가
    - M-1: `CourseListItem`에 `course_subtitle` 필드 추가 + 번역 주입
    - M-2: Course DTO OpenAPI 스키마 등록 (`IntoParams`, `ToSchema` derive)
    - M-3: UPSERT 정책 개선 — 텍스트 변경 시에만 `status='draft'` 리셋 (SQL CASE 조건)
    - L-1~L-5: `CourseListQuery` derive 추가, Video DTO import 정리
  - **프로덕션 QA 수정 (PROD-4 ~ PROD-8)**
    - PROD-4: API 보안 헤더 미들웨어 추가 (`main.rs`) — `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `X-XSS-Protection: 0`, `Permissions-Policy: camera=(), microphone=(), geolocation=()`
    - PROD-5: Health `version` 필드 프로덕션 숨김 — `Option<String>` + `skip_serializing_if`, `APP_ENV=production`이면 None
    - PROD-6: OpenAPI Swagger UI 프로덕션 비활성화 — `enable_docs` config에 따라 조건부 merge
    - PROD-7: Guard 401/403 JSON 통일 — `ip_guard.rs`, `role_guard.rs` plain text → `AppError::Forbidden/Unauthorized` JSON 응답
    - PROD-8: 404 Fallback 핸들러 추가 — 존재하지 않는 라우트에 JSON `AppError::NotFound` 반환
  - **파일 변경 목록**
    - `src/main.rs` — `security_headers` 미들웨어 함수 추가 + 레이어 적용
    - `src/api/mod.rs` — 조건부 SwaggerUi merge + `fallback_404` 핸들러
    - `src/api/health/handler.rs`, `dto.rs` — version `Option<String>`, 프로덕션 숨김
    - `src/api/admin/ip_guard.rs` — `AppError::Forbidden` JSON 응답
    - `src/api/admin/role_guard.rs` — `AppError::Unauthorized/Forbidden` JSON 응답
    - `src/api/course/` — dto.rs, repo.rs, service.rs, handler.rs (H-1, M-1, M-2, L-1)
    - `src/api/video/service.rs` — `ContentType::Video` 적용 (H-2)
    - `src/api/video/dto.rs` — import 정리 (L-5)
    - `src/types.rs` — `ContentType::Video` 추가 (H-2)
    - `src/api/admin/translation/repo.rs` — UPSERT 조건부 status 리셋 (M-3)
    - `src/docs.rs` — Course DTO 스키마 등록 (M-2)
    - `migrations/20260210_i18n_add_video_content_type.sql` — 신규

- **2026-02-09 — 이메일 인증 + 계정 복구 + Rate Limiting 강화**
  - **이메일 인증 시스템**
    - 회원가입 → 인증코드 발송 → 검증 → 로그인 가능 플로우 구현
    - `POST /auth/verify-email` (3-7): HMAC-SHA256 해시 비교, `user_check_email=true` 업데이트
    - `POST /auth/resend-verification` (3-8): Enumeration Safe, 잔여 횟수 반환
    - 로그인 시 `user_check_email=false` → **403** 차단 (`AUTH_403_EMAIL_NOT_VERIFIED:email`)
    - OAuth 자동 인증: 미인증 이메일로 OAuth 로그인 시 `user_check_email=true` 자동 업데이트
    - Redis 저장: HMAC-SHA256 해시 (평문 코드 저장 금지), TTL 10분
    - 프로덕션 fail-fast: `EMAIL_PROVIDER=none` + `APP_ENV=production` → 서버 부팅 실패
    - EmailSender trait: Resend (`src/external/email.rs`)
  - **계정 복구 (아이디/비밀번호 찾기) 통합**
    - `POST /auth/find-password` (3-9): 본인확인(이름+생일+이메일) → 인증코드 발송
    - `/account-recovery` 페이지: 탭 UI (아이디 찾기 / 비밀번호 찾기)
    - OAuth 전용 계정 경고 문구 (warning 스타일, 비밀번호 찾기 탭)
  - **Rate Limiting 강화**
    - 이메일 발송 제한: 5회/1시간 → 5회/5시간 (환경변수 조정 가능)
    - 환경변수: `RATE_LIMIT_EMAIL_WINDOW_SEC` (기본 18000초), `RATE_LIMIT_EMAIL_MAX` (기본 5)
    - 응답에 `remaining_attempts` 필드 추가 (FindPasswordRes, RequestResetRes, ResendVerificationRes)
    - 프론트: 잔여 발송 횟수 표시 + 한도 도달 시 재전송 버튼 비활성화
  - **프론트엔드 변경**
    - `verify_email_page.tsx` 신규 — 이메일 인증코드 확인 페이지
    - `account_recovery_page.tsx` 신규 — 아이디/비밀번호 찾기 통합 (Tabs)
    - `signup_page.tsx` — 가입 성공 시 `/verify-email`로 이동
    - `use_login.ts` — 403 이메일 미인증 시 `/verify-email`로 이동
    - i18n: 이메일 인증, 계정 복구, Rate Limiting 관련 키 추가 (ko.json, en.json)

- **2026-02-08 — 프로덕션 클린 배포 (DB 보안 Phase 2D+3 반영)**
  - **마이그레이션 통합**
    - 기존 11개 마이그레이션 파일 → 단일 `20260208_AMK_V1.sql` 통합 (22 ENUMs, 35 Tables, FKs, Indexes)
    - 암호화 컬럼 직접 포함 (`user_email` TEXT, `user_email_idx` TEXT 등), `ip_address` INET→TEXT 반영
  - **시드 데이터**
    - `20260208_AMK_V1_SEED.sql` 생성 (콘텐츠 10개 테이블, ~200행)
    - 컬럼 순서 불일치 수정: `lesson`, `video`, `study` 테이블에 명시적 컬럼명 추가
  - **Dockerfile 수정**
    - 멀티바이너리 빌드 지원 (`amazing-korean-api` + `rekey_encryption`)
    - `--bin` 플래그로 개별 바이너리 빌드
  - **docker-compose.prod.yml 환경변수 추가**
    - `ENCRYPTION_KEY_V1`, `ENCRYPTION_CURRENT_VERSION`, `HMAC_KEY`, `APP_ENV`
    - `GOOGLE_CLIENT_ID/SECRET`, `GOOGLE_REDIRECT_URI`, `OAUTH_STATE_TTL_SEC`
    - `FRONTEND_URL`, `ADMIN_IP_ALLOWLIST`
  - **EC2 배포 완료**
    - DB 볼륨 삭제 → 스키마 마이그레이션 → 시드 데이터 투입 → 전체 서비스 시작
    - `.env.prod` 완전 구성 (프로덕션 전용 암호화 키 생성)
    - Google OAuth redirect URI 프로덕션 설정 (`https://api.amazingkorean.net/auth/google/callback`)
  - **배포 검증 완료**
    - healthz: `{"status":"live","version":"v1.0.0"}`
    - DB 암호화 확인: `user_email` = `enc:v1:...` 형태 정상 저장
    - 시드 데이터: video=16, lesson=8 정상
  - **문서 업데이트**
    - Section 8.7: 프로덕션 클린 배포 항목 추가, 이메일 인증 상태 변경 (📋→보류)
    - `AMK_DEPLOY_OPS.md`: .env.prod 전체 변수 목록, 클린 배포 절차, 트러블슈팅 추가

- **2026-02-08 — 문서 구조 재편 (3파일 분할 + 불일치 수정)**
  - **구조 변경**
    - `AMK_API_MASTER.md` 단일 파일(8,100줄) → 3파일 분할(MASTER ~3,700줄 + CODE_PATTERNS ~4,000줄 + DEPLOY_OPS ~620줄)
    - `AMK_CODE_PATTERNS.md` 신규 — 기존 Section 7.7 코드 예시 전체 이동
    - `AMK_DEPLOY_OPS.md` 신규 — 기존 Section 6.6.2~6.6.4 배포/운영 가이드 + Phase 8 운영 도구 통합
    - `docs/patchs/` → `docs/archive/patchs/` 아카이브 이동
  - **삭제 항목**
    - Section 0.4 (웹 LLM 협업 가이드 90줄) → 5줄 AI 에이전트 규칙으로 대체
    - Section 8 (LLM 협업 규칙 74줄) 전체 삭제
    - Phase 8 (scripts 테이블) 삭제 → Course Phase로 대체
  - **불일치 수정 23건 (Section 2~5)**
    - Section 2: `src/api/docs.rs` → `src/docs.rs`, 암호화 모듈 추가, EmailTemplate 4종, Vimeo 경로 명시
    - Section 3: 액세스 토큰 TTL 1시간 → 15분, 리프레시 토큰 역할별 분리 명시
    - Section 4: 암호화 컬럼(`_enc`, `_idx`) 반영, `ip_address` INET→TEXT, Course 도메인 추가, `user_oauth` 테이블 추가
    - Section 5: Auth 라우트 3개 추가, Course 엔드포인트 3개 추가, Admin email/stats 엔드포인트 추가
  - **섹션 번호 재구성**
    - Section 9 (Open Questions) → Section 8
    - Section 10 (변경 이력) → Section 9
    - Section 6.6 "빌드/배포" → "로컬 개발" (배포 내용 DEPLOY_OPS 이관)
  - **기타**
    - Section 7.2 개발 플로우: Gemini 템플릿 단계 제거, CODE_PATTERNS 참조 추가
    - Section 0.3 관련 파일 목록 갱신 (CODE_PATTERNS, DEPLOY_OPS 추가)
    - 교차 참조 정리 (분할 파일 참조 업데이트)
    - 목차(TOC) 전면 갱신 + 앵커 링크 검증

- **2026-02-06 — Gemini 코드 리뷰 반영**
  - **백엔드 — 코드 수정 (8건)**
    - `google.rs`: ID Token 서명 검증을 Google JWKS 공개키 기반으로 변경 (RS256, kid 매칭)
    - `ipgeo.rs`: `lookup()` 반환 타입 `Option<GeoLocation>` → `GeoLocation`, `is_private_ip()`를 `std::net::IpAddr` 파싱으로 개선
    - `auth/service.rs`: 이메일 미설정 시 `AppError::ServiceUnavailable` 반환, 인라인 Argon2 해싱 → `password::hash_password()` 통합, 실패 로깅 `let _ =` → `if let Err(e)` + `warn!`
    - `admin/upgrade/service.rs`: 로컬 `hash_password()` 제거 → `password::hash_password()` 사용, 이메일 미설정 시 `ServiceUnavailable` 반환
    - `lesson/repo.rs`: DB 에러 `.unwrap_or(false)` → `?` 전파
    - `user/service.rs`: ipgeo `.unwrap_or_default()` 제거
  - **문서 정리**
    - Section 8.5/9.7에 추후 작업 항목 5건 추가 (토큰 캐싱, GeoIP 전환, i18n async, OAuth 중복 통합, enum 매핑)
    - 불일치 문서 4건 삭제: `AMK_BACKEND_STATUS.md`, `AMK_FRONTEND_STATUS.md`, `homepage_layout_design.md`, `login_table_plan.md`
    - `.gitignore`에 `.aws/` 추가
    - Section 5.3-1 소셜 전용 계정 에러 응답 형식 수정

- **2026-02-05 — Login/Login_log 테이블 개선**
  - **백엔드 — User-Agent 서버사이드 파싱**
    - `woothee` 라이브러리 추가, `ParsedUa` 구조체 및 `parse_user_agent()` 함수 구현
    - `login_os`, `login_browser`, `login_device`를 서버에서 자동 채움 (프론트엔드 전송 제거)
    - OAuth/일반 로그인/회원가입 모두 동일하게 처리
  - **백엔드 — login 테이블 컬럼 활성화**
    - `login_expire_at`: `NOW() + refresh_ttl_secs` 기록, 토큰 갱신 시 갱신
    - `login_active_at`: 토큰 갱신(refresh) 시 `NOW()` 업데이트
    - `login_revoked_reason`: 상태 변경 시 사유 기록 (기본값 `none`, revoke 시 `password_changed`/`security_concern` 등)
  - **백엔드 — login_log 테이블 감사 컬럼 활성화**
    - `login_access_log`: access token SHA-256 해시 (64자)
    - `login_token_id_log`: JWT `jti` claim (UUID v4)
    - `login_fail_reason_log`: 실패 사유 (기본값 `none`, 실패 시 `invalid_credentials`/`account_disabled`/`token_reuse`)
    - `login_expire_at_log`: 세션 만료 시각 기록
    - login_log geo 컬럼에 COALESCE 기본값 추가 (`LC`/`0`/`local`)
  - **백엔드 — JWT jti claim 추가**
    - `jwt::create_token()`에서 UUID v4 기반 `jti` 생성, `Claims` 구조체에 `jti` 필드 추가
  - **백엔드 — Geo/NULL 기본값 정책 변경**
    - Private IP 기본값: `ZZ`→`LC`, `Unknown`→`local` (login/login_log 모든 COALESCE)
    - `login_revoked_reason` NULL→`none`, `login_fail_reason_log` NULL→`none`
  - **프론트엔드 — 버그 수정**
    - `client.ts`: request interceptor 추가 (zustand → axios Authorization 헤더 자동 설정)
    - `use_user_settings.ts`: `enabled` 옵션 + `staleTime: 5분` 추가 (미로그인 시 401 루프 방지)
    - `use_language_sync.ts`: `{ enabled: isLoggedIn }` 전달
    - `types.ts`: `LoginReq`에서 불필요 필드(`device`/`browser`/`os`) 제거
  - **파일 변경 목록**
    - `Cargo.toml` — `woothee` 의존성 추가
    - `src/api/auth/handler.rs` — `ParsedUa`, `parse_user_agent()` 추가
    - `src/api/auth/dto.rs` — `LoginReq` 간소화
    - `src/api/auth/jwt.rs` — `jti` claim 추가
    - `src/api/auth/repo.rs` — INSERT/UPDATE 쿼리에 신규 컬럼 반영, COALESCE 기본값 변경
    - `src/api/auth/service.rs` — UA/geo/audit 파라미터 전달, revoked_reason/fail_reason 기본값
    - `src/api/user/handler.rs` — UA 파싱 호출
    - `src/api/user/service.rs` — 회원가입 로그에 audit 파라미터 추가
    - `frontend/src/api/client.ts` — request interceptor 추가
    - `frontend/src/category/auth/types.ts` — LoginReq 필드 제거
    - `frontend/src/category/user/hook/use_user_settings.ts` — enabled/staleTime 추가
    - `frontend/src/hooks/use_language_sync.ts` — enabled 조건 추가

- **2026-02-05 — DB 보안 강화 계획 수립**
  - 애플리케이션 레벨 AES-256-GCM 암호화 방식 결정 (pgcrypto, AWS KMS 비교 후)
  - 암호화 대상 필드 식별: `user_email`, `user_name`, `user_birthday`, `oauth_email`, `oauth_subject`, `login_ip` 등
  - Blind Index (HMAC-SHA256) 설계: 검색 필요 필드(email, oauth_subject)는 같은 테이블에 `_idx` 컬럼 추가
  - 키 관리: `ENCRYPTION_KEY` + `HMAC_KEY` (환경변수, 각 32바이트)
  - 마이그레이션 전략: 3단계 점진적 (호환 모드 → 일괄 암호화 → 정리)
  - 보안 로드맵: 1단계 앱 레벨 AES → 2단계 AWS KMS → 3단계 HSM
  - Section 8.7 로드맵에 "보안 & 데이터 보호" 섹션 추가

- **2026-02-05 — 다국어 콘텐츠 확장 계획 수립**
  - 22개 언어 지원 계획: en, zh-CN, zh-TW, ja, vi, id, th, my, km, mn, ru, uz, kk, tg, ne, si, hi, es, pt, fr, de, ar
  - `content_translations` 번역 테이블 설계 (정규화, fallback 패턴)
  - 폰트 전략: Noto Sans 패밀리 동적 로딩 (50MB+ → 언어별 선택 로드)
  - RTL 대응 (아랍어): CSS Logical Properties, direction: rtl
  - 번역 파이프라인: AI 자동 초안 → 관리자 검수 → 승인
  - 단계적 접근: Phase 1 기반 → Phase 2 핵심 5개(en,ja,zh-CN,zh-TW,vi) → Phase 3 나머지 17개
  - Section 8.7 로드맵에 "다국어 콘텐츠 확장" 섹션 추가, Section 8.9에 다국어 UI 대응 추가

- **2026-02-05 — 다국어 지원 (i18n) 구현**
  - 상세: Section 6.2.4 참조

- **2026-02-03 — MyPage UI 리디자인 & 비밀번호 재설정 플로우**
  - **백엔드**
    - `ProfileRes`에 `has_password: bool` 필드 추가 (OAuth 전용 계정 구분)
    - `GET /users/me`, `POST /users/me` 응답에 `has_password` 포함
  - **프론트엔드**
    - MyPage UI 리디자인
      - 프로필 헤더: 닉네임 + user_auth 뱃지만 표시
      - 보기 모드 필드 순서: 닉네임 → 이름 → 이메일 → 가입일 → 생년월일 → 언어 → 국가 → 성별
      - 환경 설정 버튼을 수정 버튼 옆으로 이동
      - 비밀번호 재설정 버튼 추가 (OAuth 전용 계정은 숨김)
    - `/request-reset-password` 페이지 생성 (PrivateRoute 보호)
      - 로그인 사용자 이메일 자동 채우기
      - OAuth 전용 계정 접근 시 마이페이지로 리다이렉트
      - 이메일 입력 → 인증번호 전송 → 인증번호 확인 UI (백엔드 API 연동 대기)
    - 환경 설정 페이지에 마이페이지 돌아가기 링크 추가
    - `UserDetail` 타입에 `has_password: boolean` 추가
  - **문서**
    - Section 7.7.1-1 ProfileRes 코드 예시 업데이트

- **2026-02-03 — Google OAuth 소셜 로그인 구현**
  - **백엔드**
    - `GET /auth/google` — OAuth 시작 (auth_url 반환)
    - `GET /auth/google/callback` — OAuth 콜백 처리 (토큰 발급, 프론트엔드 리다이렉트)
    - `src/external/google.rs` — Google OAuth 클라이언트 구현
    - `migrations/20260203_ADD_OAUTH_SUPPORT.sql` — `user_oauth` 테이블 추가, `users.user_password` NULL 허용
  - **프론트엔드**
    - 로그인 페이지에 "Google로 로그인" 버튼 추가
    - `use_google_login.ts` 훅 생성
    - OAuth 콜백 처리 (refreshToken 호출 → 스토어 업데이트)
  - **문서**
    - Section 5.3 Phase 3 auth에 3-6 Google OAuth 엔드포인트 추가
    - Section 8.7 외부 API 연결 로드맵 업데이트

- **2025-11-18**
  - `AMK_Feature_Roadmap.md`, `AMK_PROJECT_JOURNAL.md`, `AMK_ENGINEERING_GUIDE.md`, `AMK_API_OVERVIEW_FULL.md`, `README_for_assistant.md`의 핵심 내용을 통합.
  - 이 문서(`AMK_API_MASTER.md`)를 프로젝트의 단일 기준 문서로 지정.
- **2026-01-21**
  - Section 0.4 "LLM 협업 가이드" 추가 (LLM 활용 프롬프트 템플릿 및 참조 방법)
  - Section 3.7 "인증 & 세션 관리 (통합)" 추가 (산재된 인증 관련 내용 통합)
  - Section 5.0 "Phase 로드맵 체크박스 범례" 추가 (✅🆗⚠️❌🔄 의미 명확화)
  - 문서 전체 목차(TOC) 추가 및 양방향 링크 구현 (각 섹션 시작/끝에 "목차로 돌아가기" 링크)
  - 외부 파일 참조 링크 업데이트 (AMK_SCHEMA_PATCHED.md, LLM_PATCHS_TEMPLATE_*.md)
- **2026-01-22**
  - Section 7.7.2 "프론트엔드 패턴" 실제 코드 기반으로 전면 재작성 (기존 LLM 분석 내용 제거)
  - Section 5 Phase 번호 체계 정리 (5.3 video → 5.4, 5.4 study → 5.5, 5.5 lesson → 5.6, 5.5.6 admin → 5.7, 5.7 scripts → 5.8)
  - 목차(TOC) 실제 섹션 헤딩과 동기화 (Section 6, 7, 8, 9 하위 항목 추가)
  - Section 8.6 "코드 일관성 (Technical Debt)" 추가
  - Section 8.7 "추후 작업 항목 (문서 내 TODO 통합)" 추가
- **2026-01-28 — Vimeo API 연동 & Admin Video 문서화**
  - **Vimeo API 연동 (Phase 5 & 6 계획 기반)**
    - `GET /admin/videos/vimeo/preview` — Vimeo 메타데이터 미리보기 (7-10)
    - `POST /admin/videos/vimeo/upload-ticket` — Vimeo tus 업로드 티켓 생성 (7-11)
    - `video` 테이블에 `video_duration`, `video_thumbnail` 컬럼 추가
  - **Admin Video 엔드포인트 정비**
    - `GET /admin/videos/{id}` 상세 조회 추가 (7-9)
    - Phase 7 엔드포인트 번호 재정렬 (7-8 ~ 7-57, 이후 Study Stats 추가로 7-67까지 확장)
  - **문서 업데이트**
    - Section 4.3 비디오 도메인에 신규 컬럼 명세 추가
    - Section 5.4 Phase 4 video에 응답 스키마 상세 추가 (VideoListItem, VideoDetailRes, VideoProgressRes)
    - Section 5.7 Phase 7 admin video 엔드포인트 목록 갱신
- **2026-01-26 — v1.0.0 MVP 릴리스**
  - **MVP 배포 완료**
    - Frontend: Cloudflare Pages (`amazingkorean.net`)
    - Backend: AWS EC2 (`api.amazingkorean.net`)
    - SSL: Cloudflare Flexible 모드
  - **GitHub Actions CI/CD 파이프라인 구축**
    - Section 6.6.2-3 "GitHub Actions CI/CD 파이프라인" 추가
    - EC2에서 빌드 불필요 → t2.micro 유지 가능
    - `git push`만으로 자동 배포
  - **배포 최적화**
    - `.dockerignore` 추가 (docs, frontend, .git 등 제외)
    - `docker-compose.prod.yml` Docker Hub 이미지 사용으로 변경
    - Section 6.6.2-4 "EC2 유지보수 가이드" 추가
  - **버전 관리**: Cargo.toml `version = "1.0.0"`, Git tag `v1.0.0` 생성
  - **Section 9 확장** (Open Questions & 설계 TODO)
    - Section 8.8 "LLM 협업 도구 전환" 추가 (Patch 템플릿 처리 + GitHub Gemini)
    - Section 8.9 "인프라 로드맵 (RDS 이전)" 추가 (이전 순서 및 시점 기준)
    - Section 8.10 "데이터 모니터링 & 접근" 추가 (SSH 터널, Admin 대시보드, 동기화)
    - 이후 변경 사항은 커밋 메시지 `docs: update AMK_API_MASTER <요약>` 형식으로 관리하고, 필요 시 이 섹션에 중요한 방향 전환만 추가한다.
- **2026-01-28 — User/Login Stats & TODO 정비**
  - **User/Login Stats 구현 (현재 7-63 ~ 7-67로 재번호)**
    - `GET /admin/users/stats/summary` — 역할별(HYMN/admin/manager/learner) 통계로 변경
    - `GET /admin/users/stats/signups` — 역할별 일별 가입 통계
    - `GET /admin/logins/stats/summary` — 로그인 성공/실패/고유사용자/활성세션
    - `GET /admin/logins/stats/daily` — 일별 로그인 통계
    - `GET /admin/logins/stats/devices` — 디바이스별 통계
  - **버그 수정**
    - Video 상세 조회 시 `video_state = 'open'` 필터 추가 (비공개 영상 직접 접근 차단)
  - **Section 9 TODO 업데이트**
    - Section 8.2 로그 테이블 역할별 구분 항목 추가
    - Section 8.7 기능 개발에 Admin 폼 검증, 영상 시청 시간, 토픽 정답 검사, 학습 문제 생성 추가
    - Section 8.11.2 에러 페이지 항목 추가
    - Section 8.12 "마케팅 & 데이터 분석" 신규 추가
- **2026-01-29 — Admin Study Stats & Phase 7 정비**
  - **Study Stats 구현 (7-42 ~ 7-44)**
    - `GET /admin/studies/stats/summary` — 총 학습수/Task수/시도수/해결수/해결률, Program별(basic_pronunciation/basic_word/basic_900/topik_read/topik_listen/topik_write/tbc)/State별(ready/open/close) 분포
    - `GET /admin/studies/stats/top` — TOP 학습 조회 (시도수/해결수/해결률 정렬, limit 1-50)
    - `GET /admin/studies/stats/daily` — 일별 시도수/해결수/활성사용자, 제로필
  - **Phase 7 엔드포인트 번호 재정렬 (7-1 ~ 7-67)**
    - 중복된 번호 수정 (7-23, 7-28 중복 해소)
    - `GET /admin/studies/{id}` (7-23), `GET /admin/studies/tasks/{id}` (7-29) 명확화
    - Study Stats 추가로 인한 후속 번호 조정 (Lessons: 7-45~7-62, User/Login Stats: 7-63~7-67)
  - **프론트엔드 Study Stats 페이지 구현**
    - `/admin/studies/stats` 라우트 추가
    - Summary Cards, Program/State 분포 차트, TOP Studies 테이블, Daily Stats 테이블
    - Studies 목록 페이지에 Stats 버튼 추가
- **2026-01-31 — Admin Lesson 프론트엔드 & Phase 7 Lesson 정비**
  - **Admin Lesson 프론트엔드 완성**
    - `/admin/lessons` — 목록 (검색/정렬/페이지네이션/벌크 수정)
    - `/admin/lessons/new` — 단건 생성
    - `/admin/lessons/bulk-create` — CSV 벌크 생성
    - `/admin/lessons/:lessonId` — 상세/수정 (Info/Items/Progress 탭)
  - **Lesson Items DELETE 엔드포인트 추가 (7-57, 7-58)**
    - `DELETE /admin/lessons/{id}/items/{seq}` — 수업 아이템 단건 삭제
    - `DELETE /admin/lessons/bulk/items` — 수업 아이템 다중 삭제
  - **Phase 7 엔드포인트 번호 재정렬 (7-45 ~ 7-67)**
    - Lessons: 7-45~7-62 (DELETE 추가로 +2)
    - User/Login Stats: 7-63~7-67 (기존 7-61~7-65에서 +2)
  - **Study Task 접근 제어 개선**
    - `study_state = 'open'` 필터 추가 (부모 Study가 닫히면 Task 접근 차단)
    - `find_task_detail`, `find_answer_key`, `get_try_count`, `find_task_explain`, `exists_task` 함수에 INNER JOIN study 추가
  - **Progress 수정 UI 구현**
    - Lesson Progress 탭에 단건/벌크 수정 다이얼로그 추가
    - Last Item Seq 필드에 max 제약 (lesson items 기준)
- **2026-02-02 — URL/함수명 통일 리팩토링**
  - **Handler 네이밍 통일**
    - `create_video_handler` → `admin_create_video`
    - `get_vimeo_preview_handler` → `admin_get_vimeo_preview`
    - `create_vimeo_upload_ticket_handler` → `admin_create_vimeo_upload_ticket`
    - `get_task_explain_handler` → `get_task_explain`
    - `admin_get_lesson_detail` → `admin_get_lesson`
  - **Admin User logs 함수명 prefix 통일**
    - `get_admin_user_logs` → `admin_get_user_logs`
    - `get_user_self_logs` → `admin_get_user_self_logs`
  - **Video repo 함수명 통일**
    - `find_list_dynamic` → `list_videos`
    - `find_detail_by_id` → `get_video_detail`
    - `find_progress` → `get_progress`
    - `upsert_progress` → `update_progress`
  - **Section 8.7 "보류/낮음 우선순위" 업데이트**
    - URL/함수명 통일 ✅ 완료
    - Login 정보/로그 추가 ✅ — ip-api.com 연동 완료
    - Lesson 통계 기능 — 추후 구현 예정
- **2026-02-04 — Admin Upgrade (관리자 초대) 시스템 구현**
  - **백엔드 (7-68 ~ 7-70)**
    - `POST /admin/upgrade` — 관리자 초대 코드 생성 + 이메일 발송
    - `GET /admin/upgrade/verify` — 초대 코드 검증 (Public)
    - `POST /admin/upgrade/accept` — 관리자 계정 생성 (Public, OAuth 불가)
    - RBAC 정책: HYMN→Admin/Manager, Admin→Manager, Manager→불가
    - Redis TTL 10분, 일회용 코드 (ak_upgrade_{uuid})
    - `EmailTemplate::AdminInvite` 추가 (invite_url, role, invited_by, expires_in_min)
  - **프론트엔드**
    - `types.ts` — Upgrade 타입 추가 (UpgradeInviteReq/Res, UpgradeVerifyRes, UpgradeAcceptReq/Res)
    - `admin_api.ts` — API 함수 추가 (createAdminInvite, verifyAdminInvite, acceptAdminInvite)
    - `/admin/upgrade/join` — 초대 수락 페이지 (Public 라우트)
    - `/admin/users` — "Invite Admin" 버튼 및 초대 다이얼로그 추가
  - **파일 변경 목록**
    - `src/api/admin/upgrade/` — dto.rs, service.rs, handler.rs, router.rs, mod.rs (신규)
    - `src/api/admin/mod.rs`, `src/api/admin/router.rs` — upgrade 모듈 등록
    - `src/api/user/repo.rs` — find_user_by_email, find_user_by_nickname, create_admin_user 추가
    - `src/external/email.rs` — AdminInvite 템플릿 추가
    - `frontend/src/category/admin/types.ts` — Section 9 (Upgrade 타입)
    - `frontend/src/category/admin/admin_api.ts` — Section 9 (Upgrade API)
    - `frontend/src/category/admin/page/admin_upgrade_join.tsx` — 신규
    - `frontend/src/category/admin/page/admin_users_page.tsx` — 초대 다이얼로그 추가
    - `frontend/src/app/routes.tsx` — /admin/upgrade/join 라우트 추가
- **2026-02-04 — IP Geolocation 기능 구현**
  - **기능**: 로그인 시 IP 기반 지리정보 자동 조회 (ip-api.com 연동)
  - **저장 필드**: `login_country`, `login_asn`, `login_org`
  - **적용 테이블**: `login` (활성 세션), `login_log` (이력)
  - **파일 변경 목록**
    - `src/external/ipgeo.rs` — IpGeoClient 구현 (신규)
    - `src/external/mod.rs` — ipgeo 모듈 등록
    - `src/state.rs` — AppState에 `Arc<IpGeoClient>` 추가
    - `src/main.rs` — IpGeoClient 초기화
    - `src/api/auth/repo.rs` — insert_login_record_tx, insert_login_record_oauth_tx에 지리정보 파라미터 추가
    - `src/api/auth/service.rs` — 로그인/OAuth 세션 생성 시 geo 데이터 전달
    - `src/api/user/service.rs` — 회원가입 자동 로그인에 geo 데이터 전달

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

**문서 끝 (End of Document)**
