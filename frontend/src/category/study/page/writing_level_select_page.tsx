import { ArrowRight, BarChart3, GraduationCap, Keyboard, PenLine } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { WritingLevel } from "@/category/study/types";

interface LevelMeta {
  level: WritingLevel;
  icon: typeof Keyboard;
}

const LEVELS: LevelMeta[] = [
  { level: "beginner", icon: Keyboard },
  { level: "intermediate", icon: PenLine },
  { level: "advanced", icon: GraduationCap },
];

export function WritingLevelSelectPage() {
  const { t } = useTranslation();

  return (
    <div className="min-h-screen bg-muted/30">
      <div className="mx-auto w-full max-w-screen-lg space-y-8 px-4 py-10">
        <div className="flex flex-wrap items-start justify-between gap-4">
          <div>
            <Badge variant="secondary" className="mb-3">
              {t("study.writing.landingBadge")}
            </Badge>
            <h1 className="text-3xl font-bold tracking-tight">
              {t("study.writing.landingTitle")}
            </h1>
            <p className="mt-2 max-w-2xl text-sm text-muted-foreground">
              {t("study.writing.landingDescription")}
            </p>
          </div>
          <Button variant="outline" asChild>
            <Link to="/studies/writing/stats">
              <BarChart3 className="mr-2 h-4 w-4" />
              {t("study.writing.viewStats")}
            </Link>
          </Button>
        </div>

        <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
          {LEVELS.map(({ level, icon: Icon }) => (
            <Card key={level} className="flex flex-col">
              <CardHeader>
                <div className="flex items-center gap-2">
                  <Icon className="h-5 w-5 text-primary" />
                  <CardTitle className="text-lg">
                    {t(`study.writing.level.${level}.title`)}
                  </CardTitle>
                </div>
              </CardHeader>
              <CardContent className="flex flex-1 flex-col justify-between gap-4">
                <p className="text-sm text-muted-foreground">
                  {t(`study.writing.level.${level}.description`)}
                </p>
                <Button asChild className="w-full">
                  <Link to={`/studies/writing/${level}`}>
                    {t("study.writing.startLevel")}
                    <ArrowRight className="ml-2 h-4 w-4" />
                  </Link>
                </Button>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </div>
  );
}
