import { useState, useMemo } from "react";
import { Link } from "react-router-dom";
import {
  CalendarDays,
  Users,
  LogIn,
  CheckCircle,
  XCircle,
  Monitor,
  Smartphone,
  Tablet,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Skeleton } from "@/components/ui/skeleton";
import { Progress } from "@/components/ui/progress";
import {
  useLoginStatsSummary,
  useLoginStatsDaily,
  useLoginStatsDevices,
} from "../hook/use_admin_users";
import type { StatsQuery } from "../types";

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
  variant = "default",
}: {
  title: string;
  value: number | string;
  icon: React.ElementType;
  loading?: boolean;
  variant?: "default" | "success" | "destructive";
}) {
  const variantClasses = {
    default: "bg-primary/10 text-primary",
    success: "bg-green-500/10 text-green-600",
    destructive: "bg-red-500/10 text-red-600",
  };

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
        <div className={`h-12 w-12 rounded-full flex items-center justify-center ${variantClasses[variant]}`}>
          <Icon className="h-6 w-6" />
        </div>
      </div>
    </div>
  );
}

// ==========================================
// Device Stats
// ==========================================

function DeviceIcon({ device }: { device: string }) {
  switch (device.toLowerCase()) {
    case "desktop":
      return <Monitor className="h-5 w-5" />;
    case "mobile":
      return <Smartphone className="h-5 w-5" />;
    case "tablet":
      return <Tablet className="h-5 w-5" />;
    default:
      return <Monitor className="h-5 w-5" />;
  }
}

function DeviceStats({
  items,
  loading,
}: {
  items: { device: string; count: number; percentage: number }[];
  loading: boolean;
}) {
  if (loading) {
    return (
      <div className="space-y-4">
        {Array.from({ length: 3 }).map((_, i) => (
          <div key={i} className="space-y-2">
            <div className="flex justify-between">
              <Skeleton className="h-4 w-20" />
              <Skeleton className="h-4 w-16" />
            </div>
            <Skeleton className="h-2 w-full" />
          </div>
        ))}
      </div>
    );
  }

  if (items.length === 0) {
    return (
      <div className="text-center text-muted-foreground py-8">
        No device data available
      </div>
    );
  }

  const deviceColors: Record<string, string> = {
    desktop: "bg-blue-500",
    mobile: "bg-green-500",
    tablet: "bg-purple-500",
  };

  return (
    <div className="space-y-4">
      {items.map((item) => (
        <div key={item.device} className="space-y-2">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <DeviceIcon device={item.device} />
              <span className="font-medium capitalize">{item.device}</span>
            </div>
            <div className="text-right">
              <span className="font-medium">{formatNumber(item.count)}</span>
              <span className="text-muted-foreground ml-2">
                ({item.percentage.toFixed(1)}%)
              </span>
            </div>
          </div>
          <Progress
            value={item.percentage}
            className={`h-2 ${deviceColors[item.device.toLowerCase()] || "bg-gray-500"}`}
          />
        </div>
      ))}
    </div>
  );
}

// ==========================================
// Daily Login Table
// ==========================================

function DailyLoginTable({
  items,
  loading,
}: {
  items: { date: string; success: number; fail: number; unique_users: number }[];
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
            <th className="h-10 px-4 text-right font-medium">Success</th>
            <th className="h-10 px-4 text-right font-medium">Failed</th>
            <th className="h-10 px-4 text-right font-medium">Unique Users</th>
            <th className="h-10 px-4 text-right font-medium">Success Rate</th>
          </tr>
        </thead>
        <tbody>
          {sorted.map((item) => {
            const total = item.success + item.fail;
            const successRate = total > 0 ? ((item.success / total) * 100).toFixed(1) : "0.0";
            return (
              <tr key={item.date} className="border-b hover:bg-muted/50">
                <td className="p-4">{item.date}</td>
                <td className="p-4 text-right text-green-600 font-medium">
                  {formatNumber(item.success)}
                </td>
                <td className="p-4 text-right text-red-600">
                  {formatNumber(item.fail)}
                </td>
                <td className="p-4 text-right">{formatNumber(item.unique_users)}</td>
                <td className="p-4 text-right">{successRate}%</td>
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

export function AdminLoginStatsPage() {
  const defaultRange = useMemo(() => getDefaultDateRange(), []);
  const [fromDate, setFromDate] = useState(defaultRange.from);
  const [toDate, setToDate] = useState(defaultRange.to);

  const statsQuery: StatsQuery = { from: fromDate, to: toDate };

  const { data: summaryData, isLoading: summaryLoading } = useLoginStatsSummary(statsQuery);
  const { data: dailyData, isLoading: dailyLoading } = useLoginStatsDaily(statsQuery);
  const { data: devicesData, isLoading: devicesLoading } = useLoginStatsDevices(statsQuery);

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
          <h1 className="text-2xl font-bold">Login Statistics</h1>
          <p className="text-muted-foreground">
            Overview of login activity and authentication metrics
          </p>
        </div>
        <Button variant="outline" asChild>
          <Link to="/admin/users">
            <Users className="mr-2 h-4 w-4" />
            Back to Users
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
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-5">
        <SummaryCard
          title="Total Logins"
          value={summaryData ? formatNumber(summaryData.total_logins) : 0}
          icon={LogIn}
          loading={summaryLoading}
        />
        <SummaryCard
          title="Successful"
          value={summaryData ? formatNumber(summaryData.success_count) : 0}
          icon={CheckCircle}
          loading={summaryLoading}
          variant="success"
        />
        <SummaryCard
          title="Failed"
          value={summaryData ? formatNumber(summaryData.fail_count) : 0}
          icon={XCircle}
          loading={summaryLoading}
          variant="destructive"
        />
        <SummaryCard
          title="Unique Users"
          value={summaryData ? formatNumber(summaryData.unique_users) : 0}
          icon={Users}
          loading={summaryLoading}
        />
        <SummaryCard
          title="Active Sessions"
          value={summaryData ? formatNumber(summaryData.active_sessions) : 0}
          icon={Monitor}
          loading={summaryLoading}
        />
      </div>

      {/* Device Stats & Daily Logins */}
      <div className="grid gap-6 lg:grid-cols-3">
        {/* Device Stats */}
        <div className="rounded-lg border bg-card p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Login by Device</h2>
          </div>
          <p className="text-sm text-muted-foreground mb-4">
            Device distribution for successful logins
          </p>
          <DeviceStats
            items={devicesData?.items || []}
            loading={devicesLoading}
          />
        </div>

        {/* Daily Logins */}
        <div className="rounded-lg border bg-card p-6 lg:col-span-2">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Daily Logins</h2>
            <CalendarDays className="h-5 w-5 text-muted-foreground" />
          </div>
          <div className="max-h-[400px] overflow-y-auto">
            <DailyLoginTable
              items={dailyData?.items || []}
              loading={dailyLoading}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
