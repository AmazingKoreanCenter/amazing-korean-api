import { Link, Outlet, useLocation } from "react-router-dom";
import { Users, Video, BookOpen, GraduationCap, LayoutDashboard, Mail, Languages, CreditCard, BookText, Tablet, Copy } from "lucide-react";

import { ThemeToggle } from "@/components/ui/theme_toggle";
import { useAdminSingleTab } from "@/category/admin/hook/use_admin_single_tab";

const navItems = [
  { path: "/admin", label: "Dashboard", icon: LayoutDashboard, exact: true },
  { path: "/admin/users", label: "Users", icon: Users },
  { path: "/admin/videos", label: "Videos", icon: Video },
  { path: "/admin/studies", label: "Studies", icon: BookOpen },
  { path: "/admin/lessons", label: "Lessons", icon: GraduationCap },
  { path: "/admin/payment/subscriptions", label: "Payments", icon: CreditCard, prefix: "/admin/payment" },
  { path: "/admin/textbook/orders", label: "Textbook", icon: BookText, prefix: "/admin/textbook" },
  { path: "/admin/ebook/purchases", label: "E-book", icon: Tablet, prefix: "/admin/ebook" },
  { path: "/admin/translations", label: "Translations", icon: Languages },
  { path: "/admin/email", label: "Email", icon: Mail },
];

export function AdminLayout() {
  const location = useLocation();
  const blocked = useAdminSingleTab();

  // 관리자 단일 탭 강제: 우리 사이트가 이미 다른 탭에서 열려 있으면 이 새 탭만 차단.
  if (blocked) {
    return <AdminTabBlocked />;
  }

  const isActive = (path: string, exact?: boolean, prefix?: string) => {
    if (exact) {
      return location.pathname === path;
    }
    return location.pathname.startsWith(prefix ?? path);
  };

  return (
    <div className="min-h-screen flex bg-muted print:block print:bg-white print:min-h-0">
      {/* Sidebar — 인쇄/PDF 저장 시 숨김 */}
      <aside className="w-64 bg-card border-e border-border flex flex-col print:hidden">
        {/* Logo */}
        <div className="h-16 flex items-center px-6 border-b border-border">
          <Link to="/admin" className="text-xl font-bold text-foreground">
            Admin Panel
          </Link>
        </div>

        {/* Navigation */}
        <nav className="flex-1 p-4 space-y-1">
          {navItems.map((item) => {
            const Icon = item.icon;
            const active = isActive(item.path, item.exact, "prefix" in item ? item.prefix : undefined);
            return (
              <Link
                key={item.path}
                to={item.path}
                className={`flex items-center gap-3 px-4 py-2.5 rounded-lg text-sm font-medium transition-colors ${
                  active
                    ? "bg-primary/10 text-primary"
                    : "text-muted-foreground hover:bg-muted hover:text-foreground"
                }`}
              >
                <Icon className="w-5 h-5" />
                {item.label}
              </Link>
            );
          })}
        </nav>

        {/* Back to Site */}
        <div className="p-4 border-t border-border">
          <Link
            to="/"
            className="flex items-center gap-2 px-4 py-2 text-sm text-muted-foreground hover:text-foreground"
          >
            <span>&larr;</span>
            <span>Back to Site</span>
          </Link>
        </div>
      </aside>

      {/* Main Content */}
      <div className="flex-1 flex flex-col print:block">
        {/* Header — 인쇄/PDF 저장 시 숨김 */}
        <header className="h-16 bg-card border-b border-border flex items-center justify-between px-6 print:hidden">
          <h1 className="text-lg font-semibold text-foreground">
            Amazing Korean Admin
          </h1>
          <div className="flex items-center gap-4">
            <span className="text-sm text-muted-foreground">Admin</span>
            <ThemeToggle />
          </div>
        </header>

        {/* Page Content — 인쇄 시 패딩 제거, 전체 폭 사용 */}
        <main className="flex-1 p-6 overflow-auto print:p-0 print:overflow-visible">
          <Outlet />
        </main>
      </div>
    </div>
  );
}

/** 관리자 단일 탭 강제 — 2번째 탭에서만 표시되는 차단 화면 (기존 탭은 영향 없음). */
function AdminTabBlocked() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-muted p-6">
      <div className="max-w-md w-full bg-card border border-border rounded-xl p-8 text-center space-y-4">
        <div className="mx-auto w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center">
          <Copy className="w-6 h-6 text-primary" />
        </div>
        <h1 className="text-lg font-semibold text-foreground">
          이미 다른 탭에서 사용 중입니다
        </h1>
        <p className="text-sm text-muted-foreground">
          관리자 페이지는 한 번에 한 탭에서만 열 수 있습니다. 기존 탭에서 계속 작업하세요.
          그 탭을 닫았다면 아래에서 새로고침하면 이 탭에서 이어집니다.
        </p>
        <div className="flex flex-col gap-2 pt-2">
          <button
            type="button"
            onClick={() => window.location.reload()}
            className="w-full px-4 py-2.5 rounded-lg bg-primary text-primary-foreground text-sm font-medium hover:opacity-90 transition-opacity"
          >
            이 탭에서 새로고침
          </button>
          <Link
            to="/"
            className="w-full px-4 py-2.5 rounded-lg text-sm text-muted-foreground hover:text-foreground"
          >
            사이트로 돌아가기
          </Link>
        </div>
      </div>
    </div>
  );
}
