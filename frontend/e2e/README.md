# Playwright E2E

P10-C 단계에서 추가된 한글 자판 연습 자유 연습 플로우 E2E 스위트.

## 구조

- `playwright.config.ts` — testDir `./e2e`, outputDir `./test-results/e2e`, 1 worker, chromium 단독
- `e2e/fixtures/auth.ts` — `apiLogin` (REST 로그인) + `seedAuthStorage` (zustand persist 흉내로 localStorage 주입)
- `e2e/writing_practice.spec.ts` — 레벨 선택 → 유형 선택 → 자유 연습 1회 완료 → `GET /studies/writing/stats` 기준선 대비 `total_sessions` 증가 검증

## 전제 조건

로컬 backend + frontend dev 서버가 이미 떠 있어야 한다. Playwright 는 자체적으로 `webServer` 를 띄우지 않는다.

- backend: `cargo run` (혹은 디버그 바이너리). 포트는 기본 `0.0.0.0:3000` 이지만 다른 프로젝트(예: jevy-api) 와 충돌하면 `BIND_ADDR=127.0.0.1:3100` 로 우회 가능.
  - **E2E 셋업 시 1회에 한해** `EMAIL_PROVIDER=none` 으로 띄워서 테스트 계정을 자동 인증 상태로 생성한다.
  - production 빌드에서 `EMAIL_PROVIDER=none + APP_ENV=production` 은 부팅 거부 (AMK 안전장치).
- frontend: `npm run dev`. 백엔드 포트를 3000 이외로 띄웠다면 `VITE_PROXY_TARGET=http://127.0.0.1:3100 npm run dev` 처럼 환경 변수로 프록시 타겟 override.

## 테스트 계정 생성 (최초 1회)

`e2e_p10c@amazingkorean.net / password123!` 계정이 로컬 DB 에 필요하다.

```bash
# backend 를 EMAIL_PROVIDER=none 로 띄운 상태에서
curl -sS -X POST http://127.0.0.1:3100/users \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "e2e_p10c@amazingkorean.net",
    "password": "password123!",
    "name": "E2E Tester",
    "nickname": "e2e_p10c",
    "terms_service": true,
    "terms_personal": true,
    "language": "ko",
    "country": "KR",
    "birthday": "2000-01-01",
    "gender": "none"
  }'
```

`EMAIL_PROVIDER=none` 이면 서버가 자동으로 email_verified=true 로 세팅한다. 이후에는 `EMAIL_PROVIDER=resend` 로 돌려도 이 계정은 살아있다.

## 실행

```bash
cd frontend
npm run test:e2e
```

환경 변수 override:

- `E2E_BASE_URL` — 기본 `http://localhost:5173`
- `E2E_TEST_EMAIL` / `E2E_TEST_PASSWORD` — 기본 `e2e_p10c@amazingkorean.net` / `password123!`

## 레이트 리미트 걸렸을 때

로그인 레이트 리미트(`RATE_LIMIT_LOGIN_MAX=10` / `WINDOW=900`) 에 걸리면 Redis 키를 직접 지운다:

```bash
docker exec amk-redis redis-cli -a redis_dev_password --no-auth-warning \
  KEYS 'rl:login:*' | xargs -r docker exec -i amk-redis \
  redis-cli -a redis_dev_password --no-auth-warning DEL
```
