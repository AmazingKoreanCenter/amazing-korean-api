/**
 * K6 부하 테스트 — 단계별 부하 증가 (Ramp-up)
 *
 * 사용법:
 *   ~/.local/bin/k6 run k6/scenario_load.js
 *   K6_BASE_URL=https://api.amazingkorean.net ~/.local/bin/k6 run k6/scenario_load.js
 *
 * 목표 (AMK_STATUS.md §8.2):
 *   인증: 100 RPS, P95 < 200ms
 *   목록 조회: 200 RPS, P95 < 100ms
 *   상세 조회: 300 RPS, P95 < 50ms
 *   진도 저장: 100 RPS, P95 < 150ms
 *
 * 시나리오: 로그인 → 비디오 목록 → 비디오 상세 → 학습 목록 → 학습 상세 → 진도 저장 → 토큰 갱신 → 로그아웃
 *
 * Tagging 정책: http.* 호출에 `tags: { endpoint: ... }` 부여 → config.js 의
 * `http_req_duration{endpoint:auth}` 형식 thresholds 가 기본 metric 에 tag 필터로
 * 매칭된다. 커스텀 `Trend` 를 만들면 동일 이름의 별개 metric 이 생성되어
 * thresholds 가 데이터를 못 보게 되므로 절대 금지.
 */
import http from "k6/http";
import { check, sleep } from "k6";
import { BASE_URL, TEST_USER, THRESHOLDS } from "./config.js";

export const options = {
  stages: [
    { duration: "30s", target: 10 },  // 워밍업
    { duration: "1m", target: 50 },   // 중간 부하
    { duration: "2m", target: 100 },  // 목표 부하
    { duration: "1m", target: 100 },  // 유지
    { duration: "30s", target: 0 },   // 쿨다운
  ],
  thresholds: THRESHOLDS,
};

function authParams(token, endpoint) {
  return {
    headers: { Authorization: `Bearer ${token}`, "Content-Type": "application/json" },
    tags: { endpoint },
  };
}

function jsonParams(endpoint) {
  return {
    headers: { "Content-Type": "application/json" },
    tags: { endpoint },
  };
}

export default function () {
  // 1. 로그인
  let accessToken, refreshToken;
  {
    const res = http.post(
      `${BASE_URL}/api/auth/login`,
      JSON.stringify({ login_id: TEST_USER.login_id, password: TEST_USER.password }),
      jsonParams("auth")
    );
    if (res.status !== 200) return;
    const body = res.json();
    accessToken = body.access_token;
    refreshToken = body.refresh_token;
  }
  sleep(1);

  // 2. 비디오 목록
  let videoId;
  {
    const res = http.get(`${BASE_URL}/api/videos`, authParams(accessToken, "list"));
    check(res, { "videos 200": (r) => r.status === 200 });
    const body = res.json();
    if (body.data && body.data.length > 0) {
      videoId = body.data[Math.floor(Math.random() * body.data.length)].video_id;
    }
  }
  sleep(0.5);

  // 3. 비디오 상세
  if (videoId) {
    const res = http.get(`${BASE_URL}/api/videos/${videoId}`, authParams(accessToken, "detail"));
    check(res, { "video detail 200": (r) => r.status === 200 });
  }
  sleep(0.5);

  // 4. 학습 목록
  let studyId;
  {
    const res = http.get(`${BASE_URL}/api/studies`, authParams(accessToken, "list"));
    check(res, { "studies 200": (r) => r.status === 200 });
    const body = res.json();
    if (body.data && body.data.length > 0) {
      studyId = body.data[Math.floor(Math.random() * body.data.length)].study_id;
    }
  }
  sleep(0.5);

  // 5. 학습 상세 + task 목록
  let taskId;
  if (studyId) {
    const res = http.get(`${BASE_URL}/api/studies/${studyId}`, authParams(accessToken, "detail"));
    check(res, { "study detail 200": (r) => r.status === 200 });
    const body = res.json();
    if (body.tasks && body.tasks.length > 0) {
      taskId = body.tasks[Math.floor(Math.random() * body.tasks.length)].task_id;
    }
  }
  sleep(0.5);

  // 6. 진도 확인 (task status — 사용자 학습 진도 조회 대표 엔드포인트)
  if (taskId) {
    const res = http.get(
      `${BASE_URL}/api/studies/tasks/${taskId}/status`,
      authParams(accessToken, "progress")
    );
    check(res, { "task status 200": (r) => r.status === 200 });
  }
  sleep(0.5);

  // 7. 레슨 목록
  {
    const res = http.get(`${BASE_URL}/api/lessons`, authParams(accessToken, "list"));
    check(res, { "lessons 200": (r) => r.status === 200 });
  }
  sleep(0.5);

  // 8. 토큰 갱신
  if (refreshToken) {
    const res = http.post(
      `${BASE_URL}/api/auth/refresh`,
      JSON.stringify({ refresh_token: refreshToken }),
      jsonParams("auth")
    );
    check(res, { "refresh 200": (r) => r.status === 200 });
  }
  sleep(0.5);

  // 9. 로그아웃
  {
    http.post(`${BASE_URL}/api/auth/logout`, null, authParams(accessToken, "auth"));
  }
}
