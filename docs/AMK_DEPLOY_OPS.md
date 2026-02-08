# AMK Deploy & Operations Guide

> 규칙/스펙은 [AMK_API_MASTER.md](./AMK_API_MASTER.md), 코드 예시는 [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md)를 참조하세요.

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
| A | api | 3.39.234.157 | 300 |

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

# Field Encryption (프로덕션 필수)
APP_ENV=production
ENCRYPTION_KEY=<base64-encoded-32-bytes>
HMAC_KEY=<base64-encoded-32-bytes>
# 키 생성: openssl rand -base64 32
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

| 경로 | 관리 방법 |
|------|----------|
| `migrations/*.sql` | `sqlx migrate run` 으로 실행. 오프라인 빌드 시 `.sqlx/` 폴더 필요 |

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)
