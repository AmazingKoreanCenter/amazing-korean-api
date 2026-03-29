import { useEffect, useRef } from "react";

/**
 * DevTools 감지 유틸리티 (E-book 뷰어 보안용)
 *
 * 감지 방법 3가지:
 * 1. debugger 문 타이밍 — DevTools 중단점 활성 시 100ms+ 소요
 * 2. 창 크기 변화 — DevTools 도킹 시 outer-inner 차이 200px+
 * 3. console.log getter — DevTools 콘솔 열림 시 toString() 호출됨
 *
 * 한계: 결정적(determined) 공격자는 우회 가능. 억제(deterrent) 목적.
 */

let consoleDetected = false;

/** console.log getter 기반 감지 (DevTools 콘솔 패널 열림 감지) */
function setupConsoleDetection() {
  const element = new Image();
  Object.defineProperty(element, "id", {
    get: () => {
      consoleDetected = true;
      return "";
    },
  });

  // 주기적으로 console.log 호출 — DevTools 콘솔이 열려있으면 getter 트리거
  setInterval(() => {
    consoleDetected = false;
    // eslint-disable-next-line no-console
    console.log("%c", element);
  }, 3000);
}

/** debugger 문 타이밍 기반 감지 */
function checkDebuggerTiming(): boolean {
  const start = performance.now();
  // eslint-disable-next-line no-debugger
  debugger;
  const elapsed = performance.now() - start;
  return elapsed > 100;
}

/** 창 크기 기반 감지 (도킹된 DevTools) */
function checkWindowSize(): boolean {
  const widthDiff = window.outerWidth - window.innerWidth;
  const heightDiff = window.outerHeight - window.innerHeight;
  return widthDiff > 200 || heightDiff > 200;
}

/** 3가지 방법 중 하나라도 감지되면 true */
export function isDevToolsOpen(): boolean {
  return consoleDetected || checkDebuggerTiming() || checkWindowSize();
}

/**
 * DevTools 감지 훅 — 2초 간격 폴링, 감지 시 콜백 호출
 * 3초 유예 후 콜백 실행 (오탐 방지: F12 실수 등)
 */
export function useDevToolsDetection(onDetected: () => void, onCleared?: () => void) {
  const detectedRef = useRef(false);
  const graceTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    setupConsoleDetection();

    const interval = setInterval(() => {
      const detected = checkWindowSize() || consoleDetected;

      if (detected && !detectedRef.current) {
        // 3초 유예 후 콜백 (일시적 오탐 방지)
        if (!graceTimerRef.current) {
          graceTimerRef.current = setTimeout(() => {
            if (checkWindowSize() || consoleDetected) {
              detectedRef.current = true;
              onDetected();
            }
            graceTimerRef.current = null;
          }, 3000);
        }
      } else if (!detected && detectedRef.current) {
        detectedRef.current = false;
        onCleared?.();
      }
    }, 2000);

    return () => {
      clearInterval(interval);
      if (graceTimerRef.current) clearTimeout(graceTimerRef.current);
    };
  }, [onDetected, onCleared]);
}
