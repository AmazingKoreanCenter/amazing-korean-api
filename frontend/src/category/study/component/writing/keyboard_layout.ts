// 2벌식 (Dubeolsik) 한글 자판 레이아웃
// 표준 KS X 5002 배열 기준. 숫자 행은 타이핑 연습 범위에서 제외.

export interface KeyCap {
  code: string;
  english: string;
  base: string;
  shift?: string;
}

export const DUBEOLSIK_ROWS: KeyCap[][] = [
  [
    { code: "KeyQ", english: "q", base: "ㅂ", shift: "ㅃ" },
    { code: "KeyW", english: "w", base: "ㅈ", shift: "ㅉ" },
    { code: "KeyE", english: "e", base: "ㄷ", shift: "ㄸ" },
    { code: "KeyR", english: "r", base: "ㄱ", shift: "ㄲ" },
    { code: "KeyT", english: "t", base: "ㅅ", shift: "ㅆ" },
    { code: "KeyY", english: "y", base: "ㅛ" },
    { code: "KeyU", english: "u", base: "ㅕ" },
    { code: "KeyI", english: "i", base: "ㅑ" },
    { code: "KeyO", english: "o", base: "ㅐ", shift: "ㅒ" },
    { code: "KeyP", english: "p", base: "ㅔ", shift: "ㅖ" },
  ],
  [
    { code: "KeyA", english: "a", base: "ㅁ" },
    { code: "KeyS", english: "s", base: "ㄴ" },
    { code: "KeyD", english: "d", base: "ㅇ" },
    { code: "KeyF", english: "f", base: "ㄹ" },
    { code: "KeyG", english: "g", base: "ㅎ" },
    { code: "KeyH", english: "h", base: "ㅗ" },
    { code: "KeyJ", english: "j", base: "ㅓ" },
    { code: "KeyK", english: "k", base: "ㅏ" },
    { code: "KeyL", english: "l", base: "ㅣ" },
  ],
  [
    { code: "KeyZ", english: "z", base: "ㅋ" },
    { code: "KeyX", english: "x", base: "ㅌ" },
    { code: "KeyC", english: "c", base: "ㅊ" },
    { code: "KeyV", english: "v", base: "ㅍ" },
    { code: "KeyB", english: "b", base: "ㅠ" },
    { code: "KeyN", english: "n", base: "ㅜ" },
    { code: "KeyM", english: "m", base: "ㅡ" },
  ],
];

const JAMO_TO_KEY: Record<string, { cap: KeyCap; needsShift: boolean }> = (() => {
  const map: Record<string, { cap: KeyCap; needsShift: boolean }> = {};
  for (const row of DUBEOLSIK_ROWS) {
    for (const cap of row) {
      map[cap.base] = { cap, needsShift: false };
      if (cap.shift) map[cap.shift] = { cap, needsShift: true };
    }
  }
  return map;
})();

export function findKeyForJamo(jamo: string): { cap: KeyCap; needsShift: boolean } | null {
  return JAMO_TO_KEY[jamo] ?? null;
}

// 한글 음절(가~힣) → 초/중/종성 자모 배열
// 겹자모(ㄲ, ㄳ, ㅘ 등)는 단일 자모로 반환 (실제 타이핑 시 분해는 호출부 책임).
const CHO: readonly string[] = [
  "ㄱ", "ㄲ", "ㄴ", "ㄷ", "ㄸ", "ㄹ", "ㅁ", "ㅂ", "ㅃ", "ㅅ",
  "ㅆ", "ㅇ", "ㅈ", "ㅉ", "ㅊ", "ㅋ", "ㅌ", "ㅍ", "ㅎ",
];

const JUNG: readonly string[] = [
  "ㅏ", "ㅐ", "ㅑ", "ㅒ", "ㅓ", "ㅔ", "ㅕ", "ㅖ", "ㅗ", "ㅘ",
  "ㅙ", "ㅚ", "ㅛ", "ㅜ", "ㅝ", "ㅞ", "ㅟ", "ㅠ", "ㅡ", "ㅢ", "ㅣ",
];

const JONG: readonly string[] = [
  "", "ㄱ", "ㄲ", "ㄳ", "ㄴ", "ㄵ", "ㄶ", "ㄷ", "ㄹ", "ㄺ",
  "ㄻ", "ㄼ", "ㄽ", "ㄾ", "ㄿ", "ㅀ", "ㅁ", "ㅂ", "ㅄ", "ㅅ",
  "ㅆ", "ㅇ", "ㅈ", "ㅊ", "ㅋ", "ㅌ", "ㅍ", "ㅎ",
];

const HANGUL_BASE = 0xac00;
const HANGUL_LAST = 0xd7a3;

export function decomposeSyllable(char: string): string[] {
  if (!char) return [];
  const code = char.codePointAt(0);
  if (code === undefined) return [];
  if (code < HANGUL_BASE || code > HANGUL_LAST) return [char];

  const index = code - HANGUL_BASE;
  const cho = Math.floor(index / (21 * 28));
  const jung = Math.floor((index % (21 * 28)) / 28);
  const jong = index % 28;

  const result = [CHO[cho], JUNG[jung]];
  if (jong !== 0) result.push(JONG[jong]);
  return result;
}
