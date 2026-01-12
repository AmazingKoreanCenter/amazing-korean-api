import { z } from "zod";

import { studyProgramSchema, studyTaskKindSchema } from "../../study/types";

export const studyStateSchema = z.enum(["ready", "open", "close"]);

export type StudyState = z.infer<typeof studyStateSchema>;

export const userSetLanguageSchema = z.enum(["ko", "en"]);

export type UserSetLanguage = z.infer<typeof userSetLanguageSchema>;

export const studyListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  size: z.number().int().min(1).max(100).optional(),
  q: z.string().optional(),
  sort: z.string().optional(),
  order: z.string().optional(),
  study_state: studyStateSchema.optional(),
  study_program: studyProgramSchema.optional(),
});

export type StudyListReq = z.infer<typeof studyListReqSchema>;

export const studyCreateReqSchema = z.object({
  study_idx: z.string().min(2),
  study_title: z.string().optional(),
  study_subtitle: z.string().optional(),
  study_description: z.string().optional(),
  study_program: studyProgramSchema.optional(),
  study_state: studyStateSchema.optional(),
});

export type StudyCreateReq = z.infer<typeof studyCreateReqSchema>;

export const studyUpdateReqSchema = z.object({
  study_idx: z.string().min(2).optional(),
  study_state: studyStateSchema.optional(),
  study_program: studyProgramSchema.optional(),
  study_title: z.string().min(1).max(80).optional(),
  study_subtitle: z.string().max(120).optional(),
  study_description: z.string().optional(),
});

export type StudyUpdateReq = z.infer<typeof studyUpdateReqSchema>;

export const studyBulkCreateReqSchema = z.object({
  items: z.array(studyCreateReqSchema).min(1).max(100),
});

export type StudyBulkCreateReq = z.infer<typeof studyBulkCreateReqSchema>;

export const studyBulkUpdateItemSchema = z.object({
  id: z.number().int().min(1),
  study_idx: z.string().min(2).optional(),
  study_title: z.string().optional(),
  study_subtitle: z.string().optional(),
  study_description: z.string().optional(),
  study_program: studyProgramSchema.optional(),
  study_state: studyStateSchema.optional(),
});

export type StudyBulkUpdateItem = z.infer<typeof studyBulkUpdateItemSchema>;

export const studyBulkUpdateReqSchema = z.object({
  items: z.array(studyBulkUpdateItemSchema).min(1).max(100),
});

export type StudyBulkUpdateReq = z.infer<typeof studyBulkUpdateReqSchema>;

export const adminStudyResSchema = z.object({
  study_id: z.number().int(),
  study_idx: z.string(),
  study_title: z.string().optional(),
  study_subtitle: z.string().optional(),
  study_program: studyProgramSchema,
  study_state: studyStateSchema,
  study_created_at: z.string().datetime(),
  study_updated_at: z.string().datetime(),
});

export type AdminStudyRes = z.infer<typeof adminStudyResSchema>;

export const adminStudyListResSchema = z.object({
  list: z.array(adminStudyResSchema),
  total: z.number().int(),
  page: z.number().int(),
  size: z.number().int(),
  total_pages: z.number().int(),
});

export type AdminStudyListRes = z.infer<typeof adminStudyListResSchema>;

export const studyBulkResultSchema = z.object({
  id: z.number().int().optional(),
  idx: z.string(),
  success: z.boolean(),
  error: z.string().optional(),
});

export type StudyBulkResult = z.infer<typeof studyBulkResultSchema>;

export const studyBulkCreateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(studyBulkResultSchema),
});

export type StudyBulkCreateRes = z.infer<typeof studyBulkCreateResSchema>;

export const studyBulkUpdateResultSchema = z.object({
  id: z.number().int(),
  success: z.boolean(),
  error: z.string().optional(),
});

export type StudyBulkUpdateResult = z.infer<typeof studyBulkUpdateResultSchema>;

export const studyBulkUpdateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(studyBulkUpdateResultSchema),
});

export type StudyBulkUpdateRes = z.infer<typeof studyBulkUpdateResSchema>;

export const studyTaskListReqSchema = z.object({
  study_id: z.number().int().min(1),
  page: z.number().int().min(1).optional(),
  size: z.number().int().min(1).max(100).optional(),
});

export type StudyTaskListReq = z.infer<typeof studyTaskListReqSchema>;

export const taskExplainListReqSchema = z.object({
  task_id: z.number().int().min(1),
  page: z.number().int().min(1).optional(),
  size: z.number().int().min(1).max(100).optional(),
});

export type TaskExplainListReq = z.infer<typeof taskExplainListReqSchema>;

export const taskExplainCreateReqSchema = z.object({
  explain_lang: userSetLanguageSchema,
  explain_title: z.string().optional(),
  explain_text: z.string().optional(),
  explain_media_url: z.string().optional(),
});

export type TaskExplainCreateReq = z.infer<typeof taskExplainCreateReqSchema>;

export const taskExplainUpdateReqSchema = z.object({
  explain_lang: userSetLanguageSchema,
  explain_title: z.string().optional(),
  explain_text: z.string().optional(),
  explain_media_url: z.string().optional(),
});

export type TaskExplainUpdateReq = z.infer<typeof taskExplainUpdateReqSchema>;

export const taskExplainCreateItemSchema = z.object({
  study_task_id: z.number().int().min(1),
  explain_lang: userSetLanguageSchema,
  explain_title: z.string().optional(),
  explain_text: z.string().optional(),
  explain_media_url: z.string().optional(),
});

export type TaskExplainCreateItem = z.infer<typeof taskExplainCreateItemSchema>;

export const taskExplainBulkCreateReqSchema = z.object({
  items: z.array(taskExplainCreateItemSchema).min(1).max(100),
});

export type TaskExplainBulkCreateReq = z.infer<typeof taskExplainBulkCreateReqSchema>;

export const taskExplainBulkResultSchema = z.object({
  study_task_id: z.number().int(),
  explain_lang: userSetLanguageSchema,
  success: z.boolean(),
  error: z.string().optional(),
});

export type TaskExplainBulkResult = z.infer<typeof taskExplainBulkResultSchema>;

export const taskExplainBulkCreateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(taskExplainBulkResultSchema),
});

export type TaskExplainBulkCreateRes = z.infer<typeof taskExplainBulkCreateResSchema>;

export const taskExplainUpdateItemSchema = z.object({
  study_task_id: z.number().int().min(1),
  explain_lang: userSetLanguageSchema,
  explain_title: z.string().optional(),
  explain_text: z.string().optional(),
  explain_media_url: z.string().optional(),
});

export type TaskExplainUpdateItem = z.infer<typeof taskExplainUpdateItemSchema>;

export const taskExplainBulkUpdateReqSchema = z.object({
  items: z.array(taskExplainUpdateItemSchema).min(1).max(100),
});

export type TaskExplainBulkUpdateReq = z.infer<typeof taskExplainBulkUpdateReqSchema>;

export const taskExplainBulkUpdateResultSchema = z.object({
  study_task_id: z.number().int(),
  explain_lang: userSetLanguageSchema,
  success: z.boolean(),
  error: z.string().optional(),
});

export type TaskExplainBulkUpdateResult = z.infer<typeof taskExplainBulkUpdateResultSchema>;

export const taskExplainBulkUpdateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(taskExplainBulkUpdateResultSchema),
});

export type TaskExplainBulkUpdateRes = z.infer<typeof taskExplainBulkUpdateResSchema>;

export const adminTaskExplainResSchema = z.object({
  study_task_id: z.number().int(),
  explain_lang: userSetLanguageSchema,
  explain_title: z.string().optional(),
  explain_text: z.string().optional(),
  explain_media_url: z.string().optional(),
  explain_created_at: z.string().datetime(),
  explain_updated_at: z.string().datetime(),
});

export type AdminTaskExplainRes = z.infer<typeof adminTaskExplainResSchema>;

export const adminTaskExplainListResSchema = z.object({
  list: z.array(adminTaskExplainResSchema),
  total: z.number().int(),
  page: z.number().int(),
  size: z.number().int(),
  total_pages: z.number().int(),
});

export type AdminTaskExplainListRes = z.infer<typeof adminTaskExplainListResSchema>;

export const taskStatusListReqSchema = z.object({
  task_id: z.number().int().min(1).optional(),
  user_id: z.number().int().min(1).optional(),
  page: z.number().int().min(1).optional(),
  size: z.number().int().min(1).max(100).optional(),
});

export type TaskStatusListReq = z.infer<typeof taskStatusListReqSchema>;

export const adminTaskStatusResSchema = z.object({
  study_task_id: z.number().int(),
  user_id: z.number().int(),
  study_task_status_try: z.number().int(),
  study_task_status_best: z.number().int(),
  study_task_status_completed: z.boolean(),
  study_task_status_last_answer: z.string().datetime().optional(),
});

export type AdminTaskStatusRes = z.infer<typeof adminTaskStatusResSchema>;

export const adminTaskStatusListResSchema = z.object({
  list: z.array(adminTaskStatusResSchema),
  total: z.number().int(),
  page: z.number().int(),
  size: z.number().int(),
  total_pages: z.number().int(),
});

export type AdminTaskStatusListRes = z.infer<typeof adminTaskStatusListResSchema>;

export const taskStatusUpdateReqSchema = z.object({
  user_id: z.number().int().min(1),
  study_task_status_try: z.number().int().optional(),
  study_task_status_best: z.number().int().optional(),
  study_task_status_completed: z.boolean().optional(),
  study_task_status_last_answer: z.string().datetime().optional(),
});

export type TaskStatusUpdateReq = z.infer<typeof taskStatusUpdateReqSchema>;

export const taskStatusUpdateItemSchema = z.object({
  study_task_id: z.number().int().min(1),
  user_id: z.number().int().min(1),
  study_task_status_try: z.number().int().optional(),
  study_task_status_best: z.number().int().optional(),
  study_task_status_completed: z.boolean().optional(),
  study_task_status_last_answer: z.string().datetime().optional(),
});

export type TaskStatusUpdateItem = z.infer<typeof taskStatusUpdateItemSchema>;

export const taskStatusBulkUpdateReqSchema = z.object({
  items: z.array(taskStatusUpdateItemSchema).min(1).max(100),
});

export type TaskStatusBulkUpdateReq = z.infer<typeof taskStatusBulkUpdateReqSchema>;

export const taskStatusBulkUpdateResultSchema = z.object({
  study_task_id: z.number().int(),
  user_id: z.number().int(),
  success: z.boolean(),
  error: z.string().optional(),
});

export type TaskStatusBulkUpdateResult = z.infer<typeof taskStatusBulkUpdateResultSchema>;

export const taskStatusBulkUpdateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(taskStatusBulkUpdateResultSchema),
});

export type TaskStatusBulkUpdateRes = z.infer<typeof taskStatusBulkUpdateResSchema>;

export const adminStudyTaskResSchema = z.object({
  study_task_id: z.number().int(),
  study_task_kind: studyTaskKindSchema,
  study_task_seq: z.number().int(),
  question: z.string().optional(),
});

export type AdminStudyTaskRes = z.infer<typeof adminStudyTaskResSchema>;

export const adminStudyTaskListResSchema = z.object({
  list: z.array(adminStudyTaskResSchema),
  total: z.number().int(),
  page: z.number().int(),
  size: z.number().int(),
  total_pages: z.number().int(),
});

export type AdminStudyTaskListRes = z.infer<typeof adminStudyTaskListResSchema>;

export const studyTaskCreateReqSchema = z.object({
  study_id: z.number().int().min(1),
  study_task_kind: studyTaskKindSchema,
  study_task_seq: z.number().int().min(1).optional(),
  question: z.string().optional(),
  answer: z.string().optional(),
  image_url: z.string().optional(),
  audio_url: z.string().optional(),
  choice_1: z.string().optional(),
  choice_2: z.string().optional(),
  choice_3: z.string().optional(),
  choice_4: z.string().optional(),
  choice_correct: z.number().int().optional(),
});

export type StudyTaskCreateReq = z.infer<typeof studyTaskCreateReqSchema>;

export const studyTaskBulkCreateReqSchema = z.object({
  items: z.array(studyTaskCreateReqSchema).min(1).max(100),
});

export type StudyTaskBulkCreateReq = z.infer<typeof studyTaskBulkCreateReqSchema>;

export const studyTaskBulkResultSchema = z.object({
  task_id: z.number().int().optional(),
  seq: z.number().int(),
  kind: studyTaskKindSchema,
  error: z.string().optional(),
});

export type StudyTaskBulkResult = z.infer<typeof studyTaskBulkResultSchema>;

export const studyTaskBulkCreateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(studyTaskBulkResultSchema),
});

export type StudyTaskBulkCreateRes = z.infer<typeof studyTaskBulkCreateResSchema>;

export const studyTaskUpdateReqSchema = z.object({
  study_task_seq: z.number().int().optional(),
  question: z.string().optional(),
  answer: z.string().optional(),
  image_url: z.string().optional(),
  audio_url: z.string().optional(),
  choice_1: z.string().optional(),
  choice_2: z.string().optional(),
  choice_3: z.string().optional(),
  choice_4: z.string().optional(),
  choice_correct: z.number().int().optional(),
});

export type StudyTaskUpdateReq = z.infer<typeof studyTaskUpdateReqSchema>;

export const studyTaskUpdateItemSchema = z.object({
  study_task_id: z.number().int().min(1),
  study_task_seq: z.number().int().min(1).optional(),
  question: z.string().optional(),
  answer: z.string().optional(),
  image_url: z.string().optional(),
  audio_url: z.string().optional(),
  choice_1: z.string().optional(),
  choice_2: z.string().optional(),
  choice_3: z.string().optional(),
  choice_4: z.string().optional(),
  choice_correct: z.number().int().optional(),
});

export type StudyTaskUpdateItem = z.infer<typeof studyTaskUpdateItemSchema>;

export const studyTaskBulkUpdateReqSchema = z.object({
  items: z.array(studyTaskUpdateItemSchema).min(1).max(100),
});

export type StudyTaskBulkUpdateReq = z.infer<typeof studyTaskBulkUpdateReqSchema>;

export const studyTaskBulkUpdateResultSchema = z.object({
  task_id: z.number().int(),
  success: z.boolean(),
  error: z.string().optional(),
});

export type StudyTaskBulkUpdateResult = z.infer<typeof studyTaskBulkUpdateResultSchema>;

export const studyTaskBulkUpdateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(studyTaskBulkUpdateResultSchema),
});

export type StudyTaskBulkUpdateRes = z.infer<typeof studyTaskBulkUpdateResSchema>;

export const adminStudyTaskDetailResSchema = z.object({
  study_task_id: z.number().int(),
  study_id: z.number().int(),
  study_task_kind: studyTaskKindSchema,
  study_task_seq: z.number().int(),
  question: z.string().optional(),
  answer: z.string().optional(),
  image_url: z.string().optional(),
  audio_url: z.string().optional(),
  choice_1: z.string().optional(),
  choice_2: z.string().optional(),
  choice_3: z.string().optional(),
  choice_4: z.string().optional(),
  choice_correct: z.number().int().optional(),
});

export type AdminStudyTaskDetailRes = z.infer<typeof adminStudyTaskDetailResSchema>;
