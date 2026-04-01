import { useLocation, useNavigate, Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { CheckCircle2, Copy, BookOpen } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { PageMeta } from "@/components/page_meta";

import type { PurchaseRes } from "../types";

const BANK_ACCOUNT = "하나은행 915-910012-71304 주식회사 힘";

export function EbookPurchaseCompletePage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const location = useLocation();

  const purchase = location.state as PurchaseRes | undefined;

  // state가 없으면 (직접 URL 접근) 내 E-book 목록으로 리다이렉트
  if (!purchase) {
    return (
      <div className="flex flex-col">
        <PageMeta
          titleKey="ebook.purchaseComplete.title"
          descriptionKey="ebook.purchaseComplete.description"
        />
        <div className="max-w-2xl mx-auto px-6 py-section-sm md:py-section-lg text-center space-y-4">
          <p className="text-muted-foreground">
            {t("ebook.purchaseComplete.noData")}
          </p>
          <Button asChild>
            <Link to="/book/ebook/my">{t("ebook.purchaseComplete.goToMyEbooks")}</Link>
          </Button>
        </div>
      </div>
    );
  }

  const isBankTransfer = purchase.payment_method === "bank_transfer";
  const editionLabel =
    purchase.edition === "teacher"
      ? t("ebook.catalog.teacherEdition")
      : t("ebook.catalog.studentEdition");

  const copyCode = () => {
    navigator.clipboard.writeText(purchase.purchase_code);
    toast.success(t("ebook.purchaseComplete.codeCopied"));
  };

  return (
    <div className="flex flex-col">
      <PageMeta
        titleKey="ebook.purchaseComplete.title"
        descriptionKey="ebook.purchaseComplete.description"
      />
      <div className="max-w-2xl mx-auto px-6 py-section-sm md:py-section-lg">
        <Card>
          <CardHeader className="text-center">
            <div className="mx-auto w-16 h-16 rounded-full bg-status-success/10 flex items-center justify-center mb-4">
              <CheckCircle2 className="h-8 w-8 text-status-success" />
            </div>
            <CardTitle className="text-2xl">
              {t("ebook.purchaseComplete.title")}
            </CardTitle>
            <p className="text-muted-foreground mt-2">
              {t("ebook.purchaseComplete.subtitle")}
            </p>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* 구매 코드 */}
            <div className="p-4 rounded-lg bg-muted/50 border border-border text-center">
              <p className="text-sm text-muted-foreground mb-1">
                {t("ebook.purchaseComplete.purchaseCode")}
              </p>
              <div className="flex items-center justify-center gap-2">
                <span className="text-2xl font-bold font-mono">
                  {purchase.purchase_code}
                </span>
                <Button variant="ghost" size="icon" onClick={copyCode}>
                  <Copy className="h-4 w-4" />
                </Button>
              </div>
            </div>

            {/* 주문 요약 */}
            <div className="space-y-2 text-sm">
              <div className="flex justify-between">
                <span className="text-muted-foreground">
                  {t("ebook.purchaseComplete.edition")}
                </span>
                <span className="font-medium">{editionLabel}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">
                  {t("ebook.purchaseComplete.price")}
                </span>
                <span className="font-bold text-lg">
                  {purchase.price.toLocaleString()} {purchase.currency}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">
                  {t("ebook.purchaseComplete.paymentMethod")}
                </span>
                <span className="font-medium">
                  {isBankTransfer
                    ? t("ebook.purchase.bankTransfer")
                    : t("ebook.purchase.creditCard")}
                </span>
              </div>
            </div>

            {/* 계좌이체 입금 안내 */}
            {isBankTransfer && (
              <div className="p-4 rounded-lg border border-status-warning/30 bg-status-warning/5">
                <p className="font-semibold mb-2">
                  {t("ebook.purchaseComplete.bankGuideTitle")}
                </p>
                <p className="text-sm text-muted-foreground">
                  {t("ebook.purchaseComplete.bankAccount", { account: BANK_ACCOUNT })}
                </p>
                <p className="text-sm text-muted-foreground mt-2">
                  {t("ebook.purchaseComplete.bankNote")}
                </p>
              </div>
            )}

            {/* 버튼 */}
            <div className="flex flex-col gap-2">
              <Button onClick={() => navigate("/book/ebook/my")}>
                <BookOpen className="h-4 w-4 mr-2" />
                {t("ebook.purchaseComplete.goToMyEbooks")}
              </Button>
              <Button variant="outline" asChild>
                <Link to="/book/ebook">{t("ebook.purchaseComplete.browseCatalog")}</Link>
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
