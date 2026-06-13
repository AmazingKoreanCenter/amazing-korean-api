/**
 * 쓰기 채점 — 입력 정규화 후 완전일치 (해설집 HTML wtNormalize 이식, D-3).
 * 공백·문장부호 제거 후 비교. 점수화/유사도는 후속 디벨롭 (시도 로그는 백엔드 기록).
 */

const STRIP = /[?？!.。,，、\s]/g;

export function normalizeAnswer(s: string): string {
  return s.replace(STRIP, "");
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
