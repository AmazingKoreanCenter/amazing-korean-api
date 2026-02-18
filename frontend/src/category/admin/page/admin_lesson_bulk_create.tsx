import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { ArrowLeft, Upload, FileText, AlertCircle, CheckCircle } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

import { useCreateAdminLessonsBulk } from "../hook/use_admin_lessons";
import type {
  LessonCreateItem,
  LessonBulkCreateRes,
  LessonState,
  LessonAccess,
} from "../lesson/types";

interface ParsedLesson extends LessonCreateItem {
  rowNumber: number;
  error?: string;
}

export function AdminLessonBulkCreate() {
  const navigate = useNavigate();
  const bulkCreateMutation = useCreateAdminLessonsBulk();

  const [parsedLessons, setParsedLessons] = useState<ParsedLesson[]>([]);
  const [result, setResult] = useState<LessonBulkCreateRes | null>(null);

  const parseCSV = (text: string): ParsedLesson[] => {
    const lines = text.trim().split("\n");
    if (lines.length < 2) return [];

    const headers = lines[0].toLowerCase().split(",").map((h) => h.trim());

    const idxIdx = headers.indexOf("lesson_idx");
    const titleIdx = headers.indexOf("lesson_title");
    const subtitleIdx = headers.indexOf("lesson_subtitle");
    const descriptionIdx = headers.indexOf("lesson_description");
    const stateIdx = headers.indexOf("lesson_state");
    const accessIdx = headers.indexOf("lesson_access");

    if (idxIdx === -1) {
      toast.error("CSV must have 'lesson_idx' column");
      return [];
    }

    const validStates = ["ready", "open", "close"];
    const validAccess = ["public", "paid", "private", "promote"];

    const lessons: ParsedLesson[] = [];
    for (let i = 1; i < lines.length; i++) {
      const values = lines[i].split(",").map((v) => v.trim());
      if (values.length < 1) continue;

      const stateValue = stateIdx !== -1 ? values[stateIdx]?.toLowerCase() : "ready";
      const accessValue = accessIdx !== -1 ? values[accessIdx]?.toLowerCase() : "public";

      const lesson: ParsedLesson = {
        rowNumber: i + 1,
        lesson_idx: values[idxIdx] || "",
        lesson_title: titleIdx !== -1 ? values[titleIdx] || "" : "",
        lesson_subtitle: subtitleIdx !== -1 ? values[subtitleIdx] || undefined : undefined,
        lesson_description: descriptionIdx !== -1 ? values[descriptionIdx] || undefined : undefined,
        lesson_state: validStates.includes(stateValue)
          ? (stateValue as LessonState)
          : "ready",
        lesson_access: validAccess.includes(accessValue)
          ? (accessValue as LessonAccess)
          : "public",
      };

      // Validation
      if (!lesson.lesson_idx || lesson.lesson_idx.length < 1) {
        lesson.error = "lesson_idx is required";
      } else if (!lesson.lesson_title || lesson.lesson_title.length < 1) {
        lesson.error = "lesson_title is required";
      }

      lessons.push(lesson);
    }

    return lessons;
  };

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const text = event.target?.result as string;
      const lessons = parseCSV(text);
      setParsedLessons(lessons);
      setResult(null);
    };
    reader.readAsText(file);
  };

  const handleSubmit = async () => {
    const validLessons = parsedLessons.filter((l) => !l.error);
    if (validLessons.length === 0) {
      toast.error("No valid lessons to create");
      return;
    }

    try {
      const items = validLessons.map(({
        lesson_idx,
        lesson_title,
        lesson_subtitle,
        lesson_description,
        lesson_state,
        lesson_access,
      }) => ({
        lesson_idx,
        lesson_title,
        lesson_subtitle: lesson_subtitle || undefined,
        lesson_description: lesson_description || undefined,
        lesson_state,
        lesson_access,
      }));

      const res = await bulkCreateMutation.mutateAsync({ items });
      setResult(res);
      toast.success(`Created ${res.success_count} lessons`);
    } catch {
      toast.error("Bulk creation failed");
    }
  };

  const validCount = parsedLessons.filter((l) => !l.error).length;
  const invalidCount = parsedLessons.filter((l) => l.error).length;

  const getStateBadgeVariant = (state: string) => {
    switch (state) {
      case "ready":
        return "secondary" as const;
      case "open":
        return "default" as const;
      case "close":
        return "outline" as const;
      default:
        return "outline" as const;
    }
  };

  const getAccessBadgeVariant = (access: string) => {
    switch (access) {
      case "public":
        return "default" as const;
      case "paid":
        return "secondary" as const;
      case "private":
        return "destructive" as const;
      case "promote":
        return "outline" as const;
      default:
        return "outline" as const;
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/lessons")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Bulk Create Lessons</h1>
      </div>

      {/* CSV Format Guide */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            CSV Format
          </CardTitle>
          <CardDescription>
            Upload a CSV file with lesson data. Required: lesson_idx, lesson_title
          </CardDescription>
        </CardHeader>
        <CardContent>
          <pre className="bg-muted p-4 rounded-md text-sm overflow-x-auto">
{`lesson_idx,lesson_title,lesson_subtitle,lesson_description,lesson_state,lesson_access
LESSON-001,Introduction to Korean,Welcome to the course,This lesson covers basics,ready,public
LESSON-002,Korean Alphabet,Learning Hangul,The Korean writing system,open,paid`}
          </pre>
          <div className="text-sm text-muted-foreground mt-2 space-y-1">
            <p><strong>lesson_idx</strong> (required): Unique identifier</p>
            <p><strong>lesson_title</strong> (required): Lesson title</p>
            <p><strong>lesson_state</strong>: ready, open, close (default: ready)</p>
            <p><strong>lesson_access</strong>: public, paid, private, promote (default: public)</p>
          </div>
        </CardContent>
      </Card>

      {/* File Upload */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Upload className="h-5 w-5" />
            Upload CSV
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="csv-file">Select CSV File</Label>
              <Input
                id="csv-file"
                type="file"
                accept=".csv"
                onChange={handleFileUpload}
              />
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Preview */}
      {parsedLessons.length > 0 && !result && (
        <Card>
          <CardHeader>
            <CardTitle>Preview ({parsedLessons.length} rows)</CardTitle>
            <CardDescription>
              <span className="text-status-success">{validCount} valid</span>
              {invalidCount > 0 && (
                <span className="text-destructive ml-2">{invalidCount} invalid</span>
              )}
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border max-h-96 overflow-y-auto">
              <table className="w-full text-sm">
                <thead className="border-b bg-muted/50 sticky top-0">
                  <tr>
                    <th className="h-10 px-4 text-left font-medium">Row</th>
                    <th className="h-10 px-4 text-left font-medium">IDX</th>
                    <th className="h-10 px-4 text-left font-medium">Title</th>
                    <th className="h-10 px-4 text-left font-medium">State</th>
                    <th className="h-10 px-4 text-left font-medium">Access</th>
                    <th className="h-10 px-4 text-left font-medium">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {parsedLessons.map((lesson) => (
                    <tr key={lesson.rowNumber} className="border-b">
                      <td className="p-4">{lesson.rowNumber}</td>
                      <td className="p-4">
                        <code className="text-xs bg-muted px-1 py-0.5 rounded">
                          {lesson.lesson_idx || "-"}
                        </code>
                      </td>
                      <td className="p-4">
                        <div className="max-w-xs truncate" title={lesson.lesson_title || ""}>
                          {lesson.lesson_title || "-"}
                        </div>
                      </td>
                      <td className="p-4">
                        <Badge variant={getStateBadgeVariant(lesson.lesson_state || "ready")}>
                          {lesson.lesson_state || "ready"}
                        </Badge>
                      </td>
                      <td className="p-4">
                        <Badge variant={getAccessBadgeVariant(lesson.lesson_access || "public")}>
                          {lesson.lesson_access || "public"}
                        </Badge>
                      </td>
                      <td className="p-4">
                        {lesson.error ? (
                          <Badge variant="destructive" className="flex items-center gap-1 w-fit">
                            <AlertCircle className="h-3 w-3" />
                            {lesson.error}
                          </Badge>
                        ) : (
                          <Badge variant="outline" className="flex items-center gap-1 w-fit text-status-success">
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

            <div className="flex justify-end gap-2 pt-4">
              <Button
                variant="outline"
                onClick={() => setParsedLessons([])}
              >
                Clear
              </Button>
              <Button
                onClick={handleSubmit}
                disabled={validCount === 0 || bulkCreateMutation.isPending}
              >
                {bulkCreateMutation.isPending
                  ? "Creating..."
                  : `Create ${validCount} Lessons`}
              </Button>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Result */}
      {result && (
        <Card>
          <CardHeader>
            <CardTitle>Result</CardTitle>
            <CardDescription>
              Success: {result.success_count}, Failed: {result.failure_count}
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border max-h-96 overflow-y-auto">
              <table className="w-full text-sm">
                <thead className="border-b bg-muted/50 sticky top-0">
                  <tr>
                    <th className="h-10 px-4 text-left font-medium">ID</th>
                    <th className="h-10 px-4 text-left font-medium">IDX</th>
                    <th className="h-10 px-4 text-left font-medium">Status</th>
                    <th className="h-10 px-4 text-left font-medium">Error</th>
                  </tr>
                </thead>
                <tbody>
                  {result.results.map((item, idx) => (
                    <tr key={idx} className="border-b">
                      <td className="p-4">{item.lesson_id ?? "-"}</td>
                      <td className="p-4">
                        <code className="text-xs bg-muted px-1 py-0.5 rounded">
                          {item.lesson_idx}
                        </code>
                      </td>
                      <td className="p-4">
                        <Badge variant={item.success ? "outline" : "destructive"}>
                          {item.success ? "Success" : "Failed"}
                        </Badge>
                      </td>
                      <td className="p-4 text-muted-foreground">
                        {item.error || "-"}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            <div className="flex justify-end gap-2 pt-4">
              <Button onClick={() => navigate("/admin/lessons")}>
                Back to Lessons
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
