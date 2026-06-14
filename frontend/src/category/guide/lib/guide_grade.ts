/**
 * 쓰기 채점 — 입력 정규화 후 완전일치 (해설집 HTML wtNormalize 이식, D-3).
 * 공백·문장부호 제거 후 비교. 점수화/유사도는 후속 디벨롭 (시도 로그는 백엔드 기록).
 */

const STRIP = /[?？!.。,，、\s]/g;

export function normalizeAnswer(s: string): string {
  // NFC: macOS/일부 IME 의 분해형(NFD) 한글 입력을 조합형으로 통일 → 채점 오답 방지
  return s.normalize("NFC").replace(STRIP, "");
}

/** 입력이 정답과 일치하는가 (정규화 후 완전일치) */
export function isCorrect(input: string, answer: string | null | undefined): boolean {
  if (!answer) return false;
  return normalizeAnswer(input) === normalizeAnswer(answer);
}

/** 입력이 정답의 접두인가 (타이핑 중 오답 표시 유보용 — neg-test 관용 채점 패턴) */
export function isPrefix(input: string, answer: string | null | undefined): boolean {
  if (!answer) return false;
  const a = normalizeAnswer(answer);
  const i = normalizeAnswer(input);
  return a.startsWith(i);
}
