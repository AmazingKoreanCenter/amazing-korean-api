import { useNavigate } from "react-router-dom";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ArrowLeft, Plus } from "lucide-react";
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

import { useCreateAdminLesson } from "../hook/use_admin_lessons";
import {
  lessonCreateReqSchema,
  type LessonCreateReq,
  type LessonState,
  type LessonAccess,
} from "../lesson/types";

export function AdminLessonCreate() {
  const navigate = useNavigate();
  const createMutation = useCreateAdminLesson();

  const form = useForm<LessonCreateReq>({
    resolver: zodResolver(lessonCreateReqSchema),
    defaultValues: {
      lesson_idx: "",
      lesson_title: "",
      lesson_subtitle: "",
      lesson_description: "",
      lesson_state: "ready",
      lesson_access: "public",
    },
  });

  const onSubmit = async (data: LessonCreateReq) => {
    try {
      const result = await createMutation.mutateAsync(data);
      toast.success("Lesson created successfully");
      navigate(`/admin/lessons/${result.lesson_id}`);
    } catch {
      toast.error("Failed to create lesson");
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/lessons")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Create New Lesson</h1>
      </div>

      <form
        onSubmit={form.handleSubmit(onSubmit, (errors) => {
          const errorFields = Object.keys(errors).join(", ");
          toast.error(`Please fix errors: ${errorFields}`);
        })}
      >
        <Card>
          <CardHeader>
            <CardTitle>Lesson Information</CardTitle>
            <CardDescription>Enter the details for the new lesson</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2">
              {/* Lesson IDX */}
              <div className="space-y-2">
                <Label htmlFor="lesson_idx">Lesson IDX *</Label>
                <Input
                  id="lesson_idx"
                  placeholder="e.g., LESSON-001"
                  {...form.register("lesson_idx")}
                />
                {form.formState.errors.lesson_idx && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.lesson_idx.message}
                  </p>
                )}
              </div>

              {/* Title */}
              <div className="space-y-2">
                <Label htmlFor="lesson_title">Title *</Label>
                <Input
                  id="lesson_title"
                  placeholder="Lesson title"
                  {...form.register("lesson_title")}
                />
                {form.formState.errors.lesson_title && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.lesson_title.message}
                  </p>
                )}
              </div>

              {/* Subtitle */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="lesson_subtitle">Subtitle</Label>
                <Input
                  id="lesson_subtitle"
                  placeholder="Lesson subtitle"
                  {...form.register("lesson_subtitle")}
                />
              </div>

              {/* Description */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="lesson_description">Description</Label>
                <Textarea
                  id="lesson_description"
                  placeholder="Lesson description"
                  rows={3}
                  {...form.register("lesson_description")}
                />
              </div>

              {/* State */}
              <div className="space-y-2">
                <Label>State</Label>
                <Select
                  value={form.watch("lesson_state") ?? "ready"}
                  onValueChange={(value) => form.setValue("lesson_state", value as LessonState)}
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
                  value={form.watch("lesson_access") ?? "public"}
                  onValueChange={(value) => form.setValue("lesson_access", value as LessonAccess)}
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
              <Button variant="outline" onClick={() => navigate("/admin/lessons")}>
                Cancel
              </Button>
              <Button type="submit" disabled={createMutation.isPending}>
                <Plus className="mr-2 h-4 w-4" />
                {createMutation.isPending ? "Creating..." : "Create Lesson"}
              </Button>
            </div>
          </CardContent>
        </Card>
      </form>
    </div>
  );
}
