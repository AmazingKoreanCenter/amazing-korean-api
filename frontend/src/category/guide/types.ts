import { z } from "zod";

/**
 * guide(온라인 콘텐츠/해설집) 서빙 모델.
 * 백엔드 `src/api/guide/dto.rs` 대응. 표는 서버가 격자로 재조립(D-7)해 내려주고,
 * 문장은 학습항목(복습·채점 데이터 원천)으로 분리돼 온다.
 * text = 표시 언어 해소(번역→en→ko 폴백), text_ko = 언어불변 한국어 학습 콘텐츠.
 */

export const guideSummarySchema = z.object({
  guide_idx: z.string(),
  guide_seq: z.number(),
  guide_category: z.string(),
  /** 교재 10색 테마 */
  guide_theme: z.string(),
  sentence_start: z.number().nullable().optional(),
  sentence_end: z.number().nullable().optional(),
  title: z.string().nullable().optional(),
  title_ko: z.string().nullable().optional(),
  subtitle: z.string().nullable().optional(),
  subtitle_ko: z.string().nullable().optional(),
});

export const guideListSchema = z.object({
  items: z.array(guideSummarySchema),
  lang: z.string(),
});

export const guideCellSchema = z.object({
  text: z.string().nullable().optional(),
  text_ko: z.string().nullable().optional(),
  marker: z.string().nullable().optional(),
  header: z.boolean(),
  col_span: z.number().nullable().optional(),
  row_span: z.number().nullable().optional(),
});

export const guideItemSchema = z.object({
  /** "block" | "table" */
  kind: z.string(),
  block_seq: z.number(),
  sentence_no: z.number().nullable().optional(),
  block_type: z.string().nullable().optional(),
  text: z.string().nullable().optional(),
  text_ko: z.string().nullable().optional(),
  marker: z.string().nullable().optional(),
  table_no: z.number().nullable().optional(),
  rows: z.array(z.array(guideCellSchema)).nullable().optional(),
});

export const guideSentenceSchema = z.object({
  sentence_no: z.number(),
  text_ko: z.string().nullable().optional(),
  text: z.string().nullable().optional(),
  pron_ko: z.string().nullable().optional(),
  audio_url: z.string().nullable().optional(),
});

export const guideDetailSchema = z.object({
  guide_idx: z.string(),
  guide_seq: z.number(),
  guide_category: z.string(),
  guide_theme: z.string(),
  sentence_start: z.number().nullable().optional(),
  sentence_end: z.number().nullable().optional(),
  title: z.string().nullable().optional(),
  title_ko: z.string().nullable().optional(),
  subtitle: z.string().nullable().optional(),
  subtitle_ko: z.string().nullable().optional(),
  lang: z.string(),
  items: z.array(guideItemSchema),
  sentences: z.array(guideSentenceSchema),
});

export type GuideSummary = z.infer<typeof guideSummarySchema>;
export type GuideListRes = z.infer<typeof guideListSchema>;
export type GuideCell = z.infer<typeof guideCellSchema>;
export type GuideItem = z.infer<typeof guideItemSchema>;
export type GuideSentence = z.infer<typeof guideSentenceSchema>;
export type GuideDetail = z.infer<typeof guideDetailSchema>;

/**
 * 학습 로그 배선 (백엔드 `guide_sentence_log`/`_status`).
 * 채점은 프론트(완전일치) — 서버는 결과(action) 수신·기록. 정/오만 status 갱신.
 */
export type GuideActivity =
  | "sentence_write"
  | "read_along"
  | "flashcard"
  | "matching"
  | "writing_test";
export type GuideLogAction =
  | "view"
  | "attempt"
  | "correct"
  | "wrong"
  | "reveal"
  | "complete";

export interface GuideLogReq {
  activity: GuideActivity;
  action: GuideLogAction;
  /** 제출 답안(선택) — 향후 점수화 데이터 */
  answer?: unknown;
}

export const guideSentenceStatusSchema = z.object({
  try_count: z.number(),
  is_solved: z.boolean(),
  last_attempt_at: z.string().nullable().optional(),
});

export const guideProgressItemSchema = z.object({
  sentence_no: z.number(),
  try_count: z.number(),
  is_solved: z.boolean(),
  last_attempt_at: z.string().nullable().optional(),
});

export const guideProgressSchema = z.object({
  items: z.array(guideProgressItemSchema),
});

export type GuideSentenceStatus = z.infer<typeof guideSentenceStatusSchema>;
export type GuideProgressItem = z.infer<typeof guideProgressItemSchema>;
export type GuideProgress = z.infer<typeof guideProgressSchema>;

/** 교재 10색 테마 → CSS 변수 쌍 (books scripts/textbook/css/themes.css 동기) */
export const GUIDE_THEME_COLORS: Record<string, { color: string; bg: string }> = {
  blue: { color: "#2184fc", bg: "#eef5ff" },
  green: { color: "#10b981", bg: "#ecfdf5" },
  orange: { color: "#f59e0b", bg: "#fffbeb" },
  purple: { color: "#a855f7", bg: "#f6f0ff" },
  pink: { color: "#ec4899", bg: "#fdf2f8" },
  teal: { color: "#14b8a6", bg: "#f0fdfa" },
  indigo: { color: "#322acf", bg: "#ebebff" },
  rose: { color: "#f43f5e", bg: "#fff1f2" },
  amber: { color: "#f97316", bg: "#fff7ed" },
  slate: { color: "#64748b", bg: "#f8fafc" },
};

export function guideThemeColors(theme: string) {
  return GUIDE_THEME_COLORS[theme] ?? GUIDE_THEME_COLORS.blue;
}
