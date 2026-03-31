import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { Package, ArrowRight } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { HeroSection } from "@/components/blocks/hero_section";
import { SectionContainer } from "@/components/blocks/section_container";
import { PageMeta } from "@/components/page_meta";

import { useMyTextbookOrders } from "../hook/use_my_orders";
import type { TextbookOrderStatus } from "../types";

const STATUS_VARIANT: Record<TextbookOrderStatus, "default" | "secondary" | "destructive" | "outline"> = {
  pending: "secondary",
  confirmed: "secondary",
  paid: "default",
  printing: "default",
  shipped: "default",
  delivered: "default",
  canceled: "destructive",
};

export function TextbookMyOrdersPage() {
  const { t, i18n } = useTranslation();
  const { data, isLoading, isError } = useMyTextbookOrders();

  const orders = data?.orders ?? [];

  if (isLoading) {
    return (
      <div className="flex flex-col">
        <PageMeta titleKey="textbook.myOrders.title" descriptionKey="textbook.myOrders.description" />
        <div className="max-w-3xl mx-auto px-6 py-20 w-full space-y-4">
          <Skeleton className="h-10 w-48" />
          {Array.from({ length: 3 }).map((_, i) => (
            <Skeleton key={i} className="h-28 rounded-xl" />
          ))}
        </div>
      </div>
    );
  }

  if (isError) {
    return (
      <div className="flex flex-col">
        <PageMeta titleKey="textbook.myOrders.title" descriptionKey="textbook.myOrders.description" />
        <SectionContainer>
          <p className="text-center text-muted-foreground py-16">
            {t("textbook.myOrders.loadError")}
          </p>
        </SectionContainer>
      </div>
    );
  }

  return (
    <div className="flex flex-col">
      <PageMeta titleKey="textbook.myOrders.title" descriptionKey="textbook.myOrders.description" />

      <HeroSection
        size="sm"
        badge={
          <>
            <Package className="h-4 w-4 text-primary" />
            <span className="text-sm text-muted-foreground">{t("textbook.myOrders.badge")}</span>
          </>
        }
        title={t("textbook.myOrders.title")}
        subtitle={t("textbook.myOrders.description")}
      />

      <SectionContainer>
        {orders.length === 0 ? (
          <div className="text-center py-16 space-y-4">
            <Package className="h-12 w-12 text-muted-foreground mx-auto" />
            <p className="text-muted-foreground">{t("textbook.myOrders.empty")}</p>
            <Button asChild>
              <Link to="/book/textbook">{t("textbook.myOrders.browseCatalog")}</Link>
            </Button>
          </div>
        ) : (
          <div className="max-w-3xl mx-auto space-y-4">
            {orders.map((order) => (
              <Link
                key={order.order_id}
                to={`/book/textbook/order/${order.order_code}`}
                className="block"
              >
                <Card className="hover:shadow-card-hover hover:-translate-y-0.5 transition-all duration-200">
                  <CardContent className="p-4 sm:p-6">
                    <div className="flex items-start justify-between gap-4">
                      <div className="space-y-1.5 min-w-0">
                        <div className="flex items-center gap-2 flex-wrap">
                          <span className="font-mono text-sm font-medium">
                            {order.order_code}
                          </span>
                          <Badge variant={STATUS_VARIANT[order.status]}>
                            {t(`textbook.myOrders.status.${order.status}`)}
                          </Badge>
                        </div>
                        <p className="text-sm text-muted-foreground">
                          {order.items
                            .map((item) =>
                              `${i18n.language === "ko" ? item.language_name : item.language_name} × ${item.quantity}`
                            )
                            .join(", ")}
                        </p>
                        <p className="text-xs text-muted-foreground">
                          {new Date(order.created_at).toLocaleDateString(i18n.language)}
                        </p>
                      </div>
                      <div className="text-right flex-shrink-0">
                        <p className="font-semibold">
                          ₩{order.total_amount.toLocaleString()}
                        </p>
                        <p className="text-xs text-muted-foreground">
                          {order.total_quantity}{t("textbook.order.unit")}
                        </p>
                        <ArrowRight className="h-4 w-4 text-muted-foreground ml-auto mt-1" />
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </Link>
            ))}
          </div>
        )}
      </SectionContainer>
    </div>
  );
}
