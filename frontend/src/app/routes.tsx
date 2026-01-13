import { Route, Routes } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { HealthPage } from "@/category/health/page/health_page";
import { SignupPage } from "@/category/auth/page/signup_page";
import { LoginPage } from "@/category/auth/page/login_page";
import { LogoutButton } from "@/category/auth/components/logout_button";
import { FindIdPage } from "@/category/auth/page/find_id_page";

function HomePage() {
  return (
    <div className="flex h-screen w-full flex-col items-center justify-center gap-4 bg-background">
      <h1 className="text-4xl font-bold text-primary tracking-tight">
        Amazing Korean
      </h1>
      <p className="text-lg text-muted-foreground">
        프론트엔드 환경 설정이 완료되었습니다! 🚀
      </p>
      <div className="flex gap-2">
        <Button variant="default">Button Test</Button>
        <Button variant="secondary">Shadcn UI</Button>
        <Button variant="destructive">Tailwind CSS</Button>
        <LogoutButton />
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
      <Route path="/signup" element={<SignupPage />} />
      <Route path="/health" element={<HealthPage />} />
    </Routes>
  );
}
