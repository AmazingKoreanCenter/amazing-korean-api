import { useEffect, useMemo, useState } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";

import { ApiError } from "@/api/client";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useAuthStore } from "@/hooks/use_auth_store";
import type {
  StudyTaskKind,
  ChoicePayload,
  TypingPayload,
  VoicePayload,
  SubmitAnswerRes,
  TaskExplainRes,
} from "@/category/study/types";

import { useStudyTask } from "../hook/use_study_task";
import { useSubmitAnswer } from "../hook/use_submit_answer";
import { useTaskStatus } from "../hook/use_task_status";
import { useTaskExplain } from "../hook/use_task_explain";

const KIND_LABELS: Record<StudyTaskKind, string> = {
  choice: "객관식",
  typing: "주관식",
  voice: "음성",
};

const formatDate = (value: string) => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;

  return date.toLocaleDateString("ko-KR", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  });
};

interface ChoiceTaskProps {
  payload: ChoicePayload;
  selectedChoice: number | null;
  onSelect: (choice: number) => void;
  disabled?: boolean;
}

function ChoiceTask({ payload, selectedChoice, onSelect, disabled }: ChoiceTaskProps) {
  const choices = [
    payload.choice_1,
    payload.choice_2,
    payload.choice_3,
    payload.choice_4,
  ].filter(Boolean);

  return (
    <div className="space-y-4">
      <div className="text-lg font-medium">{payload.question}</div>
      {payload.image_url && (
        <img
          src={payload.image_url}
          alt="문제 이미지"
          className="max-w-full rounded-lg"
        />
      )}
      {payload.audio_url && (
        <audio controls className="w-full">
          <source src={payload.audio_url} />
        </audio>
      )}
      <div className="space-y-2">
        {choices.map((choice, index) => (
          <Button
            key={index}
            variant={selectedChoice === index + 1 ? "default" : "outline"}
            className="w-full justify-start text-left h-auto py-3 px-4"
            onClick={() => onSelect(index + 1)}
            disabled={disabled}
          >
            <span className="mr-3 font-bold">{index + 1}.</span>
            {choice}
          </Button>
        ))}
      </div>
    </div>
  );
}

interface TypingTaskProps {
  payload: TypingPayload;
  text: string;
  onChange: (text: string) => void;
  disabled?: boolean;
}

function TypingTask({ payload, text, onChange, disabled }: TypingTaskProps) {
  return (
    <div className="space-y-4">
      <div className="text-lg font-medium">{payload.question}</div>
      {payload.image_url && (
        <img
          src={payload.image_url}
          alt="문제 이미지"
          className="max-w-full rounded-lg"
        />
      )}
      <textarea
        className="w-full min-h-[120px] p-3 border rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-primary disabled:bg-muted"
        placeholder="답을 입력하세요..."
        value={text}
        onChange={(e) => onChange(e.target.value)}
        disabled={disabled}
      />
    </div>
  );
}

interface VoiceTaskProps {
  payload: VoicePayload;
  text: string;
  onChange: (text: string) => void;
  disabled?: boolean;
}

function VoiceTask({ payload, text, onChange, disabled }: VoiceTaskProps) {
  return (
    <div className="space-y-4">
      <div className="text-lg font-medium">{payload.question}</div>
      {payload.image_url && (
        <img
          src={payload.image_url}
          alt="문제 이미지"
          className="max-w-full rounded-lg"
        />
      )}
      {payload.audio_url && (
        <audio controls className="w-full">
          <source src={payload.audio_url} />
        </audio>
      )}
      <div className="space-y-2">
        <p className="text-sm text-muted-foreground">
          음성 입력 대신 텍스트로 답변해주세요.
        </p>
        <textarea
          className="w-full min-h-[80px] p-3 border rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-primary disabled:bg-muted"
          placeholder="답을 입력하세요..."
          value={text}
          onChange={(e) => onChange(e.target.value)}
          disabled={disabled}
        />
      </div>
    </div>
  );
}

function ResultCard({ result }: { result: SubmitAnswerRes }) {
  return (
    <Card className={result.is_correct ? "border-green-500" : "border-red-500"}>
      <CardContent className="p-4">
        <div className="flex items-center gap-2 mb-2">
          <span className={`text-lg font-bold ${result.is_correct ? "text-green-600" : "text-red-600"}`}>
            {result.is_correct ? "정답입니다!" : "오답입니다."}
          </span>
        </div>
        {result.correct_answer && (
          <p className="text-sm text-muted-foreground">
            정답: {result.correct_answer}
          </p>
        )}
        {result.explanation && (
          <p className="text-sm mt-2">{result.explanation}</p>
        )}
      </CardContent>
    </Card>
  );
}

function ExplainCard({ explain }: { explain: TaskExplainRes }) {
  return (
    <Card className="border-blue-500">
      <CardContent className="p-4 space-y-3">
        <div className="flex items-center gap-2">
          <span className="text-lg font-bold text-blue-600">해설</span>
        </div>
        {explain.title && (
          <h4 className="font-medium">{explain.title}</h4>
        )}
        {explain.explanation && (
          <p className="text-sm text-muted-foreground whitespace-pre-wrap">
            {explain.explanation}
          </p>
        )}
        {explain.resources.length > 0 && (
          <div className="space-y-1">
            <p className="text-xs font-medium text-muted-foreground">참고 자료</p>
            <ul className="text-sm space-y-1">
              {explain.resources.map((resource, index) => (
                <li key={index}>
                  <a
                    href={resource}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-blue-600 hover:underline"
                  >
                    {resource}
                  </a>
                </li>
              ))}
            </ul>
          </div>
        )}
      </CardContent>
    </Card>
  );
}

function StatusBadge({ tryCount, isSolved }: { tryCount: number; isSolved: boolean }) {
  if (tryCount === 0) return null;

  return (
    <div className="flex items-center gap-2">
      <Badge variant={isSolved ? "default" : "secondary"}>
        {isSolved ? "풀이 완료" : `${tryCount}회 시도`}
      </Badge>
    </div>
  );
}

export function StudyTaskPage() {
  const { taskId } = useParams();
  const navigate = useNavigate();
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);

  const id = useMemo(() => Number(taskId), [taskId]);
  const isValidId = Number.isFinite(id);

  const { data, isPending, isError, error } = useStudyTask(isValidId ? id : undefined);
  const submitMutation = useSubmitAnswer(id);
  const { data: statusData } = useTaskStatus(isValidId ? id : undefined);

  // Form state
  const [selectedChoice, setSelectedChoice] = useState<number | null>(null);
  const [typingText, setTypingText] = useState("");
  const [voiceText, setVoiceText] = useState("");
  const [showExplain, setShowExplain] = useState(false);

  // 해설은 1회 이상 시도 후에만 조회 가능
  const canViewExplain = (statusData?.try_count ?? 0) > 0 || submitMutation.isSuccess;
  const { data: explainData, isFetching: isExplainFetching } = useTaskExplain(
    isValidId ? id : undefined,
    showExplain && canViewExplain
  );

  useEffect(() => {
    if (!isValidId) {
      navigate("/studies", { replace: true });
    }
  }, [isValidId, navigate]);

  // Reset form when task changes
  useEffect(() => {
    setSelectedChoice(null);
    setTypingText("");
    setVoiceText("");
    setShowExplain(false);
    submitMutation.reset();
  }, [id]);

  const handleSubmit = () => {
    if (!data) return;

    switch (data.kind) {
      case "choice":
        if (selectedChoice !== null) {
          submitMutation.mutate({ kind: "choice", pick: selectedChoice });
        }
        break;
      case "typing":
        if (typingText.trim()) {
          submitMutation.mutate({ kind: "typing", text: typingText.trim() });
        }
        break;
      case "voice":
        if (voiceText.trim()) {
          submitMutation.mutate({ kind: "voice", text: voiceText.trim() });
        }
        break;
    }
  };

  const canSubmit = () => {
    if (!data || !isLoggedIn || submitMutation.isPending) return false;

    switch (data.kind) {
      case "choice":
        return selectedChoice !== null;
      case "typing":
        return typingText.trim().length > 0;
      case "voice":
        return voiceText.trim().length > 0;
      default:
        return false;
    }
  };

  if (!isValidId) return null;

  if (isPending) {
    return (
      <div className="min-h-screen bg-muted/30">
        <div className="mx-auto w-full max-w-screen-md space-y-6 px-4 py-10">
          <Skeleton className="h-8 w-1/3" />
          <Skeleton className="h-4 w-1/4" />
          <Card>
            <CardContent className="p-6 space-y-4">
              <Skeleton className="h-6 w-full" />
              <Skeleton className="h-32 w-full" />
              <Skeleton className="h-12 w-full" />
              <Skeleton className="h-12 w-full" />
            </CardContent>
          </Card>
        </div>
      </div>
    );
  }

  if (isError || !data) {
    const isNotFound = error instanceof ApiError && error.status === 404;
    return (
      <div className="min-h-screen bg-muted/30 flex items-center justify-center p-4">
        <Card className="w-full max-w-md text-center">
          <CardHeader>
            <CardTitle>
              {isNotFound ? "문제를 찾을 수 없습니다." : "오류 발생"}
            </CardTitle>
            <p className="text-sm text-muted-foreground">
              {isNotFound
                ? "존재하지 않거나 삭제된 문제입니다."
                : "일시적인 오류입니다. 다시 시도해주세요."}
            </p>
          </CardHeader>
          <CardContent>
            <Button asChild>
              <Link to="/studies/:studyId">목록으로 돌아가기</Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  const renderTask = () => {
    const isSubmitted = submitMutation.isSuccess;

    switch (data.kind) {
      case "choice":
        return (
          <ChoiceTask
            payload={data.payload as ChoicePayload}
            selectedChoice={selectedChoice}
            onSelect={setSelectedChoice}
            disabled={isSubmitted}
          />
        );
      case "typing":
        return (
          <TypingTask
            payload={data.payload as TypingPayload}
            text={typingText}
            onChange={setTypingText}
            disabled={isSubmitted}
          />
        );
      case "voice":
        return (
          <VoiceTask
            payload={data.payload as VoicePayload}
            text={voiceText}
            onChange={setVoiceText}
            disabled={isSubmitted}
          />
        );
      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-muted/30">
      <div className="mx-auto w-full max-w-screen-md space-y-6 px-4 py-10">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold tracking-tight">
              문제 #{data.seq}
            </h1>
            <p className="text-sm text-muted-foreground">
              {formatDate(data.created_at)}
            </p>
          </div>
          <div className="flex items-center gap-2">
            {statusData && (
              <StatusBadge tryCount={statusData.try_count} isSolved={statusData.is_solved} />
            )}
            <Badge variant="secondary">{KIND_LABELS[data.kind]}</Badge>
          </div>
        </div>

        <Card>
          <CardContent className="p-6">{renderTask()}</CardContent>
        </Card>

        {submitMutation.isSuccess && submitMutation.data && (
          <ResultCard result={submitMutation.data} />
        )}

        {canViewExplain && (
          <div className="space-y-4">
            {!showExplain ? (
              <Button
                variant="outline"
                className="w-full"
                onClick={() => setShowExplain(true)}
              >
                해설 보기
              </Button>
            ) : isExplainFetching ? (
              <Card>
                <CardContent className="p-4">
                  <Skeleton className="h-4 w-1/3 mb-2" />
                  <Skeleton className="h-4 w-full" />
                  <Skeleton className="h-4 w-2/3 mt-2" />
                </CardContent>
              </Card>
            ) : explainData ? (
              <ExplainCard explain={explainData} />
            ) : null}
          </div>
        )}

        <div className="flex justify-between">
          <Button variant="outline" asChild>
            <Link to="/studies">목록으로</Link>
          </Button>
          {!isLoggedIn ? (
            <Button asChild>
              <Link to="/login">로그인하고 제출하기</Link>
            </Button>
          ) : submitMutation.isSuccess ? (
            <Button
              onClick={() => {
                setSelectedChoice(null);
                setTypingText("");
                setVoiceText("");
                submitMutation.reset();
              }}
            >
              다시 풀기
            </Button>
          ) : (
            <Button
              onClick={handleSubmit}
              disabled={!canSubmit()}
            >
              {submitMutation.isPending ? "제출 중..." : "제출하기"}
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}
