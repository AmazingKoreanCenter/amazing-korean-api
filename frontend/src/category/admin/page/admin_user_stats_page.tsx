import { useState, useMemo } from "react";
import { Link } from "react-router-dom";
import { CalendarDays, Users, UserPlus, UserCheck, UserX } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import {
  useUserStatsSummary,
  useUserStatsSignups,
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
  variant?: "default" | "success" | "warning" | "destructive";
}) {
  const variantClasses = {
    default: "bg-primary/10 text-primary",
    success: "bg-status-success/10 text-status-success",
    warning: "bg-status-warning/10 text-status-warning",
    destructive: "bg-destructive/10 text-destructive",
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
// Role Distribution
// ==========================================

function RoleDistribution({
  data,
  loading,
}: {
  data?: { hymn: number; admin: number; manager: number; learner: number };
  loading: boolean;
}) {
  if (loading) {
    return (
      <div className="flex gap-2">
        {Array.from({ length: 4 }).map((_, i) => (
          <Skeleton key={i} className="h-6 w-20" />
        ))}
      </div>
    );
  }

  if (!data) return null;

  const total = data.hymn + data.admin + data.manager + data.learner;

  const roleTypes = [
    { key: "hymn", label: "HYMN", value: data.hymn, color: "bg-chart-6" },
    { key: "admin", label: "Admin", value: data.admin, color: "bg-destructive" },
    { key: "manager", label: "Manager", value: data.manager, color: "bg-chart-3" },
    { key: "learner", label: "Learner", value: data.learner, color: "bg-chart-2" },
  ];

  return (
    <div className="space-y-3">
      <div className="flex gap-2 flex-wrap">
        {roleTypes.map((role) => (
          <Badge key={role.key} variant="outline" className="gap-1">
            <span className={`h-2 w-2 rounded-full ${role.color}`} />
            {role.label}: {formatNumber(role.value)}
            {total > 0 && (
              <span className="text-muted-foreground">
                ({((role.value / total) * 100).toFixed(1)}%)
              </span>
            )}
          </Badge>
        ))}
      </div>
    </div>
  );
}

// ==========================================
// Daily Signups Table
// ==========================================

function DailySignupsTable({
  items,
  loading,
}: {
  items: {
    date: string;
    signups: number;
    by_role: { hymn: number; admin: number; manager: number; learner: number };
  }[];
  loading: boolean;
}) {
  if (loading) {
    return (
      <div className="space-y-2">
        {Array.from({ length: 7 }).map((_, i) => (
          <div key={i} className="flex items-center gap-4 p-2">
            <Skeleton className="h-4 w-24" />
            <Skeleton className="h-4 w-16" />
            <Skeleton className="h-4 w-12" />
            <Skeleton className="h-4 w-12" />
            <Skeleton className="h-4 w-12" />
            <Skeleton className="h-4 w-12" />
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
            <th className="h-10 px-4 text-right font-medium">Signups</th>
            <th className="h-10 px-4 text-right font-medium">HYMN</th>
            <th className="h-10 px-4 text-right font-medium">Admin</th>
            <th className="h-10 px-4 text-right font-medium">Manager</th>
            <th className="h-10 px-4 text-right font-medium">Learner</th>
          </tr>
        </thead>
        <tbody>
          {sorted.map((item) => (
            <tr key={item.date} className="border-b hover:bg-muted/50">
              <td className="p-4">{item.date}</td>
              <td className="p-4 text-right font-medium">{formatNumber(item.signups)}</td>
              <td className="p-4 text-right">{formatNumber(item.by_role.hymn)}</td>
              <td className="p-4 text-right">{formatNumber(item.by_role.admin)}</td>
              <td className="p-4 text-right">{formatNumber(item.by_role.manager)}</td>
              <td className="p-4 text-right">{formatNumber(item.by_role.learner)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

// ==========================================
// Main Page
// ==========================================

export function AdminUserStatsPage() {
  const defaultRange = useMemo(() => getDefaultDateRange(), []);
  const [fromDate, setFromDate] = useState(defaultRange.from);
  const [toDate, setToDate] = useState(defaultRange.to);

  const statsQuery: StatsQuery = { from: fromDate, to: toDate };

  const { data: summaryData, isLoading: summaryLoading } = useUserStatsSummary(statsQuery);
  const { data: signupsData, isLoading: signupsLoading } = useUserStatsSignups(statsQuery);

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
          <h1 className="text-2xl font-bold">User Statistics</h1>
          <p className="text-muted-foreground">
            Overview of user registrations and activity
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
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <SummaryCard
          title="Total Users"
          value={summaryData ? formatNumber(summaryData.total_users) : 0}
          icon={Users}
          loading={summaryLoading}
        />
        <SummaryCard
          title="New Users"
          value={summaryData ? formatNumber(summaryData.new_users) : 0}
          icon={UserPlus}
          loading={summaryLoading}
          variant="success"
        />
        <SummaryCard
          title="Active Users"
          value={summaryData ? formatNumber(summaryData.active_users) : 0}
          icon={UserCheck}
          loading={summaryLoading}
          variant="success"
        />
        <SummaryCard
          title="Inactive Users"
          value={summaryData ? formatNumber(summaryData.inactive_users) : 0}
          icon={UserX}
          loading={summaryLoading}
          variant="warning"
        />
      </div>

      {/* Auth Distribution & Daily Signups */}
      <div className="grid gap-6 lg:grid-cols-3">
        {/* Role Distribution */}
        <div className="rounded-lg border bg-card p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">New Users by Role</h2>
          </div>
          <p className="text-sm text-muted-foreground mb-4">
            Role distribution for new users in the selected period
          </p>
          <RoleDistribution
            data={summaryData?.by_role}
            loading={summaryLoading}
          />
        </div>

        {/* Daily Signups */}
        <div className="rounded-lg border bg-card p-6 lg:col-span-2">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Daily Signups</h2>
            <CalendarDays className="h-5 w-5 text-muted-foreground" />
          </div>
          <div className="max-h-[400px] overflow-y-auto">
            <DailySignupsTable
              items={signupsData?.items || []}
              loading={signupsLoading}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
