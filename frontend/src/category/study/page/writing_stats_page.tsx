import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { ArrowLeft, Calendar, Gauge, Keyboard, Target, TrendingUp } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useWritingStats } from "@/category/study/hook/use_writing_stats";
import type {
  WritingDailyStat,
  WritingLevel,
  WritingLevelStat,
  WritingWeakChar,
} from "@/category/study/types";

const DAYS_OPTIONS = [7, 14, 30, 60, 90, 180, 365] as const;

const LEVEL_ORDER: WritingLevel[] = ["beginner", "intermediate", "advanced"];

function formatNumber(n: number): string {
  return n.toLocaleString();
}

function formatPercent(n: number): string {
  return `${n.toFixed(1)}%`;
}

function formatCpm(n: number): string {
  return n.toFixed(1);
}

// ==========================================
// Summary Card
// ==========================================

interface SummaryCardProps {
  title: string;
  value: string;
  subtitle?: string;
  icon: React.ElementType;
  loading?: boolean;
}

function SummaryCard({ title, value, subtitle, icon: Icon, loading }: SummaryCardProps) {
  return (
    <Card>
      <CardContent className="p-6">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-muted-foreground">{title}</p>
            {loading ? (
              <Skeleton className="mt-2 h-8 w-24" />
            ) : (
              <>
                <p className="mt-2 text-2xl font-bold">{value}</p>
                {subtitle && (
                  <p className="mt-1 text-xs text-muted-foreground">{subtitle}</p>
                )}
              </>
            )}
          </div>
          <div className="flex h-12 w-12 items-center justify-center rounded-full bg-primary/10">
            <Icon className="h-6 w-6 text-primary" />
          </div>
        </div>
      </CardContent>
    </Card>
  );
}

// ==========================================
// Level Breakdown Card
// ==========================================

function LevelBreakdownCard({
  items,
  loading,
}: {
  items: WritingLevelStat[];
  loading: boolean;
}) {
  const { t } = useTranslation();

  const maxSessions = useMemo(() => {
    return items.reduce((max, item) => Math.max(max, item.sessions), 0);
  }, [items]);

  const sorted = useMemo(() => {
    return [...items].sort(
      (a, b) =>
        LEVEL_ORDER.indexOf(a.writing_level) - LEVEL_ORDER.indexOf(b.writing_level),
    );
  }, [items]);

  if (loading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("study.writing.stats.levelTitle")}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          {Array.from({ length: 3 }).map((_, i) => (
            <Skeleton key={i} className="h-16 w-full" />
          ))}
        </CardContent>
      </Card>
    );
  }

  if (sorted.length === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("study.writing.stats.levelTitle")}</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="py-6 text-center text-sm text-muted-foreground">
            {t("study.writing.stats.noLevelData")}
          </p>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base">{t("study.writing.stats.levelTitle")}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {sorted.map((item) => {
          const widthPercent =
            maxSessions > 0 ? (item.sessions / maxSessions) * 100 : 0;
          return (
            <div key={item.writing_level} className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="font-medium">
                  {t(`study.writing.level.${item.writing_level}.title`)}
                </span>
                <span className="text-muted-foreground">
                  {t("study.writing.stats.sessionCount", {
                    count: item.sessions,
                  })}
                </span>
              </div>
              <div className="h-2 overflow-hidden rounded-full bg-muted">
                <div
                  className="h-full rounded-full bg-primary"
                  style={{ width: `${widthPercent}%` }}
                />
              </div>
              <div className="flex justify-between text-xs text-muted-foreground">
                <span>
                  {t("study.writing.stats.accuracyLabel")}:{" "}
                  {formatPercent(item.avg_accuracy)}
                </span>
                <span>
                  {t("study.writing.stats.cpmLabel")}: {formatCpm(item.avg_cpm)}
                </span>
              </div>
            </div>
          );
        })}
      </CardContent>
    </Card>
  );
}

// ==========================================
// Daily Trend Card
// ==========================================

function DailyTrendCard({
  items,
  loading,
}: {
  items: WritingDailyStat[];
  loading: boolean;
}) {
  const { t } = useTranslation();

  // 최신 날짜 순으로 정렬
  const sorted = useMemo(() => {
    return [...items].sort((a, b) => b.day.localeCompare(a.day));
  }, [items]);

  if (loading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("study.writing.stats.trendTitle")}</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-2">
            {Array.from({ length: 5 }).map((_, i) => (
              <Skeleton key={i} className="h-8 w-full" />
            ))}
          </div>
        </CardContent>
      </Card>
    );
  }

  if (sorted.length === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("study.writing.stats.trendTitle")}</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="py-6 text-center text-sm text-muted-foreground">
            {t("study.writing.stats.noTrendData")}
          </p>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base">{t("study.writing.stats.trendTitle")}</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead className="border-b">
              <tr className="text-left">
                <th className="py-2 font-medium text-muted-foreground">
                  {t("study.writing.stats.trendDay")}
                </th>
                <th className="py-2 text-right font-medium text-muted-foreground">
                  {t("study.writing.stats.trendSessions")}
                </th>
                <th className="py-2 text-right font-medium text-muted-foreground">
                  {t("study.writing.stats.trendAccuracy")}
                </th>
                <th className="py-2 text-right font-medium text-muted-foreground">
                  {t("study.writing.stats.trendCpm")}
                </th>
              </tr>
            </thead>
            <tbody>
              {sorted.map((row) => (
                <tr key={row.day} className="border-b last:border-0">
                  <td className="py-2">{row.day}</td>
                  <td className="py-2 text-right font-medium">
                    {formatNumber(row.sessions)}
                  </td>
                  <td className="py-2 text-right">{formatPercent(row.avg_accuracy)}</td>
                  <td className="py-2 text-right">{formatCpm(row.avg_cpm)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </CardContent>
    </Card>
  );
}

// ==========================================
// Weak Chars Card
// ==========================================

function WeakCharsCard({
  items,
  loading,
}: {
  items: WritingWeakChar[];
  loading: boolean;
}) {
  const { t } = useTranslation();

  const maxMiss = useMemo(() => {
    return items.reduce((max, item) => Math.max(max, item.miss_count), 0);
  }, [items]);

  if (loading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("study.writing.stats.weakTitle")}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          {Array.from({ length: 5 }).map((_, i) => (
            <Skeleton key={i} className="h-8 w-full" />
          ))}
        </CardContent>
      </Card>
    );
  }

  if (items.length === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("study.writing.stats.weakTitle")}</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="py-6 text-center text-sm text-muted-foreground">
            {t("study.writing.stats.noWeakData")}
          </p>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base">{t("study.writing.stats.weakTitle")}</CardTitle>
      </CardHeader>
      <CardContent>
        <p className="mb-3 text-xs text-muted-foreground">
          {t("study.writing.stats.weakSubtitle")}
        </p>
        <div className="space-y-2">
          {items.map((item) => {
            const widthPercent = maxMiss > 0 ? (item.miss_count / maxMiss) * 100 : 0;
            return (
              <div key={item.expected} className="flex items-center gap-3">
                <div className="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-md border bg-destructive/5 text-lg font-bold text-destructive">
                  {item.expected}
                </div>
                <div className="min-w-0 flex-1">
                  <div className="h-6 overflow-hidden rounded-full bg-muted">
                    <div
                      className="flex h-full items-center justify-end rounded-full bg-destructive/70 px-2 text-xs font-medium text-destructive-foreground"
                      style={{ width: `${Math.max(widthPercent, 10)}%` }}
                    >
                      {t("study.writing.stats.missCount", {
                        count: item.miss_count,
                      })}
                    </div>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </CardContent>
    </Card>
  );
}

// ==========================================
// Main Page
// ==========================================

export function WritingStatsPage() {
  const { t } = useTranslation();
  const [days, setDays] = useState<number>(30);

  const { data, isPending } = useWritingStats({ days });

  const totalSessions = data?.total_sessions ?? 0;
  const avgAccuracy = data?.avg_accuracy ?? 0;
  const avgCpm = data?.avg_cpm ?? 0;

  return (
    <div className="min-h-screen bg-muted/30">
      <div className="mx-auto w-full max-w-screen-lg space-y-6 px-4 py-10">
        <div className="flex flex-wrap items-start justify-between gap-4">
          <div>
            <Button variant="ghost" size="sm" asChild className="mb-2 -ml-2">
              <Link to="/studies/writing">
                <ArrowLeft className="mr-2 h-4 w-4" />
                {t("study.writing.backToLevels")}
              </Link>
            </Button>
            <h1 className="text-3xl font-bold tracking-tight">
              {t("study.writing.stats.pageTitle")}
            </h1>
            <p className="mt-1 text-sm text-muted-foreground">
              {t("study.writing.stats.pageDescription")}
            </p>
          </div>
          <div className="flex items-center gap-2">
            <Calendar className="h-4 w-4 text-muted-foreground" />
            <Select
              value={String(days)}
              onValueChange={(value) => setDays(Number(value))}
            >
              <SelectTrigger className="w-[140px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {DAYS_OPTIONS.map((d) => (
                  <SelectItem key={d} value={String(d)}>
                    {t("study.writing.stats.lastDays", { count: d })}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </div>

        <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
          <SummaryCard
            title={t("study.writing.stats.totalSessions")}
            value={formatNumber(totalSessions)}
            icon={Keyboard}
            loading={isPending}
          />
          <SummaryCard
            title={t("study.writing.stats.avgAccuracy")}
            value={formatPercent(avgAccuracy)}
            icon={Target}
            loading={isPending}
          />
          <SummaryCard
            title={t("study.writing.stats.avgCpm")}
            value={formatCpm(avgCpm)}
            subtitle={t("study.writing.stats.cpmUnit")}
            icon={Gauge}
            loading={isPending}
          />
        </div>

        <div className="grid grid-cols-1 gap-4 lg:grid-cols-2">
          <LevelBreakdownCard items={data?.level_breakdown ?? []} loading={isPending} />
          <WeakCharsCard items={data?.weak_chars ?? []} loading={isPending} />
        </div>

        <DailyTrendCard items={data?.recent_trend ?? []} loading={isPending} />

        {totalSessions === 0 && !isPending && (
          <Card className="border-dashed">
            <CardContent className="p-8 text-center">
              <TrendingUp className="mx-auto mb-3 h-10 w-10 text-muted-foreground" />
              <h3 className="mb-1 font-semibold">
                {t("study.writing.stats.emptyTitle")}
              </h3>
              <p className="mb-4 text-sm text-muted-foreground">
                {t("study.writing.stats.emptyDescription")}
              </p>
              <Button asChild>
                <Link to="/studies/writing">
                  {t("study.writing.stats.emptyAction")}
                </Link>
              </Button>
            </CardContent>
          </Card>
        )}
      </div>
    </div>
  );
}
