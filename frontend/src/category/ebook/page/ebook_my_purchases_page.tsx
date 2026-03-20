import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { BookOpen, CreditCard, Eye, Loader2, X } from "lucide-react";
import { toast } from "sonner";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";

import { useMyPurchases } from "../hook/use_my_purchases";
import { useEbookCatalog } from "../hook/use_ebook_catalog";
import { cancelEbookPurchase } from "../ebook_api";
import { usePaddle } from "@/category/payment/hook/use_paddle";
import { useUserMe } from "@/category/user/hook/use_user_me";
import type { EbookPurchaseStatus, PurchaseRes } from "../types";

const STATUS_BADGE: Record<
  EbookPurchaseStatus,
  { variant: "default" | "secondary" | "destructive"; label: string }
> = {
  pending: { variant: "secondary", label: "ebook.status.pending" },
  completed: { variant: "default", label: "ebook.status.completed" },
  refunded: { variant: "destructive", label: "ebook.status.refunded" },
};

export function EbookMyPurchasesPage() {
  const { t } = useTranslation();
  const queryClient = useQueryClient();
  const { data, isLoading, isError } = useMyPurchases();
  const { data: catalog } = useEbookCatalog();
  const { data: userMe } = useUserMe();

  const items = data?.items ?? [];

  const { openEbookCheckout } = usePaddle({
    clientToken: catalog?.client_token ?? "",
    sandbox: catalog?.sandbox ?? false,
    email: userMe?.email,
    onCheckoutComplete: () => {
      queryClient.invalidateQueries({ queryKey: ["ebook", "my-purchases"] });
    },
  });

  const cancelMutation = useMutation({
    mutationFn: cancelEbookPurchase,
    onSuccess: () => {
      toast.success(t("ebook.purchase.cancelSuccess"));
      queryClient.invalidateQueries({ queryKey: ["ebook", "my-purchases"] });
    },
    onError: () => {
      toast.error(t("ebook.purchase.error"));
    },
  });

  const handlePayNow = (purchase: PurchaseRes) => {
    const priceId = catalog?.paddle_ebook_price_id;
    if (!priceId) {
      toast.error(t("ebook.purchase.paddleUnavailable"));
      return;
    }
    openEbookCheckout(priceId, purchase.purchase_code);
  };

  if (isLoading) {
    return (
      <div className="container mx-auto py-12 px-4 max-w-3xl">
        <Skeleton className="h-10 w-48 mb-8" />
        {Array.from({ length: 3 }).map((_, i) => (
          <Skeleton key={i} className="h-24 mb-4" />
        ))}
      </div>
    );
  }

  if (isError) {
    return (
      <div className="container mx-auto py-12 px-4 max-w-3xl text-center text-muted-foreground">
        {t("ebook.my.loadError")}
      </div>
    );
  }

  return (
    <div className="container mx-auto py-12 px-4 max-w-3xl">
      <div className="flex items-center gap-2 mb-8">
        <BookOpen className="w-6 h-6" />
        <h1 className="text-2xl font-bold">{t("ebook.my.title")}</h1>
      </div>

      {items.length === 0 ? (
        <Card>
          <CardContent className="py-12 text-center text-muted-foreground">
            {t("ebook.my.empty")}
            <div className="mt-4">
              <Button asChild>
                <Link to="/ebook">{t("ebook.my.browseCatalog")}</Link>
              </Button>
            </div>
          </CardContent>
        </Card>
      ) : (
        <div className="space-y-4">
          {items.map((purchase) => {
            const statusInfo = STATUS_BADGE[purchase.status];
            const isPendingPaddle =
              purchase.status === "pending" && purchase.payment_method === "paddle";

            return (
              <Card key={purchase.purchase_code}>
                <CardHeader className="pb-2">
                  <div className="flex items-center justify-between">
                    <CardTitle className="text-base">
                      {purchase.purchase_code}
                    </CardTitle>
                    <Badge variant={statusInfo.variant}>
                      {t(statusInfo.label)}
                    </Badge>
                  </div>
                </CardHeader>
                <CardContent>
                  <div className="flex items-center justify-between">
                    <div className="text-sm text-muted-foreground">
                      <span className="capitalize">{purchase.edition}</span>
                      {" · "}
                      <span>{purchase.language}</span>
                      {" · "}
                      <span>
                        {purchase.price.toLocaleString()} {purchase.currency}
                      </span>
                    </div>
                    <div className="flex gap-2">
                      {purchase.status === "completed" && (
                        <Button size="sm" asChild>
                          <Link to={`/ebook/viewer/${purchase.purchase_code}`}>
                            <Eye className="w-4 h-4 mr-1" />
                            {t("ebook.my.openViewer")}
                          </Link>
                        </Button>
                      )}
                      {isPendingPaddle && (
                        <>
                          <Button
                            size="sm"
                            onClick={() => handlePayNow(purchase)}
                          >
                            <CreditCard className="w-4 h-4 mr-1" />
                            {t("ebook.my.payNow")}
                          </Button>
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => cancelMutation.mutate(purchase.purchase_code)}
                            disabled={cancelMutation.isPending}
                          >
                            {cancelMutation.isPending ? (
                              <Loader2 className="w-4 h-4 mr-1 animate-spin" />
                            ) : (
                              <X className="w-4 h-4 mr-1" />
                            )}
                            {t("ebook.my.cancelOrder")}
                          </Button>
                        </>
                      )}
                      {purchase.status === "pending" &&
                        purchase.payment_method === "bank_transfer" && (
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => cancelMutation.mutate(purchase.purchase_code)}
                            disabled={cancelMutation.isPending}
                          >
                            {cancelMutation.isPending ? (
                              <Loader2 className="w-4 h-4 mr-1 animate-spin" />
                            ) : (
                              <X className="w-4 h-4 mr-1" />
                            )}
                            {t("ebook.my.cancelOrder")}
                          </Button>
                        )}
                    </div>
                  </div>
                </CardContent>
              </Card>
            );
          })}
        </div>
      )}
    </div>
  );
}
