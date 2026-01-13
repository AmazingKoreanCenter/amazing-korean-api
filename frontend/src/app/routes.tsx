import { Route, Routes } from "react-router-dom";
import { useNavigate } from "react-router-dom";
import { useAuthStore } from "@/hooks/use_auth_store";

import { Button } from "@/components/ui/button";
import { HealthPage } from "@/category/health/page/health_page";
import { SignupPage } from "@/category/auth/page/signup_page";
import { LoginPage } from "@/category/auth/page/login_page";
import { LogoutButton } from "@/category/auth/components/logout_button";
import { FindIdPage } from "@/category/auth/page/find_id_page";
import { ResetPasswordPage } from "@/category/auth/page/reset_password_page";

function HomePage() {
  const navigate = useNavigate();
  
  // 1. 스토어에서 유저 정보 가져오기
  const user = useAuthStore((state) => state.user);
  
  // 2. 로그인 여부 판단 (user가 있으면 true)
  const isLoggedIn = !!user;

  return (
    <div className="flex h-screen w-full flex-col items-center justify-center gap-4 bg-background">
      <h1 className="text-4xl font-bold text-primary tracking-tight">
        Amazing Korean
      </h1>
      <p className="text-lg text-muted-foreground">
      🚀 💥 프론트엔드 열심히 작업중 💥 🚀
      </p>
      <div className="flex gap-2">
        <Button variant="default">Button Test</Button>
        <Button variant="secondary">Shadcn UI</Button>
        <Button variant="destructive">Tailwind CSS</Button>
        {/* 3. 조건부 렌더링 (Toggle Logic) */}
        {isLoggedIn ? (
          <div className="flex flex-col items-center gap-3">
            <p className="text-lg font-medium text-gray-700">
              👋 환영합니다, <span className="text-primary font-bold">{user.user_id || user.user_id}</span>님!
            </p>
            {/* 로그인 상태일 때만 보임 */}
            <LogoutButton />
          </div>
        ) : (
          <div className="flex flex-col items-center gap-2">
            <p className="text-sm text-gray-500">로그인이 필요합니다.</p>
            {/* 비로그인 상태일 때만 보임 */}
            <Button onClick={() => navigate("/login")}>
              로그인 하러 가기
            </Button>
          </div>
        )}
      </div>
    </div>
  );
}

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
