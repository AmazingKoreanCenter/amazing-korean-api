# AMK Deploy & Operations Guide

> 규칙/스펙은 [AMK_API_MASTER.md](./AMK_API_MASTER.md), 코드 예시는 [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md), 작업 흐름/역할 분리는 [AMK_PIPELINE.md](./AMK_PIPELINE.md)를 참조하세요.

---

## 📋 목차 (Table of Contents)

- [1. 빌드 & 배포 전략](#1-빌드--배포-전략)
- [2. 도메인 및 DNS 설정 (Route 53)](#2-도메인-및-dns-설정-route-53)
- [3. Cloudflare Pages 배포 (프론트엔드)](#3-cloudflare-pages-배포-프론트엔드)
- [4. AWS EC2 배포 (백엔드)](#4-aws-ec2-배포-백엔드)
- [5. GitHub Actions CI/CD 파이프라인](#5-github-actions-cicd-파이프라인)
- [6. EC2 유지보수 가이드](#6-ec2-유지보수-가이드)
- [7. 품질 보증 & 스모크 체크](#7-품질-보증--스모크-체크)
- [8. 향후 확장 계획](#8-향후-확장-계획)
- [8.5. Paddle Live 전환 가이드](#85-paddle-live-전환-가이드)
- [9. 운영 도구 목록](#9-운영-도구-목록)

---

## 1. 빌드 & 배포 전략

- **빌드 커맨드 (Strict)**
  - `npm run build` 실행 시:
    1.  `tsc -b` (TypeScript 컴파일 검사)가 먼저 실행되어야 한다. **타입 에러 발생 시 빌드는 실패해야 한다.**
    2.  Vite가 프로덕션용 최적화(Minify, Tree Shaking)를 수행하고 `dist/` 폴더를 생성한다.

- **번들 크기 최적화 (TODO)**
  - 현재 메인 번들 크기: **~1,273 KB** (gzip ~350 KB) — Vite 권장 기준 500 KB 초과
  - 개선 방안:
    1. `React.lazy()` + `Suspense`를 활용한 **라우트 기반 코드 스플리팅**
    2. `vite.config.ts`의 `build.rollupOptions.output.manualChunks`로 vendor 청크 분리 (react, react-dom, i18next 등)
    3. 대형 라이브러리의 동적 import (`import()`) 전환
  - 현재 단일 번들이 기능상 문제는 없으나, 페이지/기능이 늘어날수록 초기 로딩 속도에 영향을 줄 수 있다.

- **SPA 서빙 전략 (SPA Fallback)**
  - 프론트엔드는 **Single Page Application**이므로, **모든 404 요청을 `index.html`로 리다이렉트**해야 한다.
  - **Nginx 배포 시**: `try_files $uri $uri/ /index.html;` 설정 필수.
  - **Rust(Axum) 통합 배포 시**: 정적 파일 서빙 핸들러에서 Fallback 경로 설정 필요.

## 2. 도메인 및 DNS 설정 (Route 53)

- **도메인**: `amazingkorean.net`
- **DNS 관리**: AWS Route 53 -> Cloudflare로 수정

##### DNS 레코드 설정

| 레코드 타입 | 이름 | 값 | TTL |
|------------|------|-----|-----|
| CNAME | amazingkorean.net | amazing-korean-api.pages.dev | 300 |
| CNAME | www | amazing-korean-api.pages.dev | 300 |
| A | api | `<EC2_PUBLIC_IP>` | 300 |

##### 서비스 URL

| 서비스 | URL |
|--------|-----|
| 프론트엔드 | https://amazingkorean.net |
| 프론트엔드 (www) | https://www.amazingkorean.net |
| 백엔드 API | https://api.amazingkorean.net |
| Cloudflare Pages | https://amazing-korean-api.pages.dev |

## 3. Cloudflare Pages 배포 (프론트엔드)

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

## 4. AWS EC2 배포 (백엔드)

- **EC2 인스턴스**: Amazon Linux 2023 또는 Ubuntu 22.04 LTS
- **Instance Type**: t2.micro (1 vCPU, 1GB) - 빌드 시 t3.medium 권장
- **Storage**: **최소 20GB gp3** (Rust 빌드에 필요, 8GB는 부족)
- **Public IP**: `<EC2_PUBLIC_IP>` (인스턴스 중지/시작 시 변경됨)
- **도메인**: `api.amazingkorean.net`
- **배포 방식**: Docker Compose
- **Nginx 설정**: 리버스 프록시 (80/443 → API:3000)
- **SSL**: Cloudflare Flexible (프록시 모드)
- **빌드 시간**: t2.micro에서 빌드 불가 (메모리 부족), t3.medium 권장

> **참고**: t2.micro (1GB RAM)는 Rust 빌드에 메모리가 부족합니다. 빌드 시 임시로 t3.medium으로 변경 후, 완료 후 다시 t2.micro로 변경하세요.

##### 환경 변수 (.env.prod)

```env
# ─── Docker ───
DOCKER_IMAGE=amazingkorean/amazing-korean-api

# ─── Application ───
APP_ENV=production                 # production | development (production + EMAIL_PROVIDER=none → 부팅 실패)

# ─── Database ───
POSTGRES_PASSWORD=<secure-password>
# SKIP_DB=0                          # 기본: false. "1"이면 DB 초기화 건너뛰기 (테스트/CI 용)

# ─── Redis ───
REDIS_PASSWORD=<secure-password>

# ─── JWT ───
JWT_SECRET=<min-32-bytes-secret>
# JWT_EXPIRE_HOURS=24               # 기본: 24시간. 액세스 토큰 만료 시간
DOMAIN=api.amazingkorean.net

# ─── Domain & CORS ───
CORS_ORIGINS=https://amazingkorean.net,https://www.amazingkorean.net

# ─── Field Encryption (AES-256-GCM + HMAC Blind Index) ───
# 키 생성: openssl rand -base64 32
# 프로덕션 키 ≠ 로컬 키 (반드시 다른 키 사용)
# 키 분실 시 암호화된 데이터 복구 불가 — 안전한 곳에 별도 백업 필수
ENCRYPTION_KEY_V1=<base64-encoded-32-bytes>
# ENCRYPTION_KEY_V2=<base64-encoded-32-bytes>  # 키 로테이션 시 추가 (V2~V255)
# ENCRYPTION_KEY_V3=<base64-encoded-32-bytes>  # 복호화 시 암호문 버전으로 자동 선택
HMAC_KEY=<base64-encoded-32-bytes>
ENCRYPTION_CURRENT_VERSION=1

# ─── Google OAuth ───
GOOGLE_CLIENT_ID=<google-client-id>
GOOGLE_CLIENT_SECRET=<google-client-secret>
GOOGLE_REDIRECT_URI=https://api.amazingkorean.net/auth/google/callback
FRONTEND_URL=https://amazingkorean.net

# ─── Google OAuth (모바일) ───
# GOOGLE_MOBILE_CLIENT_ID=<mobile-client-id>  # 모바일 전용 Google OAuth Client ID (Android/iOS)

# ─── Apple OAuth (모바일 Sign in with Apple) ───
# APPLE_CLIENT_ID=<bundle-id>      # Apple Bundle ID (e.g., net.amazingkorean.app)
# APPLE_TEAM_ID=<team-id>          # Apple Team ID

# ─── RevenueCat (모바일 IAP) ───
# REVENUECAT_API_KEY=<api-key>     # RevenueCat 서버 API 키
# REVENUECAT_WEBHOOK_AUTH_TOKEN=<token>  # RevenueCat 웹훅 Bearer 토큰

# ─── 동시 세션 수 제한 (역할별) ───
# MAX_SESSIONS_LEARNER=5           # 기본: 5 (초과 시 가장 오래된 세션 자동 퇴장)
# MAX_SESSIONS_MANAGER=3           # 기본: 3 (초과 시 로그인 거부)
# MAX_SESSIONS_ADMIN=2             # 기본: 2 (초과 시 로그인 거부)
# MAX_SESSIONS_HYMN=2              # 기본: 2 (초과 시 로그인 거부)

# ─── Email (Resend) ───
EMAIL_PROVIDER=resend              # resend | none (프로덕션에서 none 사용 시 서버 부팅 실패)
RESEND_API_KEY=re_xxx              # 필수 (Resend 대시보드에서 발급)

# ─── Rate Limiting (이메일 발송) ───
# RATE_LIMIT_EMAIL_WINDOW_SEC=18000  # 기본: 18000초 (5시간)
# RATE_LIMIT_EMAIL_MAX=5             # 기본: 5회/윈도우

# ─── Rate Limiting (스터디 답안 제출) ───
# RATE_LIMIT_STUDY_WINDOW_SEC=60     # 기본: 60초
# RATE_LIMIT_STUDY_MAX=30            # 기본: 30회/윈도우

# ─── Rate Limiting (교재 주문) ───
# RATE_LIMIT_TEXTBOOK_WINDOW_SEC=3600  # 기본: 3600초 (1시간)
# RATE_LIMIT_TEXTBOOK_MAX=5            # 기본: 5회/윈도우

# ─── Swagger UI ───
# ENABLE_DOCS=0                       # 기본: 0 (비활성화). 1로 설정 시 /docs Swagger UI 노출

# ─── Admin ───
# ADMIN_IP_ALLOWLIST=1.2.3.4,10.0.0.0/8

# ─── E-book ───
# EBOOK_PAGE_IMAGES_DIR=docs/textbook/page-images  # 기본: "docs/textbook/page-images". 페이지 이미지 경로
# EBOOK_IMAGES_ENCRYPTED=0           # 기본: false. "1"이면 이미지 암호화 모드

# ─── Paddle Billing (결제) ───
PADDLE_API_KEY=apikey_xxx             # Paddle API Key (Sandbox/Production)
PADDLE_CLIENT_TOKEN=test_xxx          # 프론트엔드 Paddle.js 초기화용
PADDLE_SANDBOX=true                   # true(Sandbox) / false(Production)
PADDLE_WEBHOOK_SECRET=pdl_xxx         # Webhook 서명 검증용 Secret Key
PADDLE_PRICE_MONTH_1=pri_xxx          # 1개월 Price ID ($10)
PADDLE_PRICE_MONTH_3=pri_xxx          # 3개월 Price ID ($30, 정가)
PADDLE_PRICE_MONTH_6=pri_xxx          # 6개월 Price ID ($60, 정가)
PADDLE_PRICE_MONTH_12=pri_xxx         # 12개월 Price ID ($120, 정가)
PADDLE_PRICE_EBOOK=pri_xxx            # E-book 일회성 Price ID ($10 USD)
```

> **Google OAuth 설정 시 주의**: Google Cloud Console → 사용자 인증 정보 → 승인된 리디렉션 URI에 `https://api.amazingkorean.net/auth/google/callback`을 반드시 추가해야 합니다.

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

# 2. 환경 변수 설정 (위의 ".env.prod" 전체 변수 목록 참조)
nano .env.prod
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

##### 4. 데이터베이스 마이그레이션 (자동)

앱 부팅 시 `sqlx::migrate!()` 가 `migrations/` 폴더의 SQL 파일을 자동 실행합니다.
수동 SSH 접속 불필요 — 코드 배포 = 마이그레이션 자동 적용.

**동작 원리:**

- `_sqlx_migrations` 테이블에 적용 이력 자동 기록 (version + SHA-384 checksum)
- 이미 적용된 마이그레이션은 건너뜀
- 각 마이그레이션은 개별 트랜잭션 내 실행 (실패 시 롤백)
- `pg_advisory_lock()` 으로 동시 실행 방지
- `main.rs` 에서 DB pool 생성 직후, 서버 시작 전에 실행

**파일 네이밍 규칙:**

```
migrations/YYYYMMDD_description.sql
```

- 첫 `_` 앞 숫자가 version (BIGINT PK) — **반드시 유니크**
- description은 자유 (밑줄 → 공백으로 변환되어 `_sqlx_migrations.description`에 저장)
- 한 번 적용된 파일의 내용을 수정하면 checksum 불일치로 서버 부팅 실패
- **같은 날짜 충돌 시**: 다음 날짜를 사용 (예: `20260310` 충돌 → `20260311`)

> **⚠️ HHMMSS(000001 등) 접미사 사용 금지**: `20260310000001`은 정수 `20,260,310,000,001`이 되어 `20260312`(= `20,260,312`)보다 훨씬 큰 값. sqlx는 정수 기준 오름차순으로 실행하므로 의존성 순서가 뒤집혀 서버 크래시 발생 (2026-03-23 사고).

**신규 마이그레이션 추가 방법:**

```bash
# 1. 마이그레이션 파일 작성
touch migrations/20260325_add_new_feature.sql
# SQL 작성...

# 2. 로컬 DB에서 테스트
cargo run  # → "Database migrations applied" 로그 확인

# 3. SQLx 오프라인 캐시 갱신 (Docker 빌드용)
cargo sqlx prepare
git add .sqlx

# 4. 커밋 & 푸시 → 배포 시 자동 적용
```

##### 4-1. 기존 프로덕션 전환 (1회성) — 완료 (2026-03-23)

> **이 섹션은 기록용입니다. 전환은 이미 완료되었으며, 다시 실행할 필요 없습니다.**

기존에 수동으로 적용된 마이그레이션을 `_sqlx_migrations` 테이블에 등록하는 1회성 작업.

```bash
# EC2 SSH 접속 후
cd amazing-korean-api
git pull origin KKRYOUN

# 부트스트랩 스크립트 실행
docker exec -i amk-pg psql -U postgres -d amazing_korean_db < scripts/bootstrap_sqlx_migrations.sql

# 확인
docker exec -i amk-pg psql -U postgres -d amazing_korean_db \
  -c "SELECT version, description, success FROM _sqlx_migrations ORDER BY version;"

# 앱 재빌드 & 시작
docker compose -f docker-compose.prod.yml --env-file .env.prod up -d --build

# 로그 확인: "Database migrations applied" 메시지 확인
docker logs amk-api --tail 30
```

> **⚠️ 부트스트랩 스크립트 작성 시 주의사항 (2026-03-23 사고 교훈):**
> - 부트스트랩에는 **프로덕션 DB에 실제 적용된 마이그레이션만** 등록해야 함
> - 로컬에만 적용된 마이그레이션을 포함하면 sqlx가 "이미 적용됨"으로 건너뛰어 **테이블이 생성되지 않은 채 후속 마이그레이션 실행 → 크래시**
> - 작성 전 프로덕션 DB에서 `\dt` / `\dT`로 실제 테이블/ENUM 존재 여부를 반드시 확인

##### 4-2. 클린 배포 절차 (DB 초기화)

```bash
# 1) 전체 중지 + DB 볼륨 삭제 (모든 데이터 초기화됨!)
docker compose -f docker-compose.prod.yml down
docker volume rm amazing-korean-api_postgres_data

# 2) 최신 코드 가져오기
git pull origin KKRYOUN

# 3) 전체 시작 (DB 생성 → 앱 부팅 시 마이그레이션 자동 실행)
docker compose -f docker-compose.prod.yml --env-file .env.prod up -d --build

# 4) 시드 데이터 투입 (콘텐츠 테이블 — 수동)
docker exec -i amk-pg psql -U postgres -d amazing_korean_db < seeds/20260208_AMK_V1_SEED.sql
```

> **주의**: `docker volume rm`은 PostgreSQL 전체 데이터를 삭제합니다 (users, video, study, lesson 등 모든 테이블).
> 시드 파일은 `seeds/` 폴더에 있으며, 자동 실행되지 않습니다 (수동 투입 필요).

##### 5. 배포 후 확인

```bash
# API 헬스체크 (nginx 프록시 경유)
curl http://localhost:80/healthz -H "Host: api.amazingkorean.net"
# 또는 외부에서: curl https://api.amazingkorean.net/healthz

# 컨테이너 상태 확인 (5개: api, pg, redis, nginx, certbot)
docker ps

# API 로그 확인
docker logs amk-api --tail 30
# → "Database migrations applied" + "Server listening on http://0.0.0.0:3000" 확인

# 마이그레이션 적용 상태 확인
docker exec -i amk-pg psql -U postgres -d amazing_korean_db \
  -c "SELECT version, description, success FROM _sqlx_migrations ORDER BY version;"

# DB 시드 데이터 확인
docker exec -i amk-pg psql -U postgres -d amazing_korean_db -c "SELECT count(*) FROM video;"
# → 16

# 암호화 동작 확인 (회원가입 후)
docker exec -i amk-pg psql -U postgres -d amazing_korean_db -c "SELECT user_email FROM users LIMIT 3;"
# → enc:v1:... 형태
```

##### 6. 관련 파일

| 파일 | 설명 |
|------|------|
| `Dockerfile` | Rust 백엔드 멀티스테이지 빌드 (rust:1.88, 멀티바이너리: api + rekey_encryption, Cargo 워크스페이스: `crates/crypto`) |
| `docker-compose.prod.yml` | 프로덕션 구성 (API + DB + Redis + Nginx + Certbot) |
| `nginx/nginx.conf` | 리버스 프록시 (`api.amazingkorean.net` → api:3000), SSL은 Cloudflare Flexible |
| `.sqlx/` | SQLx 오프라인 캐시 (Docker 빌드 시 필수) |
| `.env.prod` | 프로덕션 환경 변수 (Git에 포함하지 않음) — 전체 변수 목록은 위 "환경 변수" 섹션 참조 |
| `migrations/` | SQLx 자동 마이그레이션 폴더 — 앱 부팅 시 자동 실행 |
| `seeds/20260208_AMK_V1_SEED.sql` | 시드 데이터 (콘텐츠 10개 테이블, ~200행) — 클린 배포 시 수동 투입 |
| `scripts/bootstrap_sqlx_migrations.sql` | 1회성 전환 스크립트 (기존 프로덕션 → sqlx 자동 마이그레이션) |

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
| `pull access denied for amazing-korean-api` | `.env.prod`의 `DOCKER_IMAGE` 값 누락/오타 | `DOCKER_IMAGE=amazingkorean/amazing-korean-api` (org/repo 형식) |
| `400: redirect_uri_mismatch` (Google OAuth) | redirect URI 불일치 | `.env.prod` GOOGLE_REDIRECT_URI + Google Cloud Console 승인 URI 모두 `https://api.amazingkorean.net/auth/google/callback`으로 설정 |
| INSERT 시 컬럼 순서 에러 | 통합 마이그레이션과 pg_dump 컬럼 순서 불일치 | INSERT문에 명시적 컬럼명 추가 (`INSERT INTO table (col1, col2, ...) VALUES (...)`) |
| `migration X was previously applied but has been modified` | 적용 후 마이그레이션 파일 내용 변경 (checksum 불일치) | 파일을 원래 내용으로 복원. 스키마 변경이 필요하면 새 마이그레이션 파일 추가 |
| `migration X was previously applied but is missing` | 적용 후 마이그레이션 파일 삭제 | 삭제된 파일 복원, 또는 `_sqlx_migrations` 테이블에서 해당 행 삭제 |
| 앱 시작 시 `relation already exists` | `_sqlx_migrations` 테이블 없이 앱 시작 (기존 프로덕션) | `scripts/bootstrap_sqlx_migrations.sql` 먼저 실행 |
| 앱 시작 시 `relation "X" does not exist` (마이그레이션 중) | 마이그레이션 실행 순서 오류. 원인 2가지: **(1)** 부트스트랩에 프로덕션 미적용 마이그레이션을 등록하여 sqlx가 건너뜀 **(2)** 파일명의 정수 버전이 의존성 순서와 불일치 (예: `20260310000001` > `20260312`) | `_sqlx_migrations`에서 잘못 등록된 행 DELETE → 파일명 버전 순서 확인 → 앱 재시작 |

##### 8-1. 환경변수 변경 시 docker-compose.prod.yml 동시 수정 필수

> **3회 실수 발생** — `.env.prod`에 변수 추가 시 `docker-compose.prod.yml`의 `environment:` 섹션에도 반드시 추가할 것. Docker Compose는 `.env.prod` 파일을 직접 읽지 않고, `environment:` 섹션에 명시된 변수만 컨테이너에 전달한다.

**확인 절차:**
1. `config.rs`에서 해당 기능이 사용하는 **모든** `env::var()` 호출을 검색
2. 변수 접두사만 보지 말고, 관련된 모든 변수를 목록으로 정리 (예: `PAYMENT_PROVIDER` + `PADDLE_*`)
3. 목록의 각 변수가 `docker-compose.prod.yml`의 `environment:` 섹션에 있는지 하나씩 대조
4. 누락된 변수가 있으면 추가

**실수 이력:**
- 1차: 최초 환경변수 추가 시 docker-compose.prod.yml 누락
- 2차: Paddle 변수 추가 시 `PADDLE_*` 9개 전부 누락
- 3차: Paddle 변수 추가 수정 시 `PAYMENT_PROVIDER` 누락 (`PADDLE_*` 접두사에만 집중해서 놓침)

##### 8-2. Cargo 워크스페이스 멤버 추가 시 Dockerfile 동시 수정 필수

> **2회 실수 발생** — `crates/crypto` 워크스페이스 멤버 추가 후 Dockerfile 미수정 → Docker 빌드 2회 연속 실패.

**필수 수정 사항** (새 워크스페이스 멤버 추가 시):
1. 매니페스트 복사: `COPY crates/{name}/Cargo.toml ./crates/{name}/Cargo.toml`
2. 더미 소스 생성: `mkdir -p crates/{name}/src && echo "" > crates/{name}/src/lib.rs`
3. 캐시 레이어 정리: `rm -rf` 대상에 `crates/{name}/src` 추가
4. 실제 소스 복사: `COPY crates/{name} ./crates/{name}`
5. **2차 빌드 touch**: `touch` 대상에 `crates/{name}/src/lib.rs` 추가 (누락 시 Cargo가 더미 캐시 사용 → 빌드 실패)

**실수 이력:**
- 1차: `crates/crypto/` 디렉토리 자체를 COPY하지 않아 빌드 실패
- 2차: 2차 빌드 `touch`에 `crates/crypto/src/lib.rs` 누락 → Cargo가 더미 빈 lib.rs 캐시 사용 → 빌드 실패

##### 9. Cloudflare SSL & 보안 설정

Cloudflare 프록시 사용 시 Let's Encrypt 없이 SSL 적용 가능:

1. Cloudflare 대시보드 → `amazingkorean.net` → **DNS**
2. `api` A 레코드의 프록시 상태를 **주황색 구름** (Proxied)으로 설정
3. **SSL/TLS** → **Overview** → 모드를 **Flexible**로 설정

> **참고**: Flexible 모드는 Cloudflare ↔ 사용자 간 HTTPS, Cloudflare ↔ EC2 간 HTTP를 사용합니다.

**HTTPS 강제 & HSTS (2026-02-10 적용)**

| 설정 | 위치 | 값 |
|------|------|-----|
| Always Use HTTPS | SSL/TLS → Edge Certificates | **ON** — `http://` 요청을 301 → `https://`로 리다이렉트 |
| HSTS | SSL/TLS → Edge Certificates → HSTS | **ON** |
| HSTS Max Age | 〃 | `6 months` (15552000초) |
| Include subdomains | 〃 | **ON** (`api.amazingkorean.net` 포함) |
| No-Sniff | 〃 | **ON** (`X-Content-Type-Options: nosniff` 추가) |
| Preload | 〃 | OFF (향후 검토) |

> **주의**: HSTS를 활성화하면 브라우저가 HTTPS를 강제합니다. SSL 인증서가 만료되면 사이트 접근이 불가능해지므로 인증서 갱신에 주의하세요.

**백엔드 보안 강화 (2026-02-10 적용)**

코드 레벨에서 적용된 프로덕션 보안 설정:

| ID | 변경 사항 | 파일 | 설명 |
|---|---|---|---|
| PROD-4 | 보안 헤더 미들웨어 | `src/main.rs` | 모든 응답에 `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `X-XSS-Protection: 0`, `Permissions-Policy` 헤더 추가 |
| PROD-5 | 버전 정보 숨김 | `src/api/health/handler.rs` | `APP_ENV=production`일 때 `/healthz` 응답에서 `version` 필드 제거 |
| PROD-6 | Swagger UI 조건부 노출 | `src/api/mod.rs` | `ENABLE_DOCS=1`일 때만 `/docs` 경로 활성화 (기본: 비활성화) |
| PROD-7 | Guard 401/403 JSON 통일 | `src/api/admin/ip_guard.rs`, `src/api/admin/role_guard.rs` | IP/역할 Guard 거부 시 text/plain 대신 `AppError` JSON 응답 반환 |
| PROD-8 | 404 JSON 응답 | `src/api/mod.rs` | 존재하지 않는 라우트 요청 시 text/plain 대신 JSON `{"error":{"code":"NOT_FOUND",...}}` 반환 |

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

## 5. GitHub Actions CI/CD 파이프라인

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
| `EC2_HOST` | EC2 Public IP | 예: `<EC2_PUBLIC_IP>` |
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

## 6. EC2 유지보수 가이드

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

## 7. 품질 보증 (QA) & 스모크 체크

- **정적 분석 (CI Gate)**
  - `npm run lint`: ESLint (코드 스타일 및 잠재적 버그 검사)
  - `npm run typecheck`: TypeScript 타입 정합성 검사 (필수)

- **수동 스모크 테스트 (Release Checklist)**
  - 배포 전 아래 시나리오를 **반드시 1회 수동 확인**한다.
    1.  **진입**: 랜딩 페이지 로딩 및 폰트/이미지 깨짐 확인.
    2.  **인증**: 로그인(토큰 발급) → 새로고침 시 로그인 유지 확인.
    3.  **영상**: 비디오 목록 로딩 → 상세 페이지 진입 → 플레이어 재생 확인.
    4.  **라우팅**: 잘못된 URL 입력 시 404 페이지(또는 리다이렉트) 동작 확인.
    5.  **보안 헤더**: `curl -I https://api.amazingkorean.net/healthz`로 `X-Content-Type-Options`, `X-Frame-Options`, `Strict-Transport-Security` 확인.
    6.  **404 JSON**: `curl https://api.amazingkorean.net/nonexistent`로 JSON `{"error":{"code":"NOT_FOUND",...}}` 응답 확인.
    7.  **Swagger 비활성화**: `curl https://api.amazingkorean.net/docs`가 404 JSON 반환 확인 (`ENABLE_DOCS=0` 시).

## 8. 향후 확장 계획 (Roadmap)

- **자동화 테스트 도입 (Phase 3 이후)**
  - **Unit Test**: `Vitest` 도입. (유틸 함수 및 복잡한 Hook 로직 검증)
  - **E2E Test**: `Playwright` 도입. (핵심 비즈니스 플로우 자동화)

- **CI/CD 파이프라인**
  - GitHub Actions 연동:
    - Push 시: `Lint` + `Typecheck` 자동 실행.
    - Tag/Merge 시: `Build` 수행 후 Docker Image 생성 또는 S3 업로드.

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 8.5. Paddle Live 전환 가이드

> Sandbox → Production 전환 체크리스트. KYB/Onfido 승인 완료 후 실행.
> Sandbox ↔ Live는 완전 별도 환경 — 상품, 가격, API 키, 웹훅 모두 재생성 필요.
> 공식 문서: [Go-live checklist](https://developer.paddle.com/build/onboarding/go-live-checklist) · [Setup checklist](https://developer.paddle.com/build/onboarding/set-up-checklist)

### Phase 1: Dashboard 필수 설정

#### 1-1. 계정 설정

| 항목 | 경로 | 설명 |
|------|------|------|
| Domain Verification | Checkout Settings → Approved Domains | `amazingkorean.net` 승인 (checkout overlay 작동 조건) |
| Balance Currency | Business Account → Currencies | USD 권장 (은행 통화와 일치 → SWIFT 수수료 회피). 변경 어려움 |
| Payout Settings | Business Account → Payouts | 은행 계좌 등록 (Wire Transfer/Payoneer). 최소 $100 |
| Sales Tax Settings | Checkout → Sales Tax Settings | inclusive/exclusive/location 중 선택 (Tax Category와 별개) |
| Default Payment Link | Checkout → Checkout Settings | `https://amazingkorean.net/pricing` — **미설정 시 checkout 에러** |
| Payment Methods | Checkout → Checkout Settings | Card(기본), PayPal, Apple Pay, Google Pay 등 활성화 |

#### 1-2. Product & Price 생성

| Product | Price | Amount | Type | 환경변수 |
|---------|-------|--------|------|---------|
| Subscription (SaaS) | 1개월 | $10 USD | Recurring (1m, trial 1d) | `PADDLE_PRICE_MONTH_1` |
| Subscription (SaaS) | 3개월 | $30 USD (정가) | Recurring (3m, trial 1d) | `PADDLE_PRICE_MONTH_3` |
| Subscription (SaaS) | 6개월 | $60 USD (정가) | Recurring (6m, trial 1d) | `PADDLE_PRICE_MONTH_6` |
| Subscription (SaaS) | 12개월 | $120 USD (정가) | Recurring (12m, trial 1d) | `PADDLE_PRICE_MONTH_12` |
| E-book (Standard Digital Goods) | 단일가 | $10 USD | One-time | `PADDLE_PRICE_EBOOK` |

#### 1-3. API Key & Client Token

Dashboard → **Developer Tools** → **Authentication**:
- API Key 생성 (`pdl_apikey_live_...`) → `PADDLE_API_KEY`
- Client Token 생성 (`live_...`) → `PADDLE_CLIENT_TOKEN`

#### 1-4. Webhook Notification Destination

Dashboard → **Developer Tools** → **Notifications** → Create:
- URL: `https://api.amazingkorean.net/payment/webhook`
- 구독 이벤트 (11개): `subscription.created/activated/resumed/updated/canceled/paused/past_due/trialing` + `transaction.completed` + `adjustment.created/updated`
- Secret Key 즉시 복사 (재확인 불가) → `PADDLE_WEBHOOK_SECRET`

#### 1-5. 강력 권장

| 항목 | 경로 | 설명 |
|------|------|------|
| Retain | Retain → Settings | Payment Recovery + Tactical Retries 활성화 (이탈 10-15% 개선) |

#### 1-6. Discount (장기 구독 할인)

3/6/12개월 구독은 정가 대비 Paddle Discount로 자동 할인 적용:

| 구간 | 정가 | 할인 | 최종가 | Discount |
|------|------|------|--------|----------|
| 1개월 | $10 | — | $10 | 없음 |
| 3개월 | $30 | $5 off | $25 | `discountId` 자동 적용 |
| 6개월 | $60 | $10 off | $50 | `discountId` 자동 적용 |
| 12개월 | $120 | $20 off | $100 | `discountId` 자동 적용 |

- Discount Type: Flat (고정 금액), Currency: USD
- Recur: Yes (매 결제마다), restrictTo: 해당 Price ID만
- Code 없이 생성 → `discountId`로 checkout 시 자동 적용

#### 1-7. 선택

| 항목 | 설명 |
|------|------|
| Checkout Branding | Checkout Settings → Overlay 탭 → 브랜드 컬러 |

### Phase 2: 코드 변경 — pwCustomer + Retain (Go-Live 필수)

`Paddle.Initialize()`에 `pwCustomer` 전달 — Retain 동작 조건.
`use_paddle.ts`에서 `email` prop 수신 + `Paddle.Update()`로 후속 업데이트.
3개 호출 사이트(`pricing_page`, `ebook_catalog_page`, `ebook_my_purchases_page`)에서 `useUserMe()` 훅으로 이메일 전달.
`home_page.tsx`에서 Retain용 Paddle.js 초기화 (결제 실패 시 인앱 알림 표시용).

#### 이메일 인프라
- Cloudflare Email Routing: `support@amazingkorean.net` → Gmail 포워딩 (Retain 이메일 인증용)
- SPF 레코드 병합: `v=spf1 include:send.resend.com include:_spf.mx.cloudflare.net ~all`

### Phase 3: 환경변수 교체 + 배포

```env
PADDLE_SANDBOX=false
PADDLE_API_KEY=pdl_apikey_live_...
PADDLE_CLIENT_TOKEN=live_...
PADDLE_WEBHOOK_SECRET=pdl_ntfset_...
PADDLE_PRICE_MONTH_1=pri_...
PADDLE_PRICE_MONTH_3=pri_...
PADDLE_PRICE_MONTH_6=pri_...
PADDLE_PRICE_MONTH_12=pri_...
PADDLE_PRICE_EBOOK=pri_...
```

부팅 로그 확인: `💳 Payment provider enabled: Paddle Billing (Production)`

### Phase 4: E2E 검증

1. API 검증: `/health`, `/payment/plans` (sandbox=false), `/ebook/catalog` (Live ID)
2. Webhook Simulator: Dashboard에서 테스트 이벤트 전송 → 200 OK 확인
3. 실결제: 구독 $10 + e-book $10 테스트 후 Dashboard에서 환불
4. 프론트 UX: checkout overlay, 재결제, 주문 취소, 자동 refetch 검증

### 참고

| 항목 | 설명 |
|------|------|
| Webhook IP Allowlist | 권장사항이나 현재 HMAC 서명 검증으로 보안 확보 |
| Customer Portal | Paddle 자동 활성화, 별도 설정 불필요 |
| Customer Emails | Paddle 자동 발송 (영수증, 결제 실패 등) |
| Fraud Protection | Paddle 내부 자동 처리 |

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 9. 운영 도구 목록

프로젝트에서 사용하는 운영/인프라 도구 및 스크립트 목록.

### 컨테이너 & 인프라

| 파일 | 용도 |
|------|------|
| `docker-compose.yml` | 로컬 개발 환경 (PostgreSQL 16 + Redis 7) |
| `docker-compose.prod.yml` | 프로덕션 환경 (pre-built image 사용) |
| `.github/workflows/deploy.yml` | GitHub Actions CI/CD 파이프라인 |

### 스크립트

| 파일 | 용도 |
|------|------|
| `scripts/dev_preflight.sh` | 개발 환경 사전 점검 |
| `scripts/mk-support-bundle.sh` | 지원 번들 생성 (로그/설정 수집) |
| `src/api/scripts/db_fastcheck.sh` | DB 빠른 상태 확인 |
| `verify_refresh.sh` | Refresh Token 흐름 검증 |

### 바이너리 도구

| 파일 | 용도 |
|------|------|
| `src/bin/rekey_encryption.rs` | 암호화 키 로테이션 (`--check`, `--verify`, `--batch-size` 옵션) |

### DB 마이그레이션

앱 부팅 시 `sqlx::migrate!()` 가 `migrations/` 폴더의 SQL 파일을 자동 실행. 수동 실행 불필요.

| 경로 | 설명 |
|------|------|
| `migrations/*.sql` | sqlx 자동 마이그레이션 — 앱 부팅 시 버전 순서대로 자동 적용 |
| `seeds/20260208_AMK_V1_SEED.sql` | 시드 데이터 — 클린 배포 시 수동 투입 (자동 실행 안 됨) |
| `scripts/bootstrap_sqlx_migrations.sql` | 1회성 전환 스크립트 (완료됨, 재실행 불필요) |

> **네이밍 주의**: 파일명의 `_` 앞 숫자가 정수 버전. 같은 날짜 충돌 시 다음 날짜 사용. `000001` 접미사 절대 금지 (정수 비교 시 순서 뒤집힘).

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)
