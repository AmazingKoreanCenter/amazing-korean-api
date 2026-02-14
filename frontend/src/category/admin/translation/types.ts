import { z } from "zod";

// ==========================================
// Admin Translation 타입
// ==========================================

// ContentType enum (백엔드 types.rs ContentType 매핑)
export const contentTypeSchema = z.enum([
  "course",
  "lesson",
  "video",
  "video_tag",
  "study",
  "study_task_choice",
  "study_task_typing",
  "study_task_voice",
  "study_task_explain",
]);
export type ContentType = z.infer<typeof contentTypeSchema>;

// TranslationStatus enum (백엔드 types.rs TranslationStatus 매핑)
export const translationStatusSchema = z.enum([
  "draft",
  "reviewed",
  "approved",
]);
export type TranslationStatus = z.infer<typeof translationStatusSchema>;

// SupportedLanguage enum (백엔드 21개 언어, ko 제외 — ko는 원본)
export const supportedLanguageSchema = z.enum([
  "en", "ja", "zh-CN", "zh-TW", "vi", "th", "id", "my", "mn", "ru",
  "es", "pt", "fr", "de", "hi", "ne", "si", "km", "uz", "kk", "tg",
]);
export type SupportedLanguage = z.infer<typeof supportedLanguageSchema>;

// ── UI 카테고리 계층 ──────────────────────────

// UI 최상위 카테고리 (3개만 — 사용자 선택)
export type TopCategory = "video" | "study" | "lesson";

// Study 하위 타입
export type StudySubType =
  | "study"
  | "study_task_choice"
  | "study_task_typing"
  | "study_task_voice"
  | "study_task_explain";

// 최상위 카테고리 옵션
export const TOP_CATEGORIES: { value: TopCategory; label: string }[] = [
  { value: "video", label: "Video" },
  { value: "study", label: "Study" },
  { value: "lesson", label: "Lesson" },
];

// Study 하위 타입 옵션
export const STUDY_SUB_TYPES: { value: StudySubType; label: string }[] = [
  { value: "study", label: "Study (메인)" },
  { value: "study_task_choice", label: "Task Choice (객관식)" },
  { value: "study_task_typing", label: "Task Typing (타이핑)" },
  { value: "study_task_voice", label: "Task Voice (음성)" },
  { value: "study_task_explain", label: "Task Explain (해설)" },
];

// content_type → 읽기 쉬운 라벨 매핑
export const CONTENT_TYPE_LABELS: Record<string, string> = {
  course: "Course",
  lesson: "Lesson",
  video: "Video",
  video_tag: "Video (Tag)",
  study: "Study",
  study_task_choice: "Study Task (Choice)",
  study_task_typing: "Study Task (Typing)",
  study_task_voice: "Study Task (Voice)",
  study_task_explain: "Study Task (Explain)",
};

// 최상위 카테고리 → 필터링에 사용할 content_type 목록
export const CATEGORY_CONTENT_TYPES: Record<TopCategory, ContentType[]> = {
  video: ["video", "video_tag"],
  study: ["study", "study_task_choice", "study_task_typing", "study_task_voice", "study_task_explain"],
  lesson: ["lesson"],
};

// ── 요청 DTO ──────────────────────────────

// 번역 목록 조회 필터
export const translationListReqSchema = z.object({
  content_type: contentTypeSchema.optional(),
  content_id: z.number().int().optional(),
  lang: supportedLanguageSchema.optional(),
  status: translationStatusSchema.optional(),
  page: z.number().int().min(1).optional(),
  per_page: z.number().int().min(1).max(100).optional(),
});
export type TranslationListReq = z.infer<typeof translationListReqSchema>;

// 번역 생성 요청
export const translationCreateReqSchema = z.object({
  content_type: contentTypeSchema,
  content_id: z.number().int(),
  field_name: z.string().min(1).max(100),
  lang: supportedLanguageSchema,
  translated_text: z.string().min(1),
});
export type TranslationCreateReq = z.infer<typeof translationCreateReqSchema>;

// 번역 수정 요청
export const translationUpdateReqSchema = z.object({
  translated_text: z.string().min(1).optional(),
  status: translationStatusSchema.optional(),
});
export type TranslationUpdateReq = z.infer<typeof translationUpdateReqSchema>;

// 번역 상태 변경 요청
export const translationStatusReqSchema = z.object({
  status: translationStatusSchema,
});
export type TranslationStatusUpdateReq = z.infer<typeof translationStatusReqSchema>;

// ── 응답 DTO ──────────────────────────────

// 번역 단건 응답
export const translationResSchema = z.object({
  translation_id: z.number().int(),
  content_type: contentTypeSchema,
  content_id: z.number().int(),
  field_name: z.string(),
  lang: supportedLanguageSchema,
  translated_text: z.string(),
  status: translationStatusSchema,
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
});
export type TranslationRes = z.infer<typeof translationResSchema>;

// 번역 목록 메타
export const translationListMetaSchema = z.object({
  total_count: z.number().int(),
  total_pages: z.number().int(),
  current_page: z.number().int(),
  per_page: z.number().int(),
});

// 번역 목록 응답
export const translationListResSchema = z.object({
  items: z.array(translationResSchema),
  meta: translationListMetaSchema,
});
export type TranslationListRes = z.infer<typeof translationListResSchema>;

// ── 자동 번역 DTO ──────────────────────────

// 자동 번역 요청 (단건)
export const autoTranslateReqSchema = z.object({
  content_type: contentTypeSchema,
  content_id: z.number().int().positive(),
  field_name: z.string().min(1).max(100),
  source_text: z.string().min(1),
  target_langs: z.array(supportedLanguageSchema).min(1).max(20),
});
export type AutoTranslateReq = z.infer<typeof autoTranslateReqSchema>;

// 자동 번역 개별 결과
export interface AutoTranslateItemResult {
  lang: string;
  success: boolean;
  translation_id?: number;
  translated_text?: string;
  error?: string;
}

// 자동 번역 응답
export interface AutoTranslateRes {
  total: number;
  success_count: number;
  results: AutoTranslateItemResult[];
}

// ── 콘텐츠 목록 조회 DTO ──────────────────────

// 콘텐츠 레코드 개별 항목
export interface ContentRecordItem {
  id: number;
  label: string;
  detail?: string;
}

// 콘텐츠 목록 응답
export interface ContentRecordsRes {
  items: ContentRecordItem[];
}

// ── 원본 텍스트 조회 DTO ──────────────────────

// 원본 필드 개별 항목
export interface SourceFieldItem {
  content_type: ContentType;
  content_id: number;
  field_name: string;
  source_text?: string;
}

// 원본 필드 응답
export interface SourceFieldsRes {
  fields: SourceFieldItem[];
}

// ── 벌크 자동 번역 DTO ──────────────────────

// 벌크 자동 번역 개별 항목
export interface AutoTranslateBulkItem {
  content_type: ContentType;
  content_id: number;
  field_name: string;
  source_text: string;
}

// 벌크 자동 번역 요청
export interface AutoTranslateBulkReq {
  items: AutoTranslateBulkItem[];
  target_langs: SupportedLanguage[];
}

// 벌크 자동 번역 개별 결과
export interface AutoTranslateBulkItemResult {
  content_type: ContentType;
  content_id: number;
  field_name: string;
  lang: string;
  success: boolean;
  translation_id?: number;
  translated_text?: string;
  error?: string;
}

// 벌크 자동 번역 응답
export interface AutoTranslateBulkRes {
  total: number;
  success_count: number;
  fail_count: number;
  results: AutoTranslateBulkItemResult[];
}

// ── 번역 검색 DTO ──────────────────────────

export interface TranslationSearchItem {
  translation_id: number;
  content_type: ContentType;
  content_id: number;
  field_name: string;
  lang: SupportedLanguage;
  translated_text: string;
  status: TranslationStatus;
}

export interface TranslationSearchRes {
  items: TranslationSearchItem[];
}
