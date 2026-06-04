import { z } from "zod";

/**
 * 해설(explanation) 콘텐츠 — 서빙 모델 = structured 골격 + i18n 해소 맵.
 * 백엔드 `src/api/explanation/dto.rs` (ExplanationListRes / ExplanationUnitRes / ExplanationBlockRes) 대응.
 * 단순 텍스트 블록은 `text`, structured/concept/qword 는 `structured` 골격 + `i18n`(field→해소 텍스트)
 * 으로 내려오고, 프론트가 index 불변식(`*_row_{i}_*` 등)으로 재조립한다(§5.10).
 */
export const explanationBlockSchema = z.object({
  block_seq: z.number(),
  block_type: z.string(),
  level: z.number().nullable().optional(),
  /** 단순 텍스트 블록(paragraph/heading/subtitle/step) 해소 결과 */
  text: z.string().nullable().optional(),
  /** lang-invariant 원형 (table/diagram/example HTML 등) */
  raw: z.string().nullable().optional(),
  /** lang-invariant 골격 (structured_explain rows / concept_card items / qword_card table) */
  structured: z.unknown().nullable().optional(),
  /** field_name → 해소 텍스트 (user_lang→en 폴백, inherit 계승 적용) */
  i18n: z.record(z.string(), z.string()).default({}),
});

export const explanationUnitSchema = z.object({
  unit_idx: z.string(),
  unit_kind: z.string(),
  unit_source: z.string(),
  study_idx: z.string().nullable().optional(),
  study_task_idx: z.string().nullable().optional(),
  sentence_num: z.number().nullable().optional(),
  section_id: z.string().nullable().optional(),
  title: z.string().nullable().optional(),
  subtitle: z.string().nullable().optional(),
  lang: z.string(),
  blocks: z.array(explanationBlockSchema),
});

export const explanationListSchema = z.object({
  items: z.array(explanationUnitSchema),
});

export type ExplanationBlock = z.infer<typeof explanationBlockSchema>;
export type ExplanationUnit = z.infer<typeof explanationUnitSchema>;
export type ExplanationListRes = z.infer<typeof explanationListSchema>;

/** concept_card 의 structured.items[i].raw 안에 들어있는 JSON 카드 형태 */
export const conceptCardItemSchema = z.object({
  variant: z.string().optional(),
  icon: z.string().optional(),
  tag_html: z.string().optional(),
  pattern_html: z.string().optional(),
  desc_html: z.string().optional(),
  desc_ko_html: z.string().optional(),
});
export type ConceptCardItem = z.infer<typeof conceptCardItemSchema>;
