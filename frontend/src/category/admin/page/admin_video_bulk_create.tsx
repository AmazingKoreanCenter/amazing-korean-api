import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { ArrowLeft, Upload, FileText, AlertCircle, CheckCircle } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

import { useCreateAdminVideosBulk } from "../hook/use_admin_videos";
import type { VideoCreateReq, VideoBulkCreateRes } from "../types";

interface ParsedVideo extends VideoCreateReq {
  rowNumber: number;
  error?: string;
}

export function AdminVideoBulkCreate() {
  const navigate = useNavigate();
  const bulkCreateMutation = useCreateAdminVideosBulk();

  const [parsedVideos, setParsedVideos] = useState<ParsedVideo[]>([]);
  const [result, setResult] = useState<VideoBulkCreateRes | null>(null);

  const parseCSV = (text: string): ParsedVideo[] => {
    const lines = text.trim().split("\n");
    if (lines.length < 2) return [];

    const headers = lines[0].toLowerCase().split(",").map((h) => h.trim());
    // 새로운 순서: video_idx, video_state, video_access, video_tag_title, video_tag_subtitle, video_tag_key, video_url_vimeo
    const idxIdx = headers.indexOf("video_idx");
    const stateIdx = headers.indexOf("video_state");
    const accessIdx = headers.indexOf("video_access");
    const titleIdx = headers.indexOf("video_tag_title");
    const subtitleIdx = headers.indexOf("video_tag_subtitle");
    const keyIdx = headers.indexOf("video_tag_key");
    const urlIdx = headers.indexOf("video_url_vimeo");

    if (titleIdx === -1 || urlIdx === -1) {
      toast.error("CSV must have 'video_tag_title' and 'video_url_vimeo' columns");
      return [];
    }

    const validStates = ["ready", "open", "close"];
    const validAccess = ["public", "paid", "private", "promote"];

    const videos: ParsedVideo[] = [];
    for (let i = 1; i < lines.length; i++) {
      const values = lines[i].split(",").map((v) => v.trim());
      if (values.length < 2) continue;

      const stateValue = stateIdx !== -1 ? values[stateIdx]?.toLowerCase() : undefined;
      const accessValue = accessIdx !== -1 ? values[accessIdx]?.toLowerCase() : "private";

      const video: ParsedVideo = {
        rowNumber: i + 1,
        video_idx: idxIdx !== -1 ? values[idxIdx] || undefined : undefined,
        video_state: stateValue && validStates.includes(stateValue)
          ? (stateValue as "ready" | "open" | "close")
          : undefined,
        video_access: validAccess.includes(accessValue)
          ? (accessValue as "public" | "paid" | "private" | "promote")
          : "private",
        video_tag_title: values[titleIdx] || "",
        video_tag_subtitle: subtitleIdx !== -1 ? values[subtitleIdx] || undefined : undefined,
        video_tag_key: keyIdx !== -1 ? values[keyIdx] || undefined : undefined,
        video_url_vimeo: values[urlIdx] || "",
      };

      // Validation
      if (!video.video_tag_title) {
        video.error = "Title is required";
      } else if (!video.video_url_vimeo) {
        video.error = "URL is required";
      } else if (!video.video_url_vimeo.startsWith("http")) {
        video.error = "Invalid URL format";
      }

      videos.push(video);
    }

    return videos;
  };

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const text = event.target?.result as string;
      const videos = parseCSV(text);
      setParsedVideos(videos);
      setResult(null);
    };
    reader.readAsText(file);
  };

  const handleSubmit = async () => {
    const validVideos = parsedVideos.filter((v) => !v.error);
    if (validVideos.length === 0) {
      toast.error("No valid videos to create");
      return;
    }

    try {
      const items = validVideos.map(({
        video_idx,
        video_state,
        video_access,
        video_tag_title,
        video_tag_subtitle,
        video_tag_key,
        video_url_vimeo,
      }) => ({
        video_idx: video_idx || undefined,
        video_state: video_state || undefined,
        video_access,
        video_tag_title,
        video_tag_subtitle: video_tag_subtitle || undefined,
        video_tag_key: video_tag_key || undefined,
        video_url_vimeo,
      }));

      const res = await bulkCreateMutation.mutateAsync({ items });
      setResult(res);
      toast.success(`Created ${res.summary.success} videos`);
    } catch {
      toast.error("Bulk creation failed");
    }
  };

  const validCount = parsedVideos.filter((v) => !v.error).length;
  const invalidCount = parsedVideos.filter((v) => v.error).length;

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/videos")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Bulk Create Videos</h1>
      </div>

      {/* CSV Format Guide */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            CSV Format
          </CardTitle>
          <CardDescription>
            Upload a CSV file with video data. Required: video_tag_title, video_url_vimeo
          </CardDescription>
        </CardHeader>
        <CardContent>
          <pre className="bg-muted p-4 rounded-md text-sm overflow-x-auto">
{`video_idx,video_state,video_access,video_tag_title,video_tag_subtitle,video_tag_key,video_url_vimeo
V001,ready,public,Lesson 1 Introduction,Welcome to lesson 1,lesson-01,https://vimeo.com/123456789
V002,open,private,Lesson 2 Basics,Basic concepts,lesson-02,https://vimeo.com/987654321`}
          </pre>
          <p className="text-sm text-muted-foreground mt-2">
            <strong>video_state</strong>: ready, open, close (default: ready) | <strong>video_access</strong>: public, paid, private, promote (default: private)
          </p>
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
      {parsedVideos.length > 0 && !result && (
        <Card>
          <CardHeader>
            <CardTitle>Preview ({parsedVideos.length} rows)</CardTitle>
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
                    <th className="h-10 px-4 text-left font-medium">State</th>
                    <th className="h-10 px-4 text-left font-medium">Access</th>
                    <th className="h-10 px-4 text-left font-medium">Title</th>
                    <th className="h-10 px-4 text-left font-medium">Key</th>
                    <th className="h-10 px-4 text-left font-medium">URL</th>
                    <th className="h-10 px-4 text-left font-medium">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {parsedVideos.map((video) => (
                    <tr key={video.rowNumber} className="border-b">
                      <td className="p-4">{video.rowNumber}</td>
                      <td className="p-4">{video.video_idx || "-"}</td>
                      <td className="p-4">{video.video_state || "ready"}</td>
                      <td className="p-4">{video.video_access}</td>
                      <td className="p-4">
                        <div className="max-w-xs truncate" title={video.video_tag_title}>
                          {video.video_tag_title}
                        </div>
                      </td>
                      <td className="p-4">{video.video_tag_key || "-"}</td>
                      <td className="p-4">
                        <div className="max-w-xs truncate" title={video.video_url_vimeo}>
                          {video.video_url_vimeo}
                        </div>
                      </td>
                      <td className="p-4">
                        {video.error ? (
                          <Badge variant="destructive" className="flex items-center gap-1 w-fit">
                            <AlertCircle className="h-3 w-3" />
                            {video.error}
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
                onClick={() => setParsedVideos([])}
              >
                Clear
              </Button>
              <Button
                onClick={handleSubmit}
                disabled={validCount === 0 || bulkCreateMutation.isPending}
              >
                {bulkCreateMutation.isPending
                  ? "Creating..."
                  : `Create ${validCount} Videos`}
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
              Total: {result.summary.total}, Success: {result.summary.success}, Failed: {result.summary.failure}
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border max-h-96 overflow-y-auto">
              <table className="w-full text-sm">
                <thead className="border-b bg-muted/50 sticky top-0">
                  <tr>
                    <th className="h-10 px-4 text-left font-medium">ID</th>
                    <th className="h-10 px-4 text-left font-medium">Status</th>
                    <th className="h-10 px-4 text-left font-medium">Message</th>
                  </tr>
                </thead>
                <tbody>
                  {result.results.map((item, idx) => (
                    <tr key={idx} className="border-b">
                      <td className="p-4">{item.id ?? "-"}</td>
                      <td className="p-4">
                        <Badge variant={item.status === 201 ? "outline" : "destructive"}>
                          {item.status}
                        </Badge>
                      </td>
                      <td className="p-4">
                        {item.error ? item.error.message : "Created"}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            <div className="flex justify-end gap-2 pt-4">
              <Button onClick={() => navigate("/admin/videos")}>
                Back to Videos
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
