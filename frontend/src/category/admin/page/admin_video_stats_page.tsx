import { useState, useMemo } from "react";
import { Link } from "react-router-dom";
import { CalendarDays, TrendingUp, Play, CheckCircle, Video, ArrowRight } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  useVideoStatsSummary,
  useVideoStatsTop,
  useVideoStatsDaily,
} from "../hook/use_admin_videos";
import type { StatsQuery, TopVideosQuery, TopVideoItem } from "../types";

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

// ==========================================
// Summary Cards
// ==========================================

function SummaryCard({
  title,
  value,
  icon: Icon,
  loading,
}: {
  title: string;
  value: number | string;
  icon: React.ElementType;
  loading?: boolean;
}) {
  return (
    <div className="rounded-lg border bg-card p-6">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm font-medium text-muted-foreground">{title}</p>
          {loading ? (
            <Skeleton className="h-8 w-24 mt-2" />
          ) : (
            <p className="text-2xl font-bold mt-2">{value}</p>
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
// Top Videos Table
// ==========================================

function TopVideosTable({
  items,
  loading,
  sortBy,
}: {
  items: TopVideoItem[];
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

  return (
    <div className="space-y-2">
      {items.map((item) => (
        <Link
          key={item.video_id}
          to={`/admin/videos/${item.video_id}`}
          className="flex items-center gap-4 p-3 rounded-lg border hover:bg-muted/50 transition-colors"
        >
          <div className="h-8 w-8 rounded-full bg-primary/10 flex items-center justify-center text-sm font-bold">
            {item.rank}
          </div>
          <div className="flex-1 min-w-0">
            <p className="font-medium truncate">{item.title || item.video_idx}</p>
            <p className="text-sm text-muted-foreground">{item.video_idx}</p>
          </div>
          <div className="text-right">
            <p className="font-medium">
              {formatNumber(sortBy === "completes" ? item.completes : item.views)}
            </p>
            <p className="text-xs text-muted-foreground">
              {sortBy === "completes" ? "completes" : "views"}
            </p>
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
  items: { date: string; views: number; completes: number }[];
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
            <th className="h-10 px-4 text-right font-medium">Views</th>
            <th className="h-10 px-4 text-right font-medium">Completes</th>
            <th className="h-10 px-4 text-right font-medium">Rate</th>
          </tr>
        </thead>
        <tbody>
          {sorted.map((item) => {
            const rate = item.views > 0 ? ((item.completes / item.views) * 100).toFixed(1) : "0.0";
            return (
              <tr key={item.date} className="border-b hover:bg-muted/50">
                <td className="p-4">{item.date}</td>
                <td className="p-4 text-right">{formatNumber(item.views)}</td>
                <td className="p-4 text-right">{formatNumber(item.completes)}</td>
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

export function AdminVideoStatsPage() {
  const defaultRange = useMemo(() => getDefaultDateRange(), []);
  const [fromDate, setFromDate] = useState(defaultRange.from);
  const [toDate, setToDate] = useState(defaultRange.to);
  const [topLimit, setTopLimit] = useState(10);
  const [topSortBy, setTopSortBy] = useState<"views" | "completes">("views");

  const statsQuery: StatsQuery = { from: fromDate, to: toDate };
  const topQuery: TopVideosQuery = { ...statsQuery, limit: topLimit, sort_by: topSortBy };

  const { data: summaryData, isLoading: summaryLoading } = useVideoStatsSummary(statsQuery);
  const { data: topData, isLoading: topLoading } = useVideoStatsTop(topQuery);
  const { data: dailyData, isLoading: dailyLoading } = useVideoStatsDaily(statsQuery);

  const handleQuickRange = (days: number) => {
    const today = new Date();
    const to = today.toISOString().split("T")[0];
    const from = new Date(today.setDate(today.getDate() - days)).toISOString().split("T")[0];
    setFromDate(from);
    setToDate(to);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Video Statistics</h1>
          <p className="text-muted-foreground">
            Overview of video performance and engagement
          </p>
        </div>
        <Button variant="outline" asChild>
          <Link to="/admin/videos">
            <Video className="mr-2 h-4 w-4" />
            Back to Videos
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
      <div className="grid gap-4 md:grid-cols-3">
        <SummaryCard
          title="Total Views"
          value={summaryData ? formatNumber(summaryData.total_views) : 0}
          icon={Play}
          loading={summaryLoading}
        />
        <SummaryCard
          title="Total Completes"
          value={summaryData ? formatNumber(summaryData.total_completes) : 0}
          icon={CheckCircle}
          loading={summaryLoading}
        />
        <SummaryCard
          title="Active Videos"
          value={summaryData ? formatNumber(summaryData.active_video_count) : 0}
          icon={TrendingUp}
          loading={summaryLoading}
        />
      </div>

      {/* Top Videos & Daily Stats */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Top Videos */}
        <div className="rounded-lg border bg-card p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Top Videos</h2>
            <div className="flex gap-2">
              <Select
                value={topSortBy}
                onValueChange={(v) => setTopSortBy(v as "views" | "completes")}
              >
                <SelectTrigger className="w-32">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="views">By Views</SelectItem>
                  <SelectItem value="completes">By Completes</SelectItem>
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
          <TopVideosTable
            items={topData?.items || []}
            loading={topLoading}
            sortBy={topSortBy}
          />
        </div>

        {/* Daily Stats */}
        <div className="rounded-lg border bg-card p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Daily Statistics</h2>
            <CalendarDays className="h-5 w-5 text-muted-foreground" />
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
