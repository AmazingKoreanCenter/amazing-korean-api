import { Route, Routes } from "react-router-dom";
import HomePage from "@/category/home/home_page";

import { HealthPage } from "@/category/health/page/health_page";
import { SignupPage } from "@/category/auth/page/signup_page";
import { LoginPage } from "@/category/auth/page/login_page";
import { FindIdPage } from "@/category/auth/page/find_id_page";
import { ResetPasswordPage } from "@/category/auth/page/reset_password_page";

export function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/find-id" element={<FindIdPage />} />
      <Route path="/login" element={<LoginPage />} />
      <Route path="/reset-password" element={<ResetPasswordPage />} />
      <Route path="/signup" element={<SignupPage />} />
      <Route path="/health" element={<HealthPage />} />
    </Routes>
  );
}