import { useTranslation } from "react-i18next";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import type { WritingSessionRes } from "@/category/study/types";

interface WritingResultPanelProps {
  session: WritingSessionRes;
}

export function WritingResultPanel({ session }: WritingResultPanelProps) {
  const { t } = useTranslation();

  const accuracy = Math.round(session.accuracy_rate * 10) / 10;
  const cpm = Math.round(session.chars_per_minute * 10) / 10;

  return (
    <Card className="border-primary/40">
      <CardContent className="space-y-4 p-6">
        <div className="flex items-center justify-between">
          <h3 className="text-lg font-semibold">{t("study.writing.resultTitle")}</h3>
          <Badge variant={accuracy >= 95 ? "default" : "secondary"}>
            {t("study.writing.resultAccuracyBadge", { percent: accuracy })}
          </Badge>
        </div>

        <div className="grid grid-cols-3 gap-3">
          <Stat label={t("study.writing.statAccuracy")} value={`${accuracy}%`} />
          <Stat label={t("study.writing.statCpm")} value={`${cpm}`} />
          <Stat
            label={t("study.writing.statChars")}
            value={`${session.correct_chars}/${session.total_chars}`}
          />
        </div>

        {session.mistakes.length > 0 && (
          <div className="space-y-2">
            <p className="text-xs font-medium uppercase tracking-wide text-muted-foreground">
              {t("study.writing.mistakesLabel")}
            </p>
            <div className="flex flex-wrap gap-2">
              {session.mistakes.slice(0, 20).map((m, idx) => (
                <span
                  key={`${m.position}-${idx}`}
                  className="inline-flex items-center gap-1 rounded-md border border-destructive/40 bg-destructive/5 px-2 py-1 text-xs"
                >
                  <span className="font-medium text-destructive">{m.actual || "∅"}</span>
                  <span className="text-muted-foreground">→</span>
                  <span className="font-medium text-status-success">{m.expected}</span>
                </span>
              ))}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-md border bg-background p-3 text-center">
      <p className="text-xs text-muted-foreground">{label}</p>
      <p className="mt-1 text-xl font-bold">{value}</p>
    </div>
  );
}
