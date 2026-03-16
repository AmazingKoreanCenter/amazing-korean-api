import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { BookOpen, Eye } from "lucide-react";

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
import type { EbookPurchaseStatus } from "../types";

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
  const { data, isLoading, isError } = useMyPurchases();

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

  const items = data?.items ?? [];

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
                    {purchase.status === "completed" && (
                      <Button size="sm" asChild>
                        <Link to={`/ebook/viewer/${purchase.purchase_code}`}>
                          <Eye className="w-4 h-4 mr-1" />
                          {t("ebook.my.openViewer")}
                        </Link>
                      </Button>
                    )}
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
