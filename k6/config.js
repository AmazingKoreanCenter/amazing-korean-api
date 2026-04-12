// K6 부하 테스트 공통 설정
// 사용법: k6 run k6/scenario_smoke.js  (로컬 스모크)
//        k6 run k6/scenario_load.js   (부하 테스트)

// 환경변수로 타겟 서버 변경 가능
// K6_BASE_URL=https://api.amazingkorean.net k6 run k6/scenario_load.js
export const BASE_URL = __ENV.K6_BASE_URL || "http://localhost:8080";

// 테스트용 계정 (로컬 개발 환경 전용)
export const TEST_USER = {
  login_id: __ENV.K6_LOGIN_ID || "k6-test@example.com",
  password: __ENV.K6_PASSWORD || "Test1234!@",
};

// 성능 목표 (AMK_STATUS.md §8.2)
export const THRESHOLDS = {
  // 인증 (login/refresh): P95 < 200ms, 100 RPS
  "http_req_duration{endpoint:auth}": ["p(95)<200"],
  // 목록 조회 (videos/studies): P95 < 100ms, 200 RPS
  "http_req_duration{endpoint:list}": ["p(95)<100"],
  // 상세 조회: P95 < 50ms, 300 RPS
  "http_req_duration{endpoint:detail}": ["p(95)<50"],
  // 진도 저장 (progress): P95 < 150ms, 100 RPS
  "http_req_duration{endpoint:progress}": ["p(95)<150"],
  // 전체
  http_req_failed: ["rate<0.01"], // 에러율 1% 미만
};
