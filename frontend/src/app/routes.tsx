import { Navigate, Route, Routes } from "react-router-dom";

import { RootLayout } from "@/components/layout/root_layout";
import HomePage from "@/category/home/home_page";
import { AboutPage } from "@/category/about/page/about_page";

import { HealthPage } from "@/category/health/page/health_page";
import { SignupPage } from "@/category/auth/page/signup_page";
import { LoginPage } from "@/category/auth/page/login_page";
import { FindIdPage } from "@/category/auth/page/find_id_page";
import { ResetPasswordPage } from "@/category/auth/page/reset_password_page";
import { MyPage } from "@/category/user/page/my_page";
import { EditProfilePage } from "@/category/user/page/edit_profile_page";
import { SettingsPage } from "@/category/user/page/settings_page";
import { VideoListPage } from "@/category/video/page/video_list_page";
import { VideoDetailPage } from "@/category/video/page/video_detail_page";
import { StudyListPage } from "@/category/study/page/study_list_page";
import { StudyDetailPage } from "@/category/study/page/study_detail_page";
import { StudyTaskPage } from "@/category/study/page/study_task_page";
import { LessonListPage } from "@/category/lesson/page/lesson_list_page";
import { LessonDetailPage } from "@/category/lesson/page/lesson_detail_page";
import PrivateRoute from "@/routes/private_route";

// Admin imports
import { AdminRoute } from "@/routes/admin_route";
import { AdminLayout } from "@/category/admin/page/admin_layout";
import { AdminDashboard } from "@/category/admin/page/admin_dashboard";
import { AdminUsersPage } from "@/category/admin/page/admin_users_page";
import { AdminUserDetail } from "@/category/admin/page/admin_user_detail";
import { AdminUserCreate } from "@/category/admin/page/admin_user_create";
import { AdminUserBulkCreate } from "@/category/admin/page/admin_user_bulk_create";
import { AdminVideosPage } from "@/category/admin/page/admin_videos_page";
import { AdminVideoDetail } from "@/category/admin/page/admin_video_detail";
import { AdminVideoCreate } from "@/category/admin/page/admin_video_create";
import { AdminVideoBulkCreate } from "@/category/admin/page/admin_video_bulk_create";
import { AdminVideoStatsPage } from "@/category/admin/page/admin_video_stats_page";
import { AdminStudiesPage } from "@/category/admin/page/admin_studies_page";
import { AdminLessonsPage } from "@/category/admin/page/admin_lessons_page";

export function AppRoutes() {
  return (
    <Routes>
      {/* RootLayout으로 모든 페이지 감싸기 (Header + Footer) */}
      <Route element={<RootLayout />}>
        {/* 누구나 접근 가능 (Public) */}
        <Route path="/" element={<HomePage />} />
        <Route path="/about" element={<AboutPage />} />
        <Route path="/intro" element={<Navigate to="/about" replace />} />
        <Route path="/find-id" element={<FindIdPage />} />
        <Route path="/login" element={<LoginPage />} />
        <Route path="/reset-password" element={<ResetPasswordPage />} />
        <Route path="/signup" element={<SignupPage />} />
        <Route path="/health" element={<HealthPage />} />
        <Route path="/videos" element={<VideoListPage />} />
        <Route path="/videos/:videoId" element={<VideoDetailPage />} />
        <Route path="/studies" element={<StudyListPage />} />
        <Route path="/studies/:studyId" element={<StudyDetailPage />} />
        <Route path="/studies/tasks/:taskId" element={<StudyTaskPage />} />
        <Route path="/lessons" element={<LessonListPage />} />
        <Route path="/lessons/:lessonId" element={<LessonDetailPage />} />

        {/* 로그인한 사람만 접근 가능 (Private) */}
        <Route element={<PrivateRoute />}>
          <Route path="/user/me" element={<MyPage />} />
          <Route path="/user/edit" element={<EditProfilePage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Route>
      </Route>

      {/* Admin 라우트 (admin/HYMN 권한 필요) */}
      <Route element={<AdminRoute />}>
        <Route path="/admin" element={<AdminLayout />}>
          <Route index element={<AdminDashboard />} />
          <Route path="users" element={<AdminUsersPage />} />
          <Route path="users/new" element={<AdminUserCreate />} />
          <Route path="users/bulk-create" element={<AdminUserBulkCreate />} />
          <Route path="users/:userId" element={<AdminUserDetail />} />
          <Route path="videos" element={<AdminVideosPage />} />
          <Route path="videos/stats" element={<AdminVideoStatsPage />} />
          <Route path="videos/new" element={<AdminVideoCreate />} />
          <Route path="videos/bulk-create" element={<AdminVideoBulkCreate />} />
          <Route path="videos/:videoId" element={<AdminVideoDetail />} />
          <Route path="studies" element={<AdminStudiesPage />} />
          <Route path="lessons" element={<AdminLessonsPage />} />
        </Route>
      </Route>
    </Routes>
  );
}
