import { z } from "zod";

// --- Enums ---

export const studyProgramSchema = z.enum([
  "basic_pronunciation",
  "basic_word",
  "basic_900",
  "topik_read",
  "topik_listen",
  "topik_write",
  "tbc",
]);

export type StudyProgram = z.infer<typeof studyProgramSchema>;

export const studyTaskKindSchema = z.enum(["choice", "typing", "voice"]);

export type StudyTaskKind = z.infer<typeof studyTaskKindSchema>;

// --- Requests ---

export const studyListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  per_page: z.number().int().min(1).max(100).optional(),
  program: z.string().optional(),
  sort: z.string().optional(),
});

export type StudyListReq = z.infer<typeof studyListReqSchema>;

// Backend uses #[serde(tag = "kind")] - discriminated union
export const submitAnswerReqSchema = z.discriminatedUnion("kind", [
  z.object({
    kind: z.literal("choice"),
    pick: z.number().int(),
  }),
  z.object({
    kind: z.literal("typing"),
    text: z.string(),
  }),
  z.object({
    kind: z.literal("voice"),
    text: z.string(),
  }),
]);

export type SubmitAnswerReq = z.infer<typeof submitAnswerReqSchema>;

// --- Responses ---

// 1. Study List Response

export const studyListItemSchema = z.object({
  study_id: z.number().int(),
  study_idx: z.string(),
  program: studyProgramSchema, // Backend: "program"
  title: z.string().optional().nullable(),
  subtitle: z.string().optional().nullable(),
  state: z.string(), // Backend includes state field
  created_at: z.string().datetime(),
});

export type StudyListItem = z.infer<typeof studyListItemSchema>;

export const studyListMetaSchema = z.object({
  total_count: z.number().int(),
  total_pages: z.number().int(),
  page: z.number().int(), // Backend: "page" not "current_page"
  per_page: z.number().int(),
});

export type StudyListMeta = z.infer<typeof studyListMetaSchema>;

export const studyListResSchema = z.object({
  meta: studyListMetaSchema,
  list: z.array(studyListItemSchema), // Backend: "list" not "data"
});

export type StudyListRes = z.infer<typeof studyListResSchema>;

// 1-2. Study Detail Response (GET /studies/{id})

export const studyDetailReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  per_page: z.number().int().min(1).max(100).optional(),
});

export type StudyDetailReq = z.infer<typeof studyDetailReqSchema>;

export const studyTaskSummarySchema = z.object({
  task_id: z.number().int(),
  kind: studyTaskKindSchema,
  seq: z.number().int(),
});

export type StudyTaskSummary = z.infer<typeof studyTaskSummarySchema>;

export const studyDetailResSchema = z.object({
  study_id: z.number().int(),
  study_idx: z.string(),
  program: studyProgramSchema,
  title: z.string().optional().nullable(),
  subtitle: z.string().optional().nullable(),
  state: z.string(),
  tasks: z.array(studyTaskSummarySchema),
  meta: studyListMetaSchema,
});

export type StudyDetailRes = z.infer<typeof studyDetailResSchema>;

// 2. Task Detail Response

export const choicePayloadSchema = z.object({
  question: z.string(),
  choice_1: z.string(),
  choice_2: z.string(),
  choice_3: z.string(),
  choice_4: z.string(),
  audio_url: z.string().optional().nullable(),
  image_url: z.string().optional().nullable(),
});

export type ChoicePayload = z.infer<typeof choicePayloadSchema>;

export const typingPayloadSchema = z.object({
  question: z.string(),
  image_url: z.string().optional().nullable(),
});

export type TypingPayload = z.infer<typeof typingPayloadSchema>;

export const voicePayloadSchema = z.object({
  question: z.string(),
  audio_url: z.string().optional().nullable(),
  image_url: z.string().optional().nullable(),
});

export type VoicePayload = z.infer<typeof voicePayloadSchema>;

// Backend uses #[serde(untagged)], so payloads are flattened in JSON.
export const taskPayloadSchema = z.union([
  choicePayloadSchema,
  typingPayloadSchema,
  voicePayloadSchema,
]);

export type TaskPayload = z.infer<typeof taskPayloadSchema>;

export const studyTaskDetailResSchema = z.object({
  task_id: z.number().int(),
  study_id: z.number().int(),
  kind: studyTaskKindSchema,
  seq: z.number().int(),
  created_at: z.string().datetime(),
  payload: taskPayloadSchema,
});

export type StudyTaskDetailRes = z.infer<typeof studyTaskDetailResSchema>;

// 3. Action Responses (Answer, Status, Explain)

export const submitAnswerResSchema = z.object({
  is_correct: z.boolean(),
  correct_answer: z.string().optional().nullable(),
  explanation: z.string().optional().nullable(),
});

export type SubmitAnswerRes = z.infer<typeof submitAnswerResSchema>;

export const taskStatusResSchema = z.object({
  try_count: z.number().int(),
  is_solved: z.boolean(),
  last_attempt_at: z.string().datetime().optional().nullable(),
});

export type TaskStatusRes = z.infer<typeof taskStatusResSchema>;

export const taskExplainResSchema = z.object({
  title: z.string().optional().nullable(),
  explanation: z.string().optional().nullable(),
  resources: z.array(z.string()),
});

export type TaskExplainRes = z.infer<typeof taskExplainResSchema>;