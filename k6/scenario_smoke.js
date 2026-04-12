/**
 * K6 스모크 테스트 — 단일 사용자, 기본 동선 확인
 *
 * 사용법:
 *   ~/.local/bin/k6 run k6/scenario_smoke.js
 *   K6_BASE_URL=https://api.amazingkorean.net ~/.local/bin/k6 run k6/scenario_smoke.js
 *
 * 대표 시나리오: 로그인 → 비디오 목록 → 비디오 상세 → 학습 목록 → 토큰 갱신 → 로그아웃
 */
import http from "k6/http";
import { check, sleep } from "k6";
import { Trend } from "k6/metrics";
import { BASE_URL, TEST_USER, THRESHOLDS } from "./config.js";

// 커스텀 메트릭 (엔드포인트별 응답시간)
const authDuration = new Trend("http_req_duration{endpoint:auth}");
const listDuration = new Trend("http_req_duration{endpoint:list}");
const detailDuration = new Trend("http_req_duration{endpoint:detail}");

export const options = {
  vus: 1,
  iterations: 1,
  thresholds: THRESHOLDS,
};

function authHeaders(token) {
  return { headers: { Authorization: `Bearer ${token}`, "Content-Type": "application/json" } };
}

export default function () {
  // 1. 헬스체크
  {
    const res = http.get(`${BASE_URL}/health`);
    check(res, { "health 200": (r) => r.status === 200 });
  }

  // 2. 로그인
  let accessToken, refreshToken;
  {
    const res = http.post(
      `${BASE_URL}/api/auth/login`,
      JSON.stringify({ login_id: TEST_USER.login_id, password: TEST_USER.password }),
      { headers: { "Content-Type": "application/json" } }
    );
    authDuration.add(res.timings.duration);
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
    const res = http.get(`${BASE_URL}/api/videos`, authHeaders(accessToken));
    listDuration.add(res.timings.duration);
    check(res, { "videos list 200": (r) => r.status === 200 });
    const body = res.json();
    if (body.data && body.data.length > 0) {
      videoId = body.data[0].video_id;
    }
  }
  sleep(0.3);

  // 4. 비디오 상세
  if (videoId) {
    const res = http.get(`${BASE_URL}/api/videos/${videoId}`, authHeaders(accessToken));
    detailDuration.add(res.timings.duration);
    check(res, { "video detail 200": (r) => r.status === 200 });
  }
  sleep(0.3);

  // 5. 학습 목록
  {
    const res = http.get(`${BASE_URL}/api/studies`, authHeaders(accessToken));
    listDuration.add(res.timings.duration);
    check(res, { "studies list 200": (r) => r.status === 200 });
  }
  sleep(0.3);

  // 6. 토큰 갱신
  if (refreshToken) {
    const res = http.post(
      `${BASE_URL}/api/auth/refresh`,
      JSON.stringify({ refresh_token: refreshToken }),
      { headers: { "Content-Type": "application/json" } }
    );
    authDuration.add(res.timings.duration);
    check(res, { "refresh 200": (r) => r.status === 200 });
    if (res.status === 200) {
      accessToken = res.json().access_token;
    }
  }
  sleep(0.3);

  // 7. 로그아웃
  {
    const res = http.post(`${BASE_URL}/api/auth/logout`, null, authHeaders(accessToken));
    authDuration.add(res.timings.duration);
    check(res, { "logout 200": (r) => r.status === 200 });
  }
}
