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
 * 시나리오: 로그인 → 비디오 목록 → 비디오 상세 → 학습 목록 → 학습 상세 → 토큰 갱신 → 로그아웃
 */
import http from "k6/http";
import { check, sleep } from "k6";
import { Trend } from "k6/metrics";
import { BASE_URL, TEST_USER, THRESHOLDS } from "./config.js";

const authDuration = new Trend("http_req_duration{endpoint:auth}");
const listDuration = new Trend("http_req_duration{endpoint:list}");
const detailDuration = new Trend("http_req_duration{endpoint:detail}");
const progressDuration = new Trend("http_req_duration{endpoint:progress}");

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

function authHeaders(token) {
  return { headers: { Authorization: `Bearer ${token}`, "Content-Type": "application/json" } };
}

export default function () {
  // 1. 로그인
  let accessToken, refreshToken;
  {
    const res = http.post(
      `${BASE_URL}/api/auth/login`,
      JSON.stringify({ login_id: TEST_USER.login_id, password: TEST_USER.password }),
      { headers: { "Content-Type": "application/json" } }
    );
    authDuration.add(res.timings.duration);
    if (res.status !== 200) return;
    const body = res.json();
    accessToken = body.access_token;
    refreshToken = body.refresh_token;
  }
  sleep(1);

  // 2. 비디오 목록
  let videoId;
  {
    const res = http.get(`${BASE_URL}/api/videos`, authHeaders(accessToken));
    listDuration.add(res.timings.duration);
    check(res, { "videos 200": (r) => r.status === 200 });
    const body = res.json();
    if (body.data && body.data.length > 0) {
      videoId = body.data[Math.floor(Math.random() * body.data.length)].video_id;
    }
  }
  sleep(0.5);

  // 3. 비디오 상세
  if (videoId) {
    const res = http.get(`${BASE_URL}/api/videos/${videoId}`, authHeaders(accessToken));
    detailDuration.add(res.timings.duration);
    check(res, { "video detail 200": (r) => r.status === 200 });
  }
  sleep(0.5);

  // 4. 학습 목록
  let studyId;
  {
    const res = http.get(`${BASE_URL}/api/studies`, authHeaders(accessToken));
    listDuration.add(res.timings.duration);
    check(res, { "studies 200": (r) => r.status === 200 });
    const body = res.json();
    if (body.data && body.data.length > 0) {
      studyId = body.data[Math.floor(Math.random() * body.data.length)].study_id;
    }
  }
  sleep(0.5);

  // 5. 학습 상세
  if (studyId) {
    const res = http.get(`${BASE_URL}/api/studies/${studyId}`, authHeaders(accessToken));
    detailDuration.add(res.timings.duration);
    check(res, { "study detail 200": (r) => r.status === 200 });
  }
  sleep(0.5);

  // 6. 레슨 목록
  {
    const res = http.get(`${BASE_URL}/api/lessons`, authHeaders(accessToken));
    listDuration.add(res.timings.duration);
    check(res, { "lessons 200": (r) => r.status === 200 });
  }
  sleep(0.5);

  // 7. 토큰 갱신
  if (refreshToken) {
    const res = http.post(
      `${BASE_URL}/api/auth/refresh`,
      JSON.stringify({ refresh_token: refreshToken }),
      { headers: { "Content-Type": "application/json" } }
    );
    authDuration.add(res.timings.duration);
    check(res, { "refresh 200": (r) => r.status === 200 });
  }
  sleep(0.5);

  // 8. 로그아웃
  {
    const res = http.post(`${BASE_URL}/api/auth/logout`, null, authHeaders(accessToken));
    authDuration.add(res.timings.duration);
  }
}
