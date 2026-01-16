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

export const submitAnswerReqSchema = z.union([
  z.object({
    choice: z.object({
      pick: z.number().int(),
    }),
  }),
  z.object({
    typing: z.object({
      text: z.string(),
    }),
  }),
  z.object({
    voice: z.object({
      audio_url: z.string(),
    }),
  }),
]);

export type SubmitAnswerReq = z.infer<typeof submitAnswerReqSchema>;

// --- Responses ---

// 1. Study List Response

export const studyListItemSchema = z.object({
  study_id: z.number().int(),
  study_idx: z.string(),
  study_program: studyProgramSchema,
  title: z.string().optional(), // Rust: Option<String>
  subtitle: z.string().optional(),
  created_at: z.string().datetime(), // Rust: DateTime<Utc>
});

export type StudyListItem = z.infer<typeof studyListItemSchema>;

export const studyListMetaSchema = z.object({
  total_count: z.number().int(), // Fixed: total -> total_count
  total_pages: z.number().int(),
  current_page: z.number().int(), // Fixed: page -> current_page
  per_page: z.number().int(),
});

export type StudyListMeta = z.infer<typeof studyListMetaSchema>;

export const studyListResSchema = z.object({
  meta: studyListMetaSchema,
  data: z.array(studyListItemSchema),
});

export type StudyListRes = z.infer<typeof studyListResSchema>;

// 2. Task Detail Response

export const choicePayloadSchema = z.object({
  choice_1: z.string(),
  choice_2: z.string(),
  choice_3: z.string(),
  choice_4: z.string(),
  image_url: z.string().optional().nullable(), // Rust: Option<String>
});

export type ChoicePayload = z.infer<typeof choicePayloadSchema>;

export const typingPayloadSchema = z.object({
  image_url: z.string().optional().nullable(),
});

export type TypingPayload = z.infer<typeof typingPayloadSchema>;

export const voicePayloadSchema = z.object({
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
  question: z.string().optional().nullable(),
  media_url: z.string().optional().nullable(),
  payload: taskPayloadSchema,
});

export type StudyTaskDetailRes = z.infer<typeof studyTaskDetailResSchema>;

// 3. Action Responses (Answer, Status, Explain)

export const submitAnswerResSchema = z.object({
  task_id: z.number().int(),
  is_correct: z.boolean(),
  score: z.number().int(),
  correct_answer: z.string().optional().nullable(),
});

export type SubmitAnswerRes = z.infer<typeof submitAnswerResSchema>;

export const taskStatusResSchema = z.object({
  task_id: z.number().int(),
  attempts: z.number().int(),
  is_solved: z.boolean(),
  last_score: z.number().int().optional().nullable(), // Rust: Option<i32>
});

export type TaskStatusRes = z.infer<typeof taskStatusResSchema>;

export const taskExplainResSchema = z.object({
  task_id: z.number().int(),
  correct_answer: z.string(),
  explanation_text: z.string().optional().nullable(),
  explanation_media_url: z.string().optional().nullable(),
  related_video_url: z.string().optional().nullable(),
});

export type TaskExplainRes = z.infer<typeof taskExplainResSchema>;