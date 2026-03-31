import { useTranslation } from "react-i18next";
import { AlertTriangle } from "lucide-react";

import { Button } from "@/components/ui/button";
import type { EbookCatalogItem, EbookEdition } from "../types";

interface EbookSelectedDetailProps {
  item: EbookCatalogItem;
  edition: EbookEdition;
  onDetailOpen?: (item: EbookCatalogItem, edition: EbookEdition) => void;
}

export function EbookSelectedDetail({ item, edition, onDetailOpen }: EbookSelectedDetailProps) {
  const { t, i18n } = useTranslation();
  const langName = i18n.language === "ko" ? item.language_name_ko : item.language_name_en;

  return (
    <div className="flex flex-col md:flex-row gap-5 md:h-[420px]">
      {/* Cover image — shared with textbook */}
      <div className="flex-shrink-0 flex justify-center md:h-full">
        <img
          src={`/covers/${edition}-${item.language}.webp`}
          alt={langName}
          loading="lazy"
          className="h-48 md:h-full w-auto rounded-xl bg-muted shadow-lg"
        />
      </div>

      {/* Details */}
      <div className="flex-1 flex flex-col justify-between gap-3 md:gap-0 md:py-1">
        {/* 1. Title */}
        <h3 className="text-lg md:text-xl font-bold text-center md:text-left">
          {t("ebook.catalog.bookTitle", { language: langName })}
        </h3>

        {/* 2. Badge */}
        <div className="h-8 flex items-center justify-center md:justify-start">
          <span className="inline-flex items-center gap-1.5 text-sm font-medium text-status-warning bg-status-warning/5 border border-status-warning/20 rounded-md px-3 py-1.5">
            <AlertTriangle className="h-3.5 w-3.5" />
            {edition === "teacher" ? t("ebook.detail.teacherComingSoon") : t("ebook.detail.studentComingSoon")}
          </span>
        </div>

        {/* 3. Description */}
        <div className="text-sm text-muted-foreground leading-relaxed space-y-4 text-center md:text-left">
          {t("ebook.catalog.bookDescription", { language: langName }).split("\n").map((line, i) => (
            <p key={i}>{line}</p>
          ))}
        </div>

        {/* 4. Price */}
        <div className="flex gap-3">
          {onDetailOpen && <div className="flex-1 hidden md:block" />}
          <p className="flex-1 text-center text-base md:text-lg font-bold">
            {t("ebook.catalog.pricePerUnit")}
          </p>
        </div>

        {/* 5. Buttons */}
        <div className="flex gap-3">
          {onDetailOpen && (
            <Button
              variant="outline"
              size="default"
              className="rounded-full flex-1"
              onClick={() => onDetailOpen(item, edition)}
            >
              {t("ebook.detail.viewDetail")}
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}
