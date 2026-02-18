import { useMemo } from "react";
import { Users, Activity, Video, BookOpen } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import {
  useUserStatsSummary,
  useLoginStatsSummary,
} from "../hook/use_admin_users";
import { useVideoStatsSummary } from "../hook/use_admin_videos";
import { useStudyStatsSummary } from "../hook/use_admin_studies";

function formatDate(d: Date): string {
  return d.toISOString().slice(0, 10);
}

export function AdminDashboard() {
  const { t } = useTranslation();

  const dateRange = useMemo(() => {
    const to = new Date();
    const from = new Date();
    from.setDate(from.getDate() - 30);
    return { from: formatDate(from), to: formatDate(to) };
  }, []);

  const userStats = useUserStatsSummary(dateRange);
  const loginStats = useLoginStatsSummary(dateRange);
  const videoStats = useVideoStatsSummary(dateRange);
  const studyStats = useStudyStatsSummary(dateRange);

  const isLoading =
    userStats.isLoading ||
    loginStats.isLoading ||
    videoStats.isLoading ||
    studyStats.isLoading;

  const stats = [
    {
      label: t("admin.dashboard.totalUsers"),
      value: userStats.data?.total_users,
      icon: Users,
    },
    {
      label: t("admin.dashboard.activeSessions"),
      value: loginStats.data?.active_sessions,
      icon: Activity,
    },
    {
      label: t("admin.dashboard.totalVideos"),
      value: videoStats.data?.active_video_count,
      icon: Video,
    },
    {
      label: t("admin.dashboard.totalStudies"),
      value: studyStats.data?.total_studies,
      icon: BookOpen,
    },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">
          {t("admin.dashboard.title")}
        </h1>
        <p className="text-muted-foreground">
          {t("admin.dashboard.subtitle")}
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
              {isLoading ? (
                <Skeleton className="h-8 w-20" />
              ) : (
                <div className="text-2xl font-bold">
                  {stat.value?.toLocaleString() ?? "-"}
                </div>
              )}
            </CardContent>
          </Card>
        ))}
      </div>

      <Card>
        <CardHeader>
          <CardTitle>{t("admin.dashboard.quickActions")}</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-muted-foreground">
            {t("admin.dashboard.quickActionsDesc")}
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
