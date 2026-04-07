import { useCallback, useEffect, useRef } from "react";

/**
 * DevTools 감지 유틸리티 (E-book 뷰어 보안용)
 *
 * 감지 방법 2가지 (주력 + 보조):
 * 1. [주력] console.log getter — DevTools 콘솔 열림 시 toString() 호출됨
 * 2. [보조] 창 크기 변화 — DevTools 도킹 시 outer-inner 차이 (보조 신호로만 사용, 오탐 감소)
 *
 * 한계: 결정적(determined) 공격자는 우회 가능. 억제(deterrent) 목적.
 */

let consoleDetected = false;
let consoleIntervalId: ReturnType<typeof setInterval> | null = null;

/** console.log getter 기반 감지 (DevTools 콘솔 패널 열림 감지) */
function setupConsoleDetection(): void {
  // 중복 호출 방지 (React Strict Mode 대응)
  if (consoleIntervalId !== null) return;

  const element = new Image();
  Object.defineProperty(element, "id", {
    get: () => {
      consoleDetected = true;
      return "";
    },
  });

  consoleIntervalId = setInterval(() => {
    consoleDetected = false;
    // eslint-disable-next-line no-console
    console.log("%c", element);
  }, 3000);
}

/** console 감지 interval 정리 */
function cleanupConsoleDetection(): void {
  if (consoleIntervalId !== null) {
    clearInterval(consoleIntervalId);
    consoleIntervalId = null;
  }
  consoleDetected = false;
}

/** 창 크기 기반 감지 (도킹된 DevTools) — 보조 신호 */
function checkWindowSize(): boolean {
  const widthDiff = window.outerWidth - window.innerWidth;
  const heightDiff = window.outerHeight - window.innerHeight;
  // 300px 임계값 (브라우저 확장 프로그램 오탐 감소)
  return widthDiff > 300 || heightDiff > 300;
}

/**
 * DevTools 감지 훅 — 2초 간격 폴링, 감지 시 콜백 호출
 * console getter를 주력, 창 크기를 보조로 사용
 * 3초 유예 후 콜백 실행 (오탐 방지: F12 실수 등)
 */
export function useDevToolsDetection(onDetected: () => void, onCleared?: () => void) {
  const detectedRef = useRef(false);
  const graceTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const stableOnDetected = useCallback(onDetected, [onDetected]);
  const stableOnCleared = useCallback(() => onCleared?.(), [onCleared]);

  useEffect(() => {
    setupConsoleDetection();

    const interval = setInterval(() => {
      // console getter + 창 크기 둘 다 true일 때만 확정 (오탐 방지)
      const detected = consoleDetected && checkWindowSize();

      if (detected && !detectedRef.current) {
        if (!graceTimerRef.current) {
          graceTimerRef.current = setTimeout(() => {
            if (consoleDetected) {
              detectedRef.current = true;
              stableOnDetected();
            }
            graceTimerRef.current = null;
          }, 3000);
        }
      } else if (!detected && detectedRef.current) {
        detectedRef.current = false;
        stableOnCleared();
      }
    }, 2000);

    return () => {
      clearInterval(interval);
      cleanupConsoleDetection();
      if (graceTimerRef.current) clearTimeout(graceTimerRef.current);
    };
  }, [stableOnDetected, stableOnCleared]);
}
