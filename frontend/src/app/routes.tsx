import { Route, Routes } from "react-router-dom";
import HomePage from "@/category/home/home_page";

import { HealthPage } from "@/category/health/page/health_page";
import { SignupPage } from "@/category/auth/page/signup_page";
import { LoginPage } from "@/category/auth/page/login_page";
import { FindIdPage } from "@/category/auth/page/find_id_page";
import { ResetPasswordPage } from "@/category/auth/page/reset_password_page";
import { MyPage } from "@/category/user/page/my_page";
import { EditProfilePage } from "@/category/user/page/edit_profile_page";
import { SettingsPage } from "@/category/user/page/settings_page";
import PrivateRoute from "@/routes/private_route";

export function AppRoutes() {
  return (
    <Routes>
      {/* 누구나 접근 가능 (Public) */}
      <Route path="/" element={<HomePage />} />
      <Route path="/find-id" element={<FindIdPage />} />
      <Route path="/login" element={<LoginPage />} />
      <Route path="/reset-password" element={<ResetPasswordPage />} />
      <Route path="/signup" element={<SignupPage />} />
      <Route path="/health" element={<HealthPage />} />

      {/* 🔒 로그인한 사람만 접근 가능 (Private) */}
      <Route element={<PrivateRoute />}>
        {/* 이 안에 있는 모든 Route는 보호받습니다 */}
        <Route path="/user/me" element={<MyPage />} />
        <Route path="/user/edit" element={<EditProfilePage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Route>
    </Routes>
  );
}
