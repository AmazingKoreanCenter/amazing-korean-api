import { useState, useRef, useCallback } from "react";
import * as tus from "tus-js-client";
import { Upload, X, FileVideo, Loader2 } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { cn } from "@/lib/utils";

import { useCreateVimeoUploadTicket } from "../hook/use_admin_videos";

interface VimeoUploaderProps {
  onUploadComplete: (vimeoVideoId: string) => void;
  onError?: (error: Error) => void;
  disabled?: boolean;
  className?: string;
}

// 파일 크기 포맷 헬퍼
function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

export function VimeoUploader({
  onUploadComplete,
  onError,
  disabled = false,
  className,
}: VimeoUploaderProps) {
  const [file, setFile] = useState<File | null>(null);
  const [progress, setProgress] = useState(0);
  const [isUploading, setIsUploading] = useState(false);
  const [isDragOver, setIsDragOver] = useState(false);

  const uploadRef = useRef<tus.Upload | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const ticketMutation = useCreateVimeoUploadTicket();

  const handleUpload = useCallback(
    async (selectedFile: File) => {
      setFile(selectedFile);
      setIsUploading(true);
      setProgress(0);

      try {
        // 1. 백엔드에서 업로드 티켓 받기
        const ticket = await ticketMutation.mutateAsync({
          file_name: selectedFile.name,
          file_size: selectedFile.size,
        });

        // 2. tus 업로드 시작
        const upload = new tus.Upload(selectedFile, {
          uploadUrl: ticket.upload_link,
          retryDelays: [0, 3000, 5000, 10000, 20000],
          metadata: {
            filename: selectedFile.name,
            filetype: selectedFile.type,
          },
          onError: (error) => {
            setIsUploading(false);
            onError?.(error);
          },
          onProgress: (bytesUploaded, bytesTotal) => {
            const percentage = (bytesUploaded / bytesTotal) * 100;
            setProgress(percentage);
          },
          onSuccess: () => {
            setIsUploading(false);
            setProgress(100);
            onUploadComplete(ticket.vimeo_video_id);
          },
        });

        uploadRef.current = upload;
        upload.start();
      } catch (error) {
        setIsUploading(false);
        onError?.(error as Error);
      }
    },
    [ticketMutation, onUploadComplete, onError]
  );

  const handleCancel = useCallback(() => {
    if (uploadRef.current) {
      uploadRef.current.abort();
    }
    setIsUploading(false);
    setFile(null);
    setProgress(0);
  }, []);

  const handleFileSelect = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const selectedFile = e.target.files?.[0];
      if (selectedFile) {
        handleUpload(selectedFile);
      }
    },
    [handleUpload]
  );

  const handleDrop = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      e.preventDefault();
      setIsDragOver(false);

      if (disabled || isUploading) return;

      const droppedFile = e.dataTransfer.files[0];
      if (droppedFile && droppedFile.type.startsWith("video/")) {
        handleUpload(droppedFile);
      }
    },
    [disabled, isUploading, handleUpload]
  );

  const handleDragOver = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      e.preventDefault();
      if (!disabled && !isUploading) {
        setIsDragOver(true);
      }
    },
    [disabled, isUploading]
  );

  const handleDragLeave = useCallback(() => {
    setIsDragOver(false);
  }, []);

  const handleClick = useCallback(() => {
    if (!disabled && !isUploading) {
      fileInputRef.current?.click();
    }
  }, [disabled, isUploading]);

  // 업로드 중 상태
  if (isUploading && file) {
    return (
      <div className={cn("rounded-lg border p-4", className)}>
        <div className="flex items-center gap-3 mb-3">
          <FileVideo className="h-8 w-8 text-muted-foreground flex-shrink-0" />
          <div className="flex-1 min-w-0">
            <p className="font-medium truncate">{file.name}</p>
            <p className="text-sm text-muted-foreground">
              {formatFileSize(file.size * (progress / 100))} /{" "}
              {formatFileSize(file.size)}
            </p>
          </div>
          <Button
            type="button"
            variant="ghost"
            size="icon"
            onClick={handleCancel}
          >
            <X className="h-4 w-4" />
          </Button>
        </div>
        <div className="flex items-center gap-3">
          <Progress value={progress} className="flex-1" />
          <span className="text-sm font-medium w-12 text-right">
            {Math.round(progress)}%
          </span>
        </div>
        {ticketMutation.isPending && (
          <div className="flex items-center gap-2 mt-2 text-sm text-muted-foreground">
            <Loader2 className="h-4 w-4 animate-spin" />
            <span>Vimeo 업로드 준비 중...</span>
          </div>
        )}
      </div>
    );
  }

  // 업로드 완료 상태
  if (progress === 100 && file) {
    return (
      <div className={cn("rounded-lg border border-green-200 bg-green-50 p-4", className)}>
        <div className="flex items-center gap-3">
          <FileVideo className="h-8 w-8 text-green-600 flex-shrink-0" />
          <div className="flex-1 min-w-0">
            <p className="font-medium text-green-700 truncate">{file.name}</p>
            <p className="text-sm text-green-600">업로드 완료</p>
          </div>
          <Button
            type="button"
            variant="ghost"
            size="sm"
            onClick={() => {
              setFile(null);
              setProgress(0);
            }}
          >
            다른 파일 선택
          </Button>
        </div>
      </div>
    );
  }

  // 드래그 앤 드롭 영역
  return (
    <div
      className={cn(
        "rounded-lg border-2 border-dashed p-8 text-center transition-colors cursor-pointer",
        isDragOver && "border-primary bg-primary/5",
        disabled && "opacity-50 cursor-not-allowed",
        className
      )}
      onDrop={handleDrop}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onClick={handleClick}
    >
      <input
        ref={fileInputRef}
        type="file"
        accept="video/*"
        className="hidden"
        onChange={handleFileSelect}
        disabled={disabled}
      />
      <Upload className="h-10 w-10 mx-auto text-muted-foreground mb-4" />
      <p className="font-medium mb-1">
        파일을 끌어다 놓거나 클릭하여 선택하세요
      </p>
      <p className="text-sm text-muted-foreground">
        지원 형식: MP4, MOV, AVI (최대 5GB)
      </p>
    </div>
  );
}
