import { Users, Video, BookOpen, GraduationCap } from "lucide-react";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

/**
 * Admin Dashboard - 관리자 대시보드
 * TODO: 실제 통계 API 연동 (Step 4)
 */
export function AdminDashboard() {
  // 임시 통계 데이터
  const stats = [
    { label: "Total Users", value: "-", icon: Users },
    { label: "Total Videos", value: "-", icon: Video },
    { label: "Total Studies", value: "-", icon: BookOpen },
    { label: "Total Lessons", value: "-", icon: GraduationCap },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">Dashboard</h1>
        <p className="text-muted-foreground">
          Welcome to Amazing Korean Admin Panel
        </p>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        {stats.map((stat) => (
          <Card key={stat.label}>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">
                {stat.label}
              </CardTitle>
              <stat.icon className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stat.value}</div>
            </CardContent>
          </Card>
        ))}
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Quick Actions</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-muted-foreground">
            Select a menu item from the sidebar to manage content.
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
