import { z } from "zod";

// --- Enums ---

export const studyProgramSchema = z.enum([
  "basic_pronunciation",
  "basic_word",
  "basic_500",
  "topik_read",
  "topik_listen",
  "topik_write",
  "tbc",
]);

export type StudyProgram = z.infer<typeof studyProgramSchema>;

export const studyTaskKindSchema = z.enum(["choice", "typing", "voice", "writing"]);

export type StudyTaskKind = z.infer<typeof studyTaskKindSchema>;

export const writingLevelSchema = z.enum(["beginner", "intermediate", "advanced"]);

export type WritingLevel = z.infer<typeof writingLevelSchema>;

export const writingPracticeTypeSchema = z.enum([
  "jamo",
  "syllable",
  "word",
  "sentence",
  "paragraph",
]);

export type WritingPracticeType = z.infer<typeof writingPracticeTypeSchema>;

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
  z.object({
    kind: z.literal("writing"),
    text: z.string(),
    session_id: z.number().int().nullable().optional(),
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

// 한글 자판 연습 페이로드 (backend: WritingPayload)
// question 대신 prompt 사용, 초급 레벨에서만 answer 포함
export const writingPayloadSchema = z.object({
  prompt: z.string(),
  answer: z.string().optional().nullable(),
  hint: z.string().optional().nullable(),
  level: writingLevelSchema,
  practice_type: writingPracticeTypeSchema,
  keyboard_visible: z.boolean(),
  image_url: z.string().optional().nullable(),
  audio_url: z.string().optional().nullable(),
});

export type WritingPayload = z.infer<typeof writingPayloadSchema>;

// Backend uses #[serde(untagged)], so payloads are flattened in JSON.
export const taskPayloadSchema = z.union([
  choicePayloadSchema,
  typingPayloadSchema,
  voicePayloadSchema,
  writingPayloadSchema,
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

// =========================================================================
// Writing Practice Session (한글 자판 연습 세션 API)
// =========================================================================

// --- Requests ---

export const startWritingSessionReqSchema = z.object({
  study_task_id: z.number().int().nullable().optional(),
  writing_level: writingLevelSchema,
  writing_practice_type: writingPracticeTypeSchema,
});

export type StartWritingSessionReq = z.infer<typeof startWritingSessionReqSchema>;

export const writingMistakeSchema = z.object({
  position: z.number().int(),
  expected: z.string(),
  actual: z.string(),
});

export type WritingMistake = z.infer<typeof writingMistakeSchema>;

export const finishWritingSessionReqSchema = z.object({
  total_chars: z.number().int().min(0),
  correct_chars: z.number().int().min(0),
  duration_ms: z.number().int().min(0),
  mistakes: z.array(writingMistakeSchema).default([]),
});

export type FinishWritingSessionReq = z.infer<typeof finishWritingSessionReqSchema>;

export const writingSessionListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  per_page: z.number().int().min(1).max(100).optional(),
  level: writingLevelSchema.optional(),
  finished_only: z.boolean().optional(),
});

export type WritingSessionListReq = z.infer<typeof writingSessionListReqSchema>;

export const writingStatsReqSchema = z.object({
  days: z.number().int().min(1).max(365).optional(),
});

export type WritingStatsReq = z.infer<typeof writingStatsReqSchema>;

// --- Responses ---

export const writingSessionResSchema = z.object({
  session_id: z.number().int(),
  user_id: z.number().int(),
  study_task_id: z.number().int().nullable(),
  writing_level: writingLevelSchema,
  writing_practice_type: writingPracticeTypeSchema,
  started_at: z.string().datetime(),
  finished_at: z.string().datetime().nullable(),
  total_chars: z.number().int(),
  correct_chars: z.number().int(),
  accuracy_rate: z.number(),
  chars_per_minute: z.number(),
  mistakes: z.array(writingMistakeSchema),
});

export type WritingSessionRes = z.infer<typeof writingSessionResSchema>;

export const writingSessionListResSchema = z.object({
  list: z.array(writingSessionResSchema),
  meta: studyListMetaSchema,
});

export type WritingSessionListRes = z.infer<typeof writingSessionListResSchema>;

export const writingLevelStatSchema = z.object({
  writing_level: writingLevelSchema,
  sessions: z.number().int(),
  avg_accuracy: z.number(),
  avg_cpm: z.number(),
});

export type WritingLevelStat = z.infer<typeof writingLevelStatSchema>;

export const writingDailyStatSchema = z.object({
  day: z.string(), // YYYY-MM-DD
  sessions: z.number().int(),
  avg_accuracy: z.number(),
  avg_cpm: z.number(),
});

export type WritingDailyStat = z.infer<typeof writingDailyStatSchema>;

export const writingWeakCharSchema = z.object({
  expected: z.string(),
  miss_count: z.number().int(),
});

export type WritingWeakChar = z.infer<typeof writingWeakCharSchema>;

export const writingStatsResSchema = z.object({
  total_sessions: z.number().int(),
  avg_accuracy: z.number(),
  avg_cpm: z.number(),
  level_breakdown: z.array(writingLevelStatSchema),
  recent_trend: z.array(writingDailyStatSchema),
  weak_chars: z.array(writingWeakCharSchema),
});

export type WritingStatsRes = z.infer<typeof writingStatsResSchema>;

// =========================================================================
// Writing Practice Seed (자유 연습 드릴 컨텐츠)
// =========================================================================

export const writingPracticeSeedReqSchema = z.object({
  level: writingLevelSchema,
  practice_type: writingPracticeTypeSchema,
  limit: z.number().int().min(1).max(100).optional(),
});

export type WritingPracticeSeedReq = z.infer<typeof writingPracticeSeedReqSchema>;

export const writingPracticeSeedItemSchema = z.object({
  seed_id: z.number().int(),
  seq: z.number().int(),
  prompt: z.string(),
  answer: z.string(),
  hint: z.string().nullable().optional(),
});

export type WritingPracticeSeedItem = z.infer<typeof writingPracticeSeedItemSchema>;

export const writingPracticeSeedResSchema = z.object({
  level: writingLevelSchema,
  practice_type: writingPracticeTypeSchema,
  items: z.array(writingPracticeSeedItemSchema),
});

export type WritingPracticeSeedRes = z.infer<typeof writingPracticeSeedResSchema>;