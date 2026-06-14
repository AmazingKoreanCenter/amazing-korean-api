import { z } from "zod";

/** admin guide 편집 — 백엔드 src/api/admin/guide/dto.rs 대응 */

export const adminGuideSummarySchema = z.object({
  guide_id: z.number(),
  guide_idx: z.string(),
  guide_seq: z.number(),
  guide_state: z.string(),
  guide_category: z.string(),
  guide_theme: z.string(),
  sentence_start: z.number().nullable().optional(),
  sentence_end: z.number().nullable().optional(),
  title_ko: z.string().nullable().optional(),
  title_en: z.string().nullable().optional(),
  stale_count: z.number(),
  block_count: z.number(),
});

export const adminGuideListSchema = z.object({
  items: z.array(adminGuideSummarySchema),
});

export const adminGuideBlockSchema = z.object({
  guide_block_id: z.number(),
  block_seq: z.number(),
  block_type: z.string(),
  sentence_no: z.number().nullable().optional(),
  text_ko: z.string().nullable().optional(),
  text_en: z.string().nullable().optional(),
  marker: z.string().nullable().optional(),
  table_no: z.number().nullable().optional(),
  row_no: z.number().nullable().optional(),
  col_no: z.number().nullable().optional(),
  col_span: z.number().nullable().optional(),
  row_span: z.number().nullable().optional(),
  source_version: z.number(),
  legacy_key: z.string().nullable().optional(),
  edited: z.boolean(),
});

export const adminGuideSentenceSchema = z.object({
  guide_sentence_id: z.number(),
  sentence_no: z.number(),
  pron_ko: z.string().nullable().optional(),
  speech_level: z.string().nullable().optional(),
  subject_honorific: z.boolean().nullable().optional(),
  audio_url: z.string().nullable().optional(),
});

export const adminGuideDetailSchema = z.object({
  guide_id: z.number(),
  guide_idx: z.string(),
  guide_seq: z.number(),
  guide_state: z.string(),
  guide_category: z.string(),
  guide_theme: z.string(),
  sentence_start: z.number().nullable().optional(),
  sentence_end: z.number().nullable().optional(),
  title_ko: z.string().nullable().optional(),
  title_en: z.string().nullable().optional(),
  subtitle_ko: z.string().nullable().optional(),
  subtitle_en: z.string().nullable().optional(),
  blocks: z.array(adminGuideBlockSchema),
  sentences: z.array(adminGuideSentenceSchema),
});

export const staleSummarySchema = z.object({
  lang: z.string(),
  stale_count: z.number(),
  missing_count: z.number(),
});

export const staleDashboardSchema = z.object({
  rows: z.array(staleSummarySchema),
});

export type AdminGuideSummary = z.infer<typeof adminGuideSummarySchema>;
export type AdminGuideListRes = z.infer<typeof adminGuideListSchema>;
export type AdminGuideBlock = z.infer<typeof adminGuideBlockSchema>;
export type AdminGuideSentence = z.infer<typeof adminGuideSentenceSchema>;
export type AdminGuideDetail = z.infer<typeof adminGuideDetailSchema>;
export type StaleSummary = z.infer<typeof staleSummarySchema>;
export type StaleDashboard = z.infer<typeof staleDashboardSchema>;

export type GuideMetaUpdate = {
  guide_state?: string;
  guide_theme?: string;
  title_ko?: string;
  title_en?: string;
  subtitle_ko?: string;
  subtitle_en?: string;
};

export type GuideBlockUpdate = {
  text_ko?: string | null;
  text_en?: string | null;
};

export const GUIDE_STATES = ["ready", "open", "close"] as const;
export const GUIDE_THEMES = [
  "blue", "green", "orange", "purple", "pink",
  "teal", "indigo", "rose", "amber", "slate",
] as const;
