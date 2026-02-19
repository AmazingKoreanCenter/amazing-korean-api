import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ArrowLeft, Loader2, Check, ChevronRight } from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

import { SUPPORTED_LANGUAGES } from "@/i18n";
import {
  useTranslationDetail,
  useContentRecords,
  useSourceFields,
} from "../hook/use_translations";
import {
  useCreateTranslation,
  useUpdateTranslation,
  useAutoTranslateBulk,
} from "../hook/use_translation_mutations";
import {
  translationUpdateReqSchema,
  type TranslationUpdateReq,
  type ContentType,
  type TranslationStatus,
  type SupportedLanguage,
  type TopCategory,
  type StudySubType,
  type SourceFieldItem,
  type AutoTranslateBulkReq,
  type AutoTranslateBulkRes,
} from "../translation/types";
import {
  TOP_CATEGORIES,
  STUDY_SUB_TYPES,
  CONTENT_TYPE_LABELS,
} from "../translation/types";

const STATUS_OPTIONS: { value: TranslationStatus; label: string }[] = [
  { value: "draft", label: "Draft" },
  { value: "reviewed", label: "Reviewed" },
  { value: "approved", label: "Approved" },
];

const LANG_OPTIONS = SUPPORTED_LANGUAGES.filter((l) => l.code !== "ko");

// ── Step Indicator ──────────────────────────

function StepIndicator({ currentStep, totalSteps }: { currentStep: number; totalSteps: number }) {
  const labels = ["Content Type", "Record", "Fields", "Translate"];
  return (
    <div className="flex items-center gap-2 mb-6">
      {labels.slice(0, totalSteps).map((label, i) => {
        const step = i + 1;
        const isActive = step === currentStep;
        const isDone = step < currentStep;
        return (
          <div key={step} className="flex items-center gap-2">
            {i > 0 && <ChevronRight className="w-4 h-4 text-muted-foreground/70" />}
            <div
              className={`flex items-center gap-1.5 px-3 py-1 rounded-full text-sm font-medium ${
                isActive
                  ? "bg-primary/10 text-primary"
                  : isDone
                    ? "bg-status-success/10 text-status-success"
                    : "bg-muted text-muted-foreground/70"
              }`}
            >
              {isDone ? <Check className="w-3.5 h-3.5" /> : <span>{step}</span>}
              <span>{label}</span>
            </div>
          </div>
        );
      })}
    </div>
  );
}

// ── 생성 모드 — 위저드 ──────────────────────

function TranslationCreateWizard() {
  const navigate = useNavigate();
  const [step, setStep] = useState(1);

  // Step 1: Content Type
  const [topCategory, setTopCategory] = useState<TopCategory | null>(null);
  const [studySubType, setStudySubType] = useState<StudySubType | null>(null);

  // Step 2: Content Record
  const [selectedRecordId, setSelectedRecordId] = useState<number | null>(null);

  // Step 3: Field selection
  const [selectedFields, setSelectedFields] = useState<Set<string>>(new Set());

  // Step 4: Translation mode + langs
  const [translateMode, setTranslateMode] = useState<"auto" | "manual">("auto");
  const [selectedLangs, setSelectedLangs] = useState<Set<string>>(new Set());
  const [manualLang, setManualLang] = useState<string>("en");
  const [manualTexts, setManualTexts] = useState<Record<string, string>>({});
  const [bulkResult, setBulkResult] = useState<AutoTranslateBulkRes | null>(null);

  // Derived content_type
  const contentType: ContentType | undefined =
    topCategory === "study"
      ? studySubType ?? undefined
      : topCategory ?? undefined;

  // Hooks
  const contentRecords = useContentRecords(contentType);
  const sourceFields = useSourceFields(contentType, selectedRecordId ?? undefined);
  const autoTranslateBulk = useAutoTranslateBulk();
  const createTranslation = useCreateTranslation();

  // Reset downstream state when upstream changes
  const resetFromStep = (fromStep: number) => {
    if (fromStep <= 2) {
      setSelectedRecordId(null);
      setSelectedFields(new Set());
      setBulkResult(null);
    }
    if (fromStep <= 3) {
      setSelectedFields(new Set());
      setBulkResult(null);
    }
    if (fromStep <= 4) {
      setBulkResult(null);
    }
  };

  // ── Step 1: Content Type ──
  const handleTopCategory = (cat: TopCategory) => {
    setTopCategory(cat);
    setStudySubType(null);
    resetFromStep(2);
    if (cat !== "study") {
      setStep(2);
    }
  };

  const handleStudySubType = (sub: StudySubType) => {
    setStudySubType(sub);
    resetFromStep(2);
    setStep(2);
  };

  // ── Step 2: Record Selection ──
  const handleRecordSelect = (id: number) => {
    setSelectedRecordId(id);
    resetFromStep(3);
    setStep(3);
  };

  // ── Step 3: Field Toggle ──
  const toggleField = (key: string) => {
    setSelectedFields((prev) => {
      const next = new Set(prev);
      if (next.has(key)) next.delete(key);
      else next.add(key);
      return next;
    });
  };

  const selectAllFields = () => {
    if (!sourceFields.data) return;
    if (selectedFields.size === sourceFields.data.fields.length) {
      setSelectedFields(new Set());
    } else {
      setSelectedFields(new Set(sourceFields.data.fields.map(fieldKey)));
    }
  };

  // ── Step 4: Translation ──
  const toggleLang = (code: string) => {
    setSelectedLangs((prev) => {
      const next = new Set(prev);
      if (next.has(code)) next.delete(code);
      else next.add(code);
      return next;
    });
  };

  const selectAllLangs = () => {
    if (selectedLangs.size === LANG_OPTIONS.length) {
      setSelectedLangs(new Set());
    } else {
      setSelectedLangs(new Set(LANG_OPTIONS.map((l) => l.code)));
    }
  };

  const fieldKey = (f: SourceFieldItem) =>
    `${f.content_type}:${f.content_id}:${f.field_name}`;

  const getSelectedSourceFields = (): SourceFieldItem[] => {
    if (!sourceFields.data) return [];
    return sourceFields.data.fields.filter((f) => selectedFields.has(fieldKey(f)));
  };

  const handleAutoTranslate = () => {
    const fields = getSelectedSourceFields();
    if (fields.length === 0 || selectedLangs.size === 0) return;

    const req: AutoTranslateBulkReq = {
      items: fields.map((f) => ({
        content_type: f.content_type,
        content_id: f.content_id,
        field_name: f.field_name,
        source_text: f.source_text ?? "",
      })),
      target_langs: Array.from(selectedLangs) as SupportedLanguage[],
    };

    setBulkResult(null);
    autoTranslateBulk.mutate(req, {
      onSuccess: (res) => setBulkResult(res),
    });
  };

  const handleManualSave = () => {
    const fields = getSelectedSourceFields();
    if (fields.length === 0 || !manualLang) return;

    // 순차적으로 각 필드 저장
    let saved = 0;
    for (const f of fields) {
      const text = manualTexts[fieldKey(f)];
      if (!text?.trim()) continue;
      createTranslation.mutate(
        {
          content_type: f.content_type,
          content_id: f.content_id,
          field_name: f.field_name,
          lang: manualLang as SupportedLanguage,
          translated_text: text.trim(),
        },
        {
          onSuccess: () => {
            saved++;
            if (saved === fields.filter((ff) => manualTexts[fieldKey(ff)]?.trim()).length) {
              navigate("/admin/translations");
            }
          },
        },
      );
    }
  };

  return (
    <div className="max-w-3xl">
      <StepIndicator currentStep={step} totalSteps={4} />

      {/* ── Step 1: Content Type ── */}
      {step >= 1 && (
        <Card className={step === 1 ? "" : "opacity-75"}>
          <CardHeader className="pb-3">
            <CardTitle className="text-base">Step 1 — Content Type</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex gap-2 mb-3">
              {TOP_CATEGORIES.map((cat) => (
                <Button
                  key={cat.value}
                  variant={topCategory === cat.value ? "default" : "outline"}
                  size="sm"
                  onClick={() => handleTopCategory(cat.value)}
                >
                  {cat.label}
                </Button>
              ))}
            </div>

            {topCategory === "study" && (
              <div className="flex flex-wrap gap-2">
                {STUDY_SUB_TYPES.map((sub) => (
                  <Button
                    key={sub.value}
                    variant={studySubType === sub.value ? "default" : "outline"}
                    size="sm"
                    onClick={() => handleStudySubType(sub.value)}
                  >
                    {sub.label}
                  </Button>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* ── Step 2: Content Record ── */}
      {step >= 2 && contentType && (
        <Card className={`mt-4 ${step === 2 ? "" : "opacity-75"}`}>
          <CardHeader className="pb-3">
            <CardTitle className="text-base">
              Step 2 — Select {CONTENT_TYPE_LABELS[contentType] ?? contentType} Record
            </CardTitle>
          </CardHeader>
          <CardContent>
            {contentRecords.isLoading ? (
              <div className="space-y-2">
                {Array.from({ length: 3 }).map((_, i) => (
                  <Skeleton key={i} className="h-10 w-full" />
                ))}
              </div>
            ) : contentRecords.data && contentRecords.data.items.length === 0 ? (
              <p className="text-sm text-muted-foreground">No records found.</p>
            ) : (
              <div className="max-h-64 overflow-y-auto space-y-1">
                {contentRecords.data?.items.map((rec) => (
                  <button
                    key={rec.id}
                    className={`w-full text-left px-3 py-2 rounded-md text-sm transition-colors ${
                      selectedRecordId === rec.id
                        ? "bg-primary/5 border border-primary/20"
                        : "hover:bg-muted border border-transparent"
                    }`}
                    onClick={() => handleRecordSelect(rec.id)}
                  >
                    <span className="font-mono text-xs text-muted-foreground">#{rec.id}</span>
                    <span className="ml-2 font-medium">{rec.label}</span>
                    {rec.detail && (
                      <span className="ml-2 text-muted-foreground">{rec.detail}</span>
                    )}
                  </button>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* ── Step 3: Field Selection + Source Text ── */}
      {step >= 3 && selectedRecordId !== null && (
        <Card className={`mt-4 ${step === 3 ? "" : "opacity-75"}`}>
          <CardHeader className="pb-3">
            <div className="flex items-center justify-between">
              <CardTitle className="text-base">
                Step 3 — Select Fields ({selectedFields.size})
              </CardTitle>
              <Button variant="ghost" size="sm" onClick={selectAllFields}>
                {sourceFields.data && selectedFields.size === sourceFields.data.fields.length
                  ? "Deselect All"
                  : "Select All"}
              </Button>
            </div>
          </CardHeader>
          <CardContent>
            {sourceFields.isLoading ? (
              <div className="space-y-2">
                {Array.from({ length: 3 }).map((_, i) => (
                  <Skeleton key={i} className="h-16 w-full" />
                ))}
              </div>
            ) : sourceFields.data && sourceFields.data.fields.length === 0 ? (
              <p className="text-sm text-muted-foreground">No translatable fields found.</p>
            ) : (
              <div className="space-y-2 max-h-80 overflow-y-auto">
                {sourceFields.data?.fields.map((f) => {
                  const key = fieldKey(f);
                  const isNumeric = f.source_text ? !isNaN(Number(f.source_text.trim())) : false;
                  return (
                    <label
                      key={key}
                      className={`flex items-start gap-3 p-3 rounded-md border cursor-pointer transition-colors ${
                        selectedFields.has(key)
                          ? "border-primary/20 bg-primary/5"
                          : "border-border hover:bg-muted"
                      }`}
                    >
                      <Checkbox
                        checked={selectedFields.has(key)}
                        onCheckedChange={() => toggleField(key)}
                        className="mt-0.5"
                      />
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2">
                          <span className="text-sm font-medium">{f.field_name}</span>
                          {f.content_type !== contentType && (
                            <Badge variant="outline" className="text-xs">
                              {CONTENT_TYPE_LABELS[f.content_type] ?? f.content_type}
                            </Badge>
                          )}
                          {isNumeric && (
                            <Badge variant="secondary" className="text-xs">
                              Numeric
                            </Badge>
                          )}
                        </div>
                        <p className="text-xs text-muted-foreground mt-1 truncate">
                          {f.source_text || <span className="italic">No source text</span>}
                        </p>
                      </div>
                    </label>
                  );
                })}
              </div>
            )}

            {selectedFields.size > 0 && step === 3 && (
              <Button className="mt-4" onClick={() => setStep(4)}>
                Next — Choose Translation Mode
              </Button>
            )}
          </CardContent>
        </Card>
      )}

      {/* ── Step 4: Translation Mode ── */}
      {step >= 4 && (
        <Card className="mt-4">
          <CardHeader className="pb-3">
            <CardTitle className="text-base">Step 4 — Translate</CardTitle>
          </CardHeader>
          <CardContent>
            {/* Mode Tabs */}
            <div className="flex gap-2 mb-4">
              <Button
                variant={translateMode === "auto" ? "default" : "outline"}
                size="sm"
                onClick={() => setTranslateMode("auto")}
              >
                Auto Translate
              </Button>
              <Button
                variant={translateMode === "manual" ? "default" : "outline"}
                size="sm"
                onClick={() => setTranslateMode("manual")}
              >
                Manual Input
              </Button>
            </div>

            {/* ── Auto Translate ── */}
            {translateMode === "auto" && (
              <div className="space-y-4">
                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <Label>Target Languages ({selectedLangs.size})</Label>
                    <Button type="button" variant="ghost" size="sm" onClick={selectAllLangs}>
                      {selectedLangs.size === LANG_OPTIONS.length ? "Deselect All" : "Select All"}
                    </Button>
                  </div>
                  <div className="grid grid-cols-3 gap-2 max-h-48 overflow-y-auto border rounded-md p-3">
                    {LANG_OPTIONS.map((lang) => (
                      <label key={lang.code} className="flex items-center gap-2 text-sm cursor-pointer">
                        <Checkbox
                          checked={selectedLangs.has(lang.code)}
                          onCheckedChange={() => toggleLang(lang.code)}
                        />
                        <span>
                          {lang.nativeName}{" "}
                          <span className="text-muted-foreground/70 text-xs">({lang.code})</span>
                        </span>
                      </label>
                    ))}
                  </div>
                </div>

                <div className="bg-muted rounded-md p-3 text-sm text-muted-foreground">
                  {selectedFields.size} field(s) x {selectedLangs.size} language(s) ={" "}
                  <strong>{selectedFields.size * selectedLangs.size}</strong> translation(s)
                </div>

                <Button
                  className="w-full"
                  disabled={selectedLangs.size === 0 || autoTranslateBulk.isPending}
                  onClick={handleAutoTranslate}
                >
                  {autoTranslateBulk.isPending && <Loader2 className="w-4 h-4 mr-2 animate-spin" />}
                  {autoTranslateBulk.isPending
                    ? "Translating..."
                    : `Auto Translate ${selectedFields.size * selectedLangs.size} items`}
                </Button>

                {/* Result */}
                {bulkResult && (
                  <div className="border rounded-md p-3 space-y-2">
                    <p className="text-sm font-medium">
                      Result: {bulkResult.success_count}/{bulkResult.total} success
                      {bulkResult.fail_count > 0 && (
                        <span className="text-destructive ml-2">({bulkResult.fail_count} failed)</span>
                      )}
                    </p>
                    <div className="space-y-1 max-h-48 overflow-y-auto">
                      {bulkResult.results.map((r, i) => (
                        <div
                          key={i}
                          className={`text-xs px-2 py-1 rounded ${
                            r.success ? "bg-status-success/10 text-status-success" : "bg-destructive/10 text-destructive"
                          }`}
                        >
                          <span className="font-mono font-medium">{r.lang}</span>
                          <span className="mx-1 text-muted-foreground/70">|</span>
                          <span>{r.field_name}</span>
                          {r.success ? (
                            <span className="ml-2 text-muted-foreground">
                              {r.translated_text && r.translated_text.length > 40
                                ? r.translated_text.slice(0, 40) + "..."
                                : r.translated_text}
                            </span>
                          ) : (
                            <span className="ml-2">{r.error}</span>
                          )}
                        </div>
                      ))}
                    </div>
                    <Button
                      variant="outline"
                      className="w-full mt-2"
                      onClick={() => navigate("/admin/translations")}
                    >
                      Done — Back to List
                    </Button>
                  </div>
                )}
              </div>
            )}

            {/* ── Manual Input ── */}
            {translateMode === "manual" && (
              <div className="space-y-4">
                <div className="space-y-1.5">
                  <Label>Target Language</Label>
                  <Select value={manualLang} onValueChange={setManualLang}>
                    <SelectTrigger className="w-56">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent className="max-h-60">
                      {LANG_OPTIONS.map((lang) => (
                        <SelectItem key={lang.code} value={lang.code}>
                          {lang.nativeName} ({lang.code})
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>

                <div className="space-y-3 max-h-[400px] overflow-y-auto">
                  {getSelectedSourceFields().map((f) => {
                    const key = fieldKey(f);
                    return (
                      <div key={key} className="border rounded-md p-3 space-y-2">
                        <div className="flex items-center gap-2">
                          <span className="text-sm font-medium">{f.field_name}</span>
                          {f.content_type !== contentType && (
                            <Badge variant="outline" className="text-xs">
                              {CONTENT_TYPE_LABELS[f.content_type] ?? f.content_type}
                            </Badge>
                          )}
                        </div>
                        <p className="text-xs text-muted-foreground">
                          Source: {f.source_text || "N/A"}
                        </p>
                        <Textarea
                          rows={2}
                          placeholder="Enter translation..."
                          value={manualTexts[key] ?? ""}
                          onChange={(e) =>
                            setManualTexts((prev) => ({ ...prev, [key]: e.target.value }))
                          }
                        />
                      </div>
                    );
                  })}
                </div>

                <Button
                  className="w-full"
                  disabled={createTranslation.isPending}
                  onClick={handleManualSave}
                >
                  {createTranslation.isPending && <Loader2 className="w-4 h-4 mr-2 animate-spin" />}
                  Save Translations
                </Button>
              </div>
            )}
          </CardContent>
        </Card>
      )}
    </div>
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
    return <p className="text-destructive">Translation not found.</p>;
  }

  return (
    <div className="max-w-2xl">
      {/* 읽기 전용 메타 정보 */}
      <div className="mb-6 p-4 bg-muted rounded-lg space-y-2 text-sm">
        <div className="flex gap-8">
          <span className="text-muted-foreground">Type:</span>
          <span className="font-medium">
            {CONTENT_TYPE_LABELS[data.content_type] ?? data.content_type}
          </span>
        </div>
        <div className="flex gap-8">
          <span className="text-muted-foreground">Content ID:</span>
          <span className="font-mono">{data.content_id}</span>
        </div>
        <div className="flex gap-8">
          <span className="text-muted-foreground">Field:</span>
          <span className="font-mono">{data.field_name}</span>
        </div>
        <div className="flex gap-8">
          <span className="text-muted-foreground">Language:</span>
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
        <h2 className="text-2xl font-bold text-foreground">
          {isCreateMode ? "New Translation" : "Edit Translation"}
        </h2>
      </div>

      {isCreateMode ? <TranslationCreateWizard /> : <TranslationEditForm id={Number(id)} />}
    </div>
  );
}
