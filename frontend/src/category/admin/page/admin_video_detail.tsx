import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ArrowLeft, Save, ExternalLink } from "lucide-react";
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
import { Skeleton } from "@/components/ui/skeleton";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";

import {
  useAdminVideoDetail,
  useUpdateAdminVideo,
} from "../hook/use_admin_videos";
import {
  videoUpdateReqSchema,
  type VideoUpdateReq,
  type VideoAccess,
  type VideoState,
} from "../types";

export function AdminVideoDetail() {
  const { videoId } = useParams<{ videoId: string }>();
  const navigate = useNavigate();
  const id = Number(videoId);

  const { data: video, isLoading, isError } = useAdminVideoDetail(id);
  const updateMutation = useUpdateAdminVideo();

  const [cooldown, setCooldown] = useState(0);

  useEffect(() => {
    if (cooldown > 0) {
      const timer = setTimeout(() => setCooldown(cooldown - 1), 1000);
      return () => clearTimeout(timer);
    }
  }, [cooldown]);

  const form = useForm<VideoUpdateReq>({
    resolver: zodResolver(videoUpdateReqSchema),
    defaultValues: {
      video_tag_title: "",
      video_tag_subtitle: "",
      video_tag_key: "",
      video_url_vimeo: "",
      video_access: "private",
      video_state: "ready",
      video_idx: "",
    },
  });

  // video 데이터가 로드되면 폼 값 업데이트
  useEffect(() => {
    if (video) {
      form.reset({
        video_tag_title: video.title || "",
        video_tag_subtitle: video.description || "",
        video_tag_key: video.video_tag_key || "",
        video_url_vimeo: video.url || "",
        video_access: video.video_access || "private",
        video_state: video.video_state || "ready",
        video_idx: video.video_idx || "",
      });
    }
  }, [video, form]);

  const onSubmit = async (data: VideoUpdateReq) => {
    try {
      await updateMutation.mutateAsync({ id, data });
      toast.success("Video updated successfully");
      setCooldown(10);
      setTimeout(() => {
        navigate("/admin/videos");
      }, 1500);
    } catch {
      toast.error("Failed to update video");
    }
  };

  const isButtonDisabled = updateMutation.isPending || cooldown > 0;

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Card>
          <CardHeader>
            <Skeleton className="h-6 w-32" />
          </CardHeader>
          <CardContent className="space-y-4">
            {Array.from({ length: 6 }).map((_, i) => (
              <div key={i} className="space-y-2">
                <Skeleton className="h-4 w-20" />
                <Skeleton className="h-10 w-full" />
              </div>
            ))}
          </CardContent>
        </Card>
      </div>
    );
  }

  if (isError || !video) {
    return (
      <div className="space-y-4">
        <Button variant="ghost" onClick={() => navigate("/admin/videos")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Videos
        </Button>
        <p className="text-destructive">Video not found</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/videos")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Edit Video #{video.id}</h1>
      </div>

      <form onSubmit={form.handleSubmit(onSubmit, (errors) => {
        const errorFields = Object.keys(errors).join(", ");
        toast.error(`Please fix errors: ${errorFields}`);
      })}>
        <Card>
          <CardHeader>
            <CardTitle>Video Information</CardTitle>
            <CardDescription>
              Update video details, tags, and access settings
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2">
              {/* Title */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="video_tag_title">Title *</Label>
                <Input
                  id="video_tag_title"
                  placeholder="Video title"
                  {...form.register("video_tag_title")}
                />
                {form.formState.errors.video_tag_title && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.video_tag_title.message}
                  </p>
                )}
              </div>

              {/* Subtitle / Description */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="video_tag_subtitle">Subtitle / Description</Label>
                <Textarea
                  id="video_tag_subtitle"
                  placeholder="Video description"
                  rows={3}
                  {...form.register("video_tag_subtitle")}
                />
              </div>

              {/* Vimeo URL */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="video_url_vimeo">Vimeo URL *</Label>
                <div className="flex gap-2">
                  <Input
                    id="video_url_vimeo"
                    type="url"
                    placeholder="https://vimeo.com/..."
                    {...form.register("video_url_vimeo")}
                  />
                  {video.url && (
                    <Button
                      type="button"
                      variant="outline"
                      size="icon"
                      onClick={() => window.open(video.url!, "_blank")}
                    >
                      <ExternalLink className="h-4 w-4" />
                    </Button>
                  )}
                </div>
                {form.formState.errors.video_url_vimeo && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.video_url_vimeo.message}
                  </p>
                )}
              </div>

              {/* Video Index */}
              <div className="space-y-2">
                <Label htmlFor="video_idx">Video Index</Label>
                <Input
                  id="video_idx"
                  placeholder="e.g., V001"
                  maxLength={100}
                  {...form.register("video_idx")}
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
                  Unique identifier for this tag (max 30 characters)
                </p>
              </div>

              {/* State */}
              <div className="space-y-2">
                <Label>State</Label>
                <Select
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
                <Label>Access</Label>
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

              {/* Views (read-only) */}
              <div className="space-y-2">
                <Label>Views</Label>
                <Input
                  value={video.views.toLocaleString()}
                  disabled
                  className="bg-muted"
                />
              </div>

              {/* Updated By (read-only) */}
              <div className="space-y-2">
                <Label>Updated By User ID</Label>
                <Input
                  value={video.updated_by_user_id ?? "N/A"}
                  disabled
                  className="bg-muted"
                />
              </div>

              {/* Created At (read-only) */}
              <div className="space-y-2">
                <Label>Created At</Label>
                <Input
                  value={new Date(video.created_at).toLocaleString()}
                  disabled
                  className="bg-muted"
                />
              </div>

              {/* Updated At (read-only) */}
              <div className="space-y-2">
                <Label>Updated At</Label>
                <Input
                  value={new Date(video.updated_at).toLocaleString()}
                  disabled
                  className="bg-muted"
                />
              </div>
            </div>

            {/* Submit */}
            <div className="flex justify-end pt-4">
              <Button type="submit" disabled={isButtonDisabled}>
                <Save className="mr-2 h-4 w-4" />
                {updateMutation.isPending
                  ? "Saving..."
                  : cooldown > 0
                    ? `Wait ${cooldown}s`
                    : "Save Changes"}
              </Button>
            </div>
          </CardContent>
        </Card>
      </form>
    </div>
  );
}
