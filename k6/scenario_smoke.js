/**
 * K6 스모크 테스트 — 단일 사용자, 기본 동선 확인
 *
 * 사용법:
 *   ~/.local/bin/k6 run k6/scenario_smoke.js
 *   K6_BASE_URL=https://api.amazingkorean.net ~/.local/bin/k6 run k6/scenario_smoke.js
 *
 * 대표 시나리오: 로그인 → 비디오 목록 → 비디오 상세 → 학습 목록 → 진도 저장 → 토큰 갱신 → 로그아웃
 *
 * Tagging 정책은 scenario_load.js 주석 참고.
 */
import http from "k6/http";
import { check, sleep } from "k6";
import { BASE_URL, TEST_USER, THRESHOLDS } from "./config.js";

export const options = {
  vus: 1,
  iterations: 1,
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
  // 1. 헬스체크
  {
    const res = http.get(`${BASE_URL}/health`, { tags: { endpoint: "detail" } });
    check(res, { "health 200": (r) => r.status === 200 });
  }

  // 2. 로그인
  let accessToken, refreshToken;
  {
    const res = http.post(
      `${BASE_URL}/api/auth/login`,
      JSON.stringify({ login_id: TEST_USER.login_id, password: TEST_USER.password }),
      jsonParams("auth")
    );
    const ok = check(res, { "login 200": (r) => r.status === 200 });
    if (ok) {
      const body = res.json();
      accessToken = body.access_token;
      refreshToken = body.refresh_token;
    } else {
      console.error(`Login failed: ${res.status} ${res.body}`);
      return;
    }
  }
  sleep(0.5);

  // 3. 비디오 목록
  let videoId;
  {
    const res = http.get(`${BASE_URL}/api/videos`, authParams(accessToken, "list"));
    check(res, { "videos list 200": (r) => r.status === 200 });
    const body = res.json();
    if (body.data && body.data.length > 0) {
      videoId = body.data[0].video_id;
    }
  }
  sleep(0.3);

  // 4. 비디오 상세
  if (videoId) {
    const res = http.get(`${BASE_URL}/api/videos/${videoId}`, authParams(accessToken, "detail"));
    check(res, { "video detail 200": (r) => r.status === 200 });
  }
  sleep(0.3);

  // 5. 학습 목록
  let studyId;
  {
    const res = http.get(`${BASE_URL}/api/studies`, authParams(accessToken, "list"));
    check(res, { "studies list 200": (r) => r.status === 200 });
    const body = res.json();
    if (body.data && body.data.length > 0) {
      studyId = body.data[0].study_id;
    }
  }
  sleep(0.3);

  // 6. 학습 상세 → task 진도 확인
  let taskId;
  if (studyId) {
    const res = http.get(`${BASE_URL}/api/studies/${studyId}`, authParams(accessToken, "detail"));
    check(res, { "study detail 200": (r) => r.status === 200 });
    const body = res.json();
    if (body.tasks && body.tasks.length > 0) {
      taskId = body.tasks[0].task_id;
    }
  }
  sleep(0.3);

  if (taskId) {
    const res = http.get(
      `${BASE_URL}/api/studies/tasks/${taskId}/status`,
      authParams(accessToken, "progress")
    );
    check(res, { "task status 200": (r) => r.status === 200 });
  }
  sleep(0.3);

  // 7. 토큰 갱신
  if (refreshToken) {
    const res = http.post(
      `${BASE_URL}/api/auth/refresh`,
      JSON.stringify({ refresh_token: refreshToken }),
      jsonParams("auth")
    );
    check(res, { "refresh 200": (r) => r.status === 200 });
    if (res.status === 200) {
      accessToken = res.json().access_token;
    }
  }
  sleep(0.3);

  // 8. 로그아웃
  {
    const res = http.post(`${BASE_URL}/api/auth/logout`, null, authParams(accessToken, "auth"));
    check(res, { "logout 200": (r) => r.status === 200 });
  }
}
