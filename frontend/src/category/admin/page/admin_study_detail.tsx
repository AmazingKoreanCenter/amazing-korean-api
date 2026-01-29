import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ArrowLeft, Save, ListTodo, ChevronDown, ChevronRight, Loader2, Plus, Pencil, Globe, Users } from "lucide-react";
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
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

import { Checkbox } from "@/components/ui/checkbox";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

import {
  useAdminStudyDetail,
  useUpdateAdminStudy,
  useAdminStudyTaskDetail,
  useAdminTaskExplains,
  useCreateAdminTaskExplain,
  useUpdateAdminTaskExplain,
  useAdminTaskStatus,
  useUpdateAdminTaskStatus,
  useUpdateAdminStudyTasksBulk,
  useCreateAdminTaskExplainsBulk,
  useUpdateAdminTaskExplainsBulk,
  useUpdateAdminTaskStatusBulk,
} from "../hook/use_admin_studies";
import {
  studyUpdateReqSchema,
  type StudyUpdateReq,
  type StudyState,
  type StudyAccess,
  type AdminTaskExplainRes,
  type AdminTaskStatusRes,
} from "../types";
import type { StudyProgram } from "../../study/types";
import type { UserSetLanguage } from "../study/types";

export function AdminStudyDetail() {
  const { studyId } = useParams<{ studyId: string }>();
  const navigate = useNavigate();
  const id = Number(studyId);

  const { data: study, isLoading, isError, refetch } = useAdminStudyDetail(id);
  const updateMutation = useUpdateAdminStudy();
  const bulkUpdateTasksMutation = useUpdateAdminStudyTasksBulk();
  const bulkCreateExplainsMutation = useCreateAdminTaskExplainsBulk();
  const bulkUpdateExplainsMutation = useUpdateAdminTaskExplainsBulk();
  const bulkUpdateStatusMutation = useUpdateAdminTaskStatusBulk();

  const [cooldown, setCooldown] = useState(0);
  const [expandedTaskId, setExpandedTaskId] = useState<number | null>(null);
  const [selectedTaskIds, setSelectedTaskIds] = useState<Set<number>>(new Set());

  // Bulk Edit Tasks Dialog
  const [isBulkEditDialogOpen, setIsBulkEditDialogOpen] = useState(false);
  const [bulkEditFormData, setBulkEditFormData] = useState({
    question: "",
    answer: "",
  });

  // Bulk Add Explains Dialog
  const [isBulkAddExplainsDialogOpen, setIsBulkAddExplainsDialogOpen] = useState(false);
  const [bulkAddExplainsFormData, setBulkAddExplainsFormData] = useState({
    explain_lang: "ko" as UserSetLanguage,
    explain_title: "",
    explain_text: "",
    explain_media_url: "",
  });

  // Bulk Update Explains Dialog
  const [isBulkUpdateExplainsDialogOpen, setIsBulkUpdateExplainsDialogOpen] = useState(false);
  const [bulkUpdateExplainsFormData, setBulkUpdateExplainsFormData] = useState({
    explain_lang: "ko" as UserSetLanguage,
    explain_title: "",
    explain_text: "",
    explain_media_url: "",
  });

  // Bulk Update Status Dialog
  const [isBulkUpdateStatusDialogOpen, setIsBulkUpdateStatusDialogOpen] = useState(false);
  const [bulkUpdateStatusFormData, setBulkUpdateStatusFormData] = useState({
    user_id: 0,
    study_task_status_try_count: 0,
    study_task_status_is_solved: false,
  });

  useEffect(() => {
    if (cooldown > 0) {
      const timer = setTimeout(() => setCooldown(cooldown - 1), 1000);
      return () => clearTimeout(timer);
    }
  }, [cooldown]);

  const toggleTaskExpansion = (taskId: number) => {
    setExpandedTaskId(expandedTaskId === taskId ? null : taskId);
  };

  const toggleTaskSelection = (taskId: number) => {
    setSelectedTaskIds((prev) => {
      const next = new Set(prev);
      if (next.has(taskId)) {
        next.delete(taskId);
      } else {
        next.add(taskId);
      }
      return next;
    });
  };

  const toggleAllTasks = () => {
    if (!study) return;
    if (selectedTaskIds.size === study.tasks.length) {
      setSelectedTaskIds(new Set());
    } else {
      setSelectedTaskIds(new Set(study.tasks.map((t) => t.study_task_id)));
    }
  };

  const openBulkEditDialog = () => {
    setBulkEditFormData({ question: "", answer: "" });
    setIsBulkEditDialogOpen(true);
  };

  const handleBulkEditSubmit = async () => {
    if (selectedTaskIds.size === 0) return;

    const items = Array.from(selectedTaskIds).map((taskId) => ({
      study_task_id: taskId,
      question: bulkEditFormData.question || undefined,
      answer: bulkEditFormData.answer || undefined,
    }));

    try {
      const result = await bulkUpdateTasksMutation.mutateAsync({ items });
      toast.success(`Updated ${result.success_count} tasks`);
      if (result.failure_count > 0) {
        toast.warning(`${result.failure_count} tasks failed to update`);
      }
      setIsBulkEditDialogOpen(false);
      setSelectedTaskIds(new Set());
      refetch();
    } catch {
      toast.error("Bulk update failed");
    }
  };

  // Bulk Add Explains handlers
  const openBulkAddExplainsDialog = () => {
    setBulkAddExplainsFormData({
      explain_lang: "ko",
      explain_title: "",
      explain_text: "",
      explain_media_url: "",
    });
    setIsBulkAddExplainsDialogOpen(true);
  };

  const handleBulkAddExplainsSubmit = async () => {
    if (selectedTaskIds.size === 0) return;

    const items = Array.from(selectedTaskIds).map((taskId) => ({
      study_task_id: taskId,
      explain_lang: bulkAddExplainsFormData.explain_lang,
      explain_title: bulkAddExplainsFormData.explain_title || undefined,
      explain_text: bulkAddExplainsFormData.explain_text || undefined,
      explain_media_url: bulkAddExplainsFormData.explain_media_url || undefined,
    }));

    try {
      const result = await bulkCreateExplainsMutation.mutateAsync({ items });
      toast.success(`Created explains for ${result.success_count} tasks`);
      if (result.failure_count > 0) {
        toast.warning(`${result.failure_count} tasks failed`);
      }
      setIsBulkAddExplainsDialogOpen(false);
      setSelectedTaskIds(new Set());
      refetch();
    } catch {
      toast.error("Bulk create explains failed");
    }
  };

  // Bulk Update Explains handlers
  const openBulkUpdateExplainsDialog = () => {
    setBulkUpdateExplainsFormData({
      explain_lang: "ko",
      explain_title: "",
      explain_text: "",
      explain_media_url: "",
    });
    setIsBulkUpdateExplainsDialogOpen(true);
  };

  const handleBulkUpdateExplainsSubmit = async () => {
    if (selectedTaskIds.size === 0) return;

    const items = Array.from(selectedTaskIds).map((taskId) => ({
      study_task_id: taskId,
      explain_lang: bulkUpdateExplainsFormData.explain_lang,
      explain_title: bulkUpdateExplainsFormData.explain_title || undefined,
      explain_text: bulkUpdateExplainsFormData.explain_text || undefined,
      explain_media_url: bulkUpdateExplainsFormData.explain_media_url || undefined,
    }));

    try {
      const result = await bulkUpdateExplainsMutation.mutateAsync({ items });
      toast.success(`Updated explains for ${result.success_count} tasks`);
      if (result.failure_count > 0) {
        toast.warning(`${result.failure_count} tasks failed`);
      }
      setIsBulkUpdateExplainsDialogOpen(false);
      setSelectedTaskIds(new Set());
      refetch();
    } catch {
      toast.error("Bulk update explains failed");
    }
  };

  // Bulk Update Status handlers
  const openBulkUpdateStatusDialog = () => {
    setBulkUpdateStatusFormData({
      user_id: 0,
      study_task_status_try_count: 0,
      study_task_status_is_solved: false,
    });
    setIsBulkUpdateStatusDialogOpen(true);
  };

  const handleBulkUpdateStatusSubmit = async () => {
    if (selectedTaskIds.size === 0) return;
    if (bulkUpdateStatusFormData.user_id <= 0) {
      toast.error("Please enter a valid User ID");
      return;
    }

    const items = Array.from(selectedTaskIds).map((taskId) => ({
      study_task_id: taskId,
      user_id: bulkUpdateStatusFormData.user_id,
      study_task_status_try_count: bulkUpdateStatusFormData.study_task_status_try_count,
      study_task_status_is_solved: bulkUpdateStatusFormData.study_task_status_is_solved,
    }));

    try {
      const result = await bulkUpdateStatusMutation.mutateAsync({ items });
      toast.success(`Updated status for ${result.success_count} tasks`);
      if (result.failure_count > 0) {
        toast.warning(`${result.failure_count} tasks failed`);
      }
      setIsBulkUpdateStatusDialogOpen(false);
      setSelectedTaskIds(new Set());
      refetch();
    } catch {
      toast.error("Bulk update status failed");
    }
  };

  const form = useForm<StudyUpdateReq>({
    resolver: zodResolver(studyUpdateReqSchema),
    defaultValues: {
      study_idx: "",
      study_title: "",
      study_subtitle: "",
      study_description: "",
      study_program: "tbc",
      study_state: "ready",
      study_access: "public",
    },
  });

  useEffect(() => {
    if (study) {
      form.reset({
        study_idx: study.study_idx || "",
        study_title: study.study_title || "",
        study_subtitle: study.study_subtitle || "",
        study_description: study.study_description || "",
        study_program: study.study_program || "tbc",
        study_state: study.study_state || "ready",
        study_access: study.study_access || "public",
      });
    }
  }, [study, form]);

  const onSubmit = async (data: StudyUpdateReq) => {
    try {
      await updateMutation.mutateAsync({ id, data });
      toast.success("Study updated successfully");
      setCooldown(10);
      setTimeout(() => {
        navigate("/admin/studies");
      }, 1500);
    } catch {
      toast.error("Failed to update study");
    }
  };

  const isButtonDisabled = updateMutation.isPending || cooldown > 0;

  const getTaskKindBadgeVariant = (kind: string) => {
    switch (kind) {
      case "choice":
        return "default" as const;
      case "typing":
        return "secondary" as const;
      case "voice":
        return "destructive" as const;
      default:
        return "outline" as const;
    }
  };

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

  if (isError || !study) {
    return (
      <div className="space-y-4">
        <Button variant="ghost" onClick={() => navigate("/admin/studies")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Studies
        </Button>
        <p className="text-destructive">Study not found</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/studies")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Edit Study #{study.study_id}</h1>
      </div>

      <form onSubmit={form.handleSubmit(onSubmit, (errors) => {
        const errorFields = Object.keys(errors).join(", ");
        toast.error(`Please fix errors: ${errorFields}`);
      })}>
        <Card>
          <CardHeader>
            <CardTitle>Study Information</CardTitle>
            <CardDescription>
              Update study details and settings
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2">
              {/* Study IDX */}
              <div className="space-y-2">
                <Label htmlFor="study_idx">Study IDX *</Label>
                <Input
                  id="study_idx"
                  placeholder="e.g., TBC-001"
                  {...form.register("study_idx")}
                />
                {form.formState.errors.study_idx && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.study_idx.message}
                  </p>
                )}
              </div>

              {/* Program */}
              <div className="space-y-2">
                <Label>Program</Label>
                <Select
                  value={form.watch("study_program") ?? "tbc"}
                  onValueChange={(value) =>
                    form.setValue("study_program", value as StudyProgram)
                  }
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select program" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="basic_pronunciation">Basic Pronunciation</SelectItem>
                    <SelectItem value="basic_word">Basic Word</SelectItem>
                    <SelectItem value="basic_900">Basic 900</SelectItem>
                    <SelectItem value="topik_read">TOPIK Read</SelectItem>
                    <SelectItem value="topik_listen">TOPIK Listen</SelectItem>
                    <SelectItem value="topik_write">TOPIK Write</SelectItem>
                    <SelectItem value="tbc">TBC</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {/* Title */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="study_title">Title</Label>
                <Input
                  id="study_title"
                  placeholder="Study title"
                  maxLength={80}
                  {...form.register("study_title")}
                />
                {form.formState.errors.study_title && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.study_title.message}
                  </p>
                )}
              </div>

              {/* Subtitle */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="study_subtitle">Subtitle</Label>
                <Input
                  id="study_subtitle"
                  placeholder="Study subtitle"
                  maxLength={120}
                  {...form.register("study_subtitle")}
                />
              </div>

              {/* Description */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="study_description">Description</Label>
                <Textarea
                  id="study_description"
                  placeholder="Study description"
                  rows={3}
                  {...form.register("study_description")}
                />
              </div>

              {/* State */}
              <div className="space-y-2">
                <Label>State</Label>
                <Select
                  value={form.watch("study_state") ?? "ready"}
                  onValueChange={(value) =>
                    form.setValue("study_state", value as StudyState)
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
                  value={form.watch("study_access") ?? "public"}
                  onValueChange={(value) =>
                    form.setValue("study_access", value as StudyAccess)
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

              {/* Task Count (read-only) */}
              <div className="space-y-2">
                <Label>Task Count</Label>
                <Input
                  value={study.task_count}
                  disabled
                  className="bg-muted"
                />
              </div>

              {/* Created At (read-only) */}
              <div className="space-y-2">
                <Label>Created At</Label>
                <Input
                  value={new Date(study.study_created_at).toLocaleString()}
                  disabled
                  className="bg-muted"
                />
              </div>

              {/* Updated At (read-only) */}
              <div className="space-y-2">
                <Label>Updated At</Label>
                <Input
                  value={new Date(study.study_updated_at).toLocaleString()}
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

      {/* Tasks List */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle className="flex items-center gap-2">
                <ListTodo className="h-5 w-5" />
                Tasks ({study.task_count})
              </CardTitle>
              <CardDescription>
                Tasks associated with this study
              </CardDescription>
            </div>
            {study.tasks.length > 0 && (
              <div className="flex items-center gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={toggleAllTasks}
                >
                  {selectedTaskIds.size === study.tasks.length ? "Deselect All" : "Select All"}
                </Button>
                {selectedTaskIds.size > 0 && (
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button size="sm">
                        Bulk Actions ({selectedTaskIds.size})
                        <ChevronDown className="h-4 w-4 ml-1" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuLabel>Bulk Actions</DropdownMenuLabel>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem onClick={openBulkEditDialog}>
                        <Pencil className="h-4 w-4 mr-2" />
                        Edit Tasks
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem onClick={openBulkAddExplainsDialog}>
                        <Plus className="h-4 w-4 mr-2" />
                        Add Explains
                      </DropdownMenuItem>
                      <DropdownMenuItem onClick={openBulkUpdateExplainsDialog}>
                        <Globe className="h-4 w-4 mr-2" />
                        Update Explains
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem onClick={openBulkUpdateStatusDialog}>
                        <Users className="h-4 w-4 mr-2" />
                        Update Status
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                )}
              </div>
            )}
          </div>
        </CardHeader>
        <CardContent>
          {study.tasks.length === 0 ? (
            <p className="text-muted-foreground text-center py-4">
              No tasks found for this study
            </p>
          ) : (
            <div className="space-y-2">
              {study.tasks.map((task) => (
                <TaskExpandableRow
                  key={task.study_task_id}
                  task={task}
                  isExpanded={expandedTaskId === task.study_task_id}
                  onToggle={() => toggleTaskExpansion(task.study_task_id)}
                  getTaskKindBadgeVariant={getTaskKindBadgeVariant}
                  isSelected={selectedTaskIds.has(task.study_task_id)}
                  onSelectionChange={() => toggleTaskSelection(task.study_task_id)}
                />
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Bulk Edit Dialog */}
      <Dialog open={isBulkEditDialogOpen} onOpenChange={setIsBulkEditDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Edit Tasks</DialogTitle>
            <DialogDescription>
              Update {selectedTaskIds.size} selected task(s). Leave fields empty to keep current values.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">
            <div className="space-y-2">
              <Label>Question (optional)</Label>
              <Textarea
                value={bulkEditFormData.question}
                onChange={(e) =>
                  setBulkEditFormData({ ...bulkEditFormData, question: e.target.value })
                }
                placeholder="Leave empty to keep current values"
                rows={3}
              />
            </div>

            <div className="space-y-2">
              <Label>Answer (optional)</Label>
              <Textarea
                value={bulkEditFormData.answer}
                onChange={(e) =>
                  setBulkEditFormData({ ...bulkEditFormData, answer: e.target.value })
                }
                placeholder="Leave empty to keep current values"
                rows={2}
              />
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setIsBulkEditDialogOpen(false)}>
              Cancel
            </Button>
            <Button
              onClick={handleBulkEditSubmit}
              disabled={bulkUpdateTasksMutation.isPending || (!bulkEditFormData.question && !bulkEditFormData.answer)}
            >
              {bulkUpdateTasksMutation.isPending ? (
                <>
                  <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  Updating...
                </>
              ) : (
                `Update ${selectedTaskIds.size} Tasks`
              )}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Bulk Add Explains Dialog */}
      <Dialog open={isBulkAddExplainsDialogOpen} onOpenChange={setIsBulkAddExplainsDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Add Explains</DialogTitle>
            <DialogDescription>
              Add the same explanation to {selectedTaskIds.size} selected task(s).
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">
            <div className="space-y-2">
              <Label>Language</Label>
              <Select
                value={bulkAddExplainsFormData.explain_lang}
                onValueChange={(value) =>
                  setBulkAddExplainsFormData({ ...bulkAddExplainsFormData, explain_lang: value as UserSetLanguage })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="ko">Korean (ko)</SelectItem>
                  <SelectItem value="en">English (en)</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Title</Label>
              <Input
                value={bulkAddExplainsFormData.explain_title}
                onChange={(e) =>
                  setBulkAddExplainsFormData({ ...bulkAddExplainsFormData, explain_title: e.target.value })
                }
                placeholder="Explanation title"
              />
            </div>

            <div className="space-y-2">
              <Label>Text</Label>
              <Textarea
                value={bulkAddExplainsFormData.explain_text}
                onChange={(e) =>
                  setBulkAddExplainsFormData({ ...bulkAddExplainsFormData, explain_text: e.target.value })
                }
                placeholder="Explanation content"
                rows={4}
              />
            </div>

            <div className="space-y-2">
              <Label>Media URL (optional)</Label>
              <Input
                value={bulkAddExplainsFormData.explain_media_url}
                onChange={(e) =>
                  setBulkAddExplainsFormData({ ...bulkAddExplainsFormData, explain_media_url: e.target.value })
                }
                placeholder="https://..."
              />
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setIsBulkAddExplainsDialogOpen(false)}>
              Cancel
            </Button>
            <Button
              onClick={handleBulkAddExplainsSubmit}
              disabled={bulkCreateExplainsMutation.isPending}
            >
              {bulkCreateExplainsMutation.isPending ? (
                <>
                  <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  Creating...
                </>
              ) : (
                `Add Explains to ${selectedTaskIds.size} Tasks`
              )}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Bulk Update Explains Dialog */}
      <Dialog open={isBulkUpdateExplainsDialogOpen} onOpenChange={setIsBulkUpdateExplainsDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Update Explains</DialogTitle>
            <DialogDescription>
              Update explains for {selectedTaskIds.size} selected task(s). Leave fields empty to keep current values.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">
            <div className="space-y-2">
              <Label>Language (target)</Label>
              <Select
                value={bulkUpdateExplainsFormData.explain_lang}
                onValueChange={(value) =>
                  setBulkUpdateExplainsFormData({ ...bulkUpdateExplainsFormData, explain_lang: value as UserSetLanguage })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="ko">Korean (ko)</SelectItem>
                  <SelectItem value="en">English (en)</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Title (optional)</Label>
              <Input
                value={bulkUpdateExplainsFormData.explain_title}
                onChange={(e) =>
                  setBulkUpdateExplainsFormData({ ...bulkUpdateExplainsFormData, explain_title: e.target.value })
                }
                placeholder="Leave empty to keep current"
              />
            </div>

            <div className="space-y-2">
              <Label>Text (optional)</Label>
              <Textarea
                value={bulkUpdateExplainsFormData.explain_text}
                onChange={(e) =>
                  setBulkUpdateExplainsFormData({ ...bulkUpdateExplainsFormData, explain_text: e.target.value })
                }
                placeholder="Leave empty to keep current"
                rows={4}
              />
            </div>

            <div className="space-y-2">
              <Label>Media URL (optional)</Label>
              <Input
                value={bulkUpdateExplainsFormData.explain_media_url}
                onChange={(e) =>
                  setBulkUpdateExplainsFormData({ ...bulkUpdateExplainsFormData, explain_media_url: e.target.value })
                }
                placeholder="Leave empty to keep current"
              />
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setIsBulkUpdateExplainsDialogOpen(false)}>
              Cancel
            </Button>
            <Button
              onClick={handleBulkUpdateExplainsSubmit}
              disabled={bulkUpdateExplainsMutation.isPending || (
                !bulkUpdateExplainsFormData.explain_title &&
                !bulkUpdateExplainsFormData.explain_text &&
                !bulkUpdateExplainsFormData.explain_media_url
              )}
            >
              {bulkUpdateExplainsMutation.isPending ? (
                <>
                  <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  Updating...
                </>
              ) : (
                `Update Explains for ${selectedTaskIds.size} Tasks`
              )}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Bulk Update Status Dialog */}
      <Dialog open={isBulkUpdateStatusDialogOpen} onOpenChange={setIsBulkUpdateStatusDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Update Status</DialogTitle>
            <DialogDescription>
              Update status for {selectedTaskIds.size} selected task(s) for a specific user.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">
            <div className="space-y-2">
              <Label>User ID</Label>
              <Input
                type="number"
                min={1}
                value={bulkUpdateStatusFormData.user_id || ""}
                onChange={(e) =>
                  setBulkUpdateStatusFormData({ ...bulkUpdateStatusFormData, user_id: parseInt(e.target.value) || 0 })
                }
                placeholder="Enter user ID"
              />
            </div>

            <div className="space-y-2">
              <Label>Try Count</Label>
              <Input
                type="number"
                min={0}
                value={bulkUpdateStatusFormData.study_task_status_try_count}
                onChange={(e) =>
                  setBulkUpdateStatusFormData({ ...bulkUpdateStatusFormData, study_task_status_try_count: parseInt(e.target.value) || 0 })
                }
              />
            </div>

            <div className="flex items-center gap-2">
              <Checkbox
                id="bulk_is_solved"
                checked={bulkUpdateStatusFormData.study_task_status_is_solved}
                onCheckedChange={(checked) =>
                  setBulkUpdateStatusFormData({ ...bulkUpdateStatusFormData, study_task_status_is_solved: !!checked })
                }
              />
              <Label htmlFor="bulk_is_solved">Solved</Label>
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setIsBulkUpdateStatusDialogOpen(false)}>
              Cancel
            </Button>
            <Button
              onClick={handleBulkUpdateStatusSubmit}
              disabled={bulkUpdateStatusMutation.isPending || bulkUpdateStatusFormData.user_id <= 0}
            >
              {bulkUpdateStatusMutation.isPending ? (
                <>
                  <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  Updating...
                </>
              ) : (
                `Update Status for ${selectedTaskIds.size} Tasks`
              )}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}

// Task Expandable Row Component
function TaskExpandableRow({
  task,
  isExpanded,
  onToggle,
  getTaskKindBadgeVariant,
  isSelected,
  onSelectionChange,
}: {
  task: { study_task_id: number; study_task_kind: string; study_task_seq: number; question?: string | null };
  isExpanded: boolean;
  onToggle: () => void;
  getTaskKindBadgeVariant: (kind: string) => "default" | "secondary" | "destructive" | "outline";
  isSelected: boolean;
  onSelectionChange: () => void;
}) {
  const [activeTab, setActiveTab] = useState("details");
  const { data: taskDetail, isLoading } = useAdminStudyTaskDetail(
    isExpanded ? task.study_task_id : 0
  );

  return (
    <div className={`rounded-lg border ${isSelected ? "ring-2 ring-primary" : ""}`}>
      {/* Header Row - Clickable */}
      <div className="flex items-center gap-2 p-4 hover:bg-muted/50 transition-colors">
        {/* Checkbox - Stops propagation */}
        <div
          onClick={(e) => e.stopPropagation()}
          onKeyDown={(e) => e.stopPropagation()}
        >
          <Checkbox
            checked={isSelected}
            onCheckedChange={onSelectionChange}
            aria-label={`Select task ${task.study_task_id}`}
          />
        </div>
        {/* Rest of header - Clickable for expand/collapse */}
        <button
          type="button"
          onClick={onToggle}
          className="flex-1 flex items-center gap-4 text-left"
        >
          <span className="text-muted-foreground">
            {isExpanded ? (
              <ChevronDown className="h-4 w-4" />
            ) : (
              <ChevronRight className="h-4 w-4" />
            )}
          </span>
          <span className="w-12 font-mono text-center">{task.study_task_seq}</span>
          <Badge variant={getTaskKindBadgeVariant(task.study_task_kind)}>
            {task.study_task_kind}
          </Badge>
          <span className="flex-1 truncate text-muted-foreground">
            {task.question || "-"}
          </span>
          <span className="text-xs text-muted-foreground">#{task.study_task_id}</span>
        </button>
      </div>

      {/* Expanded Content with Tabs */}
      {isExpanded && (
        <div className="border-t bg-muted/30">
          <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
            <TabsList className="w-full justify-start rounded-none border-b bg-transparent h-auto p-0">
              <TabsTrigger
                value="details"
                className="rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent px-4 py-2"
              >
                Details
              </TabsTrigger>
              <TabsTrigger
                value="explains"
                className="rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent px-4 py-2"
              >
                <Globe className="h-4 w-4 mr-1" />
                Explains
              </TabsTrigger>
              <TabsTrigger
                value="status"
                className="rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent px-4 py-2"
              >
                <Users className="h-4 w-4 mr-1" />
                Status
              </TabsTrigger>
            </TabsList>

            <TabsContent value="details" className="p-4 mt-0">
              {isLoading ? (
                <div className="flex items-center gap-2 text-muted-foreground">
                  <Loader2 className="h-4 w-4 animate-spin" />
                  Loading task details...
                </div>
              ) : taskDetail ? (
                <TaskDetailsContent taskDetail={taskDetail} taskKind={task.study_task_kind} />
              ) : (
                <p className="text-sm text-destructive">Failed to load task details</p>
              )}
            </TabsContent>

            <TabsContent value="explains" className="p-4 mt-0">
              <TaskExplainsTab taskId={task.study_task_id} />
            </TabsContent>

            <TabsContent value="status" className="p-4 mt-0">
              <TaskStatusTab taskId={task.study_task_id} />
            </TabsContent>
          </Tabs>
        </div>
      )}
    </div>
  );
}

// Task Details Content Component
function TaskDetailsContent({
  taskDetail,
  taskKind,
}: {
  taskDetail: {
    question?: string | null;
    answer?: string | null;
    image_url?: string | null;
    audio_url?: string | null;
    choice_1?: string | null;
    choice_2?: string | null;
    choice_3?: string | null;
    choice_4?: string | null;
    choice_correct?: number | null;
  };
  taskKind: string;
}) {
  return (
    <div className="grid gap-4 md:grid-cols-2">
      {/* Question */}
      <div className="space-y-1">
        <Label className="text-xs text-muted-foreground">Question</Label>
        <p className="text-sm">{taskDetail.question || "-"}</p>
      </div>

      {/* Answer */}
      <div className="space-y-1">
        <Label className="text-xs text-muted-foreground">Answer</Label>
        <p className="text-sm">{taskDetail.answer || "-"}</p>
      </div>

      {/* Image URL */}
      {taskDetail.image_url && (
        <div className="space-y-1">
          <Label className="text-xs text-muted-foreground">Image URL</Label>
          <p className="text-sm text-blue-600 truncate">
            <a href={taskDetail.image_url} target="_blank" rel="noopener noreferrer">
              {taskDetail.image_url}
            </a>
          </p>
        </div>
      )}

      {/* Audio URL */}
      {taskDetail.audio_url && (
        <div className="space-y-1">
          <Label className="text-xs text-muted-foreground">Audio URL</Label>
          <p className="text-sm text-blue-600 truncate">
            <a href={taskDetail.audio_url} target="_blank" rel="noopener noreferrer">
              {taskDetail.audio_url}
            </a>
          </p>
        </div>
      )}

      {/* Choice fields (only for choice type) */}
      {taskKind === "choice" && (
        <div className="md:col-span-2 space-y-2">
          <Label className="text-xs text-muted-foreground">Choices</Label>
          <div className="grid gap-2 md:grid-cols-4">
            {[1, 2, 3, 4].map((num) => {
              const choiceKey = `choice_${num}` as keyof typeof taskDetail;
              const isCorrect = taskDetail.choice_correct === num;
              return (
                <div
                  key={num}
                  className={`p-2 rounded border text-sm ${
                    isCorrect
                      ? "border-green-500 bg-green-50 dark:bg-green-950"
                      : "border-muted"
                  }`}
                >
                  <span className="font-medium">{num}.</span>{" "}
                  {(taskDetail[choiceKey] as string) || "-"}
                  {isCorrect && (
                    <Badge variant="default" className="ml-2 text-xs">
                      Correct
                    </Badge>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
}

// Task Explains Tab Component
function TaskExplainsTab({ taskId }: { taskId: number }) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [editingExplain, setEditingExplain] = useState<AdminTaskExplainRes | null>(null);
  const [formData, setFormData] = useState({
    explain_lang: "ko" as UserSetLanguage,
    explain_title: "",
    explain_text: "",
    explain_media_url: "",
  });

  const { data: explains, isLoading, refetch } = useAdminTaskExplains({ task_id: taskId });
  const createMutation = useCreateAdminTaskExplain();
  const updateMutation = useUpdateAdminTaskExplain();

  const openCreateDialog = () => {
    setEditingExplain(null);
    setFormData({
      explain_lang: "ko",
      explain_title: "",
      explain_text: "",
      explain_media_url: "",
    });
    setIsDialogOpen(true);
  };

  const openEditDialog = (explain: AdminTaskExplainRes) => {
    setEditingExplain(explain);
    setFormData({
      explain_lang: explain.explain_lang,
      explain_title: explain.explain_title || "",
      explain_text: explain.explain_text || "",
      explain_media_url: explain.explain_media_url || "",
    });
    setIsDialogOpen(true);
  };

  const handleSubmit = async () => {
    try {
      if (editingExplain) {
        await updateMutation.mutateAsync({
          taskId,
          data: {
            explain_lang: formData.explain_lang,
            explain_title: formData.explain_title || undefined,
            explain_text: formData.explain_text || undefined,
            explain_media_url: formData.explain_media_url || undefined,
          },
        });
        toast.success("Explain updated");
      } else {
        await createMutation.mutateAsync({
          taskId,
          data: {
            explain_lang: formData.explain_lang,
            explain_title: formData.explain_title || undefined,
            explain_text: formData.explain_text || undefined,
            explain_media_url: formData.explain_media_url || undefined,
          },
        });
        toast.success("Explain created");
      }
      setIsDialogOpen(false);
      refetch();
    } catch {
      toast.error(editingExplain ? "Failed to update explain" : "Failed to create explain");
    }
  };

  const isPending = createMutation.isPending || updateMutation.isPending;

  if (isLoading) {
    return (
      <div className="flex items-center gap-2 text-muted-foreground">
        <Loader2 className="h-4 w-4 animate-spin" />
        Loading explains...
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <p className="text-sm text-muted-foreground">
          {explains?.list.length || 0} explain(s) found
        </p>
        <Button size="sm" variant="outline" onClick={openCreateDialog}>
          <Plus className="h-4 w-4 mr-1" />
          Add Explain
        </Button>
      </div>

      {explains?.list.length === 0 ? (
        <p className="text-center text-muted-foreground py-4">No explains yet</p>
      ) : (
        <div className="space-y-2">
          {explains?.list.map((explain) => (
            <div
              key={`${explain.study_task_id}-${explain.explain_lang}`}
              className="flex items-start gap-4 p-3 rounded-lg border bg-background"
            >
              <Badge variant="outline" className="uppercase">
                {explain.explain_lang}
              </Badge>
              <div className="flex-1 min-w-0">
                <p className="font-medium text-sm">{explain.explain_title || "(No title)"}</p>
                <p className="text-sm text-muted-foreground line-clamp-2">
                  {explain.explain_text || "(No content)"}
                </p>
              </div>
              <Button
                size="sm"
                variant="ghost"
                onClick={() => openEditDialog(explain)}
              >
                <Pencil className="h-4 w-4" />
              </Button>
            </div>
          ))}
        </div>
      )}

      {/* Create/Edit Dialog */}
      <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {editingExplain ? "Edit Explain" : "Add Explain"}
            </DialogTitle>
            <DialogDescription>
              {editingExplain
                ? "Update the explanation for this task"
                : "Add a new explanation for this task"}
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">
            <div className="space-y-2">
              <Label>Language</Label>
              <Select
                value={formData.explain_lang}
                onValueChange={(value) =>
                  setFormData({ ...formData, explain_lang: value as UserSetLanguage })
                }
                disabled={!!editingExplain}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="ko">Korean (ko)</SelectItem>
                  <SelectItem value="en">English (en)</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Title</Label>
              <Input
                value={formData.explain_title}
                onChange={(e) =>
                  setFormData({ ...formData, explain_title: e.target.value })
                }
                placeholder="Explanation title"
              />
            </div>

            <div className="space-y-2">
              <Label>Text</Label>
              <Textarea
                value={formData.explain_text}
                onChange={(e) =>
                  setFormData({ ...formData, explain_text: e.target.value })
                }
                placeholder="Explanation content"
                rows={4}
              />
            </div>

            <div className="space-y-2">
              <Label>Media URL (optional)</Label>
              <Input
                value={formData.explain_media_url}
                onChange={(e) =>
                  setFormData({ ...formData, explain_media_url: e.target.value })
                }
                placeholder="https://..."
              />
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setIsDialogOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={isPending}>
              {isPending ? (
                <>
                  <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  Saving...
                </>
              ) : (
                "Save"
              )}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}

// Task Status Tab Component
function TaskStatusTab({ taskId }: { taskId: number }) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [editingStatus, setEditingStatus] = useState<AdminTaskStatusRes | null>(null);
  const [formData, setFormData] = useState({
    user_id: 0,
    study_task_status_try_count: 0,
    study_task_status_is_solved: false,
  });

  const { data: statuses, isLoading, refetch } = useAdminTaskStatus({ task_id: taskId });
  const updateMutation = useUpdateAdminTaskStatus();

  const openEditDialog = (status: AdminTaskStatusRes) => {
    setEditingStatus(status);
    setFormData({
      user_id: status.user_id,
      study_task_status_try_count: status.study_task_status_try_count,
      study_task_status_is_solved: status.study_task_status_is_solved,
    });
    setIsDialogOpen(true);
  };

  const handleSubmit = async () => {
    if (!editingStatus) return;
    try {
      await updateMutation.mutateAsync({
        taskId,
        data: {
          user_id: formData.user_id,
          study_task_status_try_count: formData.study_task_status_try_count,
          study_task_status_is_solved: formData.study_task_status_is_solved,
        },
      });
      toast.success("Status updated");
      setIsDialogOpen(false);
      refetch();
    } catch {
      toast.error("Failed to update status");
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center gap-2 text-muted-foreground">
        <Loader2 className="h-4 w-4 animate-spin" />
        Loading status...
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <p className="text-sm text-muted-foreground">
        {statuses?.list.length || 0} user status(es) found
      </p>

      {statuses?.list.length === 0 ? (
        <p className="text-center text-muted-foreground py-4">No user status yet</p>
      ) : (
        <div className="rounded-md border">
          <table className="w-full text-sm">
            <thead className="border-b bg-muted/50">
              <tr>
                <th className="h-10 px-4 text-left font-medium">User ID</th>
                <th className="h-10 px-4 text-left font-medium">Try Count</th>
                <th className="h-10 px-4 text-left font-medium">Solved</th>
                <th className="h-10 px-4 text-left font-medium">Last Attempt</th>
                <th className="h-10 px-4 text-right font-medium">Action</th>
              </tr>
            </thead>
            <tbody>
              {statuses?.list.map((status) => (
                <tr key={`${status.study_task_id}-${status.user_id}`} className="border-b">
                  <td className="p-4 font-mono">#{status.user_id}</td>
                  <td className="p-4">{status.study_task_status_try_count}</td>
                  <td className="p-4">
                    <Badge variant={status.study_task_status_is_solved ? "default" : "secondary"}>
                      {status.study_task_status_is_solved ? "Yes" : "No"}
                    </Badge>
                  </td>
                  <td className="p-4 text-muted-foreground text-xs">
                    {status.study_task_status_last_attempt_at
                      ? new Date(status.study_task_status_last_attempt_at).toLocaleString()
                      : "-"}
                  </td>
                  <td className="p-4 text-right">
                    <Button
                      size="sm"
                      variant="ghost"
                      onClick={() => openEditDialog(status)}
                    >
                      <Pencil className="h-4 w-4" />
                    </Button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {/* Edit Dialog */}
      <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Edit Status</DialogTitle>
            <DialogDescription>
              Update user progress for this task (User #{editingStatus?.user_id})
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">
            <div className="space-y-2">
              <Label>Try Count</Label>
              <Input
                type="number"
                min={0}
                value={formData.study_task_status_try_count}
                onChange={(e) =>
                  setFormData({ ...formData, study_task_status_try_count: parseInt(e.target.value) || 0 })
                }
              />
            </div>

            <div className="flex items-center gap-2">
              <input
                type="checkbox"
                id="is_solved"
                checked={formData.study_task_status_is_solved}
                onChange={(e) =>
                  setFormData({ ...formData, study_task_status_is_solved: e.target.checked })
                }
                className="h-4 w-4"
              />
              <Label htmlFor="is_solved">Solved</Label>
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setIsDialogOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={updateMutation.isPending}>
              {updateMutation.isPending ? (
                <>
                  <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  Saving...
                </>
              ) : (
                "Save"
              )}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
