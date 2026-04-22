import { useEffect, useRef, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { X, Search, Loader2 } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { getAdminUsers } from "@/category/admin/admin_api";
import type { AdminUserSummary } from "@/category/admin/types";

type Props = {
  /** 선택된 사용자 (null 이면 미선택 상태) */
  value: AdminUserSummary | null;
  /** 선택 변경 콜백 (null 이면 해제) */
  onChange: (user: AdminUserSummary | null) => void;
  /** 입력 placeholder (미지정 시 i18n 기본값) */
  placeholder?: string;
  disabled?: boolean;
};

/**
 * 관리자 전용 사용자 검색 콤보박스 (Q5, 2026-04-22).
 *
 * 이메일 (@ 포함 시 blind index exact match) 또는 닉네임 LIKE 검색.
 * 300ms debounce, 최대 10건 표시.
 * 백엔드 `GET /admin/users?q=&size=10` 재사용.
 */
export function UserSearchCombobox({
  value,
  onChange,
  placeholder,
  disabled,
}: Props) {
  const { t } = useTranslation();
  const [keyword, setKeyword] = useState("");
  const [debounced, setDebounced] = useState("");
  const [open, setOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  // 300ms debounce
  useEffect(() => {
    const timer = setTimeout(() => setDebounced(keyword.trim()), 300);
    return () => clearTimeout(timer);
  }, [keyword]);

  // 외부 클릭 시 드롭다운 닫기
  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (
        containerRef.current &&
        !containerRef.current.contains(e.target as Node)
      ) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, []);

  const { data, isFetching } = useQuery({
    queryKey: ["admin", "users", "search", debounced],
    queryFn: () =>
      getAdminUsers({
        page: 1,
        size: 10,
        q: debounced,
      }),
    enabled: debounced.length >= 2 && open,
    staleTime: 60 * 1000,
  });

  const handleSelect = (user: AdminUserSummary) => {
    onChange(user);
    setOpen(false);
    setKeyword("");
    setDebounced("");
  };

  const handleClear = () => {
    onChange(null);
    setKeyword("");
    setDebounced("");
  };

  // 선택된 상태 표시
  if (value) {
    return (
      <div className="flex items-center gap-2 rounded-md border px-3 py-2">
        <div className="flex-1 min-w-0">
          <div className="text-sm font-medium truncate">
            {value.nickname || t("admin.textbook.create.userSearch.noNickname")}
          </div>
          <div className="text-xs text-muted-foreground truncate">
            {value.email} · ID: {value.id}
          </div>
        </div>
        <Button
          type="button"
          variant="ghost"
          size="icon"
          className="h-6 w-6"
          onClick={handleClear}
          disabled={disabled}
          aria-label={t("admin.textbook.create.userSearch.clear")}
        >
          <X className="h-4 w-4" />
        </Button>
      </div>
    );
  }

  // 검색 입력 모드
  const items = data?.items ?? [];
  const showDropdown = open && debounced.length >= 2;

  return (
    <div className="relative" ref={containerRef}>
      <div className="relative">
        <Search className="absolute left-2 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
        <Input
          value={keyword}
          onChange={(e) => {
            setKeyword(e.target.value);
            setOpen(true);
          }}
          onFocus={() => setOpen(true)}
          placeholder={
            placeholder ?? t("admin.textbook.create.userSearch.placeholder")
          }
          className="pl-8 pr-8"
          disabled={disabled}
        />
        {isFetching && (
          <Loader2 className="absolute right-2 top-1/2 -translate-y-1/2 h-4 w-4 animate-spin text-muted-foreground" />
        )}
      </div>

      {showDropdown && (
        <div className="absolute z-50 mt-1 w-full rounded-md border bg-popover shadow-md">
          {items.length === 0 && !isFetching && (
            <div className="px-3 py-4 text-sm text-muted-foreground text-center">
              {t("admin.textbook.create.userSearch.empty")}
            </div>
          )}
          {items.length > 0 && (
            <ul className="max-h-64 overflow-y-auto py-1">
              {items.map((user) => (
                <li key={user.id}>
                  <button
                    type="button"
                    className="w-full text-left px-3 py-2 hover:bg-accent focus:bg-accent focus:outline-none"
                    onClick={() => handleSelect(user)}
                  >
                    <div className="text-sm font-medium truncate">
                      {user.nickname ||
                        t("admin.textbook.create.userSearch.noNickname")}
                    </div>
                    <div className="text-xs text-muted-foreground truncate">
                      {user.email} · ID: {user.id}
                    </div>
                  </button>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}

      {debounced.length > 0 && debounced.length < 2 && open && (
        <div className="absolute z-50 mt-1 w-full rounded-md border bg-popover shadow-md px-3 py-2 text-xs text-muted-foreground">
          {t("admin.textbook.create.userSearch.minChars")}
        </div>
      )}
    </div>
  );
}
