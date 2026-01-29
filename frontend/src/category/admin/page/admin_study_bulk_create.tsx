import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { ArrowLeft, Upload, FileText, AlertCircle, CheckCircle } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

import { useCreateAdminStudiesBulk } from "../hook/use_admin_studies";
import type { StudyCreateReq, StudyBulkCreateRes } from "../types";

interface ParsedStudy extends StudyCreateReq {
  rowNumber: number;
  error?: string;
}

export function AdminStudyBulkCreate() {
  const navigate = useNavigate();
  const bulkCreateMutation = useCreateAdminStudiesBulk();

  const [parsedStudies, setParsedStudies] = useState<ParsedStudy[]>([]);
  const [result, setResult] = useState<StudyBulkCreateRes | null>(null);

  const parseCSV = (text: string): ParsedStudy[] => {
    const lines = text.trim().split("\n");
    if (lines.length < 2) return [];

    const headers = lines[0].toLowerCase().split(",").map((h) => h.trim());

    const idxIdx = headers.indexOf("study_idx");
    const programIdx = headers.indexOf("study_program");
    const stateIdx = headers.indexOf("study_state");
    const accessIdx = headers.indexOf("study_access");
    const titleIdx = headers.indexOf("study_title");
    const subtitleIdx = headers.indexOf("study_subtitle");
    const descriptionIdx = headers.indexOf("study_description");

    if (idxIdx === -1) {
      toast.error("CSV must have 'study_idx' column");
      return [];
    }

    const validPrograms = ["basic_pronunciation", "basic_word", "basic_900", "topik_read", "topik_listen", "topik_write", "tbc"];
    const validStates = ["ready", "open", "close"];
    const validAccess = ["public", "paid", "private", "promote"];

    const studies: ParsedStudy[] = [];
    for (let i = 1; i < lines.length; i++) {
      const values = lines[i].split(",").map((v) => v.trim());
      if (values.length < 1) continue;

      const programValue = programIdx !== -1 ? values[programIdx]?.toLowerCase() : "tbc";
      const stateValue = stateIdx !== -1 ? values[stateIdx]?.toLowerCase() : "ready";
      const accessValue = accessIdx !== -1 ? values[accessIdx]?.toLowerCase() : "public";

      const study: ParsedStudy = {
        rowNumber: i + 1,
        study_idx: values[idxIdx] || "",
        study_program: validPrograms.includes(programValue)
          ? (programValue as ParsedStudy["study_program"])
          : "tbc",
        study_state: validStates.includes(stateValue)
          ? (stateValue as ParsedStudy["study_state"])
          : "ready",
        study_access: validAccess.includes(accessValue)
          ? (accessValue as ParsedStudy["study_access"])
          : "public",
        study_title: titleIdx !== -1 ? values[titleIdx] || undefined : undefined,
        study_subtitle: subtitleIdx !== -1 ? values[subtitleIdx] || undefined : undefined,
        study_description: descriptionIdx !== -1 ? values[descriptionIdx] || undefined : undefined,
      };

      // Validation
      if (!study.study_idx || study.study_idx.length < 2) {
        study.error = "study_idx is required (min 2 chars)";
      }

      studies.push(study);
    }

    return studies;
  };

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const text = event.target?.result as string;
      const studies = parseCSV(text);
      setParsedStudies(studies);
      setResult(null);
    };
    reader.readAsText(file);
  };

  const handleSubmit = async () => {
    const validStudies = parsedStudies.filter((s) => !s.error);
    if (validStudies.length === 0) {
      toast.error("No valid studies to create");
      return;
    }

    try {
      const items = validStudies.map(({
        study_idx,
        study_program,
        study_state,
        study_access,
        study_title,
        study_subtitle,
        study_description,
      }) => ({
        study_idx,
        study_program,
        study_state,
        study_access,
        study_title: study_title || undefined,
        study_subtitle: study_subtitle || undefined,
        study_description: study_description || undefined,
      }));

      const res = await bulkCreateMutation.mutateAsync({ items });
      setResult(res);
      toast.success(`Created ${res.success_count} studies`);
    } catch {
      toast.error("Bulk creation failed");
    }
  };

  const validCount = parsedStudies.filter((s) => !s.error).length;
  const invalidCount = parsedStudies.filter((s) => s.error).length;

  const getProgramBadgeVariant = (program: string) => {
    switch (program) {
      case "basic_pronunciation":
      case "basic_word":
      case "basic_900":
        return "secondary" as const;
      case "topik_read":
      case "topik_listen":
      case "topik_write":
        return "default" as const;
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
        <h1 className="text-2xl font-bold">Bulk Create Studies</h1>
      </div>

      {/* CSV Format Guide */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            CSV Format
          </CardTitle>
          <CardDescription>
            Upload a CSV file with study data. Required: study_idx
          </CardDescription>
        </CardHeader>
        <CardContent>
          <pre className="bg-muted p-4 rounded-md text-sm overflow-x-auto">
{`study_idx,study_program,study_state,study_access,study_title,study_subtitle,study_description
TBC-001,tbc,ready,public,Lesson 1 Introduction,Welcome to lesson 1,This is description
TBC-002,basic_word,open,paid,Lesson 2 Basics,Basic concepts,Another description`}
          </pre>
          <div className="text-sm text-muted-foreground mt-2 space-y-1">
            <p><strong>study_program</strong>: basic_pronunciation, basic_word, basic_900, topik_read, topik_listen, topik_write, tbc (default: tbc)</p>
            <p><strong>study_state</strong>: ready, open, close (default: ready)</p>
            <p><strong>study_access</strong>: public, paid, private, promote (default: public)</p>
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
      {parsedStudies.length > 0 && !result && (
        <Card>
          <CardHeader>
            <CardTitle>Preview ({parsedStudies.length} rows)</CardTitle>
            <CardDescription>
              <span className="text-green-600">{validCount} valid</span>
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
                    <th className="h-10 px-4 text-left font-medium">Program</th>
                    <th className="h-10 px-4 text-left font-medium">State</th>
                    <th className="h-10 px-4 text-left font-medium">Access</th>
                    <th className="h-10 px-4 text-left font-medium">Title</th>
                    <th className="h-10 px-4 text-left font-medium">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {parsedStudies.map((study) => (
                    <tr key={study.rowNumber} className="border-b">
                      <td className="p-4">{study.rowNumber}</td>
                      <td className="p-4 font-mono">{study.study_idx || "-"}</td>
                      <td className="p-4">
                        <Badge variant={getProgramBadgeVariant(study.study_program || "tbc")}>
                          {study.study_program || "tbc"}
                        </Badge>
                      </td>
                      <td className="p-4">{study.study_state || "ready"}</td>
                      <td className="p-4">{study.study_access || "public"}</td>
                      <td className="p-4">
                        <div className="max-w-xs truncate" title={study.study_title || ""}>
                          {study.study_title || "-"}
                        </div>
                      </td>
                      <td className="p-4">
                        {study.error ? (
                          <Badge variant="destructive" className="flex items-center gap-1 w-fit">
                            <AlertCircle className="h-3 w-3" />
                            {study.error}
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

            <div className="flex justify-end gap-2 pt-4">
              <Button
                variant="outline"
                onClick={() => setParsedStudies([])}
              >
                Clear
              </Button>
              <Button
                onClick={handleSubmit}
                disabled={validCount === 0 || bulkCreateMutation.isPending}
              >
                {bulkCreateMutation.isPending
                  ? "Creating..."
                  : `Create ${validCount} Studies`}
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
                      <td className="p-4">{item.id ?? "-"}</td>
                      <td className="p-4 font-mono">{item.idx}</td>
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
              <Button onClick={() => navigate("/admin/studies")}>
                Back to Studies
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
