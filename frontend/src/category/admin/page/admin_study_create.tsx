import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useForm, useFieldArray } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ArrowLeft, Plus, Trash2, CheckCircle, ListPlus, Upload, FileText, AlertCircle } from "lucide-react";
import { toast } from "sonner";
import { z } from "zod";

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
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

import {
  useCreateAdminStudy,
  useCreateAdminStudyTask,
  useCreateAdminStudyTasksBulk,
} from "../hook/use_admin_studies";
import {
  studyCreateReqSchema,
  type StudyCreateReq,
  type StudyState,
  type StudyAccess,
  type AdminStudyRes,
} from "../types";
import type { StudyProgram } from "../../study/types";
import { studyTaskKindSchema } from "../../study/types";

type StudyTaskKind = z.infer<typeof studyTaskKindSchema>;

// Task form schema for single task (study_task_seq는 submit 시 자동 계산)
const taskFormSchema = z.object({
  study_task_kind: studyTaskKindSchema,
  question: z.string().optional(),
  answer: z.string().optional(),
  image_url: z.string().optional(),
  audio_url: z.string().optional(),
  choice_1: z.string().optional(),
  choice_2: z.string().optional(),
  choice_3: z.string().optional(),
  choice_4: z.string().optional(),
  choice_correct: z.number().int().min(1).max(4).optional(),
});

type TaskFormData = z.infer<typeof taskFormSchema>;

// Bulk task form schema (study_task_seq는 submit 시 자동 계산)
const bulkTaskFormSchema = z.object({
  items: z.array(
    z.object({
      study_task_kind: studyTaskKindSchema,
      question: z.string().optional(),
      answer: z.string().optional(),
      image_url: z.string().optional(),
      audio_url: z.string().optional(),
      choice_1: z.string().optional(),
      choice_2: z.string().optional(),
      choice_3: z.string().optional(),
      choice_4: z.string().optional(),
      choice_correct: z.number().int().min(1).max(4).optional(),
    })
  ).min(1),
});

type BulkTaskFormData = z.infer<typeof bulkTaskFormSchema>;

// CSV parsed task interface
interface ParsedCSVTask {
  rowNumber: number;
  study_task_kind: StudyTaskKind;
  question?: string;
  answer?: string;
  image_url?: string;
  audio_url?: string;
  choice_1?: string;
  choice_2?: string;
  choice_3?: string;
  choice_4?: string;
  choice_correct?: number;
  error?: string;
}

export function AdminStudyCreate() {
  const navigate = useNavigate();
  const createStudyMutation = useCreateAdminStudy();
  const createTaskMutation = useCreateAdminStudyTask();
  const createTasksBulkMutation = useCreateAdminStudyTasksBulk();

  // Created study state
  const [createdStudy, setCreatedStudy] = useState<AdminStudyRes | null>(null);
  const [taskCount, setTaskCount] = useState(0);

  // CSV import state
  const [parsedCSVTasks, setParsedCSVTasks] = useState<ParsedCSVTask[]>([]);

  // Study form
  const studyForm = useForm<StudyCreateReq>({
    resolver: zodResolver(studyCreateReqSchema),
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

  // Single task form
  const taskForm = useForm<TaskFormData>({
    resolver: zodResolver(taskFormSchema),
    defaultValues: {
      study_task_kind: "choice",
      question: "",
      answer: "",
      image_url: "",
      audio_url: "",
      choice_1: "",
      choice_2: "",
      choice_3: "",
      choice_4: "",
      choice_correct: 1,
    },
  });

  // Bulk task form
  const bulkTaskForm = useForm<BulkTaskFormData>({
    resolver: zodResolver(bulkTaskFormSchema),
    defaultValues: {
      items: [
        {
          study_task_kind: "choice",
          question: "",
          answer: "",
          image_url: "",
          audio_url: "",
          choice_1: "",
          choice_2: "",
          choice_3: "",
          choice_4: "",
          choice_correct: 1,
        },
      ],
    },
  });

  const { fields, append, remove } = useFieldArray({
    control: bulkTaskForm.control,
    name: "items",
  });

  const onStudySubmit = async (data: StudyCreateReq) => {
    try {
      const result = await createStudyMutation.mutateAsync(data);
      toast.success("Study created successfully! Now you can add tasks.");
      setCreatedStudy(result);
    } catch {
      toast.error("Failed to create study");
    }
  };

  const onTaskSubmit = async (data: TaskFormData) => {
    if (!createdStudy) return;

    try {
      // 자동으로 seq 계산 (taskCount + 1)
      const nextSeq = taskCount + 1;
      await createTaskMutation.mutateAsync({
        study_id: createdStudy.study_id,
        ...data,
        study_task_seq: nextSeq,
      });
      toast.success(`Task #${nextSeq} added successfully`);
      setTaskCount((prev) => prev + 1);
      // Reset form
      taskForm.reset({
        study_task_kind: "choice",
        question: "",
        answer: "",
        image_url: "",
        audio_url: "",
        choice_1: "",
        choice_2: "",
        choice_3: "",
        choice_4: "",
        choice_correct: 1,
      });
    } catch {
      toast.error("Failed to add task");
    }
  };

  const onBulkTaskSubmit = async (data: BulkTaskFormData) => {
    if (!createdStudy) return;

    try {
      // 자동으로 seq 계산 (taskCount + 1부터 순차적으로)
      const items = data.items.map((item, index) => ({
        study_id: createdStudy.study_id,
        ...item,
        study_task_seq: taskCount + index + 1,
      }));

      const result = await createTasksBulkMutation.mutateAsync({ items });
      toast.success(`${result.success_count} tasks added (seq ${taskCount + 1} ~ ${taskCount + result.success_count})`);
      setTaskCount((prev) => prev + result.success_count);
      // Reset form
      bulkTaskForm.reset({
        items: [
          {
            study_task_kind: "choice",
            question: "",
            answer: "",
            image_url: "",
            audio_url: "",
            choice_1: "",
            choice_2: "",
            choice_3: "",
            choice_4: "",
            choice_correct: 1,
          },
        ],
      });
    } catch {
      toast.error("Failed to add tasks");
    }
  };

  const addBulkTaskRow = () => {
    append({
      study_task_kind: "choice",
      question: "",
      answer: "",
      image_url: "",
      audio_url: "",
      choice_1: "",
      choice_2: "",
      choice_3: "",
      choice_4: "",
      choice_correct: 1,
    });
  };

  // CSV parsing function for tasks
  const parseCSV = (text: string): ParsedCSVTask[] => {
    const lines = text.trim().split("\n");
    if (lines.length < 2) return [];

    const headers = lines[0].toLowerCase().split(",").map((h) => h.trim());

    const kindIdx = headers.indexOf("study_task_kind");
    const questionIdx = headers.indexOf("question");
    const answerIdx = headers.indexOf("answer");
    const imageUrlIdx = headers.indexOf("image_url");
    const audioUrlIdx = headers.indexOf("audio_url");
    const choice1Idx = headers.indexOf("choice_1");
    const choice2Idx = headers.indexOf("choice_2");
    const choice3Idx = headers.indexOf("choice_3");
    const choice4Idx = headers.indexOf("choice_4");
    const choiceCorrectIdx = headers.indexOf("choice_correct");

    if (kindIdx === -1) {
      toast.error("CSV must have 'study_task_kind' column");
      return [];
    }

    const validKinds = ["choice", "typing", "voice"];

    const tasks: ParsedCSVTask[] = [];
    for (let i = 1; i < lines.length; i++) {
      const values = lines[i].split(",").map((v) => v.trim());
      if (values.length < 1 || !values[0]) continue;

      const kindValue = values[kindIdx]?.toLowerCase() || "";
      const choiceCorrectValue = choiceCorrectIdx !== -1 ? parseInt(values[choiceCorrectIdx], 10) : undefined;

      const task: ParsedCSVTask = {
        rowNumber: i + 1,
        study_task_kind: validKinds.includes(kindValue)
          ? (kindValue as StudyTaskKind)
          : "choice",
        question: questionIdx !== -1 ? values[questionIdx] || undefined : undefined,
        answer: answerIdx !== -1 ? values[answerIdx] || undefined : undefined,
        image_url: imageUrlIdx !== -1 ? values[imageUrlIdx] || undefined : undefined,
        audio_url: audioUrlIdx !== -1 ? values[audioUrlIdx] || undefined : undefined,
        choice_1: choice1Idx !== -1 ? values[choice1Idx] || undefined : undefined,
        choice_2: choice2Idx !== -1 ? values[choice2Idx] || undefined : undefined,
        choice_3: choice3Idx !== -1 ? values[choice3Idx] || undefined : undefined,
        choice_4: choice4Idx !== -1 ? values[choice4Idx] || undefined : undefined,
        choice_correct: !isNaN(choiceCorrectValue!) && choiceCorrectValue! >= 1 && choiceCorrectValue! <= 4
          ? choiceCorrectValue
          : undefined,
      };

      // Validation
      if (!validKinds.includes(kindValue)) {
        task.error = `Invalid kind: ${kindValue}`;
      } else if (task.study_task_kind === "choice" && !task.choice_1 && !task.choice_2) {
        task.error = "Choice type requires at least 2 choices";
      }

      tasks.push(task);
    }

    return tasks;
  };

  const handleCSVFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const text = event.target?.result as string;
      const tasks = parseCSV(text);
      setParsedCSVTasks(tasks);
    };
    reader.readAsText(file);
  };

  const onCSVTasksSubmit = async () => {
    if (!createdStudy) return;

    const validTasks = parsedCSVTasks.filter((t) => !t.error);
    if (validTasks.length === 0) {
      toast.error("No valid tasks to create");
      return;
    }

    try {
      const items = validTasks.map((task, index) => ({
        study_id: createdStudy.study_id,
        study_task_seq: taskCount + index + 1,
        study_task_kind: task.study_task_kind,
        question: task.question,
        answer: task.answer,
        image_url: task.image_url,
        audio_url: task.audio_url,
        choice_1: task.choice_1,
        choice_2: task.choice_2,
        choice_3: task.choice_3,
        choice_4: task.choice_4,
        choice_correct: task.choice_correct,
      }));

      const result = await createTasksBulkMutation.mutateAsync({ items });
      toast.success(`${result.success_count} tasks added (seq ${taskCount + 1} ~ ${taskCount + result.success_count})`);
      setTaskCount((prev) => prev + result.success_count);
      setParsedCSVTasks([]);
    } catch {
      toast.error("Failed to add tasks from CSV");
    }
  };

  const validCSVCount = parsedCSVTasks.filter((t) => !t.error).length;
  const invalidCSVCount = parsedCSVTasks.filter((t) => t.error).length;

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

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/studies")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Create New Study</h1>
        {createdStudy && (
          <Badge variant="outline" className="ml-2">
            <CheckCircle className="mr-1 h-3 w-3 text-green-500" />
            Study #{createdStudy.study_id} Created
          </Badge>
        )}
      </div>

      {/* Study Creation Form */}
      <form onSubmit={studyForm.handleSubmit(onStudySubmit, (errors) => {
        const errorFields = Object.keys(errors).join(", ");
        toast.error(`Please fix errors: ${errorFields}`);
      })}>
        <Card className={createdStudy ? "border-green-500/50" : ""}>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              {createdStudy && <CheckCircle className="h-5 w-5 text-green-500" />}
              Study Information
            </CardTitle>
            <CardDescription>
              {createdStudy
                ? "Study has been created. You can now add tasks below."
                : "Enter the details for the new study"}
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
                  disabled={!!createdStudy}
                  {...studyForm.register("study_idx")}
                />
                {studyForm.formState.errors.study_idx && (
                  <p className="text-sm text-destructive">
                    {studyForm.formState.errors.study_idx.message}
                  </p>
                )}
              </div>

              {/* Program */}
              <div className="space-y-2">
                <Label>Program</Label>
                <Select
                  value={studyForm.watch("study_program") ?? "tbc"}
                  onValueChange={(value) =>
                    studyForm.setValue("study_program", value as StudyProgram)
                  }
                  disabled={!!createdStudy}
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
                  disabled={!!createdStudy}
                  {...studyForm.register("study_title")}
                />
              </div>

              {/* Subtitle */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="study_subtitle">Subtitle</Label>
                <Input
                  id="study_subtitle"
                  placeholder="Study subtitle"
                  maxLength={120}
                  disabled={!!createdStudy}
                  {...studyForm.register("study_subtitle")}
                />
              </div>

              {/* Description */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="study_description">Description</Label>
                <Textarea
                  id="study_description"
                  placeholder="Study description"
                  rows={3}
                  disabled={!!createdStudy}
                  {...studyForm.register("study_description")}
                />
              </div>

              {/* State */}
              <div className="space-y-2">
                <Label>State</Label>
                <Select
                  value={studyForm.watch("study_state") ?? "ready"}
                  onValueChange={(value) =>
                    studyForm.setValue("study_state", value as StudyState)
                  }
                  disabled={!!createdStudy}
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
                  value={studyForm.watch("study_access") ?? "public"}
                  onValueChange={(value) =>
                    studyForm.setValue("study_access", value as StudyAccess)
                  }
                  disabled={!!createdStudy}
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
            {!createdStudy && (
              <div className="flex justify-end pt-4">
                <Button type="submit" disabled={createStudyMutation.isPending}>
                  <Plus className="mr-2 h-4 w-4" />
                  {createStudyMutation.isPending ? "Creating..." : "Create Study"}
                </Button>
              </div>
            )}
          </CardContent>
        </Card>
      </form>

      {/* Task Creation Section - Only shown after study is created */}
      {createdStudy && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <ListPlus className="h-5 w-5" />
              Add Tasks
              {taskCount > 0 && (
                <Badge variant="secondary" className="ml-2">
                  {taskCount} task{taskCount > 1 ? "s" : ""} added
                </Badge>
              )}
            </CardTitle>
            <CardDescription>
              Add tasks to Study #{createdStudy.study_id} ({createdStudy.study_idx})
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Tabs defaultValue="single">
              <TabsList className="mb-4">
                <TabsTrigger value="single">Single Task</TabsTrigger>
                <TabsTrigger value="bulk">Bulk Tasks</TabsTrigger>
                <TabsTrigger value="csv">CSV Import</TabsTrigger>
              </TabsList>

              {/* Single Task Tab */}
              <TabsContent value="single">
                <form onSubmit={taskForm.handleSubmit(onTaskSubmit, (errors) => {
                  const errorFields = Object.keys(errors).join(", ");
                  toast.error(`Please fix errors: ${errorFields}`);
                })}>
                  <div className="space-y-4">
                    <div className="grid gap-4 md:grid-cols-3">
                      {/* Task Kind */}
                      <div className="space-y-2">
                        <Label>Task Kind *</Label>
                        <Select
                          value={taskForm.watch("study_task_kind")}
                          onValueChange={(value) =>
                            taskForm.setValue("study_task_kind", value as StudyTaskKind)
                          }
                        >
                          <SelectTrigger>
                            <SelectValue />
                          </SelectTrigger>
                          <SelectContent>
                            <SelectItem value="choice">Choice</SelectItem>
                            <SelectItem value="typing">Typing</SelectItem>
                            <SelectItem value="voice">Voice</SelectItem>
                          </SelectContent>
                        </Select>
                      </div>

                      {/* Sequence (자동 계산) */}
                      <div className="space-y-2">
                        <Label htmlFor="study_task_seq">Sequence</Label>
                        <Input
                          id="study_task_seq"
                          type="number"
                          value={taskCount + 1}
                          disabled
                          className="bg-muted"
                        />
                      </div>

                      {/* Answer */}
                      <div className="space-y-2">
                        <Label htmlFor="answer">Answer</Label>
                        <Input
                          id="answer"
                          placeholder="Correct answer"
                          {...taskForm.register("answer")}
                        />
                      </div>
                    </div>

                    {/* Question */}
                    <div className="space-y-2">
                      <Label htmlFor="question">Question</Label>
                      <Textarea
                        id="question"
                        placeholder="Enter the question"
                        rows={2}
                        {...taskForm.register("question")}
                      />
                    </div>

                    {/* Choice fields - only shown for choice type */}
                    {taskForm.watch("study_task_kind") === "choice" && (
                      <div className="space-y-4 rounded-lg border p-4">
                        <Label className="text-sm font-medium">Choices</Label>
                        <div className="grid gap-4 md:grid-cols-2">
                          <div className="space-y-2">
                            <Label htmlFor="choice_1" className="text-xs">Choice 1</Label>
                            <Input
                              id="choice_1"
                              placeholder="Choice 1"
                              {...taskForm.register("choice_1")}
                            />
                          </div>
                          <div className="space-y-2">
                            <Label htmlFor="choice_2" className="text-xs">Choice 2</Label>
                            <Input
                              id="choice_2"
                              placeholder="Choice 2"
                              {...taskForm.register("choice_2")}
                            />
                          </div>
                          <div className="space-y-2">
                            <Label htmlFor="choice_3" className="text-xs">Choice 3</Label>
                            <Input
                              id="choice_3"
                              placeholder="Choice 3"
                              {...taskForm.register("choice_3")}
                            />
                          </div>
                          <div className="space-y-2">
                            <Label htmlFor="choice_4" className="text-xs">Choice 4</Label>
                            <Input
                              id="choice_4"
                              placeholder="Choice 4"
                              {...taskForm.register("choice_4")}
                            />
                          </div>
                        </div>
                        <div className="space-y-2">
                          <Label htmlFor="choice_correct" className="text-xs">Correct Choice (1-4)</Label>
                          <Input
                            id="choice_correct"
                            type="number"
                            min={1}
                            max={4}
                            className="w-24"
                            {...taskForm.register("choice_correct", { valueAsNumber: true })}
                          />
                        </div>
                      </div>
                    )}

                    {/* Media URLs */}
                    <div className="grid gap-4 md:grid-cols-2">
                      <div className="space-y-2">
                        <Label htmlFor="image_url">Image URL</Label>
                        <Input
                          id="image_url"
                          placeholder="https://..."
                          {...taskForm.register("image_url")}
                        />
                      </div>
                      <div className="space-y-2">
                        <Label htmlFor="audio_url">Audio URL</Label>
                        <Input
                          id="audio_url"
                          placeholder="https://..."
                          {...taskForm.register("audio_url")}
                        />
                      </div>
                    </div>

                    <div className="flex justify-end">
                      <Button type="submit" disabled={createTaskMutation.isPending}>
                        <Plus className="mr-2 h-4 w-4" />
                        {createTaskMutation.isPending ? "Adding..." : "Add Task"}
                      </Button>
                    </div>
                  </div>
                </form>
              </TabsContent>

              {/* Bulk Tasks Tab */}
              <TabsContent value="bulk">
                <form onSubmit={bulkTaskForm.handleSubmit(onBulkTaskSubmit, (errors) => {
                  console.error(errors);
                  toast.error("Please fix errors in the form");
                })}>
                  <div className="space-y-4">
                    {fields.map((field, index) => (
                      <div key={field.id} className="rounded-lg border p-4">
                        <div className="mb-3 flex items-center justify-between">
                          <Badge variant={getTaskKindBadgeVariant(bulkTaskForm.watch(`items.${index}.study_task_kind`))}>
                            Task #{index + 1}
                          </Badge>
                          {fields.length > 1 && (
                            <Button
                              type="button"
                              variant="ghost"
                              size="sm"
                              onClick={() => remove(index)}
                            >
                              <Trash2 className="h-4 w-4 text-destructive" />
                            </Button>
                          )}
                        </div>

                        <div className="grid gap-4 md:grid-cols-5">
                          {/* Task Kind */}
                          <div className="space-y-2">
                            <Label className="text-xs">Kind</Label>
                            <Select
                              value={bulkTaskForm.watch(`items.${index}.study_task_kind`)}
                              onValueChange={(value) =>
                                bulkTaskForm.setValue(`items.${index}.study_task_kind`, value as StudyTaskKind)
                              }
                            >
                              <SelectTrigger>
                                <SelectValue />
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value="choice">Choice</SelectItem>
                                <SelectItem value="typing">Typing</SelectItem>
                                <SelectItem value="voice">Voice</SelectItem>
                              </SelectContent>
                            </Select>
                          </div>

                          {/* Sequence (자동 계산) */}
                          <div className="space-y-2">
                            <Label className="text-xs">Seq</Label>
                            <Input
                              type="number"
                              value={taskCount + index + 1}
                              disabled
                              className="bg-muted"
                            />
                          </div>

                          {/* Question */}
                          <div className="space-y-2 md:col-span-2">
                            <Label className="text-xs">Question</Label>
                            <Input
                              placeholder="Question"
                              {...bulkTaskForm.register(`items.${index}.question`)}
                            />
                          </div>

                          {/* Answer */}
                          <div className="space-y-2">
                            <Label className="text-xs">Answer</Label>
                            <Input
                              placeholder="Answer"
                              {...bulkTaskForm.register(`items.${index}.answer`)}
                            />
                          </div>
                        </div>

                        {/* Media URLs */}
                        <div className="mt-3 grid gap-2 md:grid-cols-2">
                          <Input
                            placeholder="Image URL"
                            {...bulkTaskForm.register(`items.${index}.image_url`)}
                          />
                          <Input
                            placeholder="Audio URL"
                            {...bulkTaskForm.register(`items.${index}.audio_url`)}
                          />
                        </div>

                        {/* Choice fields for choice type */}
                        {bulkTaskForm.watch(`items.${index}.study_task_kind`) === "choice" && (
                          <div className="mt-3 grid gap-2 md:grid-cols-5">
                            <Input
                              placeholder="Choice 1"
                              {...bulkTaskForm.register(`items.${index}.choice_1`)}
                            />
                            <Input
                              placeholder="Choice 2"
                              {...bulkTaskForm.register(`items.${index}.choice_2`)}
                            />
                            <Input
                              placeholder="Choice 3"
                              {...bulkTaskForm.register(`items.${index}.choice_3`)}
                            />
                            <Input
                              placeholder="Choice 4"
                              {...bulkTaskForm.register(`items.${index}.choice_4`)}
                            />
                            <Input
                              type="number"
                              min={1}
                              max={4}
                              placeholder="Correct (1-4)"
                              {...bulkTaskForm.register(`items.${index}.choice_correct`, { valueAsNumber: true })}
                            />
                          </div>
                        )}
                      </div>
                    ))}

                    <div className="flex justify-between">
                      <Button type="button" variant="outline" onClick={addBulkTaskRow}>
                        <Plus className="mr-2 h-4 w-4" />
                        Add Row
                      </Button>
                      <Button type="submit" disabled={createTasksBulkMutation.isPending}>
                        <ListPlus className="mr-2 h-4 w-4" />
                        {createTasksBulkMutation.isPending
                          ? "Adding..."
                          : `Add ${fields.length} Task${fields.length > 1 ? "s" : ""}`}
                      </Button>
                    </div>
                  </div>
                </form>
              </TabsContent>

              {/* CSV Import Tab */}
              <TabsContent value="csv">
                <div className="space-y-4">
                  {/* CSV Format Guide */}
                  <div className="rounded-lg border p-4">
                    <div className="flex items-center gap-2 mb-2">
                      <FileText className="h-4 w-4" />
                      <span className="font-medium text-sm">CSV Format</span>
                    </div>
                    <pre className="bg-muted p-3 rounded-md text-xs overflow-x-auto">
{`study_task_kind,question,answer,image_url,audio_url,choice_1,choice_2,choice_3,choice_4,choice_correct
choice,What is 1+1?,2,,https://audio.com/q1.mp3,1,2,3,4,2
typing,Type the answer,Hello,,,,,,,
voice,Read this sentence,안녕하세요,,,,,,,`}
                    </pre>
                    <div className="text-xs text-muted-foreground mt-2 space-y-1">
                      <p><strong>study_task_kind</strong> (required): choice, typing, voice</p>
                      <p><strong>choice</strong> type: needs choice_1 ~ choice_4, choice_correct (1-4)</p>
                      <p>Sequence is auto-calculated starting from {taskCount + 1}</p>
                    </div>
                  </div>

                  {/* File Upload */}
                  <div className="space-y-2">
                    <Label htmlFor="csv-task-file" className="flex items-center gap-2">
                      <Upload className="h-4 w-4" />
                      Upload CSV File
                    </Label>
                    <Input
                      id="csv-task-file"
                      type="file"
                      accept=".csv"
                      onChange={handleCSVFileUpload}
                    />
                  </div>

                  {/* Preview Table */}
                  {parsedCSVTasks.length > 0 && (
                    <div className="space-y-3">
                      <div className="flex items-center gap-2">
                        <span className="font-medium">Preview ({parsedCSVTasks.length} rows)</span>
                        <span className="text-green-600 text-sm">{validCSVCount} valid</span>
                        {invalidCSVCount > 0 && (
                          <span className="text-destructive text-sm">{invalidCSVCount} invalid</span>
                        )}
                      </div>

                      <div className="rounded-md border max-h-80 overflow-y-auto">
                        <table className="w-full text-sm">
                          <thead className="border-b bg-muted/50 sticky top-0">
                            <tr>
                              <th className="h-9 px-3 text-left font-medium">Seq</th>
                              <th className="h-9 px-3 text-left font-medium">Kind</th>
                              <th className="h-9 px-3 text-left font-medium">Question</th>
                              <th className="h-9 px-3 text-left font-medium">Answer</th>
                              <th className="h-9 px-3 text-left font-medium">Status</th>
                            </tr>
                          </thead>
                          <tbody>
                            {parsedCSVTasks.map((task, index) => (
                              <tr key={task.rowNumber} className="border-b">
                                <td className="p-3 font-mono text-xs">{taskCount + index + 1}</td>
                                <td className="p-3">
                                  <Badge variant={getTaskKindBadgeVariant(task.study_task_kind)}>
                                    {task.study_task_kind}
                                  </Badge>
                                </td>
                                <td className="p-3">
                                  <div className="max-w-xs truncate" title={task.question || ""}>
                                    {task.question || "-"}
                                  </div>
                                </td>
                                <td className="p-3">
                                  <div className="max-w-xs truncate" title={task.answer || ""}>
                                    {task.answer || "-"}
                                  </div>
                                </td>
                                <td className="p-3">
                                  {task.error ? (
                                    <Badge variant="destructive" className="flex items-center gap-1 w-fit">
                                      <AlertCircle className="h-3 w-3" />
                                      {task.error}
                                    </Badge>
                                  ) : (
                                    <Badge variant="outline" className="flex items-center gap-1 w-fit text-green-600">
                                      <CheckCircle className="h-3 w-3" />
                                      Valid
                                    </Badge>
                                  )}
                                </td>
                              </tr>
                            ))}
                          </tbody>
                        </table>
                      </div>

                      <div className="flex justify-end gap-2">
                        <Button
                          variant="outline"
                          onClick={() => setParsedCSVTasks([])}
                        >
                          Clear
                        </Button>
                        <Button
                          onClick={onCSVTasksSubmit}
                          disabled={validCSVCount === 0 || createTasksBulkMutation.isPending}
                        >
                          <Upload className="mr-2 h-4 w-4" />
                          {createTasksBulkMutation.isPending
                            ? "Adding..."
                            : `Add ${validCSVCount} Task${validCSVCount > 1 ? "s" : ""}`}
                        </Button>
                      </div>
                    </div>
                  )}
                </div>
              </TabsContent>
            </Tabs>
          </CardContent>
        </Card>
      )}

      {/* Navigation after tasks are added */}
      {createdStudy && taskCount > 0 && (
        <div className="flex justify-end gap-4">
          <Button variant="outline" onClick={() => navigate("/admin/studies")}>
            Go to Studies List
          </Button>
          <Button onClick={() => navigate(`/admin/studies/${createdStudy.study_id}`)}>
            View Study Detail
          </Button>
        </div>
      )}
    </div>
  );
}
