import { Link, Outlet, useLocation } from "react-router-dom";
import { Users, Video, BookOpen, GraduationCap, LayoutDashboard, Mail, Languages, CreditCard } from "lucide-react";

import { ThemeToggle } from "@/components/ui/theme_toggle";

const navItems = [
  { path: "/admin", label: "Dashboard", icon: LayoutDashboard, exact: true },
  { path: "/admin/users", label: "Users", icon: Users },
  { path: "/admin/videos", label: "Videos", icon: Video },
  { path: "/admin/studies", label: "Studies", icon: BookOpen },
  { path: "/admin/lessons", label: "Lessons", icon: GraduationCap },
  { path: "/admin/payment/subscriptions", label: "Payments", icon: CreditCard, prefix: "/admin/payment" },
  { path: "/admin/translations", label: "Translations", icon: Languages },
  { path: "/admin/email", label: "Email", icon: Mail },
];

export function AdminLayout() {
  const location = useLocation();

  const isActive = (path: string, exact?: boolean, prefix?: string) => {
    if (exact) {
      return location.pathname === path;
    }
    return location.pathname.startsWith(prefix ?? path);
  };

  return (
    <div className="min-h-screen flex bg-muted">
      {/* Sidebar */}
      <aside className="w-64 bg-card border-r border-border flex flex-col">
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
      <div className="flex-1 flex flex-col">
        {/* Header */}
        <header className="h-16 bg-card border-b border-border flex items-center justify-between px-6">
          <h1 className="text-lg font-semibold text-foreground">
            Amazing Korean Admin
          </h1>
          <div className="flex items-center gap-4">
            <span className="text-sm text-muted-foreground">Admin</span>
            <ThemeToggle />
          </div>
        </header>

        {/* Page Content */}
        <main className="flex-1 p-6 overflow-auto">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
