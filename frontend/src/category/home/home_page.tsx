import { Link, useNavigate } from "react-router-dom";
import { useAuthStore } from "@/hooks/use_auth_store";
import { Button } from "@/components/ui/button";
import { LogoutButton } from "@/category/auth/components/logout_button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function HomePage() {
  const navigate = useNavigate();
  const user = useAuthStore((state) => state.user);
  const isLoggedIn = !!user;

  // 페이지 이동 헬퍼 함수
  const go = (path: string) => navigate(path);

  return (
    <div className="flex min-h-screen flex-col items-center justify-center gap-8 p-4 bg-slate-50">
      {/* 1. 메인 헤더 및 로그인 상태 표시 */}
      <div className="flex flex-col items-center gap-4">
        <h1 className="text-4xl font-extrabold tracking-tight lg:text-5xl text-primary">
          Amazing Korean API
        </h1>
        
        {isLoggedIn ? (
          <div className="flex flex-col items-center gap-3">
            <p className="text-xl font-medium text-gray-700">
              👋 환영합니다,{" "}
              <span className="text-primary font-bold">
                {/* 타입 에러 방지를 위한 임시 처리 */}
                {(user as any).nickname || (user as any).name || `User ${user.user_id}`}
              </span>
              님!
            </p>
            <div className="flex flex-wrap gap-2">
              <Button asChild>
                <Link to="/videos">영상 학습하기</Link>
              </Button>
              <Button variant="secondary" asChild>
                <Link to="/studies">학습하기</Link>
              </Button>
              <Button variant="outline" onClick={() => go("/user/me")}>
                👤 마이 페이지
              </Button>
              {/* 👇 설정 버튼 추가됨 */}
              <Button variant="outline" onClick={() => go("/settings")}>
                ⚙️ 설정
              </Button>
              <LogoutButton />
            </div>
          </div>
        ) : (
          <div className="flex flex-col items-center gap-2">
            <p className="text-sm text-gray-500 mb-2">서비스를 이용하려면 로그인이 필요합니다.</p>
            <div className="flex flex-wrap gap-2">
              <Button size="lg" variant="secondary" asChild>
                <Link to="/videos">영상 학습하기</Link>
              </Button>
              <Button size="lg" variant="outline" asChild>
                <Link to="/studies">학습하기</Link>
              </Button>
            </div>
            <div className="flex gap-2">
                <Button size="lg" onClick={() => go("/login")}>
                로그인
                </Button>
                <Button size="lg" variant="outline" onClick={() => go("/signup")}>
                회원가입
                </Button>
            </div>
          </div>
        )}
      </div>

      {/* 🚧 구분선 🚧 */}
      <div className="w-full max-w-md border-t border-gray-300 my-4"></div>

      {/* 2. 개발자 전용 네비게이션 (나중에 삭제 예정) */}
      <Card className="w-full max-w-md bg-white shadow-lg border-dashed border-2 border-gray-300">
        <CardHeader className="pb-2">
          <CardTitle className="text-sm font-mono text-gray-500 flex justify-between">
            <span>🚧 DEV ROUTE MAP</span>
            <span className="text-xs text-red-400">배포 시 삭제</span>
          </CardTitle>
        </CardHeader>
        <CardContent className="grid grid-cols-2 gap-2">
          {/* Auth 관련 링크 */}
          <Button variant="ghost" className="justify-start h-auto py-2 px-3 text-sm" onClick={() => go("/login")}>
            🔑 로그인 (/login)
          </Button>
          <Button variant="ghost" className="justify-start h-auto py-2 px-3 text-sm" onClick={() => go("/signup")}>
            📝 회원가입 (/signup)
          </Button>
          <Button variant="ghost" className="justify-start h-auto py-2 px-3 text-sm" onClick={() => go("/find-id")}>
            🔍 아이디 찾기 (/find-id)
          </Button>
          {/* 테스트용 토큰 자동 포함 */}
          <Button 
            variant="ghost" 
            className="justify-start h-auto py-2 px-3 text-sm text-left" 
            onClick={() => go("/reset-password?token=DEV_TEST_TOKEN")}
          >
            🔐 비번 재설정<br/>(Testing Token)
          </Button>

          {/* User 관련 링크 (Phase 2 준비) */}
          <div className="col-span-2 border-t my-1"></div>
          
          <Button variant="secondary" className="justify-start text-sm" onClick={() => go("/user/me")}>
            👤 내 정보 (User Me)
          </Button>
          <Button variant="secondary" className="justify-start text-sm" onClick={() => go("/user/edit")}>
             📝 정보 수정 (Edit)
          </Button>
           {/* 👇 설정 버튼 추가됨 */}
          <Button variant="secondary" className="justify-start text-sm col-span-2" onClick={() => go("/settings")}>
             ⚙️ 설정 (Settings)
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
