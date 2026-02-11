import { useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ArrowLeft, Loader2 } from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";

import { SUPPORTED_LANGUAGES } from "@/i18n";
import { useTranslationDetail } from "../hook/use_translations";
import {
  useCreateTranslation,
  useUpdateTranslation,
} from "../hook/use_translation_mutations";
import {
  translationCreateReqSchema,
  translationUpdateReqSchema,
  type TranslationCreateReq,
  type TranslationUpdateReq,
  type ContentType,
  type TranslationStatus,
} from "../translation/types";

const CONTENT_TYPE_OPTIONS: { value: ContentType; label: string }[] = [
  { value: "course", label: "Course" },
  { value: "lesson", label: "Lesson" },
  { value: "video", label: "Video" },
  { value: "video_tag", label: "Video Tag" },
  { value: "study", label: "Study" },
];

const STATUS_OPTIONS: { value: TranslationStatus; label: string }[] = [
  { value: "draft", label: "Draft" },
  { value: "reviewed", label: "Reviewed" },
  { value: "approved", label: "Approved" },
];

const LANG_OPTIONS = SUPPORTED_LANGUAGES.filter((l) => l.code !== "ko");

// ── 생성 모드 ──────────────────────────────

function TranslationCreateForm() {
  const navigate = useNavigate();
  const createMutation = useCreateTranslation();

  const form = useForm<TranslationCreateReq>({
    resolver: zodResolver(translationCreateReqSchema),
    defaultValues: {
      content_type: "video",
      content_id: 0,
      field_name: "",
      lang: "en",
      translated_text: "",
    },
  });

  const onSubmit = (values: TranslationCreateReq) => {
    createMutation.mutate(values, {
      onSuccess: () => {
        navigate("/admin/translations");
      },
    });
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6 max-w-2xl">
        <FormField
          control={form.control}
          name="content_type"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Content Type</FormLabel>
              <Select value={field.value} onValueChange={field.onChange}>
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Select content type" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  {CONTENT_TYPE_OPTIONS.map((opt) => (
                    <SelectItem key={opt.value} value={opt.value}>
                      {opt.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="content_id"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Content ID</FormLabel>
              <FormControl>
                <Input
                  type="number"
                  value={field.value || ""}
                  onChange={(e) => field.onChange(Number(e.target.value))}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="field_name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Field Name</FormLabel>
              <FormControl>
                <Input placeholder="e.g. title, description" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="lang"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Language</FormLabel>
              <Select value={field.value} onValueChange={field.onChange}>
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Select language" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent className="max-h-60">
                  {LANG_OPTIONS.map((lang) => (
                    <SelectItem key={lang.code} value={lang.code}>
                      {lang.nativeName} ({lang.code})
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="translated_text"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Translated Text</FormLabel>
              <FormControl>
                <Textarea rows={5} placeholder="Enter translated text..." {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <div className="flex gap-3">
          <Button type="submit" disabled={createMutation.isPending}>
            {createMutation.isPending && <Loader2 className="w-4 h-4 mr-2 animate-spin" />}
            Create Translation
          </Button>
          <Button type="button" variant="outline" onClick={() => navigate("/admin/translations")}>
            Cancel
          </Button>
        </div>
      </form>
    </Form>
  );
}

// ── 수정 모드 ──────────────────────────────

function TranslationEditForm({ id }: { id: number }) {
  const navigate = useNavigate();
  const { data, isLoading } = useTranslationDetail(id);
  const updateMutation = useUpdateTranslation(id);

  const form = useForm<TranslationUpdateReq>({
    resolver: zodResolver(translationUpdateReqSchema),
    defaultValues: {
      translated_text: "",
      status: "draft",
    },
  });

  useEffect(() => {
    if (data) {
      form.reset({
        translated_text: data.translated_text,
        status: data.status,
      });
    }
  }, [data, form]);

  const onSubmit = (values: TranslationUpdateReq) => {
    updateMutation.mutate(values, {
      onSuccess: () => {
        navigate("/admin/translations");
      },
    });
  };

  if (isLoading) {
    return (
      <div className="space-y-4 max-w-2xl">
        {Array.from({ length: 4 }).map((_, i) => (
          <div key={i} className="space-y-2">
            <Skeleton className="h-4 w-24" />
            <Skeleton className="h-10 w-full" />
          </div>
        ))}
      </div>
    );
  }

  if (!data) {
    return <p className="text-red-500">Translation not found.</p>;
  }

  return (
    <div className="max-w-2xl">
      {/* 읽기 전용 메타 정보 */}
      <div className="mb-6 p-4 bg-gray-50 rounded-lg space-y-2 text-sm">
        <div className="flex gap-8">
          <span className="text-gray-500">Type:</span>
          <span className="font-medium">{data.content_type}</span>
        </div>
        <div className="flex gap-8">
          <span className="text-gray-500">Content ID:</span>
          <span className="font-mono">{data.content_id}</span>
        </div>
        <div className="flex gap-8">
          <span className="text-gray-500">Field:</span>
          <span className="font-mono">{data.field_name}</span>
        </div>
        <div className="flex gap-8">
          <span className="text-gray-500">Language:</span>
          <span>{SUPPORTED_LANGUAGES.find((l) => l.code === data.lang)?.nativeName ?? data.lang}</span>
        </div>
      </div>

      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
          <FormField
            control={form.control}
            name="translated_text"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Translated Text</FormLabel>
                <FormControl>
                  <Textarea rows={6} {...field} value={field.value ?? ""} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="status"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Status</FormLabel>
                <Select value={field.value ?? "draft"} onValueChange={field.onChange}>
                  <FormControl>
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                    {STATUS_OPTIONS.map((opt) => (
                      <SelectItem key={opt.value} value={opt.value}>
                        {opt.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <FormMessage />
              </FormItem>
            )}
          />

          <div className="flex gap-3">
            <Button type="submit" disabled={updateMutation.isPending}>
              {updateMutation.isPending && <Loader2 className="w-4 h-4 mr-2 animate-spin" />}
              Update Translation
            </Button>
            <Button type="button" variant="outline" onClick={() => navigate("/admin/translations")}>
              Cancel
            </Button>
          </div>
        </form>
      </Form>
    </div>
  );
}

// ── 메인 컴포넌트 ──────────────────────────

export function AdminTranslationEdit() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const isCreateMode = !id;

  return (
    <div>
      <div className="flex items-center gap-3 mb-6">
        <Button variant="ghost" size="sm" onClick={() => navigate("/admin/translations")}>
          <ArrowLeft className="w-4 h-4 mr-1" />
          Back
        </Button>
        <h2 className="text-2xl font-bold text-gray-900">
          {isCreateMode ? "New Translation" : "Edit Translation"}
        </h2>
      </div>

      {isCreateMode ? <TranslationCreateForm /> : <TranslationEditForm id={Number(id)} />}
    </div>
  );
}
