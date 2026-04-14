import { ArrowLeft, CheckCircle2, Loader2 } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link, useParams, Navigate } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { WritingTask } from "@/category/study/component/writing/WritingTask";
import type { WritingStats } from "@/category/study/component/writing/WritingPracticeInput";
import { useFinishWritingSession } from "@/category/study/hook/use_writing_session";
import { useWritingPracticeSeed } from "@/category/study/hook/use_writing_practice_seed";
import {
  writingLevelSchema,
  writingPracticeTypeSchema,
  type WritingLevel,
  type WritingPayload,
  type WritingPracticeType,
  type WritingSessionRes,
} from "@/category/study/types";

const PRACTICE_TYPES_BY_LEVEL: Record<WritingLevel, WritingPracticeType[]> = {
  beginner: ["jamo", "syllable", "word"],
  intermediate: ["word", "sentence"],
  advanced: ["sentence", "paragraph"],
};

const INITIAL_STATS: WritingStats = {
  total_chars: 0,
  correct_chars: 0,
  mistakes: [],
  duration_ms: 0,
};

export function WritingPracticePage() {
  const { t } = useTranslation();
  const { level, practiceType } = useParams();

  const parsedLevel = writingLevelSchema.safeParse(level);
  if (!parsedLevel.success) {
    return <Navigate to="/studies/writing" replace />;
  }
  const validLevel = parsedLevel.data;
  const availableTypes = PRACTICE_TYPES_BY_LEVEL[validLevel];

  // 유형 미지정 → 유형 선택 화면
  if (!practiceType) {
    return (
      <div className="min-h-screen bg-muted/30">
        <div className="mx-auto w-full max-w-screen-md space-y-6 px-4 py-10">
          <Button variant="ghost" size="sm" asChild>
            <Link to="/studies/writing">
              <ArrowLeft className="mr-2 h-4 w-4" />
              {t("study.writing.backToLevels")}
            </Link>
          </Button>

          <div>
            <h1 className="text-2xl font-bold tracking-tight">
              {t(`study.writing.level.${validLevel}.title`)}
            </h1>
            <p className="mt-1 text-sm text-muted-foreground">
              {t("study.writing.selectPracticeType")}
            </p>
          </div>

          <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
            {availableTypes.map((type) => (
              <Button key={type} variant="outline" className="h-auto justify-start p-4" asChild>
                <Link to={`/studies/writing/${validLevel}/${type}`}>
                  <div className="text-left">
                    <div className="font-semibold">
                      {t(`study.writing.practiceType.${type}`)}
                    </div>
                  </div>
                </Link>
              </Button>
            ))}
          </div>
        </div>
      </div>
    );
  }

  const parsedType = writingPracticeTypeSchema.safeParse(practiceType);
  if (!parsedType.success) {
    return <Navigate to={`/studies/writing/${validLevel}`} replace />;
  }

  return (
    <FreePracticeRunner
      level={validLevel}
      practiceType={parsedType.data}
    />
  );
}

interface FreePracticeRunnerProps {
  level: WritingLevel;
  practiceType: WritingPracticeType;
}

function FreePracticeRunner({ level, practiceType }: FreePracticeRunnerProps) {
  const { t } = useTranslation();
  const { data, isPending, isError } = useWritingPracticeSeed({ level, practice_type: practiceType });

  const [currentIndex, setCurrentIndex] = useState(0);
  const [attempt, setAttempt] = useState(0);
  const [text, setText] = useState("");
  const [stats, setStats] = useState<WritingStats>(INITIAL_STATS);
  const [sessionId, setSessionId] = useState<number | null>(null);
  const [finishedSession, setFinishedSession] = useState<WritingSessionRes | null>(null);

  const finishMutation = useFinishWritingSession();

  // 레벨/유형 변경 시 완전 초기화
  useEffect(() => {
    setCurrentIndex(0);
    setAttempt(0);
    setText("");
    setStats(INITIAL_STATS);
    setSessionId(null);
    setFinishedSession(null);
  }, [level, practiceType]);

  const items = data?.items ?? [];
  const totalItems = items.length;
  const currentItem = items[currentIndex];
  const isAllCompleted = totalItems > 0 && currentIndex >= totalItems;

  // 초급 레벨만 answer 를 클라이언트에 전달 (study_task 흐름과 동일 정책)
  const writingPayload = useMemo<WritingPayload | null>(() => {
    if (!currentItem) return null;
    return {
      prompt: currentItem.prompt,
      answer: level === "beginner" ? currentItem.answer : null,
      hint: currentItem.hint ?? null,
      level,
      practice_type: practiceType,
      keyboard_visible: level === "beginner",
    };
  }, [currentItem, level, practiceType]);

  const handleFinish = () => {
    if (!text.trim() || sessionId === null || finishMutation.isPending) return;
    finishMutation.mutate(
      {
        sessionId,
        body: {
          total_chars: stats.total_chars,
          correct_chars: stats.correct_chars,
          duration_ms: stats.duration_ms,
          mistakes: stats.mistakes,
        },
      },
      {
        onSuccess: (session) => setFinishedSession(session),
      },
    );
  };

  const handleNext = () => {
    setCurrentIndex((idx) => idx + 1);
    setAttempt((a) => a + 1);
    setText("");
    setStats(INITIAL_STATS);
    setSessionId(null);
    setFinishedSession(null);
  };

  const handleRestart = () => {
    setCurrentIndex(0);
    setAttempt((a) => a + 1);
    setText("");
    setStats(INITIAL_STATS);
    setSessionId(null);
    setFinishedSession(null);
  };

  return (
    <div className="min-h-screen bg-muted/30">
      <div className="mx-auto w-full max-w-screen-md space-y-6 px-4 py-10">
        <Button variant="ghost" size="sm" asChild>
          <Link to={`/studies/writing/${level}`}>
            <ArrowLeft className="mr-2 h-4 w-4" />
            {t("study.writing.backToTypes")}
          </Link>
        </Button>

        <div className="flex items-end justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold tracking-tight">
              {t(`study.writing.practiceType.${practiceType}`)}
            </h1>
            <p className="mt-1 text-sm text-muted-foreground">
              {t(`study.writing.level.${level}.title`)}
            </p>
          </div>
          {totalItems > 0 && !isAllCompleted && (
            <div className="text-sm font-medium text-muted-foreground">
              {t("study.writing.freePracticeProgress", {
                current: Math.min(currentIndex + 1, totalItems),
                total: totalItems,
              })}
            </div>
          )}
        </div>

        {isPending && (
          <Card>
            <CardContent className="flex items-center justify-center gap-2 py-10 text-sm text-muted-foreground">
              <Loader2 className="h-4 w-4 animate-spin" />
              {t("common.loading")}
            </CardContent>
          </Card>
        )}

        {isError && (
          <Card>
            <CardContent className="py-10 text-center text-sm text-destructive">
              {t("study.writing.freePracticeLoadError")}
            </CardContent>
          </Card>
        )}

        {!isPending && !isError && totalItems === 0 && (
          <Card>
            <CardContent className="py-10 text-center text-sm text-muted-foreground">
              {t("study.writing.freePracticeEmpty")}
            </CardContent>
          </Card>
        )}

        {isAllCompleted && (
          <Card>
            <CardHeader>
              <div className="flex items-center gap-2">
                <CheckCircle2 className="h-5 w-5 text-primary" />
                <CardTitle>{t("study.writing.allCompleted")}</CardTitle>
              </div>
            </CardHeader>
            <CardContent className="flex flex-wrap gap-2">
              <Button onClick={handleRestart}>{t("study.writing.restart")}</Button>
              <Button variant="outline" asChild>
                <Link to={`/studies/writing/${level}`}>
                  {t("study.writing.selectOther")}
                </Link>
              </Button>
            </CardContent>
          </Card>
        )}

        {currentItem && writingPayload && !isAllCompleted && (
          <>
            <WritingTask
              key={`free-${level}-${practiceType}-${currentItem.seed_id}-${attempt}`}
              taskId={null}
              payload={writingPayload}
              text={text}
              onChange={setText}
              onStatsChange={setStats}
              onSessionStart={setSessionId}
              finishedSession={finishedSession}
              disabled={finishedSession !== null}
            />

            {!finishedSession ? (
              <Button
                className="w-full"
                onClick={handleFinish}
                disabled={
                  !text.trim() || sessionId === null || finishMutation.isPending
                }
              >
                {finishMutation.isPending && (
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                )}
                {t("study.writing.finishItem")}
              </Button>
            ) : (
              <Button className="w-full" onClick={handleNext}>
                {currentIndex + 1 >= totalItems
                  ? t("study.writing.finishAll")
                  : t("study.writing.nextItem")}
              </Button>
            )}
          </>
        )}
      </div>
    </div>
  );
}
