import { Link, NavLink } from "react-router-dom";
import { Menu, X, User, Globe } from "lucide-react";
import { useState, useEffect, useCallback } from "react";
import { useTranslation } from "react-i18next";

import { Button } from "@/components/ui/button";
import { useAuthStore } from "@/hooks/use_auth_store";
import { LogoutButton } from "@/category/auth/components/logout_button";
import { useUpdateSettings } from "@/category/user/hook/use_update_settings";
import { changeLanguage } from "@/i18n";
import { cn } from "@/lib/utils";

const NAV_ITEMS = [
  { labelKey: "nav.about", path: "/about" },
  { labelKey: "nav.videos", path: "/videos" },
  { labelKey: "nav.studies", path: "/studies" },
  { labelKey: "nav.lessons", path: "/lessons" },
] as const;

export function Header() {
  const { t, i18n } = useTranslation();
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const [scrolled, setScrolled] = useState(false);
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
  const updateSettings = useUpdateSettings();

  const closeMobileMenu = useCallback(() => {
    setMobileMenuOpen(false);
  }, []);

  const handleLanguageToggle = useCallback(() => {
    const newLang = i18n.language === "ko" ? "en" : "ko";
    changeLanguage(newLang);
    if (isLoggedIn) {
      updateSettings.mutate({ user_set_language: newLang });
    }
  }, [i18n.language, isLoggedIn, updateSettings]);

  // Handle scroll effect
  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 10);
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  return (
    <header
      className={cn(
        "sticky top-0 z-50 w-full transition-all duration-300",
        scrolled
          ? "bg-white/95 backdrop-blur-md shadow-sm border-b"
          : "bg-white border-b border-transparent"
      )}
    >
      <div className="max-w-[1350px] mx-auto flex h-[72px] items-center justify-between px-6 lg:px-8">
        {/* Logo */}
        <Link to="/" className="flex items-center gap-3 group">
          <div className="w-10 h-10 rounded-xl gradient-primary flex items-center justify-center shadow-md group-hover:shadow-lg transition-shadow">
            <span className="text-white font-bold text-lg">A</span>
          </div>
          <span className="text-xl font-bold text-primary hidden sm:block">
            Amazing Korean
          </span>
        </Link>

        {/* Desktop Navigation */}
        <nav className="hidden lg:flex items-center gap-1">
          {NAV_ITEMS.map((item) => (
            <NavLink
              key={item.path}
              to={item.path}
              className={({ isActive }) =>
                cn(
                  "px-5 py-2.5 text-[15px] font-medium rounded-lg transition-all duration-200",
                  isActive
                    ? "text-primary bg-primary/5"
                    : "text-muted-foreground hover:text-primary hover:bg-muted/50"
                )
              }
            >
              {t(item.labelKey)}
            </NavLink>
          ))}
        </nav>

        {/* Desktop Auth Buttons */}
        <div className="hidden lg:flex items-center gap-3">
          {/* Language Switcher */}
          <button
            type="button"
            onClick={handleLanguageToggle}
            className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium text-muted-foreground hover:text-primary rounded-lg hover:bg-muted/50 transition-colors"
          >
            <Globe className="h-4 w-4" />
            {i18n.language === "ko" ? "EN" : "KO"}
          </button>

          {isLoggedIn ? (
            <>
              <Button
                variant="ghost"
                size="sm"
                asChild
                className="gap-2 text-muted-foreground hover:text-primary"
              >
                <Link to="/user/me">
                  <User className="h-4 w-4" />
                  {t("nav.myPage")}
                </Link>
              </Button>
              <LogoutButton />
            </>
          ) : (
            <>
              <Button
                variant="ghost"
                size="sm"
                asChild
                className="text-muted-foreground hover:text-primary"
              >
                <Link to="/login">{t("nav.login")}</Link>
              </Button>
              <Button
                size="sm"
                asChild
                className="gradient-primary hover:opacity-90 text-white shadow-md hover:shadow-lg transition-all rounded-full px-6"
              >
                <Link to="/signup">{t("nav.signup")}</Link>
              </Button>
            </>
          )}
        </div>

        {/* Mobile Menu Button */}
        <button
          type="button"
          className="lg:hidden p-2 rounded-lg hover:bg-muted/50 transition-colors"
          onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
          aria-label="Toggle menu"
        >
          {mobileMenuOpen ? (
            <X className="h-6 w-6 text-foreground" />
          ) : (
            <Menu className="h-6 w-6 text-foreground" />
          )}
        </button>
      </div>

      {/* Mobile Menu */}
      <div
        className={cn(
          "lg:hidden overflow-hidden transition-all duration-300 ease-in-out",
          mobileMenuOpen ? "max-h-[400px] border-t" : "max-h-0"
        )}
      >
        <nav className="max-w-[1350px] mx-auto px-6 py-4 flex flex-col gap-1 bg-white">
          {NAV_ITEMS.map((item) => (
            <NavLink
              key={item.path}
              to={item.path}
              onClick={closeMobileMenu}
              className={({ isActive }) =>
                cn(
                  "px-4 py-3 text-[15px] font-medium rounded-lg transition-colors",
                  isActive
                    ? "text-primary bg-primary/5"
                    : "text-muted-foreground hover:text-primary hover:bg-muted/50"
                )
              }
            >
              {t(item.labelKey)}
            </NavLink>
          ))}

          {/* Mobile Language Switcher */}
          <button
            type="button"
            onClick={handleLanguageToggle}
            className="flex items-center gap-2 px-4 py-3 text-[15px] font-medium text-muted-foreground hover:text-primary hover:bg-muted/50 rounded-lg transition-colors"
          >
            <Globe className="h-4 w-4" />
            {i18n.language === "ko" ? "English" : "한국어"}
          </button>

          <div className="border-t my-3" />

          <div className="flex flex-col gap-2 px-1">
            {isLoggedIn ? (
              <>
                <Button variant="ghost" asChild className="justify-start gap-2" onClick={closeMobileMenu}>
                  <Link to="/user/me">
                    <User className="h-4 w-4" />
                    {t("nav.myPage")}
                  </Link>
                </Button>
                <LogoutButton />
              </>
            ) : (
              <>
                <Button variant="ghost" asChild className="justify-start" onClick={closeMobileMenu}>
                  <Link to="/login">{t("nav.login")}</Link>
                </Button>
                <Button
                  asChild
                  className="gradient-primary text-white rounded-full"
                  onClick={closeMobileMenu}
                >
                  <Link to="/signup">{t("nav.signup")}</Link>
                </Button>
              </>
            )}
          </div>
        </nav>
      </div>
    </header>
  );
}
