import { ArrowLeft, Construction } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Link, useParams, Navigate } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  writingLevelSchema,
  writingPracticeTypeSchema,
  type WritingLevel,
  type WritingPracticeType,
} from "@/category/study/types";

const PRACTICE_TYPES_BY_LEVEL: Record<WritingLevel, WritingPracticeType[]> = {
  beginner: ["jamo", "syllable", "word"],
  intermediate: ["word", "sentence"],
  advanced: ["sentence", "paragraph"],
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

  // 유형 선택 완료 → 실제 연습 세션 (P10 시드 이후 활성화)
  return (
    <div className="min-h-screen bg-muted/30">
      <div className="mx-auto w-full max-w-screen-md space-y-6 px-4 py-10">
        <Button variant="ghost" size="sm" asChild>
          <Link to={`/studies/writing/${validLevel}`}>
            <ArrowLeft className="mr-2 h-4 w-4" />
            {t("study.writing.backToTypes")}
          </Link>
        </Button>

        <Card>
          <CardHeader>
            <div className="flex items-center gap-2">
              <Construction className="h-5 w-5 text-muted-foreground" />
              <CardTitle>{t("study.writing.freePracticeComingTitle")}</CardTitle>
            </div>
          </CardHeader>
          <CardContent className="space-y-3 text-sm text-muted-foreground">
            <p>{t("study.writing.freePracticeComingDescription")}</p>
            <p>
              {t("study.writing.freePracticeMeta", {
                level: t(`study.writing.level.${validLevel}.title`),
                type: t(`study.writing.practiceType.${parsedType.data}`),
              })}
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
