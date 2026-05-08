import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ArrowLeft, Video, Download, Clock, Loader2, Link, Upload } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

import { useCreateAdminVideo, useVimeoPreview } from "../hook/use_admin_videos";
import { videoCreateReqSchema, type VideoCreateReq, type VideoAccess, type VideoState, type VimeoPreviewRes } from "../types";
import { VimeoUploader } from "../components/vimeo_uploader";

// Duration formatting helper
function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  if (hours > 0) {
    return `${hours}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}

export function AdminVideoCreate() {
  const navigate = useNavigate();
  const createMutation = useCreateAdminVideo();
  const previewMutation = useVimeoPreview();

  const [vimeoPreview, setVimeoPreview] = useState<VimeoPreviewRes | null>(null);
  const [uploadMode, setUploadMode] = useState<"url" | "upload">("url");

  const form = useForm<VideoCreateReq>({
    resolver: zodResolver(videoCreateReqSchema),
    defaultValues: {
      video_idx: "",
      video_state: "ready",
      video_access: "private",
      // Q1c B (2026-04-22): video 테이블 물리 컬럼 필드.
      video_title: "",
      video_subtitle: "",
      video_tag_title: "",
      video_tag_subtitle: "",
      video_tag_key: "",
      video_url_vimeo: "",
    },
  });

  const handleFetchVimeo = async () => {
    const url = form.getValues("video_url_vimeo");
    if (!url) {
      toast.error("Vimeo URL을 입력해주세요");
      return;
    }

    try {
      const data = await previewMutation.mutateAsync(url);
      setVimeoPreview(data);

      // Auto-fill video + tag title/subtitle (Q1c B 이후 video 필드 우선 세팅,
      // tag 필드는 동일값으로 초기화하여 사용자가 필요 시 별도 편집).
      form.setValue("video_title", data.title);
      form.setValue("video_tag_title", data.title);
      if (data.description) {
        form.setValue("video_subtitle", data.description);
        form.setValue("video_tag_subtitle", data.description);
      }

      toast.success("Vimeo 정보를 불러왔습니다");
    } catch {
      toast.error("Vimeo 정보를 가져올 수 없습니다");
      setVimeoPreview(null);
    }
  };

  const onSubmit = async (data: VideoCreateReq) => {
    try {
      const submitData = {
        video_idx: data.video_idx || undefined,
        video_state: data.video_state || undefined,
        video_access: data.video_access,
        // Q1c B: video 테이블 물리 컬럼. 미입력 시 undefined 로 보내면 backend 가
        // video_tag_title 로 폴백 (backward-compat).
        video_title: data.video_title?.trim() || undefined,
        video_subtitle: data.video_subtitle?.trim() || undefined,
        video_tag_title: data.video_tag_title,
        video_tag_subtitle: data.video_tag_subtitle || undefined,
        video_tag_key: data.video_tag_key || undefined,
        video_url_vimeo: data.video_url_vimeo,
      };
      await createMutation.mutateAsync(submitData);
      toast.success("Video created successfully");
      navigate("/admin/videos");
    } catch {
      toast.error("Failed to create video");
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/videos")}>
          <ArrowLeft className="me-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Create New Video</h1>
      </div>

      <form
        onSubmit={form.handleSubmit(onSubmit, (errors) => {
          const errorFields = Object.keys(errors).join(", ");
          toast.error(`Please fill in required fields: ${errorFields}`);
        })}
      >
        {/* Step 1: Vimeo URL Input or Upload */}
        <Card className="mb-4">
          <CardHeader>
            <CardTitle>1. Vimeo 영상</CardTitle>
            <CardDescription>
              기존 Vimeo URL을 입력하거나 새 영상을 업로드하세요
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Tabs value={uploadMode} onValueChange={(v) => setUploadMode(v as "url" | "upload")}>
              <TabsList className="grid w-full grid-cols-2 mb-4">
                <TabsTrigger value="url" className="flex items-center gap-2">
                  <Link className="h-4 w-4" />
                  URL 입력
                </TabsTrigger>
                <TabsTrigger value="upload" className="flex items-center gap-2">
                  <Upload className="h-4 w-4" />
                  새 영상 업로드
                </TabsTrigger>
              </TabsList>

              <TabsContent value="url">
                <div className="flex gap-2">
                  <Input
                    id="video_url_vimeo"
                    type="url" dir="ltr"
                    placeholder="https://vimeo.com/123456789"
                    className="flex-1"
                    {...form.register("video_url_vimeo")}
                  />
                  <Button
                    type="button"
                    variant="secondary"
                    onClick={handleFetchVimeo}
                    disabled={previewMutation.isPending}
                  >
                    {previewMutation.isPending ? (
                      <Loader2 className="me-2 h-4 w-4 animate-spin" />
                    ) : (
                      <Download className="me-2 h-4 w-4" />
                    )}
                    불러오기
                  </Button>
                </div>
                {form.formState.errors.video_url_vimeo && (
                  <p className="text-sm text-destructive mt-2">
                    {form.formState.errors.video_url_vimeo.message}
                  </p>
                )}
              </TabsContent>

              <TabsContent value="upload">
                <VimeoUploader
                  onUploadComplete={async (vimeoVideoId) => {
                    // 업로드 완료 후 URL 설정 및 메타데이터 불러오기
                    const vimeoUrl = `https://vimeo.com/${vimeoVideoId}`;
                    form.setValue("video_url_vimeo", vimeoUrl);

                    // 메타데이터 불러오기 (약간의 지연 후 - Vimeo 처리 시간)
                    toast.success("업로드 완료! 메타데이터를 불러옵니다...");
                    setTimeout(async () => {
                      try {
                        const data = await previewMutation.mutateAsync(vimeoUrl);
                        setVimeoPreview(data);
                        form.setValue("video_tag_title", data.title);
                        if (data.description) {
                          form.setValue("video_tag_subtitle", data.description);
                        }
                      } catch {
                        toast.error("메타데이터를 가져오는데 실패했습니다. 잠시 후 다시 시도해주세요.");
                      }
                    }, 2000);
                  }}
                  onError={(error) => {
                    toast.error(`업로드 실패: ${error.message}`);
                  }}
                />
              </TabsContent>
            </Tabs>
          </CardContent>
        </Card>

        {/* Step 2: Vimeo Preview (shown after fetch) */}
        {vimeoPreview && (
          <Card className="mb-4 border-primary/20 bg-primary/5">
            <CardHeader>
              <CardTitle className="text-base">Vimeo 영상 정보</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="flex gap-4">
                {vimeoPreview.thumbnail_url && (
                  <div className="flex-shrink-0">
                    <img
                      src={vimeoPreview.thumbnail_url}
                      alt="Video thumbnail"
                      className="w-40 h-24 object-cover rounded-md"
                    />
                  </div>
                )}
                <div className="flex-1 space-y-2">
                  <div>
                    <span className="text-sm text-muted-foreground">제목:</span>
                    <p className="font-medium">{vimeoPreview.title}</p>
                  </div>
                  <div className="flex items-center gap-1 text-sm text-muted-foreground">
                    <Clock className="h-4 w-4" />
                    <span>
                      {formatDuration(vimeoPreview.duration)} ({vimeoPreview.duration}초)
                    </span>
                  </div>
                  {vimeoPreview.description && (
                    <div>
                      <span className="text-sm text-muted-foreground">설명:</span>
                      <p className="text-sm line-clamp-2">{vimeoPreview.description}</p>
                    </div>
                  )}
                  <div className="text-xs text-muted-foreground">
                    Vimeo ID: {vimeoPreview.vimeo_video_id}
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        )}

        {/* Step 3: Video Information */}
        <Card>
          <CardHeader>
            <CardTitle>2. Video Information</CardTitle>
            <CardDescription>
              영상 정보를 확인하고 필요시 수정하세요
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* Video 정보 (Q1c B, 2026-04-22) — video 테이블 물리 컬럼 */}
            <div className="grid gap-4 md:grid-cols-2">
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="video_title">Video Title</Label>
                <Input
                  id="video_title"
                  placeholder="e.g., 한글 자음 기초 1강"
                  maxLength={150}
                  {...form.register("video_title")}
                />
                <p className="text-sm text-muted-foreground">
                  비디오 자체 제목 (video.video_title 컬럼). 비워두면 Tag Title 로 폴백.
                </p>
                {form.formState.errors.video_title && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.video_title.message}
                  </p>
                )}
              </div>

              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="video_subtitle">Video Subtitle</Label>
                <Textarea
                  id="video_subtitle"
                  placeholder="비디오 부제목 또는 간단한 설명"
                  maxLength={250}
                  rows={2}
                  {...form.register("video_subtitle")}
                />
                <p className="text-sm text-muted-foreground">
                  비디오 부제목 (video.video_subtitle 컬럼, 선택). 비워두면 Tag Subtitle 폴백.
                </p>
              </div>

              {/* Video Index */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="video_idx">Video Index</Label>
                <Input
                  id="video_idx"
                  placeholder="e.g., V001"
                  maxLength={100}
                  {...form.register("video_idx")}
                />
                <p className="text-sm text-muted-foreground">
                  비즈니스 식별 코드 (video_idx)
                </p>
              </div>
            </div>

            {/* Tag 정보 — video_tag 테이블 (분류/검색용) */}
            <div className="pt-4 border-t">
              <h3 className="text-sm font-semibold mb-2">Tag Information</h3>
              <p className="text-xs text-muted-foreground mb-3">
                영상을 분류/검색하기 위한 태그 정보. Video Title/Subtitle 과
                다르게 관리할 수 있지만, 일반적으로 같은 값으로 두어도 됩니다.
              </p>
              <div className="grid gap-4 md:grid-cols-2">
                {/* Tag Title */}
                <div className="space-y-2 md:col-span-2">
                  <Label htmlFor="video_tag_title">Tag Title *</Label>
                  <Input
                    id="video_tag_title"
                    placeholder="Tag title (분류용)"
                    {...form.register("video_tag_title")}
                  />
                  {form.formState.errors.video_tag_title && (
                    <p className="text-sm text-destructive">
                      {form.formState.errors.video_tag_title.message}
                    </p>
                  )}
                </div>

                {/* Tag Subtitle */}
                <div className="space-y-2 md:col-span-2">
                  <Label htmlFor="video_tag_subtitle">Tag Subtitle</Label>
                  <Textarea
                    id="video_tag_subtitle"
                    placeholder="Tag subtitle (선택)"
                    rows={2}
                    {...form.register("video_tag_subtitle")}
                  />
                </div>

                {/* Tag Key */}
                <div className="space-y-2">
                  <Label htmlFor="video_tag_key">Tag Key</Label>
                  <Input
                    id="video_tag_key"
                    placeholder="e.g., lesson-01"
                    maxLength={30}
                    {...form.register("video_tag_key")}
                  />
                  <p className="text-sm text-muted-foreground">
                    Unique identifier (max 30 characters)
                  </p>
                </div>
              </div>
            </div>

            {/* Access & State */}
            <div className="pt-4 border-t grid gap-4 md:grid-cols-2">
              {/* State */}
              <div className="space-y-2">
                <Label>State</Label>
                <Select
                  // eslint-disable-next-line react-hooks/incompatible-library -- react-hook-form watch() 메모이제이션 불가, 라이브러리 한계
                  value={form.watch("video_state") ?? "ready"}
                  onValueChange={(value) =>
                    form.setValue("video_state", value as VideoState)
                  }
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select state" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="ready">Ready</SelectItem>
                    <SelectItem value="open">Open</SelectItem>
                    <SelectItem value="close">Close</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {/* Access */}
              <div className="space-y-2">
                <Label>Access *</Label>
                <Select
                  value={form.watch("video_access") ?? "private"}
                  onValueChange={(value) =>
                    form.setValue("video_access", value as VideoAccess)
                  }
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select access" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="public">Public</SelectItem>
                    <SelectItem value="paid">Paid</SelectItem>
                    <SelectItem value="private">Private</SelectItem>
                    <SelectItem value="promote">Promote</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>

            {/* Submit */}
            <div className="flex justify-end gap-2 pt-4">
              <Button
                type="button"
                variant="outline"
                onClick={() => navigate("/admin/videos")}
              >
                Cancel
              </Button>
              <Button type="submit" disabled={createMutation.isPending}>
                <Video className="me-2 h-4 w-4" />
                {createMutation.isPending ? "Creating..." : "Create Video"}
              </Button>
            </div>
          </CardContent>
        </Card>
      </form>
    </div>
  );
}
