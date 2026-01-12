import { z } from "zod";

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

export const studyListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  per_page: z.number().int().min(1).max(100).optional(),
  program: z.string().optional(),
  sort: z.string().optional(),
});

export type StudyListReq = z.infer<typeof studyListReqSchema>;

export const studyListItemSchema = z.object({
  study_id: z.number().int(),
  study_idx: z.string(),
  study_program: studyProgramSchema,
  title: z.string().optional(),
  subtitle: z.string().optional(),
  created_at: z.string().datetime(),
});

export type StudyListItem = z.infer<typeof studyListItemSchema>;

export const studyListMetaSchema = z.object({
  page: z.number().int(),
  per_page: z.number().int(),
  total: z.number().int(),
  total_pages: z.number().int(),
});

export type StudyListMeta = z.infer<typeof studyListMetaSchema>;

export const studyListResSchema = z.object({
  data: z.array(studyListItemSchema),
  meta: studyListMetaSchema,
});

export type StudyListRes = z.infer<typeof studyListResSchema>;

export const choicePayloadSchema = z.object({
  choice_1: z.string(),
  choice_2: z.string(),
  choice_3: z.string(),
  choice_4: z.string(),
  image_url: z.string().optional(),
});

export type ChoicePayload = z.infer<typeof choicePayloadSchema>;

export const typingPayloadSchema = z.object({
  image_url: z.string().optional(),
});

export type TypingPayload = z.infer<typeof typingPayloadSchema>;

export const voicePayloadSchema = z.object({
  image_url: z.string().optional(),
});

export type VoicePayload = z.infer<typeof voicePayloadSchema>;

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
  question: z.string().optional(),
  media_url: z.string().optional(),
  payload: taskPayloadSchema,
});

export type StudyTaskDetailRes = z.infer<typeof studyTaskDetailResSchema>;

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

export const submitAnswerResSchema = z.object({
  task_id: z.number().int(),
  is_correct: z.boolean(),
  score: z.number().int(),
  correct_answer: z.string().optional(),
});

export type SubmitAnswerRes = z.infer<typeof submitAnswerResSchema>;

export const taskStatusResSchema = z.object({
  task_id: z.number().int(),
  attempts: z.number().int(),
  is_solved: z.boolean(),
  best_score: z.number().int(),
  last_score: z.number().int(),
  progress: z.number().int(),
  last_attempt_at: z.string().datetime().optional(),
});

export type TaskStatusRes = z.infer<typeof taskStatusResSchema>;

export const taskExplainResSchema = z.object({
  task_id: z.number().int(),
  explanation: z.string().optional(),
  resources: z.array(z.string()),
});

export type TaskExplainRes = z.infer<typeof taskExplainResSchema>;
