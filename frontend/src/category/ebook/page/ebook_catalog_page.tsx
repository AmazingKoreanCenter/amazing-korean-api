import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { BookOpen, Loader2, ShoppingCart } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { useAuthStore } from "@/hooks/use_auth_store";

import { useEbookCatalog } from "../hook/use_ebook_catalog";
import { useCreateEbookPurchase } from "../hook/use_create_purchase";
import type { EbookEdition } from "../types";

export function EbookCatalogPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { isLoggedIn } = useAuthStore();
  const { data, isLoading, isError } = useEbookCatalog();
  const purchaseMutation = useCreateEbookPurchase();

  const [selectedLang, setSelectedLang] = useState<string | null>(null);
  const [selectedEdition, setSelectedEdition] = useState<EbookEdition>("teacher");

  const handlePurchase = () => {
    if (!isLoggedIn) {
      navigate("/login");
      return;
    }

    if (!selectedLang) {
      toast.error(t("ebook.catalog.selectLanguage"));
      return;
    }

    purchaseMutation.mutate(
      {
        language: selectedLang,
        edition: selectedEdition,
        payment_method: "bank_transfer",
      },
      {
        onSuccess: () => {
          toast.success(t("ebook.purchase.success"));
          navigate("/ebook/my");
        },
        onError: (error) => {
          toast.error(error.message || t("ebook.purchase.error"));
        },
      }
    );
  };

  if (isLoading) {
    return (
      <div className="container mx-auto py-12 px-4 max-w-5xl">
        <Skeleton className="h-10 w-64 mb-8" />
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {Array.from({ length: 6 }).map((_, i) => (
            <Skeleton key={i} className="h-48" />
          ))}
        </div>
      </div>
    );
  }

  if (isError) {
    return (
      <div className="container mx-auto py-12 px-4 max-w-5xl text-center text-muted-foreground">
        {t("ebook.catalog.loadError")}
      </div>
    );
  }

  const items = data?.items ?? [];

  const formatPrice = (editionInfo: { price: number; currency: string }) => {
    return `${editionInfo.price.toLocaleString()} ${editionInfo.currency}`;
  };

  return (
    <div className="container mx-auto py-12 px-4 max-w-5xl">
      <div className="text-center mb-10">
        <Badge variant="secondary" className="mb-4">
          <BookOpen className="w-3 h-3 mr-1" />
          E-book
        </Badge>
        <h1 className="text-3xl font-bold mb-2">{t("ebook.catalog.title")}</h1>
        <p className="text-muted-foreground">{t("ebook.catalog.subtitle")}</p>
      </div>

      {/* 에디션 선택 */}
      <div className="flex justify-center gap-4 mb-8">
        <Button
          variant={selectedEdition === "teacher" ? "default" : "outline"}
          onClick={() => { setSelectedEdition("teacher"); setSelectedLang(null); }}
        >
          {t("ebook.catalog.teacherEdition")}
        </Button>
        <Button
          variant={selectedEdition === "student" ? "default" : "outline"}
          onClick={() => { setSelectedEdition("student"); setSelectedLang(null); }}
        >
          {t("ebook.catalog.studentEdition")}
        </Button>
      </div>

      {/* 언어 그리드 */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 mb-8">
        {items.map((item) => {
          const editionInfo = item.editions.find(
            (e) => e.edition === selectedEdition
          );
          if (!editionInfo) return null;

          const isSelected = selectedLang === item.language;

          return (
            <Card
              key={item.language}
              className={`cursor-pointer transition-all ${
                isSelected
                  ? "ring-2 ring-primary"
                  : editionInfo.available
                    ? "hover:shadow-md"
                    : "opacity-50"
              }`}
              onClick={() => {
                if (editionInfo.available) {
                  setSelectedLang(isSelected ? null : item.language);
                }
              }}
            >
              <CardHeader className="pb-2">
                <CardTitle className="text-lg flex items-center justify-between">
                  <span>{item.language_name_ko}</span>
                  {!editionInfo.available && (
                    <Badge variant="secondary">{t("ebook.catalog.comingSoon")}</Badge>
                  )}
                </CardTitle>
                <p className="text-sm text-muted-foreground">
                  {item.language_name_en}
                </p>
              </CardHeader>
              <CardContent>
                <div className="flex justify-between items-center">
                  <span className="text-lg font-semibold">
                    {formatPrice(editionInfo)}
                  </span>
                  <span className="text-sm text-muted-foreground">
                    {editionInfo.total_pages}p
                  </span>
                </div>
              </CardContent>
            </Card>
          );
        })}
      </div>

      {/* 구매 섹션 */}
      {selectedLang && (
        <Card className="max-w-md mx-auto">
          <CardHeader>
            <CardTitle className="text-lg">{t("ebook.purchase.title")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <Button
              className="w-full"
              onClick={handlePurchase}
              disabled={purchaseMutation.isPending}
            >
              {purchaseMutation.isPending ? (
                <Loader2 className="w-4 h-4 mr-2 animate-spin" />
              ) : (
                <ShoppingCart className="w-4 h-4 mr-2" />
              )}
              {isLoggedIn
                ? t("ebook.purchase.buy")
                : t("ebook.purchase.loginRequired")}
            </Button>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
