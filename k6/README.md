# K6 부하 테스트

## 설치

```bash
# 이미 설치됨: ~/.local/bin/k6 (v0.56.0)
k6 version
```

## 사용법

### 스모크 테스트 (기본 동선 확인)
```bash
# 로컬 백엔드
k6 run k6/scenario_smoke.js

# 프로덕션
K6_BASE_URL=https://api.amazingkorean.net k6 run k6/scenario_smoke.js
```

### 부하 테스트 (단계별 증가)
```bash
# 테스트 계정 지정
K6_LOGIN_ID=test@example.com K6_PASSWORD=Test1234 k6 run k6/scenario_load.js
```

## 사전 준비

1. 테스트용 계정 생성 (로컬 DB)
2. 로컬 백엔드 실행: `cargo run`
3. 환경변수 `K6_LOGIN_ID`, `K6_PASSWORD` 설정

## 성능 목표 (AMK_STATUS.md §8.2)

| 엔드포인트 | 목표 RPS | P95 응답시간 |
|----------|---------|-------------|
| 인증 (login/refresh) | 100 | < 200ms |
| 목록 조회 (videos/studies) | 200 | < 100ms |
| 상세 조회 | 300 | < 50ms |
| 진도 저장 (progress) | 100 | < 150ms |

## 파일 구조

```
k6/
├── config.js            # 공통 설정 (BASE_URL, 계정, 임계값)
├── scenario_smoke.js    # 스모크 (VU=1, 1회)
├── scenario_load.js     # 부하 (10→50→100 VU, 5분)
└── README.md
```
