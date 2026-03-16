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
  - [5.12 Phase 12 — 교재 주문 (Textbook Ordering)](#512-phase-12--교재-주문-textbook-ordering)
  - [5.12.5 Phase 12.5 — E-book 웹 뷰어 (E-book Web Viewer)](#5125-phase-125--e-book-웹-뷰어-e-book-web-viewer)
  - [5.13 Phase 13 — 학습 콘텐츠 시딩 (Content Seeding)](#513-phase-13--학습-콘텐츠-시딩-content-seeding)
  - [5.14 Phase 14 — AI 발음 평가 (Pronunciation Assessment)](#514-phase-14--ai-발음-평가-pronunciation-assessment)
  - [5.15 Phase 15 — 조음 애니메이션 (Articulation Animation)](#515-phase-15--조음-애니메이션-articulation-animation)
  - [5.16 Phase 16 — AI TTS 영상 제작 (Video Production)](#516-phase-16--ai-tts-영상-제작-video-production)

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

- [8. 작업 현황](#8-작업-현황)
  - [8.1 완료 항목](#81-완료-항목-)
  - [8.2 진행 예정 항목](#82-진행-예정-항목)
  - [8.3 세부 검토 사항 — 한국어 발음 교정 AI](#83-세부-검토-사항--한국어-발음-교정-ai-pronunciation-coaching-ai)
  - [8.4 상시 모니터링 항목](#84-상시-모니터링-항목)
  - [8.5 Paddle Live 전환 체크리스트](#85-paddle-live-전환-체크리스트)
  - [8.6 학습 콘텐츠 보안 전략 (Content Protection)](#86-학습-콘텐츠-보안-전략-content-protection)

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

- **데이터베이스 스키마**: [`docs/AMK_SCHEMA_PATCHED.md`](./AMK_SCHEMA_PATCHED.md) - 전체 DDL 정의
- **코드 예시 (Best Practices)**: [`docs/AMK_CODE_PATTERNS.md`](./AMK_CODE_PATTERNS.md) - 백엔드/프론트엔드 검증된 코드 패턴
- **배포 & 운영 가이드**: [`docs/AMK_DEPLOY_OPS.md`](./AMK_DEPLOY_OPS.md) - 빌드, 배포, CI/CD, 유지보수
- **변경 이력**: [`docs/AMK_CHANGELOG.md`](./AMK_CHANGELOG.md) - 시간 역순 변경 이력
- **개발 파이프라인**: [`docs/AMK_PIPELINE.md`](./AMK_PIPELINE.md) - 멀티 AI 오케스트레이션, 작업 흐름, 역할 분리
- **시장 분석 & 모바일 전략**: [`docs/AMK_MARKET_ANALYSIS.md`](./AMK_MARKET_ANALYSIS.md) - 경쟁사 분석, 결제 전략, 수익 최적화
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

#### 10-4 : `GET /admin/payment/transactions` (트랜잭션 목록)

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

#### 10-5 : `POST /admin/payment/grants` (수동 수강권 부여)

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

#### 10-6 : `GET /admin/payment/grants` (수동 부여 내역 조회)

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

#### 10-7 : `DELETE /admin/payment/grants/{userId}` (수동 수강권 회수)

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

### 5.12 Phase 12 — 교재 주문 (Textbook Ordering)

> 비회원 교재 주문 시스템. 계좌이체 기반, 20개 언어 × 2종(학생용/교사용), ₩25,000/권, 최소 10권.
> 마이그레이션: `migrations/20260226_textbook.sql`, `migrations/20260303_textbook_improvements.sql`

<details>
<summary>📋 Textbook 엔드포인트 상세 (클릭)</summary>

#### 12-1 : `GET /textbook/catalog` (교재 카탈로그)

> 주문 가능한 교재 목록과 가격 정보를 반환.

**인증**: 불필요

**응답 (성공 200)**
```json
{
  "items": [
    {
      "language": "ja",
      "language_name_ko": "일본어",
      "language_name_en": "Japanese",
      "available_types": ["student", "teacher"],
      "unit_price": 25000,
      "available": true
    }
  ],
  "currency": "KRW",
  "min_total_quantity": 10
}
```

#### 12-2 : `POST /textbook/orders` (주문 생성)

> 교재 주문 접수. 비회원도 주문 가능.

**인증**: 불필요

**요청**
```json
{
  "orderer_name": "홍길동",
  "orderer_email": "hong@example.com",
  "orderer_phone": "010-1234-5678",
  "org_name": "한국어학원",
  "org_type": "academy",
  "delivery_postal_code": "06234",
  "delivery_address": "서울특별시 강남구 ...",
  "delivery_detail": "3층",
  "payment_method": "bank_transfer",
  "depositor_name": "홍길동",
  "tax_invoice": false,
  "items": [
    { "language": "ja", "textbook_type": "student", "quantity": 10 }
  ],
  "notes": "빠른 배송 부탁드립니다"
}
```

**검증 규칙**:
- 총 수량 ≥ 10
- 각 항목 수량: 1~9999
- 중복 항목 거부 (같은 language + textbook_type 조합 불가)
- 비활성 언어 주문 차단 (카탈로그 `available=false`)
- tax_invoice=true일 때 tax_biz_number + tax_email 필수
- IP 기반 Rate Limiting (Redis, 기본 5회/시간)

**프론트엔드 약관 동의**: 주문 제출 전 약관 동의 모달 표시 (6개 조항 — 주문 접수, 결제, 배송, 교환/반품, 개인정보, 기타). 동의 체크 후 제출 가능.

**응답 (성공 201)**: OrderRes (주문 상세 + 항목)

#### 12-3 : `GET /textbook/orders/{code}` (주문 조회)

> 주문번호(order_code)로 주문 상태 조회. 비회원도 조회 가능.

**인증**: 불필요

**응답 (성공 200)**: OrderRes

#### 12-4 : `GET /admin/textbook/orders` (관리자 주문 목록)

> 교재 주문 목록 조회. 상태 필터, 검색, 페이지네이션 지원.

**인증**: Admin (IP Guard + Role Guard)

**쿼리 파라미터**: `page`, `size`, `q` (주문번호/신청자/기관 검색), `status`

**응답 (성공 200)**
```json
{
  "items": [OrderRes],
  "meta": { "total_count": 42, "total_pages": 3, "current_page": 1, "per_page": 20 }
}
```

#### 12-5 : `GET /admin/textbook/orders/{id}` (관리자 주문 상세)

**인증**: Admin

**응답 (성공 200)**: OrderRes

#### 12-6 : `PATCH /admin/textbook/orders/{id}/status` (상태 변경)

> 주문 상태 변경 + admin_textbook_log에 변경 이력 기록 + 고객 이메일 알림 발송.
> 상태 전환은 State Machine 규칙을 따름 (유효하지 않은 전환 시 400 반환).
> Shipped 전환 시 tracking_number 필수 (없으면 400 반환).

**인증**: Admin

**요청**: `{ "status": "confirmed" }`

**상태 전환 규칙**:
```
pending → confirmed → paid → printing → shipped → delivered (정방향)
pending/confirmed/paid/printing/shipped → canceled (취소)
delivered/canceled → (전환 불가)
```

**응답 (성공 200)**: OrderRes

#### 12-7 : `PATCH /admin/textbook/orders/{id}/tracking` (배송 추적 정보 업데이트)

> 배송 추적번호/택배사 정보 업데이트 + admin_textbook_log 기록.

**인증**: Admin

**요청**: `{ "tracking_number": "1234567890", "tracking_provider": "CJ대한통운" }`

**응답 (성공 200)**: OrderRes

#### 12-8 : `DELETE /admin/textbook/orders/{id}` (주문 삭제)

> Soft Delete (is_deleted = true, deleted_at = NOW()). admin_textbook_log에 삭제 이력 기록.
> FK 제약조건: RESTRICT (감사 로그 보존을 위해 물리 삭제 불가).

**인증**: Admin

**응답**: `204 No Content`

</details>

### 5.12.5 Phase 12.5 — E-book 웹 뷰어 (E-book Web Viewer) ✅

> 자체 사이트에서 교재(학생용/교사용) e-book을 구매하고 열람할 수 있는 웹 뷰어 시스템.
> **회원 전용** — 로그인 필수 (비회원 구매 불가), `user_id`로 구매 연동.
> **웹 전용** — 페이지 이미지 기반 렌더링 (EPUB/PDF 원본 미노출, 다운로드 없음).
> 향후 모바일 앱(React Native) 및 데스크탑 앱(Tauri)에서 오프라인 EPUB 뷰어로 확장.

**교재 3종 유통 정책**:
| 종류 | 용도 | 유통 채널 |
|------|------|----------|
| 학생용 | 교사가 가르칠 때 학생이 사용 | **자체 사이트만** (인쇄물 + e-book) |
| 교사용 | 교사가 학생을 가르칠 때 사용 | **자체 사이트만** (인쇄물 + e-book) |
| 해설용 | 학생이 혼자 공부 | 자체 사이트 + 외부 플랫폼 (Amazon/Apple/Google/Kobo/교보/Yes24) |

**3중 보안 아키텍처**:
```
Layer 1: 구조적 보안
  • 회원 전용 — AuthUser JWT 필수 (비회원 구매 불가)
  • EPUB/PDF 파일 미노출 (페이지 이미지만 제공)
  • 다운로드 없음 — 웹 뷰어 전용 (오프라인은 추후 앱)
  • user_id로 구매 소유 확인 (타인 구매 접근 차단)

Layer 2: 포렌식 워터마크 (실시간 동적)
  • 가시적: 사용자 이메일 대각선 반투명 오버레이
  • 비가시적: LSB 스테가노그래피 (purchase_code 인코딩)
  • 매 요청마다 고유 watermark_id 생성 → 감사 로그 연동

Layer 3: 플랫폼 보안
  • 브라우저: 우클릭/선택/인쇄/드래그 차단
  • Cache-Control: no-store (브라우저 캐시 방지)
  • blob:// URL 사용 (네트워크 탭에 이미지 URL 미노출)
  • 레이트 리밋: 30페이지/분/user (벌크 크롤링 차단)
```

**앱 확장 로드맵**:
```
Phase 1 [웹] ✅ 페이지 이미지 뷰어 — 온라인 전용, EPUB 미노출
Phase 2 [모바일 앱] React Native + EPUB 암호화 저장 — 오프라인 지원
Phase 3 [데스크탑 앱] Tauri(Rust) + DevTools 차단 — 오프라인 지원
```

**DB**: ENUM 3개 (`ebook_edition_enum`, `ebook_purchase_status_enum`, `ebook_payment_method_enum`) + 테이블 3개 (`ebook_purchase`, `ebook_access_log`, `admin_ebook_log`)

**가격**: 교사용 ₩15,000 / 학생용 ₩12,000 (KRW)

<details>
<summary>📋 E-book 웹 뷰어 엔드포인트 상세 (클릭)</summary>

#### 12.5-1 : `GET /ebook/catalog` (e-book 카탈로그)

> 구매 가능한 e-book 목록 (언어별, 에디션별). manifest.json에서 페이지 수 로드.

**인증**: 불필요

**응답 (성공 200)**
```json
{
  "items": [
    {
      "language": "vi",
      "language_name_ko": "베트남어",
      "language_name_en": "Vietnamese",
      "editions": [
        { "edition": "teacher", "price": 15000, "currency": "KRW", "total_pages": 124, "available": true },
        { "edition": "student", "price": 12000, "currency": "KRW", "total_pages": 90, "available": true }
      ]
    }
  ]
}
```

#### 12.5-2 : `POST /ebook/purchase` (e-book 구매)

> e-book 구매 생성. 로그인 필수. Paddle 결제 또는 계좌이체.
> 중복 구매 방지 (동일 user + language + edition, completed 상태).

**인증**: AuthUser (JWT)

**Rate Limit**: IP 기반 5회/시간

**요청**
```json
{
  "language": "vi",
  "edition": "teacher",
  "payment_method": "paddle"
}
```

**응답 (성공 200)**
```json
{
  "purchase_code": "VN-ST-20260310-CA-0001",
  "status": "pending",
  "language": "vi",
  "edition": "teacher",
  "payment_method": "paddle",
  "price": 15000,
  "currency": "KRW",
  "created_at": "2026-03-09T12:00:00Z"
}
```

**구매코드 형식**: `{LANG}-{ED}-{YYYYMMDD}-{PAY}-{NNNN}`
- LANG: 언어 코드 (VI, JA, ZH_CN, ZH_TW, RU, MN, MY, TH, HI, NE, SI, KM, ES, PT, FR, DE, ID, UZ, KK, TG, TL)
- ED: ST (학생용) / TC (교사용)
- PAY: CA (Paddle 카드) / BT (계좌이체)
- NNNN: 일별 순번 (MAX 기반, Advisory Lock으로 동시성 안전)
- DB: `purchase_code VARCHAR(30)` (최대 25자: ZH_CN-TC-20260310-BT-0001)

#### 12.5-3 : `GET /ebook/my` (내 구매 목록)

> 로그인한 사용자의 e-book 구매 목록 조회.

**인증**: AuthUser (JWT)

**응답 (성공 200)**
```json
{
  "items": [
    {
      "purchase_code": "VN-ST-20260310-CA-0001",
      "status": "completed",
      "language": "vi",
      "edition": "teacher",
      "payment_method": "paddle",
      "price": 15000,
      "currency": "KRW",
      "created_at": "2026-03-09T12:00:00Z"
    }
  ]
}
```

#### 12.5-4 : `GET /ebook/viewer/{code}/meta` (뷰어 메타 정보)

> 뷰어 초기화 데이터 (TOC, 총 페이지 수). 구매 소유 + completed 상태 확인.

**인증**: AuthUser (JWT) + 구매 소유 확인

**응답 (성공 200)**
```json
{
  "purchase_code": "EB-260309-0001",
  "language": "vi",
  "edition": "teacher",
  "total_pages": 124,
  "toc": [
    { "title": "Part I. 발음", "page": 1 },
    { "title": "Part II. 어휘", "page": 25 }
  ]
}
```

#### 12.5-5 : `GET /ebook/viewer/{code}/pages/{page_num}` (페이지 이미지 조회)

> 워터마크 적용된 페이지 이미지 반환. 보안 핵심 엔드포인트.

**인증**: AuthUser (JWT) + 구매 소유 확인 + completed 상태

**Rate Limit**: 30페이지/분/user_id (Redis INCR)

**보안 헤더**:
```
Content-Type: image/webp
Cache-Control: private, max-age=300
X-Content-Type-Options: nosniff
```

**워터마크 (4중 비가시적 보안)**:
1. 풋터 워터마크: `{pageNum} | {purchaseCode} | Amazing Korean` — 페이지 하단 풋터 영역, #999999 회색
2. 마이크로 도트: user_id 64비트를 4 모서리에 16비트씩 near-white(#FEFEFE) 도트 인코딩
3. LSB 스테가노그래피: purchase_code + watermark_id SHA-256 해시 → R 채널 LSB (비트별 고유 시드)
4. 접근 로그: `ebook_access_log` 테이블에 purchase_id, user_id, page_number, watermark_id, IP, UA 기록

**알려진 제약사항**:
- 풋터 텍스트 중앙 정렬은 글자당 ~9px 추정 기반 — 폰트에 따라 약간의 좌우 편차 가능 (기능 영향 없음)
- 커버 페이지(1~4)에도 풋터 워터마크 적용됨 — 커버 디자인에 따라 시각 QA 필요
- `Cache-Control: private, max-age=300` — 5분 내 동일 페이지 재방문 시 캐시 히트로 새 워터마크 미적용 (기존 watermark_id와 이미지 일치하므로 포렌식 추적 정상)
- 감사 로그는 `tokio::spawn` fire-and-forget — DB 일시 장애 시 로그 유실 가능 (이미지 반환 우선)

#### 12.5-6 : `GET /admin/ebook/purchases` (관리자 구매 목록)

> e-book 구매 내역 조회 (검색, 필터, 페이지네이션).

**인증**: Admin (IP Guard + Role Guard)

**쿼리 파라미터**: `page`, `per_page`, `status`, `search`

#### 12.5-7 : `GET /admin/ebook/purchases/{id}` (관리자 구매 상세)

> 구매 상세 정보 + 접근 로그 조회.

**인증**: Admin

#### 12.5-8 : `PATCH /admin/ebook/purchases/{id}/status` (상태 변경)

> 구매 상태 변경. 유효 전환만 허용 (pending→completed, pending→refunded, completed→refunded).

**인증**: Admin

**요청**: `{ "status": "completed" }`

#### 12.5-9 : `DELETE /admin/ebook/purchases/{id}` (구매 삭제)

> 구매 Soft Delete + 관리자 로그 기록.

**인증**: Admin

</details>

**Paddle 연동 (일회성 결제)**:
- `transaction.completed` 웹훅에서 `custom_data.type == "ebook"` + `custom_data.purchase_code` 확인
- `ebook_purchase.paddle_txn_id` 저장 + `status` → `completed` 업데이트

**빌드 파이프라인 (페이지 이미지 생성)**:
```bash
# 기존 EPUB 빌드 → Puppeteer → 페이지별 WebP 이미지
node scripts/textbook/generate_page_images.js vi teacher   # 단일
node scripts/textbook/generate_page_images.js all all       # 전체
```
- 입력: `docs/textbook/books/{edition}-inner/AMK_{EDITION}_INNER_{LANG}.html`
- 출력: `docs/textbook/page-images/{edition}/{lang}/page-001.webp ~ page-NNN.webp`
- 매니페스트: `docs/textbook/page-images/{edition}/{lang}/manifest.json`

**프론트엔드 페이지**:
- `/ebook` — e-book 카탈로그 (언어/에디션 선택, 가격, 구매 — 로그인 필수)
- `/ebook/viewer/{purchaseCode}` — 웹 뷰어 (blob:// URL, 키보드/버튼 네비, TOC, 줌, 풀스크린)
- `/ebook/my` — 내 구매 목록 (상태 배지, 뷰어 열기 버튼)
- `/admin/ebook/purchases` — 관리자 구매 목록 (필터/검색/페이지네이션)
- `/admin/ebook/purchases/{id}` — 관리자 구매 상세 (상태 변경, 삭제)

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | DB 마이그레이션 (ENUM 3개 + 테이블 3개) | ✅ |
| 2 | 페이지 이미지 생성 스크립트 (generate_page_images.js) | ✅ |
| 3 | 백엔드 API (catalog, purchase, my, viewer meta/pages) | ✅ |
| 4 | 서버사이드 워터마크 (4중 비가시적: 풋터+마이크로도트+LSB+접근로그) | ✅ |
| 5 | 프론트엔드 뷰어 + 카탈로그 + 구매 목록 | ✅ |
| 6 | Paddle 결제 연동 (e-book 일회성 결제) | ✅ |
| 7 | 관리자 페이지 (구매 목록, 상세, 상태 변경, 삭제) | ✅ |

### 5.13 Phase 13 — 학습 콘텐츠 시딩 (Content Seeding)

> 교재 JSON 데이터를 DB(study, study_task, lesson, course)로 시딩하여 웹 학습 콘텐츠를 구성한다.
> 교재 페이지 순서 = 학습 순서 (page_manifest.json 기준).

**데이터 소스**: `scripts/textbook/data/` (11개 JSON 파일)

| JSON 파일 | 내용 | Study Program |
|-----------|------|---------------|
| vocabulary.json | 어휘 카드 (한국어 + 20개 언어 번역, 280+) | basic_word |
| sentences.json | 문법 예문 (한국어 + 번역, 496+) | basic_900 |
| pronunciation.json | 한글 조합표 (자음×모음, 테이블 7+) | basic_pronunciation |
| pronunciation_test.json | 발음 테스트 연습 문제 | basic_pronunciation |
| particles.json | 조사 활용표 | basic_900 |
| conjugation.json | 동사/형용사 활용 (현재/과거/미래) | basic_900 |
| structure.json | 문장 구조 (의문사 패턴) | basic_900 |
| appendix.json | 숫자, 문법 연습 | basic_900 |

**DB 계층 구조**:
```
Course "놀라운 한국어 기초"
├── Part I. 발음 (Lesson 1~7): pronunciation + pronunciation_test + vocabulary(발음)
├── Part II. 문법 기초 (Lesson 8~10): particles + structure
├── Part III~IV. 서술어/부사어 문법 (Lesson 11~30): sentences (섹션별 1 Lesson)
├── Part V. 동사 활용 (Lesson 31~33): conjugation
└── Part VI. 부록 (Lesson 34~35): appendix
```

**문제 유형**: choice (4지선다), typing (직접 입력 / 클릭 배열), voice (발음)

**구현**: Seed Script (`scripts/textbook/JSON → seed_script → DB`)

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | Seed Script 설계 (JSON → DB 매핑) | ⬜ |
| 2 | Study 세트 생성 (program별) | ⬜ |
| 3 | StudyTask 생성 (choice/typing 문제 자동 생성 + 오답 생성) | ⬜ |
| 4 | Lesson 생성 + LessonItem 연결 | ⬜ |
| 5 | Course 생성 + course_lesson 연결 | ⬜ |
| 6 | 다국어 explain 시딩 (20개 언어) | ⬜ |

### 5.14 Phase 14 — AI 발음 평가 (Pronunciation Assessment)

> 한국어 학습자 발음을 음소 단위로 평가하고 교정 피드백을 제공한다.
> 3단계 접근: Phase 1(따라하기 안내) → Phase 2(SpeechSuper API) → Phase 3(커스텀 모델).

**Phase 14-1**: 따라하기 안내 (녹음/판별 없음, UI에서 "따라 해보세요" 안내)

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | Study voice task에 "따라하기" UI 안내 추가 | ⬜ |
| 2 | audio_url 개별 음성 재생 기능 | ⬜ |

**Phase 14-2**: SpeechSuper API 프로토타이핑 (콘텐츠 완성 후)

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | SpeechSuper API 통합 (Rust 백엔드) | ⬜ |
| 2 | 한 글자 / 단어 / 문장 발음 평가 엔드포인트 | ⬜ |
| 3 | 음소별 점수 + 오발음 피드백 UI | ⬜ |
| 4 | 사용자 반응/평가 데이터 수집 | ⬜ |

**Phase 14-3**: 커스텀 모델 개발 (기술 검증 후)

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | AIHub 71469 데이터셋 확보 (HYMN 법인 명의) | ⬜ |
| 2 | wav2vec2-large-xlsr-korean 파인튜닝 | ⬜ |
| 3 | 초성/중성/종성 3-way 분류 + GOP 점수화 | ⬜ |
| 4 | L1별(20개 언어) 맞춤 오류 피드백 | ⬜ |
| 5 | API 서버 배포 + 백엔드 통합 | ⬜ |

### 5.15 Phase 15 — 조음 애니메이션 (Articulation Animation)

> 한국어 음소별 입모양/혀위치를 SVG 애니메이션으로 시각화한다.
> 15~17개 다이어그램으로 전체 음소 커버. 한국어 전용 조음 애니메이션 도구 부재 → 차별화 기회.

**기술 스택**: Wikimedia CC0 SVG + Figma 수정 → GSAP MorphSVG + React

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | CC0 SVG 다운로드 (IPA 조음도) + 한국어 특화 수정 (Figma) | ⬜ |
| 2 | 자음 7개 조음 위치 SVG path 데이터 추출 | ⬜ |
| 3 | 모음 7~9개 혀 위치 SVG path 데이터 추출 | ⬜ |
| 4 | 성문 상태도 (평음/경음/격음) SVG 제작 | ⬜ |
| 5 | ㅈ/ㅊ/ㅉ 치경구개 파찰음 SVG 자체 제작 | ⬜ |
| 6 | JSON 데이터 모델 (phoneme → path + audio + metadata) | ⬜ |
| 7 | React `<ArticulationDiagram>` 컴포넌트 + GSAP MorphSVG | ⬜ |
| 8 | TTS 오디오 동기화 (남성/여성) | ⬜ |

### 5.16 Phase 16 — AI TTS 영상 제작 (Video Production)

> 교재 JSON 데이터 기반 AI 음성 + 애니메이션 학습 영상 자동 생성 파이프라인.
> 영상 1개 + 자막 20개 언어 = 20개 언어 커버.

**파트별 구성**:
- **Part I. 발음**: AI 음성(남/여) + 조음 애니메이션 + 자막(한글 + 발음기호 + 학습자 모국어)
- **Part II~VI. 문법/문장**: AI 음성 + 텍스트 + 예문 순차 재생 + 모국어 자막

**학습 흐름**: [영상] 전체 흐름 한 번 시청 → [Study] 자기 페이스 연습/복습 → [이후] Study만 반복

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | TTS 기술 선정 (Google Cloud TTS / CLOVA / OpenAI TTS 비교) | ⬜ |
| 2 | TTS 녹음 파이프라인 (JSON → 스크립트 → TTS → 오디오 파일) | ⬜ |
| 3 | 자막 자동 생성 (교재 JSON translations → SRT/VTT, 20개 언어) | ⬜ |
| 4 | 영상 템플릿 제작 (파트별 레이아웃) | ⬜ |
| 5 | 영상 자동 렌더링 파이프라인 (오디오 + 자막 + 애니메이션 → 영상) | ⬜ |
| 6 | Lesson/Video DB 연결 + lesson_item kind=video 시딩 | ⬜ |

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
    textbook/              # 교재 주문 (Public)
      page/
      hook/
      types.ts
      textbook_api.ts
    admin/
      # ... (동일 구조)
      textbook/            # 교재 주문 관리 (Admin)
        page/
        hook/
        types.ts

  components/            # 공용 컴포넌트 (Horizontal Slicing)
    ui/                  # ★ shadcn/ui 설치 경로 (Button, Dialog 등)
    layout/              # Header, Footer, Sidebar 등 레이아웃 조각
    sections/            # HeroSection, SectionContainer, PaginationBar 등 페이지 섹션
    shared/              # 도메인에 종속되지 않는 재사용 컴포넌트 (LoadingSpinner 등)
    page_meta.tsx        # ★ SEO: React 19 네이티브 metadata (title, canonical, OG/Twitter)

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

> 상세 사용법, 금지 규칙, PR 체크리스트: [`docs/AMK_DESIGN_SYSTEM.md`](AMK_DESIGN_SYSTEM.md)

- **색상 토큰 (index.css 기반)**
  - `primary`: 브랜드 메인 (Navy `#051D55`) → 주요 액션, Footer
  - `secondary`: 보조 (Blue `#4F71EB`) → 서브 버튼, 데코
  - `accent`: 강조 (Cyan `#129DD8`) → 학습 완료, 아이콘
  - `destructive`: 위험/삭제/에러 (= error 통일)
  - `muted`: 비활성/배경
  - `brand-soft` / `brand-soft-alt`: Hero 그라데이션 배경
  - `status-success` / `warning` / `info`: 상태 색상 (HSL + foreground 세트)

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
     - `/admin/**` 경로는 기본적으로 **"허용된 롤만 접근 가능"**(default deny) 원칙을 따른다.
     - 롤별 세부 권한 매트릭스:

       | 역할 | Admin 접근 | 데이터 범위 | 비고 |
       |------|----------|------------|------|
       | **HYMN** | 가능 | 전체 | 모든 기능 + 시스템 설정 |
       | **admin** | 가능 | 전체 | 읽기/쓰기 모든 기능 |
       | **manager** | 불가 | 담당 class | 향후 class 기반 접근 구현 예정 |
       | **learner** | 불가 | 자신만 | 일반 사용자 |

     - 구현: 백엔드 `src/api/admin/role_guard.rs` (미들웨어 RBAC), Admin IP Allowlist (`admin_ip_guard.rs`), 프론트 `AdminRoute` 컴포넌트
     - TODO: manager 역할 — class 테이블 구현 후 담당 학습자 범위 내 접근 권한 부여
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

## 8. 작업 현황

### 8.1 완료 항목 ✅

| # | 항목 | 카테고리 | 내역 | 완료일 | 향후 확장 |
|:-:|------|---------|------|:------:|----------|
| 1 | Admin 통계 API | 기능 | users/logins/studies/videos 통계 엔드포인트 + 프론트 UI | 2026-01 | 시스템 모니터링 (DB/Redis) |
| 2 | RBAC + Admin 감사 로그 | 보안 | role_guard, IP Allowlist, actor_user_id 전달, AdminRoute, 에러 페이지 | 2026-02-02 | manager class 기반 접근, 세분화 권한 |
| 3 | 코드 일관성 + 네이밍 | 코드 품질 | 함수명/URL 통일, Stateless 패턴, Refresh Token 포맷 등 8건 | 2026-02-02 | — |
| 4 | 내부 DB 작업 | 인프라 | Redis 보안(인증+포트), 시청 시간 추적, Study 레이트리밋, Course 도메인, 수강권 정책 | 2026-02-02 | — |
| 5 | Google OAuth | 외부 API | Authorization Code Flow, 프론트엔드 소셜 로그인 | 2026-02-03 | Apple OAuth (비용 보류) |
| 6 | Login/Login_log 개선 | 기능 | UA 서버파싱(woothee), 세션 컬럼 활성화, JWT jti, Geo 기본값 | 2026-02-05 | — |
| 7 | DB 암호화 (Phase 1~3) | 보안 | AES-256-GCM + HMAC Blind Index, 키 로테이션, 55+ call sites 적용 | 2026-02-08 | AWS KMS envelope → HSM |
| 8 | 프로덕션 배포 + 하드닝 | 인프라 | 통합 마이그레이션, Redis 보안, 보안 헤더, Swagger/Health 숨김, 404 Fallback | 2026-02-10 | — |
| 9 | 이메일 시스템 (Resend) | 외부 API | 회원가입 인증, 계정 복구, Rate Limiting, 관리자 초대, 도메인 검증 | 2026-02-09 | — |
| 10 | 다국어 (i18n) | 기능 | 21개 언어, 번역 CRUD API 7개, `?lang=` fallback, Google Translate, Noto Sans 동적 로딩 | 2026-02-14 | — |
| 11 | 세션 보안 + MFA | 보안 | 역할별 TTL, 토큰 재사용 탐지 (409 Conflict), TOTP MFA + 백업 코드 10개, 강제 설정 가드 | 2026-02-14 | 동시 세션 제한, step-up MFA |
| 12 | 결제 시스템 (Paddle) | 외부 API | Webhook 9종, 구독 취소, 수강권 자동 부여/회수, 관리자 9개 API, Pricing UI (Paddle.js) | 2026-02-16 | Paddle Live 전환 |
| 13 | Design System v2/v3 | UI | 공유 컴포넌트 6개 (PaginationBar, EmptyState, SkeletonGrid, ListStatsBar, StatCard, Card CVA), 다크모드 (next-themes, CSS 변수 60+ 토큰, 22개 로케일), UI/UX 가이드라인 문서화 | 2026-02-19 | — |
| 14 | Paddle KYB 서류 제출 | 결제 | 사업자등록증 + 주주명세서 (한/영) Paddle Dashboard 업로드 | 2026-02-19 | Live 전환 심사 대기 (2~4 영업일) |
| 15 | CEO 영문 이름 통일 | 관리 | i18n 18개 로케일 `Kyoung Ryun KIM`, noscript `KIM KYEONGRYUN` (사업자등록증 기준) | 2026-02-19 | — |
| 16 | 교재 주문 시스템 | 기능 | 비회원 주문, 계좌이체, 20개 언어 × 2종, ₩25,000/권, 최소 10권, DB 4 ENUM + 3 테이블, 백엔드 8 API, 프론트 5 페이지, 견적서/주문확인서 인쇄, 약관 동의 모달, 상태 머신 검증, 이메일 알림 (주문확인/상태변경), Rate Limiting, Advisory Lock, Soft Delete | 2026-03-03 | 카드/Paddle 결제 |
| 17 | 교재 HTML 재구축 시스템 | 콘텐츠 | JSON(11) + JS 컴포넌트(10) + CSS(9) → HTML → PDF 자동 생성 파이프라인, 120페이지, 원본 99.2% 일치, Puppeteer CSS 전수 감사 | 2026-03-02 | 20개 언어 자동 생성 토대 |
| 18 | 교재 번역 Wave 1 | 콘텐츠 | ja, zh_cn, id, th — 923항목 × 4언어 번역, translate_extract/merge 도구, PDF 생성 완료 | 2026-03-02 | Wave 2~5 (16개 언어) |

> **암호화 참고**: 대상 PII — `user_email`, `user_name`, `user_birthday`, `user_phone`, `oauth_email`, `oauth_subject`, `login_ip`, `admin_action_log.ip_address`
> **키 관리**: `ENCRYPTION_KEY_V{n}` (AES-256, 다중 버전) + `HMAC_KEY` (blind index), KeyRing 로드
> **다국어 참고**: 21개 언어 (아랍어 RTL 제외), Fallback: 사용자 언어 → en → ko, 공개 조건: `status = 'approved'`

### 8.2 진행 예정 항목

#### 핵심 (우선순위 순)

| # | 항목 | 카테고리 | 내역 | 예상 결과 | 조건/시점 |
|:-:|------|---------|------|----------|----------|
| 0 | **Paddle Live 전환** | 결제 | Sandbox → 프로덕션 전환 (§8.5 체크리스트). KYB 서류 심사 완료, Identity Verification (Onfido) 완료, 계정 활성화 대기 중 (1~3 영업일) | 실결제 수신 가능 | **최우선 — 활성화 후 환경변수 교체만** |
| 0.5 | **e-book 웹 뷰어** | e-book | 페이지 이미지 기반 웹 뷰어 (Phase 12.5), 구매/다운로드 시스템, Paddle 연동, 워터마크 | 자체 사이트 e-book 열람/구매 | **다음 구현 대상** |
| 0.6 | **학습 콘텐츠 시딩** | 콘텐츠 | 교재 JSON → DB 시딩 (Phase 13), 기존 테스트 데이터 삭제 후 실 콘텐츠 투입 | 웹 학습 서비스 실데이터 | Paddle Live 전환 후 |
| 0.6 | **교재 번역 Wave 2~5** | 콘텐츠 | 잔여 16개 언어 번역 (zh_tw, es, hi 완료 → km 진행 중 → Wave 3~5) | 20개 언어 교재 + 학습 콘텐츠 완성 | 병행 진행 |
| 1 | 동시 세션 수 제한 | 보안 | 역할별 동시 세션 상한 설정 | 다중 기기 무분별 로그인 방지 | RDS 이전 후 |
| 2 | RDS/ElastiCache 이전 | 인프라 | EC2 단일 DB → AWS RDS + ElastiCache | TLS, 자동 백업, maxmemory 자동 적용 | 다음 우선순위 |
| 3 | 다중 서버 구성 (HA) | 인프라 | ①nginx+컨테이너 복제 → ②ALB+EC2 → ③ECS Fargate | 고가용성, 무중단 배포, Auto Scaling | RDS 완료 후 |
| 4 | 시스템 모니터링 | 인프라 | DB/Redis 상태, 서버 리소스 실시간 확인 | Admin 대시보드 통합 | 필요 시 |
| 5 | K6 성능 테스트 | 테스트 | 인증/조회/진도저장 부하 테스트, CI 연계 | SLA 기준 검증 (아래 표 참조) | CI 구축 시 |
| 6 | ~~디자인 시스템~~ | ~~UI~~ | ~~브랜딩, 타이포그래피, 반응형 점검~~ | ~~일관된 UI/UX 체계~~ | ✅ §8.1 #13 |

**K6 성능 목표치 (엔드포인트별)**:

| 엔드포인트 | 목표 RPS | P95 응답시간 |
|----------|---------|-------------|
| 인증 (login/refresh) | 100 | < 200ms |
| 목록 조회 (videos/studies) | 200 | < 100ms |
| 상세 조회 | 300 | < 50ms |
| 진도 저장 (progress) | 100 | < 150ms |

**대표 시나리오**: 회원가입 → 로그인 → 비디오 조회 → 시청 → 진도 저장 → 학습 문제 풀이
| 7 | 마케팅/데이터 분석 | 기능 | 사용자 세그먼트, 리텐션 분석, 마케팅 자동화 | 데이터 기반 의사결정 | 사용자 확보 후 |

#### 보류/조건부

| # | 항목 | 카테고리 | 내역 | 예상 결과 | 조건/시점 |
|:-:|------|---------|------|----------|----------|
| 8 | Apple OAuth | 외부 API | Apple Sign In 구현 | iOS 사용자 편의성 | 비용/환경 해결 시 |
| 9 | GeoIP 전환 | 인프라 | ip-api.com → MaxMind GeoLite2 로컬 DB | HTTPS, 무제한 쿼리 | 트래픽 증가 시 |
| 10 | step-up MFA | 보안 | 민감 작업 시 추가 인증 요구 | 결제/비밀번호 변경 시 보안 강화 | 필요 시 |
| 11 | 이메일 수신 | 외부 API | `support@amazingkorean.net` 수신 | 사용자 문의 처리 | Cloudflare Routing 또는 Workspace |
| 12 | 토큰 Redis 캐싱 | 보안 | 재발급 시 DB 조회 → Redis 캐시 | 동시 접속 성능 개선 | 동시접속 10K+ |
| 13 | enum sqlx::Type 전환 | 코드 품질 | 수동 match → `#[sqlx(type_name)]` derive | 보일러플레이트 감소 | 일괄 전환 시점 검토 |
| 14 | Keyset 페이징 | 기능 | page/size → keyset pagination | 대용량 테이블 성능 개선 | 데이터 1만 건+ |
| 15 | Lesson 통계 | 기능 | `/admin/lessons/stats` 구현 | 수업별 진행도 분석 | 필요 시 |
| 16 | 학습 문제 동적 생성 | 기능 | 커리큘럼 기반 문제 자동 생성/전달 | 학습 콘텐츠 확장 | 커리큘럼 완비 후 |
| 17 | 통계 비동기/배치 분리 | 인프라 | 집계 로직 비동기 처리 | API 응답 속도 개선 | 집계 복잡화 시 |
| 18 | OAuth 중복 통합 | 코드 품질 | auth repo/service 리팩토링 | 코드 중복 제거 | 세 번째 OAuth 추가 시 |
| 19 | manager 역할 구현 | 기능 | class 기반 접근 권한 부여 | 담당 학습자 범위 내 관리 | class 테이블 구현 후 |

#### 다국어 UI 참고 (21개 언어, LTR 전용)

| 항목 | 설명 |
|------|------|
| **폰트** | Noto Sans 패밀리 동적 로딩 (Latin/Cyrillic/CJK/Thai/Myanmar/Khmer/Sinhala/Devanagari) |
| **텍스트 길이** | 독일어 등 60%+ 길어질 수 있음 → 고정 폭 금지, flex/grid 사용, `text-overflow: ellipsis` |
| **줄 높이** | Thai/Myanmar/Khmer/Sinhala 결합 문자 → `line-height: 1.6~1.8` |

### 8.3 세부 검토 사항 — 한국어 발음 교정 AI (Pronunciation Coaching AI)

**현재 상태**: 조사 완료, 단계별 전략 확정 (2026-03-03)

**문제 정의**: 한국어 학습자의 발음 교정은 1:1 원어민 교사 없이는 사실상 불가능하다. **한 글자 단위 발음 교정이 초급 학습자에게 가장 중요**하며, 기존 서비스는 이를 지원하지 않는다.

**핵심 원칙**: 한국어는 비성조 언어 → 음높이 변화 거의 없음 → 한 글자씩 끊어서 발음 연습이 핵심.

#### 기존 API 비교 (조사 완료)

| API | 한 글자 | 음소별 점수 | 한국어 | 가격 | 평가 |
|-----|:-------:|:---------:|:------:|------|------|
| **SpeechSuper** | **지원** | **있음** (오발음 피드백 포함) | 4단계 (글자/단어/문장/단락) | $0.004/건 (단어) | **최적** |
| ETRI e-PreTX | 미지원 | 없음 (1~5점 단일 점수) | 문장만 | 무료 (1K/일) | 제한적 |
| Azure Speech | 미설계 | 있으나 음소명 미제공 (ko-KR) | 있음 | ~$1/시간 | 부적합 |

- **SpeechSuper**: 유일하게 한글 1자 음소별 평가 지원, REST + WebSocket, Rust SDK 제공
- **ETRI**: 2025.07 aiopen → e-PreTX 이전, PCM 전용, 자유 발화 시 전문가와 상관관계 없음 (Kim & Ko, 2022)

#### 3단계 전략 (확정)

**Phase 1** (현재): 따라하기 안내만 (녹음/판별 없음)

**Phase 2** (콘텐츠 완성 후): SpeechSuper API 프로토타이핑
- 한 글자 + 단어/문장 발음 평가, 음소별 점수 + 오발음 피드백
- Rust 백엔드 통합, 사용자 반응/데이터 수집
- 비용: $0.004/건 → 1,000명 활동 시 ~$1,680/월 (Growth $0.0028)

**Phase 3** (기술 검증 후): 커스텀 모델 개발
- **왜 한국어가 유리한가**: 음절 = 초성(19) + 중성(21) + 종성(28) 고정 구조, ~40개 음소 소규모 분류 문제, 성조 없음
- **베이스 모델**: `kresnik/wav2vec2-large-xlsr-korean` (Apache 2.0, WER 4.74%, CER 1.78%)
- **학습 데이터**: AIHub 71469 (1,030시간, 영어모국어 한국어 음성, 음소 시간 정렬 + 오류 태그) — HYMN 법인으로 접근 가능
- **아키텍처**: wav2vec2 파인튜닝 → 초성/중성/종성 3-way 분류 헤드 + GOP 점수화 (0~100)
- **학술 검증**: PER 10.25%, 전문가 일치 90% (2024, 1.56h 데이터), L1별 39개 오류 패턴 분류 (ICPhS 2023)
- **개발 비용**: GPU $200~$1,000, 추론 CPU 가능 (<100ms/음절, $50~200/월), 기간 2~3개월
- **장점**: API 비용 제거 + 데이터 주권 + L1별(20개 언어) 맞춤 피드백

#### L2 학습자 공통 오류 패턴 (ICPhS 2023)

1. 격음/경음 → 평음 대치 (ㅋ/ㄲ→ㄱ, ㅌ/ㄸ→ㄷ, ㅍ/ㅃ→ㅂ, ㅊ/ㅉ→ㅈ, ㅆ→ㅅ)
2. 종성(받침) 탈락
3. 이중모음 → 단모음 대치
4. L1별 고유 패턴 (베트남어: /l/→/n/ 종성, 일본어: /ŋ/ 삽입 등)

#### 조음 애니메이션 (Phase 15 연동)

- **15~17개 다이어그램**으로 한국어 전체 음소 커버 (자음 7 조음 위치 + 성문 1 + 모음 7~9)
- 평음/경음/격음은 같은 입 위치 → 성문 상태도로 차이 표현
- **Wikimedia CC0 SVG** 활용 (IPA 조음 단면도, 퍼블릭 도메인) → 자체 제작 최소화
- 기술: Figma → SVG → GSAP MorphSVG (무료) + React
- **한국어 전용 조음 애니메이션 도구 부재 → 차별화 기회** (경쟁사 분석 완료)

#### 맞춤형 학습 AI 확장 로드맵

```
데이터 플라이휠:
사용자 발음 녹음 → 모델 평가 → 피드백/재시도 → 데이터 축적 → 모델 재학습 → 정확도 향상 → 사용자 증가 → ...
```

1. **발음 평가 AI** (Phase 14) — 음소별 정확도 판정
2. **학습자 프로파일링** — L1 기반 취약 음소 자동 파악 + 개인별 반복 오류 패턴 추적
3. **맞춤형 학습 경로** — 취약 부분 자동 반복 출제, 조음 영상 재안내
4. **20개 언어별 특화 모델** — L1 간섭 오류 집중

**핵심 차별점**: 교재(콘텐츠) + AI(개인화) 조합 → 데이터가 쌓일수록 격차 벌어짐 → 진입 장벽

> **상세 조사**: `memory/pronunciation_ai_research.md`, `memory/articulation_animation_research.md`

### 8.4 상시 모니터링 항목

프로젝트 전반에 걸쳐 지속적으로 수행해야 하는 조사·분석·모니터링 활동.

| # | 항목 | 분류 | 내역 | 주기 | 참고 |
|:-:|------|------|------|:----:|------|
| 1 | 한국어 교육 시장 조사 | 시장 | 경쟁사 동향 (신규 앱, 가격 변동, 기능 출시), TOPIK 응시자/학습자 통계, 시장 규모 업데이트 | 월 1회 | [`AMK_MARKET_ANALYSIS.md`](./AMK_MARKET_ANALYSIS.md) |
| 2 | 교육 앱 UX/UI 트렌드 | 시장 | 주요 교육 앱 UI 변화, 온보딩 플로우, 게이미피케이션 패턴, 접근성 트렌드 | 월 1회 | — |
| 3 | 결제/수익 모델 동향 | 시장 | Apple/Google IAP 정책 변경, 수수료율 변동, 지역별 가격 전략, 프로모션 사례 | 분기 1회 | [`AMK_MARKET_ANALYSIS.md §4`](./AMK_MARKET_ANALYSIS.md#4-모바일-앱-결제-전략) |
| 4 | AI/ML 기술 동향 | 기술 | LLM 경량화 (BitNet 후속), 음성인식 (Whisper 후속), 온디바이스 AI SDK, 발음 평가 API | 월 1회 | [`AMK_PIPELINE.md §11`](./AMK_PIPELINE.md) |
| 5 | 모바일 프레임워크 동향 | 기술 | React Native / SwiftUI / Kotlin Multiplatform 변화, 크로스플랫폼 AI 통합 사례 | 분기 1회 | — |
| 6 | 인프라/보안 동향 | 기술 | AWS 신규 서비스, 컨테이너 오케스트레이션, 인증 표준 (Passkey 등), OWASP 업데이트 | 분기 1회 | [`AMK_DEPLOY_OPS.md`](./AMK_DEPLOY_OPS.md) |
| 7 | 규제/법률 동향 | 사업 | 교육 앱 개인정보보호 (COPPA, GDPR-K), DMA/DSA 후속 조치, 각국 앱스토어 규제 | 분기 1회 | — |

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

### 8.5 Paddle Live 전환 체크리스트

Sandbox → Live(프로덕션) 전환을 위한 단계별 체크리스트. 코드 변경 불필요 — 환경변수만 교체.

> **참고 문서**: [Go-live checklist (Paddle Developer)](https://developer.paddle.com/build/onboarding/go-live-checklist)

#### Step 1: Paddle 대시보드 작업 (수동)

| # | 작업 | 상세 | 상태 |
|:-:|------|------|:----:|
| 1 | 계정 인증 (Account Verification) | Paddle Live 계정 사업자 인증 — KYB 서류 심사 완료, Identity Verification (Onfido) 완료, 계정 활성화 대기 중 (1~3 영업일) | ✅ |
| 2 | 도메인 인증 (Domain Verification) | `amazingkorean.net` 도메인 승인 요청 (Dashboard > Checkout > Request domain approval) | ⬜ |
| 3 | 상품 생성 (Product) | Live 계정에서 "Amazing Korean" 상품 새로 생성 (Sandbox 복사 X) | ⬜ |
| 4 | 가격 생성 (Prices) | 4개 플랜: 1m/$10, 3m/$25, 6m/$50, 12m/$100 — USD, 자동갱신, 1일 무료 체험 | ⬜ |
| 5 | API Key 발급 | Live 계정에서 API Key 생성 (형식: `pdl_live_apikey_...`) | ⬜ |
| 6 | Client Token 발급 | Live 계정에서 Client-side Token 생성 (형식: `live_...`) | ⬜ |
| 7 | Webhook 설정 | Notification Destination 생성: URL `https://api.amazingkorean.net/payment/webhook`, 이벤트: `subscription.*` + `transaction.completed` | ⬜ |
| 8 | 결제수단 확인 | 카드(기본 ON) + PayPal/Apple Pay/Google Pay/KakaoPay/NaverPay 등 자동 제공 확인 | ⬜ |
| 9 | 통화/세금 설정 | Balance Currency: USD, 세금: Paddle MoR 자동 처리 (별도 설정 불필요) | ⬜ |

#### Step 2: GitHub Secrets 교체 (수동)

| Secret | Sandbox 값 | Live 값 |
|--------|-----------|---------|
| `PADDLE_SANDBOX` | `true` | `false` |
| `PADDLE_API_KEY` | `pdl_sdbx_apikey_...` | 새 발급 (`pdl_live_apikey_...`) |
| `PADDLE_CLIENT_TOKEN` | `test_...` | 새 발급 (`live_...`) |
| `PADDLE_WEBHOOK_SECRET` | `pdl_ntfset_...` | 새 발급 |
| `PADDLE_PRODUCT_ID` | `pro_01khg4n...` | 새 Live Product ID |
| `PADDLE_PRICE_MONTH_1` | `pri_01khg4r...` | 새 Live Price ID |
| `PADDLE_PRICE_MONTH_3` | `pri_01khg4s...` | 새 Live Price ID |
| `PADDLE_PRICE_MONTH_6` | `pri_01khg4t...` | 새 Live Price ID |
| `PADDLE_PRICE_MONTH_12` | `pri_01khg4w...` | 새 Live Price ID |

#### Step 3: 배포 및 검증

1. GitHub Secrets 교체 후 CI/CD 배포 (main push 또는 workflow_dispatch)
2. 서버 로그 확인: `💳 Payment provider enabled: Paddle Billing` (**Sandbox 미표시**)
3. 테스트 결제 수행 → Webhook 수신 확인 (`webhook_events` 테이블)
4. 구독 활성화 → `user_course` 자동 부여 확인

#### 가격 변경 전략 (추후)

현재 가격은 샘플. 콘텐츠 담당자와 최종 확정 후 변경 예정.

- **방법**: 새 Price 객체 생성 (Paddle 대시보드) → `PADDLE_PRICE_MONTH_*` Secrets 업데이트 → 재배포
- **기존 구독자**: 이전 Price로 자동 유지 (영향 없음), 신규 구독자만 새 가격 적용
- **기존 구독자도 변경 시**: Subscription Update API (`proration_billing_mode` 설정)
- **코드 변경 불필요** — 환경변수만 교체

#### 결제수단 / 세금 참고

**결제수단**: Paddle 기본 수수료(5% + $0.50/건)에 **모든 결제수단 포함** (추가 비용 없음)
- 카드(Visa/MC/Amex), PayPal, Apple Pay, Google Pay, KakaoPay, NaverPay, Samsung Pay, Alipay, UnionPay, iDEAL 등
- 고객 위치/기기/통화에 따라 Paddle이 자동으로 적절한 결제수단 표시

**세금**: Paddle MoR(Merchant of Record)가 100개국+ **자동 처리**
- VAT (EU/UK), GST (호주/인도), Sales Tax (미국 주별) — 자동 계산 + 징수 + 납부
- 세금 관련 법적 책임도 Paddle에 귀속
- 설정: "Prices include tax" OFF 권장 (표시 가격 + 세금 별도)

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

### 8.6 학습 콘텐츠 보안 전략 (Content Protection)

> 자체 플랫폼(웹/앱)의 학습 콘텐츠와 외부 유통 EPUB3에 대한 3중 보안 아키텍처.

#### 유통 채널별 전략

| 채널 | 형식 | 보안 방식 |
|------|------|----------|
| **자체 웹/앱** | 인터랙티브 학습 플랫폼 (DB → API → 렌더링) | 구조적 보안 + 포렌식 마킹 + 플랫폼 보안 |
| **외부 플랫폼** (Amazon/Apple/Google/Kobo) | EPUB3 | 플랫폼 DRM 자동 적용 |

- 자체 사이트에서는 EPUB3 파일을 제공하지 않음 — **다운로드 가능한 파일 자체가 존재하지 않음**
- 교재 콘텐츠(JSON 데이터)를 서버에서 인터랙티브 학습 콘텐츠로 동적 렌더링
- EPUB3는 외부 플랫폼 유통 전용으로만 생성

#### Layer 1 — 구조적 보안 (콘텐츠 제공 방식 자체가 방어)

| 방어 요소 | 설명 |
|----------|------|
| 파일 없음 | 다운로드할 파일이 존재하지 않음 (EPUB3/PDF 미제공) |
| 서버 렌더링 | 콘텐츠가 DB → API → 조각 단위 전송, 전체 데이터 한 번에 노출 안 됨 |
| 페이지/단원 단위 접근 | 요청한 부분만 내려줌, 전체 책을 한 번에 수집 불가 |
| 인증 + 구독 확인 | 로그인 + 활성 구독 상태에서만 접근 가능 |
| 데이터 분해 | 어휘/문장/활용/조사가 개별 API 응답으로 분리 — 크롤링해도 원본 교재 형태로 재조립 극히 어려움 |

#### Layer 2 — 포렌식 워터마킹 (유출 시 추적, 실시간 동적 생성)

API 응답마다 구매자별 고유 마킹 패턴을 **실시간 동적 생성** — 세션마다 패턴이 바뀌어 패턴 분석 자체가 불가능.

| 층 | 기법 | 추적 대상 | 제거 난이도 | 비고 |
|:--:|------|----------|:---------:|------|
| 1 | **ZWC (Zero-Width Character) 삽입** | 텍스트 복사 | 쉬움 | 유니코드 영폭 문자로 구매자 ID 인코딩 |
| 2 | **동형 문자(Homoglyph) 치환** | ZWC 제거 후에도 생존 | 중간 | 육안 구분 불가 유니코드 문자 교체 |
| 3 | **이미지 LSB 삽입** | 이미지 캡처 | 어려움 | 픽셀 최하위 비트에 데이터 삽입 |
| 4 | **CSS 미세 변형** | 화면 캡처 | 어려움 | letter-spacing/color 0.01px/0.01% 단위 차이 |

- 인터랙티브 플랫폼 장점: EPUB3는 구매 시 1회 마킹이지만, 플랫폼에서는 **매 API 요청마다 패턴 변경 가능**
- 마킹 정보: 구매자 ID + 타임스탬프 + IP + 기기 + 세션 ID
- 4개 레이어 중 **하나만 살아남아도 법적 추적 근거 확보**
- "마킹이 되어있다"는 사실 자체가 심리적 유출 억제 효과

#### Layer 3 — 플랫폼 보안 (기존 인프라 활용)

| 환경 | 보안 조치 |
|------|----------|
| **웹** | 우클릭 차단, 텍스트 선택 차단 (`user-select: none`), 개발자도구 감지, 인쇄 차단 |
| **앱 (iOS/Android)** | 스크린샷/화면 녹화 차단 API, 암호화 캐시, 오프라인 읽기 시 암호화 유지 |
| **외부 플랫폼** | Amazon/Apple/Google/Kobo 자체 DRM 자동 적용 (별도 설정 불필요) |

#### 보안 한계 인식

- **어떤 DRM도 100% 완벽하지 않음** — 넷플릭스도 뚫림
- 핵심은 "완벽한 차단"이 아니라 **"비용 대비 효과"** — 대다수에게 충분히 어렵게 만드는 것
- 3중 방어 구조에서 콘텐츠를 완전히 탈취하려면: 인증 돌파 → 조각 데이터 전수 크롤링 → 원본 재조립 → 4중 마킹 전부 제거 필요
- ₩25,000 교재를 이 수준으로 뚫으려는 동기 대비 방어 비용이 충분히 합리적

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 9. 변경 이력

> 상세 변경 이력은 [`AMK_CHANGELOG.md`](./AMK_CHANGELOG.md)로 분리됨 (2026-02-17).

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

**문서 끝 (End of Document)**
