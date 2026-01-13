import { Navigate, Outlet } from "react-router-dom";
import { useAuthStore } from "@/hooks/use_auth_store";

export default function PrivateRoute() {
  const user = useAuthStore((state) => state.user);

  // 유저 정보(토큰)가 없으면 로그인 페이지로 튕겨냄
  if (!user) {
    // replace: 뒤로 가기 눌러도 다시 못 오게 기록을 덮어씀
    return <Navigate to="/login" replace />;
  }

  // 있으면 자식 컴포넌트(Outlet) 보여줌
  return <Outlet />;
}