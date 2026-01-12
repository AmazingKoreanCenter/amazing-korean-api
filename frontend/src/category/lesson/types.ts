import { z } from "zod";

export const lessonItemKindSchema = z.enum(["video", "task"]);

export type LessonItemKind = z.infer<typeof lessonItemKindSchema>;

export const lessonListReqSchema = z.object({
  page: z.number().int().optional(),
  per_page: z.number().int().optional(),
  sort: z.string().optional(),
});

export type LessonListReq = z.infer<typeof lessonListReqSchema>;

export const lessonResSchema = z.object({
  id: z.number().int(),
  title: z.string(),
  description: z.string().optional(),
  lesson_idx: z.string(),
  thumbnail_url: z.string().optional(),
});

export type LessonRes = z.infer<typeof lessonResSchema>;

export const lessonListMetaSchema = z.object({
  total_count: z.number().int(),
  total_pages: z.number().int(),
  current_page: z.number().int(),
  per_page: z.number().int(),
});

export type LessonListMeta = z.infer<typeof lessonListMetaSchema>;

export const lessonListResSchema = z.object({
  items: z.array(lessonResSchema),
  meta: lessonListMetaSchema,
});

export type LessonListRes = z.infer<typeof lessonListResSchema>;

export const lessonDetailReqSchema = z.object({
  page: z.number().int().optional(),
  per_page: z.number().int().optional(),
});

export type LessonDetailReq = z.infer<typeof lessonDetailReqSchema>;

export const lessonItemResSchema = z.object({
  seq: z.number().int(),
  kind: lessonItemKindSchema,
  video_id: z.number().int().optional(),
  task_id: z.number().int().optional(),
});

export type LessonItemRes = z.infer<typeof lessonItemResSchema>;

export const lessonDetailResSchema = z.object({
  lesson_id: z.number().int(),
  title: z.string(),
  description: z.string().optional(),
  items: z.array(lessonItemResSchema),
  meta: lessonListMetaSchema,
});

export type LessonDetailRes = z.infer<typeof lessonDetailResSchema>;

export const lessonItemsReqSchema = z.object({
  page: z.number().int().optional(),
  per_page: z.number().int().optional(),
});

export type LessonItemsReq = z.infer<typeof lessonItemsReqSchema>;

export const lessonItemDetailResSchema = z.object({
  seq: z.number().int(),
  kind: lessonItemKindSchema,
  video_id: z.number().int().optional(),
  study_task_id: z.number().int().optional(),
});

export type LessonItemDetailRes = z.infer<typeof lessonItemDetailResSchema>;

export const lessonItemsResSchema = z.object({
  items: z.array(lessonItemDetailResSchema),
  meta: lessonListMetaSchema,
});

export type LessonItemsRes = z.infer<typeof lessonItemsResSchema>;

export const lessonProgressResSchema = z.object({
  percent: z.number().int(),
  last_seq: z.number().int().optional(),
  updated_at: z.string().datetime().optional(),
});

export type LessonProgressRes = z.infer<typeof lessonProgressResSchema>;

export const lessonProgressUpdateReqSchema = z.object({
  percent: z.number().int(),
  last_seq: z.number().int().optional(),
});

export type LessonProgressUpdateReq = z.infer<typeof lessonProgressUpdateReqSchema>;
