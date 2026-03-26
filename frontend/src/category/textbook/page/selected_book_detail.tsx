import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { AlertTriangle, ArrowRight, CircleCheck } from "lucide-react";

import { Button } from "@/components/ui/button";
import type { CatalogItem, TextbookType } from "../types";

interface SelectedBookDetailProps {
  item: CatalogItem;
  type: TextbookType;
  onDetailOpen?: (item: CatalogItem, type: TextbookType) => void;
}

export function SelectedBookDetail({ item, type, onDetailOpen }: SelectedBookDetailProps) {
  const { t, i18n } = useTranslation();
  const langName = i18n.language === "ko" ? item.language_name_ko : item.language_name_en;

  return (
    <div className="flex flex-col md:flex-row gap-5 md:h-[420px]">
      {/* Cover image */}
      <div className="flex-shrink-0 flex justify-center md:h-full">
        <img
          src={`/covers/${type}-${item.language}.webp`}
          alt={langName}
          className="h-48 md:h-full w-auto rounded-xl bg-muted shadow-lg"
        />
      </div>

      {/* Details */}
      <div className="flex-1 flex flex-col justify-between gap-3 md:gap-0 md:py-1">
        {/* 1. Title */}
        <h3 className="text-lg md:text-xl font-bold text-center md:text-left">{t("textbook.catalog.bookTitle", { language: langName })}</h3>

        {/* 2. Status badge */}
        <div className="h-8 flex items-center justify-center md:justify-start">
          {item.isbn_ready ? (
            <span className="inline-flex items-center gap-1.5 text-sm font-medium text-emerald-600 bg-emerald-50 border border-emerald-200 rounded-md px-3 py-1.5">
              <CircleCheck className="h-3.5 w-3.5" />
              {t("textbook.catalog.editionInfo", { edition: type === "student" ? t("textbook.catalog.studentSection") : t("textbook.catalog.teacherSection") })}
            </span>
          ) : (
            <span className="inline-flex items-center gap-1.5 text-sm font-medium text-amber-600 bg-amber-50 border border-amber-200 rounded-md px-3 py-1.5">
              <AlertTriangle className="h-3.5 w-3.5" />
              {t("textbook.catalog.isbnPending")}
            </span>
          )}
        </div>

        {/* 3. Description */}
        <div className="text-sm text-muted-foreground leading-relaxed space-y-4 text-center md:text-left">
          {t(type === "student" ? "textbook.catalog.bookDescriptionStudent" : "textbook.catalog.bookDescriptionTeacher", { language: langName }).split("\n").map((line, i) => (
            <p key={i}>{line}</p>
          ))}
        </div>

        {/* 4. Price + 5. Buttons */}
        <div className="flex gap-3">
          {onDetailOpen && <div className="flex-1 hidden md:block" />}
          <p className="flex-1 text-center text-base md:text-lg font-bold">
            {t("textbook.catalog.pricePerUnit")}
          </p>
        </div>
        <div className="flex gap-3">
          {onDetailOpen && (
            <Button
              variant="outline"
              size="default"
              className="rounded-full flex-1"
              onClick={() => onDetailOpen(item, type)}
            >
              {t("textbook.detail.viewDetail")}
            </Button>
          )}
          <Button asChild size="default" className="rounded-full flex-1">
            <Link to={`/book/textbook/order?lang=${item.language}&type=${type}`}>
              {t("textbook.catalog.orderButton")}
              <ArrowRight className="ml-1.5 h-4 w-4" />
            </Link>
          </Button>
        </div>
      </div>
    </div>
  );
}
