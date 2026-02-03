---
title: AMK_API_MASTER — Amazing Korean API  Master Spec
updated: 2026-02-03
owner: HYMN Co., Ltd. (Amazing Korean)
audience: server / database / backend / frontend / lead / LLM assistant
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
  - [0.4 LLM 협업 가이드](#04-llm-협업-가이드)

- [1. 프로젝트 개요 & 목표](#1-프로젝트-개요--목표)
  - [1.1 서비스 개요](#11-서비스-개요)
  - [1.2 비즈니스 흐름 (Business Logic)](#12-비즈니스-흐름-business-logic)

- [2. 시스템 & 개발 환경 개요](#2-시스템--개발-환경-개요)
  - [2.1 런타임 / 스택](#21-런타임--스택)
  - [2.2 라우팅 & OpenAPI](#22-라우팅--openapi)
  - [2.3 로컬 개발 & 실행](#23-로컬-개발--실행)

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
  - [4.6 향후 업데이트 도메인](#46-향후-업데이트-도메인)

- [5. 기능 & API 로드맵 (Phase / 화면 / 엔드포인트 / 상태 / DoD)](#5-기능--api-로드맵-phase--화면--엔드포인트--상태--dod)
  - [5.0 Phase 로드맵 체크박스 범례](#50-phase-로드맵-체크박스-범례)
  - [5.1 Phase 1 — health](#51-phase-1--health-)
  - [5.2 Phase 2 — user](#52-phase-2--user-)
  - [5.3 Phase 3 — auth](#53-phase-3--auth-)
  - [5.4 Phase 4 — video](#54-phase-4--video-)
  - [5.5 Phase 5 — study](#55-phase-5--study-)
  - [5.6 Phase 6 — lesson](#56-phase-6--lesson-)
  - [5.7 Phase 7 — admin](#57-phase-7--admin-)
  - [5.8 Phase 8 — scripts](#58-phase-8--scripts)

- [6. 프론트엔드 구조 & 규칙](#6-프론트엔드-구조--규칙)
  - [6.1 프론트엔드 스택 & 기본 원칙](#61-프론트엔드-스택--기본-원칙)
  - [6.2 프론트 디렉터리 구조 & 컴포넌트 계층](#62-프론트-디렉터리-구조--컴포넌트-계층)
  - [6.3 라우팅 & 접근 제어](#63-라우팅--접근-제어)
  - [6.4 상태 관리 & API 연동 패턴](#64-상태-관리--api-연동-패턴)
  - [6.5 UI/UX & Tailwind 규칙 (shadcn/ui System)](#65-uiux--tailwind-규칙-shadcnui-system)
  - [6.6 프론트 테스트 & 빌드/배포 (요약)](#66-프론트-테스트--빌드배포-요약)
    - [6.6.2-1 Cloudflare Pages 배포 (프론트엔드)](#6621-cloudflare-pages-배포-프론트엔드)
    - [6.6.2-2 AWS EC2 배포 (백엔드)](#6622-aws-ec2-배포-백엔드)
    - [6.6.2-3 GitHub Actions CI/CD 파이프라인](#6623-github-actions-cicd-파이프라인)
    - [6.6.2-4 EC2 유지보수 가이드](#6624-ec2-유지보수-가이드)

- [7. 작업 방식 / 엔지니어링 가이드 (요약)](#7-작업-방식--엔지니어링-가이드-요약)
  - [7.1 작업 원칙](#71-작업-원칙)
  - [7.2 개발 플로우](#72-개발-플로우)
  - [7.3 DTO/검증 규칙 (요약)](#73-dto검증-규칙-요약)
  - [7.4 서비스 계층 및 파일 구조](#74-서비스-계층-및-파일-구조)
  - [7.5 트랜잭션 패턴](#75-트랜잭션-패턴)
  - [7.6 테스트 & 자동화](#76-테스트--자동화)
  - [7.7 코드 예시 (Best Practices)](#77-코드-예시-best-practices)
    - [7.7.1 백엔드 패턴 (Rust/Axum)](#771-백엔드-패턴-rustaxum)
      - [7.7.1-0. 공용 코드 (Common Code)](#771-0-공용-코드-common-code)
      - [7.7.1-1. dto.rs](#771-1-dtors)
      - [7.7.1-2. repo.rs](#771-2-repors)
      - [7.7.1-3. service.rs](#771-3-servicers)
      - [7.7.1-4. handler.rs](#771-4-handlerrs)
      - [7.7.1-5. router.rs](#771-5-routerrs)
      - [7.7.1-6. 기타 파일들](#771-6-기타-파일들-auth-유틸리티)
    - [7.7.2 프론트엔드 패턴 (React/TypeScript)](#772-프론트엔드-패턴-reacttypescript)
      - [7.7.2-1. types.ts (Zod 스키마 & 타입 정의)](#772-1-typests-zod-스키마--타입-정의)
      - [7.7.2-2. *_api.ts (API 함수)](#772-2-_apits-api-함수)
      - [7.7.2-3. hook/*.ts (TanStack Query 훅)](#772-3-hookts-tanstack-query-훅)
      - [7.7.2-4. page/*.tsx (페이지 컴포넌트)](#772-4-pagetsx-페이지-컴포넌트)
      - [7.7.2-5. 공용 인프라 (Shared Infrastructure)](#772-5-공용-인프라-shared-infrastructure)
      - [7.7.2-6. 프론트엔드 데이터 흐름 (Data Flow)](#772-6-프론트엔드-데이터-흐름-data-flow)

- [8. LLM 협업 규칙 (나와 일하는 법)](#8-llm-협업-규칙-나와-일하는-법)
  - [8.1 질문/요청 방식](#81-질문요청-방식)
  - [8.2 LLM 응답 기대 형식](#82-llm-응답-기대-형식)
  - [8.3 LLM_PATCH_TEMPLATE 연동](#83-llm_patch_template-연동)
  - [8.4 표준 작업 절차 (SOP)](#84-표준-작업-절차-sop-standard-operating-procedure)

- [9. Open Questions & 설계 TODO](#9-open-questions--설계-todo)
  - [9.1 RBAC / 관리자 권한](#91-rbac--관리자-권한)
  - [9.2 Admin action log actor 연결](#92-admin-action-log-actor-연결)
  - [9.3 페이징 고도화 (Keyset vs Page)](#93-페이징-고도화-keyset-vs-page)
  - [9.4 테스트 전략](#94-테스트-전략)
  - [9.5 보안/운영 (후순위 계획)](#95-보안운영-후순위-계획)
  - [9.6 코드 일관성 (Technical Debt)](#96-코드-일관성-technical-debt)
  - [9.7 추후 작업 항목 (문서 내 TODO 통합)](#97-추후-작업-항목-문서-내-todo-통합)
  - [9.8 LLM 협업 도구 전환](#98-llm-협업-도구-전환)
  - [9.9 인프라 로드맵 (RDS 이전)](#99-인프라-로드맵-rds-이전)
  - [9.10 데이터 모니터링 & 접근](#910-데이터-모니터링--접근)
  - [9.11 디자인 & UI](#911-디자인--ui)

- [10. 변경 이력 (요약)](#10-변경-이력-요약)

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 0. 문서 메타 & 사용 방법

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

### 0.1 목적

- Amazing Korean server / database / backend / frontend / web&app 대한:
  - **기능 & API 로드맵 (Phase / 화면 / 엔드포인트 / 완료 상태)**
  - **공통 규칙 (에러 / 시간 / 인증 / 페이징 / 응답 래퍼 등)**
  - **개발 / 작업 방식 (엔지니어링 가이드)**
  - **LLM 협업 규칙 (패치 방식, 템플릿 사용법)**
  - **Open Questions & 설계 TODO**
- 을 한 파일에서 관리하기 위함.

### 0.2 사용 원칙

- **스펙 / 기능 / 엔드포인트를 변경할 때는 항상 이 파일을 먼저 수정**한다.
- 코드/마이그레이션/테스트를 변경한 뒤에는, 여기의 관련 섹션(Phase 표, 규칙, TODO)을 반드시 갱신한다.
- 과거 md 문서들은 모두 **참고용 아카이브**이며, 새로운 정보는 **여기에만 적는다**.

### 0.3 관련 파일

- **데이터베이스 스키마**: [`docs/AMK_SCHEMA_PATCHED.md`](./AMK_SCHEMA_PATCHED.md) - 전체 DDL 정의
- **LLM 협업 템플릿**:
  - 백엔드: [`docs/patchs/LLM_PATCHS_TEMPLATE_BACKEND.md`](./patchs/LLM_PATCHS_TEMPLATE_BACKEND.md)
  - 프론트엔드: [`frontend/docs/LLM_PATCHS_TEMPLATE_FRONTEND.md`](../frontend/docs/LLM_PATCHS_TEMPLATE_FRONTEND.md)
- (선택) 이 문서는 레포 내 `docs/AMK_API_MASTER.md` 경로에 위치하는 것을 기본으로 한다.

### 0.4 LLM 협업 가이드

> 이 문서를 LLM(Gemini, Claude, Codex 등)과 협업 시 활용하는 방법

#### 전체 컨텍스트 로딩

- **프롬프트 시작**: "첨부된 `AMK_API_MASTER.md`를 참조하여..."
- 이 문서 하나로 전체 프로젝트 아키텍처, 규칙, API 명세를 파악 가능
- 문서 크기: 약 2,300줄 (Gemini 2.0, Claude Sonnet 등 최신 LLM은 충분히 처리 가능)

#### 섹션별 참조 방법

**백엔드 작업 시**:
- **Section 2**: 시스템 & 개발 환경 (스택, 라우팅, OpenAPI)
- **Section 3**: 공통 규칙 (네이밍, 에러, 인증, 페이징)
- **Section 4**: 데이터 모델 개요
- **Section 5**: 기능 & API 로드맵 (Phase별 엔드포인트 명세)
- **Section 7**: 엔지니어링 가이드 (파일 구조, 작업 원칙, 트랜잭션)

**프론트엔드 작업 시**:
- **Section 2**: 시스템 & 개발 환경 (프론트 스택)
- **Section 3**: 공통 규칙 (상태축, 에러 처리)
- **Section 6**: 프론트엔드 규칙 (디렉토리, 라우팅, 상태 관리, UI 패턴)

**신규 API 추가 시**:
- **Section 3.2**: 네이밍 규칙 (테이블, 컬럼, 엔드포인트, DTO)
- **Section 5.x**: Phase 로드맵에 추가
- **Section 7.4**: 파일 구조 (어디에 코드를 추가할지)

#### 외부 상세 파일 (구현 시 추가 참조)

- **백엔드 패치 작업**: [`docs/patchs/LLM_PATCHS_TEMPLATE_BACKEND.md`](./patchs/LLM_PATCHS_TEMPLATE_BACKEND.md)
  - MCP 액션, 구현 단계, 패치 규칙, SOP(Standard Operating Procedure)
  - Full File Replacement 원칙, Compile-Ready 요구사항
- **프론트엔드 패치 작업**: [`frontend/docs/LLM_PATCHS_TEMPLATE_FRONTEND.md`](../frontend/docs/LLM_PATCHS_TEMPLATE_FRONTEND.md)
  - types.ts ReadOnly 원칙, 5-Step 프로세스, Iron Rules
  - No Type Hallucinations, DTO 1:1 매핑, No Silent Failures
- **데이터베이스 스키마**: [`docs/AMK_SCHEMA_PATCHED.md`](./AMK_SCHEMA_PATCHED.md)
  - 전체 테이블 DDL, ENUM 타입, 인덱스, 제약조건

#### LLM 프롬프트 템플릿 예시

**백엔드 엔드포인트 추가**:
```
첨부된 AMK_API_MASTER.md를 참조하여 다음 작업을 수행해줘:

1. Section 3.2 네이밍 규칙 준수
2. Section 7.7의 "보호된 엔드포인트 + 트랜잭션 패턴" 사용
3. Section 3.4 에러 응답 스키마 준수
4. docs/patchs/LLM_PATCHS_TEMPLATE_BACKEND.md의 체크리스트 확인

작업: GET /courses/{id}/lessons 엔드포인트 추가
- Phase: 5.6 (신규)
- 인증: 필요 (Claims 사용)
- 응답: LessonListDto (페이지네이션)
```

**프론트엔드 페이지 생성**:
```
첨부된 AMK_API_MASTER.md를 참조하여 다음 작업을 수행해줘:

1. Section 6.2 디렉토리 구조 준수
2. Section 6.4 상태 관리 (6개 축) 적용
3. Section 7.7의 "보호된 페이지 + 상태축 체크 패턴" 사용
4. frontend/docs/LLM_PATCHS_TEMPLATE_FRONTEND.md의 체크리스트 확인

작업: /courses/[id] 페이지 생성
- 요구사항: 강좌 상세 + 강의 목록 표시
- 상태축: auth=pass, course=buy 체크 필요
- API: GET /courses/{id}, GET /courses/{id}/lessons
```

#### LLM 협업 시 주의사항

1. **Full File Replacement 원칙**: LLM에게 코드 수정을 요청할 때는 항상 "전체 파일 내용을 출력"하도록 명시
   - ❌ `// ... existing code` 주석으로 생략
   - ✅ 파일 전체를 처음부터 끝까지 출력

2. **SSOT 우선순위**: 이 문서(`AMK_API_MASTER.md`)가 항상 최우선 참조
   - 다른 문서와 충돌 시 이 문서를 정답으로 간주
   - 코드와 문서가 다를 경우 이 문서 기준으로 코드 수정

3. **타입 일관성**:
   - 프론트엔드: `types.ts`에 정의된 타입만 사용 (새로 만들지 말 것)
   - 백엔드: `amk_schema_patched.sql`의 스키마와 일치

4. **에러 처리 필수**:
   - Silent Failure 금지
   - 사용자에게 명확한 피드백 제공 (toast, 에러 페이지 등)

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
  - `src/api/docs.rs` (예: `ApiDoc`)
  - Swagger UI: `GET /docs`
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

## 3. 공통 규칙 (전역 컨벤션)

### 3.1 시간/타임존

- DB의 시간 컬럼(특히 로그/이력)은:
  - 타입: `TIMESTAMPTZ`
  - 기본값: `DEFAULT now()` (UTC)
- 클라이언트(웹/앱)에선 KST or 로컬 타임존으로 변환하여 표시.

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

- HTTP 헤더:
  - `Authorization: Bearer <ACCESS_TOKEN>`
    - 인증 필요한 모든 엔드포인트에 필수
  - `Content-Type: application/json`
    - 요청 본문이 JSON일 때
  - `Accept: application/json`
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
  - 수명: **1시간** (3600초)
  - 전송 방식: `Authorization: Bearer <ACCESS_TOKEN>` 헤더
  - 페이로드 구조:
    ```json
    {
      "sub": "<user_id>",       // i64 - 사용자 ID
      "role": "<user_auth>",    // "HYMN" | "admin" | "manager" | "learner"
      "session_id": "<uuid>",   // 세션 식별자 (로그아웃 시 무효화용)
      "iss": "amazing-korean",  // 발급자 식별
      "exp": 1234567890,        // Unix timestamp (1시간 후)
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
  2. 액세스 토큰 생성 (JWT, 1시간)
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
  3. **새 액세스 토큰 생성** (JWT, 1시간)
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
- `users_log`
  - 회원 정보 활동 기록
  - `user_action_log_enum` ('signup', 'find_id', 'reset_pw', 'update') 사용자 활동 이력
  - `user_auth_enum` ('HYMN', 'admin', 'manager', 'learner') 사용자 권한 이력
  - `user_language_enum` ('ko', 'en') 사용자 구사 언어 이력
  - `user_gender_enum` ('none', 'male', 'female', 'other') 사용자 성별 이력
- `users_setting`
  - 사용자 관련 UI 언어, 알림 등 개인 설정
  - `user_set_language_enum` ('ko', 'en') 사용자 설정 언어
- `admin_users_log`
  - 사용자 관련 관리자 활동 기록
  - `admin_action_enum` ('create', 'update', 'banned', 'reorder', 'publish', 'unpublish') 관리자 활동 이력
- `user_export_data`
  - 개인정보 내보내기/백업 요청 상태 및 결과 관리(비동기 처리용)

### 4.2 인증 로그인 도메인 (AUTH LOGIN)

- `login`
  - 로그인 정보(지역, 방식, 시간, 상태)
  - `login_device_enum` ('mobile', 'tablet', 'desktop', 'other') 로그인 기기
  - `login_method_enum` ('email', 'google', 'apple') 로그인 방법
  - `login_state_enum` ('active', 'revoked', 'expired', 'logged_out', 'compromised') 로그인 상태
- `login_log`
  - 로그인 정보 활동 이력(로그인 이벤트, 세부 지역, 세부 방식)
  - `login_event_enum` ('login', 'logout', 'refresh', 'rotate', 'fail', 'reuse_detected') 로그인 활동 이력
  - `login_device_enum` ('mobile', 'tablet', 'desktop', 'other') 로그인 기기 이력
  - `login_method_enum` ('email', 'google', 'apple') 로그인 방법 이력
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

### 4.6 향후 업데이트 도메인

- `pay`
  - 결제 : 사용자 결제 관련 테이블, 결제 후 콘텐츠 이용 가능
  - `pay_state` ('ready', 'done', 'cancel')
- `course`
  - 결제 맵핑 : 결제 후 `course`와 `lesson`를 맵핑해 콘텐츠 이용 진행
  - `course_type` ('video', 'study', 'live', 'package')
  - `course_state` ('active', 'inactive', 'deleted')
- `course_video` / `course_live`
  - 코스 구성 맵핑 테이블
- `live`
  - 실시간 강의 : ZOOM API 연동을 통한 실시간 강의 서비스 관련 테이블
  - `live_state` ('ready', 'open', 'close')
- `live_zoom`
  - 줌 연동 정보
  - `live_zoom_state` ('pending', 'registered', 'failed')
- `live_log`
  - 라이브 강의 참여 로그

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
  - Then: `200 OK`, JSON 바디 `{"status":"live","uptime_ms":..., "version":"v0.1.0"}`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→success / Data=present
- **실패**
  - When: 헬스 핸들러 내부 예외
  - Then: `500 Internal Server Error`, 에러 바디 `{"error":{"http_status":500,"code":"HEALTH_INTERNAL"}}`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→error / Data=error

---

#### 5.1-2 : `GET /docs` 시나리오
- **성공**
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
  - Then: **201**, `Location: /users/{id}`(권장)
    - **Body**: `SignupRes` (안전한 유저 정보 + **Access Token**, `session_id`)
    - **Cookie**: `ak_refresh` (**Refresh Token**, HttpOnly, Secure)
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` pristine→dirty→validating→submitting→success** / Request=`signup` pending→success / Data=`signup` present
  - 로그: USERS insert 후 **USERS_LOG(성공 스냅샷)** 기록(민감정보 제외)
- **실패(형식/누락) → 400 Bad Request**
  - 예: 이메일 형식 불일치, 필수 항목 누락, JSON 파싱 오류
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.client** / Request=`signup` pending→error / **Data=`signup` empty**
  - 에러 바디: `{ "error": { "http_status": 400, "code": "BAD_REQUEST", "message": "...", "trace_id": "..." } }`
  - 로그: **USERS_LOG(실패 이벤트)** 기록(에러코드/사유, 민감값 마스킹)
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: birthday 범위 위반, 금지값, 정책 규칙 위반
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.client** / Request=`signup` pending→error / **Data=`signup` error**
  - 에러 바디: `http_status:422, code:"UNPROCESSABLE_ENTITY"`
  - 로그: 실패 이벤트 기록
- **실패(중복/충돌) → 409 Conflict**
  - 예: 이메일 UNIQUE 충돌
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
  - 백엔드: 시간/조건 기반 중복 생성 방지(최근 N분 동일 이메일 재시도 시 409 또는 200 재응답 정책 중 택1)

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
| 3-3 | `POST /auth/refresh` | (전역처리) | 토큰 재발급 | ***리프레시 로테이션/검증/재사용탐지 + 로그(rotate)***<br>성공: Auth pass / Page app ready / Request refresh pending→success / Data refresh present → **200**<br>실패(형식/누락): Auth pass / Page app ready / Request refresh pending→error / Data refresh empty → **400**<br>실패(도메인 제약): Auth pass / Page app ready / Request refresh pending→error / Data refresh error → **422**<br>실패(리프레시 무효/만료): Auth stop / Page app ready / Request refresh pending→error / Data refresh error → **401**<br>실패(재사용탐지/위조): Auth forbid / Page app ready / Request refresh pending→error / Data refresh error → **409**(또는 **403**) | [✅🆗] |
| 3-4 | `POST /auth/find-id` | `/find-id` | 회원 아이디 찾기 | ***개인정보 보호: 결과 폭로 금지(Enumeration Safe), USERS_LOG 저장***<br>성공(요청 수락/존재 여부와 무관):<br> Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→submitting→success / Request find_id pending→success / Data find_id present → **200**(항상 동일 메시지)<br>실패(형식/누락): Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→error.client / Request find_id pending→error / Data find_id empty → **400**<br>실패(도메인 제약): Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→error.client / Request find_id pending→error / Data find_id error → **422**<br>실패(레이트리밋): Auth pass / Page find_id ready / Form find_id error.client / Request find_id pending→error / Data find_id error → **429** | [✅🆗] |
| 3-5 | `POST /auth/reset-pw` | `/reset-password` | 회원 비밀번호 재설정 | ***요청→검증→재설정의 단일 엔드포인트(토큰/코드 포함), USERS_LOG 저장***<br>성공(재설정 완료):<br> Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→submitting→success / Request reset_pw pending→success / Data reset_pw present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→error.client / Request reset_pw pending→error / Data reset_pw empty → **400**<br>실패(도메인 제약): Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→error.client / Request reset_pw pending→error / Data reset_pw error → **422**<br>실패(토큰/코드 무효·만료): Auth stop / Page reset_pw ready / Form reset_pw error.client / Request reset_pw pending→error / Data reset_pw error → **401**<br>실패(레이트리밋): Auth pass / Page reset_pw ready / Form reset_pw error.client / Request reset_pw pending→error / Data reset_pw error → **429** | [✅🆗] |
| 3-6 | `GET /auth/google`<br>`GET /auth/google/callback` | `/login` | Google OAuth 로그인 | ***Google OAuth 2.0 Authorization Code Flow, 자동 계정 연결/생성, USER_OAUTH/LOGIN/LOGIN_LOG 저장***<br>성공(OAuth 시작): Auth pass / Page login ready / Request google pending→success / Data google_auth_url present → **200**<br>성공(OAuth 콜백): Auth pass / Page login redirect→ready / Request callback pending→success / Data login present → **302**(프론트엔드 리다이렉트)<br>실패(OAuth 설정 누락): Auth pass / Page login ready / Request google pending→error / Data google error → **500**<br>실패(State 검증 실패/CSRF): Auth stop / Page login ready / Request callback pending→error / Data callback error → **401**<br>실패(사용자 취소): Auth pass / Page login ready / Request callback pending→error / Data callback error → **302**(에러 정보와 함께 리다이렉트) | [✅🆗] |

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
  - Then: **401**, `{ "error": { "code": "AUTH_401_SOCIAL_ONLY_ACCOUNT", "providers": ["google"] } }`
  - 프론트엔드 처리: 소셜 로그인 유도 UI 표시 (amber 색상 안내 박스 + Google 로그인 버튼)
  - 상태축: Auth=stop / Form error.client / Data error (socialOnlyError)

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
- **Audience 검증**: ID Token의 aud가 client_id와 일치해야 함

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
    3. ID Token 디코딩 및 검증 (nonce, aud, exp)
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

### 5.8 Phase 8 — scripts
| 번호 | 작업 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|
| 8-1 | Docker/ENV | 로컬/배포 스크립트 | 일관된 `up/run` 스크립트화 | [ ] |
| 8-2 | Migration | DB 초기화/업데이트 | `sqlx migrate run` 표준화 | [ ] |
| 8-3 | Smoke | cURL/K6 스모크 | 성공·실패 1케이스 자동화 | [ ] |

---

### 비고
- 코스(Course)는 후순위. ERD 정리 후 별도 Phase로 추가 예정.
- 모든 Phase는 "**백엔드 엔드포인트 구현 → 프론트 1화면 연동 → 스모크(성공+대표 에러)**" 순으로 완료 표시.

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

  hooks/                 # 전역 Custom Hook
    use_auth.ts          # 인증 상태 관리 (Zustand + Logic)
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

### 6.6 프론트 테스트 & 빌드/배포 (요약)

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

#### 6.6.2 빌드 & 배포 전략

- **빌드 커맨드 (Strict)**
  - `npm run build` 실행 시:
    1.  `tsc -b` (TypeScript 컴파일 검사)가 먼저 실행되어야 한다. **타입 에러 발생 시 빌드는 실패해야 한다.**
    2.  Vite가 프로덕션용 최적화(Minify, Tree Shaking)를 수행하고 `dist/` 폴더를 생성한다.

- **SPA 서빙 전략 (SPA Fallback)**
  - 프론트엔드는 **Single Page Application**이므로, **모든 404 요청을 `index.html`로 리다이렉트**해야 한다.
  - **Nginx 배포 시**: `try_files $uri $uri/ /index.html;` 설정 필수.
  - **Rust(Axum) 통합 배포 시**: 정적 파일 서빙 핸들러에서 Fallback 경로 설정 필요.

#### 6.6.2-0 도메인 및 DNS 설정 (Route 53)

- **도메인**: `amazingkorean.net`
- **DNS 관리**: AWS Route 53

##### DNS 레코드 설정

| 레코드 타입 | 이름 | 값 | TTL |
|------------|------|-----|-----|
| CNAME | amazingkorean.net | amazing-korean-api.pages.dev | 300 |
| CNAME | www | amazing-korean-api.pages.dev | 300 |
| A | api | 3.39.234.157 | 300 |

##### 서비스 URL

| 서비스 | URL |
|--------|-----|
| 프론트엔드 | https://amazingkorean.net |
| 프론트엔드 (www) | https://www.amazingkorean.net |
| 백엔드 API | https://api.amazingkorean.net |
| Cloudflare Pages | https://amazing-korean-api.pages.dev |

#### 6.6.2-1 Cloudflare Pages 배포 (프론트엔드)

- **배포 플랫폼**: Cloudflare Pages
- **GitHub 연동**: `AmazingKoreanCenter/amazing-korean-api`
- **빌드 설정**:
  - Framework preset: `Vite`
  - Build command: `npm run build`
  - Build output directory: `dist`
  - Root directory: `frontend`
- **환경 변수**:
  - `VITE_API_BASE_URL`: `https://api.amazingkorean.net`
- **커스텀 도메인**:
  - `amazingkorean.net`
  - `www.amazingkorean.net`
- **SPA 라우팅**: Cloudflare Pages는 SPA Fallback을 자동 지원 (별도 설정 불필요)

#### 6.6.2-2 AWS EC2 배포 (백엔드)

- **EC2 인스턴스**: Amazon Linux 2023 또는 Ubuntu 22.04 LTS
- **Instance Type**: t2.micro (1 vCPU, 1GB) - 빌드 시 t3.medium 권장
- **Storage**: **최소 20GB gp3** (Rust 빌드에 필요, 8GB는 부족)
- **Public IP**: `43.200.180.110` (인스턴스 중지/시작 시 변경됨)
- **도메인**: `api.amazingkorean.net`
- **배포 방식**: Docker Compose
- **Nginx 설정**: 리버스 프록시 (80/443 → API:3000)
- **SSL**: Cloudflare Flexible (프록시 모드)
- **빌드 시간**: t2.micro에서 빌드 불가 (메모리 부족), t3.medium 권장

> **참고**: t2.micro (1GB RAM)는 Rust 빌드에 메모리가 부족합니다. 빌드 시 임시로 t3.medium으로 변경 후, 완료 후 다시 t2.micro로 변경하세요.

##### 환경 변수 (.env.prod)

```env
POSTGRES_PASSWORD=your-secure-password
JWT_SECRET=your-32-byte-minimum-secret-key
DOMAIN=api.amazingkorean.net
CORS_ORIGINS=http://localhost:5173,https://amazingkorean.net,https://www.amazingkorean.net
```

##### 0. SQLx 오프라인 모드 준비 (Docker 빌드 전 필수)

Docker 빌드 시 데이터베이스 연결 없이 SQLx 매크로를 컴파일하려면 `.sqlx` 캐시가 필요합니다.

```bash
# 로컬에서 PostgreSQL 실행 중인 상태에서
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# .sqlx 캐시 생성
cargo sqlx prepare

# Git에 커밋
git add .sqlx
git commit -m "Add SQLx offline cache"
git push
```

> **참고**: Dockerfile에 `ENV SQLX_OFFLINE=true`와 `COPY .sqlx ./.sqlx`가 설정되어 있어야 합니다.
> Rust 버전은 **1.85 이상** 필요 (edition2024 지원).

##### 1. EC2 인스턴스 준비

**Amazon Linux 2023 (권장)**

```bash
# 1. EC2 인스턴스 생성 (권장 사양)
# - OS: Amazon Linux 2023
# - Instance Type: t2.micro (프리티어) 또는 t3.small
# - Storage: 20GB gp3 (8GB는 Rust 빌드 시 디스크 부족 발생)
# - Security Group: 22(SSH), 80(HTTP), 443(HTTPS) 포트 오픈

# 2. SSH 접속 (Amazon Linux는 ec2-user 사용)
ssh -i your-key.pem ec2-user@your-ec2-ip

# 3. Git 설치 (Amazon Linux에는 기본 설치 안됨)
sudo yum install -y git

# 4. Docker 설치
sudo yum install -y docker
sudo systemctl start docker
sudo systemctl enable docker
sudo usermod -aG docker $USER

# 5. Docker Compose (Buildx) 설치
DOCKER_CONFIG=${DOCKER_CONFIG:-$HOME/.docker}
mkdir -p $DOCKER_CONFIG/cli-plugins
curl -SL https://github.com/docker/compose/releases/latest/download/docker-compose-linux-x86_64 \
  -o $DOCKER_CONFIG/cli-plugins/docker-compose
chmod +x $DOCKER_CONFIG/cli-plugins/docker-compose

# Buildx 설치 (compose build에 필요)
curl -SL https://github.com/docker/buildx/releases/download/v0.15.1/buildx-v0.15.1.linux-amd64 \
  -o $DOCKER_CONFIG/cli-plugins/docker-buildx
chmod +x $DOCKER_CONFIG/cli-plugins/docker-buildx

# 6. 로그아웃 후 재접속 (docker 그룹 적용)
exit
ssh -i your-key.pem ec2-user@your-ec2-ip
```

**Ubuntu 22.04 LTS (대안)**

```bash
# SSH 접속 (Ubuntu는 ubuntu 사용)
ssh -i your-key.pem ubuntu@your-ec2-ip

# 시스템 업데이트
sudo apt update && sudo apt upgrade -y

# Docker 설치
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER

# Docker Compose 설치
sudo apt install docker-compose-plugin -y

# 로그아웃 후 재접속
exit
ssh -i your-key.pem ubuntu@your-ec2-ip
```

##### 1-1. EBS 볼륨 확장 (디스크 부족 시)

```bash
# AWS 콘솔에서 EBS 볼륨 크기 변경 후 (예: 8GB → 20GB)

# 파티션 확장 (Amazon Linux / Ubuntu 공통)
sudo growpart /dev/xvda 1

# 파일시스템 확장
# Amazon Linux (xfs):
sudo xfs_growfs /

# Ubuntu (ext4):
sudo resize2fs /dev/xvda1

# 확인
df -h
```

##### 2. 프로젝트 배포

```bash
# 1. 프로젝트 클론 및 브랜치 설정
git clone https://github.com/AmazingKoreanCenter/amazing-korean-api.git
cd amazing-korean-api
git checkout KKRYOUN  # 또는 배포할 브랜치

# 2. 환경 변수 설정
cat > .env.prod << 'EOF'
POSTGRES_PASSWORD=your-secure-password
JWT_SECRET=your-32-byte-minimum-secret-key
DOMAIN=api.amazingkorean.net
CORS_ORIGINS=http://localhost:5173,https://amazingkorean.net,https://www.amazingkorean.net
EOF
```

```bash
# 3. 필요 디렉토리 생성
mkdir -p certbot/www certbot/conf

# 4. Docker Compose 실행 (t2.micro 기준 15-30분 소요)
docker compose -f docker-compose.prod.yml --env-file .env.prod up -d --build

# 5. 로그 확인
docker compose -f docker-compose.prod.yml logs -f
```

> **주의**: `.sqlx` 폴더가 없으면 빌드 실패합니다. "Step 0. SQLx 오프라인 모드 준비" 참조.

##### 3. SSL 인증서 발급 (Let's Encrypt)

```bash
# 1. 초기 인증서 발급 (HTTP 모드로 nginx 실행 중인 상태에서)
docker compose -f docker-compose.prod.yml run --rm certbot certonly \
  --webroot \
  --webroot-path=/var/www/certbot \
  -d api.yourdomain.com \
  --email your-email@example.com \
  --agree-tos \
  --no-eff-email

# 2. nginx.conf HTTPS 섹션 활성화 (주석 해제)
nano nginx/nginx.conf

# 3. Nginx 재시작
docker compose -f docker-compose.prod.yml restart nginx
```

##### 4. 데이터베이스 마이그레이션

```bash
# SQLx CLI 설치 (로컬 또는 EC2에서)
cargo install sqlx-cli --no-default-features --features postgres

# 마이그레이션 실행
DATABASE_URL=postgres://postgres:your-password@localhost:5432/amazing_korean_db \
  sqlx migrate run
```

##### 5. 배포 후 확인

```bash
# API 헬스체크
curl http://your-ec2-ip/health

# 컨테이너 상태 확인
docker compose -f docker-compose.prod.yml ps

# 로그 확인
docker compose -f docker-compose.prod.yml logs api
```

##### 6. 관련 파일

| 파일 | 설명 |
|------|------|
| `Dockerfile` | Rust 백엔드 멀티스테이지 빌드 (rust:1.85, SQLx offline mode) |
| `docker-compose.prod.yml` | 프로덕션 구성 (API + DB + Redis + Nginx) |
| `nginx/nginx.conf` | 리버스 프록시 + SSL + CORS 설정 |
| `.sqlx/` | SQLx 오프라인 캐시 (Docker 빌드 시 필수) |
| `.env.prod` | 프로덕션 환경 변수 (Git에 포함하지 않음) |

##### 7. 유용한 명령어

```bash
# 전체 재시작
docker compose -f docker-compose.prod.yml down && docker compose -f docker-compose.prod.yml up -d

# 특정 서비스만 재빌드
docker compose -f docker-compose.prod.yml up -d --build api

# 로그 실시간 확인
docker compose -f docker-compose.prod.yml logs -f api

# 컨테이너 쉘 접속
docker exec -it amk-api /bin/bash
docker exec -it amk-pg psql -U postgres -d amazing_korean_db

# 빌드 진행 상황 확인 (다른 터미널에서)
docker stats
```

##### 8. 트러블슈팅

| 에러 | 원인 | 해결 |
|------|------|------|
| `Permission denied (publickey)` | SSH 사용자 이름 오류 | Amazon Linux: `ec2-user@`, Ubuntu: `ubuntu@` |
| `git: command not found` | Git 미설치 (Amazon Linux) | `sudo yum install -y git` |
| `compose build requires buildx` | Buildx 미설치 | 위 Docker 설치 섹션 참조 |
| `feature 'edition2024' is required` | Rust 버전 낮음 | Dockerfile에서 `rust:1.85-bookworm` 사용 |
| `No space left on device` | 디스크 부족 (8GB) | EBS 볼륨 20GB gp3로 확장 |
| `set DATABASE_URL to use query macros` | SQLx 캐시 없음 | `cargo sqlx prepare` 후 `.sqlx` 커밋 |
| `divergent branches` (git pull) | 브랜치 충돌 | `git fetch origin && git reset --hard origin/BRANCH` |
| `address already in use` (443) | 포트 충돌 | `sudo fuser -k 443/tcp` 후 재시작 |
| `database is being accessed` | DB 연결 중 | API 중지 후 `pg_terminate_backend()` 실행 |

##### 9. Cloudflare SSL 설정 (Let's Encrypt 대안)

Cloudflare 프록시 사용 시 Let's Encrypt 없이 SSL 적용 가능:

1. Cloudflare 대시보드 → `amazingkorean.net` → **DNS**
2. `api` A 레코드의 프록시 상태를 **주황색 구름** (Proxied)으로 설정
3. **SSL/TLS** → **Overview** → 모드를 **Flexible**로 설정

> **참고**: Flexible 모드는 Cloudflare ↔ 사용자 간 HTTPS, Cloudflare ↔ EC2 간 HTTP를 사용합니다.

##### 10. 로컬 → EC2 데이터 이전

개발 환경의 테스트 데이터를 프로덕션으로 이전하는 방법:

**로컬 (WSL)에서:**
```bash
# 1. SSH 키 권한 설정 (WSL에서 Windows 드라이브 사용 시)
cp /mnt/d/YOUR_PATH/your-key.pem ~/
chmod 400 ~/your-key.pem

# 2. 데이터베이스 덤프 (스키마 + 데이터)
docker exec amk-pg pg_dump -U postgres -d amazing_korean_db --exclude-table=_sqlx_migrations > ~/db_full.sql

# 3. EC2로 파일 전송
scp -i ~/your-key.pem ~/db_full.sql ec2-user@YOUR_EC2_IP:~/db_full.sql
```

**EC2에서:**
```bash
# 1. API 중지
docker stop amk-api

# 2. 기존 연결 종료 및 DB 리셋
docker exec -it amk-pg psql -U postgres -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = 'amazing_korean_db' AND pid <> pg_backend_pid();"
docker exec -it amk-pg psql -U postgres -c "DROP DATABASE amazing_korean_db;"
docker exec -it amk-pg psql -U postgres -c "CREATE DATABASE amazing_korean_db;"

# 3. 데이터 가져오기
docker exec -i amk-pg psql -U postgres -d amazing_korean_db < ~/db_full.sql

# 4. API 재시작
docker start amk-api

# 5. 확인
docker exec -it amk-pg psql -U postgres -d amazing_korean_db -c "\dt"
docker exec -it amk-pg psql -U postgres -d amazing_korean_db -c "SELECT COUNT(*) FROM users;"
```

> **주의**: `--exclude-table=_sqlx_migrations`로 마이그레이션 기록 테이블은 제외합니다.

#### 6.6.2-3 GitHub Actions CI/CD 파이프라인

> **목적**: EC2에서 Rust 빌드 없이 자동 배포. t2.micro (1GB RAM)로 운영 가능.

##### CI/CD 흐름

```
┌─────────────┐    ┌──────────────────┐    ┌─────────────┐    ┌─────────┐
│  git push   │ →  │  GitHub Actions  │ →  │ Docker Hub  │ →  │   EC2   │
│  (로컬)      │    │  (빌드 서버)      │    │ (이미지저장) │    │  (실행)  │
└─────────────┘    └──────────────────┘    └─────────────┘    └─────────┘
```

1. **코드 Push** → `main` 또는 `KKRYOUN` 브랜치에 push
2. **GitHub Actions** → GitHub 서버(7GB RAM)에서 Docker 이미지 빌드
3. **Docker Hub Push** → 빌드된 이미지를 Docker Hub에 업로드
4. **EC2 배포** → SSH로 EC2 접속 → 이미지 pull → 컨테이너 재시작

##### GitHub Secrets 설정

GitHub repo → **Settings** → **Secrets and variables** → **Actions**에서 추가:

| Secret Name | 값 | 설명 |
|-------------|-----|------|
| `DOCKERHUB_USERNAME` | Docker Hub 사용자명 | |
| `DOCKERHUB_TOKEN` | Docker Hub Access Token | Read & Write 권한 |
| `EC2_HOST` | EC2 Public IP | 예: `43.200.180.110` |
| `EC2_SSH_KEY` | .pem 파일 내용 전체 | `-----BEGIN` ~ `END-----` |
| `POSTGRES_PASSWORD` | DB 비밀번호 | |
| `JWT_SECRET` | JWT 시크릿 키 | |

##### Workflow 파일 (.github/workflows/deploy.yml)

```yaml
name: Deploy to EC2

on:
  push:
    branches: [main, KKRYOUN]
  workflow_dispatch:  # 수동 실행 가능

env:
  DOCKER_IMAGE: ${{ secrets.DOCKERHUB_USERNAME }}/amazing-korean-api

jobs:
  build-and-push:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3

      - name: Login to Docker Hub
        uses: docker/login-action@v3
        with:
          username: ${{ secrets.DOCKERHUB_USERNAME }}
          password: ${{ secrets.DOCKERHUB_TOKEN }}

      - name: Build and push Docker image
        uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: |
            ${{ env.DOCKER_IMAGE }}:latest
            ${{ env.DOCKER_IMAGE }}:${{ github.sha }}
          cache-from: type=gha
          cache-to: type=gha,mode=max

  deploy:
    needs: build-and-push
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to EC2
        uses: appleboy/ssh-action@v1.0.3
        with:
          host: ${{ secrets.EC2_HOST }}
          username: ec2-user
          key: ${{ secrets.EC2_SSH_KEY }}
          script: |
            cd ~/amazing-korean-api
            docker pull ${{ env.DOCKER_IMAGE }}:latest
            docker compose -f docker-compose.prod.yml --env-file .env.prod up -d
            docker image prune -f
```

##### docker-compose.prod.yml (이미지 사용 방식)

```yaml
services:
  api:
    image: ${DOCKER_IMAGE:-amazing-korean-api}:latest  # Docker Hub 이미지 사용
    container_name: amk-api
    # ... 이하 동일
```

> **참고**: 기존 `build:` 블록 대신 `image:` 사용. EC2에서 빌드하지 않음.

##### .dockerignore

```
# Documentation
docs/
*.md

# Frontend (Cloudflare Pages에서 별도 배포)
frontend/

# Git
.git/
.github/

# Development
.env
target/
tests/
```

##### 배포 방법

```bash
# 자동 배포 (push만 하면 끝)
git add . && git commit -m "feat: 새 기능" && git push origin KKRYOUN

# 수동 배포 (GitHub Actions 페이지에서)
# Actions → Deploy to EC2 → Run workflow
```

##### 장점

| 항목 | 이전 (EC2 빌드) | 현재 (CI/CD) |
|------|----------------|--------------|
| Rust 컴파일 | EC2에서 (t3.medium 필요) | GitHub Actions에서 |
| 빌드 시간 | 15-30분 | 5-10분 |
| EC2 스펙 | t3.medium 임시 필요 | t2.micro 유지 가능 |
| 배포 방식 | SSH 접속 후 수동 | `git push`만 |

#### 6.6.2-4 EC2 유지보수 가이드

##### 디스크 사용량 확인

```bash
# 전체 디스크 사용량
df -h

# Docker 관련 용량
docker system df

# Docker 이미지별 용량
docker images --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}"
```

##### 디스크 정리

```bash
# Docker Build Cache 정리 (CI/CD 사용 시 불필요)
docker builder prune -f

# 사용하지 않는 이미지 정리
docker image prune -a

# 사용하지 않는 볼륨 정리 (주의: 데이터 손실 가능)
docker volume prune
```

##### Docker/시스템 업데이트

```bash
# Docker 업데이트 (Amazon Linux)
sudo yum update docker -y
sudo systemctl restart docker

# 이미지 업데이트 후 재시작
docker compose -f docker-compose.prod.yml --env-file .env.prod pull
docker compose -f docker-compose.prod.yml --env-file .env.prod up -d
```

> **참고**: CI/CD 적용 후 EC2에서는 빌드 작업이 없으므로 t2.micro (1GB RAM)로 모든 유지보수 작업 가능.

#### 6.6.3 품질 보증 (QA) & 스모크 체크

- **정적 분석 (CI Gate)**
  - `npm run lint`: ESLint (코드 스타일 및 잠재적 버그 검사)
  - `npm run typecheck`: TypeScript 타입 정합성 검사 (필수)

- **수동 스모크 테스트 (Release Checklist)**
  - 배포 전 아래 시나리오를 **반드시 1회 수동 확인**한다.
    1.  **진입**: 랜딩 페이지 로딩 및 폰트/이미지 깨짐 확인.
    2.  **인증**: 로그인(토큰 발급) → 새로고침 시 로그인 유지 확인.
    3.  **영상**: 비디오 목록 로딩 → 상세 페이지 진입 → 플레이어 재생 확인.
    4.  **라우팅**: 잘못된 URL 입력 시 404 페이지(또는 리다이렉트) 동작 확인.

#### 6.6.4 향후 확장 계획 (Roadmap)

- **자동화 테스트 도입 (Phase 3 이후)**
  - **Unit Test**: `Vitest` 도입. (유틸 함수 및 복잡한 Hook 로직 검증)
  - **E2E Test**: `Playwright` 도입. (핵심 비즈니스 플로우 자동화)

- **CI/CD 파이프라인**
  - GitHub Actions 연동:
    - Push 시: `Lint` + `Typecheck` 자동 실행.
    - Tag/Merge 시: `Build` 수행 후 Docker Image 생성 또는 S3 업로드.

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
     - 롤별 세부 권한 매트릭스는 **Open Questions 섹션**에서 정의/업데이트 한다.
   - 통신
     - 운영 환경에서는 반드시 HTTPS를 사용하고, 토큰/세션 ID를 URL(query string)에 노출하지 않는다.

### 7.2 개발 플로우

1. 개발 사항에 대한 이 문서(**AMK_API_MASTER.md**) 확인 및 참조
2. 1) 기존 개발 사항 : **AMK_API_MASTER.md** 확인 및 참조 후 해당 개발 사항 작업 진행
   2) 신규 개발 사항 : 신규 API 명시 → **AMK_API_MASTER.md** 확인 및 참조 → **AMK_API_MASTER.md** 형식으로 문서 업데이트 → 해당 개발 사항 작업 진행
3. GEMINI 템플릿에 따라 patch prompt 작성
4. 코드/마이그레이션 생성
5. 정적 가드(cargo check/fmt/clippy) + 최소 스모크 테스트(cURL or 스크립트)
6. 관련 로드맵 행의 “기능 완료” 체크박스 업데이트

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

## 7.7 코드 예시 (Best Practices)

> 이 섹션의 코드는 **실제 프로젝트에서 검증된 패턴**입니다.
> LLM에게 새 기능 요청 시 "Section 7.7의 패턴 X 사용" 지시하면 일관된 코드 생성 가능.

### 7.7.1 백엔드 패턴 (Rust/Axum)

---

#### 7.7.1-0. 공용 코드 (Common Code)

> **📋 SSoT 검증 완료** (2026-01-22)
> 아래 내용은 실제 코드 기반으로 검증되었습니다.

##### 파일 목록 및 역할

| 파일 | 역할 | 의존 관계 |
|------|------|----------|
| `src/config.rs` | 런타임 설정 SSoT (환경변수 파싱) | dotenvy |
| `src/state.rs` | AppState 의존성 컨테이너 | config.rs |
| `src/error.rs` | 전역 에러 타입 + HTTP 응답 표준화 | 독립 |
| `src/types.rs` | DB enum ↔ Rust enum ↔ JSON 매핑 | 독립 |
| `src/docs.rs` | OpenAPI 문서 집계 + 보안 스키마 | 도메인 핸들러들 |
| `src/main.rs` | 부트스트랩 (리소스 생성 → 서버 실행) | 모든 모듈 |
| `src/api/mod.rs` | 도메인 라우터 조립 | 도메인 라우터들 |

---

##### 1️⃣ `src/config.rs` — 런타임 설정 SSoT

**역할**: 환경변수 기반 설정의 **단일 진입점**. 모든 런타임 파라미터가 이 파일에서 관리됨.

```rust
use std::env;

#[derive(Clone)]
pub struct Config {
    // 필수 인프라
    pub database_url: String,
    pub bind_addr: String,
    pub redis_url: String,

    // JWT 설정 (필수)
    pub jwt_secret: String,
    pub jwt_expire_hours: i64,
    pub jwt_access_ttl_min: i64,

    // Refresh Token 설정
    pub refresh_ttl_days: i64,
    pub refresh_cookie_name: String,
    pub refresh_cookie_domain: Option<String>,
    pub refresh_cookie_secure: bool,
    pub refresh_cookie_samesite: String,

    // 기능 토글
    pub enable_docs: bool,
    pub skip_db: bool,

    // Rate Limit
    pub rate_limit_login_window_sec: i64,
    pub rate_limit_login_max: i64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        // JWT_SECRET만 필수 (expect)
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        // 나머지는 기본값 제공
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://...".into());
        // ... 생략
    }
}
```

**🔑 핵심 포인트**:
- `JWT_SECRET`만 **필수** (없으면 panic) — 보안 강제
- 나머지 설정은 **기본값 제공** — 로컬 개발 편의성
- `refresh_cookie_samesite_or()` 헬퍼로 빈 문자열 처리

**⚠️ 규칙**:
- 새 환경변수 추가 시 → **반드시 Config에 필드 추가**
- 기본값 결정 시 → 로컬 개발 편의 vs 프로덕션 안전성 고려

---

##### 2️⃣ `src/state.rs` — AppState 의존성 컨테이너

**역할**: 핸들러/서비스/레포에서 공통 접근하는 **의존성 묶음**

```rust
use axum::extract::FromRef;
use deadpool_redis::Pool as RedisPool;
use sqlx::{Pool, Postgres};
use std::time::Instant;

use crate::config::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: Pool<Postgres>,    // Postgres 커넥션 풀
    pub redis: RedisPool,       // Redis 커넥션 풀
    pub cfg: Config,            // 런타임 설정
    pub started_at: Instant,    // 서버 시작 시간 (uptime 계산용)
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}
```

**🔑 핵심 포인트**:
- `#[derive(Clone, FromRef)]` → State 추출 + 부분 추출 가능
- 핸들러에서: `State(state): State<AppState>`
- 서브스테이트 추출: `State(db): State<Pool<Postgres>>`

**⚠️ 규칙**:
- 핸들러 → `&state.db`, `&state.redis`, `&state.cfg`로 하위 레이어에 전달
- 새 전역 리소스 추가 시 → AppState에 필드 추가 + main.rs에서 초기화

---

##### 3️⃣ `src/error.rs` — 전역 에러 타입 + HTTP 응답 표준화

**역할**: 모든 레이어의 에러를 **통일된 HTTP 응답**으로 변환

```rust
use thiserror::Error;
use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    Json,
};

#[derive(Debug, Error)]
pub enum AppError {
    // 비즈니스 에러
    #[error("Internal server error")]
    Internal(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Unprocessable(String),         // 422
    #[error("{0}")]
    Unauthorized(String),
    #[error("Forbidden")]
    Forbidden,
    #[error("Not found")]
    NotFound,
    #[error("{0}")]
    Conflict(String),              // 409
    #[error("{0}")]
    TooManyRequests(String),       // 429

    // 인프라 에러 (자동 변환)
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Redis(#[from] deadpool_redis::redis::RedisError),
    #[error(transparent)]
    RedisPool(#[from] deadpool_redis::PoolError),
    #[error(transparent)]
    Jsonwebtoken(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

// 전역 Result 타입
pub type AppResult<T> = Result<T, AppError>;
```

**HTTP 응답 표준화**:

```rust
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message, details, retry_after) = match &self {
            AppError::BadRequest(msg) =>
                (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone(), None, None),
            AppError::Unauthorized(msg) =>
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone(), None, None),
            AppError::TooManyRequests(msg) =>
                (StatusCode::TOO_MANY_REQUESTS, "TOO_MANY_REQUESTS", msg.clone(), None, Some(60)),
            // ... 기타 매칭
        };

        // 표준 에러 바디
        let body = serde_json::json!({
            "error": {
                "code": code,
                "http_status": status.as_u16(),
                "message": message,
                "details": details,
                "trace_id": "req-TODO"  // TODO: Request ID 연동
            }
        });

        let mut res = (status, Json(body)).into_response();
        if let Some(sec) = retry_after {
            res.headers_mut().insert(
                header::RETRY_AFTER,
                sec.to_string().parse().unwrap()
            );
        }
        res
    }
}
```

**🔑 핵심 포인트**:
- `AppResult<T>` = `Result<T, AppError>` — 전 레이어 공용
- `?` 연산자로 에러 전파 → 자동으로 HTTP 응답 변환
- `#[from]` 어트리뷰트로 인프라 에러 자동 래핑
- 429 응답 시 `Retry-After` 헤더 자동 추가

**⚠️ 규칙**:
- 새 에러 타입 필요 시 → `AppError` variant 추가
- 서비스/레포에서 `Err(AppError::NotFound)` 형태로 반환
- 프론트엔드는 `error.code` 필드로 에러 종류 판단

---

##### 4️⃣ `src/types.rs` — DB enum ↔ Rust enum ↔ JSON 매핑

**역할**: DB enum 타입의 **단일 정의** (중복 enum 금지)

```rust
use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use std::fmt;

// Triple Derive 패턴: sqlx + serde + utoipa
#[derive(Clone, Debug, Serialize, Deserialize, Type, ToSchema, PartialEq)]
#[sqlx(type_name = "user_auth")]        // DB enum 이름
#[serde(rename_all = "lowercase")]      // JSON: "google", "email"
pub enum UserAuth {
    Google,
    Apple,
    Email,
}

// Display 구현 (로깅용)
impl fmt::Display for UserAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserAuth::Google => write!(f, "google"),
            UserAuth::Apple => write!(f, "apple"),
            UserAuth::Email => write!(f, "email"),
        }
    }
}

// 특수 케이스: DB와 API 이름이 다를 때
#[derive(Clone, Debug, Serialize, Deserialize, Type, ToSchema, PartialEq)]
#[sqlx(type_name = "lesson_type")]
#[serde(rename_all = "lowercase")]
pub enum LessonType {
    Video,
    #[sqlx(rename = "HYMN")]           // DB에는 대문자로 저장
    #[serde(rename = "hymn")]          // JSON에는 소문자로 노출
    Hymn,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type, ToSchema, PartialEq)]
#[sqlx(type_name = "user_level")]
pub enum UserLevel {
    #[serde(rename = "basic_900")]     // JSON: "basic_900"
    Basic900,
    #[serde(rename = "basic_1800")]
    Basic1800,
    // ...
}
```

**🔑 핵심 포인트**:
- **Triple Derive**: `sqlx::Type` + `serde` + `utoipa::ToSchema`
- DB enum 이름: `#[sqlx(type_name = "...")]`
- JSON 직렬화: `#[serde(rename_all = "...")]` 또는 개별 `#[serde(rename = "...")]`
- 예외 케이스: `#[sqlx(rename = "...")]`로 DB 값 명시

**⚠️ 규칙**:
- **중복 enum 정의 금지** — 모든 도메인에서 `crate::types::*` import
- 새 DB enum 추가 시 → 여기에 정의 + 마이그레이션 작성
- Swagger에 자동 노출됨 (ToSchema)

---

##### 5️⃣ `src/docs.rs` — OpenAPI 문서 집계

**역할**: 모든 API 경로와 스키마를 **단일 OpenAPI 문서**로 집계

```rust
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

// 보안 스키마 등록
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert(Default::default());

        // JWT Bearer Auth
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );

        // Refresh Token Cookie
        components.add_security_scheme(
            "refreshCookie",
            SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Cookie(
                utoipa::openapi::security::ApiKeyValue::new("ak_refresh"),
            )),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Amazing Korean API", version = "0.1.0"),
    paths(
        // 모든 핸들러 함수 나열
        crate::api::auth::handler::login,
        crate::api::auth::handler::refresh,
        crate::api::user::handler::get_me,
        crate::api::video::handler::get_video_detail,
        // ... 전체 paths
    ),
    components(schemas(
        // 모든 DTO/Enum 나열
        crate::api::auth::dto::LoginRequest,
        crate::api::auth::dto::LoginResponse,
        crate::types::UserAuth,
        crate::error::ErrorBody,
        // ... 전체 schemas
    )),
    tags(
        (name = "Auth", description = "인증 관련 API"),
        (name = "User", description = "사용자 관련 API"),
        (name = "Video", description = "비디오 관련 API"),
        // ... 전체 tags
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
```

**🔑 핵심 포인트**:
- `paths(...)`: 문서화할 핸들러 함수 목록
- `components(schemas(...))`: 문서화할 DTO/Enum 목록
- `tags(...)`: Swagger UI 그룹핑
- `SecurityAddon`: `bearerAuth` + `refreshCookie` 스키마 등록

**⚠️ 규칙**:
- 새 핸들러 추가 시 → `paths(...)`에 등록 **필수**
- 새 DTO 추가 시 → `components(schemas(...))`에 등록 **필수**
- 핸들러에 `#[utoipa::path(...)]` 매크로 필수

---

##### 6️⃣ `src/main.rs` — 부트스트랩

**역할**: 애플리케이션 시작 순서 정의

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) 설정 로드
    let cfg = Config::from_env();

    // 2) Tracing 초기화
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "amazing_korean_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 3) Postgres 풀 생성
    let database_url = if cfg.database_url.contains("?") {
        cfg.database_url.clone()
    } else {
        format!("{}?connect_timeout=5", cfg.database_url)
    };

    let pool: Pool<Postgres> = if std::env::var("DB_EAGER").ok().as_deref() == Some("1") {
        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .connect(&database_url)      // 즉시 연결 시도
            .await?
    } else {
        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .connect_lazy(&database_url)?  // 첫 쿼리 시 연결
    };

    // 4) Redis 풀 생성
    let redis_cfg = deadpool_redis::Config::from_url(cfg.redis_url.clone());
    let redis: RedisPool = redis_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    // 5) AppState 생성
    let app_state = AppState {
        db: pool,
        redis,
        cfg: cfg.clone(),
        started_at: Instant::now(),
    };

    // 6) CORS 설정
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT,
                       Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_credentials(true);  // 쿠키 교환 허용

    // 7) 라우터 조립 + CORS 레이어
    let app = api::app_router(app_state).layer(cors);

    // 8) 서버 시작
    let listener = TcpListener::bind(&cfg.bind_addr).await?;
    tracing::info!("✅ Server listening on http://{}", cfg.bind_addr);

    axum::serve(listener, app).await?;
    Ok(())
}
```

**🔑 핵심 포인트**:
- **부트스트랩 순서**: Config → Tracing → DB Pool → Redis Pool → AppState → CORS → Router → Serve
- `DB_EAGER=1`: 즉시 DB 연결 (CI/프로덕션 권장)
- `connect_lazy()`: 첫 쿼리 시 연결 (로컬 개발 빠른 시작)
- `allow_credentials(true)`: Refresh Token 쿠키 교환 필수

**⚠️ 규칙**:
- 새 전역 리소스 추가 시 → main.rs에서 초기화 + AppState에 주입
- CORS origin 추가 필요 시 → `allow_origin()` 수정

---

##### 7️⃣ `src/api/mod.rs` — 도메인 라우터 조립

**역할**: 모든 도메인 라우터를 **최종 조립**

```rust
use crate::state::AppState;
use axum::routing::get;

use crate::docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// 도메인 모듈 선언
pub mod admin;
pub mod auth;
pub mod course;
pub mod health;
pub mod lesson;
pub mod study;
pub mod user;
pub mod video;

// 도메인 라우터 import
use self::admin::router::admin_router;
use self::auth::router::auth_router;
use self::course::router::course_router;
use self::lesson::router::router as lesson_router;
use self::study::router::router as study_router;
use self::user::router::user_router;
use self::video::router::router as video_router;

pub fn app_router(state: AppState) -> axum::Router {
    axum::Router::new()
        // merge: 루트 레벨 라우터 결합
        .merge(course_router())
        .merge(user_router())
        // nest: URL prefix 분리
        .nest("/auth", auth_router())
        .nest("/admin", admin_router())
        .nest("/lessons", lesson_router())
        .nest("/videos", video_router())
        .nest("/studies", study_router())
        // Health check (직접 등록)
        .route("/healthz", get(health::handler::health))
        .route("/health", get(health::handler::health))
        .route("/ready", get(health::handler::ready))
        // Swagger UI
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // 전역 상태 주입
        .with_state(state)
}
```

**🔑 핵심 포인트**:
- `merge()`: 루트 레벨 경로 결합 (예: `/users`, `/courses`)
- `nest("/prefix", router)`: URL prefix 추가 (예: `/auth/login`, `/videos/123`)
- `.with_state(state)`: **마지막에 한 번만** 호출
- 도메인 라우터는 상태 없이 경로만 정의

**⚠️ 규칙**:
- 새 도메인 추가 시 → `pub mod xxx;` + `use self::xxx::router::xxx_router;` + `merge()` 또는 `nest()`
- Swagger에 노출할 경로만 docs.rs에 등록

---

##### 📊 공통 패턴 요약

| 관심사 | 파일 | 패턴 |
|--------|------|------|
| 설정 | `config.rs` | 환경변수 → Config 구조체 |
| 상태 | `state.rs` | AppState + FromRef |
| 에러 | `error.rs` | AppError + IntoResponse |
| 타입 | `types.rs` | Triple Derive (sqlx + serde + utoipa) |
| 문서 | `docs.rs` | utoipa OpenApi derive |
| 조립 | `api/mod.rs` | merge/nest + with_state |

##### 🔄 레이어 간 데이터 흐름

```
[HTTP Request]
      ↓
[Router] → Path 매칭
      ↓
[Handler] → State<AppState> 주입
      ↓
[Service] → 비즈니스 로직, &state.db 사용
      ↓
[Repo] → sqlx 쿼리, AppResult<T> 반환
      ↓
[Handler] → AppResult<Json<Response>>
      ↓
[AppError::IntoResponse] → 표준 JSON 에러
      ↓
[HTTP Response]
```

---

#### 7.7.1-1. dto.rs

> **📋 SSoT 검증 완료** (2026-01-22)
> 아래 내용은 실제 코드 기반으로 검증되었습니다.

##### 파일 목록 및 역할

| 파일 | 역할 | 특징 |
|------|------|------|
| `src/api/auth/dto.rs` | 인증 요청/응답 (로그인, 토큰 등) | `#[schema(example)]` 적극 사용 |
| `src/api/lesson/dto.rs` | 레슨 목록/상세/진도 | `IntoParams` 사용, `sqlx::FromRow` |
| `src/api/study/dto.rs` | 학습 목록/문제/제출 | Tagged Union, types.rs enum 재사용 |
| `src/api/user/dto.rs` | 회원가입/프로필/설정 | PATCH 패턴, 자동 로그인 응답 |
| `src/api/video/dto.rs` | 비디오 목록/상세/진도 | JSONB 매핑, default 함수 |

---

##### dto.rs의 역할 (AMK 기준)

**API 경계 타입**: handler가 받는 입력(Query/Path/Json Body)과 반환(응답 바디)의 **"계약(Contract)"**을 정의

**문서화/검증의 중심**:
- `utoipa::ToSchema` / `IntoParams`로 OpenAPI 스키마 생성
- `validator::Validate`로 입력 검증 (특히 Body DTO)

**DB 스키마와의 관계**:
- 1:1로 같을 필요 없음 (보안/UX 목적에 따라 축약·가공 가능)
- 단, DB enum은 `crate::types::*`를 재사용해서 불일치/파싱 비용 최소화

---

##### 1️⃣ `src/api/auth/dto.rs` — 인증 요청/응답

**역할**: 로그인, 토큰 갱신, 아이디 찾기, 비밀번호 재설정, 로그아웃

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// =====================================================================
// Request DTOs (요청)
// =====================================================================

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "email": "front@front.com",
    "password": "front123!",
    "device": "web",
    "browser": "chrome",
    "os": "linux"
}))]
pub struct LoginReq {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6, max = 72))]
    pub password: String,

    // 클라이언트가 명시적으로 보낼 경우를 위해 Option 유지
    #[serde(default)]
    pub device: Option<String>,
    #[serde(default)]
    pub browser: Option<String>,
    #[serde(default)]
    pub os: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct RefreshReq {
    // 쿠키를 사용할 수 없는 환경(앱 등)을 위해 바디로도 받을 수 있게 유지
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

// =====================================================================
// Response DTOs (응답)
// =====================================================================

/// 액세스 토큰 공통 규격
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct AccessTokenRes {
    pub access_token: String,
    pub token_type: String, // "Bearer" 고정
    pub expires_in: i64,    // 초 단위
    pub expires_at: String, // 프론트엔드 편의용 ISO String
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct LoginRes {
    pub user_id: i64,
    pub access: AccessTokenRes,
    pub session_id: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct LogoutRes {
    pub ok: bool,
}
```

**🔑 핵심 포인트**:
- **Request/Response 섹션 분리**: 주석으로 명확히 구분
- **`#[schema(example = json!(...))]`**: Swagger UI에서 즉시 테스트 가능
- **`#[serde(default)]`**: 클라이언트가 보내지 않아도 OK (Option + default)
- **`AccessTokenRes` 공통화**: `LoginRes`, `RefreshRes` 등에서 재사용

**⚠️ 규칙**:
- `expires_at`은 프론트엔드 편의를 위해 String 유지 (ISO 8601 형식)
- `json!` 매크로 사용 시 파일 상단에 `use serde_json::json;` 확인

---

##### 2️⃣ `src/api/user/dto.rs` — 회원가입/프로필/설정

**역할**: 회원가입 (자동 로그인 포함), 프로필 CRUD, 환경설정

```rust
use crate::api::auth::dto::AccessTokenRes;
use crate::types::{UserAuth, UserGender};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// =====================================================================
// Request DTOs (요청)
// =====================================================================

/// 회원가입 요청
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct SignupReq {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 72))]
    pub password: String,

    #[validate(length(min = 1, max = 50))]
    pub name: String,

    #[validate(length(min = 1, max = 100))]
    pub nickname: String,

    /// ISO 639-1 언어 코드 (예: "ko", "en")
    #[validate(length(min = 2, max = 2))]
    pub language: String,

    /// ISO 3166-1 alpha-2 국가 코드 (예: "KR", "US")
    #[validate(length(min = 2, max = 50))]
    pub country: String,

    #[schema(value_type = String, format = "date")]
    pub birthday: NaiveDate,

    pub gender: UserGender, // Enum: male, female, other, none

    pub terms_service: bool,
    pub terms_personal: bool,
}

/// 프로필 수정 요청 (PATCH)
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProfileUpdateReq {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 100))]
    pub nickname: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 50))]
    pub language: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date")]
    pub birthday: Option<NaiveDate>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<UserGender>,
}

// =====================================================================
// Response DTOs (응답)
// =====================================================================

/// 회원가입 완료 응답 (자동 로그인 처리됨)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct SignupRes {
    pub user_id: i64,
    pub email: String,
    pub name: String,
    pub nickname: String,
    // ... 기타 필드

    /// 자동 발급된 액세스 토큰
    pub access: AccessTokenRes,
    /// 현재 세션 ID
    pub session_id: String,
}

/// 사용자 프로필 정보
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProfileRes {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub nickname: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    #[schema(value_type = String, format = "date")]
    pub birthday: Option<NaiveDate>,
    pub gender: UserGender,
    pub user_state: bool,
    pub user_auth: UserAuth,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    /// 비밀번호 설정 여부 (OAuth 전용 계정은 false)
    pub has_password: bool,
}
```

**🔑 핵심 포인트**:
- **`crate::types::*` enum 재사용**: `UserGender`, `UserAuth` 등 DB enum과 일치
- **PATCH 패턴**: `Option<T>` + `#[serde(default, skip_serializing_if = "Option::is_none")]`
- **자동 로그인 응답**: `SignupRes`에 `AccessTokenRes` + `session_id` 포함
- **`#[schema(value_type = String, format = "date")]`**: Swagger에서 날짜 형식 표시
- **`sqlx::FromRow`**: DB 조회 결과 직접 매핑 가능

**⚠️ 규칙**:
- enum 필드는 **반드시** `crate::types::*` 사용 (String 금지)
- 날짜 필드에 `#[schema(value_type = String, format = "...")]` 필수

---

##### 3️⃣ `src/api/video/dto.rs` — 비디오 목록/상세/진도

**역할**: 비디오 검색/목록, 상세 정보, 학습 진도 관리

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};
use utoipa::ToSchema;
use validator::Validate;

// =====================================================================
// Request DTOs (요청)
// =====================================================================

/// 비디오 목록 조회 및 검색 요청 (Query String)
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoListReq {
    #[serde(default = "default_page")]
    #[validate(range(min = 1))]
    pub page: u64,

    #[serde(default = "default_per_page")]
    #[validate(range(min = 1, max = 100))]
    pub per_page: u64,

    pub q: Option<String>,          // 검색어
    pub tag: Option<String>,        // 태그 필터
    pub state: Option<String>,      // 상태 필터
    pub sort: Option<String>,       // 정렬
}

fn default_page() -> u64 { 1 }
fn default_per_page() -> u64 { 20 }

/// 학습 진도 업데이트 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoProgressUpdateReq {
    #[validate(range(min = 0, max = 100))]
    pub progress_rate: i32,
}

// =====================================================================
// Response DTOs (응답)
// =====================================================================

/// 목록 페이징 메타데이터
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoListMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: u64,
    pub per_page: u64,
}

/// 비디오 목록 응답 (Data + Meta)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoListRes {
    pub meta: VideoListMeta,
    pub data: Vec<VideoListItem>,  // ⭐ 표준: { meta, data }
}

/// 상세 태그 정보 (JSONB 구조)
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub struct VideoTagDetail {
    pub key: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
}

/// 비디오 상세 정보
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoDetailRes {
    pub video_id: i64,
    pub video_url_vimeo: String,
    pub video_state: String,

    // DB의 JSONB 타입을 Rust 구조체로 매핑
    #[schema(value_type = Vec<VideoTagDetail>)]
    pub tags: Json<Vec<VideoTagDetail>>,

    pub created_at: DateTime<Utc>,
}

/// 학습 진도 조회 응답
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoProgressRes {
    pub video_id: i64,

    #[sqlx(rename = "video_progress_log")]
    pub progress_rate: i32,

    #[sqlx(rename = "video_completed_log")]
    pub is_completed: bool,
}
```

**🔑 핵심 포인트**:
- **`#[serde(default = "함수명")]`**: 페이징 기본값 설정
- **`#[validate(range(min = 0, max = 100))]`**: 범위 검증
- **JSONB 매핑**: `sqlx::types::Json<Vec<T>>` + `#[schema(value_type = Vec<T>)]`
- **`#[sqlx(rename = "...")]`**: DB 컬럼명 ↔ DTO 필드명 매핑
- **응답 표준**: `{ meta, data }` 구조 ⭐

**⚠️ 규칙**:
- 페이징 응답은 **`{ meta, data }`** 형태로 통일 권장
- JSONB 필드는 `#[schema(value_type = ...)]`로 Swagger 문서화

---

##### 4️⃣ `src/api/study/dto.rs` — 학습 목록/문제/제출

**역할**: 학습 프로그램 목록, 문제 상세, 정답 제출, 해설/상태 조회

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::types::{StudyProgram, StudyState, StudyTaskKind};

// =========================================================================
// Request DTOs (요청)
// =========================================================================

/// 학습 목록 조회 요청 (Query String)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListReq {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub program: Option<String>,
    pub sort: Option<String>,
}

/// 정렬 옵션 (서비스에서 파싱)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudyListSort {
    Latest,
    Oldest,
    Alphabetical,
}

impl StudyListSort {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "latest" => Some(Self::Latest),
            "oldest" => Some(Self::Oldest),
            "alphabetical" => Some(Self::Alphabetical),
            _ => None,
        }
    }
}

/// 정답 제출 요청 (JSON Body) - Tagged Union
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SubmitAnswerReq {
    Choice { pick: i32 },
    Typing { text: String },
    Voice { text: String },
}

// =========================================================================
// Response DTOs (응답)
// =========================================================================

/// 학습 목록 아이템 (DB Row)
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudySummaryDto {
    pub study_id: i32,
    pub study_idx: String,
    pub program: StudyProgram,     // ⭐ types.rs enum 재사용
    pub title: Option<String>,
    pub state: StudyState,         // ⭐ types.rs enum 재사용
    pub created_at: DateTime<Utc>,
}

/// 학습 목록 전체 응답
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListResp {
    pub list: Vec<StudySummaryDto>,  // 참고: video는 data, lesson은 items
    pub meta: StudyListMeta,
}

/// 학습 문제 상세 정보 (Payload 포함)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyTaskDetailRes {
    pub task_id: i32,
    pub study_id: i32,
    pub kind: StudyTaskKind,       // ⭐ types.rs enum 재사용
    pub seq: i32,
    pub created_at: DateTime<Utc>,
    pub payload: TaskPayload,
}

/// 문제 유형별 페이로드 (Untagged Union)
#[derive(Debug, Serialize, ToSchema)]
#[serde(untagged)]
pub enum TaskPayload {
    Choice(ChoicePayload),
    Typing(TypingPayload),
    Voice(VoicePayload),
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChoicePayload {
    pub question: String,
    pub choice_1: String,
    pub choice_2: String,
    pub choice_3: String,
    pub choice_4: String,
    pub audio_url: Option<String>,
    pub image_url: Option<String>,
}

/// 정답 제출 결과
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct SubmitAnswerRes {
    pub is_correct: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}
```

**🔑 핵심 포인트**:
- **`crate::types::*` enum 적극 재사용**: `StudyProgram`, `StudyState`, `StudyTaskKind`
- **Tagged Union**: `#[serde(tag = "kind")]`로 요청 다형성 처리
- **Untagged Union**: `#[serde(untagged)]`로 응답 페이로드 구분
- **별도 파싱 enum**: `StudyListSort::parse()`로 정렬 옵션 처리
- **`#[serde(skip_serializing_if = "Option::is_none")]`**: null 필드 생략

**⚠️ 규칙**:
- 문제 유형별 로직은 서비스에서 `match` 처리
- DB enum은 **절대** String으로 받지 않음

---

##### 5️⃣ `src/api/lesson/dto.rs` — 레슨 목록/상세/진도

**역할**: 레슨 목록, 레슨 상세 (아이템 포함), 학습 진도

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// Query DTO - IntoParams 사용
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct LessonListReq {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort: Option<String>,
}

// Response DTO - sqlx::FromRow 직접 derive
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct LessonRes {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub lesson_idx: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LessonListMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LessonListRes {
    pub items: Vec<LessonRes>,    // 참고: video는 data, study는 list
    pub meta: LessonListMeta,
}

/// 레슨 아이템 (비디오 or 과제)
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct LessonItemRes {
    pub seq: i32,
    pub kind: crate::types::LessonItemKind,  // ⭐ types.rs enum 재사용
    pub video_id: Option<i64>,
    pub task_id: Option<i64>,
}

/// 학습 진도 조회 응답
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct LessonProgressRes {
    pub percent: i32,
    pub last_seq: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 진도 업데이트 요청
#[derive(Debug, Deserialize, ToSchema)]
pub struct LessonProgressUpdateReq {
    pub percent: i32,
    pub last_seq: Option<i32>,
}
```

**🔑 핵심 포인트**:
- **`IntoParams`**: Query String 파라미터 Swagger 문서화
- **`sqlx::FromRow` 직접 derive**: DTO가 DB Row와 1:1 매핑
- **`crate::types::LessonItemKind`**: enum 재사용으로 타입 안전성 확보

**⚠️ 규칙**:
- lesson은 `#[serde(rename_all = "snake_case")]` 생략됨 (필드명이 이미 snake_case)
- 모든 DTO에 `rename_all` 명시 권장 (안전성)

---

##### 📊 DTO 공통 패턴 요약

| 패턴 | 설명 | 예시 |
|------|------|------|
| Request/Response 분리 | 주석으로 섹션 구분 | `// ===== Request DTOs =====` |
| snake_case 강제 | `#[serde(rename_all = "snake_case")]` | 모든 DTO에 적용 권장 |
| enum 재사용 | `crate::types::*` import | `UserGender`, `StudyTaskKind` 등 |
| 검증 | `validator::Validate` + 속성 | `#[validate(email)]`, `#[validate(range(...))]` |
| Swagger 예시 | `#[schema(example = json!(...))]` | Request DTO에 적용 |
| 날짜 형식 | `#[schema(value_type = String, format = "date")]` | `NaiveDate`, `DateTime<Utc>` |
| JSONB 매핑 | `Json<T>` + `#[schema(value_type = T)]` | `tags: Json<Vec<VideoTagDetail>>` |
| PATCH 패턴 | `Option<T>` + `skip_serializing_if` | `ProfileUpdateReq` |
| Tagged Union | `#[serde(tag = "kind")]` | `SubmitAnswerReq` |
| DB 컬럼 매핑 | `#[sqlx(rename = "...")]` | `VideoProgressRes` |

---

##### ⚠️ 현재 불일치 사항 (개선 권장)

| 항목 | 현재 상태 | 권장 |
|------|----------|------|
| **응답 배열 키** | video: `data`, lesson: `items`, study: `list` | `{ meta, data }`로 통일 |
| **enum 사용** | user/video 일부에서 String 사용 | 모두 `crate::types::*` enum으로 |
| **rename_all** | lesson만 생략 | 모든 DTO에 명시 |
| **IntoParams** | lesson만 사용 | Query DTO 전체에 적용 권장 |

---

##### 📋 dto.rs 표준 템플릿

```rust
// dto.rs (Best Practices Template)

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

// DB enum 재사용 (도메인별로 필요한 것만)
use crate::types::*;

// =====================================================================
// Request DTOs (Query / Path / Body)
// =====================================================================

/// (Query) List pagination
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListReq {
    #[serde(default = "default_page")]
    pub page: Option<u32>,
    #[serde(default = "default_per_page")]
    pub per_page: Option<u32>,
}

fn default_page() -> Option<u32> { Some(1) }
fn default_per_page() -> Option<u32> { Some(20) }

/// (Body) Create/Update - Validate 필수
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct CreateReq {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

// =====================================================================
// Response DTOs
// =====================================================================

/// Pagination meta (표준)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageMeta {
    pub total_count: i64,
    pub total_pages: u32,
    pub current_page: u32,
    pub per_page: u32,
}

/// List response (표준: meta + data)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListRes<T> {
    pub meta: PageMeta,
    pub data: Vec<T>,
}
```

---

##### 🔄 DTO ↔ DB 데이터 흐름

```
[HTTP Request]
      ↓
[DTO Request] → validator::Validate 검증
      ↓
[Service] → 비즈니스 로직
      ↓
[Repo] → sqlx 쿼리
      ↓
[DTO Response] ← sqlx::FromRow 또는 수동 매핑
      ↓
[HTTP Response] → serde::Serialize → JSON
```

---

#### 7.7.1-2. repo.rs
> **Claude 코드 분석 기반** (2025-01-22)

##### 📁 파일 개요

| 파일 | 라인수 | 구조 | 에러 타입 | 특징 |
|------|--------|------|-----------|------|
| [auth/repo.rs](src/api/auth/repo.rs) | 476 | `struct AuthRepo;` (stateless) | `AppResult` | TX 분리, FOR UPDATE |
| [user/repo.rs](src/api/user/repo.rs) | 286 | standalone functions | `AppResult` | RETURNING, audit log |
| [video/repo.rs](src/api/video/repo.rs) | 254 | `struct VideoRepo;` (stateless) | `AppResult` | QueryBuilder, JSONB |
| [study/repo.rs](src/api/study/repo.rs) | 467 | `struct StudyRepo;` (stateless) | `AppResult` | query_as! macro, Row→DTO 변환 |
| [lesson/repo.rs](src/api/lesson/repo.rs) | 232 | `struct LessonRepo { pool }` ⚠️ | `sqlx::Error` ⚠️ | Upsert, stateful |

##### 1️⃣ Auth Domain ([auth/repo.rs](src/api/auth/repo.rs))

**역할**: 로그인 세션 관리, 사용자 인증 정보 조회

**핵심 패턴**:

```rust
// 1. Internal Row Model (DB 전용 구조체)
#[derive(Debug, sqlx::FromRow)]
pub struct LoginRecord {
    pub user_id: i64,
    pub session_id: String,      // UUID → String 변환
    pub refresh_hash: String,
    pub login_ip: Option<String>, // Inet → String
    pub login_device: String,     // Enum → String
    // ...
}

// 2. TX vs Pool 함수 분리
pub async fn find_login_by_session_id_tx(
    tx: &mut Transaction<'_, Postgres>,
    session_id: &str,
) -> AppResult<Option<LoginRecord>> { /* ... */ }

pub async fn find_login_by_session_id(
    pool: &PgPool,
    session_id: &str,
) -> AppResult<Option<LoginRecord>> { /* ... */ }

// 3. FOR UPDATE Lock 패턴
pub async fn find_login_by_session_id_for_update_tx(
    tx: &mut Transaction<'_, Postgres>,
    session_id: &str,
) -> AppResult<Option<LoginRecord>> {
    sqlx::query_as::<_, LoginRecord>(r#"
        SELECT /* ... */
        FROM public.login
        WHERE login_session_id = CAST($1 AS uuid)
        FOR UPDATE  -- 동시성 제어
    "#)
    .bind(session_id)
    .fetch_optional(&mut **tx)
    .await?
}

// 4. Enum 안전 매핑 (CASE WHEN)
INSERT INTO public.login (/* ... */)
VALUES (
    $1,
    CASE lower($3)
        WHEN 'mobile'  THEN 'mobile'::login_device_enum
        WHEN 'tablet'  THEN 'tablet'::login_device_enum
        WHEN 'desktop' THEN 'desktop'::login_device_enum
        WHEN 'web'     THEN 'desktop'::login_device_enum
        ELSE 'other'::login_device_enum
    END,
    /* ... */
)
```

##### 2️⃣ User Domain ([user/repo.rs](src/api/user/repo.rs))

**역할**: 회원가입, 프로필/설정 CRUD, 감사 로그

**핵심 패턴**:

```rust
// 1. INSERT + RETURNING으로 즉시 응답 구성
pub async fn signup_tx(
    tx: &mut Transaction<'_, Postgres>,
    /* params... */
) -> AppResult<ProfileRes> {
    let res = sqlx::query_as::<_, ProfileRes>(r#"
        INSERT INTO users (/* ... */)
        VALUES ($1, $2, $3::user_language_enum, $4::user_gender_enum, /* ... */)
        RETURNING
            user_id as id,
            user_email as email,
            user_language::TEXT as language,  -- DB Enum → String
            user_gender as gender,             -- DB Enum → Rust Enum
            /* ... */
    "#)
    .bind(/* ... */)
    .fetch_one(&mut **tx)
    .await?;
    Ok(res)
}

// 2. PATCH 업데이트 (COALESCE 패턴)
pub async fn update_profile_tx(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    req: &ProfileUpdateReq,
) -> AppResult<Option<ProfileRes>> {
    sqlx::query_as::<_, ProfileRes>(r#"
        UPDATE users SET
            user_nickname = COALESCE($2, user_nickname),
            user_language = COALESCE($3::user_language_enum, user_language),
            user_country  = COALESCE($4, user_country)
        WHERE user_id = $1
        RETURNING /* ... */
    "#)
    .bind(user_id)
    .bind(req.nickname.as_ref())  // Option → bind
    .bind(req.language.as_ref())
    /* ... */
}

// 3. 감사 로그 (현재 row를 SELECT로 복사)
pub async fn insert_user_log_after_tx(
    tx: &mut Transaction<'_, Postgres>,
    actor_user_id: Option<i64>,
    user_id: i64,
    action: &str,
    success: bool,
) -> AppResult<()> {
    sqlx::query(r#"
        INSERT INTO public.users_log (
            updated_by_user_id, user_action_log, /* ... */
        )
        SELECT
            $1, CAST($2 AS user_action_log_enum), $3, u.user_id,
            u.user_auth, u.user_state, u.user_email, /* ... */
        FROM public.users u
        WHERE u.user_id = $4
    "#)
    /* ... */
}
```

##### 3️⃣ Video Domain ([video/repo.rs](src/api/video/repo.rs))

**역할**: 비디오 목록/상세 조회, 학습 진도 관리

**핵심 패턴**:

```rust
// 1. QueryBuilder로 동적 쿼리
pub async fn list_videos(
    pool: &PgPool,
    req: &VideoListReq,
) -> AppResult<(Vec<VideoListItem>, i64)> {
    let mut qb = QueryBuilder::new(r#"
        SELECT
            v.video_id::bigint as video_id,  -- INT4 → INT8 캐스팅
            COUNT(*) OVER() as total_count   -- Window Function
        FROM video v
        WHERE 1=1
    "#);

    // 동적 필터 추가
    if let Some(state) = &req.state {
        qb.push(" AND v.video_state = ");
        qb.push_bind(state);
        qb.push("::video_state_enum");
    }

    if let Some(q) = &req.q {
        qb.push(" AND (title ILIKE ");
        qb.push_bind(format!("%{}%", q));
        qb.push(")");
    }

    // 페이징
    qb.push(" LIMIT ").push_bind(req.per_page as i64);
    qb.push(" OFFSET ").push_bind(offset);

    let rows = qb.build().fetch_all(pool).await?;
    /* ... */
}

// 2. JSONB 집계
pub async fn get_video_detail(/* ... */) -> AppResult<Option<VideoDetailRes>> {
    sqlx::query_as::<_, VideoDetailRes>(r#"
        SELECT
            COALESCE(
                jsonb_agg(
                    jsonb_build_object(
                        'key', vt.video_tag_key,
                        'title', vt.video_tag_title
                    )
                ) FILTER (WHERE vt.video_tag_id IS NOT NULL),
                '[]'::jsonb
            ) as tags
        FROM video v
        LEFT JOIN video_tag vt ON /* ... */
        GROUP BY v.video_id
    "#)
}

// 3. Upsert + 조건부 업데이트
pub async fn update_progress(/* ... */) -> AppResult<VideoProgressRes> {
    sqlx::query_as::<_, VideoProgressRes>(r#"
        INSERT INTO video_log (user_id, video_id, video_progress_log, video_completed_log)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, video_id) DO UPDATE SET
            video_progress_log = EXCLUDED.video_progress_log,
            video_completed_log = CASE
                WHEN video_log.video_completed_log = true THEN true  -- 한번 완료면 유지
                ELSE EXCLUDED.video_completed_log
            END
        RETURNING /* ... */
    "#)
}
```

##### 4️⃣ Study Domain ([study/repo.rs](src/api/study/repo.rs))

**역할**: 학습 과제 조회, 채점, 상태 관리

**핵심 패턴**:

```rust
// 1. 내부 Row → DTO 변환 패턴
#[derive(sqlx::FromRow)]
struct StudyTaskDetailRow {
    task_id: i32,
    kind: StudyTaskKind,
    // LEFT JOIN 필드들 (모두 nullable)
    choice_1: Option<String>,
    typing_image_url: Option<String>,
    voice_audio_url: Option<String>,
}

impl StudyTaskDetailRow {
    fn map_to_res(self) -> Option<StudyTaskDetailRes> {
        let payload = match self.kind {
            StudyTaskKind::Choice => TaskPayload::Choice(ChoicePayload { /* ... */ }),
            StudyTaskKind::Typing => TaskPayload::Typing(TypingPayload { /* ... */ }),
            StudyTaskKind::Voice  => TaskPayload::Voice(VoicePayload { /* ... */ }),
        };
        Some(StudyTaskDetailRes { /* ... */ })
    }
}

// 2. sqlx::query_as! 매크로 (타입 명시)
pub async fn find_task_detail(
    pool: &PgPool,
    task_id: i64,
) -> AppResult<Option<StudyTaskDetailRes>> {
    let row = sqlx::query_as!(
        StudyTaskDetailRow,
        r#"
        SELECT
            t.study_task_id::INT AS task_id,
            t.study_task_kind AS "kind!: StudyTaskKind",       -- "!" = non-null 강제
            stc.study_task_choice_1::TEXT AS "choice_1?",      -- "?" = nullable 명시
            stt.study_task_typing_image_url::TEXT AS "typing_image_url?",
            stv.study_task_voice_audio_url::TEXT AS "voice_audio_url?"
        FROM study_task t
        LEFT JOIN study_task_choice stc ON t.study_task_id = stc.study_task_id
        LEFT JOIN study_task_typing stt ON t.study_task_id = stt.study_task_id
        LEFT JOIN study_task_voice stv ON t.study_task_id = stv.study_task_id
        WHERE t.study_task_id = $1
        "#,
        task_id as i32  // input 캐스팅
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(r.map_to_res()),
        None => Ok(None),
    }
}

// 3. Count + List 2쿼리 패턴
pub async fn find_open_studies(
    pool: &PgPool,
    page: u32,
    per_page: u32,
) -> AppResult<(Vec<StudySummaryDto>, i64)> {
    // A. Count
    let count: i64 = QueryBuilder::new("SELECT COUNT(*) FROM study WHERE ...")
        .build_query_scalar()
        .fetch_one(pool)
        .await?;

    // B. List
    let list = QueryBuilder::new("SELECT /* ... */ FROM study WHERE ...")
        .push(" ORDER BY ...")
        .push(" LIMIT ").push_bind(per_page)
        .push(" OFFSET ").push_bind(offset)
        .build_query_as::<StudySummaryDto>()
        .fetch_all(pool)
        .await?;

    Ok((list, count))
}
```

##### 5️⃣ Lesson Domain ([lesson/repo.rs](src/api/lesson/repo.rs))

**역할**: 레슨 목록/상세, 아이템 조회, 진도 관리

**핵심 패턴**:

```rust
// 1. Upsert (ON CONFLICT)
pub async fn upsert_progress(
    &self,
    lesson_id: i64,
    user_id: i64,
    percent: i32,
    last_seq: Option<i32>,
) -> Result<LessonProgressRes, sqlx::Error> {
    sqlx::query_as::<_, LessonProgressRes>(r#"
        INSERT INTO lesson_progress (
            lesson_id, user_id, lesson_progress_percent,
            lesson_progress_last_item_seq, lesson_progress_last_progress_at
        )
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (lesson_id, user_id) DO UPDATE SET
            lesson_progress_percent = EXCLUDED.lesson_progress_percent,
            lesson_progress_last_item_seq = EXCLUDED.lesson_progress_last_item_seq,
            lesson_progress_last_progress_at = EXCLUDED.lesson_progress_last_progress_at
        RETURNING /* ... */
    "#)
    .bind(lesson_id)
    .bind(user_id)
    .bind(percent)
    .bind(last_seq)  -- Option<i32> 직접 바인딩
    .fetch_one(&self.pool)
    .await
}

// 2. EXISTS 체크
pub async fn exists_lesson(&self, lesson_id: i64) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(r#"
        SELECT EXISTS(SELECT 1 FROM lesson WHERE lesson_id = $1)
    "#)
    .bind(lesson_id)
    .fetch_one(&self.pool)
    .await
}
```

##### 📊 공통 패턴 요약

| 패턴 | 용도 | 사용처 | 코드 예시 |
|------|------|--------|----------|
| **TX 분리** | 동일 쿼리의 Pool/TX 버전 | auth | `_tx` suffix |
| **FOR UPDATE** | 동시성 제어 (refresh) | auth | `FOR UPDATE` lock |
| **RETURNING** | INSERT 후 즉시 반환 | user, video, lesson | `RETURNING col AS alias` |
| **COALESCE** | PATCH nullable 처리 | user, video | `COALESCE($2, col)` |
| **QueryBuilder** | 동적 WHERE/ORDER | video, study | `push_bind()` |
| **COUNT OVER()** | 1쿼리 페이징 | video | `COUNT(*) OVER()` |
| **Count+List** | 2쿼리 페이징 | study | 별도 count 쿼리 |
| **query_as!** | 타입 안전 쿼리 | study | `"field!: Type"`, `"field?"` |
| **Row→DTO** | 다형성 변환 | study | `map_to_res()` |
| **ON CONFLICT** | Upsert | video, lesson | `DO UPDATE SET` |
| **CASE WHEN** | Enum 안전 매핑 | auth | `CASE lower($x) WHEN...` |
| **JSONB agg** | 1:N 집계 | video | `jsonb_agg(jsonb_build_object())` |

##### ⚠️ 현재 불일치/개선 필요 사항

| 이슈 | 현재 상태 | 권장 표준 | 파일 |
|------|----------|----------|------|
| **에러 타입** | `sqlx::Error` | `AppResult<T>` | lesson_repo |
| **Repo 구조** | `LessonRepo { pool }` (stateful) | `struct XxxRepo;` (stateless) | lesson_repo |
| **TX 책임** | repo가 tx 시작 | service가 tx 관리 | study_repo.submit_grade_tx |
| **nullable 매핑** | `refresh_hash: String` | `Option<String>` | auth_repo.LoginRecord |

##### 📋 표준 템플릿

```rust
// repo.rs (AMK 표준 골격)
use sqlx::{PgPool, Postgres, Transaction, QueryBuilder};
use crate::error::AppResult;

// ✅ Stateless 구조체
pub struct XxxRepo;

impl XxxRepo {
    // =====================================================
    // A. 단건 조회
    // =====================================================
    pub async fn find_by_id(pool: &PgPool, id: i64) -> AppResult<Option<XxxDto>> {
        sqlx::query_as::<_, XxxDto>(r#"
            SELECT xxx_id::bigint as id, /* ... */
            FROM xxx WHERE xxx_id = $1
        "#)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(Into::into)
    }

    // =====================================================
    // B. 존재 여부
    // =====================================================
    pub async fn exists(pool: &PgPool, id: i64) -> AppResult<bool> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM xxx WHERE xxx_id = $1)"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    // =====================================================
    // C. 리스트 (Count + List 패턴)
    // =====================================================
    pub async fn list(
        pool: &PgPool,
        page: u32,
        per_page: u32,
    ) -> AppResult<(Vec<XxxDto>, i64)> {
        // 1) Count
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM xxx")
            .fetch_one(pool).await?;

        // 2) List
        let offset = (page as i64 - 1) * per_page as i64;
        let list = sqlx::query_as::<_, XxxDto>(r#"
            SELECT /* ... */ FROM xxx
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        "#)
        .bind(per_page as i64)
        .bind(offset)
        .fetch_all(pool).await?;

        Ok((list, count))
    }

    // =====================================================
    // D. 쓰기 (TX 버전)
    // =====================================================
    pub async fn create_tx(
        tx: &mut Transaction<'_, Postgres>,
        req: &XxxReq,
    ) -> AppResult<XxxDto> {
        sqlx::query_as::<_, XxxDto>(r#"
            INSERT INTO xxx (col1, col2)
            VALUES ($1, $2::some_enum)
            RETURNING xxx_id::bigint as id, col1, col2
        "#)
        .bind(&req.col1)
        .bind(&req.col2)
        .fetch_one(&mut **tx)
        .await
        .map_err(Into::into)
    }

    // =====================================================
    // E. Upsert
    // =====================================================
    pub async fn upsert(pool: &PgPool, key: i64, val: i32) -> AppResult<XxxDto> {
        sqlx::query_as::<_, XxxDto>(r#"
            INSERT INTO xxx (key_col, val_col)
            VALUES ($1, $2)
            ON CONFLICT (key_col) DO UPDATE SET
                val_col = EXCLUDED.val_col
            RETURNING /* ... */
        "#)
        .bind(key)
        .bind(val)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }
}
```

##### 🔄 데이터 흐름

```
[Service] → pool or tx 전달
      ↓
[Repo Function]
      ↓
sqlx::query_as / query_scalar / query
      ↓
.bind() → Parameter 바인딩
      ↓
.fetch_one/optional/all(&mut **tx) or (pool)
      ↓
Result<T, sqlx::Error> → AppResult<T> 변환
      ↓
[Service]로 반환
```

---

#### 7.7.1-3. service.rs
> **Claude 코드 분석 기반** (2025-01-22)

##### 📁 파일 개요

| 파일 | 라인수 | 구조 | 주요 역할 | 특징 |
|------|--------|------|----------|------|
| [auth/service.rs](src/api/auth/service.rs) | 569 | `struct AuthService;` (stateless) | 로그인, 토큰 갱신, 로그아웃 | Rate limit, Refresh rotation |
| [user/service.rs](src/api/user/service.rs) | 266 | `struct UserService;` (stateless) | 회원가입, 프로필, 설정 | Auto login, Validation |
| [video/service.rs](src/api/video/service.rs) | 105 | `struct VideoService;` (stateless) | 비디오 목록, 진도 관리 | 단순 CRUD |
| [study/service.rs](src/api/study/service.rs) | 309 | `struct StudyService;` (stateless) | 학습 과제, 채점, 해설 | Enum 파싱, Optional 로깅 |
| [lesson/service.rs](src/api/lesson/service.rs) | 197 | `struct LessonService { repo }` ⚠️ | 레슨 목록, 진도 | Stateful (다른 패턴) |

##### 1️⃣ Auth Domain ([auth/service.rs](src/api/auth/service.rs))

**역할**: 인증 전체 흐름 (로그인/리프레시/로그아웃/비밀번호 재설정)

**핵심 패턴**:

```rust
// 1. 타이밍 공격 방지 (Dummy Hash)
fn dummy_password_hash() -> AppResult<PasswordHash<'static>> {
    static DUMMY_HASH: OnceLock<String> = OnceLock::new();
    let hash_str = DUMMY_HASH.get_or_init(|| {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(b"dummy_password", &salt)
            .expect("argon2 dummy hash should succeed")
            .to_string()
    });
    PasswordHash::new(hash_str).map_err(|_| AppError::Internal("...".into()))
}

// 2. Rate Limiting (Redis INCR + EXPIRE)
let rl_key = format!("rl:login:{}:{}", email, login_ip);
let mut redis_conn = st.redis.get().await?;

let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
if attempts == 1 {
    let _: () = redis_conn.expire(&rl_key, st.cfg.rate_limit_login_window_sec).await?;
}
if attempts > st.cfg.rate_limit_login_max {
    return Err(AppError::TooManyRequests("AUTH_429_TOO_MANY_ATTEMPTS".into()));
}

// 3. Refresh Token Rotation + Reuse Detection
pub async fn refresh(st: &AppState, old_refresh_token: &str, ...) -> AppResult<...> {
    let (session_id, incoming_hash) = Self::parse_refresh_token(old_refresh_token)?;

    // DB Lock (FOR UPDATE)
    let mut tx = st.db.begin().await?;
    let login_record = AuthRepo::find_login_by_session_id_for_update_tx(&mut tx, &session_id).await?;

    // Reuse Detection (Critical Security)
    if login_record.refresh_hash != incoming_hash {
        warn!("Refresh token reuse detected! Session: {}", session_id);
        AuthRepo::update_login_state_by_session_tx(&mut tx, &session_id, "compromised").await?;
        tx.commit().await?;

        // Redis 즉시 무효화
        let _ = redis_conn.del(format!("ak:refresh:{}", login_record.refresh_hash)).await;
        let _ = redis_conn.del(format!("ak:session:{}", session_id)).await;

        return Err(AppError::Conflict("AUTH_409_REUSE_DETECTED".into()));
    }

    // Rotate: 새 토큰 발급
    let (new_refresh_token, new_refresh_hash) = Self::generate_refresh_token_and_hash(&session_id);
    AuthRepo::update_login_refresh_hash_tx(&mut tx, &session_id, &new_refresh_hash).await?;
    tx.commit().await?;

    // Redis Sync (Old 삭제 → New 등록)
    let _: () = redis_conn.del(format!("ak:refresh:{}", login_record.refresh_hash)).await?;
    let _: () = redis_conn.set_ex(format!("ak:refresh:{}", new_refresh_hash), &session_id, ttl).await?;
    // ...
}

// 4. DB TX 후 Redis 반영 패턴 (Login 예시)
pub async fn login(st: &AppState, req: LoginReq, ...) -> AppResult<(LoginRes, Cookie, i64)> {
    // ... validation, password verify ...

    // [DB Transaction]
    let mut tx = st.db.begin().await?;
    AuthRepo::insert_login_record_tx(&mut tx, ...).await?;
    AuthRepo::insert_login_log_tx(&mut tx, ...).await?;
    tx.commit().await?;  // DB 먼저 커밋

    // [Redis Caching] - 커밋 후 실행
    let _: () = redis_conn.set_ex(format!("ak:session:{}", session_id), user_id, ttl).await?;
    let _: () = redis_conn.set_ex(format!("ak:refresh:{}", refresh_hash), &session_id, ttl).await?;

    // [Cookie 생성]
    let mut refresh_cookie = Cookie::new(st.cfg.refresh_cookie_name.clone(), refresh_token);
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(st.cfg.refresh_cookie_secure);
    // ...
}
```

##### 2️⃣ User Domain ([user/service.rs](src/api/user/service.rs))

**역할**: 회원가입 (자동 로그인 포함), 프로필/설정 CRUD

**핵심 패턴**:

```rust
// 1. DB Unique Violation 감지 (PG 에러 코드)
const PG_UNIQUE_VIOLATION: &'static str = "23505";

fn is_unique_violation(err: &AppError) -> bool {
    if let AppError::Sqlx(sqlx::Error::Database(db)) = err {
        db.code().as_deref() == Some(Self::PG_UNIQUE_VIOLATION)
    } else {
        false
    }
}

// 2. 사전 체크 + DB 최종 방어 패턴
pub async fn signup(st: &AppState, req: SignupReq, ...) -> AppResult<...> {
    // [사전 체크]
    if repo::find_user_id_by_email(&st.db, &req.email).await?.is_some() {
        return Err(AppError::Conflict("Email already exists".into()));
    }

    // [DB Insert with Unique Constraint]
    let user = match repo::signup_tx(&mut tx, ...).await {
        Ok(u) => u,
        Err(e) if Self::is_unique_violation(&e) => {
            return Err(AppError::Conflict("Email exists".into()))  // 동시 요청 방어
        },
        Err(e) => return Err(e),
    };
    // ...
}

// 3. Best-Effort 감사 로그 (실패해도 업무 흐름 유지)
if let Err(e) = repo::insert_user_log_after_tx(&mut tx, Some(user.id), user.id, "signup", true).await {
    warn!(error = ?e, user_id = user.id, "Failed to insert signup log");
    // 로그 실패해도 계속 진행
}

// 4. 기본값 Fallback (설정 없으면 기본값 반환)
pub async fn get_settings(st: &AppState, user_id: i64) -> AppResult<SettingsRes> {
    let settings = repo::find_users_setting(&st.db, user_id).await?;

    Ok(settings.unwrap_or_else(|| SettingsRes {
        user_set_language: "ko".to_string(),
        user_set_timezone: "UTC".to_string(),
        user_set_note_email: false,
        user_set_note_push: false,
        updated_at: chrono::Utc::now(),
    }))
}
```

##### 3️⃣ Video Domain ([video/service.rs](src/api/video/service.rs))

**역할**: 비디오 목록/상세 조회, 진도율 관리

**핵심 패턴**:

```rust
// 1. 단순 Validation → Repo → Meta 계산 패턴
pub async fn list_videos(st: &AppState, req: VideoListReq) -> AppResult<VideoListRes> {
    // Validation
    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    // Repo 호출 (Data + Total Count)
    let (data, total_count) = VideoRepo::list_videos(&st.db, &req).await?;

    // Meta 계산
    let total_pages = if total_count == 0 { 0 }
    else { (total_count + req.per_page as i64 - 1) / req.per_page as i64 };

    Ok(VideoListRes { meta: VideoListMeta { total_count, total_pages, ... }, data })
}

// 2. 존재 확인 후 기본값 반환 패턴
pub async fn get_video_progress(st: &AppState, user_id: i64, video_id: i64) -> AppResult<VideoProgressRes> {
    // 비디오 존재 확인
    if !VideoRepo::exists_by_id(&st.db, video_id).await? {
        return Err(AppError::NotFound);
    }

    // 진도 조회 → 없으면 기본값
    let progress = VideoRepo::find_progress(&st.db, user_id, video_id).await?;
    Ok(progress.unwrap_or_else(|| VideoProgressRes {
        video_id,
        progress_rate: 0,
        is_completed: false,
        last_watched_at: None,
    }))
}
```

##### 4️⃣ Study Domain ([study/service.rs](src/api/study/service.rs))

**역할**: 학습 과제 조회, 채점, 상태/해설 관리

**핵심 패턴**:

```rust
// 1. Enum 문자열 파싱 (헬퍼 함수)
fn parse_study_program(value: &str) -> Option<StudyProgram> {
    match value {
        "basic_pronunciation" => Some(StudyProgram::BasicPronunciation),
        "basic_word" => Some(StudyProgram::BasicWord),
        "topik_read" => Some(StudyProgram::TopikRead),
        // ...
        _ => None,
    }
}

// 2. 요청 Kind ↔ DB Kind 일치 검증
pub async fn submit_answer(st: &AppState, auth: AuthUser, task_id: i32, req: SubmitAnswerReq) -> AppResult<SubmitAnswerRes> {
    let answer_key = StudyRepo::find_answer_key(&st.db, task_id).await?.ok_or(AppError::NotFound)?;

    let req_kind = match &req {
        SubmitAnswerReq::Choice { .. } => StudyTaskKind::Choice,
        SubmitAnswerReq::Typing { .. } => StudyTaskKind::Typing,
        SubmitAnswerReq::Voice { .. } => StudyTaskKind::Voice,
    };

    if req_kind != answer_key.kind {
        return Err(AppError::BadRequest("Task kind mismatch".into()));
    }
    // ... 채점 로직 ...
}

// 3. Best-Effort 액션 로깅 (실패해도 경고만)
pub async fn get_study_task(st: &AppState, task_id: i32, auth: Option<AuthUser>) -> AppResult<StudyTaskDetailRes> {
    let task = StudyRepo::find_task_detail(&st.db, i64::from(task_id)).await?.ok_or(AppError::NotFound)?;

    if let Some(AuthUser(claims)) = auth {
        if let Err(err) = StudyRepo::log_task_action(&st.db, claims.sub, &claims.session_id, task_id, StudyTaskLogAction::View).await {
            warn!(error = ?err, user_id = claims.sub, task_id, "Failed to log study task view");
            // 로그 실패해도 계속 반환
        }
    }
    Ok(task)
}

// 4. 권한 검증 (시도 횟수 기반)
pub async fn get_task_explain(st: &AppState, auth: AuthUser, task_id: i32) -> AppResult<TaskExplainRes> {
    let try_count = StudyRepo::get_try_count(&st.db, auth.0.sub, task_id).await?;
    if try_count < 1 {
        return Err(AppError::Forbidden);  // 1회 이상 시도해야 해설 조회 가능
    }
    // ...
}
```

##### 5️⃣ Lesson Domain ([lesson/service.rs](src/api/lesson/service.rs))

**역할**: 레슨 목록/상세, 진도 관리

**핵심 패턴**: ⚠️ **Stateful 구조 (다른 도메인과 다름)**

```rust
// ⚠️ 다른 Service와 달리 repo를 필드로 소유
pub struct LessonService {
    repo: LessonRepo,  // AppState가 아닌 직접 소유
}

impl LessonService {
    pub fn new(repo: LessonRepo) -> Self {
        Self { repo }
    }

    pub async fn list_lessons(&self, req: LessonListReq) -> AppResult<LessonListRes> {
        // &self.repo 사용 (AppState 미사용)
        let total_count = self.repo.count_all().await?;  // sqlx::Error 반환 ⚠️
        let items = self.repo.find_all(per_page, offset).await?;
        // ...
    }
}
```

##### 📊 공통 패턴 요약

| 패턴 | 용도 | 사용처 | 코드 |
|------|------|--------|------|
| **Rate Limiting** | 브루트포스 방지 | auth, user | `redis.incr()` + `expire()` |
| **Timing Attack 방어** | 로그인 보안 | auth | `dummy_password_hash()` |
| **Refresh Rotation** | 토큰 탈취 감지 | auth | FOR UPDATE + hash 비교 |
| **DB→Redis 순서** | 일관성 보장 | auth, user | `tx.commit()` 후 Redis |
| **Unique Violation** | 중복 방어 | user | `code == "23505"` |
| **Best-Effort Log** | 로깅 실패 허용 | user, study | `warn!()` + 계속 진행 |
| **Default Fallback** | 데이터 없을 때 | video, study, lesson | `unwrap_or_else(|| default)` |
| **Kind Mismatch** | 타입 검증 | study | 요청 Kind ↔ DB Kind |
| **권한 검증** | 접근 제어 | study | `try_count < 1` → Forbidden |

##### ⚠️ 현재 불일치/개선 필요 사항

| 이슈 | 현재 상태 | 권장 표준 | 파일 |
|------|----------|----------|------|
| **Refresh Token 포맷** | `session_id:uuid` (auth) vs `random_32bytes` (user) | 통일 필요 | auth vs user |
| **Service 구조** | `LessonService { repo }` (stateful) | `struct XxxService;` (stateless) | lesson |
| **에러 타입** | `sqlx::Error` 직접 반환 | `AppResult<T>` | lesson (via repo) |
| **SADD 누락** | login에서 user_sessions SADD 안함 | SADD 추가 필요 | auth |
| **set_domain 중복** | 2번 호출 | 1번으로 정리 | auth |

##### 📋 표준 템플릿

```rust
// service.rs (AMK 표준 골격)
use crate::{error::{AppError, AppResult}, state::AppState};
use super::{dto::*, repo};
use validator::Validate;
use tracing::warn;

// ✅ Stateless 구조체
pub struct XxxService;

impl XxxService {
    // =====================================================
    // A. 목록 조회 (Validation → Repo → Meta)
    // =====================================================
    pub async fn list(st: &AppState, req: XxxListReq) -> AppResult<XxxListRes> {
        // 1. Validation
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 2. Repo 호출
        let (list, total_count) = repo::find_list(&st.db, &req).await?;

        // 3. Meta 계산
        let total_pages = if total_count == 0 { 0 }
        else { (total_count + req.per_page - 1) / req.per_page };

        Ok(XxxListRes { list, meta: XxxMeta { total_count, total_pages, ... } })
    }

    // =====================================================
    // B. 상세 조회 (Exists Check → Fetch)
    // =====================================================
    pub async fn get_detail(st: &AppState, id: i64) -> AppResult<XxxDetail> {
        let item = repo::find_by_id(&st.db, id).await?.ok_or(AppError::NotFound)?;
        Ok(item)
    }

    // =====================================================
    // C. 쓰기 (TX → Log → Commit)
    // =====================================================
    pub async fn create(st: &AppState, user_id: i64, req: XxxCreateReq) -> AppResult<XxxRes> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

        let mut tx = st.db.begin().await?;

        let result = repo::create_tx(&mut tx, &req).await?;

        // Best-effort logging
        if let Err(e) = repo::log_action_tx(&mut tx, user_id, "create").await {
            warn!(error = ?e, "Failed to log action");
        }

        tx.commit().await?;
        Ok(result)
    }
}
```

##### 🔄 데이터 흐름

```
[Handler] → AppState, AuthUser, Req DTO 추출
      ↓
[Service] → Validation → Rate Limit (Redis) → Business Logic
      ↓
[Repo] → DB Query (TX or Pool)
      ↓
[Service] → DB Commit → Redis Sync → Response 구성
      ↓
[Handler] → HTTP Response (JSON)
```

---

#### 7.7.1-4. handler.rs
> **Claude 코드 분석 기반** (2025-01-22)

##### 📁 파일 개요

| 파일 | 라인수 | Extractor 사용 | 주요 역할 | 특징 |
|------|--------|---------------|----------|------|
| [auth/handler.rs](src/api/auth/handler.rs) | 282 | State, HeaderMap, CookieJar, Json, AuthUser | 로그인, 토큰 갱신, 로그아웃 | Cookie 직접 관리 |
| [user/handler.rs](src/api/user/handler.rs) | 240 | State, HeaderMap, CookieJar, Json, AuthUser | 회원가입, 프로필, 설정 | 201 + Location 헤더 |
| [video/handler.rs](src/api/video/handler.rs) | 117 | State, Query, Path, Json, AuthUser | 비디오 목록, 진도 | 완전히 얇은 레이어 |
| [study/handler.rs](src/api/study/handler.rs) | 142 | State, Query, Path, Json, AuthUser, OptionalAuthUser | 학습 과제, 채점 | Optional 인증 |
| [lesson/handler.rs](src/api/lesson/handler.rs) | 150 | State, Query, Path, Json, AuthUser | 레슨 목록, 진도 | Service 인스턴스화 ⚠️ |

##### 1️⃣ Auth Domain ([auth/handler.rs](src/api/auth/handler.rs))

**역할**: 로그인, 토큰 갱신, 로그아웃, 아이디 찾기, 비밀번호 재설정

**핵심 패턴**:

```rust
// 1. Client Context 추출 헬퍼
fn extract_client_ip(headers: &HeaderMap) -> String {
    // x-forwarded-for → x-real-ip → fallback 순서
    if let Some(v) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = v.split(',').next() {
            let ip = first.trim();
            if !ip.is_empty() { return ip.to_string(); }
        }
    }
    if let Some(v) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        let ip = v.trim();
        if !ip.is_empty() { return ip.to_string(); }
    }
    // Fallback (env 설정 가능)
    "127.0.0.1".to_string()
}

fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers.get("user-agent").and_then(|v| v.to_str().ok()).map(|s| s.to_string())
}

// 2. Login Handler - Service가 Cookie 반환
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginReq,
    responses(
        (status = 200, description = "Login successful", body = LoginRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody)
    )
)]
pub async fn login(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<LoginReq>,
) -> Result<(CookieJar, Json<LoginRes>), AppError> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    // Service가 Cookie까지 생성해서 반환
    let (login_res, cookie, _) = AuthService::login(&st, req, ip, ua).await?;
    let jar = jar.add(cookie);

    Ok((jar, Json(login_res)))
}

// 3. Refresh Handler - Handler가 Cookie 직접 생성
pub async fn refresh(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<(CookieJar, Json<LoginRes>), AppError> {
    // 쿠키에서 리프레시 토큰 추출
    let refresh_token = jar
        .get(&st.cfg.refresh_cookie_name)
        .map(|c| c.value().to_string())
        .ok_or(AppError::Unauthorized("Missing refresh token".into()))?;

    let (refresh_res, new_token_str, ttl_secs) =
        AuthService::refresh(&st, &refresh_token, ip, ua).await?;

    // Handler에서 쿠키 직접 설정 (Rotation)
    let mut refresh_cookie = Cookie::new(st.cfg.refresh_cookie_name.clone(), new_token_str);
    refresh_cookie.set_path("/");
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(st.cfg.refresh_cookie_secure);
    refresh_cookie.set_same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    });
    refresh_cookie.set_expires(OffsetDateTime::now_utc() + Duration::seconds(ttl_secs));

    if let Some(domain) = &st.cfg.refresh_cookie_domain {
        refresh_cookie.set_domain(domain.clone());
    }

    Ok((jar.add(refresh_cookie), Json(refresh_res)))
}

// 4. Logout Handler - Cookie 만료 설정
pub async fn logout(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), AppError> {
    AuthService::logout(&st, auth_user.sub, &auth_user.session_id, ip, ua).await?;

    // 쿠키 만료 (과거 시간 설정)
    let mut refresh_cookie = Cookie::new(st.cfg.refresh_cookie_name.clone(), "");
    refresh_cookie.set_expires(OffsetDateTime::now_utc() - Duration::days(1));
    // ... 기타 속성 설정

    Ok((jar.add(refresh_cookie), StatusCode::NO_CONTENT))
}
```

##### 2️⃣ User Domain ([user/handler.rs](src/api/user/handler.rs))

**역할**: 회원가입 (자동 로그인), 프로필/설정 CRUD

**핵심 패턴**:

```rust
// 1. Signup - 201 + Location 헤더 + Cookie
#[utoipa::path(
    post,
    path = "/users",
    tag = "user",
    request_body = SignupReq,
    responses(
        (status = 201, description = "회원가입 성공 (자동 로그인)", body = SignupRes),
        (status = 409, description = "이메일 중복", body = crate::error::ErrorBody)
    )
)]
pub async fn signup(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<SignupReq>,
) -> AppResult<(CookieJar, (StatusCode, HeaderMap, Json<SignupRes>))> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let (res, refresh_token, refresh_ttl_secs) = UserService::signup(&st, req, ip, ua).await?;

    // Cookie::build() 방식 (다른 스타일)
    let refresh_cookie = Cookie::build(Cookie::new(
        st.cfg.refresh_cookie_name.clone(),
        refresh_token,
    ))
    .path("/")
    .http_only(true)
    .secure(st.cfg.refresh_cookie_secure)
    .same_site(...)
    .expires(...)
    .domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default())  // ⚠️
    .build();

    // Location 헤더 설정 (RESTful)
    let mut resp_headers = HeaderMap::new();
    let location = format!("/users/{}", res.user_id);
    resp_headers.insert(axum::http::header::LOCATION, HeaderValue::from_str(&location)?);

    Ok((jar.add(refresh_cookie), (StatusCode::CREATED, resp_headers, Json(res))))
}

// 2. 얇은 Handler 패턴 (대부분의 엔드포인트)
pub async fn get_me(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
) -> AppResult<Json<ProfileRes>> {
    let user = UserService::get_me(&st, auth_user.sub).await?;
    Ok(Json(user))
}

pub async fn update_me(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Json(req): Json<ProfileUpdateReq>,
) -> AppResult<Json<ProfileRes>> {
    let user = UserService::update_me(&st, auth_user.sub, req).await?;
    Ok(Json(user))
}
```

##### 3️⃣ Video Domain ([video/handler.rs](src/api/video/handler.rs))

**역할**: 비디오 목록/상세, 학습 진도 관리

**핵심 패턴**: **완전히 얇은 Handler** (Best Practice)

```rust
// 1. Query Parameter + OpenAPI 수동 문서화
#[utoipa::path(
    get,
    path = "/videos",
    params(
        ("page" = Option<u64>, Query, description = "Page number (default 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default 20, max 100)"),
        ("q" = Option<String>, Query, description = "Search query"),
        ("tag" = Option<String>, Query, description = "Filter by tag key"),
        ("state" = Option<String>, Query, description = "Filter by state")
    ),
    responses((status = 200, description = "List of videos", body = VideoListRes)),
    tag = "videos"
)]
pub async fn list_videos(
    State(state): State<AppState>,
    Query(req): Query<VideoListReq>,
) -> AppResult<Json<VideoListRes>> {
    let res = VideoService::list_videos(&state, req).await?;
    Ok(Json(res))
}

// 2. Path Parameter (IdParam DTO 사용)
pub async fn get_video_detail(
    State(state): State<AppState>,
    Path(IdParam { id }): Path<IdParam>,
) -> AppResult<Json<VideoDetailRes>> {
    let video = VideoService::get_video_detail(&state, id).await?;
    Ok(Json(video))
}

// 3. 인증 필요 엔드포인트
#[utoipa::path(
    // ...
    security(("bearerAuth" = [])),
    tag = "videos"
)]
pub async fn get_video_progress(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,  // 인증 필수
    Path(IdParam { id }): Path<IdParam>,
) -> AppResult<Json<VideoProgressRes>> {
    let progress = VideoService::get_video_progress(&state, auth_user.sub, id).await?;
    Ok(Json(progress))
}
```

##### 4️⃣ Study Domain ([study/handler.rs](src/api/study/handler.rs))

**역할**: 학습 목록, 문제 상세, 정답 제출, 상태/해설 조회

**핵심 패턴**: **OptionalAuthUser** (비로그인 접근 허용)

```rust
// 1. Optional Auth - 비로그인도 접근 가능
pub async fn get_study_task(
    State(state): State<AppState>,
    OptionalAuthUser(auth): OptionalAuthUser,  // ⭐ Optional
    Path(task_id): Path<i32>,
) -> AppResult<Json<StudyTaskDetailRes>> {
    let res = StudyService::get_study_task(&state, task_id, auth).await?;
    Ok(Json(res))
}

// 2. AuthUser 전체 전달 패턴 (다른 도메인과 다름)
pub async fn submit_answer(
    State(state): State<AppState>,
    auth_user: AuthUser,  // ⚠️ 구조 분해 없이 전체 전달
    Path(task_id): Path<i32>,
    Json(req): Json<SubmitAnswerReq>,
) -> AppResult<Json<SubmitAnswerRes>> {
    let res = StudyService::submit_answer(&state, auth_user, task_id, req).await?;
    Ok(Json(res))
}

// 3. Forbidden 응답 가능 엔드포인트
#[utoipa::path(
    get,
    path = "/studies/tasks/{id}/explain",
    responses(
        (status = 200, description = "Task Explanation", body = TaskExplainRes),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody)  // 1회 이상 풀어야 조회 가능
    ),
    security(("bearerAuth" = [])),
    tag = "study"
)]
pub async fn get_task_explain(/* ... */) { /* ... */ }
```

##### 5️⃣ Lesson Domain ([lesson/handler.rs](src/api/lesson/handler.rs))

**역할**: 레슨 목록/상세, 아이템, 진도 관리

**핵심 패턴**: ⚠️ **Service 인스턴스화** (다른 도메인과 다름)

```rust
// ⚠️ 매 요청마다 Service 인스턴스 생성
pub async fn list_lessons(
    State(state): State<AppState>,
    Query(req): Query<LessonListReq>,
) -> AppResult<Json<LessonListRes>> {
    // 다른 도메인: VideoService::list_videos(&state, req)
    // Lesson: 인스턴스화 필요
    let service = LessonService::new(LessonRepo::new(state.db.clone()));
    let res = service.list_lessons(req).await?;
    Ok(Json(res))
}

pub async fn get_lesson_progress(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(lesson_id): Path<i64>,
) -> AppResult<Json<LessonProgressRes>> {
    let service = LessonService::new(LessonRepo::new(state.db.clone()));
    let res = service.get_lesson_progress(auth_user.sub, lesson_id).await?;
    Ok(Json(res))
}
```

##### 📊 공통 패턴 요약

| 패턴 | 용도 | 사용처 | 코드 예시 |
|------|------|--------|----------|
| **State<AppState>** | 전역 상태 주입 | 모든 handler | `State(st): State<AppState>` |
| **Json<Req>** | JSON Body 추출 | POST/PUT/PATCH | `Json(req): Json<LoginReq>` |
| **Query(Req)** | Query String 추출 | GET (목록) | `Query(req): Query<VideoListReq>` |
| **Path(id)** | Path Parameter 추출 | 상세/수정/삭제 | `Path(id): Path<i64>` |
| **AuthUser** | 인증 필수 | 보호된 엔드포인트 | `AuthUser(auth): AuthUser` |
| **OptionalAuthUser** | 인증 선택 | 비로그인 허용 | `OptionalAuthUser(auth)` |
| **CookieJar** | 쿠키 관리 | auth, user | `jar: CookieJar` |
| **HeaderMap** | 헤더 추출 | IP/UA 필요 시 | `headers: HeaderMap` |

##### ⚠️ 현재 불일치/개선 필요 사항

| 이슈 | 현재 상태 | 권장 표준 | 파일 |
|------|----------|----------|------|
| **헬퍼 중복** | auth, user 각각 정의 | 공통 모듈로 추출 | `api/common/http.rs` |
| **쿠키 생성 책임** | login: service, refresh/signup: handler | 한 곳으로 통일 | auth, user |
| **쿠키 domain 설정** | `unwrap_or_default()` | `if let Some()` 패턴 | user_handler |
| **Service 호출 방식** | lesson만 인스턴스화 | stateless 통일 | lesson_handler |
| **AuthUser 전달** | study만 전체 전달 | 구조 분해 통일 | study_handler |
| **반환 타입** | auth: `Result<_, AppError>`, 기타: `AppResult` | `AppResult` 통일 | auth_handler |

##### 📋 표준 템플릿

```rust
// handler.rs (AMK 표준 골격)
use axum::{
    extract::{State, Path, Query},
    http::HeaderMap,
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

use crate::{state::AppState, error::AppResult};
use crate::api::auth::extractor::{AuthUser, OptionalAuthUser};
use super::{dto::*, service::XxxService};

// =====================================================================
// 공통 헬퍼 (권장: api/common/http.rs로 분리)
// =====================================================================

fn extract_client_ip(headers: &HeaderMap) -> String {
    headers.get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| headers.get("x-real-ip")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim().to_string()))
        .unwrap_or_else(|| "127.0.0.1".to_string())
}

fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers.get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

// =====================================================================
// 1. 공개 목록 (인증 불필요)
// =====================================================================
#[utoipa::path(
    get,
    path = "/xxx",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List", body = XxxListRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody)
    ),
    tag = "xxx"
)]
pub async fn list_xxx(
    State(st): State<AppState>,
    Query(req): Query<XxxListReq>,
) -> AppResult<Json<XxxListRes>> {
    let res = XxxService::list(&st, req).await?;
    Ok(Json(res))
}

// =====================================================================
// 2. 상세 조회 (선택적 인증)
// =====================================================================
#[utoipa::path(
    get,
    path = "/xxx/{id}",
    params(("id" = i64, Path, description = "Resource ID")),
    responses(
        (status = 200, description = "Detail", body = XxxDetailRes),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    tag = "xxx"
)]
pub async fn get_xxx_detail(
    State(st): State<AppState>,
    OptionalAuthUser(auth): OptionalAuthUser,  // 비로그인 허용
    Path(id): Path<i64>,
) -> AppResult<Json<XxxDetailRes>> {
    let res = XxxService::get_detail(&st, id, auth).await?;
    Ok(Json(res))
}

// =====================================================================
// 3. 생성/수정 (인증 필수)
// =====================================================================
#[utoipa::path(
    post,
    path = "/xxx",
    request_body = XxxCreateReq,
    responses(
        (status = 201, description = "Created", body = XxxRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "xxx"
)]
pub async fn create_xxx(
    State(st): State<AppState>,
    AuthUser(auth): AuthUser,  // 구조 분해
    Json(req): Json<XxxCreateReq>,
) -> AppResult<Json<XxxRes>> {
    let res = XxxService::create(&st, auth.sub, req).await?;
    Ok(Json(res))
}

// =====================================================================
// 4. 쿠키 반환 필요 시
// =====================================================================
pub async fn login(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<LoginReq>,
) -> AppResult<(CookieJar, Json<LoginRes>)> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let (res, token, ttl) = AuthService::login(&st, req, ip, ua).await?;
    let cookie = build_refresh_cookie(&st, token, ttl);

    Ok((jar.add(cookie), Json(res)))
}

// 쿠키 빌드 헬퍼 (권장: 공통 모듈)
fn build_refresh_cookie(st: &AppState, token: String, ttl: i64) -> Cookie<'static> {
    let mut c = Cookie::new(st.cfg.refresh_cookie_name.clone(), token);
    c.set_path("/");
    c.set_http_only(true);
    c.set_secure(st.cfg.refresh_cookie_secure);
    c.set_same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    });
    c.set_expires(cookie::time::OffsetDateTime::now_utc() + cookie::time::Duration::seconds(ttl));
    if let Some(domain) = &st.cfg.refresh_cookie_domain {
        c.set_domain(domain.clone());
    }
    c
}
```

##### 🔄 데이터 흐름

```
[HTTP Request]
      ↓
[Extractor] → State, Query, Path, Json, AuthUser 추출
      ↓
[Handler] → 얇은 레이어 (Service 호출만)
      ↓
[Service] → 비즈니스 로직, 검증, TX
      ↓
[Handler] → AppResult<Json<Res>> 또는 (CookieJar, Json<Res>)
      ↓
[HTTP Response] → 200/201/4xx/5xx + JSON Body
```

---

#### 7.7.1-5. router.rs
> **Claude 코드 분석 기반** (2025-01-22)

##### 📁 파일 개요

| 파일 | 라인수 | 함수명 | 조립 방식 | 주요 경로 |
|------|--------|--------|----------|----------|
| [auth/router.rs](src/api/auth/router.rs) | 16 | `auth_router()` | nest | /login, /logout, /refresh 등 |
| [user/router.rs](src/api/user/router.rs) | 15 | `user_router()` | merge ⚠️ | /users, /users/me 등 |
| [video/router.rs](src/api/video/router.rs) | 16 | `router()` | nest | /, /{id}, /{id}/progress |
| [study/router.rs](src/api/study/router.rs) | 17 | `router()` | nest | /, /tasks/{id}/answer 등 |
| [lesson/router.rs](src/api/lesson/router.rs) | 17 | `router()` | nest | /, /{id}, /{id}/progress |

##### 1️⃣ Auth Domain ([auth/router.rs](src/api/auth/router.rs))

**역할**: 인증 관련 라우트 (로그인, 로그아웃, 토큰 갱신, 계정 찾기/복구)

```rust
use axum::{routing::post, Router};
use crate::state::AppState;
use super::handler;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        // 세션/토큰 관련
        .route("/login", post(handler::login))
        .route("/logout", post(handler::logout))
        .route("/logout/all", post(handler::logout_all)) // 모든 기기 로그아웃
        .route("/refresh", post(handler::refresh))

        // 계정 찾기/복구
        .route("/find-id", post(handler::find_id))
        .route("/reset-pw", post(handler::reset_password))
}
```

**특징**:
- 모든 엔드포인트가 **POST** (액션 중심 API)
- 상위에서 `nest("/auth", auth_router())` 방식으로 조립
- 함수명에 도메인 접두사 포함 (`auth_router`)

##### 2️⃣ User Domain ([user/router.rs](src/api/user/router.rs))

**역할**: 사용자 관련 라우트 (회원가입, 프로필, 설정)

```rust
use super::handler::{get_me, get_settings, signup, update_me, update_settings};
use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

/// 서브 라우터는 Router<AppState> 반환(프로젝트 규칙)
pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/users", post(signup))
        .route("/users/me", get(get_me).put(update_me).post(update_me))
        .route("/users/me/settings", get(get_settings).post(update_settings))
}
```

**특징**:
- ⚠️ **절대 경로** 사용 (`/users`, `/users/me`)
- 상위에서 `merge(user_router())` 방식으로 조립 (nest 아님)
- 한 경로에 여러 메서드: `.get(get_me).put(update_me).post(update_me)`
- 함수명에 도메인 접두사 포함 (`user_router`)

##### 3️⃣ Video Domain ([video/router.rs](src/api/video/router.rs))

**역할**: 비디오 관련 라우트 (목록, 상세, 진도)

```rust
use axum::{routing::get, Router};
use crate::state::AppState;
use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_videos))
        .route("/{id}", get(handler::get_video_detail))
        .route(
            "/{id}/progress",
            get(handler::get_video_progress).post(handler::update_video_progress),
        )
}
```

**특징**:
- **상대 경로** 사용 (`/`, `/{id}`)
- 상위에서 `nest("/videos", router())` 방식으로 조립
- 진도: GET (조회) + POST (업데이트) 동일 경로
- 함수명 단순 (`router`)

##### 4️⃣ Study Domain ([study/router.rs](src/api/study/router.rs))

**역할**: 학습 관련 라우트 (목록, 문제, 제출, 상태, 해설)

```rust
use axum::{
    routing::{get, post},
    Router,
};
use crate::state::AppState;
use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_studies))
        .route("/tasks/{id}", get(handler::get_study_task))
        .route("/tasks/{id}/answer", post(handler::submit_answer))
        .route("/tasks/{id}/status", get(handler::get_task_status))
        .route("/tasks/{id}/explain", get(handler::get_task_explain))
}
```

**특징**:
- **중첩 리소스** 패턴: `/studies` → `/tasks/{id}` → `/answer`, `/status`, `/explain`
- 제출만 POST, 나머지는 GET
- 상위에서 `nest("/studies", router())` 방식으로 조립

##### 5️⃣ Lesson Domain ([lesson/router.rs](src/api/lesson/router.rs))

**역할**: 레슨 관련 라우트 (목록, 상세, 아이템, 진도)

```rust
use axum::{routing::get, Router};
use crate::state::AppState;
use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_lessons))
        .route("/{id}", get(handler::get_lesson_detail))
        .route("/{id}/items", get(handler::get_lesson_items))
        .route(
            "/{id}/progress",
            get(handler::get_lesson_progress).post(handler::update_lesson_progress),
        )
}
```

**특징**:
- Video와 동일한 **progress 패턴** (GET + POST)
- 추가 서브 리소스: `/{id}/items`
- 상위에서 `nest("/lessons", router())` 방식으로 조립

##### 📊 공통 패턴 요약

| 패턴 | 설명 | 사용처 |
|------|------|--------|
| **Router<AppState> 반환** | 서브 라우터 표준 시그니처 | 모든 router |
| **상대 경로 + nest** | 프리픽스는 상위에서 관리 | auth, video, study, lesson |
| **절대 경로 + merge** | 라우터가 전체 경로 정의 | user ⚠️ |
| **RESTful 구조** | `/`, `/{id}`, `/{id}/sub` | video, lesson |
| **다중 메서드 체이닝** | `.get(...).post(...)` | progress, me, settings |
| **POST 전용** | 액션 중심 API | auth |
| **중첩 리소스** | `/parent/child/{id}/action` | study |

##### 📋 라우트 전체 매핑

| 도메인 | 최종 경로 | Method | Handler |
|--------|----------|--------|---------|
| **Auth** | `/auth/login` | POST | login |
| | `/auth/logout` | POST | logout |
| | `/auth/logout/all` | POST | logout_all |
| | `/auth/refresh` | POST | refresh |
| | `/auth/find-id` | POST | find_id |
| | `/auth/reset-pw` | POST | reset_password |
| **User** | `/users` | POST | signup |
| | `/users/me` | GET/PUT/POST | get_me, update_me |
| | `/users/me/settings` | GET/POST | get_settings, update_settings |
| **Video** | `/videos` | GET | list_videos |
| | `/videos/{id}` | GET | get_video_detail |
| | `/videos/{id}/progress` | GET/POST | get/update_video_progress |
| **Study** | `/studies` | GET | list_studies |
| | `/studies/tasks/{id}` | GET | get_study_task |
| | `/studies/tasks/{id}/answer` | POST | submit_answer |
| | `/studies/tasks/{id}/status` | GET | get_task_status |
| | `/studies/tasks/{id}/explain` | GET | get_task_explain |
| **Lesson** | `/lessons` | GET | list_lessons |
| | `/lessons/{id}` | GET | get_lesson_detail |
| | `/lessons/{id}/items` | GET | get_lesson_items |
| | `/lessons/{id}/progress` | GET/POST | get/update_lesson_progress |

##### ⚠️ 현재 불일치/개선 필요 사항

| 이슈 | 현재 상태 | 권장 표준 |
|------|----------|----------|
| **함수명 불일치** | `auth_router()`, `user_router()` vs `router()` | 하나로 통일 |
| **조립 방식 불일치** | user만 merge + 절대경로 | nest + 상대경로 통일 |
| **PUT vs POST** | update_me에 PUT과 POST 둘 다 | PUT 또는 PATCH 하나만 |

##### 📋 표준 템플릿

```rust
// router.rs (AMK 표준 골격)
use axum::{
    routing::{get, post},
    Router,
};
use crate::state::AppState;
use super::handler;

/// 서브 라우터는 Router<AppState> 반환
/// 상위에서 nest("/xxx", router())로 조립
pub fn router() -> Router<AppState> {
    Router::new()
        // 목록
        .route("/", get(handler::list))

        // 상세
        .route("/{id}", get(handler::get_detail))

        // 서브 리소스 (조회 + 수정)
        .route(
            "/{id}/progress",
            get(handler::get_progress).post(handler::update_progress),
        )

        // 액션 (POST only)
        .route("/{id}/action", post(handler::do_action))
}
```

##### 🔄 상위 조립 예시

```rust
// api/mod.rs 또는 main.rs
use axum::Router;
use crate::state::AppState;

pub fn api_router(state: AppState) -> Router {
    Router::new()
        // nest 방식 (권장) - 상대 경로 라우터
        .nest("/auth", auth::router::auth_router())
        .nest("/videos", video::router::router())
        .nest("/studies", study::router::router())
        .nest("/lessons", lesson::router::router())

        // merge 방식 - 절대 경로 라우터 (user만 해당)
        .merge(user::router::user_router())

        // 전역 상태 주입 (가장 마지막)
        .with_state(state)
}
```

---

#### 7.7.1-6. 기타 파일들 (Auth 유틸리티)
> **Claude 코드 분석 기반** (2025-01-22)

##### 📁 파일 개요

| 파일 | 라인수 | 역할 | 주요 함수/타입 |
|------|--------|------|---------------|
| [extractor.rs](src/api/auth/extractor.rs) | 85 | 인증 Extractor | `AuthUser`, `OptionalAuthUser` |
| [jwt.rs](src/api/auth/jwt.rs) | 62 | JWT 토큰 관리 | `Claims`, `create_token`, `decode_token` |
| [password.rs](src/api/auth/password.rs) | 37 | 비밀번호 해싱 | `hash_password`, `verify_password` |
| [token_utils.rs](src/api/auth/token_utils.rs) | 44 | Refresh 토큰 유틸 | `parse_refresh_token_bytes`, `generate_refresh_cookie_value` |

##### 1️⃣ extractor.rs - 인증 Extractor

**역할**: Handler에서 인증 로직을 분리하여 `AuthUser`, `OptionalAuthUser` Extractor 제공

```rust
use axum::extract::{FromRef, FromRequestParts};
use axum::http::{header::AUTHORIZATION, request::Parts};
use crate::api::auth::jwt::{self, Claims};
use crate::error::AppError;
use crate::state::AppState;

// 인증 필수 Extractor
pub struct AuthUser(pub Claims);

// 인증 선택 Extractor (비로그인 허용)
pub struct OptionalAuthUser(pub Option<AuthUser>);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,  // State에서 AppState 추출 가능해야 함
{
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl core::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        // AppState에서 JWT secret 추출
        let app_state = AppState::from_ref(state);
        let secret = app_state.cfg.jwt_secret.clone();
        let auth_header = parts.headers.get(AUTHORIZATION).cloned();

        async move {
            // Authorization: Bearer <token> 파싱
            let token = auth_header
                .and_then(|h| h.to_str().ok().map(|s| s.to_string()))
                .and_then(|s| s.strip_prefix("Bearer ").map(|t| t.to_string()))
                .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".into()))?;

            // JWT 검증
            let claims = jwt::decode_token(&token, &secret)
                .map_err(|_| AppError::Unauthorized("Invalid token".into()))?;

            Ok(AuthUser(claims))
        }
    }
}

// OptionalAuthUser: 헤더 없으면 Ok(None), 있으면 검증
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    fn from_request_parts(parts: &mut Parts, state: &S) -> impl core::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let app_state = AppState::from_ref(state);
        let secret = app_state.cfg.jwt_secret.clone();
        let auth_header = parts.headers.get(AUTHORIZATION).cloned();

        async move {
            let Some(header_value) = auth_header else {
                return Ok(OptionalAuthUser(None));  // 헤더 없으면 None
            };
            // 헤더 있으면 검증 진행...
            let claims = jwt::decode_token(token, &secret)?;
            Ok(OptionalAuthUser(Some(AuthUser(claims))))
        }
    }
}
```

**특징**:
- `FromRef<S>` 패턴으로 State에서 AppState 추출
- Handler에서 인증 코드 완전 제거 가능
- `OptionalAuthUser`: 공개 API + 선택적 사용자 컨텍스트에 적합

##### 2️⃣ jwt.rs - JWT 토큰 관리

**역할**: Access Token 생성/검증, Claims 구조체 정의

```rust
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use crate::api::auth::dto::AccessTokenRes;
use crate::error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,           // User ID
    pub session_id: String, // Session ID (로그아웃 시 폐기용)
    pub exp: i64,           // Expiration time (Unix timestamp)
    pub iat: i64,           // Issued at
    pub iss: String,        // Issuer ("amk" 고정)
}

pub fn create_token(
    user_id: i64,
    session_id: &str,
    ttl_minutes: i64,
    secret: &str,
) -> AppResult<AccessTokenRes> {
    let now = OffsetDateTime::now_utc();
    let duration = Duration::minutes(ttl_minutes);
    let expires_in_dt = now + duration;

    let claims = Claims {
        sub: user_id,
        session_id: session_id.to_string(),
        exp: expires_in_dt.unix_timestamp(),
        iat: now.unix_timestamp(),
        iss: "amk".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    // ISO 8601 포맷 (프론트엔드 편의용)
    let expires_at_str = expires_in_dt.format(&Rfc3339)?;

    Ok(AccessTokenRes {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: ttl_minutes * 60,  // 초 단위
        expires_at: expires_at_str,
    })
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
```

**특징**:
- `session_id` 포함 → 세션 기반 폐기(로그아웃) 지원
- `iss: "amk"` 고정 발급자
- `AccessTokenRes`: `token_type`, `expires_in`, `expires_at` 포함 (OAuth 2.0 스타일)

##### 3️⃣ password.rs - 비밀번호 해싱

**역할**: Argon2id 기반 비밀번호 해싱/검증

```rust
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use crate::error::{AppError, AppResult};

/// 비밀번호 해싱
pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2id 설정 (메모리 19MB, 2 iterations, 1 parallelism)
    let params = Params::new(19_456, 2, 1, None)
        .map_err(|e| AppError::Internal(format!("Failed to create Argon2 params: {}", e)))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?
        .to_string();

    Ok(password_hash)
}

/// 비밀번호 검증
#[allow(dead_code)]
pub fn verify_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|_| AppError::Internal("Failed to parse password hash".into()))?;

    let valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(valid)
}
```

**특징**:
- **Argon2id** 알고리즘 사용 (현재 권장 표준)
- 파라미터 중앙 관리 (메모리 19MB, 2 iterations, 1 parallelism)
- `OsRng`로 보안 난수 Salt 생성

##### 4️⃣ token_utils.rs - Refresh 토큰 유틸

**역할**: Refresh 토큰 생성/파싱

```rust
use base64::engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD};
use base64::Engine as _;
use percent_encoding::percent_decode_str;
use rand::RngCore;
use uuid::Uuid;
use crate::error::AppError;

/// Refresh 토큰 파싱 (다양한 포맷 허용)
pub fn parse_refresh_token_bytes(s: &str) -> Result<Vec<u8>, AppError> {
    // 0) URL 디코딩
    let decoded = percent_decode_str(s)
        .decode_utf8()
        .map_err(|_| AppError::Unauthorized("Invalid refresh token format".into()))?;
    let ss = decoded.as_ref();

    // 1) UUID 허용
    if let Ok(u) = Uuid::parse_str(ss) {
        return Ok(u.as_bytes().to_vec());
    }
    // 2) base64url no-pad
    if let Ok(b) = URL_SAFE_NO_PAD.decode(ss) {
        return Ok(b);
    }
    // 3) base64url with pad
    if let Ok(b) = URL_SAFE.decode(ss) {
        return Ok(b);
    }
    // 4) 일반 base64
    if let Ok(b) = STANDARD.decode(ss) {
        return Ok(b);
    }

    Err(AppError::Unauthorized("Invalid refresh token format".into()))
}

/// Refresh 토큰 생성 (랜덤 32바이트)
pub fn generate_refresh_cookie_value() -> (String, [u8; 32]) {
    let mut raw = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut raw);
    let cookie_val = URL_SAFE_NO_PAD.encode(raw);
    (cookie_val, raw)  // (쿠키값, raw 바이트)
}
```

**특징**:
- **Opaque 토큰**: 랜덤 32바이트 → base64url 인코딩
- 다양한 포맷 허용 (UUID, base64url, base64) - 마이그레이션 호환성
- Raw 바이트 반환 → SHA256 해시 후 DB 저장 가능

##### 📊 공통 패턴 요약

| 패턴 | 파일 | 설명 |
|------|------|------|
| **중앙 집중화** | 모두 | 인증/보안 로직을 유틸로 분리, Service에서 직접 구현 금지 |
| **FromRef 패턴** | extractor.rs | State에서 AppState 추출하여 config 접근 |
| **Opaque Token** | token_utils.rs | Refresh는 랜덤 바이트, Access는 JWT |
| **파라미터 고정** | password.rs | Argon2 설정을 한 곳에서 관리 |
| **Claims 확장** | jwt.rs | session_id 포함으로 세션 폐기 지원 |

##### ⚠️ 현재 불일치/개선 필요 사항

| 이슈 | 현재 상태 | 권장 표준 |
|------|----------|----------|
| **JWT Validation** | `Validation::default()` | 알고리즘/issuer 명시적 검증 |
| **verify_password** | `Argon2::default()` | hash_password와 동일 파라미터 사용 |
| **Refresh 토큰 스펙** | 다양한 포맷 허용 | 단일 포맷 (opaque + hash) 통일 |

##### 📋 Auth 유틸리티 사용 흐름

```
[로그인 요청]
      ↓
[password.rs] → hash_password() 비교 또는 verify_password()
      ↓
[jwt.rs] → create_token() → AccessTokenRes
      ↓
[token_utils.rs] → generate_refresh_cookie_value() → (쿠키, raw)
      ↓
[DB/Redis] → SHA256(raw) 저장
      ↓
[HTTP Response] → Access Token (Body) + Refresh Token (Cookie)

---

[API 요청 with Token]
      ↓
[extractor.rs] → AuthUser 또는 OptionalAuthUser
      ↓
[jwt.rs] → decode_token() → Claims { sub, session_id, ... }
      ↓
[Handler] → Claims.sub (user_id) 사용
```

---

## 7.7.2 프론트엔드 패턴 (React/TypeScript)

백엔드 레이어(`dto.rs`, `repo.rs`, `service.rs`, `handler.rs`)와 1:1로 대응되는 **Category-First** 아키텍처를 따른다.

**레이어 대응표:**

| 백엔드 (Rust/Axum) | 프론트엔드 (React/TS) | 역할 |
|-------------------|----------------------|------|
| `dto.rs` | `types.ts` | 요청/응답 타입 정의 |
| `repo.rs` | `*_api.ts` | API 호출 함수 |
| `service.rs` | `hook/*.ts` | 비즈니스 로직 (TanStack Query) |
| `handler.rs` | `page/*.tsx` | UI 조립 및 렌더링 |

**규칙:**
1. **Backend Parity:** DTO는 `snake_case`를 유지하여 변환 비용을 없앤다
2. **Auth Strategy:** Refresh Token은 HttpOnly Cookie, Access Token은 메모리(Zustand persist)에 저장
3. **Tech Stack:** `Axios` + `TanStack Query` + `Zustand` + `react-hook-form` + `Zod` + `shadcn/ui`

**디렉터리 구조:**
```
frontend/src/
├── api/
│   └── client.ts              # Axios 클라이언트 + 401 refresh interceptor
├── app/
│   └── routes.tsx             # React Router 라우팅 정의
├── category/                  # 도메인별 폴더
│   ├── auth/
│   │   ├── page/              # login_page.tsx, signup_page.tsx 등
│   │   ├── hook/              # use_login.ts, use_logout.ts 등
│   │   ├── components/        # logout_button.tsx
│   │   ├── types.ts           # Zod schema + 타입
│   │   └── auth_api.ts        # API 함수들
│   ├── video/
│   │   ├── page/
│   │   ├── hook/
│   │   ├── components/
│   │   ├── types.ts
│   │   └── video_api.ts
│   └── ...
├── components/
│   ├── ui/                    # shadcn/ui 컴포넌트
│   ├── layout/
│   └── shared/
├── hooks/
│   └── use_auth_store.ts      # Zustand + persist (전역 인증 상태)
└── routes/
    └── private_route.tsx      # PrivateRoute 가드
```

---

### 7.7.2-1. types.ts (Zod 스키마 & 타입 정의)

백엔드 `dto.rs`와 1:1 대응. **snake_case 필드명 유지**, Zod로 런타임 검증 + 타입 추론.

#### 파일 개요

| 파일 | 주요 타입 | 비고 |
|------|----------|------|
| `category/auth/types.ts` | `LoginReq`, `LoginRes`, `SignupReq`, `SignupRes` | 공용 Enum 포함 (`UserAuth`, `UserGender`) |
| `category/video/types.ts` | `VideoListReq`, `VideoListRes`, `VideoDetail`, `VideoProgressRes` | List/Detail/Progress 분리 |
| `category/user/types.ts` | `UserDetail`, `UpdateUserReq`, `SettingsRes` | Auth에서 Enum import |

#### 코드 예시: `category/auth/types.ts`

```typescript
import { z } from "zod";

// ==========================================
// 공통 Enum
// ==========================================
export const userAuthSchema = z.enum(["HYMN", "admin", "manager", "learner"]);
export type UserAuth = z.infer<typeof userAuthSchema>;

export const userGenderSchema = z.enum(["none", "male", "female", "other"]);
export type UserGender = z.infer<typeof userGenderSchema>;

// ==========================================
// 액세스 토큰 응답 (공통)
// ==========================================
export const accessTokenResSchema = z.object({
  access_token: z.string(),
  expires_in: z.number().int(),
});
export type AccessTokenRes = z.infer<typeof accessTokenResSchema>;

// ==========================================
// 로그인
// ==========================================
export const loginReqSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6).max(72),
  device: z.string().optional(),
  browser: z.string().optional(),
});
export type LoginReq = z.infer<typeof loginReqSchema>;

export const loginResSchema = z.object({
  user_id: z.number().int(),
  access: accessTokenResSchema,
  session_id: z.string(),
});
export type LoginRes = z.infer<typeof loginResSchema>;

// ==========================================
// 회원가입
// ==========================================
export const signupReqSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8).max(72),
  name: z.string().min(1).max(50),
  nickname: z.string().min(1).max(100),
  terms_service: z.boolean(),
  terms_personal: z.boolean(),
  language: z.string().min(2).max(2),
  country: z.string().min(2).max(50),
  birthday: z.string().date(),  // YYYY-MM-DD
  gender: userGenderSchema,
});
export type SignupReq = z.infer<typeof signupReqSchema>;
```

#### 코드 예시: `category/video/types.ts`

```typescript
import { z } from "zod";

// Request DTO
export const videoListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  per_page: z.number().int().min(1).max(100).optional(),
  q: z.string().optional(),
  tag: z.string().optional(),
});
export type VideoListReq = z.infer<typeof videoListReqSchema>;

// Response DTO - List Meta (페이지네이션)
export const videoListMetaSchema = z.object({
  total_count: z.number().int(),
  total_pages: z.number().int(),
  current_page: z.number().int(),
  per_page: z.number().int(),
});

// Response DTO - List Item
export const videoListItemSchema = z.object({
  video_id: z.number().int(),
  video_idx: z.string(),
  title: z.string().nullable(),        // Option<String> → nullable
  thumbnail_url: z.string().nullable(),
  state: z.string(),
  tags: z.array(z.string()),           // 목록에서는 문자열 배열
  created_at: z.string().datetime(),
});

// Response DTO - List 전체
export const videoListResSchema = z.object({
  meta: videoListMetaSchema,
  data: z.array(videoListItemSchema),
});
export type VideoListRes = z.infer<typeof videoListResSchema>;

// Response DTO - Progress
export const videoProgressResSchema = z.object({
  video_id: z.number().int(),
  progress_rate: z.number().int(),
  is_completed: z.boolean(),
  last_watched_at: z.string().datetime().nullable(),
});
export type VideoProgressRes = z.infer<typeof videoProgressResSchema>;
```

#### 핵심 패턴

| 패턴 | 설명 |
|------|------|
| **snake_case 유지** | 백엔드 DTO와 동일한 필드명 사용 (camelCase 변환 금지) |
| **Zod + infer** | 스키마 정의 → `z.infer<>` 로 타입 추출 |
| **nullable vs optional** | Rust `Option<T>` → `.nullable()`, 선택 필드 → `.optional()` |
| **Enum 중앙화** | 공용 Enum은 `auth/types.ts`에 정의, 다른 도메인에서 import |
| **List 표준 구조** | `{ meta: { total_count, ... }, data: T[] }` |

---

### 7.7.2-2. *_api.ts (API 함수)

백엔드 `repo.rs`와 1:1 대응. `request()` 래퍼를 사용하여 API 엔드포인트 호출.

#### 파일 개요

| 파일 | 주요 함수 | HTTP 메서드 |
|------|----------|-------------|
| `category/auth/auth_api.ts` | `login`, `signup`, `logout`, `findId`, `resetPassword` | POST |
| `category/video/video_api.ts` | `getVideoList`, `getVideoDetail`, `getVideoProgress`, `updateVideoProgress` | GET, POST |
| `category/user/user_api.ts` | `getUserMe`, `updateUserMe`, `getUserSettings`, `updateUserSettings` | GET, POST |

#### 코드 예시: `category/auth/auth_api.ts`

```typescript
import { request } from "@/api/client";
import type {
  FindIdReq,
  LoginReq,
  LoginRes,
  ResetPasswordReq,
  SignupReq,
  SignupRes,
} from "@/category/auth/types";

export const login = (data: LoginReq) => {
  return request<LoginRes>("/auth/login", {
    method: "POST",
    data,
  });
};

export const signup = (data: SignupReq) => {
  // RESTful: 사용자 생성은 /users
  return request<SignupRes>("/users", {
    method: "POST",
    data,
  });
};

export const findId = (data: FindIdReq) => {
  return request<void>("/auth/find-id", {
    method: "POST",
    data,
  });
};

export const logout = () => {
  // 토큰 헤더는 client interceptor가 자동 주입
  return request<void>("/auth/logout", {
    method: "POST",
  });
};
```

#### 코드 예시: `category/video/video_api.ts`

```typescript
import { request } from "@/api/client";
import type {
  VideoDetail,
  VideoListReq,
  VideoListRes,
  VideoProgressUpdateReq,
  VideoProgressRes,
} from "@/category/video/types";

export const getVideoList = (params: VideoListReq = {}) => {
  return request<VideoListRes>("/videos", {
    params,
  });
};

export const getVideoDetail = (id: number) => {
  return request<VideoDetail>(`/videos/${id}`);
};

export const getVideoProgress = (videoId: number) => {
  return request<VideoProgressRes>(`/videos/${videoId}/progress`);
};

export const updateVideoProgress = (videoId: number, data: VideoProgressUpdateReq) => {
  return request<void>(`/videos/${videoId}/progress`, {
    method: "POST",
    data,
  });
};
```

#### 코드 예시: `category/user/user_api.ts`

```typescript
import type {
  SettingsRes,
  SettingsUpdateReq,
  UpdateUserReq,
  UserDetail,
} from "@/category/user/types";
import { request } from "@/api/client";

export const getUserMe = () => {
  return request<UserDetail>("/users/me");
};

export const updateUserMe = (data: UpdateUserReq) => {
  return request<void>("/users/me", {
    method: "POST",
    data,
  });
};

export const getUserSettings = () => {
  return request<SettingsRes>("/users/me/settings");
};

export const updateUserSettings = (data: SettingsUpdateReq) => {
  return request<void>("/users/me/settings", {
    method: "POST",
    data,
  });
};
```

#### 핵심 패턴

| 패턴 | 설명 |
|------|------|
| **request<T> 래퍼** | 제네릭으로 응답 타입 강제 |
| **GET은 params** | `{ params: { page, per_page, ... } }` |
| **POST는 data** | `{ method: "POST", data: { ... } }` |
| **void 반환** | 204 No Content 응답 시 `request<void>` |
| **경로 파라미터** | 템플릿 리터럴 사용 `` `/videos/${id}` `` |

---

### 7.7.2-3. hook/*.ts (TanStack Query 훅)

백엔드 `service.rs`와 1:1 대응. `useMutation` (변경) / `useQuery` (조회) 패턴.

#### 파일 개요

| 파일 | 훅 이름 | Query/Mutation | 특징 |
|------|--------|----------------|------|
| `auth/hook/use_login.ts` | `useLogin` | Mutation | Store 업데이트, 네비게이션 |
| `auth/hook/use_logout.ts` | `useLogout` | Mutation | Store 클리어 |
| `video/hook/use_video_list.ts` | `useVideoList` | Query | staleTime 5분 |
| `video/hook/use_video_detail.ts` | `useVideoDetail` | Query | videoId 기반 |

#### 코드 예시: `category/auth/hook/use_login.ts` (Mutation)

```typescript
import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { LoginReq } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";

import { login } from "../auth_api";

// 에러 코드별 메시지 매핑
const statusMessageMap: Record<number, string> = {
  400: "입력 형식을 확인해주세요.",
  401: "이메일 또는 비밀번호가 일치하지 않습니다.",
  403: "접근이 차단된 계정입니다. 관리자에게 문의하세요.",
  429: "너무 많은 시도가 있었습니다. 잠시 후 다시 시도해주세요.",
  500: "서버 오류가 발생했습니다.",
};

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return statusMessageMap[error.status] ?? error.message;
  }
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useLogin = () => {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: (data: LoginReq) => login(data),
    onSuccess: (data) => {
      useAuthStore.getState().login(data);   // Store 업데이트
      toast.success("로그인 성공!");
      navigate("/");                         // 홈으로 이동
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};
```

#### 코드 예시: `category/video/hook/use_video_list.ts` (Query)

```typescript
import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { VideoListReq } from "@/category/video/types";

import { getVideoList } from "../video_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useVideoList = (params: VideoListReq) => {
  const query = useQuery({
    queryKey: ["videos", params],    // 캐시 키: ["videos", { page, per_page, ... }]
    queryFn: () => getVideoList(params),
    staleTime: 1000 * 60 * 5,        // 5분간 fresh 유지
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
```

#### 핵심 패턴

| 패턴 | 설명 |
|------|------|
| **Query Key 규칙** | `["도메인", params]` 형태 (ex: `["videos", { page: 1 }]`) |
| **staleTime** | 데이터 fresh 유지 시간 (기본 5분) |
| **onSuccess/onError** | Mutation 성공/실패 시 부수 효과 처리 |
| **Store 연동** | `useAuthStore.getState()` 로 Zustand 액션 호출 |
| **에러 메시지 매핑** | HTTP 상태 코드별 사용자 친화적 메시지 |
| **toast 알림** | `sonner` 라이브러리로 토스트 알림 |

---

### 7.7.2-4. page/*.tsx (페이지 컴포넌트)

백엔드 `handler.rs`와 1:1 대응. **조립(Composition)만 담당**, 로직은 훅에 위임.

#### 파일 개요

| 파일 | 컴포넌트 | 사용 훅 | 특징 |
|------|---------|--------|------|
| `auth/page/login_page.tsx` | `LoginPage` | `useLogin`, `useForm` | RHF + Zod + shadcn |
| `video/page/video_list_page.tsx` | `VideoListPage` | `useVideoList` | 목록 렌더링 |
| `user/page/my_page.tsx` | `MyPage` | `useUserMe` | 내 정보 조회 |

#### 코드 예시: `category/auth/page/login_page.tsx`

```tsx
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2 } from "lucide-react";
import { useForm } from "react-hook-form";
import { Link } from "react-router-dom";

import {
  Card, CardContent, CardDescription, CardHeader, CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Form, FormControl, FormField, FormItem, FormLabel, FormMessage,
} from "@/components/ui/form";
import { loginReqSchema, type LoginReq } from "@/category/auth/types";

import { useLogin } from "../hook/use_login";

export function LoginPage() {
  const loginMutation = useLogin();

  const form = useForm<LoginReq>({
    resolver: zodResolver(loginReqSchema),  // Zod 스키마로 검증
    mode: "onChange",
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const onSubmit = (values: LoginReq) => {
    loginMutation.mutate(values);
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2">
          <CardTitle>로그인</CardTitle>
          <CardDescription>
            다시 돌아오신 것을 환영합니다.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              {/* 이메일 입력 */}
              <FormField
                control={form.control}
                name="email"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>이메일</FormLabel>
                    <FormControl>
                      <Input
                        type="email"
                        placeholder="email@example.com"
                        autoComplete="email"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 비밀번호 입력 */}
              <FormField
                control={form.control}
                name="password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>비밀번호</FormLabel>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder="비밀번호를 입력하세요"
                        autoComplete="current-password"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 하단 링크 */}
              <div className="flex items-center justify-between text-sm">
                <Link to="/signup" className="text-primary hover:underline">
                  계정이 없으신가요? 회원가입
                </Link>
                <Link to="/find-id" className="text-muted-foreground hover:underline">
                  아이디/비밀번호 찾기
                </Link>
              </div>

              {/* 제출 버튼 */}
              <Button
                type="submit"
                className="w-full"
                disabled={loginMutation.isPending}
              >
                {loginMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin mr-2" />
                    로그인 중...
                  </>
                ) : (
                  "로그인"
                )}
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
```

#### 핵심 패턴

| 패턴 | 설명 |
|------|------|
| **RHF + Zod** | `zodResolver(schema)` 로 폼 검증 자동화 |
| **shadcn/ui Form** | `Form`, `FormField`, `FormItem` 컴포넌트 조합 |
| **Mutation 상태** | `isPending` 으로 로딩 상태 표시 |
| **페이지는 조립만** | 로직은 훅(`useLogin`)에 위임 |
| **반응형 레이아웃** | `max-w-md`, `min-h-screen` 등 Tailwind 유틸리티 |

---

### 7.7.2-5. 공용 인프라 (Shared Infrastructure)

도메인 간 공유되는 핵심 모듈들.

#### 파일 개요

| 파일 | 역할 | 주요 기능 |
|------|------|----------|
| `api/client.ts` | HTTP 클라이언트 | Axios + 401 refresh interceptor |
| `hooks/use_auth_store.ts` | 전역 인증 상태 | Zustand + persist |
| `app/routes.tsx` | 라우팅 정의 | Public/Private 라우트 분리 |
| `routes/private_route.tsx` | 인증 가드 | 로그인 필수 라우트 보호 |

#### 코드 예시: `api/client.ts` (핵심 부분)

```typescript
import axios, { type AxiosRequestConfig } from "axios";

import type { LoginRes } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? "/api";

export class ApiError extends Error {
  status: number;
  constructor(status: number, message: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
  }
}

export const api = axios.create({
  baseURL: API_BASE_URL,
  withCredentials: true,  // Refresh Cookie 전송 필수
});

// 401 Silent Refresh Interceptor
api.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config;

    if (
      error.response?.status === 401 &&
      originalRequest &&
      !originalRequest._retry
    ) {
      originalRequest._retry = true;
      try {
        // Refresh 요청 (Cookie 기반)
        const refreshResponse = await api.post("/auth/refresh", {});
        const loginData = refreshResponse.data as LoginRes;
        const newToken = `Bearer ${loginData.access.access_token}`;

        // 헤더 갱신
        api.defaults.headers.common["Authorization"] = newToken;
        originalRequest.headers["Authorization"] = newToken;

        // Store 업데이트
        useAuthStore.getState().login(loginData);

        // 원 요청 재시도
        return api(originalRequest);
      } catch (refreshError) {
        // Refresh 실패 시 로그아웃
        useAuthStore.getState().logout();
        window.location.href = "/login";
        return Promise.reject(refreshError);
      }
    }
    return Promise.reject(error);
  }
);

// 제네릭 request 함수
export async function request<T>(
  path: string,
  options: Omit<AxiosRequestConfig, "url"> = {}
): Promise<T> {
  const response = await api.request<T>({ url: path, ...options });
  if (response.status === 204 || response.data === "") {
    return undefined as T;
  }
  return response.data;
}
```

#### 코드 예시: `hooks/use_auth_store.ts`

```typescript
import { create } from "zustand";
import { persist } from "zustand/middleware";

import type { LoginRes } from "@/category/auth/types";
import type { SignupRes } from "@/category/user/types";

type StoredUser = Omit<SignupRes, "access" | "session_id"> | Pick<LoginRes, "user_id">;

type AuthState = {
  user: StoredUser | null;
  accessToken: string | null;
  isLoggedIn: boolean;
  login: (data: LoginRes | SignupRes) => void;
  logout: () => void;
};

const initialState: Pick<AuthState, "user" | "accessToken" | "isLoggedIn"> = {
  user: null,
  accessToken: null,
  isLoggedIn: false,
};

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      ...initialState,
      login: (data) => {
        set({
          user: "email" in data
            ? { user_id: data.user_id, email: data.email, name: data.name, nickname: data.nickname }
            : { user_id: data.user_id },
          accessToken: data.access.access_token,
          isLoggedIn: true,
        });
      },
      logout: () => {
        set({ ...initialState });
        useAuthStore.persist.clearStorage();
      },
    }),
    {
      name: "auth-storage",  // localStorage 키
    }
  )
);
```

#### 코드 예시: `routes/private_route.tsx`

```tsx
import { Navigate, Outlet } from "react-router-dom";
import { useAuthStore } from "@/hooks/use_auth_store";

export default function PrivateRoute() {
  const user = useAuthStore((state) => state.user);

  // 유저 정보가 없으면 로그인 페이지로 리다이렉트
  if (!user) {
    return <Navigate to="/login" replace />;
  }

  // 있으면 자식 컴포넌트 렌더링
  return <Outlet />;
}
```

#### 코드 예시: `app/routes.tsx`

```tsx
import { Route, Routes } from "react-router-dom";

import { LoginPage } from "@/category/auth/page/login_page";
import { SignupPage } from "@/category/auth/page/signup_page";
import { VideoListPage } from "@/category/video/page/video_list_page";
import { MyPage } from "@/category/user/page/my_page";
import PrivateRoute from "@/routes/private_route";

export function AppRoutes() {
  return (
    <Routes>
      {/* 누구나 접근 가능 (Public) */}
      <Route path="/login" element={<LoginPage />} />
      <Route path="/signup" element={<SignupPage />} />
      <Route path="/videos" element={<VideoListPage />} />

      {/* 로그인한 사람만 접근 가능 (Private) */}
      <Route element={<PrivateRoute />}>
        <Route path="/user/me" element={<MyPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Route>
    </Routes>
  );
}
```

#### 핵심 패턴

| 패턴 | 설명 |
|------|------|
| **401 Silent Refresh** | 토큰 만료 시 자동 갱신 후 원 요청 재시도 |
| **Zustand persist** | `localStorage`에 인증 상태 유지 (새로고침 대응) |
| **PrivateRoute 가드** | `<Outlet />` 패턴으로 자식 라우트 보호 |
| **withCredentials** | Refresh Token Cookie 전송을 위해 필수 |
| **ApiError 클래스** | HTTP 상태 코드 기반 에러 핸들링 |

---

### 7.7.2-6. 프론트엔드 데이터 흐름 (Data Flow)

```
[사용자 액션]
      ↓
[Page Component] → useForm() + zodResolver(schema)
      ↓
[Custom Hook] → useMutation() / useQuery()
      ↓
[*_api.ts] → request<T>() 호출
      ↓
[api/client.ts] → axios + interceptor
      ↓
[Backend API]
      ↓
[Response]
      ↓
[Custom Hook] → onSuccess: Store 업데이트, toast 알림
      ↓
[Page Component] → UI 반영 (isPending, data, error)
```

---

## 8. LLM 협업 규칙 (나와 일하는 법)

> 기존 `README_for_assistant.md` + GEMINI 템플릿 관련 내용 정리.

### 8.1 질문/요청 방식

1) 요청 : 기존 작업 진행, 신규 작업 추가 및 진행, 작업 관련 질문
2) 대상 : 작업 환경(aws, docker, linux 등등), 작업 코드(database, backend, frontend, web&app 등등)
3) 방식 : **AMK_API_MASTER.md의 섹션 / 계층 / 파일** 바탕으로 작업 관련 사항 작성 ex) “Phase 3-5 `/videos/{id}/progress`에 대한 Rust 핸들러/서비스/테스트를 구현해줘”
4) 결과 : 해당 작업의 기댓값 작성 ex) “Phase 3-5 `/videos/{id}/progress`에 대한 Rust 핸들러/서비스/테스트를 구현해서 오류 없이 잘 작동 할 수 있도록 만들어줘”

### 8.2 LLM 응답 기대 형식

1) 질문 단계  
   - LLM은 요청 내용을 한 줄로 요약해서 “지금 어떤 작업을 하려는지”를 먼저 정리한다.  
   - 문서(AMK_API_MASTER.md)와 코드/요청이 명백히 충돌하거나, 선택지가 크게 갈리는 지점이 있을 때만 짧게 질의한다.  
   - 그 외에는 필요한 가정을 명시하고 바로 작업을 진행한다.

2) 진행 기준  
   - 요청한 작업에 대해, 내가 제시한 **AMK_API_MASTER.md의 섹션 / 계층 / 파일**을 최우선으로 참조하여 작업을 진행한다.  
   - 문서와 현재 코드가 다를 경우, 문서를 “정답”으로 보고 코드를 문서에 맞춘다(예외가 필요하면 명시).

3) 답변 구조  
   - 3-1) 요약: 이번 답변에서 무엇을 했는지 한 줄 또는 짧은 단락으로 정리  
   - 3-2) 세부내용: 파일/함수/쿼리 단위로 구체적인 변경 내용 제시(필요 시 코드블록)  
   - 3-3) 결과 및 효과: 변경 후 어떤 시나리오가 가능해졌는지, 어떤 문제가 해결되었는지 설명  
   - 3-4) 우려점 및 개선 사항(있다면): 성능/보안/확장성/추가 리팩터링 포인트 등  
   - 3-5) 다음 작업 추천: 자연스럽게 이어질 Phase/태스크 한두 개 제안  
   - *간단한 Q&A/개념 설명만 필요한 경우에는 3-1~3-2 중심으로 답변하고, 나머지는 필요 시에만 포함한다.*

4) 마무리 / 문서 반영  
   - 작업이 완료되면, LLM은 **AMK_API_MASTER.md에서 수정이 필요한 위치와 변경 내용(체크박스, 메모 등)**을 제안한다.  
   - 실제 파일 수정은 사용자가 수행하되, LLM은 복붙 가능한 형태로 변경안을 제공한다.

### 8.3 LLM_PATCH_TEMPLATE 연동

- 실제 코드 패치는 `LLM_PATCHS_TEMPLATE_BACKEND.md`, `LLM_PATCHS_TEMPLATE_FRONTEND.md` 형식을 따른다.
- 기본 구조:
  - ROLE / OBJECTIVE / CONTEXT / CONTRACT / PATCH RULES / ACCEPTANCE / FILE PATCHES / cURL SMOKE
- 요청 시:
  - AMK_API_MASTER.md의 **해당 섹션/Phase/엔드포인트**를 CONTRACT·CONTEXT에 명시한다.
  - 예) Phase 3-5 `/videos/{id}/progress` 스펙을 기준으로 패치 요청.
- 응답/패치 시:
  - FILE PATCHES에 나오는 각 `// FILE: ...` 블록은 **파일 전체 교체본**이다(부분 패치 금지).
  - 네이밍/enum/스키마는 AMK_API_MASTER.md의 3.2(네이밍 규칙), 4.x(데이터 모델)를 우선적으로 따른다.

---

### 8.4 표준 작업 절차 (SOP: Standard Operating Procedure)
우리는 항상 다음 11단계 프로세스를 준수하며 작업을 진행한다.

**[Phase 1: 준비 (Plan)]**
1. **Request**: 사용자가 SSoT(`AMK_API_MASTER.md`)를 기반으로 특정 기능 구현을 위한 **Codex 프롬프트 생성**을 요청.
2. **Draft**: LLM은 구현할 파일/코드 전체가 담긴 프롬프트를 작성하여 제공.
3. **Save**: 사용자는 해당 프롬프트를 프로젝트 `docs/` 내에 저장.

**[Phase 2: 구현 (Execute)]**
4. **Run**: 사용자가 저장된 프롬프트를 Codex(에디터)에서 실행.
5. **Code**: Codex가 코드 작성 완료.

**[Phase 3: 검증 (Verify)]**
6. **Test**: 사용자가 로컬 환경에서 직접 수동 테스팅 및 스모크 테스트 수행.
7. **Fix**: 트러블슈팅 발생 시, 로그를 제공하고 LLM이 해결책 제시 (반복).

**[Phase 4: 문서화 (Document)]**
8. **Log Request**: 테스트 완료 후, 사용자가 "이번 작업의 이슈/교훈 정리해 줘"라고 요청.
9. **Log Update**: LLM이 `AMK_DEV_LOG.md`에 추가할 내용을 정리하여 제공 -> 사용자가 저장.
10. **Status Request**: 사용자가 "작업 현황 업데이트해 줘"라고 요청.
11. **Status Update**: LLM이 `AMK_..._STATUS.md`의 체크리스트 갱신 내용을 제공 -> 사용자가 저장.

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 9. Open Questions & 설계 TODO

> 기존 `AMK_PROJECT_JOURNAL.md`의 Open Questions + Engineering Guide의 “다음 단계 로드맵”에서 정책 수준만 정리.

### 9.1 RBAC / 관리자 권한 ✅ 구현 완료 (2026-02-01)

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

### 9.2 Admin action log actor 연결 ✅ 구현 완료 (2026-02-02)

- ~~`ADMIN_USERS_LOG` 및 비디오/스터디/레슨 admin 로그에 **actor user id** 채우기~~ → **완료**
  - `AuthUser` extractor에서 JWT Claims의 `sub` (user_id) 추출
  - 모든 Admin handler → service → repo까지 `actor_user_id` 전달
  - `create_audit_log()`에서 `admin_id`로 정상 저장
- 향후 검토: 역할별 로그 조회 범위 제한 (manager는 담당 class만 조회 등)

### 9.3 페이징 고도화 (Keyset vs Page)

- 현재 표준은 page/size 기반
- **트리거**: 테이블 데이터 **1만 건 이상** 시 Keyset pagination 검토
- 대상 테이블: `video_log`, `study_task_log`, `login_log`
- 기존 API와 호환성 유지 (page/size 파라미터 병행)

### 9.4 테스트 전략

**목표 성능 (K6 부하 테스트 기준)**:

| 엔드포인트 | 목표 RPS | P95 응답시간 |
|----------|---------|-------------|
| 인증 (login/refresh) | 100 | < 200ms |
| 목록 조회 (videos/studies) | 200 | < 100ms |
| 상세 조회 | 300 | < 50ms |
| 진도 저장 (progress) | 100 | < 150ms |

**대표 시나리오**: 회원가입 → 로그인 → 비디오 조회 → 시청 → 진도 저장 → 학습 문제 풀이

### 9.5 보안/운영 (후순위 계획)

**✅ 완료 항목 (2026-02-01):**
- ~~세션/리프레시 토큰 정책 강화: 역할별 TTL~~ → **완료** (HYMN: 1일, admin/manager: 7일, learner: 30일)
- ~~접근 제어: 관리자 IP allowlist~~ → **완료** (`admin_ip_guard.rs`, CIDR 지원)
- ~~RBAC 미들웨어~~ → **완료** (`role_guard.rs`, HYMN/admin만 admin 접근 허용)

**📋 남은 항목 (외부 API 연결 작업 후 진행):**
- 관리자 MFA 도입 (특히 HYMN/admin 계정) — 소셜 로그인/결제 시스템 후
- 동시 세션 수 제한 — RDS 이전 후
- 토큰 재사용 탐지 (Refresh Token Replay Attack 방지) — RDS 이전 후
- step-up MFA (민감한 작업 시 추가 인증) — MFA 도입 후

### 9.6 코드 일관성 (Technical Debt) ✅

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

### 9.7 작업 로드맵

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
| 2 | 이메일 인증 (AWS SES) | 📋 | 일반 가입 시 이메일 인증 필수화 (Phase 2 예정) |
| 3 | 결제 시스템 | 📋 | Stripe, Polar 연동 (수강권과 연계) |
| 4 | RDS/ElastiCache 이전 | 📋 | EC2 → AWS RDS + ElastiCache (TLS, maxmemory 자동 적용) |

#### 보류/낮음 우선순위

| 항목 | 상태 | 설명 |
|------|:----:|------|
| 학습 문제 동적 생성/전달 | 보류 | 커리큘럼 데이터 완비 후, 사용자 요구 시 구현 |
| Lesson 통계 기능 | 보류 | `/admin/lessons/stats` — 기본 progress 데이터 있음, 추후 구현 예정 |
| Login 정보/로그 추가 | 보류 | `login_country`, `login_asn`, `login_org` — 외부 API 연동 시 IP Geolocation 적용 |
| 통계 비동기/배치 분리 | 보류 | 집계/통계 복잡해지면 검토 |
| URL/함수명 통일 | ✅ | 2026-02-02 완료 — handler/service/repo 네이밍 패턴 통일 |

### 9.8 데이터 모니터링 & 접근

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

### 9.9 디자인 & UI

**현재 상태**: shadcn/ui + Tailwind 사용, 디자인 시스템 미정립

**TODO**: 브랜딩, 타이포그래피, 반응형 점검

### 9.10 마케팅 & 데이터 분석

**현재 상태**: login_log, video_log, study_task_log로 기본 데이터 수집 중

**TODO**: 사용자 세그먼트 정의, 리텐션 분석, 마케팅 자동화 연동

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 10. 변경 이력 (요약)

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
    - Section 9.7 외부 API 연결 로드맵 업데이트

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
  - Section 9.6 "코드 일관성 (Technical Debt)" 추가
  - Section 9.7 "추후 작업 항목 (문서 내 TODO 통합)" 추가
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
    - Section 9.8 "LLM 협업 도구 전환" 추가 (Patch 템플릿 처리 + GitHub Gemini)
    - Section 9.9 "인프라 로드맵 (RDS 이전)" 추가 (이전 순서 및 시점 기준)
    - Section 9.10 "데이터 모니터링 & 접근" 추가 (SSH 터널, Admin 대시보드, 동기화)
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
    - Section 9.2 로그 테이블 역할별 구분 항목 추가
    - Section 9.7 기능 개발에 Admin 폼 검증, 영상 시청 시간, 토픽 정답 검사, 학습 문제 생성 추가
    - Section 9.11.2 에러 페이지 항목 추가
    - Section 9.12 "마케팅 & 데이터 분석" 신규 추가
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
  - **Section 9.7 "보류/낮음 우선순위" 업데이트**
    - URL/함수명 통일 ✅ 완료
    - Login 정보/로그 추가 — 외부 API 연동 시 진행 예정
    - Lesson 통계 기능 — 추후 구현 예정

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

**문서 끝 (End of Document)**
