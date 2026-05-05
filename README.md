# Amazing Korean API

온라인 한국어 학습 서비스의 풀스택 애플리케이션.

## 스택

- **백엔드**: Rust 2021 + Axum 0.8 + SQLx + PostgreSQL + Redis
- **프론트엔드**: React + Vite + TypeScript + shadcn/ui + TanStack Query
- **인증**: JWT 액세스/리프레시 토큰 + Google OAuth + Apple OAuth + MFA (TOTP)
- **암호화**: AES-256-GCM + HMAC-SHA256 Blind Index
- **이메일**: Resend
- **결제**: Paddle (구독) + RevenueCat (IAP)
- **배포**: AWS EC2 (백엔드) + Cloudflare Pages (프론트엔드)
- **도메인**: amazingkorean.net (프론트) / api.amazingkorean.net (백엔드)

## 디렉터리 구조

```
.
├── src/                  # 백엔드 (Rust)
│   ├── api/{domain}/     # dto → repo → service → handler → router
│   ├── config.rs         # 환경변수 SSoT
│   ├── error.rs          # 전역 에러 타입
│   ├── types.rs          # DB enum ↔ Rust enum
│   └── crypto/           # 암호화 유틸
├── crates/crypto/        # 암호화 crate (workspace member)
├── frontend/             # 프론트엔드 (React + Vite)
├── migrations/           # SQL 마이그레이션 (sqlx)
├── docs/                 # 프로젝트 문서 (private)
├── docker-compose.prod.yml
└── Dockerfile            # 멀티 스테이지 (build → runtime)
```

## 문서 (private)

- `CLAUDE.md` — Claude Code 작업 가이드
- `docs/AMK_API_MASTER.md` — 공통 규칙 + 시스템 환경 (SSoT Core)
- `docs/AMK_API_*.md` — 도메인별 API 스펙
- `docs/AMK_DEPLOY_OPS.md` — 배포 + 운영
- `docs/AMK_DEBTS.md` — 부채 카탈로그
- `docs/AMK_AUDIT_2026-05-04.md` — 정합성 조사 결과
- `docs/AMK_AI_MISTAKES.md` — AI 작업 사고 기록

## 개발

로컬 개발 시:

```bash
# 백엔드
cp .env.example .env
docker compose up -d           # postgres + redis 띄움
cargo run

# 프론트엔드
cd frontend
npm install
npm run dev
```

## 라이선스

Proprietary. See [LICENSE](LICENSE).

© 2026 HYMN Co., Ltd. (Amazing Korean Center).
