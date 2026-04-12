// Lighthouse 베이스라인 측정 스크립트.
//
// 사용법:
//   cd frontend
//   node perf-audit/audit.mjs <label>
//
// label 예: baseline-pre, baseline-post-quickwin, after-route-split.
// label은 영소문자, 숫자, 하이픈만 허용 (artifacts 디렉터리명으로 사용).
//
// 동작:
//   1) `vite preview`를 child process로 백그라운드 기동 (port 4173)
//   2) Playwright Chromium을 lighthouse가 사용
//   3) AUDIT_PAGES를 순회하며 lighthouse 실행, JSON + 요약 저장
//   4) preview 종료 + chrome 종료
//
// 출력:
//   perf-audit/artifacts/<label>/<slug>.json   (전체 lighthouse 리포트)
//   perf-audit/artifacts/<label>/_summary.json (페이지별 점수/지표 요약)
//   콘솔에 비교용 표

import { spawn, execSync } from "node:child_process";
import { writeFileSync, mkdirSync, existsSync, readdirSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { homedir } from "node:os";
import lighthouse from "lighthouse";
import * as chromeLauncher from "chrome-launcher";
import { AUDIT_PAGES } from "./pages.mjs";

const __dirname = dirname(fileURLToPath(import.meta.url));
const FRONTEND_ROOT = resolve(__dirname, "..");
const ARTIFACTS_DIR = resolve(__dirname, "artifacts");
const PREVIEW_PORT = 4173;
const PREVIEW_URL = `http://localhost:${PREVIEW_PORT}`;

// --- (#5) LABEL 입력값 검증 ---
const LABEL = process.argv[2];
if (!LABEL) {
  console.error("Usage: node perf-audit/audit.mjs <label>");
  console.error("Example: node perf-audit/audit.mjs baseline-pre");
  process.exit(1);
}
if (!/^[a-z0-9][a-z0-9\-]*$/.test(LABEL)) {
  console.error(`Invalid label "${LABEL}". Only lowercase alphanumeric + hyphens allowed.`);
  process.exit(1);
}

// --- (#1) Playwright Chromium 동적 탐색 ---
// Playwright가 설치한 Chromium을 자동 탐색. 버전/경로 변경에도 대응.
function findPlaywrightChrome() {
  const msPlaywright = join(homedir(), ".cache", "ms-playwright");
  if (!existsSync(msPlaywright)) return null;

  // chromium-XXXX 디렉터리 중 가장 최신(숫자 큰 것) 선택
  const dirs = readdirSync(msPlaywright)
    .filter((d) => d.startsWith("chromium-"))
    .sort()
    .reverse();

  for (const dir of dirs) {
    const candidate = join(msPlaywright, dir, "chrome-linux64", "chrome");
    if (existsSync(candidate)) return candidate;
  }
  return null;
}

const CHROME_PATH = process.env.CHROME_PATH || findPlaywrightChrome();

if (!CHROME_PATH || !existsSync(CHROME_PATH)) {
  console.error(`Chrome binary not found.`);
  console.error("Options:");
  console.error("  1) Set CHROME_PATH env var: CHROME_PATH=/usr/bin/chromium node perf-audit/audit.mjs <label>");
  console.error("  2) Install Playwright chromium: npx playwright install chromium");
  process.exit(1);
}

// --- (#3) Preview server 관리 (detached spawn + process group kill) ---
function startPreview() {
  return new Promise((resolveStart, rejectStart) => {
    const proc = spawn(
      "npm",
      ["run", "preview", "--", "--port", String(PREVIEW_PORT), "--strictPort"],
      {
        cwd: FRONTEND_ROOT,
        stdio: ["ignore", "pipe", "pipe"],
        env: { ...process.env, BROWSER: "none" },
        detached: true, // process group으로 관리해 하위 프로세스(vite) 포함 종료 가능
      },
    );

    let resolved = false;
    let stderrBuf = "";

    const onReady = () => {
      if (!resolved) {
        resolved = true;
        setTimeout(() => resolveStart(proc), 800);
      }
    };

    proc.stdout.on("data", (data) => {
      const str = data.toString();
      if (str.includes(`localhost:${PREVIEW_PORT}`) || /Local:\s+http/.test(str)) {
        onReady();
      }
    });
    proc.stderr.on("data", (data) => {
      stderrBuf += data.toString();
      if (stderrBuf.includes(`localhost:${PREVIEW_PORT}`) || /Local:\s+http/.test(stderrBuf)) {
        onReady();
      }
    });

    proc.on("error", rejectStart);
    proc.on("exit", (code) => {
      if (!resolved) {
        rejectStart(new Error(`Preview exited early (code ${code})\n${stderrBuf}`));
      }
    });

    // detached unref — 부모 종료 시 자식도 종료되도록 cleanup에서 명시적 kill
    proc.unref();

    setTimeout(() => {
      if (!resolved) rejectStart(new Error("Preview server timeout (30s)"));
    }, 30000);
  });
}

function killPreview(proc) {
  if (!proc || proc.exitCode !== null) return;
  try {
    // process group kill (-pid) — npm + vite 하위 프로세스 모두 종료
    process.kill(-proc.pid, "SIGTERM");
  } catch {
    // 이미 종료된 경우 무시
  }
}

async function auditOne(url, chromePort) {
  // 모든 카테고리 측정 (Performance/Accessibility/Best Practices/SEO)
  const result = await lighthouse(url, {
    port: chromePort,
    output: "json",
    logLevel: "error",
    onlyCategories: ["performance", "accessibility", "best-practices", "seo"],
  });
  // (#11) lighthouse result 유효성 검증
  if (!result || !result.report) {
    throw new Error(`Lighthouse returned no report for ${url}`);
  }
  return result;
}

function extractScores(json) {
  const cat = json.categories;
  const a = json.audits;
  return {
    performance: Math.round((cat.performance?.score ?? 0) * 100),
    accessibility: Math.round((cat.accessibility?.score ?? 0) * 100),
    bestPractices: Math.round((cat["best-practices"]?.score ?? 0) * 100),
    seo: Math.round((cat.seo?.score ?? 0) * 100),
    fcp: Math.round(a["first-contentful-paint"]?.numericValue ?? 0),
    lcp: Math.round(a["largest-contentful-paint"]?.numericValue ?? 0),
    tbt: Math.round(a["total-blocking-time"]?.numericValue ?? 0),
    cls: +(a["cumulative-layout-shift"]?.numericValue ?? 0).toFixed(3),
    si: Math.round(a["speed-index"]?.numericValue ?? 0),
    tti: Math.round(a["interactive"]?.numericValue ?? 0),
  };
}

function pad(s, n) {
  return String(s).padEnd(n);
}
function padStart(s, n) {
  return String(s).padStart(n);
}

function printSummary(summary) {
  console.log("");
  console.log(
    "Page                 | Perf | A11y | BP   | SEO  | FCP    | LCP    | TBT    | CLS   | SI",
  );
  console.log(
    "---------------------|------|------|------|------|--------|--------|--------|-------|--------",
  );
  for (const row of summary) {
    console.log(
      `${pad(row.slug, 20)} | ${padStart(row.performance, 4)} | ${padStart(
        row.accessibility,
        4,
      )} | ${padStart(row.bestPractices, 4)} | ${padStart(row.seo, 4)} | ${padStart(
        row.fcp + "ms",
        6,
      )} | ${padStart(row.lcp + "ms", 6)} | ${padStart(row.tbt + "ms", 6)} | ${padStart(
        row.cls.toFixed(3),
        5,
      )} | ${padStart(row.si + "ms", 6)}`,
    );
  }
  console.log("");
}

async function main() {
  // (#5 continued) artifacts 디렉터리 생성
  const labelDir = resolve(ARTIFACTS_DIR, LABEL);
  try {
    mkdirSync(labelDir, { recursive: true });
  } catch (err) {
    console.error(`Failed to create artifacts directory: ${labelDir}`, err.message);
    process.exit(1);
  }
  console.log(`[perf-audit] label=${LABEL}, artifacts=${labelDir}`);

  console.log(`[perf-audit] starting vite preview on :${PREVIEW_PORT}...`);
  const preview = await startPreview();
  console.log(`[perf-audit] preview ready`);

  // (#4) userDataDir를 /tmp로 지정해 프로젝트 디렉터리 오염 방지.
  // (#13) --headless=new: Chromium 112+ 최신 헤드리스 모드. lighthouse 호환성 최적.
  //       구형 Chrome에서는 --headless 로 대체.
  const userDataDir = `/tmp/lighthouse-profile-${Date.now()}`;
  console.log(`[perf-audit] launching chrome from ${CHROME_PATH}`);
  const chrome = await chromeLauncher.launch({
    chromePath: CHROME_PATH,
    chromeFlags: [
      "--headless=new",
      "--no-sandbox",
      "--disable-dev-shm-usage",
      "--disable-gpu",
    ],
    userDataDir,
  });

  const summary = [];
  let failed = 0;
  try {
    for (const page of AUDIT_PAGES) {
      const url = `${PREVIEW_URL}${page.path}`;
      process.stdout.write(`[perf-audit] auditing ${page.slug} (${page.path}) ... `);
      try {
        const result = await auditOne(url, chrome.port);
        const json = JSON.parse(result.report);
        writeFileSync(resolve(labelDir, `${page.slug}.json`), result.report);
        const scores = extractScores(json);
        summary.push({ slug: page.slug, group: page.group, path: page.path, ...scores });
        console.log(
          `P:${scores.performance} A:${scores.accessibility} BP:${scores.bestPractices} SEO:${scores.seo}`,
        );
      } catch (err) {
        failed++;
        console.log(`FAIL: ${err.message}`);
        summary.push({ slug: page.slug, group: page.group, path: page.path, error: err.message });
      }
    }
  } finally {
    // (#2) chrome.kill() 에러 핸들링
    try {
      await chrome.kill();
    } catch {
      // Chrome이 이미 종료된 경우 무시
    }
    // (#3) preview process group 종료
    killPreview(preview);
  }

  writeFileSync(
    resolve(labelDir, "_summary.json"),
    JSON.stringify({ label: LABEL, capturedAt: new Date().toISOString(), pages: summary }, null, 2),
  );

  printSummary(summary.filter((r) => !r.error));
  console.log(
    `[perf-audit] done. ${summary.length - failed} ok, ${failed} failed. summary saved to ${labelDir}/_summary.json`,
  );

  // (#3) 정상 종료 보장 — cleanup 후 명시적 exit
  process.exit(failed > 0 ? 1 : 0);
}

main().catch((err) => {
  console.error("[perf-audit] fatal:", err);
  process.exit(1);
});
