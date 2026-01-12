import { Route, Routes } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { HealthPage } from "@/category/health/page/health_page";

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
      </div>
    </div>
  );
}

export function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/health" element={<HealthPage />} />
    </Routes>
  );
}
