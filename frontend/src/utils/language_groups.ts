/**
 * 타이포그래피 특성별 언어 그룹 분류.
 *
 * - CJK: word-break: keep-all (어절 단위 줄바꿈)
 * - Tall Script: line-height 1.8 (성조 부호/모음 마크 위아래 확장)
 * - Relaxed Tracking: letter-spacing 0 (복합 자모/상하 결합 스크립트)
 */

const CJK = new Set(["ko", "ja", "zh-CN", "zh-TW"]);
const TALL_SCRIPT = new Set(["th", "my", "km"]);
const RELAXED_TRACKING = new Set(["th", "my", "km", "si", "hi", "ne", "mn"]);

export const isCJK = (lang: string) => CJK.has(lang);
export const isTallScript = (lang: string) => TALL_SCRIPT.has(lang);
export const needsRelaxedTracking = (lang: string) => RELAXED_TRACKING.has(lang);

export const LANG_CLASSES = [
  "lang-cjk",
  "lang-tall-script",
  "lang-relaxed-tracking",
] as const;
