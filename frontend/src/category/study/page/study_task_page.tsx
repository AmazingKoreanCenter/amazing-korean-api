import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link, useNavigate, useParams, useSearchParams } from "react-router-dom";

import { ApiError } from "@/api/client";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useAuthStore } from "@/hooks/use_auth_store";
import { useLessonDetail } from "@/category/lesson/hook/use_lesson_detail";
import { useUpdateLessonProgress } from "@/category/lesson/hook/use_lesson_progress";
import type {
  StudyTaskKind,
  ChoicePayload,
  TypingPayload,
  VoicePayload,
  SubmitAnswerRes,
  TaskExplainRes,
} from "@/category/study/types";

import { useStudyTask } from "../hook/use_study_task";
import { useStudyDetail } from "../hook/use_study_detail";
import { useSubmitAnswer } from "../hook/use_submit_answer";
import { useTaskStatus } from "../hook/use_task_status";
import { useTaskExplain } from "../hook/use_task_explain";

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
  const { t } = useTranslation();
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
          alt={t("study.questionImage")}
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
  const { t } = useTranslation();
  return (
    <div className="space-y-4">
      <div className="text-lg font-medium">{payload.question}</div>
      {payload.image_url && (
        <img
          src={payload.image_url}
          alt={t("study.questionImage")}
          className="max-w-full rounded-lg"
        />
      )}
      <textarea
        className="w-full min-h-[120px] p-3 border rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-primary disabled:bg-muted"
        placeholder={t("study.typingPlaceholder")}
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
  const { t } = useTranslation();
  return (
    <div className="space-y-4">
      <div className="text-lg font-medium">{payload.question}</div>
      {payload.image_url && (
        <img
          src={payload.image_url}
          alt={t("study.questionImage")}
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
          {t("study.voiceNotice")}
        </p>
        <textarea
          className="w-full min-h-[80px] p-3 border rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-primary disabled:bg-muted"
          placeholder={t("study.typingPlaceholder")}
          value={text}
          onChange={(e) => onChange(e.target.value)}
          disabled={disabled}
        />
      </div>
    </div>
  );
}

function ResultCard({ result }: { result: SubmitAnswerRes }) {
  const { t } = useTranslation();
  return (
    <Card className={result.is_correct ? "border-status-success" : "border-destructive"}>
      <CardContent className="p-4">
        <div className="flex items-center gap-2 mb-2">
          <span className={`text-lg font-bold ${result.is_correct ? "text-status-success" : "text-destructive"}`}>
            {result.is_correct ? t("study.correct") : t("study.incorrect")}
          </span>
        </div>
        {result.correct_answer && (
          <p className="text-sm text-muted-foreground">
            {t("study.correctAnswer", { answer: result.correct_answer })}
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
  const { t } = useTranslation();
  return (
    <Card className="border-primary">
      <CardContent className="p-4 space-y-3">
        <div className="flex items-center gap-2">
          <span className="text-lg font-bold text-primary">{t("study.explanation")}</span>
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
            <p className="text-xs font-medium text-muted-foreground">{t("study.references")}</p>
            <ul className="text-sm space-y-1">
              {explain.resources.map((resource, index) => (
                <li key={index}>
                  <a
                    href={resource}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-primary hover:underline"
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
  const { t } = useTranslation();
  if (tryCount === 0) return null;

  return (
    <div className="flex items-center gap-2">
      <Badge variant={isSolved ? "default" : "secondary"}>
        {isSolved ? t("study.solvedBadge") : t("study.tryCount", { count: tryCount })}
      </Badge>
    </div>
  );
}

export function StudyTaskPage() {
  const { t } = useTranslation();
  const { taskId } = useParams();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);

  const id = useMemo(() => Number(taskId), [taskId]);
  const isValidId = Number.isFinite(id);

  const KIND_LABELS: Record<StudyTaskKind, string> = {
    choice: t("study.kindChoice"),
    typing: t("study.kindTypingAlt"),
    voice: t("study.kindVoiceAlt"),
  };

  // Lesson 컨텍스트 파싱 (쿼리 파라미터)
  const lessonId = useMemo(() => {
    const param = searchParams.get("lessonId");
    return param ? Number(param) : undefined;
  }, [searchParams]);

  const currentItemSeq = useMemo(() => {
    const param = searchParams.get("itemSeq");
    return param ? Number(param) : undefined;
  }, [searchParams]);

  const totalItems = useMemo(() => {
    const param = searchParams.get("totalItems");
    return param ? Number(param) : undefined;
  }, [searchParams]);

  const isInLessonContext = lessonId !== undefined && currentItemSeq !== undefined;

  const { data, isPending, isError, error } = useStudyTask(isValidId ? id : undefined);
  const submitMutation = useSubmitAnswer(id);
  const { data: statusData } = useTaskStatus(isValidId ? id : undefined);

  // Study 정보 가져오기 (다음 문제 파악용) - lesson 컨텍스트가 아닐 때만
  const studyId = data?.study_id;
  const { data: studyData } = useStudyDetail(
    !isInLessonContext ? studyId : undefined,
    { per_page: 100 }
  );

  // Lesson 데이터 조회 (lesson 컨텍스트일 때만)
  const { data: lessonData } = useLessonDetail(isInLessonContext ? lessonId : undefined);
  const updateLessonProgress = useUpdateLessonProgress(lessonId ?? 0);

  // 다음 lesson_item 찾기
  const nextLessonItem = useMemo(() => {
    if (!isInLessonContext || !lessonData?.items || !currentItemSeq) return null;
    const currentIndex = lessonData.items.findIndex((item) => item.seq === currentItemSeq);
    if (currentIndex >= 0 && currentIndex < lessonData.items.length - 1) {
      return lessonData.items[currentIndex + 1];
    }
    return null;
  }, [isInLessonContext, lessonData, currentItemSeq]);

  const isLastLessonItem = useMemo(() => {
    if (!isInLessonContext || !totalItems || !currentItemSeq) return false;
    return currentItemSeq >= totalItems;
  }, [isInLessonContext, totalItems, currentItemSeq]);

  // Form state
  const [selectedChoice, setSelectedChoice] = useState<number | null>(null);
  const [typingText, setTypingText] = useState("");
  const [voiceText, setVoiceText] = useState("");
  const [showExplain, setShowExplain] = useState(false);
  const [showCompletion, setShowCompletion] = useState(false);

  // 다음 문제 찾기
  const tasks = studyData?.tasks ?? [];
  const sortedTasks = [...tasks].sort((a, b) => a.seq - b.seq);
  const currentIndex = sortedTasks.findIndex((t) => t.task_id === id);
  const nextTask = currentIndex >= 0 && currentIndex < sortedTasks.length - 1
    ? sortedTasks[currentIndex + 1]
    : null;
  const isLastTask = currentIndex >= 0 && currentIndex === sortedTasks.length - 1 && sortedTasks.length > 0;

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
    setShowCompletion(false);
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
              {isNotFound ? t("study.notFoundTitle") : t("common.errorOccurred")}
            </CardTitle>
            <p className="text-sm text-muted-foreground">
              {isNotFound
                ? t("study.notFoundDescription")
                : t("common.temporaryError")}
            </p>
          </CardHeader>
          <CardContent>
            <Button asChild>
              <Link to="/studies/:studyId">{t("common.backToList")}</Link>
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
              {t("study.problemHashNumber", { seq: data.seq })}
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
                {t("study.viewExplanation")}
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

        {/* 마지막 문제 완료 메시지 (Study 컨텍스트) */}
        {showCompletion && studyData && !isInLessonContext && (
          <Card className="border-status-success bg-status-success/5">
            <CardContent className="p-6 text-center space-y-4">
              <div className="text-4xl">🎉</div>
              <h2 className="text-xl font-bold text-status-success">
                {t("study.completionTitle", { title: studyData.title ?? t("common.noTitle") })}
              </h2>
              <p className="text-sm text-muted-foreground">
                {t("study.completionDescription")}
              </p>
              <Button asChild>
                <Link to="/studies">{t("study.backToStudyList")}</Link>
              </Button>
            </CardContent>
          </Card>
        )}

        {!showCompletion && (
          <div className="flex justify-between gap-2">
            <Button variant="outline" asChild>
              <Link to={isInLessonContext ? `/lessons/${lessonId}` : (studyId ? `/studies/${studyId}` : "/studies")}>
                {isInLessonContext ? t("study.toLesson") : t("common.backToListShort")}
              </Link>
            </Button>
            <div className="flex gap-2">
              {!isLoggedIn ? (
                <Button asChild>
                  <Link to="/login">{t("auth.loginAndSubmit")}</Link>
                </Button>
              ) : submitMutation.isSuccess ? (
                <>
                  <Button
                    variant="outline"
                    onClick={() => {
                      setSelectedChoice(null);
                      setTypingText("");
                      setVoiceText("");
                      submitMutation.reset();
                    }}
                  >
                    {t("study.retryButton")}
                  </Button>

                  {/* Lesson 컨텍스트: 다음 아이템 또는 수업 완료 */}
                  {isInLessonContext ? (
                    isLastLessonItem ? (
                      <Button
                        onClick={() => {
                          if (lessonId && currentItemSeq) {
                            updateLessonProgress.mutate({
                              percent: 100,
                              last_seq: currentItemSeq,
                            });
                          }
                          navigate(`/lessons/${lessonId}`);
                        }}
                      >
                        {t("study.completeLesson")}
                      </Button>
                    ) : nextLessonItem ? (
                      <Button
                        asChild
                        onClick={() => {
                          if (lessonId && currentItemSeq && totalItems) {
                            const percent = Math.floor((currentItemSeq / totalItems) * 100);
                            updateLessonProgress.mutate({
                              percent,
                              last_seq: currentItemSeq,
                            });
                          }
                        }}
                      >
                        <Link
                          to={
                            nextLessonItem.kind === "video" && nextLessonItem.video_id
                              ? `/videos/${nextLessonItem.video_id}?lessonId=${lessonId}&itemSeq=${nextLessonItem.seq}&totalItems=${totalItems}`
                              : nextLessonItem.kind === "task" && nextLessonItem.task_id
                                ? `/studies/tasks/${nextLessonItem.task_id}?lessonId=${lessonId}&itemSeq=${nextLessonItem.seq}&totalItems=${totalItems}`
                                : `/lessons/${lessonId}`
                          }
                        >
                          {nextLessonItem.kind === "video" ? t("study.nextItemVideo") : t("study.nextItemTask")}
                        </Link>
                      </Button>
                    ) : null
                  ) : (
                    /* Study 컨텍스트: 기존 로직 */
                    isLastTask ? (
                      <Button onClick={() => setShowCompletion(true)}>
                        {t("study.completeLearning")}
                      </Button>
                    ) : nextTask ? (
                      <Button asChild>
                        <Link to={`/studies/tasks/${nextTask.task_id}`}>
                          {t("study.nextProblem")}
                        </Link>
                      </Button>
                    ) : null
                  )}
                </>
              ) : (
                <Button
                  onClick={handleSubmit}
                  disabled={!canSubmit()}
                >
                  {submitMutation.isPending ? t("study.submitting") : t("study.submitButton")}
                </Button>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
