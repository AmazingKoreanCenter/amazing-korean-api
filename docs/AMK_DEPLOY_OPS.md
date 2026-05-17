# AMK Deploy & Operations Guide

> 규칙/스펙은 [AMK_API_MASTER.md](./AMK_API_MASTER.md), 코드 예시는 [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md), 작업 흐름/역할 분리는 `amazing-korean-ai/docs/AMK_AI_PIPELINE.md` (이관됨)를 참조하세요.

---

## 📋 목차 (Table of Contents)

- [1. 빌드 & 배포 전략](#1-빌드--배포-전략)
- [2. 도메인 및 DNS 설정 (Route 53)](#2-도메인-및-dns-설정-route-53)
- [3. Cloudflare Pages 배포 (프론트엔드)](#3-cloudflare-pages-배포-프론트엔드)
- [7.6. Cloudflare 운영 정책 (통합 SSoT)](#76-cloudflare-운영-정책-dns--pages--ssl--email-routing)
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
REVENUECAT_API_KEY=<api-key>              # RevenueCat 서버 API 키
REVENUECAT_WEBHOOK_AUTH_TOKEN=<token>     # RevenueCat 웹훅 Bearer 토큰

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
PADDLE_DISCOUNT_MONTH_3=dsc_xxx       # 3개월 할인 ID
PADDLE_DISCOUNT_MONTH_6=dsc_xxx       # 6개월 할인 ID
PADDLE_DISCOUNT_MONTH_12=dsc_xxx      # 12개월 할인 ID
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

##### 3. SSL 인증서 발급 (Let's Encrypt) — Phase B 단계별 가이드 (2026-05-07 보강)

> **현재 상태 (2026-05-07)**: Phase A (코드/docs/compose 정비) 완료. Phase B (production 활성) 미실행. 본 절차 = Phase B 단계별 가이드. **production 영향 큼 = 단계별 검증 + 롤백 가능 상태 유지**.
>
> **선행 조건**:
> 1. Cloudflare DNS 의 `api.amazingkorean.net` A 레코드 = grey-cloud (proxy off) 임시 전환 필요 (HTTP-01 챌린지가 origin 까지 도달해야). 발급 완료 후 다시 orange-cloud (proxy on).
> 2. 현재 Cloudflare SSL 모드 = **Flexible** (`§9` 참조). Phase B 완료 후 **Full Strict** 전환.

```bash
# === Phase B-1: 인증서 초기 발급 (origin 영향 0, 실패해도 안전) ===

# 1.1 Cloudflare DNS: api.amazingkorean.net A 레코드 = grey-cloud 임시 전환 (orange → grey).
#     EC2 public IP 가 직접 노출되므로 발급 직후 즉시 orange 복귀.

# 1.2 EC2 SSH 접속 후 작업 디렉터리 이동
cd ~/amazing-korean-api

# 1.3 인증서 발급 (--dry-run 으로 먼저 검증)
docker compose -f docker-compose.prod.yml run --rm certbot certonly \
  --webroot --webroot-path=/var/www/certbot \
  -d api.amazingkorean.net \
  --email kkryoun300@gmail.com \
  --agree-tos --no-eff-email \
  --dry-run

# dry-run 성공 확인 후 실제 발급
docker compose -f docker-compose.prod.yml run --rm certbot certonly \
  --webroot --webroot-path=/var/www/certbot \
  -d api.amazingkorean.net \
  --email kkryoun300@gmail.com \
  --agree-tos --no-eff-email

# 1.4 발급 결과 확인
ls -la certbot/conf/live/api.amazingkorean.net/
# 예상: cert.pem / chain.pem / fullchain.pem / privkey.pem

# 1.5 Cloudflare DNS: A 레코드 = orange-cloud 복귀 (grey → orange)
#     이 시점까지 origin nginx = HTTP only = 사용자 영향 0 (Cloudflare 가 HTTPS 종단)

# === Phase B-2: nginx HTTPS 블록 활성 (origin 영향 작음, reload 시 < 1초 다운) ===

# 2.1 nginx.conf 의 HTTPS server 블록 + ssl_stapling 블록 + HTTP→HTTPS redirect = 모두 주석 해제
#     주석 해제 위치 (3 곳):
#       - http { ... ssl_stapling on; ... } (HTTPS 활성 후 effect)
#       - HTTP server 안 redirect location / { return 301 https... } (Phase B-3 와 함께 활성)
#       - HTTPS server { listen 443 ssl; ... } 블록 전체
nano nginx/nginx.conf

# 2.2 nginx 설정 검증 (컨테이너 내에서 nginx -t)
docker exec amk-nginx nginx -t
# 예상: "syntax is ok" + "test is successful"

# 2.3 nginx reload (zero-downtime, master 프로세스 유지)
docker exec amk-nginx nginx -s reload

# 2.4 HTTPS 응답 검증 (origin 직접, Cloudflare 우회)
#     Phase B-1 에서 grey-cloud 잠시 전환 또는 EC2 public IP 직접 SNI 검증
curl -v https://api.amazingkorean.net/health --resolve api.amazingkorean.net:443:<EC2_PUBLIC_IP>
# 예상: TLS handshake 성공 + 200 + {"status":"ok",...}

# === Phase B-3: Cloudflare SSL 모드 전환 (Flexible → Full Strict) ===
#     ⚠️ 가장 위험. origin HTTPS 검증 안 되면 502 = production down.

# 3.1 Cloudflare 대시보드 → SSL/TLS → Overview
#     모드 = Flexible → Full (먼저 Full 으로, Strict 는 마지막)
# 3.2 사용자 페이지 정상 동작 확인 (curl + 브라우저)
# 3.3 모드 = Full → Full Strict 전환
#     Strict = origin cert valid + CA 신뢰 검증. Let's Encrypt = 자동 OK.
# 3.4 페이지 정상 동작 재확인. 502 발생 시 즉시 Full 또는 Flexible 로 롤백.

# === Phase B-4: HTTP→HTTPS redirect 활성 (선택, Cloudflare 가 이미 HTTPS 강제하므로 보조) ===
#     nginx HTTP server 블록의 location / { return 301 ... } 주석 해제
#     이미 Phase B-2 에서 동시에 주석 해제 가능.
```

**검증 절차 (Phase B 완료 후)**:

```bash
# 1. 외부 HTTPS 응답 (Cloudflare edge)
curl -sI https://api.amazingkorean.net/health | grep -E "HTTP|server|cf-"
# 예상: HTTP/2 200 / server: cloudflare / cf-ray: ...

# 2. 인증서 만료일 확인
echo | openssl s_client -connect api.amazingkorean.net:443 -servername api.amazingkorean.net 2>/dev/null \
  | openssl x509 -noout -dates
# 예상: notBefore = 발급 직전 / notAfter = +90일

# 3. SSL Labs 등급 (외부)
# https://www.ssllabs.com/ssltest/analyze.html?d=api.amazingkorean.net
# 목표: A 또는 A+ (TLS 1.2+1.3, OCSP stapling, HSTS)

# 4. nginx 컨테이너 안에서 cert 무결성
docker exec amk-nginx test -f /etc/letsencrypt/live/api.amazingkorean.net/fullchain.pem && echo "cert OK"
```

**자동 갱신 검증 (90일 주기)**:

```bash
# certbot 컨테이너 = 12시간마다 renew 시도 (docker-compose.prod.yml certbot.entrypoint).
# 갱신 30일 전부터 실제 갱신 trigger.

# renew dry-run 으로 갱신 가능 여부 검증
docker compose -f docker-compose.prod.yml run --rm certbot renew --dry-run
# 예상: "Congratulations, all simulated renewals succeeded"

# nginx reload hook (현재 미자동, 수동 또는 host crontab 권장)
# 갱신 후 nginx 가 신규 cert 자동 로드 X = reload 필수.
# host 측 crontab 예시 (매일 03:00):
#   0 3 * * * docker exec amk-nginx nginx -s reload >> /var/log/nginx-reload.log 2>&1
```

**롤백 절차 (Phase B 실패 시)**:

```bash
# 시나리오 1: nginx HTTPS 활성 후 origin 502 / 5xx 빈발
# → nginx.conf HTTPS 블록 재주석 + reload + Cloudflare 모드 = Flexible 복귀
nano nginx/nginx.conf  # HTTPS server { ... } 다시 주석
docker exec amk-nginx nginx -s reload
# Cloudflare 대시보드 → SSL/TLS → Overview → Flexible

# 시나리오 2: Cloudflare Full Strict 전환 후 502 빈발
# → 즉시 Full 또는 Flexible 로 롤백 (Cloudflare 대시보드 1 클릭)

# 시나리오 3: 인증서 발급 실패 (Let's Encrypt rate limit 또는 챌린지 실패)
# → Cloudflare DNS A 레코드 = grey-cloud 확인. 80 port 외부 도달 가능 확인.
# rate limit (50/주, 5/시간 per domain)
docker compose -f docker-compose.prod.yml logs certbot --tail 50
```

**관련 부채 마킹** (2026-05-07 갱신):
- `AMK_DEBTS.md` A4-1 SSL/HTTPS / A4-2 certbot = ✅ **Phase B 완료** (2026-05-07)
- `AMK_AUDIT_2026-05-04.md` N-13 nginx HTTPS = ✅ **Phase B 완료**
- 신규 부채 등재: `AMK_DEBTS.md` B8 SSL Labs B → A+ 강화 (Cloudflare SSL/TLS, 사용자 결정)

##### 3.1 Phase B 운영 학습 (2026-05-07 실측 정착)

> Phase B 실제 실행 시 발견한 운영 함정 + 해결 패턴. 향후 동일 작업 (다른 도메인 / 다른 EC2 / 인증서 재발급 등) 시 참조.

###### 함정 1: Docker bind mount stale (가장 큰 함정)

**증상**: host 측에서 `cp nginx-https-enabled.conf nginx.conf` 후 nginx 컨테이너 안에서 `nginx -t` 통과 (옛 config 가 syntax OK 라 그냥 통과) + `nginx -s reload` 성공 메시지. 그러나 실제로는 컨테이너 안 nginx.conf 가 **옛 버전 그대로**. TLS handshake 실패 (Connection refused / reset by peer).

**진단**:
```bash
# host vs 컨테이너 nginx.conf md5 비교 = 다르면 mount stale
md5sum nginx/nginx.conf
docker exec amk-nginx md5sum /etc/nginx/nginx.conf
```

**해결**: `docker restart amk-nginx` (mount 새로 attach + nginx fresh start). reload 로 안 되는 케이스 = restart.

**원인 추정**: docker bind mount 가 host file 의 inode 추적 시 일부 캐시 또는 docker engine 버그로 즉시 반영 안 되는 케이스. 흔하지 않으나 발생 가능.

**예방**: nginx config 변경 시 항상 md5 검증 후 reload, 다르면 restart.

###### 함정 2: Amazon Linux 2023 = cron 미설치

**증상**: `crontab -e` → "command not found".

**해결**:
```bash
sudo dnf install -y cronie
sudo systemctl enable --now crond
```

**근거**: Amazon Linux 2023 default = systemd timers 권장, cronie 미포함. 단순 cron 패턴은 cronie 가 더 직관적.

###### 함정 3: vim editor 입력 실수

**증상**: `crontab -e` 가 default editor (vim) 열림 → 입력 모드 / 명령 모드 혼란 / Ctrl+X 자동완성 모드 진입 등.

**우회 (가장 안전)**:
```bash
# editor 우회, file 로 직접 install
echo "0 3 * * * docker exec amk-nginx nginx -s reload >> /var/log/nginx-reload.log 2>&1" > /tmp/mycron
crontab /tmp/mycron
crontab -l  # 확인
rm /tmp/mycron
```

###### 함정 4: docker compose run --rm 멈춤

**증상**: `docker compose -f docker-compose.prod.yml --env-file .env.prod run --rm certbot certonly ...` 가 "Container Created" 후 5분 넘게 멈춤. stdout buffering 또는 기존 amk-certbot 컨테이너 lock.

**해결**: `docker run` 직접 사용 (compose 우회):
```bash
docker run --rm \
  -v /home/ec2-user/amazing-korean-api/certbot/www:/var/www/certbot \
  -v /home/ec2-user/amazing-korean-api/certbot/conf:/etc/letsencrypt \
  certbot/certbot certonly --webroot --webroot-path=/var/www/certbot \
  -d api.amazingkorean.net --email <admin@email> --agree-tos --no-eff-email
```

compose 의 env 검증 + service 디펜던시 우회 = 즉시 진행 (보통 30초-1분).

###### 함정 5: cert permission denied (host 측 ec2-user)

**증상**: `ls /home/ec2-user/.../certbot/conf/live/api.amazingkorean.net/` → "Permission denied".

**원인**: certbot 이 root 권한으로 cert 생성 (보안적 정상). nginx container 는 root 라 read 가능.

**해결 (확인 목적)**:
```bash
sudo ls /home/ec2-user/.../certbot/conf/live/api.amazingkorean.net/
# 또는
docker exec amk-nginx ls /etc/letsencrypt/live/api.amazingkorean.net/
```

###### 함정 6: SSL Labs B 등급 = Cloudflare edge default

**증상**: 모든 설정 정상인데 SSL Labs A+ 가 아닌 B 등급.

**원인**: Cloudflare edge 가 구식 클라이언트 호환 위해 일부 weak cipher 활성. origin nginx 와 무관.

**해결 옵션** (선택, B8 부채):
- Cloudflare → SSL/TLS → Edge Certificates → Minimum TLS Version = 1.2 + TLS 1.3 활성 (Free 가능)
- Pro+ 플랜 = Modern cipher suite

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

> **⚠️ HHMMSS(000001 등) 접미사 사용 금지**: `20260310000001`은 정수 `20,260,310,000,001`이 되어 `20260312`(= `20,260,312`)보다 훨씬 큰 값. sqlx는 정수 기준 오름차순으로 실행하므로 의존성 순서가 뒤집혀 서버 크래시 발생 (2026-03-23 사고). 정책 빠른 참조 = `migrations/README.md`. 부채 추적 = `AMK_DEBTS.md` G16 (legacy 14자리 2건 미해결, production 점진 적용으로 우회).

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

> **2026-05-07 정정**: 본 §9 = 초기 운영 시점 (2026-02-10) 기준 Flexible 모드 가이드. 본 docs §3 Phase B 절차로 **Full Strict 전환 권장 (보안 갭 해결)**. 본 §9 는 이력 + Cloudflare 정책 SSoT.

Cloudflare 프록시 사용 시 Let's Encrypt 없이 SSL 적용 가능:

1. Cloudflare 대시보드 → `amazingkorean.net` → **DNS**
2. `api` A 레코드의 프록시 상태를 **주황색 구름** (Proxied)으로 설정
3. **SSL/TLS** → **Overview** → 모드 = **Flexible** (초기) → **Full Strict** (Phase B 권장)

> **참고 (Flexible)**: Cloudflare ↔ 사용자 간 HTTPS, Cloudflare ↔ EC2 간 HTTP. **Cloudflare ↔ origin 사이 = 평문 = 보안 갭**. 중간자 공격 가능 (Cloudflare 인프라 신뢰 가정에 의존).
>
> **권장 (Full Strict)**: origin 도 HTTPS + valid CA 검증 (Let's Encrypt 자동 OK). end-to-end 암호화. 전환 절차 = `§3 Phase B` 참조.

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

##### 11. EC2 호스트 OS 보안 (커널 mitigation)

> §1~§10 = 네트워크/앱 레이어 (Cloudflare / 인증서 / 데이터). 본 §11 = 호스트 OS / Linux 커널 레이어. distro 보안 패치 백포트 도착 전, modprobe 블랙리스트로 취약 커널 모듈을 차단하는 임시 mitigation 절차.

###### 11-1. dirtyfrag (CVE-2026-43284 xfrm-ESP + CVE-2026-43500 RxRPC) — 2026-05-13 적용

**배경**

- 공개: 2026-05-07 (embargo 깨짐, [V4bel/dirtyfrag](https://github.com/V4bel/dirtyfrag))
- 영향: Linux 커널 Local Privilege Escalation (LPE) 2 CVE 체인. xfrm-ESP / RxRPC Page-Cache Write
- 메인라인 패치: 2026-05-05 (`f4c50a4034e6`) / 2026-05-10 (`aa54b1d27fe0`)
- 영향 distro: Ubuntu 24.04 / RHEL 10 / CentOS Stream 10 / AlmaLinux 10 / Fedora 44 / openSUSE Tumbleweed 등 — 우리 EC2 (Amazon Linux 2023) 도 잠재 영향
- 전제: 공격자가 **로컬 셸 접근(RCE 선행)** 을 얻은 후 root 권한 탈취. 단독 인터넷 공격 불가
- 대응 필요성: distro 백포트 도착 전까지 블랙리스트로 1차 방어선 확보

**적용 절차** (EC2 SSH 후 `ec2-user`)

```bash
sudo sh -c "printf 'install esp4 /bin/false\ninstall esp6 /bin/false\ninstall rxrpc /bin/false\n' > /etc/modprobe.d/dirtyfrag.conf"
sudo rmmod esp4 esp6 rxrpc 2>/dev/null
sudo sh -c "echo 3 > /proc/sys/vm/drop_caches"
```

**검증**

```bash
cat /etc/modprobe.d/dirtyfrag.conf   # 3 줄: install esp4/esp6/rxrpc /bin/false
lsmod | grep -E "(esp4|esp6|rxrpc)"  # 빈 결과 (취약 모듈 로드 없음)
```

**영향 평가**

우리 앱 (Rust/Axum HTTP API + Cloudflare 엣지) 은 IPsec(ESP) / RxRPC 커널 모듈을 미사용. 블랙리스트로 차단해도 기능 영향 0.

**해제 시점**

Amazon Linux 2023 보안 패치 백포트 도착 후 `sudo dnf update` 적용. 블랙리스트는 모듈 미사용이라 그대로 둬도 무해 (지워도 OK).

###### 11-2. 일반 커널 CVE 대응 SOP (재사용)

다른 커널 CVE 공시 시 §11-1 패턴 재사용:

1. **영향 모듈 식별** — CVE writeup 의 affected subsystem / module 명
2. **우리 앱 사용 여부 확인** — `lsmod | grep <module>` + 코드 grep
3. **미사용 = 블랙리스트** — `/etc/modprobe.d/<cve-id>.conf` 에 `install <module> /bin/false` 추가
4. **즉시 적용** — `sudo rmmod <module>` + `echo 3 > /proc/sys/vm/drop_caches`
5. **distro 패치 도착** — `sudo dnf update` (AL2023) / `sudo apt upgrade` (Ubuntu)
6. **문서 동기화** — `docs/AMK_DEPLOY_OPS.md §11-N` 신설 + `STATUS` 신규 entry + `CHANGELOG` 기록

> **주의**: 모듈을 우리 앱이 **사용 중**이면 블랙리스트 불가. 그 경우 (a) distro 패치 빠른 적용 / (b) 인스턴스 격리 / (c) WAF/방화벽 추가 룰 중 선택.

###### 11-3. RCE 선행 공격면 검증 (2026-05-17, dirtyfrag 후속)

> §11-1 dirtyfrag mitigation 은 "공격자가 로컬 셸(RCE 선행) 확보 후 root 탈취" 가 전제 — 2차 방어선. 1차 방어선(애초에 셸을 못 따게)이 비어 있으면 반쪽이라, STATUS #140 후속 별도 트랙으로 RCE 선행 공격면을 실측 검증. **결과 = 이미 양호, 코드/구성 변경 0건.**

| 점검 | 도구 | 결과 |
|------|------|------|
| Docker socket 마운트 | repo grep | **0건** — certbot 주석이 회피 사유 명시 |
| privileged / cap_add / host network | repo grep | **0건** |
| 내부 서비스 포트 노출 | `docker-compose.prod.yml` | db/redis/api 호스트 매핑 없음, nginx만 80/443 (`amk-network` 브리지 격리) |
| 의존성 advisory CI | `.github/workflows/security-audit.yml` | **이미 통합** — cargo-deny(RUSTSEC) + npm audit, 주간 cron + 수동 |
| 의존성 advisory PR 게이트 | `.github/workflows/pr-check.yml` (`cargo-deny` job) | **2026-05-17 신설(2.4)** — KKRYOUN push 머지 전 차단. **deny.toml 변경 = 푸시 전 로컬 `cargo deny check` 선행 필수**(blind CI 왕복 금지, `cargo-deny 0.19.6` 로컬 설치됨). yanked=warn(core2 회피불가 전이) / 외부 wildcard·advisory·미상 라이선스는 deny 유지. SSoT=`AMK_API_SECURITY_AUDIT.md §2.4` |
| EC2 배포 인증 | `deploy.yml` | SSH **키 인증** (ec2-user, `EC2_SSH_KEY`), 비번 미사용 |
| EC2 sshd 실효 설정 | `sudo sshd -T` | `passwordauthentication no` / `permitemptypasswords no` / `kbdinteractiveauthentication no` / `pubkeyauthentication yes` — **비번 무차별 공격면 0** |

**비채택 hardening (효익 0 수렴, 기록만)**

- `PermitRootLogin without-password` → `no`: SSH root 로그인 경로는 (a) 비번 = 이미 `no` 로 차단 / (b) `/root/.ssh/authorized_keys` 키 = Amazon Linux 기본값이 강제 커맨드로 직접 root SSH 무력화. `no` 가 막는 건 "미래에 root authorized_keys 에 평키 추가" 가상 시나리오뿐 → 현재 갭 아님. **그대로 둠.**
- fail2ban 설치: `passwordauthentication no` 라 무차별 표적 자체가 없음 → 가치 낮음. **미설치 유지.**

> **재검증 시점**: sshd_config / docker-compose.prod.yml / 워크플로 보안 관련 변경 시 본 표 항목 재확인.

##### 12. 해설(설명) 콘텐츠 프로덕션 시딩

> 해설 콘텐츠(`explanation_unit`/`explanation_block` + `content_translations`)는 마이그레이션과 **별개**로 시드 데이터를 1회 투입해야 실제 서빙된다. 자동 배포에 결합하지 않음(멱등이나 매 배포 불필요·결합 회피, `seeds/` "수동 투입" 관례 동일). SSoT = `AMK_API_LEARNING.md §5.10`.

**구성**

- 시드 파일: `seeds/explanation_seed.json` (books 산출, ~1.8MB / 568 unit · 1,317 block · 4,362 en 번역행). 런타임 이미지에 `COPY seeds`로 포함.
- 적재 바이너리: `seed_explanation` (Dockerfile builder→runtime COPY, `/app/seed_explanation`). 멱등(unit_idx·(unit_idx,block_seq)·튜플 ON CONFLICT upsert).

**실행 (EC2, 배포 후 1회 — 수동)**

```bash
# ~/amazing-korean-api 에서. amk-api 컨테이너 내 바이너리 + 이미지 내 시드 파일 사용
docker exec amk-api /app/seed_explanation --input /app/seeds/explanation_seed.json
```

출력: `적재 완료: unit=568 block=1317 translation=4362` + 연결키 정합 리포트(`study_idx`/`study_task_idx` 미해소 count = study/study_task 시드 상태 의존, 논리 참조라 경고). DB 접속은 컨테이너의 `DATABASE_URL`(compose env) 사용 — 별도 설정 불필요.

**재시딩 시점**

books 가 원문(guide_67/tense_v1/josa_v1) 변경으로 `explanation_seed.json` 재생성·통지 시에만 → 새 파일 `seeds/` 커밋·배포 후 위 명령 재실행(멱등). 평시 불필요.

**검증**

```bash
curl -s "https://api.amazingkorean.net/explanations/guide67:pr_105_114?lang=ko" | head -c 300
curl -s "https://api.amazingkorean.net/explanations?study_task_idx=amk500-sent-001&lang=en"
```

> **번역 트랙 (맥미니 Phase C, 미도착)**: 35언어는 `seed_explanation --translations explanation_translations.{lang}.json` 모드로 후속 적재 (계약 = `AMK_API_LEARNING.md §5.10` 번역 트랙, 구현은 산출 도착 시).

##### 13. DB 최소권한 전환 (보안 감사 2.3) — `amk_app` 비-superuser role

> 앱이 PostgreSQL `postgres`(cluster superuser)로 접속하던 것을 비-superuser 소유 role 로 전환. superuser 폭발 반경(COPY PROGRAM=서버 RCE / CREATE ROLE / 타 DB / ALTER SYSTEM) 제거. **2단계** — Phase 1=프로비저닝(런타임 영향 0), Phase 2=컷오버(게이트). SSoT = `AMK_API_SECURITY_AUDIT.md §2.3`.

**Phase 1 — 프로비저닝 (런타임 영향 0, 앱은 계속 postgres 접속)**

- `db-init/10_least_priv_role.sql` (멱등) = `amk_app` LOGIN NOSUPERUSER 생성 + public 스키마 GRANT/ALTER DEFAULT PRIVILEGES + 기존 앱 객체(table/seq/view/enum) OWNER → amk_app.
- `docker-compose.prod.yml` db 서비스: `./db-init:/docker-entrypoint-initdb.d:ro` 마운트(신규/재생성 DB 자동·멱등) + `APP_DB_PASSWORD` env.
- **기존 prod 볼륨엔 initdb 미실행** → 1회 수동 적용 (EC2, §12 시딩과 동일 패턴). `APP_DB_PASSWORD` = Phase 2 에서 쓸 신규 비밀번호를 이때 생성·고정:

```bash
APP_DB_PASSWORD='<생성한-강한-비밀번호>' \
docker exec -i -e APP_DB_PASSWORD="$APP_DB_PASSWORD" amk-pg \
  psql -U postgres -d amazing_korean_db -v ON_ERROR_STOP=1 \
  -f /docker-entrypoint-initdb.d/10_least_priv_role.sql
# 검증: SELECT rolname,rolsuper FROM pg_roles WHERE rolname='amk_app'; → amk_app|f
```

> 가드: `APP_DB_PASSWORD` 미설정/빈값이면 스크립트가 중단(빈 비밀번호 role 방지). 멱등 — 재실행 안전.

**Phase 2 — 컷오버 (별도 게이트, 사용자 명시 승인 필요)**

`DATABASE_URL` 의 user 를 `postgres` → `amk_app` 으로 교체. **4곳 동시 반영 필수**(feedback_deploy_env_sync / INC-001 클래스):

1. GitHub Secret `APP_DB_PASSWORD` 추가 (Phase 1 에서 고정한 값)
2. `docker-compose.prod.yml`: `DATABASE_URL: postgres://amk_app:${APP_DB_PASSWORD}@db:5432/amazing_korean_db`
3. `.github/workflows/deploy.yml` `.env.prod`: `APP_DB_PASSWORD=${{ secrets.APP_DB_PASSWORD }}`
4. 본 §13 갱신

- 배포 후 검증: `/health` 200 + `SELECT current_user` = `amk_app` + 부팅 sqlx 마이그 정상.
- **롤백**: `DATABASE_URL` user 를 `postgres` 로 환원(시크릿/compose 1줄) = 즉시 복구. `postgres` 는 break-glass(수동 관리)용으로만 잔존.

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

##### Workflow 파일 구성 (2026-05-04 후)

| 파일 | 책임 | 트리거 | 비고 |
|------|------|--------|------|
| `.github/workflows/deploy.yml` | EC2 배포 (Docker build + push + SCP + SSH + health check) | `push: [main]` + `workflow_dispatch` | 머지 후 production 배포만. KKRYOUN 제거 (INC-005) |
| `.github/workflows/pr-check.yml` | 머지 전 검증 (backend cargo check + clippy / frontend build + lint) | `push: [KKRYOUN]` + `workflow_dispatch` | 배포 책임 없음. 검증 fail 시 PR status 에 빨간 X |

**책임 분리 원칙**: 배포와 검증을 별도 워크플로로 분리. INC-005 학습 = 한 워크플로에 다중 책임 (배포 + 검증) 두면 트리거 충돌 + race condition 위험.

##### Workflow 파일 (.github/workflows/deploy.yml)

> **Branch trigger 정책 (2026-05-04, INC-005 후)**: `branches: [main]` 단일. KKRYOUN push deploy 는 INC-005 근본 원인 (KKRYOUN deploy 의 migration 적용 + 다른 PR main 머지 시 sqlx 'previously applied but missing' panic) 으로 영구 제거. PR 머지 시점 (main push) 에만 deploy. 긴급 hotfix 는 `workflow_dispatch` 수동 실행. 상세: `AMK_CHANGELOG.md` 2026-05-04 INC-005 엔트리.

```yaml
name: Deploy to EC2

on:
  push:
    branches: [main]      # KKRYOUN 제거 (INC-005, 2026-05-04)
  workflow_dispatch:  # 수동 실행 가능 (긴급 hotfix 등)

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

# E-book 페이지 이미지 디렉터리 (RDS 이전 전까지 EC2 local fs)
du -sh "${EBOOK_PAGE_IMAGES_DIR:-docs/textbook/page-images}"
ls "${EBOOK_PAGE_IMAGES_DIR:-docs/textbook/page-images}/student" 2>/dev/null | wc -l   # 언어 디렉터리 수
```

##### E-book 페이지 이미지 모니터링 (2026-05-03 정책)

`${EBOOK_PAGE_IMAGES_DIR}` 은 books 측 빌드 결과 (`dist/ebook_pages/`) 가 EC2 로 동기화되는 디렉터리. RDS 이전 전까지 EC2 local fs 정책 (`AMK_API_EBOOK.md` "페이지 이미지 저장 위치 정책" 참조).

| 항목 | 임계값 | 액션 |
|------|--------|------|
| EC2 디스크 여유 (전체) | < 30% | 즉시 점검. Docker prune + 로그 정리 우선 |
| `${EBOOK_PAGE_IMAGES_DIR}` 크기 | > 2GB | 콘텐츠 추가 추세 점검. 2GB 도달 시 RDS 이전 + S3 전환 우선순위 상향 결정 |
| 36 lang × 2 edition manifest 카운트 | < 72 | books 동기화 누락 점검 |
| `manifest.json` 무결성 | json parse 실패 | books 빌드 재실행 + 동기화 재시도 |

**현재 baseline** (2026-05-03 books 빌드 기준):
- 8,928 페이지 / 693MB / 36 lang × 2 edition = 72 manifest
- 단일 페이지 평균 ~78KB (1587×2245px WebP quality 85)

**RDS 이전 시점 전환 트리거** (`AMK_STATUS §8.2 검증된 리스크` Q9):
- ebook 도메인 9곳 `fs::read` → S3 SDK 호출 전환
- `${EBOOK_PAGE_IMAGES_DIR}` 데이터 → S3 bucket 1회 `aws s3 sync`
- 환경변수 `EBOOK_PAGE_IMAGES_DIR` → `EBOOK_PAGE_IMAGES_S3_BUCKET` (가칭) 전환
- CloudFront signed URL 보안 강화 검토

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

##### EC2 디스크 사용량 모니터링 (A4-3)

EC2 t2.micro = 8GB 기본 디스크. 누적 항목:
- Docker 이미지 (build cache, dangling, postgres/redis volume)
- 로그 파일 (Docker container — A4-5 log 로테이션 후 서비스당 max 30MB)
- ebook 페이지 이미지 (Q14 이후 추가, EC2 local fs 정책)
- DB 데이터 (postgres_data volume)

**조회 명령** (EC2 SSH 후):

```bash
# 전체 디스크 사용량
df -h

# Docker 사용량 (이미지 / 컨테이너 / 볼륨 / 빌드 캐시)
docker system df -v

# 가장 큰 디렉토리 top 20
sudo du -sh /var/lib/docker/* /home/* /opt/* 2>/dev/null | sort -hr | head -20

# postgres volume 사용량
docker exec amk-pg du -sh /var/lib/postgresql/data
```

**임계값 권고** (수동 확인 시):
- /dev/root 사용률 > **70%** = 정리 권장 (`docker system prune -a`)
- /dev/root > **85%** = 즉시 정리 (운영 위험)
- /dev/root > **95%** = 알림 (배포 중단 가능)

**정리 명령**:

```bash
# 미사용 Docker 자원 정리 (안전, 활성 컨테이너 영향 X)
docker system prune -a --volumes  # 단 unused volume 도 삭제 = 신중

# 안전한 정리 (volume 보존)
docker system prune -a  # image + container + network + build cache

# 컨테이너 로그 강제 truncate (max-size 외 임시)
sudo truncate -s 0 $(docker inspect --format='{{.LogPath}}' amk-api)
```

**향후 자동화 후속**: GitHub Action 으로 EC2 SSH + df -h 점검 + 임계 초과 시 알림 (별도 트랙). UptimeRobot / Cloudflare Pro 시점에 통합 가능.

##### nginx Rate Limit 위반 모니터링 (A4-7)

`nginx/nginx.conf:31` 의 `limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s` 정의 = 10 req/sec/IP 초과 시 503/429 응답. 위반 발생 시 nginx 가 `error.log` 에 `warn` 레벨 로그 기록.

**조회 명령** (EC2 SSH 후):

```bash
# 최근 위반 (실시간)
docker logs amk-nginx 2>&1 | grep 'limiting requests' | tail -20

# 24시간 위반 카운트
docker logs --since 24h amk-nginx 2>&1 | grep -c 'limiting requests'

# 위반 IP 별 카운트 (분석)
docker logs --since 24h amk-nginx 2>&1 | grep 'limiting requests' \
  | grep -oE 'client: [0-9.]+' | sort | uniq -c | sort -rn | head -10
```

**로그 형식 예시**:
```
2026/05/05 06:30:12 [warn] 7#7: *123 limiting requests, excess: 0.456 by zone "api_limit", client: 1.2.3.4, server: api.amazingkorean.net, request: "GET /lessons HTTP/2.0"
```

**대응 정책**:
| 상황 | 대응 |
|------|------|
| 단발성 위반 (정상 사용자 burst) | 수용 (10r/s = 충분, 일시 spike 정상) |
| 동일 IP 다수 위반 (>50건/h) | Cloudflare WAF 에서 해당 IP 차단 검토 |
| zone 차원 다수 위반 | rate / burst 파라미터 재조정 또는 zone 분리 (auth / general 등) |

**향후 자동화 후속**: cron + 위반 카운트 임계 초과 시 알림 (현재 수동 조회).

##### DB·Redis 백업 절차 (A4-4 옵션 A 수동 정기, 2026-05-07 정착)

> 정기 백업 절차. 데이터 마이그레이션 (§4-3 dev→prod 이전) 과 별개.
> **현재 상태 (2026-05-07)**: **옵션 A 수동 정기 결정 정착**. EC2 측 archive 일일 cron + 사용자 PC scp pull (`D:\` 로컬 보관). 본 docs = SSoT.

###### PostgreSQL 백업

**컨테이너**: `amk-pg` / **DB**: `amazing_korean_db` / **Volume**: `amazing-korean-api_postgres_data`

```bash
# 백업 디렉터리 준비
mkdir -p ~/backup

# 논리 백업 (권장 — 마이그레이션 호환성 ↑)
docker exec amk-pg pg_dump -U postgres -d amazing_korean_db \
  --exclude-table=_sqlx_migrations \
  > ~/backup/db-$(date +%Y%m%d-%H%M%S).sql

# 압축 백업 (디스크 절약)
docker exec amk-pg pg_dump -U postgres -d amazing_korean_db \
  --exclude-table=_sqlx_migrations \
  | gzip > ~/backup/db-$(date +%Y%m%d-%H%M%S).sql.gz

# 전체 cluster 백업 (모든 DB + role + 권한, 신규 EC2 마이그 시 유용)
docker exec amk-pg pg_dumpall -U postgres > ~/backup/cluster-$(date +%Y%m%d).sql
```

###### PostgreSQL 복구

```bash
# 1. API 중지 (DB 연결 종료)
docker stop amk-api

# 2. 기존 연결 강제 종료 + DB 리셋
docker exec -it amk-pg psql -U postgres -c \
  "SELECT pg_terminate_backend(pid) FROM pg_stat_activity \
   WHERE datname = 'amazing_korean_db' AND pid <> pg_backend_pid();"
docker exec -it amk-pg psql -U postgres -c "DROP DATABASE amazing_korean_db;"
docker exec -it amk-pg psql -U postgres -c "CREATE DATABASE amazing_korean_db;"

# 3. 백업 파일 import (압축본은 zcat)
docker exec -i amk-pg psql -U postgres -d amazing_korean_db < ~/backup/db-YYYYMMDD-HHMMSS.sql
# 또는: zcat ~/backup/db-YYYYMMDD-HHMMSS.sql.gz | docker exec -i amk-pg psql -U postgres -d amazing_korean_db

# 4. API 재시작
docker start amk-api
docker logs -f amk-api  # 마이그레이션 자동 실행 확인
```

###### Redis 백업

**컨테이너**: `amk-redis` / **Volume**: `redis_data` (full: `amazing-korean-api_redis_data`) / **인증**: `REDIS_PASSWORD` 필수

```bash
# 1. RDB snapshot 강제 생성 (BGSAVE = non-blocking)
docker exec amk-redis redis-cli -a "$REDIS_PASSWORD" BGSAVE

# 2. 마지막 SAVE 시각 확인 (BGSAVE 완료 polling)
docker exec amk-redis redis-cli -a "$REDIS_PASSWORD" LASTSAVE

# 3. RDB 파일 호스트로 복사 (volume 내 위치 = /data/dump.rdb)
docker cp amk-redis:/data/dump.rdb ~/backup/redis-$(date +%Y%m%d-%H%M%S).rdb
```

###### Redis 복구

```bash
# 1. API + Redis 중지
docker stop amk-api amk-redis

# 2. RDB 파일을 redis_data volume 으로 복사
docker run --rm \
  -v amazing-korean-api_redis_data:/data \
  -v ~/backup:/backup \
  alpine cp /backup/redis-YYYYMMDD-HHMMSS.rdb /data/dump.rdb

# 3. Redis 먼저 시작 → API 시작
docker start amk-redis
sleep 3
docker start amk-api
```

###### 백업 정책 (옵션 A 수동 정기, 2026-05-07 결정 정착)

| 항목 | 결정 |
|------|------|
| 방식 | **옵션 A 수동 정기**. EC2 측 일일 archive cron + 사용자 PC scp pull (D:\ 보관). RDS 이전 전까지 (A2 트리거) 본 방식 유지 |
| 주기 | EC2 archive: 일 1회 KST 03:00 (트래픽 최저) / 사용자 PC pull: 사용자 재량 (주 1-2회 권장) |
| EC2 측 보관 | 일 7개 (`BACKUP_RETENTION_DAYS=7`, `scripts/backup.sh` 자동 회전) |
| 사용자 PC 측 보관 | `D:\` 폴더 회전 권장 = 일 14개 (2주) + 주 4개 (1개월) — 사용자 재량 |
| 저장 위치 | EC2: `~/backup/amk-YYYYMMDD-HHMMSS.tar.gz` / 사용자 PC: `D:\amk-backup\` |
| 암호화 | 미적용 (옵션 A = 사용자 PC 신뢰). RDS 이전 시 SSE-S3 자동화 |
| 복구 RTO/RPO | RTO < 1h (수동 scp + 복구 절차) / RPO < 24h (일 1회 cron) |

###### 자동화 — EC2 측 cron (정착, `scripts/backup.sh`)

**스크립트**: `scripts/backup.sh` (배포 시 EC2 동기화 = `git pull` 자동 포함)

**동작**:
1. PostgreSQL `pg_dump` (gzip, `_sqlx_migrations` 제외)
2. Redis `BGSAVE` + `LASTSAVE` polling (최대 60초) → `dump.rdb` 복사
3. tar.gz 통합 archive (`amk-YYYYMMDD-HHMMSS.tar.gz`)
4. `BACKUP_RETENTION_DAYS=7` 초과 archive 자동 삭제

**EC2 cron 등록 (1회성)**:

```bash
# 1) backup.sh 권한 확인 (배포 후 실행 권한 보존되는지 확인)
chmod +x ~/amazing-korean-api/scripts/backup.sh

# 2) 백업 디렉터리 사전 생성
mkdir -p ~/backup

# 3) crontab 등록
crontab -e
# 아래 라인 추가 (KST 03:00 = UTC 18:00 KST 이전일, EC2 timezone 확인 필요)
0 3 * * * /home/ubuntu/amazing-korean-api/scripts/backup.sh >> /home/ubuntu/backup/backup.log 2>&1

# 4) 등록 확인
crontab -l

# 5) 즉시 1회 테스트 (cron 등록 전 검증)
~/amazing-korean-api/scripts/backup.sh
ls -lah ~/backup/  # amk-*.tar.gz 생성 확인
tail -30 ~/backup/backup.log  # 로그 확인 (수동 실행 시 stdout)
```

**EC2 timezone 확인 (cron 시간 기준)**:

```bash
date  # 현재 EC2 timezone 확인 (KST or UTC)
# UTC 면 cron 라인을 KST 03:00 환산 = UTC 18:00 (전일)
# 0 18 * * * → KST 03:00 (다음날)
```

###### 사용자 PC scp pull 가이드 (Windows + WSL)

**1회성 셋업**:

```powershell
# Windows PowerShell (관리자) — 보관 폴더 생성
New-Item -Path "D:\amk-backup" -ItemType Directory -Force
```

```bash
# WSL — SSH key 설정 (이미 EC2 SSH 가능하면 skip)
ls ~/.ssh/  # 기존 EC2 키 확인
# scp 시 사용할 EC2 host alias 등록 권장 (~/.ssh/config)
```

**정기 pull 명령 (주 1-2회 권장, 수동 실행)**:

```bash
# WSL 안에서 실행
EC2_HOST="ubuntu@<EC2_IP_OR_DNS>"
EC2_KEY="~/.ssh/<key>.pem"
DEST="/mnt/d/amk-backup"

# EC2 의 archive 목록 확인
ssh -i "$EC2_KEY" "$EC2_HOST" 'ls -lah ~/backup/amk-*.tar.gz'

# 최신 archive 만 pull (rsync 권장 = 중복 전송 회피)
rsync -avz -e "ssh -i $EC2_KEY" "$EC2_HOST:~/backup/amk-*.tar.gz" "$DEST/"

# 또는 scp (단순)
scp -i "$EC2_KEY" "$EC2_HOST:~/backup/amk-*.tar.gz" "$DEST/"
```

**사용자 PC 측 회전 (선택, 수동)**:

```powershell
# Windows PowerShell — D:\amk-backup 안 14일 이전 파일 삭제
Get-ChildItem "D:\amk-backup\amk-*.tar.gz" | Where-Object { $_.LastWriteTime -lt (Get-Date).AddDays(-14) } | Remove-Item
```

**복구 절차**: 위 §6 [PostgreSQL 복구 절차] + [Redis 복구 절차] 참조. archive 압축 해제 후 동일.

###### 향후 자동화 (RDS 이전 시점, A2 트리거)

- AWS RDS automated backups (point-in-time restore + 자동 보관 7-35일)
- ElastiCache automated snapshots (일 1회 + 보관 35일)
- S3 lifecycle policy (보관 기간 만료 자동 삭제)
- CloudWatch alarm (백업 실패 시 알림)

> **본 옵션 A 의 한계**: 사용자 PC 의존 (PC 손상 시 백업 손실 위험) + 수동 pull (잊을 위험) + 암호화 미적용. RDS 이전 시 AWS 관리형으로 전환 권장.

###### 데이터 손실 우려 시점

| 상황 | 영향 | 대응 |
|------|------|------|
| EBS 볼륨 손상 | postgres_data + redis_data 모두 손실 | 마지막 백업으로 복구 (RPO = 백업 주기) |
| EC2 인스턴스 종료/재시작 | volume 분리 시 영향, 통상 재시작 시 무영향 | volume 무결성 확인 (`docker volume inspect`) |
| `docker volume rm` 실수 (§4-2 클린 배포) | 전체 데이터 초기화 | 백업 복구 필수 — 본 절차 |
| RDS 이전 시 (Q9, A2 묶음) | 일회성, 기존 데이터 신규 RDS 로 마이그 | dev→prod 이전 패턴 (§4-3) 응용 + RDS native restore |

## 7. 품질 보증 (QA) & 스모크 체크

- **CI Gate — `pr-check.yml` 자동 실행 (KKRYOUN push 시점, 2026-05-04 도입)**

  | Job | 명령 | 목적 |
  |-----|------|------|
  | backend | `cargo check --locked --workspace` | 컴파일 + Cargo.lock 정합성 (`SQLX_OFFLINE=true`, `.sqlx/` 캐시 사용) |
  | backend | `cargo clippy --lib --bins --locked -- -D warnings` | lint + warning fail-closed |
  | frontend | `npm run build` (= `tsc -b && vite build`) | 타입 체크 + 빌드 성공 |
  | frontend | `npm run lint` | ESLint |
  | frontend | `npm run lint:ui` | 하드코딩 색상 검사 (Tailwind 토큰화 정책) |

  실패 시 PR 페이지에 빨간 X. 머지 전 차단 트리거 (현재 branch protection 미적용이라 강제는 아님 — 사용자 본인 판단).

- **정적 분석 (CI Gate, frontend 만 — 별도 PR 검사 도입 전 기존 권장)**
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

## 7.5. 외부 모니터링 & 알림 (External Uptime Monitoring)

> INC-001 (2026-04-15 프로덕션 2h33m 다운) 재발 방지.
> GitHub Actions 의 deploy "success" 가 실제 서비스 가동을 보장하지 않는 것이 교훈이라 **독립된 외부 감시** 체계를 둔다.

### 구성

| 항목 | 값 |
|------|-----|
| 서비스 | **UptimeRobot** (Free 플랜, 50 모니터 한도) |
| 모니터 타입 | **HTTP / website monitoring** (Status code 200 기반) |
| 대상 URL | `https://api.amazingkorean.net/health` |
| Interval | **5 minutes** (Free 플랜 최소) |
| Alert 채널 | E-mail → `amazingkoreancenter@gmail.com` |
| Region | Default (auto-select, N. America) |

### 왜 Status code 모니터인가 (Keyword 모니터 포기 경위)

초기 설계로 Keyword 모니터(body 에 `live` 문자열 검색) 를 시도했으나 **Cloudflare Free 플랜에서 불가능**:

1. nginx `/health` 응답을 40B JSON 으로 반환 → CF edge 가 `Accept-Encoding: gzip` 요청에 대해 **자체 Brotli/gzip 압축** → UptimeRobot probe 가 raw 바이트에서 `live` substring 검색 실패 → 영원히 DOWN.
2. nginx `gzip off` + `Cache-Control: no-transform` 헤더 시도했으나 CF 가 무시하거나 헤더 자체를 strip (curl 실측 확인).
3. CF Free 플랜은 **Custom Request Headers** 도 유료 (UptimeRobot 측에서 `Accept-Encoding: identity` 보내는 회피 불가).
4. Grey-cloud DNS + 자체 SSL 은 작업 과다 (Let's Encrypt 자동 갱신 등).

**결론**: 상태코드 200 체크만으로 INC-001 감지 목적 달성. 실제 INC-001 은 컨테이너 crash → CF 521 → 200 아님 → DOWN 감지됨. "200 + 이상한 body" 시나리오는 현실적 발생 빈도 낮음.

### 발사 테스트 (최초 세팅 시 1회)

```
Keyword (임시) 모니터로 테스트 → 알림 이메일 수신 확인 → 실 HTTP 모니터로 교체
```

2026-04-17 초기 세팅 시 DOWN 알림 1~2분 내 수신 확인됨 (Keyword 모니터로 발사).

재발사 절차 (정기 점검 시):
1. EC2 SSH → `docker compose -f docker-compose.prod.yml --env-file .env.prod stop api`
2. 5~10분 대기 → 이메일 수신 확인
3. `docker compose ... start api` → 복구 알림 수신 확인

### 향후 업그레이드 옵션

- **Cloudflare Pro ($25/월)**: CF Health Checks (60초 간격) + Notifications → 즉시성 향상, 외부 의존 축소.
- **grey-cloud 서브도메인**: `origin-health.amazingkorean.net` A 레코드 직접 EC2 IP + 자체 SSL → CF 우회해 Keyword 모니터 복원 가능. 작업 공수 중.
- **Better Uptime / Pingdom**: 복수 regions, 3분 간격 등 UptimeRobot Free 보다 세밀.

### nginx `/health` 특수 처리

외부 모니터링 확장성 대비 origin 응답을 평문으로 유지:

```nginx
# nginx/nginx.conf
gzip_min_length 1024;  # 작은 응답 전반 압축 제외

location = /health {
    gzip off;          # origin 레벨 명시 (이중 안전장치)
    proxy_pass http://api;
    ...
}
```

CF 재압축은 Free 플랜에서 끊기 어려우나 origin 은 깨끗하게 유지 → 향후 grey-cloud 도입 시 즉시 keyword 매칭 동작.

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

## 7.6. Cloudflare 운영 정책 (DNS / Pages / SSL / Email Routing)

> 본 절 = A4-6 (AMK_DEBTS) 처리 — Cloudflare 사용 영역 통합 SSoT.

### 사용 영역

| 영역 | 용도 | Cloudflare 대시보드 위치 |
|------|------|---------|
| **DNS** | `amazingkorean.net` zone — A/CNAME/MX/TXT 관리 | DNS → Records |
| **Pages** | frontend 배포 (`amazingkorean.net`) — Git 연동 자동 빌드 | Workers & Pages → `amazing-korean-api` |
| **SSL/TLS** | **Full (Strict)** 모드 (Phase B 완료 2026-05-07: CF↔사용자 HTTPS + CF↔EC2 HTTPS end-to-end, origin Let's Encrypt cert, Min TLS 1.2) | SSL/TLS → Overview / Edge Certificates |
| **WAF / Security** | AI Crawl Control / Bot Fight Mode / Managed robots.txt | Security → Settings |
| **Email Routing** | `noreply@amazingkorean.net` 등 alias → 외부 메일박스 (Resend 발신과 별개) | Email Routing → Routes |

### 변경 절차 (정상 운영 시)

| 작업 | 절차 |
|------|------|
| DNS 신규 레코드 | DNS → Records → Add → 변경 후 propagation 1-5분 |
| frontend 재배포 | Cloudflare Pages 자동 (KKRYOUN main 머지 시 트리거) — 수동 재배포 = Pages → Deployments → "Retry deployment" |
| SSL 인증서 갱신 | Universal SSL (자동, 90일 갱신) — 수동 개입 X |
| WAF 룰 변경 | Security → WAF → Custom rules. 적용 즉시 |
| Email 라우팅 | Email Routing → Routes → Edit. propagation 1-2분 |
| 관리형 robots.txt | Security → Settings → "AI Crawl Control" 토글. 본 프로젝트 = ON 유지 (#65 SEO hardening 결정) |

### 외부 의존성

- **Cloudflare 계정**: HYMN Co., Ltd. 단일 계정 (`amazingkorean.net` zone owner)
- **API 토큰**: 자동화 시 별도 발급 (현재 미사용 = 수동 운영)
- **DNS 변경 권한**: 계정 owner 만

### 비상 시 절차

| 상황 | 대응 |
|------|------|
| Cloudflare edge 다운 (region-wide) | DNS grey-cloud 임시 전환 (orange→grey, EC2 IP 직접 노출). Phase B 완료로 origin Let's Encrypt cert 활성 = HTTPS 정상 동작 (단 WAF/Bot/캐시 사라짐). 복구 후 orange-cloud 복귀 |
| Pages 배포 실패 | Pages → Deployments → Retry deployment. 빌드 로그 확인 |
| DNS propagation 지연 | TTL 5분 → 짧게 변경 (1분) → 변경 후 재설정 |
| Email Routing 미수신 | Routes → 활성 상태 확인 + 외부 메일박스 도착 확인 (Cloudflare 측 + 외부 측 모두) |

### Cloudflare Free 플랜 한계 (참고)

- **CF Health Checks 미사용 가능** (Pro 이상) — 외부 모니터 = UptimeRobot 사용 (#71)
- **Page Rules 3개** — 현재 `_redirects` 파일로 SPA fallback / 301 처리
- **Workers 무료 100K req/day** — 현재 미사용

### 변경 이력 추적

본 SSoT 변경 시 git commit `docs(deploy): Cloudflare ...` 형식 + AMK_CHANGELOG entry 추가.

### Branch Protection 정책 (G8, 2026-05-07 가이드 정착 → 2026-05-08 ✅ 적용 완료)

> 1인 환경에서도 main 직접 push 사고 / 실수 force push / 실수 브랜치 삭제 방지. 적용 = GitHub 웹 UI (Settings → Branches → Add rule).
>
> **2026-05-08 적용 검증** (`gh api`): main = `Require PR (0 reviews) + Linear history + force/deletion 차단 + admin 우회 허용`. KKRYOUN = `Require PR OFF + Linear history OFF + Force push 허용 + Deletion 차단`.

#### main 브랜치 룰

| 항목 | 값 | 사유 |
|------|------|------|
| Require a pull request before merging | ✅ ON | KKRYOUN → PR 머지 패턴 강제. main direct push 차단 |
| Required approving reviews | **0** | 1인 환경 (셀프 머지 허용) |
| Dismiss stale reviews | OFF | 의미 없음 (review 0) |
| Require status checks | OFF (현재) → 향후 ON (`pr-check` job) | G9 PR 검사 워크플로 등록 후 |
| Require linear history | ✅ ON | merge commit 차단 (rebase / squash 만) |
| Require conversation resolution | OFF | 1인 환경 |
| Do not allow bypassing | OFF (admin 우회 허용) | 비상 시 안전망 |
| Allow force pushes | ❌ OFF | history 손실 방지 |
| Allow deletions | ❌ OFF | 실수 삭제 방지 |
| Lock branch | OFF | 정상 운영 |

#### KKRYOUN 브랜치 룰 (작업 브랜치)

| 항목 | 값 | 사유 |
|------|------|------|
| Require a pull request before merging | OFF | 작업 브랜치, direct push 자유 |
| Allow force pushes | ✅ ON | rebase / amend 허용 (`feedback_git_branching.md` 단일 브랜치 룰) |
| Allow deletions | ❌ OFF | 실수 삭제 방지 |

#### GitHub 웹 UI 적용 절차

1. https://github.com/AmazingKoreanCenter/amazing-korean-api/settings/branches
2. **Add branch protection rule** → Branch name pattern: `main` → 위 표 적용 → Create
3. **Add branch protection rule** → Branch name pattern: `KKRYOUN` → 위 표 적용 → Create
4. 적용 검증:
   ```bash
   gh api /repos/AmazingKoreanCenter/amazing-korean-api/branches/main/protection
   gh api /repos/AmazingKoreanCenter/amazing-korean-api/branches/KKRYOUN/protection
   ```
5. AMK_DEBTS G8 ✅ 해결 마킹

#### 적용 후 워크플로 영향

- `git push origin main` 직접 = 차단 (KKRYOUN → PR 강제)
- `git push --force origin main` = 차단
- `git push --force origin KKRYOUN` = 허용 (rebase 시 사용)
- `git branch -D main` (원격) = 차단

#### 비상 시 우회 (admin 본인 한정)

main 룰 우회 필요 시 (예: 긴급 hotfix):
1. Settings → Branches → main 룰 → "Do not allow bypassing" OFF (이미 default)
2. 또는 임시 룰 비활성: "Branch matches" 체크 해제 → 작업 → 원복

### 이메일 발송 SPF 정책 (A1-4, 2026-05-08 외부 검증 후 정정)

> **2026-05-08 정정**: 어제 (2026-05-07) 신설 본 섹션 = 호스트명 외부 검증 없이 작성 → 잘못된 SPF 호스트명 (`_spf.resend.com` = NXDOMAIN). 본 정정 = 실 DNS lookup + Resend SPF chain 추적 검증 후 정확한 표기.
> 한 도메인에 여러 메일 발송 서비스가 있을 때 **SPF TXT 레코드 1개에 모든 서비스 include 통합**. 표준상 SPF TXT 가 여러 개면 모두 무효 처리되므로 병합 필수.

#### 현재 실 DNS 상태 (2026-05-08 검증)

```
SPF:   v=spf1 include:_spf.mx.cloudflare.net ~all   (Cloudflare Email Routing 만)
MX:    route1/2/3.mx.cloudflare.net (Cloudflare Email Routing 활성)
DKIM:  resend._domainkey.amazingkorean.net ✅ 등록됨 (Resend 측)
DMARC: v=DMARC1; p=quarantine; rua=mailto:noreply@amazingkorean.net; pct=100
```

→ **현재 Resend 발송 메일 = SPF fail 이지만 DKIM pass 로 DMARC 통과 중** (relaxed alignment). 일부 엄격한 받는 측은 spam 처리 가능 → SPF 추가로 완전 정착 권장.

#### 발송 서비스별 SPF 필요성

| 서비스 | SPF 필요 | 사유 |
|---|:--:|---|
| **Resend** (인증/구독/이메일 인증코드, `@amazingkorean.net` From) | ✅ 필수 | Custom domain From 발송 = SPF + DKIM 둘 다 필요 |
| **Cloudflare Email Routing** (현재 active) | ✅ 필수 | 현재 정착 (`_spf.mx.cloudflare.net`) |
| **Paddle Customer Emails** (영수증/구독 알림) | ❌ 불필요 | Paddle 자체 도메인 (`@paddle.com`) 발송 = `amazingkorean.net` SPF 영향 없음 (Custom email domain 사용 시만 필요, 우리는 미사용) |

#### 정확한 SPF 변경 (실 작업)

| 상태 | 값 |
|---|---|
| **현재** | `v=spf1 include:_spf.mx.cloudflare.net ~all` |
| **변경 후** | `v=spf1 include:send.resend.com include:_spf.mx.cloudflare.net ~all` |

> Resend 공식 호스트명 = **`send.resend.com`** (실 DNS lookup 검증 = `v=spf1 include:amazonses.com ~all`. Resend = AWS SES 위에 빌드).

Cloudflare DNS → Records → 기존 SPF TXT 레코드 편집:

```
Type: TXT
Name: amazingkorean.net (또는 @)
Value: v=spf1 include:send.resend.com include:_spf.mx.cloudflare.net ~all
TTL: Auto
```

#### 검증 절차

본 리포 환경 (dig 미설치) = curl + Google DNS over HTTPS 사용:

```bash
# 1) DNS 적용 확인 (1-5분 propagation)
curl -s 'https://dns.google/resolve?name=amazingkorean.net&type=TXT' | python3 -c "import sys, json; [print(a['data']) for a in json.load(sys.stdin).get('Answer', []) if 'spf' in a.get('data', '').lower()]"
# 기대: v=spf1 include:send.resend.com include:_spf.mx.cloudflare.net ~all

# 2) 외부 SPF 검증 도구
# https://mxtoolbox.com/spf.aspx → SPF Valid + 0 errors
# https://www.kitterman.com/spf/validate.html → SPF Record Lookup

# 3) 실 메일 헤더 확인 (Resend 발송 → Gmail 수신)
# Authentication-Results 라인:
#   spf=pass smtp.mailfrom=amazingkorean.net   (이전: fail → 변경 후: pass)
#   dkim=pass header.d=amazingkorean.net       (이미 pass, 변경 없음)
#   dmarc=pass
```

#### 함정 / 사고 회피

| 패턴 | 영향 | 회피 |
|------|------|------|
| SPF TXT 레코드 2개 분리 (Resend 1개 + Cloudflare 1개) | RFC 7208 상 둘 다 invalid → 모든 메일 SPF fail | **반드시 1개로 병합** |
| `include:` 한도 10개 초과 | DNS lookup 한계 (RFC 7208) → permerror | 본 케이스 = `send.resend.com` (1) → `amazonses.com` (1) + `_spf.mx.cloudflare.net` (1-2) = ~3-4회. 안전 범위 |
| `~all` (soft fail) → `-all` (hard fail) 변경 | 강한 보호이지만 잘못 설정 시 정상 메일도 차단 | 현재 `~all` 권장 (DMARC `quarantine` 정책으로 보완). `-all` 은 검증 후 |
| Resend 호스트명 추측 (`_spf.resend.com` = NXDOMAIN) | SPF resolve 실패 → fail | 정확한 표기 = `send.resend.com` (실 DNS 검증됨, AWS SES chain) |
| Paddle SPF 추가 시도 (불필요) | 추가 lookup 비용만 발생 | Paddle Customer Emails = `@paddle.com` 발송 → SPF 영향 없음 |

#### 작업 흐름 (사용자, 5-10분) — 2026-05-08 ✅ 적용 완료

1. ~~Cloudflare 대시보드 → `amazingkorean.net` zone → DNS → Records~~ ✅
2. ~~기존 `v=spf1 include:_spf.mx.cloudflare.net ~all` SPF TXT 레코드 → Edit~~ ✅
3. ~~Value = `v=spf1 include:send.resend.com include:_spf.mx.cloudflare.net ~all` → Save~~ ✅
4. ~~1-5분 propagation → 검증~~ ✅ (Google DNS polling 으로 propagation 감지 + chain 검증 통과)
5. 외부 도구 검증 (mxtoolbox / kitterman) — 권장, 사용자 재량
6. 실 메일 테스트 (선택, 효과 확인)
7. ~~AMK_DEBTS A1-4 ✅ 해결 마킹~~ ✅

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

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

> Sandbox → Production 전환 체크리스트. **KYB/Onfido 승인 ✅ 완료 (2026-02-21~25 추정)**. 본 가이드는 §8.5 표 (18개 항목 모두 ✅) + "남은 작업" Step 3~6 (사용자 작업) 으로 구성.
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
