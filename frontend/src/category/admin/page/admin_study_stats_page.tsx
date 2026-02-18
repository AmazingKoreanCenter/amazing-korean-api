import { useState, useMemo } from "react";
import { Link } from "react-router-dom";
import {
  CalendarDays,
  TrendingUp,
  BookOpen,
  CheckCircle,
  Users,
  Target,
  ArrowRight,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  useStudyStatsSummary,
  useStudyStatsTop,
  useStudyStatsDaily,
} from "../hook/use_admin_studies";
import type { StudyStatsQuery, TopStudiesQuery, TopStudyItem } from "../types";

// 날짜 헬퍼
function getDefaultDateRange(): { from: string; to: string } {
  const today = new Date();
  const to = today.toISOString().split("T")[0];
  const from = new Date(today.setDate(today.getDate() - 30)).toISOString().split("T")[0];
  return { from, to };
}

function formatNumber(n: number): string {
  return n.toLocaleString();
}

function formatPercent(n: number): string {
  return n.toFixed(1) + "%";
}

// ==========================================
// Summary Cards
// ==========================================

function SummaryCard({
  title,
  value,
  icon: Icon,
  loading,
  subtitle,
}: {
  title: string;
  value: number | string;
  icon: React.ElementType;
  loading?: boolean;
  subtitle?: string;
}) {
  return (
    <div className="rounded-lg border bg-card p-6">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm font-medium text-muted-foreground">{title}</p>
          {loading ? (
            <Skeleton className="h-8 w-24 mt-2" />
          ) : (
            <>
              <p className="text-2xl font-bold mt-2">{value}</p>
              {subtitle && (
                <p className="text-xs text-muted-foreground mt-1">{subtitle}</p>
              )}
            </>
          )}
        </div>
        <div className="h-12 w-12 rounded-full bg-primary/10 flex items-center justify-center">
          <Icon className="h-6 w-6 text-primary" />
        </div>
      </div>
    </div>
  );
}

// ==========================================
// Distribution Cards
// ==========================================

function DistributionCard({
  title,
  items,
  loading,
}: {
  title: string;
  items: { label: string; value: number; color: string }[];
  loading?: boolean;
}) {
  const total = items.reduce((sum, item) => sum + item.value, 0);

  return (
    <div className="rounded-lg border bg-card p-6">
      <h3 className="text-sm font-medium text-muted-foreground mb-4">{title}</h3>
      {loading ? (
        <div className="space-y-3">
          {Array.from({ length: 3 }).map((_, i) => (
            <Skeleton key={i} className="h-6 w-full" />
          ))}
        </div>
      ) : (
        <div className="space-y-3">
          {items.map((item) => {
            const percent = total > 0 ? (item.value / total) * 100 : 0;
            return (
              <div key={item.label} className="space-y-1">
                <div className="flex justify-between text-sm">
                  <span className="capitalize">{item.label}</span>
                  <span className="font-medium">
                    {formatNumber(item.value)} ({formatPercent(percent)})
                  </span>
                </div>
                <div className="h-2 rounded-full bg-muted overflow-hidden">
                  <div
                    className={`h-full rounded-full ${item.color}`}
                    style={{ width: `${percent}%` }}
                  />
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}

// ==========================================
// Top Studies Table
// ==========================================

const programBadgeColors: Record<string, string> = {
  basic_pronunciation: "bg-chart-1/10 text-chart-1",
  basic_word: "bg-chart-2/10 text-chart-2",
  basic_900: "bg-chart-6/10 text-chart-6",
  topik_read: "bg-chart-4/10 text-chart-4",
  topik_listen: "bg-chart-5/10 text-chart-5",
  topik_write: "bg-chart-3/10 text-chart-3",
  tbc: "bg-muted text-muted-foreground",
};

function TopStudiesTable({
  items,
  loading,
  sortBy,
}: {
  items: TopStudyItem[];
  loading: boolean;
  sortBy: string;
}) {
  if (loading) {
    return (
      <div className="space-y-3">
        {Array.from({ length: 5 }).map((_, i) => (
          <div key={i} className="flex items-center gap-4 p-3 rounded-lg border">
            <Skeleton className="h-8 w-8 rounded-full" />
            <div className="flex-1">
              <Skeleton className="h-4 w-48" />
              <Skeleton className="h-3 w-24 mt-1" />
            </div>
            <Skeleton className="h-4 w-16" />
          </div>
        ))}
      </div>
    );
  }

  if (items.length === 0) {
    return (
      <div className="text-center text-muted-foreground py-8">
        No data available for the selected period
      </div>
    );
  }

  const getDisplayValue = (item: TopStudyItem): string => {
    switch (sortBy) {
      case "solves":
        return formatNumber(item.solve_count);
      case "solve_rate":
        return formatPercent(item.solve_rate);
      default:
        return formatNumber(item.attempt_count);
    }
  };

  const getDisplayLabel = (): string => {
    switch (sortBy) {
      case "solves":
        return "solves";
      case "solve_rate":
        return "solve rate";
      default:
        return "attempts";
    }
  };

  return (
    <div className="space-y-2">
      {items.map((item) => (
        <Link
          key={item.study_id}
          to={`/admin/studies/${item.study_id}`}
          className="flex items-center gap-4 p-3 rounded-lg border hover:bg-muted/50 transition-colors"
        >
          <div className="h-8 w-8 rounded-full bg-primary/10 flex items-center justify-center text-sm font-bold">
            {item.rank}
          </div>
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2">
              <p className="font-medium truncate">{item.study_title || item.study_idx}</p>
              <Badge
                variant="secondary"
                className={`${programBadgeColors[item.study_program] || ""} text-xs`}
              >
                {item.study_program.toUpperCase()}
              </Badge>
            </div>
            <p className="text-sm text-muted-foreground">
              {item.study_idx} - {item.task_count} tasks
            </p>
          </div>
          <div className="text-right">
            <p className="font-medium">{getDisplayValue(item)}</p>
            <p className="text-xs text-muted-foreground">{getDisplayLabel()}</p>
          </div>
          <ArrowRight className="h-4 w-4 text-muted-foreground" />
        </Link>
      ))}
    </div>
  );
}

// ==========================================
// Daily Stats Table
// ==========================================

function DailyStatsTable({
  items,
  loading,
}: {
  items: { date: string; attempts: number; solves: number; active_users: number }[];
  loading: boolean;
}) {
  if (loading) {
    return (
      <div className="space-y-2">
        {Array.from({ length: 7 }).map((_, i) => (
          <div key={i} className="flex items-center gap-4 p-2">
            <Skeleton className="h-4 w-24" />
            <Skeleton className="h-4 w-16" />
            <Skeleton className="h-4 w-16" />
            <Skeleton className="h-4 w-16" />
          </div>
        ))}
      </div>
    );
  }

  if (items.length === 0) {
    return (
      <div className="text-center text-muted-foreground py-8">
        No data available for the selected period
      </div>
    );
  }

  // 최근 날짜 순 정렬 (desc)
  const sorted = [...items].sort((a, b) => b.date.localeCompare(a.date));

  return (
    <div className="rounded-md border">
      <table className="w-full text-sm">
        <thead className="border-b bg-muted/50">
          <tr>
            <th className="h-10 px-4 text-left font-medium">Date</th>
            <th className="h-10 px-4 text-right font-medium">Attempts</th>
            <th className="h-10 px-4 text-right font-medium">Solves</th>
            <th className="h-10 px-4 text-right font-medium">Users</th>
            <th className="h-10 px-4 text-right font-medium">Rate</th>
          </tr>
        </thead>
        <tbody>
          {sorted.map((item) => {
            const rate =
              item.attempts > 0 ? ((item.solves / item.attempts) * 100).toFixed(1) : "0.0";
            return (
              <tr key={item.date} className="border-b hover:bg-muted/50">
                <td className="p-4">{item.date}</td>
                <td className="p-4 text-right">{formatNumber(item.attempts)}</td>
                <td className="p-4 text-right">{formatNumber(item.solves)}</td>
                <td className="p-4 text-right">{formatNumber(item.active_users)}</td>
                <td className="p-4 text-right">{rate}%</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}

// ==========================================
// Main Page
// ==========================================

export function AdminStudyStatsPage() {
  const defaultRange = useMemo(() => getDefaultDateRange(), []);
  const [fromDate, setFromDate] = useState(defaultRange.from);
  const [toDate, setToDate] = useState(defaultRange.to);
  const [topLimit, setTopLimit] = useState(10);
  const [topSortBy, setTopSortBy] = useState<"attempts" | "solves" | "solve_rate">("attempts");

  const statsQuery: StudyStatsQuery = { from: fromDate, to: toDate };
  const topQuery: TopStudiesQuery = { ...statsQuery, limit: topLimit, sort_by: topSortBy };

  const { data: summaryData, isLoading: summaryLoading } = useStudyStatsSummary(statsQuery);
  const { data: topData, isLoading: topLoading } = useStudyStatsTop(topQuery);
  const { data: dailyData, isLoading: dailyLoading } = useStudyStatsDaily(statsQuery);

  const handleQuickRange = (days: number) => {
    const today = new Date();
    const to = today.toISOString().split("T")[0];
    const from = new Date(today.setDate(today.getDate() - days)).toISOString().split("T")[0];
    setFromDate(from);
    setToDate(to);
  };

  const programItems = summaryData
    ? [
        { label: "Basic Pron.", value: summaryData.by_program.basic_pronunciation, color: "bg-chart-1" },
        { label: "Basic Word", value: summaryData.by_program.basic_word, color: "bg-chart-2" },
        { label: "Basic 900", value: summaryData.by_program.basic_900, color: "bg-chart-6" },
        { label: "TOPIK Read", value: summaryData.by_program.topik_read, color: "bg-chart-4" },
        { label: "TOPIK Listen", value: summaryData.by_program.topik_listen, color: "bg-chart-5" },
        { label: "TOPIK Write", value: summaryData.by_program.topik_write, color: "bg-chart-3" },
        { label: "TBC", value: summaryData.by_program.tbc, color: "bg-muted-foreground" },
      ]
    : [];

  const stateItems = summaryData
    ? [
        { label: "Ready", value: summaryData.by_state.ready, color: "bg-chart-4" },
        { label: "Open", value: summaryData.by_state.open, color: "bg-status-success" },
        { label: "Close", value: summaryData.by_state.close, color: "bg-muted-foreground" },
      ]
    : [];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Study Statistics</h1>
          <p className="text-muted-foreground">
            Overview of study performance and user engagement
          </p>
        </div>
        <Button variant="outline" asChild>
          <Link to="/admin/studies">
            <BookOpen className="mr-2 h-4 w-4" />
            Back to Studies
          </Link>
        </Button>
      </div>

      {/* Date Range Filter */}
      <div className="rounded-lg border bg-card p-4">
        <div className="flex flex-wrap items-end gap-4">
          <div className="space-y-2">
            <Label>From</Label>
            <Input
              type="date"
              value={fromDate}
              onChange={(e) => setFromDate(e.target.value)}
              className="w-40"
            />
          </div>
          <div className="space-y-2">
            <Label>To</Label>
            <Input
              type="date"
              value={toDate}
              onChange={(e) => setToDate(e.target.value)}
              className="w-40"
            />
          </div>
          <div className="flex gap-2">
            <Button variant="outline" size="sm" onClick={() => handleQuickRange(7)}>
              7 days
            </Button>
            <Button variant="outline" size="sm" onClick={() => handleQuickRange(30)}>
              30 days
            </Button>
            <Button variant="outline" size="sm" onClick={() => handleQuickRange(90)}>
              90 days
            </Button>
          </div>
        </div>
      </div>

      {/* Summary Cards */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <SummaryCard
          title="Total Studies"
          value={summaryData ? formatNumber(summaryData.total_studies) : 0}
          icon={BookOpen}
          loading={summaryLoading}
          subtitle={summaryData ? `${summaryData.open_studies} open` : undefined}
        />
        <SummaryCard
          title="Total Tasks"
          value={summaryData ? formatNumber(summaryData.total_tasks) : 0}
          icon={Target}
          loading={summaryLoading}
        />
        <SummaryCard
          title="Total Attempts"
          value={summaryData ? formatNumber(summaryData.total_attempts) : 0}
          icon={TrendingUp}
          loading={summaryLoading}
        />
        <SummaryCard
          title="Solve Rate"
          value={summaryData ? formatPercent(summaryData.solve_rate) : "0%"}
          icon={CheckCircle}
          loading={summaryLoading}
          subtitle={summaryData ? `${formatNumber(summaryData.total_solves)} solved` : undefined}
        />
      </div>

      {/* Distribution Cards */}
      <div className="grid gap-4 md:grid-cols-2">
        <DistributionCard
          title="By Program"
          items={programItems}
          loading={summaryLoading}
        />
        <DistributionCard
          title="By State"
          items={stateItems}
          loading={summaryLoading}
        />
      </div>

      {/* Top Studies & Daily Stats */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Top Studies */}
        <div className="rounded-lg border bg-card p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Top Studies</h2>
            <div className="flex gap-2">
              <Select
                value={topSortBy}
                onValueChange={(v) => setTopSortBy(v as "attempts" | "solves" | "solve_rate")}
              >
                <SelectTrigger className="w-32">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="attempts">By Attempts</SelectItem>
                  <SelectItem value="solves">By Solves</SelectItem>
                  <SelectItem value="solve_rate">By Rate</SelectItem>
                </SelectContent>
              </Select>
              <Select
                value={topLimit.toString()}
                onValueChange={(v) => setTopLimit(parseInt(v))}
              >
                <SelectTrigger className="w-20">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="5">Top 5</SelectItem>
                  <SelectItem value="10">Top 10</SelectItem>
                  <SelectItem value="20">Top 20</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
          <TopStudiesTable
            items={topData?.items || []}
            loading={topLoading}
            sortBy={topSortBy}
          />
        </div>

        {/* Daily Stats */}
        <div className="rounded-lg border bg-card p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Daily Statistics</h2>
            <div className="flex items-center gap-2 text-muted-foreground">
              <Users className="h-4 w-4" />
              <CalendarDays className="h-5 w-5" />
            </div>
          </div>
          <div className="max-h-[400px] overflow-y-auto">
            <DailyStatsTable
              items={dailyData?.items || []}
              loading={dailyLoading}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
