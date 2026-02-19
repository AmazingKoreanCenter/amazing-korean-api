import { useTranslation } from "react-i18next";
import type { LucideIcon } from "lucide-react";

import { cn } from "@/lib/utils";

interface ListStatsBarProps {
  icon: LucideIcon;
  totalLabel: string;
  total: number;
  currentPage: number;
  totalPages: number;
  isFetching?: boolean;
  className?: string;
}

export function ListStatsBar({
  icon: Icon,
  totalLabel,
  total,
  currentPage,
  totalPages,
  isFetching,
  className,
}: ListStatsBarProps) {
  const { t } = useTranslation();

  return (
    <div className={cn("mb-8 flex items-center justify-between", className)}>
      <div className="flex items-center gap-2 text-sm text-muted-foreground">
        <Icon className="h-4 w-4" />
        <span>{totalLabel.replace("{count}", String(total))}</span>
        <span className="text-border">|</span>
        <span>
          {currentPage} / {totalPages} {t("common.page")}
        </span>
      </div>
      {isFetching && (
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <span className="h-2 w-2 animate-pulse rounded-full bg-secondary" />
          {t("common.loading")}
        </div>
      )}
    </div>
  );
}
