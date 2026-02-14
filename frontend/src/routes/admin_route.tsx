import { Navigate, Outlet, useLocation } from "react-router-dom";

import { useAuthStore } from "@/hooks/use_auth_store";
import { useUserMe } from "@/category/user/hook/use_user_me";

/**
 * Admin 전용 라우트 가드
 * - 로그인 여부 확인 (auth store)
 * - user_auth가 "admin" 또는 "HYMN"인지 확인 (API)
 * - MFA 미설정 시 MFA 설정 페이지로 강제 이동
 * - 권한 없으면 홈으로 리다이렉트
 */
export function AdminRoute() {
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
  const { data: user, isLoading, isError } = useUserMe();
  const location = useLocation();

  // 로그인 안 되어 있으면 로그인 페이지로
  if (!isLoggedIn) {
    return <Navigate to="/login" replace />;
  }

  // 유저 정보 로딩 중
  if (isLoading) {
    return (
      <div className="flex min-h-screen items-center justify-center">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    );
  }

  // API 에러 또는 유저 정보 없음
  if (isError || !user) {
    return <Navigate to="/login" replace />;
  }

  // 권한 확인: admin 또는 HYMN만 허용
  const allowedRoles = ["admin", "HYMN"];
  if (!allowedRoles.includes(user.user_auth)) {
    // 권한 없으면 403 페이지로
    return <Navigate to="/403" replace />;
  }

  // MFA 미설정 시 MFA 설정 페이지로 강제 이동
  // (MFA 설정 페이지 자체는 예외 — 무한 리다이렉트 방지)
  if (!user.mfa_enabled && location.pathname !== "/admin/mfa/setup") {
    return <Navigate to="/admin/mfa/setup" replace />;
  }

  // 권한 있으면 자식 라우트 렌더링
  return <Outlet />;
}
