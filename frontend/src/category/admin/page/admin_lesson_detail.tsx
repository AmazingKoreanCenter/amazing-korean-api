import { useState, useEffect, useMemo } from "react";
import { useParams, useNavigate, Link } from "react-router-dom";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ArrowLeft, Save, Video, ListOrdered, Users, ExternalLink, Plus, Search, Check, ChevronRight, ChevronDown, X, Pencil, Trash2, GripVertical } from "lucide-react";
import { toast } from "sonner";
import {
  DndContext,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  type DragEndEvent,
} from "@dnd-kit/core";
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  useSortable,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";

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
import { Progress } from "@/components/ui/progress";
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
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

import {
  useAdminLessonDetail,
  useUpdateAdminLesson,
  useAdminLessonItemsDetail,
  useAdminLessonProgressDetail,
  useCreateAdminLessonItem,
  useCreateAdminLessonItemsBulk,
  useUpdateAdminLessonItem,
  useUpdateAdminLessonItemsBulk,
  useDeleteAdminLessonItem,
  useDeleteAdminLessonItemsBulk,
  useUpdateAdminLessonProgress,
  useUpdateAdminLessonProgressBulk,
} from "../hook/use_admin_lessons";
import { useAdminVideos } from "../hook/use_admin_videos";
import { useAdminStudies, useAdminStudyDetail } from "../hook/use_admin_studies";
import {
  lessonUpdateReqSchema,
  type LessonUpdateReq,
  type LessonState,
  type LessonAccess,
  type InsertMode,
  type AdminLessonItemDetailRes,
  type AdminLessonProgressDetailRes,
  type LessonProgressUpdateReq,
} from "../lesson/types";

export function AdminLessonDetail() {
  const { lessonId } = useParams<{ lessonId: string }>();
  const navigate = useNavigate();
  const id = Number(lessonId);

  const { data: lesson, isLoading, isError } = useAdminLessonDetail(id);
  const updateMutation = useUpdateAdminLesson();

  const [cooldown, setCooldown] = useState(0);
  const [activeTab, setActiveTab] = useState("info");

  useEffect(() => {
    if (cooldown > 0) {
      const timer = setTimeout(() => setCooldown(cooldown - 1), 1000);
      return () => clearTimeout(timer);
    }
  }, [cooldown]);

  const form = useForm<LessonUpdateReq>({
    resolver: zodResolver(lessonUpdateReqSchema),
    defaultValues: {
      lesson_idx: "",
      lesson_title: "",
      lesson_subtitle: "",
      lesson_description: "",
      lesson_state: "ready",
      lesson_access: "public",
    },
  });

  useEffect(() => {
    if (lesson) {
      form.reset({
        lesson_idx: lesson.lesson_idx || "",
        lesson_title: lesson.lesson_title || "",
        lesson_subtitle: lesson.lesson_subtitle || "",
        lesson_description: lesson.lesson_description || "",
        lesson_state: lesson.lesson_state || "ready",
        lesson_access: lesson.lesson_access || "public",
      });
    }
  }, [lesson, form]);

  const onSubmit = async (data: LessonUpdateReq) => {
    try {
      await updateMutation.mutateAsync({ id, data });
      toast.success("Lesson updated successfully");
      setCooldown(10);
      setTimeout(() => {
        navigate("/admin/lessons");
      }, 1500);
    } catch {
      toast.error("Failed to update lesson");
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

  if (isError || !lesson) {
    return (
      <div className="space-y-4">
        <Button variant="ghost" onClick={() => navigate("/admin/lessons")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Lessons
        </Button>
        <p className="text-destructive">Lesson not found</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/lessons")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Edit Lesson #{lesson.lesson_id}</h1>
      </div>

      <Tabs value={activeTab} onValueChange={setActiveTab}>
        <TabsList>
          <TabsTrigger value="info">Info</TabsTrigger>
          <TabsTrigger value="items">
            <ListOrdered className="h-4 w-4 mr-1" />
            Items
          </TabsTrigger>
          <TabsTrigger value="progress">
            <Users className="h-4 w-4 mr-1" />
            Progress
          </TabsTrigger>
        </TabsList>

        <TabsContent value="info" className="mt-4">
          <form
            onSubmit={form.handleSubmit(onSubmit, (errors) => {
              const errorFields = Object.keys(errors).join(", ");
              toast.error(`Please fix errors: ${errorFields}`);
            })}
          >
            <Card>
              <CardHeader>
                <CardTitle>Lesson Information</CardTitle>
                <CardDescription>Update lesson details and settings</CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid gap-4 md:grid-cols-2">
                  {/* Lesson IDX */}
                  <div className="space-y-2">
                    <Label htmlFor="lesson_idx">Lesson IDX</Label>
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
                    <Label htmlFor="lesson_title">Title</Label>
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
                      onValueChange={(value) =>
                        form.setValue("lesson_state", value as LessonState)
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
                      value={form.watch("lesson_access") ?? "public"}
                      onValueChange={(value) =>
                        form.setValue("lesson_access", value as LessonAccess)
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

                  {/* Created At (read-only) */}
                  <div className="space-y-2">
                    <Label>Created At</Label>
                    <Input
                      value={new Date(lesson.lesson_created_at).toLocaleString()}
                      disabled
                      className="bg-muted"
                    />
                  </div>

                  {/* Updated At (read-only) */}
                  <div className="space-y-2">
                    <Label>Updated At</Label>
                    <Input
                      value={new Date(lesson.lesson_updated_at).toLocaleString()}
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
        </TabsContent>

        <TabsContent value="items" className="mt-4">
          <LessonItemsTab lessonId={id} />
        </TabsContent>

        <TabsContent value="progress" className="mt-4">
          <LessonProgressTab lessonId={id} />
        </TabsContent>
      </Tabs>
    </div>
  );
}

// Sortable Item Component for drag-and-drop
interface SortableItemProps {
  id: string;
  item: {
    lesson_id: number;
    lesson_item_seq: number;
    lesson_item_kind: string;
    video?: {
      video_id: number;
      video_idx: string;
      video_tag_title?: string | null;
      video_tag_subtitle?: string | null;
      video_views: number;
      video_state: string;
      video_duration?: number | null;
    } | null;
    study_task?: {
      study_task_id: number;
      study_id: number;
      study_task_kind: string;
      study_task_seq: number;
    } | null;
  };
  isSelected: boolean;
  onToggleSelection: (seq: number) => void;
  onEdit: () => void;
  onDelete: () => void;
  getKindBadgeVariant: (kind: string) => "default" | "secondary" | "outline";
}

function SortableItem({
  id,
  item,
  isSelected,
  onToggleSelection,
  onEdit,
  onDelete,
  getKindBadgeVariant,
}: SortableItemProps) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  return (
    <div
      ref={setNodeRef}
      style={style}
      className={`flex items-start gap-4 p-4 rounded-lg border ${
        isSelected ? "border-primary bg-primary/5" : ""
      } ${isDragging ? "shadow-lg" : ""}`}
    >
      <div
        {...attributes}
        {...listeners}
        className="cursor-grab active:cursor-grabbing p-1 hover:bg-muted rounded"
      >
        <GripVertical className="h-5 w-5 text-muted-foreground" />
      </div>
      <Checkbox
        checked={isSelected}
        onCheckedChange={() => onToggleSelection(item.lesson_item_seq)}
        className="mt-1"
      />
      <span className="w-8 text-center font-mono text-muted-foreground">
        {item.lesson_item_seq}
      </span>
      <Badge variant={getKindBadgeVariant(item.lesson_item_kind)}>
        {item.lesson_item_kind}
      </Badge>
      <div className="flex-1 min-w-0">
        {item.video && (
          <div className="space-y-1">
            <div className="flex items-center gap-2">
              <Video className="h-4 w-4 text-muted-foreground" />
              <span className="font-medium">{item.video.video_tag_title || item.video.video_idx}</span>
              <Badge variant="outline" className="text-xs">
                {item.video.video_state}
              </Badge>
            </div>
            <p className="text-sm text-muted-foreground truncate">
              {item.video.video_tag_subtitle || "-"}
            </p>
            <div className="flex items-center gap-4 text-xs text-muted-foreground">
              <span>Views: {item.video.video_views}</span>
              {item.video.video_duration && (
                <span>Duration: {Math.floor(item.video.video_duration / 60)}min</span>
              )}
              <Link
                to={`/admin/videos/${item.video.video_id}`}
                className="flex items-center gap-1 text-primary hover:underline"
              >
                View <ExternalLink className="h-3 w-3" />
              </Link>
            </div>
          </div>
        )}
        {item.study_task && (
          <div className="space-y-1">
            <div className="flex items-center gap-2">
              <span className="font-medium">Task #{item.study_task.study_task_id}</span>
              <Badge variant="outline" className="text-xs">
                {item.study_task.study_task_kind}
              </Badge>
            </div>
            <p className="text-sm text-muted-foreground">
              Seq: {item.study_task.study_task_seq} | Study ID:{" "}
              {item.study_task.study_id}
            </p>
          </div>
        )}
        {!item.video && !item.study_task && (
          <p className="text-sm text-muted-foreground">No linked content</p>
        )}
      </div>
      <div className="flex items-center gap-1 shrink-0">
        <Button
          variant="ghost"
          size="sm"
          onClick={onEdit}
        >
          <Pencil className="h-4 w-4" />
        </Button>
        <Button
          variant="ghost"
          size="sm"
          className="text-destructive hover:text-destructive"
          onClick={onDelete}
        >
          <Trash2 className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
}

// Lesson Items Tab Component
function LessonItemsTab({ lessonId }: { lessonId: number }) {
  const { data, isLoading, isError } = useAdminLessonItemsDetail(lessonId);
  const createItemMutation = useCreateAdminLessonItem();
  const bulkCreateMutation = useCreateAdminLessonItemsBulk();
  const updateItemMutation = useUpdateAdminLessonItem();
  const bulkUpdateItemsMutation = useUpdateAdminLessonItemsBulk();
  const deleteItemMutation = useDeleteAdminLessonItem();
  const bulkDeleteItemsMutation = useDeleteAdminLessonItemsBulk();

  // Selection state for bulk operations
  const [selectedItems, setSelectedItems] = useState<Set<number>>(new Set());

  // Drag-and-drop state for reordering
  const [orderedItems, setOrderedItems] = useState<AdminLessonItemDetailRes[] | null>(null);
  const [isReordering, setIsReordering] = useState(false);

  // Sync orderedItems with data when data changes
  useEffect(() => {
    if (data?.items) {
      setOrderedItems(data.items);
    }
  }, [data?.items]);

  // DnD sensors configuration
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8,
      },
    }),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  // Generate unique IDs for sortable items
  const itemIds = useMemo(
    () => (orderedItems || []).map((item) => `${item.lesson_id}-${item.lesson_item_seq}`),
    [orderedItems]
  );

  // Handle drag end - reorder items and save via bulk update
  const handleDragEnd = async (event: DragEndEvent) => {
    const { active, over } = event;

    if (!over || active.id === over.id || !orderedItems) {
      return;
    }

    const oldIndex = itemIds.indexOf(active.id as string);
    const newIndex = itemIds.indexOf(over.id as string);

    if (oldIndex === -1 || newIndex === -1) return;

    // Reorder items locally first for immediate UI feedback
    const reordered = arrayMove(orderedItems, oldIndex, newIndex);
    setOrderedItems(reordered);

    // Create bulk update request to save new sequence order
    // Each item gets new sequence number based on its position
    const items = reordered.map((item, index) => ({
      lesson_id: lessonId,
      current_lesson_item_seq: item.lesson_item_seq,
      new_lesson_item_seq: index + 1,
    }));

    setIsReordering(true);
    try {
      const result = await bulkUpdateItemsMutation.mutateAsync({ items });
      if (result.failure_count > 0) {
        toast.warning(`Reorder partially failed: ${result.failure_count} items`);
      } else {
        toast.success("Items reordered successfully");
      }
    } catch {
      toast.error("Failed to save item order");
      // Revert to original order on failure
      if (data?.items) {
        setOrderedItems(data.items);
      }
    } finally {
      setIsReordering(false);
    }
  };

  // Bulk Edit dialog state
  const [bulkEditDialogOpen, setBulkEditDialogOpen] = useState(false);
  const [bulkEditKind, setBulkEditKind] = useState<"video" | "task" | "">("");
  const [bulkEditSelectedVideo, setBulkEditSelectedVideo] = useState<{ id: number; title: string } | null>(null);
  const [bulkEditSelectedTask, setBulkEditSelectedTask] = useState<{ id: number; kind: string; seq: number } | null>(null);
  const [bulkEditVideoSearchQuery, setBulkEditVideoSearchQuery] = useState("");
  const [bulkEditVideoDropdownOpen, setBulkEditVideoDropdownOpen] = useState(false);
  const [bulkEditStudySearchQuery, setBulkEditStudySearchQuery] = useState("");
  const [bulkEditSelectedStudyId, setBulkEditSelectedStudyId] = useState<number | null>(null);
  const [bulkEditStudyDropdownOpen, setBulkEditStudyDropdownOpen] = useState(false);

  // Bulk Delete dialog state
  const [bulkDeleteDialogOpen, setBulkDeleteDialogOpen] = useState(false);

  // Delete Item dialog state
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [deletingItem, setDeletingItem] = useState<{
    seq: number;
    kind: string;
    title: string;
  } | null>(null);

  // Edit Item dialog state
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [editingItem, setEditingItem] = useState<{
    originalSeq: number;
    kind: "video" | "task";
    videoId?: number;
    videoTitle?: string;
    taskId?: number;
    taskKind?: string;
    taskSeq?: number;
  } | null>(null);
  const [editSeq, setEditSeq] = useState("");
  const [editKind, setEditKind] = useState<"video" | "task">("video");
  const [editSelectedVideo, setEditSelectedVideo] = useState<{ id: number; title: string } | null>(null);
  const [editSelectedTask, setEditSelectedTask] = useState<{ id: number; kind: string; seq: number } | null>(null);
  const [editVideoSearchQuery, setEditVideoSearchQuery] = useState("");
  const [editVideoDropdownOpen, setEditVideoDropdownOpen] = useState(false);
  const [editStudySearchQuery, setEditStudySearchQuery] = useState("");
  const [editSelectedStudyId, setEditSelectedStudyId] = useState<number | null>(null);
  const [editStudyDropdownOpen, setEditStudyDropdownOpen] = useState(false);

  // Fetch videos for edit search
  const { data: editVideosData, isLoading: editVideosLoading } = useAdminVideos({
    page: 1,
    size: 20,
    q: editVideoSearchQuery,
  });

  // Fetch studies for edit search
  const { data: editStudiesData, isLoading: editStudiesLoading } = useAdminStudies({
    page: 1,
    size: 20,
    q: editStudySearchQuery,
  });

  // Fetch selected study detail for edit tasks
  const { data: editStudyDetail, isLoading: editStudyDetailLoading } = useAdminStudyDetail(editSelectedStudyId ?? 0);

  // Fetch videos for bulk edit search
  const { data: bulkEditVideosData, isLoading: bulkEditVideosLoading } = useAdminVideos({
    page: 1,
    size: 20,
    q: bulkEditVideoSearchQuery,
  });

  // Fetch studies for bulk edit search
  const { data: bulkEditStudiesData, isLoading: bulkEditStudiesLoading } = useAdminStudies({
    page: 1,
    size: 20,
    q: bulkEditStudySearchQuery,
  });

  // Fetch selected study detail for bulk edit tasks
  const { data: bulkEditStudyDetail, isLoading: bulkEditStudyDetailLoading } = useAdminStudyDetail(bulkEditSelectedStudyId ?? 0);

  // Add Single Item dialog state
  const [dialogOpen, setDialogOpen] = useState(false);
  const [itemKind, setItemKind] = useState<"video" | "task">("video");
  const [itemSeq, setItemSeq] = useState("");
  const [shiftIfExists, setShiftIfExists] = useState(true);

  // Video search state (shared between single and multi-select)
  const [videoSearchQuery, setVideoSearchQuery] = useState("");
  const [selectedVideo, setSelectedVideo] = useState<{ id: number; title: string } | null>(null);
  const [videoDropdownOpen, setVideoDropdownOpen] = useState(false);

  // Study task search state (for single select)
  const [studySearchQuery, setStudySearchQuery] = useState("");
  const [selectedStudyId, setSelectedStudyId] = useState<number | null>(null);
  const [selectedTask, setSelectedTask] = useState<{ id: number; kind: string; seq: number; question?: string } | null>(null);
  const [studyDropdownOpen, setStudyDropdownOpen] = useState(false);

  // Multi-select dialog state
  const [multiDialogOpen, setMultiDialogOpen] = useState(false);
  const [multiItemKind, setMultiItemKind] = useState<"video" | "task">("video");
  const [multiStartSeq, setMultiStartSeq] = useState("");
  const [selectedVideos, setSelectedVideos] = useState<Array<{ id: number; title: string }>>([]);
  const [selectedTasks, setSelectedTasks] = useState<Array<{ id: number; kind: string; seq: number; question?: string }>>([]);
  const [multiStudySearchQuery, setMultiStudySearchQuery] = useState("");
  const [multiSelectedStudyId, setMultiSelectedStudyId] = useState<number | null>(null);
  const [multiVideoSearchQuery, setMultiVideoSearchQuery] = useState("");
  const [multiVideoDropdownOpen, setMultiVideoDropdownOpen] = useState(false);
  const [multiStudyDropdownOpen, setMultiStudyDropdownOpen] = useState(false);

  // Fetch videos for multi-select search
  const { data: multiVideosData, isLoading: multiVideosLoading } = useAdminVideos({
    page: 1,
    size: 30,
    q: multiVideoSearchQuery,
  });

  // Fetch studies for multi-select search
  const { data: multiStudiesData, isLoading: multiStudiesLoading } = useAdminStudies({
    page: 1,
    size: 20,
    q: multiStudySearchQuery,
  });

  // Fetch selected study detail for multi-select tasks
  const { data: multiStudyDetail, isLoading: multiStudyDetailLoading } = useAdminStudyDetail(multiSelectedStudyId ?? 0);

  // Fetch videos for search
  const { data: videosData, isLoading: videosLoading } = useAdminVideos({
    page: 1,
    size: 20,
    q: videoSearchQuery,
  });

  // Fetch studies for search
  const { data: studiesData, isLoading: studiesLoading } = useAdminStudies({
    page: 1,
    size: 20,
    q: studySearchQuery,
  });

  // Fetch selected study detail for tasks
  const { data: studyDetail, isLoading: studyDetailLoading } = useAdminStudyDetail(selectedStudyId ?? 0);

  const getKindBadgeVariant = (kind: string) => {
    switch (kind) {
      case "video":
        return "default" as const;
      case "task":
        return "secondary" as const;
      default:
        return "outline" as const;
    }
  };

  const resetForm = () => {
    setItemKind("video");
    setItemSeq("");
    setShiftIfExists(true);
    setVideoSearchQuery("");
    setSelectedVideo(null);
    setVideoDropdownOpen(false);
    setStudySearchQuery("");
    setSelectedStudyId(null);
    setSelectedTask(null);
    setStudyDropdownOpen(false);
  };

  const resetMultiForm = () => {
    setMultiItemKind("video");
    setMultiStartSeq("");
    setSelectedVideos([]);
    setSelectedTasks([]);
    setMultiVideoSearchQuery("");
    setMultiVideoDropdownOpen(false);
    setMultiStudySearchQuery("");
    setMultiSelectedStudyId(null);
    setMultiStudyDropdownOpen(false);
  };

  const resetBulkEditForm = () => {
    setBulkEditKind("");
    setBulkEditSelectedVideo(null);
    setBulkEditSelectedTask(null);
    setBulkEditVideoSearchQuery("");
    setBulkEditVideoDropdownOpen(false);
    setBulkEditStudySearchQuery("");
    setBulkEditSelectedStudyId(null);
    setBulkEditStudyDropdownOpen(false);
  };

  // Selection functions for bulk operations
  const toggleItemSelection = (seq: number) => {
    setSelectedItems((prev) => {
      const next = new Set(prev);
      if (next.has(seq)) {
        next.delete(seq);
      } else {
        next.add(seq);
      }
      return next;
    });
  };

  const selectAllItems = () => {
    if (!data?.items) return;
    const allSeqs = data.items.map((item) => item.lesson_item_seq);
    setSelectedItems(new Set(allSeqs));
  };

  const deselectAllItems = () => {
    setSelectedItems(new Set());
  };

  const isAllSelected = data?.items && data.items.length > 0 && selectedItems.size === data.items.length;

  const handleBulkEdit = async () => {
    if (selectedItems.size === 0) {
      toast.error("Please select at least one item");
      return;
    }

    // Validate that kind is selected if changing kind
    if (bulkEditKind === "video" && !bulkEditSelectedVideo) {
      toast.error("Please select a video");
      return;
    }
    if (bulkEditKind === "task" && !bulkEditSelectedTask) {
      toast.error("Please select a study task");
      return;
    }

    const items = Array.from(selectedItems).map((seq) => ({
      lesson_id: lessonId,
      current_lesson_item_seq: seq,
      ...(bulkEditKind === "video" ? {
        lesson_item_kind: "video",
        video_id: bulkEditSelectedVideo?.id,
      } : bulkEditKind === "task" ? {
        lesson_item_kind: "task",
        study_task_id: bulkEditSelectedTask?.id,
      } : {}),
    }));

    try {
      const result = await bulkUpdateItemsMutation.mutateAsync({ items });
      toast.success(`Updated ${result.success_count} items successfully`);
      if (result.failure_count > 0) {
        toast.warning(`${result.failure_count} items failed`);
      }
      setBulkEditDialogOpen(false);
      resetBulkEditForm();
      setSelectedItems(new Set());
    } catch {
      toast.error("Failed to update items");
    }
  };

  const handleBulkDelete = async () => {
    if (selectedItems.size === 0) {
      toast.error("Please select at least one item");
      return;
    }

    const items = Array.from(selectedItems).map((seq) => ({
      lesson_id: lessonId,
      lesson_item_seq: seq,
    }));

    try {
      const result = await bulkDeleteItemsMutation.mutateAsync({ items });
      toast.success(`Deleted ${result.success_count} items successfully`);
      if (result.failure_count > 0) {
        toast.warning(`${result.failure_count} items failed`);
      }
      setBulkDeleteDialogOpen(false);
      setSelectedItems(new Set());
    } catch {
      toast.error("Failed to delete items");
    }
  };

  const toggleVideoSelection = (video: { id: number; title: string }) => {
    setSelectedVideos((prev) => {
      const exists = prev.find((v) => v.id === video.id);
      if (exists) {
        return prev.filter((v) => v.id !== video.id);
      }
      return [...prev, video];
    });
  };

  const toggleTaskSelection = (task: { id: number; kind: string; seq: number; question?: string }) => {
    setSelectedTasks((prev) => {
      const exists = prev.find((t) => t.id === task.id);
      if (exists) {
        return prev.filter((t) => t.id !== task.id);
      }
      return [...prev, task];
    });
  };

  const handleAddMultipleItems = async () => {
    const startSeq = parseInt(multiStartSeq);
    if (isNaN(startSeq) || startSeq < 1) {
      toast.error("Please enter a valid starting sequence (>= 1)");
      return;
    }

    const items = multiItemKind === "video"
      ? selectedVideos.map((v, idx) => ({
          lesson_id: lessonId,
          lesson_item_seq: startSeq + idx,
          lesson_item_kind: "video" as const,
          video_id: v.id,
        }))
      : selectedTasks.map((t, idx) => ({
          lesson_id: lessonId,
          lesson_item_seq: startSeq + idx,
          lesson_item_kind: "task" as const,
          study_task_id: t.id,
        }));

    if (items.length === 0) {
      toast.error(`Please select at least one ${multiItemKind === "video" ? "video" : "task"}`);
      return;
    }

    try {
      const result = await bulkCreateMutation.mutateAsync({ items });
      toast.success(`Added ${result.success_count} items successfully`);
      if (result.failure_count > 0) {
        toast.warning(`${result.failure_count} items failed`);
      }
      setMultiDialogOpen(false);
      resetMultiForm();
    } catch {
      toast.error("Failed to add items");
    }
  };

  const handleAddItem = async () => {
    const seq = parseInt(itemSeq);
    if (isNaN(seq) || seq < 1) {
      toast.error("Please enter a valid sequence number (>= 1)");
      return;
    }

    if (itemKind === "video") {
      if (!selectedVideo) {
        toast.error("Please select a video");
        return;
      }
    } else {
      if (!selectedTask) {
        toast.error("Please select a study task");
        return;
      }
    }

    try {
      await createItemMutation.mutateAsync({
        lessonId,
        data: {
          lesson_item_seq: seq,
          lesson_item_kind: itemKind,
          video_id: itemKind === "video" ? selectedVideo?.id : undefined,
          study_task_id: itemKind === "task" ? selectedTask?.id : undefined,
          insert_mode: shiftIfExists ? "shift" : "error" as InsertMode,
        },
      });
      toast.success("Item added successfully");
      setDialogOpen(false);
      resetForm();
    } catch {
      toast.error("Failed to add item. Check if the video/task exists.");
    }
  };

  const openEditDialog = (item: {
    lesson_item_seq: number;
    lesson_item_kind: string;
    video?: { video_id: number; video_tag_title?: string | null; video_idx: string } | null;
    study_task?: { study_task_id: number; study_task_kind: string; study_task_seq: number } | null;
  }) => {
    setEditingItem({
      originalSeq: item.lesson_item_seq,
      kind: item.lesson_item_kind as "video" | "task",
      videoId: item.video?.video_id,
      videoTitle: item.video?.video_tag_title || item.video?.video_idx,
      taskId: item.study_task?.study_task_id,
      taskKind: item.study_task?.study_task_kind,
      taskSeq: item.study_task?.study_task_seq,
    });
    setEditSeq(String(item.lesson_item_seq));
    setEditKind(item.lesson_item_kind as "video" | "task");
    if (item.video) {
      setEditSelectedVideo({
        id: item.video.video_id,
        title: item.video.video_tag_title || item.video.video_idx,
      });
    } else {
      setEditSelectedVideo(null);
    }
    if (item.study_task) {
      setEditSelectedTask({
        id: item.study_task.study_task_id,
        kind: item.study_task.study_task_kind,
        seq: item.study_task.study_task_seq,
      });
    } else {
      setEditSelectedTask(null);
    }
    setEditVideoSearchQuery("");
    setEditVideoDropdownOpen(false);
    setEditStudySearchQuery("");
    setEditSelectedStudyId(null);
    setEditStudyDropdownOpen(false);
    setEditDialogOpen(true);
  };

  const resetEditForm = () => {
    setEditingItem(null);
    setEditSeq("");
    setEditKind("video");
    setEditSelectedVideo(null);
    setEditSelectedTask(null);
    setEditVideoSearchQuery("");
    setEditVideoDropdownOpen(false);
    setEditStudySearchQuery("");
    setEditSelectedStudyId(null);
    setEditStudyDropdownOpen(false);
  };

  const handleEditItem = async () => {
    if (!editingItem) return;

    const newSeq = parseInt(editSeq);
    if (isNaN(newSeq) || newSeq < 1) {
      toast.error("Please enter a valid sequence number (>= 1)");
      return;
    }

    if (editKind === "video" && !editSelectedVideo) {
      toast.error("Please select a video");
      return;
    }
    if (editKind === "task" && !editSelectedTask) {
      toast.error("Please select a study task");
      return;
    }

    try {
      await updateItemMutation.mutateAsync({
        lessonId,
        seq: editingItem.originalSeq,
        data: {
          lesson_item_seq: newSeq !== editingItem.originalSeq ? newSeq : undefined,
          lesson_item_kind: editKind !== editingItem.kind ? editKind : undefined,
          video_id: editKind === "video" ? editSelectedVideo?.id : undefined,
          study_task_id: editKind === "task" ? editSelectedTask?.id : undefined,
        },
      });
      toast.success("Item updated successfully");
      setEditDialogOpen(false);
      resetEditForm();
    } catch {
      toast.error("Failed to update item");
    }
  };

  const openDeleteDialog = (item: {
    lesson_item_seq: number;
    lesson_item_kind: string;
    video?: { video_tag_title?: string | null; video_idx: string } | null;
    study_task?: { study_task_id: number; study_task_kind: string } | null;
  }) => {
    const title = item.video
      ? item.video.video_tag_title || item.video.video_idx
      : item.study_task
        ? `Task #${item.study_task.study_task_id} (${item.study_task.study_task_kind})`
        : "Unknown";
    setDeletingItem({
      seq: item.lesson_item_seq,
      kind: item.lesson_item_kind,
      title,
    });
    setDeleteDialogOpen(true);
  };

  const handleDeleteItem = async () => {
    if (!deletingItem) return;

    try {
      await deleteItemMutation.mutateAsync({
        lessonId,
        seq: deletingItem.seq,
      });
      toast.success("Item deleted successfully");
      setDeleteDialogOpen(false);
      setDeletingItem(null);
    } catch {
      toast.error("Failed to delete item");
    }
  };

  // Calculate next sequence number
  const nextSeq = data?.items.length ? Math.max(...data.items.map((i) => i.lesson_item_seq)) + 1 : 1;

  if (isLoading) {
    return (
      <Card>
        <CardHeader>
          <Skeleton className="h-6 w-32" />
        </CardHeader>
        <CardContent>
          {Array.from({ length: 3 }).map((_, i) => (
            <div key={i} className="flex items-center gap-4 p-4 border-b">
              <Skeleton className="h-4 w-8" />
              <Skeleton className="h-5 w-16" />
              <Skeleton className="h-4 w-48" />
            </div>
          ))}
        </CardContent>
      </Card>
    );
  }

  if (isError || !data) {
    return (
      <Card>
        <CardContent className="py-8 text-center text-destructive">
          Failed to load lesson items
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <div>
          <CardTitle className="flex items-center gap-2">
            <ListOrdered className="h-5 w-5" />
            Lesson Items ({data.total_items})
          </CardTitle>
          <CardDescription>{data.lesson_title}</CardDescription>
        </div>
        <div className="flex items-center gap-2">
          {/* Selection controls */}
          {data.items.length > 0 && (
            <div className="flex items-center gap-2 mr-2">
              <Button
                variant="outline"
                size="sm"
                onClick={isAllSelected ? deselectAllItems : selectAllItems}
              >
                {isAllSelected ? "Deselect All" : "Select All"}
              </Button>
              {selectedItems.size > 0 && (
                <>
                  <Button
                    variant="secondary"
                    size="sm"
                    onClick={() => setBulkEditDialogOpen(true)}
                  >
                    <Pencil className="h-4 w-4 mr-1" />
                    Edit {selectedItems.size}
                  </Button>
                  <Button
                    variant="destructive"
                    size="sm"
                    onClick={() => setBulkDeleteDialogOpen(true)}
                  >
                    <Trash2 className="h-4 w-4 mr-1" />
                    Delete {selectedItems.size}
                  </Button>
                </>
              )}
            </div>
          )}
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button size="sm">
                <Plus className="h-4 w-4 mr-1" />
                Add Item
                <ChevronDown className="h-4 w-4 ml-1" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem
                onClick={() => {
                  setItemSeq(String(nextSeq));
                  setDialogOpen(true);
                }}
              >
                <Plus className="h-4 w-4 mr-2" />
                Add Single Item
              </DropdownMenuItem>
              <DropdownMenuItem
                onClick={() => {
                  setMultiStartSeq(String(nextSeq));
                  setMultiDialogOpen(true);
                }}
              >
                <ListOrdered className="h-4 w-4 mr-2" />
                Add Multiple Items
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>

        {/* Single Item Dialog */}
        <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Add Lesson Item</DialogTitle>
              <DialogDescription>
                Add an existing video or study task to this lesson.
              </DialogDescription>
            </DialogHeader>
            <div className="space-y-4 py-4">
              {/* Sequence */}
              <div className="space-y-2">
                <Label htmlFor="item-seq">Sequence</Label>
                <Input
                  id="item-seq"
                  type="number"
                  min={1}
                  value={itemSeq}
                  onChange={(e) => setItemSeq(e.target.value)}
                  placeholder="e.g., 1"
                />
                <p className="text-xs text-muted-foreground">
                  Next available: {nextSeq}
                </p>
              </div>

              {/* Item Type */}
              <div className="space-y-2">
                <Label>Type</Label>
                <Select
                  value={itemKind}
                  onValueChange={(value: string) => setItemKind(value as "video" | "task")}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select type" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="video">Video</SelectItem>
                    <SelectItem value="task">Study Task</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {/* Video or Study Task Selection */}
              {itemKind === "video" ? (
                <div className="space-y-2">
                  <Label>Select Video</Label>
                  <div className="relative">
                    <div className="flex items-center border rounded-md">
                      <Search className="h-4 w-4 ml-2 text-muted-foreground" />
                      <Input
                        className="border-0 focus-visible:ring-0"
                        placeholder="Search videos by title..."
                        value={videoSearchQuery}
                        onChange={(e) => {
                          setVideoSearchQuery(e.target.value);
                          setVideoDropdownOpen(true);
                        }}
                        onFocus={() => setVideoDropdownOpen(true)}
                      />
                    </div>
                    {videoDropdownOpen && videoSearchQuery && (
                      <div className="absolute z-50 w-full mt-1 bg-background border rounded-md shadow-lg max-h-48 overflow-y-auto">
                        {videosLoading ? (
                          <div className="p-2 text-sm text-muted-foreground">Loading...</div>
                        ) : videosData?.items && videosData.items.length > 0 ? (
                          videosData.items.map((video) => (
                            <div
                              key={video.id}
                              className="flex items-center justify-between p-2 hover:bg-muted cursor-pointer"
                              onClick={() => {
                                setSelectedVideo({ id: video.id, title: video.title || video.video_idx });
                                setVideoSearchQuery("");
                                setVideoDropdownOpen(false);
                              }}
                            >
                              <div className="flex-1 min-w-0">
                                <p className="text-sm font-medium truncate">{video.title || video.video_idx}</p>
                                <p className="text-xs text-muted-foreground">
                                  ID: {video.id} | {video.video_state}
                                </p>
                              </div>
                            </div>
                          ))
                        ) : (
                          <div className="p-2 text-sm text-muted-foreground">No videos found</div>
                        )}
                      </div>
                    )}
                  </div>
                  {selectedVideo && (
                    <div className="flex items-center gap-2 p-2 bg-muted rounded-md">
                      <Check className="h-4 w-4 text-green-600" />
                      <span className="text-sm font-medium">{selectedVideo.title}</span>
                      <span className="text-xs text-muted-foreground">(ID: {selectedVideo.id})</span>
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        className="ml-auto h-6 px-2"
                        onClick={() => setSelectedVideo(null)}
                      >
                        Clear
                      </Button>
                    </div>
                  )}
                </div>
              ) : (
                <div className="space-y-2">
                  <Label>Select Study Task</Label>
                  {/* Step 1: Search and select study */}
                  <div className="relative">
                    <div className="flex items-center border rounded-md">
                      <Search className="h-4 w-4 ml-2 text-muted-foreground" />
                      <Input
                        className="border-0 focus-visible:ring-0"
                        placeholder="Search studies by title..."
                        value={studySearchQuery}
                        onChange={(e) => {
                          setStudySearchQuery(e.target.value);
                          setStudyDropdownOpen(true);
                        }}
                        onFocus={() => setStudyDropdownOpen(true)}
                      />
                    </div>
                    {studyDropdownOpen && studySearchQuery && (
                      <div className="absolute z-50 w-full mt-1 bg-background border rounded-md shadow-lg max-h-48 overflow-y-auto">
                        {studiesLoading ? (
                          <div className="p-2 text-sm text-muted-foreground">Loading...</div>
                        ) : studiesData?.list && studiesData.list.length > 0 ? (
                          studiesData.list.map((study) => (
                            <div
                              key={study.study_id}
                              className="flex items-center justify-between p-2 hover:bg-muted cursor-pointer"
                              onClick={() => {
                                setSelectedStudyId(study.study_id);
                                setSelectedTask(null);
                                setStudySearchQuery("");
                                setStudyDropdownOpen(false);
                              }}
                            >
                              <div className="flex-1 min-w-0">
                                <p className="text-sm font-medium truncate">{study.study_title || study.study_idx}</p>
                                <p className="text-xs text-muted-foreground">
                                  ID: {study.study_id} | {study.study_program}
                                </p>
                              </div>
                              <ChevronRight className="h-4 w-4 text-muted-foreground" />
                            </div>
                          ))
                        ) : (
                          <div className="p-2 text-sm text-muted-foreground">No studies found</div>
                        )}
                      </div>
                    )}
                  </div>

                  {/* Step 2: Show tasks from selected study */}
                  {selectedStudyId && (
                    <div className="border rounded-md p-2 space-y-2">
                      <p className="text-xs text-muted-foreground">
                        Tasks in study #{selectedStudyId}:
                        <Button
                          type="button"
                          variant="ghost"
                          size="sm"
                          className="ml-2 h-5 px-1 text-xs"
                          onClick={() => {
                            setSelectedStudyId(null);
                            setSelectedTask(null);
                          }}
                        >
                          Change
                        </Button>
                      </p>
                      {studyDetailLoading ? (
                        <div className="text-sm text-muted-foreground">Loading tasks...</div>
                      ) : studyDetail?.tasks && studyDetail.tasks.length > 0 ? (
                        <div className="max-h-32 overflow-y-auto space-y-1">
                          {studyDetail.tasks.map((task) => (
                            <div
                              key={task.study_task_id}
                              className={`flex items-center gap-2 p-2 rounded cursor-pointer ${
                                selectedTask?.id === task.study_task_id
                                  ? "bg-primary/10 border border-primary"
                                  : "hover:bg-muted"
                              }`}
                              onClick={() => {
                                setSelectedTask({
                                  id: task.study_task_id,
                                  kind: task.study_task_kind,
                                  seq: task.study_task_seq,
                                  question: task.question ?? undefined,
                                });
                              }}
                            >
                              {selectedTask?.id === task.study_task_id && (
                                <Check className="h-4 w-4 text-green-600" />
                              )}
                              <div className="flex-1 min-w-0">
                                <p className="text-sm">
                                  <span className="font-mono">#{task.study_task_seq}</span>{" "}
                                  <Badge variant="outline" className="text-xs">
                                    {task.study_task_kind}
                                  </Badge>
                                </p>
                                {task.question && (
                                  <p className="text-xs text-muted-foreground truncate">
                                    {task.question}
                                  </p>
                                )}
                              </div>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <div className="text-sm text-muted-foreground">No tasks in this study</div>
                      )}
                    </div>
                  )}

                  {selectedTask && (
                    <div className="flex items-center gap-2 p-2 bg-muted rounded-md">
                      <Check className="h-4 w-4 text-green-600" />
                      <span className="text-sm">
                        Task #{selectedTask.seq} ({selectedTask.kind})
                      </span>
                      <span className="text-xs text-muted-foreground">(ID: {selectedTask.id})</span>
                    </div>
                  )}
                </div>
              )}

              {/* Shift option */}
              <div className="flex items-center space-x-2">
                <Checkbox
                  id="shift-items"
                  checked={shiftIfExists}
                  onCheckedChange={(checked) => setShiftIfExists(checked === true)}
                />
                <Label htmlFor="shift-items" className="text-sm cursor-pointer">
                  Shift existing items if sequence already exists
                </Label>
              </div>
            </div>
            <DialogFooter>
              <Button variant="outline" onClick={() => setDialogOpen(false)}>
                Cancel
              </Button>
              <Button onClick={handleAddItem} disabled={createItemMutation.isPending}>
                {createItemMutation.isPending ? "Adding..." : "Add Item"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
        {/* Multi-Select Dialog */}
        <Dialog open={multiDialogOpen} onOpenChange={setMultiDialogOpen}>
          <DialogContent className="max-w-2xl">
            <DialogHeader>
              <DialogTitle>Add Multiple Items</DialogTitle>
              <DialogDescription>
                Select multiple videos or tasks to add to this lesson at once.
              </DialogDescription>
            </DialogHeader>
            <div className="space-y-4 py-4">
              {/* Starting Sequence */}
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label>Starting Sequence</Label>
                  <Input
                    type="number"
                    min={1}
                    value={multiStartSeq}
                    onChange={(e) => setMultiStartSeq(e.target.value)}
                    placeholder="e.g., 1"
                  />
                  <p className="text-xs text-muted-foreground">
                    Items will be assigned sequences {multiStartSeq || "?"}, {parseInt(multiStartSeq || "0") + 1 || "?"}, ...
                  </p>
                </div>
                <div className="space-y-2">
                  <Label>Type</Label>
                  <Select
                    value={multiItemKind}
                    onValueChange={(value: string) => {
                      setMultiItemKind(value as "video" | "task");
                      setSelectedVideos([]);
                      setSelectedTasks([]);
                    }}
                  >
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="video">Videos</SelectItem>
                      <SelectItem value="task">Study Tasks</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>

              {/* Multi-select Video */}
              {multiItemKind === "video" && (
                <div className="space-y-2">
                  <Label>Search & Select Videos</Label>
                  <div className="relative">
                    <div className="flex items-center border rounded-md">
                      <Search className="h-4 w-4 ml-2 text-muted-foreground" />
                      <Input
                        className="border-0 focus-visible:ring-0"
                        placeholder="Search videos..."
                        value={multiVideoSearchQuery}
                        onChange={(e) => {
                          setMultiVideoSearchQuery(e.target.value);
                          setMultiVideoDropdownOpen(true);
                        }}
                        onFocus={() => setMultiVideoDropdownOpen(true)}
                      />
                    </div>
                    {multiVideoDropdownOpen && multiVideoSearchQuery && (
                      <div className="absolute z-50 w-full mt-1 bg-background border rounded-md shadow-lg max-h-48 overflow-y-auto">
                        {multiVideosLoading ? (
                          <div className="p-2 text-sm text-muted-foreground">Loading...</div>
                        ) : multiVideosData?.items && multiVideosData.items.length > 0 ? (
                          multiVideosData.items.map((video) => {
                            const isSelected = selectedVideos.some((v) => v.id === video.id);
                            return (
                              <div
                                key={video.id}
                                className={`flex items-center gap-2 p-2 cursor-pointer ${
                                  isSelected ? "bg-primary/10" : "hover:bg-muted"
                                }`}
                                onClick={() => toggleVideoSelection({ id: video.id, title: video.title || video.video_idx })}
                              >
                                <Checkbox checked={isSelected} />
                                <div className="flex-1 min-w-0">
                                  <p className="text-sm font-medium truncate">{video.title || video.video_idx}</p>
                                  <p className="text-xs text-muted-foreground">ID: {video.id} | {video.video_state}</p>
                                </div>
                              </div>
                            );
                          })
                        ) : (
                          <div className="p-2 text-sm text-muted-foreground">No videos found</div>
                        )}
                      </div>
                    )}
                  </div>

                  {/* Selected Videos */}
                  {selectedVideos.length > 0 && (
                    <div className="border rounded-md p-2 space-y-1">
                      <p className="text-xs text-muted-foreground mb-2">
                        Selected ({selectedVideos.length}):
                      </p>
                      <div className="max-h-32 overflow-y-auto space-y-1">
                        {selectedVideos.map((video, idx) => (
                          <div key={video.id} className="flex items-center justify-between p-1.5 bg-muted rounded text-sm">
                            <span>
                              <span className="font-mono text-xs text-muted-foreground mr-2">
                                Seq {parseInt(multiStartSeq || "1") + idx}:
                              </span>
                              {video.title}
                            </span>
                            <Button
                              type="button"
                              variant="ghost"
                              size="sm"
                              className="h-5 w-5 p-0"
                              onClick={() => toggleVideoSelection(video)}
                            >
                              <X className="h-3 w-3" />
                            </Button>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}

              {/* Multi-select Study Tasks */}
              {multiItemKind === "task" && (
                <div className="space-y-2">
                  <Label>Search Study & Select Tasks</Label>
                  <div className="relative">
                    <div className="flex items-center border rounded-md">
                      <Search className="h-4 w-4 ml-2 text-muted-foreground" />
                      <Input
                        className="border-0 focus-visible:ring-0"
                        placeholder="Search studies..."
                        value={multiStudySearchQuery}
                        onChange={(e) => {
                          setMultiStudySearchQuery(e.target.value);
                          setMultiStudyDropdownOpen(true);
                        }}
                        onFocus={() => setMultiStudyDropdownOpen(true)}
                      />
                    </div>
                    {multiStudyDropdownOpen && multiStudySearchQuery && (
                      <div className="absolute z-50 w-full mt-1 bg-background border rounded-md shadow-lg max-h-48 overflow-y-auto">
                        {multiStudiesLoading ? (
                          <div className="p-2 text-sm text-muted-foreground">Loading...</div>
                        ) : multiStudiesData?.list && multiStudiesData.list.length > 0 ? (
                          multiStudiesData.list.map((study) => (
                            <div
                              key={study.study_id}
                              className="flex items-center justify-between p-2 hover:bg-muted cursor-pointer"
                              onClick={() => {
                                setMultiSelectedStudyId(study.study_id);
                                setMultiStudySearchQuery("");
                                setMultiStudyDropdownOpen(false);
                              }}
                            >
                              <div className="flex-1 min-w-0">
                                <p className="text-sm font-medium truncate">{study.study_title || study.study_idx}</p>
                                <p className="text-xs text-muted-foreground">ID: {study.study_id} | {study.study_program}</p>
                              </div>
                              <ChevronRight className="h-4 w-4 text-muted-foreground" />
                            </div>
                          ))
                        ) : (
                          <div className="p-2 text-sm text-muted-foreground">No studies found</div>
                        )}
                      </div>
                    )}
                  </div>

                  {/* Tasks from selected study */}
                  {multiSelectedStudyId && (
                    <div className="border rounded-md p-2 space-y-2">
                      <div className="flex items-center justify-between">
                        <p className="text-xs text-muted-foreground">
                          Tasks in study #{multiSelectedStudyId}
                        </p>
                        <Button
                          type="button"
                          variant="ghost"
                          size="sm"
                          className="h-5 px-1 text-xs"
                          onClick={() => {
                            setMultiSelectedStudyId(null);
                            setSelectedTasks([]);
                          }}
                        >
                          Change Study
                        </Button>
                      </div>
                      {multiStudyDetailLoading ? (
                        <div className="text-sm text-muted-foreground">Loading tasks...</div>
                      ) : multiStudyDetail?.tasks && multiStudyDetail.tasks.length > 0 ? (
                        <div className="max-h-40 overflow-y-auto space-y-1">
                          {multiStudyDetail.tasks.map((task) => {
                            const isSelected = selectedTasks.some((t) => t.id === task.study_task_id);
                            return (
                              <div
                                key={task.study_task_id}
                                className={`flex items-center gap-2 p-2 rounded cursor-pointer ${
                                  isSelected ? "bg-primary/10 border border-primary" : "hover:bg-muted"
                                }`}
                                onClick={() => toggleTaskSelection({
                                  id: task.study_task_id,
                                  kind: task.study_task_kind,
                                  seq: task.study_task_seq,
                                  question: task.question ?? undefined,
                                })}
                              >
                                <Checkbox checked={isSelected} />
                                <div className="flex-1 min-w-0">
                                  <p className="text-sm">
                                    <span className="font-mono">#{task.study_task_seq}</span>{" "}
                                    <Badge variant="outline" className="text-xs">{task.study_task_kind}</Badge>
                                  </p>
                                  {task.question && (
                                    <p className="text-xs text-muted-foreground truncate">{task.question}</p>
                                  )}
                                </div>
                              </div>
                            );
                          })}
                        </div>
                      ) : (
                        <div className="text-sm text-muted-foreground">No tasks in this study</div>
                      )}
                    </div>
                  )}

                  {/* Selected Tasks Summary */}
                  {selectedTasks.length > 0 && (
                    <div className="border rounded-md p-2 bg-muted/50">
                      <p className="text-xs text-muted-foreground mb-1">Selected ({selectedTasks.length}):</p>
                      <div className="flex flex-wrap gap-1">
                        {selectedTasks.map((task, idx) => (
                          <Badge key={task.id} variant="secondary" className="text-xs">
                            Seq {parseInt(multiStartSeq || "1") + idx}: Task #{task.seq} ({task.kind})
                            <button
                              className="ml-1 hover:text-destructive"
                              onClick={() => toggleTaskSelection(task)}
                            >
                              <X className="h-3 w-3" />
                            </button>
                          </Badge>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>
            <DialogFooter>
              <Button variant="outline" onClick={() => setMultiDialogOpen(false)}>
                Cancel
              </Button>
              <Button
                onClick={handleAddMultipleItems}
                disabled={bulkCreateMutation.isPending || (multiItemKind === "video" ? selectedVideos.length === 0 : selectedTasks.length === 0)}
              >
                {bulkCreateMutation.isPending
                  ? "Adding..."
                  : `Add ${multiItemKind === "video" ? selectedVideos.length : selectedTasks.length} Items`}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

        {/* Edit Item Dialog */}
        <Dialog open={editDialogOpen} onOpenChange={setEditDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Edit Lesson Item</DialogTitle>
              <DialogDescription>
                Modify the sequence, type, or linked content of this item.
              </DialogDescription>
            </DialogHeader>
            <div className="space-y-4 py-4">
              {/* Current Info */}
              {editingItem && (
                <div className="p-2 bg-muted rounded-md text-sm">
                  <p className="text-muted-foreground">
                    Editing item at sequence <span className="font-mono font-bold">{editingItem.originalSeq}</span>
                  </p>
                </div>
              )}

              {/* New Sequence */}
              <div className="space-y-2">
                <Label htmlFor="edit-seq">Sequence</Label>
                <Input
                  id="edit-seq"
                  type="number"
                  min={1}
                  value={editSeq}
                  onChange={(e) => setEditSeq(e.target.value)}
                  placeholder="e.g., 1"
                />
              </div>

              {/* Type */}
              <div className="space-y-2">
                <Label>Type</Label>
                <Select
                  value={editKind}
                  onValueChange={(value: string) => {
                    setEditKind(value as "video" | "task");
                    if (value === "video") {
                      setEditSelectedTask(null);
                    } else {
                      setEditSelectedVideo(null);
                    }
                  }}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select type" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="video">Video</SelectItem>
                    <SelectItem value="task">Study Task</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {/* Video Selection */}
              {editKind === "video" && (
                <div className="space-y-2">
                  <Label>Select Video</Label>
                  <div className="relative">
                    <div className="flex items-center border rounded-md">
                      <Search className="h-4 w-4 ml-2 text-muted-foreground" />
                      <Input
                        className="border-0 focus-visible:ring-0"
                        placeholder="Search videos..."
                        value={editVideoSearchQuery}
                        onChange={(e) => {
                          setEditVideoSearchQuery(e.target.value);
                          setEditVideoDropdownOpen(true);
                        }}
                        onFocus={() => setEditVideoDropdownOpen(true)}
                      />
                    </div>
                    {editVideoDropdownOpen && editVideoSearchQuery && (
                      <div className="absolute z-50 w-full mt-1 bg-background border rounded-md shadow-lg max-h-48 overflow-y-auto">
                        {editVideosLoading ? (
                          <div className="p-2 text-sm text-muted-foreground">Loading...</div>
                        ) : editVideosData?.items && editVideosData.items.length > 0 ? (
                          editVideosData.items.map((video) => (
                            <div
                              key={video.id}
                              className="flex items-center justify-between p-2 hover:bg-muted cursor-pointer"
                              onClick={() => {
                                setEditSelectedVideo({ id: video.id, title: video.title || video.video_idx });
                                setEditVideoSearchQuery("");
                                setEditVideoDropdownOpen(false);
                              }}
                            >
                              <div className="flex-1 min-w-0">
                                <p className="text-sm font-medium truncate">{video.title || video.video_idx}</p>
                                <p className="text-xs text-muted-foreground">ID: {video.id} | {video.video_state}</p>
                              </div>
                            </div>
                          ))
                        ) : (
                          <div className="p-2 text-sm text-muted-foreground">No videos found</div>
                        )}
                      </div>
                    )}
                  </div>
                  {editSelectedVideo && (
                    <div className="flex items-center gap-2 p-2 bg-muted rounded-md">
                      <Check className="h-4 w-4 text-green-600" />
                      <span className="text-sm font-medium">{editSelectedVideo.title}</span>
                      <span className="text-xs text-muted-foreground">(ID: {editSelectedVideo.id})</span>
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        className="ml-auto h-6 px-2"
                        onClick={() => setEditSelectedVideo(null)}
                      >
                        Clear
                      </Button>
                    </div>
                  )}
                </div>
              )}

              {/* Task Selection */}
              {editKind === "task" && (
                <div className="space-y-2">
                  <Label>Select Study Task</Label>
                  <div className="relative">
                    <div className="flex items-center border rounded-md">
                      <Search className="h-4 w-4 ml-2 text-muted-foreground" />
                      <Input
                        className="border-0 focus-visible:ring-0"
                        placeholder="Search studies..."
                        value={editStudySearchQuery}
                        onChange={(e) => {
                          setEditStudySearchQuery(e.target.value);
                          setEditStudyDropdownOpen(true);
                        }}
                        onFocus={() => setEditStudyDropdownOpen(true)}
                      />
                    </div>
                    {editStudyDropdownOpen && editStudySearchQuery && (
                      <div className="absolute z-50 w-full mt-1 bg-background border rounded-md shadow-lg max-h-48 overflow-y-auto">
                        {editStudiesLoading ? (
                          <div className="p-2 text-sm text-muted-foreground">Loading...</div>
                        ) : editStudiesData?.list && editStudiesData.list.length > 0 ? (
                          editStudiesData.list.map((study) => (
                            <div
                              key={study.study_id}
                              className="flex items-center justify-between p-2 hover:bg-muted cursor-pointer"
                              onClick={() => {
                                setEditSelectedStudyId(study.study_id);
                                setEditStudySearchQuery("");
                                setEditStudyDropdownOpen(false);
                              }}
                            >
                              <div className="flex-1 min-w-0">
                                <p className="text-sm font-medium truncate">{study.study_title || study.study_idx}</p>
                                <p className="text-xs text-muted-foreground">ID: {study.study_id} | {study.study_program}</p>
                              </div>
                              <ChevronRight className="h-4 w-4 text-muted-foreground" />
                            </div>
                          ))
                        ) : (
                          <div className="p-2 text-sm text-muted-foreground">No studies found</div>
                        )}
                      </div>
                    )}
                  </div>

                  {/* Tasks from selected study */}
                  {editSelectedStudyId && (
                    <div className="border rounded-md p-2 space-y-2">
                      <p className="text-xs text-muted-foreground">
                        Tasks in study #{editSelectedStudyId}:
                        <Button
                          type="button"
                          variant="ghost"
                          size="sm"
                          className="ml-2 h-5 px-1 text-xs"
                          onClick={() => {
                            setEditSelectedStudyId(null);
                            setEditSelectedTask(null);
                          }}
                        >
                          Change
                        </Button>
                      </p>
                      {editStudyDetailLoading ? (
                        <div className="text-sm text-muted-foreground">Loading tasks...</div>
                      ) : editStudyDetail?.tasks && editStudyDetail.tasks.length > 0 ? (
                        <div className="max-h-32 overflow-y-auto space-y-1">
                          {editStudyDetail.tasks.map((task) => (
                            <div
                              key={task.study_task_id}
                              className={`flex items-center gap-2 p-2 rounded cursor-pointer ${
                                editSelectedTask?.id === task.study_task_id
                                  ? "bg-primary/10 border border-primary"
                                  : "hover:bg-muted"
                              }`}
                              onClick={() => {
                                setEditSelectedTask({
                                  id: task.study_task_id,
                                  kind: task.study_task_kind,
                                  seq: task.study_task_seq,
                                });
                              }}
                            >
                              {editSelectedTask?.id === task.study_task_id && (
                                <Check className="h-4 w-4 text-green-600" />
                              )}
                              <div className="flex-1 min-w-0">
                                <p className="text-sm">
                                  <span className="font-mono">#{task.study_task_seq}</span>{" "}
                                  <Badge variant="outline" className="text-xs">{task.study_task_kind}</Badge>
                                </p>
                                {task.question && (
                                  <p className="text-xs text-muted-foreground truncate">{task.question}</p>
                                )}
                              </div>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <div className="text-sm text-muted-foreground">No tasks in this study</div>
                      )}
                    </div>
                  )}

                  {editSelectedTask && (
                    <div className="flex items-center gap-2 p-2 bg-muted rounded-md">
                      <Check className="h-4 w-4 text-green-600" />
                      <span className="text-sm">Task #{editSelectedTask.seq} ({editSelectedTask.kind})</span>
                      <span className="text-xs text-muted-foreground">(ID: {editSelectedTask.id})</span>
                    </div>
                  )}
                </div>
              )}
            </div>
            <DialogFooter>
              <Button variant="outline" onClick={() => setEditDialogOpen(false)}>
                Cancel
              </Button>
              <Button onClick={handleEditItem} disabled={updateItemMutation.isPending}>
                {updateItemMutation.isPending ? "Saving..." : "Save Changes"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </CardHeader>
      <CardContent>
        {isReordering && (
          <div className="absolute inset-0 bg-background/50 flex items-center justify-center z-10 rounded-lg">
            <div className="flex items-center gap-2 text-muted-foreground">
              <div className="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent" />
              <span>Saving order...</span>
            </div>
          </div>
        )}
        {!orderedItems || orderedItems.length === 0 ? (
          <p className="text-center text-muted-foreground py-8">No items in this lesson</p>
        ) : (
          <DndContext
            sensors={sensors}
            collisionDetection={closestCenter}
            onDragEnd={handleDragEnd}
          >
            <SortableContext items={itemIds} strategy={verticalListSortingStrategy}>
              <div className="space-y-2">
                {orderedItems.map((item) => (
                  <SortableItem
                    key={`${item.lesson_id}-${item.lesson_item_seq}`}
                    id={`${item.lesson_id}-${item.lesson_item_seq}`}
                    item={item}
                    isSelected={selectedItems.has(item.lesson_item_seq)}
                    onToggleSelection={toggleItemSelection}
                    onEdit={() => openEditDialog(item)}
                    onDelete={() => openDeleteDialog(item)}
                    getKindBadgeVariant={getKindBadgeVariant}
                  />
                ))}
              </div>
            </SortableContext>
          </DndContext>
        )}

        {/* Delete Confirmation Dialog */}
        <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Delete Lesson Item</DialogTitle>
              <DialogDescription>
                Are you sure you want to remove this item from the lesson?
                This will not delete the original video or task.
              </DialogDescription>
            </DialogHeader>
            {deletingItem && (
              <div className="py-4">
                <div className="p-4 bg-muted rounded-md space-y-2">
                  <p className="text-sm">
                    <span className="text-muted-foreground">Sequence:</span>{" "}
                    <span className="font-mono font-bold">{deletingItem.seq}</span>
                  </p>
                  <p className="text-sm">
                    <span className="text-muted-foreground">Type:</span>{" "}
                    <Badge variant="outline">{deletingItem.kind}</Badge>
                  </p>
                  <p className="text-sm">
                    <span className="text-muted-foreground">Content:</span>{" "}
                    {deletingItem.title}
                  </p>
                </div>
              </div>
            )}
            <DialogFooter>
              <Button variant="outline" onClick={() => setDeleteDialogOpen(false)}>
                Cancel
              </Button>
              <Button
                variant="destructive"
                onClick={handleDeleteItem}
                disabled={deleteItemMutation.isPending}
              >
                {deleteItemMutation.isPending ? "Deleting..." : "Delete Item"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

        {/* Bulk Edit Dialog */}
        <Dialog open={bulkEditDialogOpen} onOpenChange={setBulkEditDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Bulk Edit Lesson Items</DialogTitle>
              <DialogDescription>
                Change the linked content for {selectedItems.size} selected item{selectedItems.size > 1 ? "s" : ""}.
                This will replace the current video/task with the new selection.
              </DialogDescription>
            </DialogHeader>
            <div className="space-y-4 py-4">
              {/* Selected Items Summary */}
              <div className="p-3 bg-muted rounded-md">
                <p className="text-sm text-muted-foreground mb-1">Selected sequences:</p>
                <div className="flex flex-wrap gap-1">
                  {Array.from(selectedItems).sort((a, b) => a - b).map((seq) => (
                    <Badge key={seq} variant="outline" className="font-mono">
                      {seq}
                    </Badge>
                  ))}
                </div>
              </div>

              {/* New Content Type Selection */}
              <div className="space-y-2">
                <Label>New Content Type</Label>
                <Select
                  value={bulkEditKind}
                  onValueChange={(value: string) => {
                    setBulkEditKind(value as "video" | "task" | "");
                    setBulkEditSelectedVideo(null);
                    setBulkEditSelectedTask(null);
                  }}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select new content type" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="video">Video</SelectItem>
                    <SelectItem value="task">Study Task</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {/* Video Selection for Bulk Edit */}
              {bulkEditKind === "video" && (
                <div className="space-y-2">
                  <Label>Select Video</Label>
                  <div className="relative">
                    <div className="flex items-center border rounded-md">
                      <Search className="h-4 w-4 ml-2 text-muted-foreground" />
                      <Input
                        className="border-0 focus-visible:ring-0"
                        placeholder="Search videos..."
                        value={bulkEditVideoSearchQuery}
                        onChange={(e) => {
                          setBulkEditVideoSearchQuery(e.target.value);
                          setBulkEditVideoDropdownOpen(true);
                        }}
                        onFocus={() => setBulkEditVideoDropdownOpen(true)}
                      />
                    </div>
                    {bulkEditVideoDropdownOpen && bulkEditVideoSearchQuery && (
                      <div className="absolute z-50 w-full mt-1 bg-background border rounded-md shadow-lg max-h-48 overflow-y-auto">
                        {bulkEditVideosLoading ? (
                          <div className="p-2 text-sm text-muted-foreground">Loading...</div>
                        ) : bulkEditVideosData?.items && bulkEditVideosData.items.length > 0 ? (
                          bulkEditVideosData.items.map((video) => (
                            <div
                              key={video.id}
                              className="flex items-center justify-between p-2 hover:bg-muted cursor-pointer"
                              onClick={() => {
                                setBulkEditSelectedVideo({ id: video.id, title: video.title || video.video_idx });
                                setBulkEditVideoSearchQuery("");
                                setBulkEditVideoDropdownOpen(false);
                              }}
                            >
                              <div className="flex-1 min-w-0">
                                <p className="text-sm font-medium truncate">{video.title || video.video_idx}</p>
                                <p className="text-xs text-muted-foreground">ID: {video.id} | {video.video_state}</p>
                              </div>
                            </div>
                          ))
                        ) : (
                          <div className="p-2 text-sm text-muted-foreground">No videos found</div>
                        )}
                      </div>
                    )}
                  </div>
                  {bulkEditSelectedVideo && (
                    <div className="flex items-center gap-2 p-2 bg-muted rounded-md">
                      <Check className="h-4 w-4 text-green-600" />
                      <span className="text-sm font-medium">{bulkEditSelectedVideo.title}</span>
                      <span className="text-xs text-muted-foreground">(ID: {bulkEditSelectedVideo.id})</span>
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        className="ml-auto h-6 px-2"
                        onClick={() => setBulkEditSelectedVideo(null)}
                      >
                        Clear
                      </Button>
                    </div>
                  )}
                </div>
              )}

              {/* Task Selection for Bulk Edit */}
              {bulkEditKind === "task" && (
                <div className="space-y-2">
                  <Label>Select Study Task</Label>
                  <div className="relative">
                    <div className="flex items-center border rounded-md">
                      <Search className="h-4 w-4 ml-2 text-muted-foreground" />
                      <Input
                        className="border-0 focus-visible:ring-0"
                        placeholder="Search studies..."
                        value={bulkEditStudySearchQuery}
                        onChange={(e) => {
                          setBulkEditStudySearchQuery(e.target.value);
                          setBulkEditStudyDropdownOpen(true);
                        }}
                        onFocus={() => setBulkEditStudyDropdownOpen(true)}
                      />
                    </div>
                    {bulkEditStudyDropdownOpen && bulkEditStudySearchQuery && (
                      <div className="absolute z-50 w-full mt-1 bg-background border rounded-md shadow-lg max-h-48 overflow-y-auto">
                        {bulkEditStudiesLoading ? (
                          <div className="p-2 text-sm text-muted-foreground">Loading...</div>
                        ) : bulkEditStudiesData?.list && bulkEditStudiesData.list.length > 0 ? (
                          bulkEditStudiesData.list.map((study) => (
                            <div
                              key={study.study_id}
                              className="flex items-center justify-between p-2 hover:bg-muted cursor-pointer"
                              onClick={() => {
                                setBulkEditSelectedStudyId(study.study_id);
                                setBulkEditStudySearchQuery("");
                                setBulkEditStudyDropdownOpen(false);
                              }}
                            >
                              <div className="flex-1 min-w-0">
                                <p className="text-sm font-medium truncate">{study.study_title || study.study_idx}</p>
                                <p className="text-xs text-muted-foreground">ID: {study.study_id} | {study.study_program}</p>
                              </div>
                              <ChevronRight className="h-4 w-4 text-muted-foreground" />
                            </div>
                          ))
                        ) : (
                          <div className="p-2 text-sm text-muted-foreground">No studies found</div>
                        )}
                      </div>
                    )}
                  </div>

                  {/* Tasks from selected study */}
                  {bulkEditSelectedStudyId && (
                    <div className="border rounded-md p-2 space-y-2">
                      <p className="text-xs text-muted-foreground">
                        Tasks in study #{bulkEditSelectedStudyId}:
                        <Button
                          type="button"
                          variant="ghost"
                          size="sm"
                          className="ml-2 h-5 px-1 text-xs"
                          onClick={() => {
                            setBulkEditSelectedStudyId(null);
                            setBulkEditSelectedTask(null);
                          }}
                        >
                          Change
                        </Button>
                      </p>
                      {bulkEditStudyDetailLoading ? (
                        <div className="text-sm text-muted-foreground">Loading tasks...</div>
                      ) : bulkEditStudyDetail?.tasks && bulkEditStudyDetail.tasks.length > 0 ? (
                        <div className="max-h-32 overflow-y-auto space-y-1">
                          {bulkEditStudyDetail.tasks.map((task) => (
                            <div
                              key={task.study_task_id}
                              className={`flex items-center gap-2 p-2 rounded cursor-pointer ${
                                bulkEditSelectedTask?.id === task.study_task_id
                                  ? "bg-primary/10 border border-primary"
                                  : "hover:bg-muted"
                              }`}
                              onClick={() => {
                                setBulkEditSelectedTask({
                                  id: task.study_task_id,
                                  kind: task.study_task_kind,
                                  seq: task.study_task_seq,
                                });
                              }}
                            >
                              {bulkEditSelectedTask?.id === task.study_task_id && (
                                <Check className="h-4 w-4 text-green-600" />
                              )}
                              <div className="flex-1 min-w-0">
                                <p className="text-sm">
                                  <span className="font-mono">#{task.study_task_seq}</span>{" "}
                                  <Badge variant="outline" className="text-xs">{task.study_task_kind}</Badge>
                                </p>
                                {task.question && (
                                  <p className="text-xs text-muted-foreground truncate">{task.question}</p>
                                )}
                              </div>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <div className="text-sm text-muted-foreground">No tasks in this study</div>
                      )}
                    </div>
                  )}

                  {bulkEditSelectedTask && (
                    <div className="flex items-center gap-2 p-2 bg-muted rounded-md">
                      <Check className="h-4 w-4 text-green-600" />
                      <span className="text-sm">Task #{bulkEditSelectedTask.seq} ({bulkEditSelectedTask.kind})</span>
                      <span className="text-xs text-muted-foreground">(ID: {bulkEditSelectedTask.id})</span>
                    </div>
                  )}
                </div>
              )}
            </div>
            <DialogFooter>
              <Button variant="outline" onClick={() => {
                setBulkEditDialogOpen(false);
                resetBulkEditForm();
              }}>
                Cancel
              </Button>
              <Button
                onClick={handleBulkEdit}
                disabled={bulkUpdateItemsMutation.isPending || !bulkEditKind}
              >
                {bulkUpdateItemsMutation.isPending
                  ? "Updating..."
                  : `Update ${selectedItems.size} Item${selectedItems.size > 1 ? "s" : ""}`}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

        {/* Bulk Delete Dialog */}
        <Dialog open={bulkDeleteDialogOpen} onOpenChange={setBulkDeleteDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Delete Multiple Items</DialogTitle>
              <DialogDescription>
                Are you sure you want to delete {selectedItems.size} selected item{selectedItems.size > 1 ? "s" : ""}?
                This will remove them from the lesson but will not delete the original videos or tasks.
              </DialogDescription>
            </DialogHeader>
            <div className="py-4">
              <div className="p-3 bg-muted rounded-md">
                <p className="text-sm text-muted-foreground mb-1">Selected sequences to delete:</p>
                <div className="flex flex-wrap gap-1">
                  {Array.from(selectedItems).sort((a, b) => a - b).map((seq) => (
                    <Badge key={seq} variant="outline" className="font-mono">
                      {seq}
                    </Badge>
                  ))}
                </div>
              </div>
            </div>
            <DialogFooter>
              <Button variant="outline" onClick={() => setBulkDeleteDialogOpen(false)}>
                Cancel
              </Button>
              <Button
                variant="destructive"
                onClick={handleBulkDelete}
                disabled={bulkDeleteItemsMutation.isPending}
              >
                {bulkDeleteItemsMutation.isPending
                  ? "Deleting..."
                  : `Delete ${selectedItems.size} Item${selectedItems.size > 1 ? "s" : ""}`}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </CardContent>
    </Card>
  );
}

// Lesson Progress Tab Component
function LessonProgressTab({ lessonId }: { lessonId: number }) {
  const { data, isLoading, isError, refetch } = useAdminLessonProgressDetail(lessonId);
  const { data: itemsData } = useAdminLessonItemsDetail(lessonId);
  const updateMutation = useUpdateAdminLessonProgress();
  const bulkUpdateMutation = useUpdateAdminLessonProgressBulk();

  // Get max seq from lesson items
  const maxItemSeq = useMemo(() => {
    if (!itemsData?.items || itemsData.items.length === 0) return 1;
    return Math.max(...itemsData.items.map((item) => item.lesson_item_seq));
  }, [itemsData]);

  const [selectedItems, setSelectedItems] = useState<Set<number>>(new Set());
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [bulkDialogOpen, setBulkDialogOpen] = useState(false);
  const [editingProgress, setEditingProgress] = useState<AdminLessonProgressDetailRes | null>(null);

  // Single edit form state
  const [editPercent, setEditPercent] = useState<string>("");
  const [editLastSeq, setEditLastSeq] = useState<string>("");

  // Bulk edit form state
  const [bulkPercent, setBulkPercent] = useState<string>("");
  const [bulkLastSeq, setBulkLastSeq] = useState<string>("");

  const handleSelectAll = (checked: boolean) => {
    if (checked && data?.list) {
      setSelectedItems(new Set(data.list.map((p) => p.user_id)));
    } else {
      setSelectedItems(new Set());
    }
  };

  const handleSelectItem = (userId: number, checked: boolean) => {
    const newSelected = new Set(selectedItems);
    if (checked) {
      newSelected.add(userId);
    } else {
      newSelected.delete(userId);
    }
    setSelectedItems(newSelected);
  };

  const handleEditClick = (progress: AdminLessonProgressDetailRes) => {
    setEditingProgress(progress);
    setEditPercent(String(progress.lesson_progress_percent));
    setEditLastSeq(progress.lesson_progress_last_item_seq ? String(progress.lesson_progress_last_item_seq) : "");
    setEditDialogOpen(true);
  };

  const handleSingleUpdate = async () => {
    if (!editingProgress) return;

    const req: LessonProgressUpdateReq = {
      user_id: editingProgress.user_id,
    };

    if (editPercent !== "") {
      const percent = parseInt(editPercent, 10);
      if (!isNaN(percent) && percent >= 0 && percent <= 100) {
        req.lesson_progress_percent = percent;
      }
    }

    if (editLastSeq !== "") {
      const seq = parseInt(editLastSeq, 10);
      if (!isNaN(seq) && seq >= 1 && seq <= maxItemSeq) {
        req.lesson_progress_last_item_seq = seq;
      } else if (!isNaN(seq) && seq > maxItemSeq) {
        toast.error(`Last Item Seq must be between 1 and ${maxItemSeq}`);
        return;
      }
    }

    try {
      await updateMutation.mutateAsync({ lessonId, data: req });
      toast.success(`User #${editingProgress.user_id} progress updated`);
      setEditDialogOpen(false);
      setEditingProgress(null);
      refetch();
    } catch {
      toast.error("Failed to update progress");
    }
  };

  const handleBulkUpdate = async () => {
    if (selectedItems.size === 0) return;

    const items = Array.from(selectedItems).map((userId) => {
      const item: {
        lesson_id: number;
        user_id: number;
        lesson_progress_percent?: number;
        lesson_progress_last_item_seq?: number;
      } = {
        lesson_id: lessonId,
        user_id: userId,
      };

      if (bulkPercent !== "") {
        const percent = parseInt(bulkPercent, 10);
        if (!isNaN(percent) && percent >= 0 && percent <= 100) {
          item.lesson_progress_percent = percent;
        }
      }

      if (bulkLastSeq !== "") {
        const seq = parseInt(bulkLastSeq, 10);
        if (!isNaN(seq) && seq >= 1 && seq <= maxItemSeq) {
          item.lesson_progress_last_item_seq = seq;
        }
      }

      return item;
    });

    // Validate bulkLastSeq before making API call
    if (bulkLastSeq !== "") {
      const seq = parseInt(bulkLastSeq, 10);
      if (!isNaN(seq) && seq > maxItemSeq) {
        toast.error(`Last Item Seq must be between 1 and ${maxItemSeq}`);
        return;
      }
    }

    try {
      const result = await bulkUpdateMutation.mutateAsync({ items });
      if (result.failure_count === 0) {
        toast.success(`${result.success_count} progress records updated`);
      } else {
        toast.warning(`${result.success_count} updated, ${result.failure_count} failed`);
      }
      setBulkDialogOpen(false);
      setSelectedItems(new Set());
      setBulkPercent("");
      setBulkLastSeq("");
      refetch();
    } catch {
      toast.error("Failed to bulk update progress");
    }
  };

  if (isLoading) {
    return (
      <Card>
        <CardHeader>
          <Skeleton className="h-6 w-32" />
        </CardHeader>
        <CardContent>
          {Array.from({ length: 3 }).map((_, i) => (
            <div key={i} className="flex items-center gap-4 p-4 border-b">
              <Skeleton className="h-4 w-16" />
              <Skeleton className="h-4 w-24" />
              <Skeleton className="h-4 w-32" />
            </div>
          ))}
        </CardContent>
      </Card>
    );
  }

  if (isError || !data) {
    return (
      <Card>
        <CardContent className="py-8 text-center text-destructive">
          Failed to load progress data
        </CardContent>
      </Card>
    );
  }

  const allSelected = data.list.length > 0 && selectedItems.size === data.list.length;
  const someSelected = selectedItems.size > 0 && selectedItems.size < data.list.length;

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle className="flex items-center gap-2">
              <Users className="h-5 w-5" />
              User Progress ({data.total_progress})
            </CardTitle>
            <CardDescription>{data.lesson_title}</CardDescription>
          </div>
          {selectedItems.size > 0 && (
            <Button
              variant="outline"
              size="sm"
              onClick={() => setBulkDialogOpen(true)}
            >
              <Pencil className="h-4 w-4 mr-2" />
              Bulk Edit ({selectedItems.size})
            </Button>
          )}
        </div>
      </CardHeader>
      <CardContent>
        {data.list.length === 0 ? (
          <p className="text-center text-muted-foreground py-8">
            No user progress for this lesson
          </p>
        ) : (
          <div className="rounded-md border">
            <table className="w-full text-sm">
              <thead className="border-b bg-muted/50">
                <tr>
                  <th className="h-10 px-4 text-left font-medium w-10">
                    <Checkbox
                      checked={allSelected}
                      ref={(el) => {
                        if (el) (el as HTMLButtonElement & { indeterminate?: boolean }).indeterminate = someSelected;
                      }}
                      onCheckedChange={handleSelectAll}
                    />
                  </th>
                  <th className="h-10 px-4 text-left font-medium">User ID</th>
                  <th className="h-10 px-4 text-left font-medium">Progress</th>
                  <th className="h-10 px-4 text-left font-medium">Last Item</th>
                  <th className="h-10 px-4 text-left font-medium">Last Progress At</th>
                  <th className="h-10 px-4 text-left font-medium w-20">Actions</th>
                </tr>
              </thead>
              <tbody>
                {data.list.map((progress) => (
                  <tr
                    key={`${progress.lesson_id}-${progress.user_id}`}
                    className="border-b hover:bg-muted/50"
                  >
                    <td className="p-4">
                      <Checkbox
                        checked={selectedItems.has(progress.user_id)}
                        onCheckedChange={(checked) => handleSelectItem(progress.user_id, !!checked)}
                      />
                    </td>
                    <td className="p-4 font-mono">#{progress.user_id}</td>
                    <td className="p-4">
                      <div className="flex items-center gap-2">
                        <Progress value={progress.lesson_progress_percent} className="w-24" />
                        <span className="text-sm">{progress.lesson_progress_percent}%</span>
                      </div>
                    </td>
                    <td className="p-4">
                      {progress.lesson_progress_last_item_seq ? (
                        <Badge variant="outline">Seq {progress.lesson_progress_last_item_seq}</Badge>
                      ) : (
                        "-"
                      )}
                      {progress.current_item && (
                        <span className="ml-2 text-xs text-muted-foreground">
                          ({progress.current_item.lesson_item_kind})
                        </span>
                      )}
                    </td>
                    <td className="p-4 text-muted-foreground text-xs">
                      {progress.lesson_progress_last_progress_at
                        ? new Date(progress.lesson_progress_last_progress_at).toLocaleString()
                        : "-"}
                    </td>
                    <td className="p-4">
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => handleEditClick(progress)}
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
      </CardContent>

      {/* Single Edit Dialog */}
      <Dialog open={editDialogOpen} onOpenChange={setEditDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Edit User Progress</DialogTitle>
            <DialogDescription>
              Update progress for User #{editingProgress?.user_id}
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="edit-percent">Progress Percent (0-100)</Label>
              <Input
                id="edit-percent"
                type="number"
                min={0}
                max={100}
                value={editPercent}
                onChange={(e) => setEditPercent(e.target.value)}
                placeholder="Leave empty to keep current"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="edit-seq">Last Item Seq (1-{maxItemSeq})</Label>
              <Input
                id="edit-seq"
                type="number"
                min={1}
                max={maxItemSeq}
                value={editLastSeq}
                onChange={(e) => setEditLastSeq(e.target.value)}
                placeholder="Leave empty to keep current"
              />
              <p className="text-xs text-muted-foreground">
                Max: {maxItemSeq} (total items in this lesson)
              </p>
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setEditDialogOpen(false)}>
              Cancel
            </Button>
            <Button
              onClick={handleSingleUpdate}
              disabled={updateMutation.isPending}
            >
              {updateMutation.isPending ? "Saving..." : "Save"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Bulk Edit Dialog */}
      <Dialog open={bulkDialogOpen} onOpenChange={setBulkDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Edit Progress</DialogTitle>
            <DialogDescription>
              Update progress for {selectedItems.size} selected users
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="bulk-percent">Progress Percent (0-100)</Label>
              <Input
                id="bulk-percent"
                type="number"
                min={0}
                max={100}
                value={bulkPercent}
                onChange={(e) => setBulkPercent(e.target.value)}
                placeholder="Leave empty to skip"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="bulk-seq">Last Item Seq (1-{maxItemSeq})</Label>
              <Input
                id="bulk-seq"
                type="number"
                min={1}
                max={maxItemSeq}
                value={bulkLastSeq}
                onChange={(e) => setBulkLastSeq(e.target.value)}
                placeholder="Leave empty to skip"
              />
              <p className="text-xs text-muted-foreground">
                Max: {maxItemSeq} (total items in this lesson)
              </p>
            </div>
            <p className="text-xs text-muted-foreground">
              Only filled fields will be updated. Empty fields will keep their current values.
            </p>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setBulkDialogOpen(false)}>
              Cancel
            </Button>
            <Button
              onClick={handleBulkUpdate}
              disabled={bulkUpdateMutation.isPending || (bulkPercent === "" && bulkLastSeq === "")}
            >
              {bulkUpdateMutation.isPending ? "Updating..." : `Update ${selectedItems.size} Users`}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Card>
  );
}
