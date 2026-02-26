import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useSearchParams } from "react-router-dom";
import { CheckCircle2, Sparkles, Crown, XCircle, Tag } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { HeroSection } from "@/components/sections/hero_section";
import { PageMeta } from "@/components/page_meta";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { useAuthStore } from "@/hooks/use_auth_store";

import { usePaymentPlans } from "../hook/use_payment_plans";
import { useSubscription } from "../hook/use_subscription";
import { usePaddle } from "../hook/use_paddle";
import { useCancelSubscription } from "../hook/use_manage_subscription";
import type { PlanInfo } from "../types";

const POPULAR_INTERVAL = "month_12";

export function PricingPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();
  const isLoggedIn = useAuthStore((s) => s.isLoggedIn);

  const { data: plansData, isLoading: plansLoading } = usePaymentPlans();
  const { data: subData, isLoading: subLoading } = useSubscription();

  const { openCheckout } = usePaddle({
    clientToken: plansData?.client_token ?? "",
    sandbox: plansData?.sandbox ?? true,
  });

  const cancelMutation = useCancelSubscription();

  const [cancelDialogOpen, setCancelDialogOpen] = useState(false);
  const [promoCode, setPromoCode] = useState("");

  const subscription = subData?.subscription ?? null;
  const hasActiveSub = subscription && ["active", "trialing"].includes(subscription.status);
  const hasSub = subscription != null;

  // checkout 성공 시 toast 표시
  useEffect(() => {
    if (searchParams.get("success") === "true") {
      toast.success(t("payment.checkoutSuccess"));
      setSearchParams({}, { replace: true });
    }
  }, [searchParams, setSearchParams, t]);

  const handleSelectPlan = (plan: PlanInfo) => {
    if (!isLoggedIn) {
      navigate("/login?redirect=/pricing");
      return;
    }
    if (hasActiveSub) {
      toast.info(t("payment.alreadySubscribed"));
      return;
    }
    openCheckout(plan.price_id, promoCode.trim() || undefined);
  };

  const perMonth = (plan: PlanInfo) => {
    const monthly = plan.price_cents / plan.months;
    return `$${(monthly / 100).toFixed(2)}`;
  };

  if (plansLoading || (isLoggedIn && subLoading)) {
    return (
      <div className="max-w-[1350px] mx-auto px-6 lg:px-8 py-20">
        <div className="text-center mb-16">
          <Skeleton className="h-10 w-64 mx-auto mb-4" />
          <Skeleton className="h-6 w-96 mx-auto" />
        </div>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          {[1, 2, 3, 4].map((i) => (
            <Skeleton key={i} className="h-80 rounded-2xl" />
          ))}
        </div>
      </div>
    );
  }

  const plans = plansData?.plans ?? [];

  return (
    <div className="flex flex-col">
      <PageMeta titleKey="seo.pricing.title" descriptionKey="seo.pricing.description" />
      <HeroSection
        size="sm"
        badge={
          <>
            <Sparkles className="h-4 w-4 text-status-warning" />
            <span className="text-sm text-muted-foreground">
              {t("payment.trialBadge", { days: 1 })}
            </span>
          </>
        }
        title={t("payment.title")}
        subtitle={t("payment.subtitle")}
      />

      {/* Pricing Cards */}
      <section className="py-16 lg:py-24">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8">
          {/* Subscription Banner */}
          {hasSub && subscription && (() => {
            const status = subscription.status;
            const isCanceled = status === "canceled";
            const isCancelScheduled = !isCanceled && !!subscription.canceled_at;
            const isActive = (status === "active" || status === "trialing") && !isCancelScheduled;
            const isBusy = cancelMutation.isPending;

            const bannerStyle = isCanceled || isCancelScheduled
              ? "from-destructive/10 to-destructive/5 border-destructive/20"
              : "from-status-success/10 to-status-success/5 border-status-success/20";

            const iconBg = isCanceled || isCancelScheduled ? "bg-destructive/10" : "bg-status-success/10";
            const iconColor = isCanceled || isCancelScheduled ? "text-destructive" : "text-status-success";
            const textColor = isCanceled || isCancelScheduled ? "text-destructive" : "text-status-success";
            const subTextColor = isCanceled || isCancelScheduled ? "text-destructive/80" : "text-status-success/80";
            const StatusIcon = isCanceled || isCancelScheduled ? XCircle : CheckCircle2;

            return (
              <div className={`mb-12 p-6 rounded-2xl bg-gradient-to-r ${bannerStyle} border`}>
                <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                  <div className="flex items-center gap-3">
                    <div className={`w-10 h-10 rounded-full ${iconBg} flex items-center justify-center`}>
                      <StatusIcon className={`h-5 w-5 ${iconColor}`} />
                    </div>
                    <div>
                      <p className={`font-semibold ${textColor}`}>
                        {t("payment.currentPlan")}: {t(`payment.interval.${subscription.billing_interval}`)}
                      </p>
                      <p className={`text-sm ${subTextColor}`}>
                        {isCancelScheduled
                          ? t("payment.cancelScheduled")
                          : t(`payment.status.${status}`)}
                        {(isCanceled || isCancelScheduled) && subscription.current_period_end && (
                          <> &middot; {t("payment.expiresAt")}: {new Date(subscription.current_period_end).toLocaleDateString()}</>
                        )}
                        {!isCanceled && !isCancelScheduled && subscription.current_period_end && (
                          <> &middot; {t("payment.nextBilling")}: {new Date(subscription.current_period_end).toLocaleDateString()}</>
                        )}
                      </p>
                    </div>
                  </div>

                  {/* 취소 버튼 (active/trialing 상태 + 취소 예정 아닌 경우만) */}
                  {isActive && (
                    <Button
                      variant="outline"
                      size="sm"
                      className="text-destructive border-destructive/20 hover:bg-destructive/10 hover:text-destructive"
                      disabled={isBusy}
                      onClick={() => setCancelDialogOpen(true)}
                    >
                      {t("payment.cancelSubscription")}
                    </Button>
                  )}
                </div>
              </div>
            );
          })()}

          {/* Plan Cards */}
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
            {plans.map((plan) => {
              const isPopular = plan.interval === POPULAR_INTERVAL;
              const isCurrentPlan = hasActiveSub && subscription?.billing_interval === plan.interval;

              return (
                <Card
                  key={plan.interval}
                  className={`relative rounded-2xl transition-all duration-300 hover:shadow-lg ${
                    isPopular
                      ? "border-2 border-primary shadow-lg scale-[1.02]"
                      : "border hover:border-primary/30"
                  }`}
                >
                  {isPopular && (
                    <div className="absolute -top-3 left-1/2 -translate-x-1/2">
                      <Badge className="gradient-primary text-white px-4 py-1 text-xs font-medium shadow-md">
                        <Crown className="h-3 w-3 mr-1" />
                        {t("payment.popular")}
                      </Badge>
                    </div>
                  )}

                  <CardHeader className="text-center pt-8 pb-2">
                    <h3 className="text-lg font-semibold text-muted-foreground">
                      {t(`payment.interval.${plan.interval}`)}
                    </h3>
                    <div className="mt-4">
                      <span className="text-4xl font-bold">{plan.price_display}</span>
                      {plan.months > 1 && (
                        <span className="text-sm text-muted-foreground ml-1">
                          / {plan.months}{t("payment.months")}
                        </span>
                      )}
                    </div>
                    {plan.months > 1 && (
                      <p className="text-sm text-primary font-medium mt-1">
                        {perMonth(plan)}{t("payment.perMonth")}
                      </p>
                    )}
                  </CardHeader>

                  <CardContent className="px-6 py-4">
                    <ul className="space-y-3">
                      <li className="flex items-center gap-2 text-sm">
                        <CheckCircle2 className="h-4 w-4 text-status-success shrink-0" />
                        <span>{t("payment.featureAllCourses")}</span>
                      </li>
                      <li className="flex items-center gap-2 text-sm">
                        <CheckCircle2 className="h-4 w-4 text-status-success shrink-0" />
                        <span>{t("payment.featureAllVideos")}</span>
                      </li>
                      <li className="flex items-center gap-2 text-sm">
                        <CheckCircle2 className="h-4 w-4 text-status-success shrink-0" />
                        <span>{t("payment.featureStudyMaterials")}</span>
                      </li>
                      <li className="flex items-center gap-2 text-sm">
                        <CheckCircle2 className="h-4 w-4 text-status-success shrink-0" />
                        <span>{t("payment.featureTrial", { days: plan.trial_days })}</span>
                      </li>
                      {plan.months >= 6 && (
                        <li className="flex items-center gap-2 text-sm">
                          <CheckCircle2 className="h-4 w-4 text-status-success shrink-0" />
                          <span className="text-primary font-medium">
                            {t("payment.featureSave", {
                              percent: Math.round(
                                (1 - plan.price_cents / plan.months / (plans[0]?.price_cents ?? plan.price_cents)) * 100
                              ),
                            })}
                          </span>
                        </li>
                      )}
                    </ul>
                  </CardContent>

                  <CardFooter className="px-6 pb-8">
                    <Button
                      className={`w-full rounded-xl h-12 text-base ${
                        isPopular
                          ? "gradient-primary text-white hover:opacity-90"
                          : ""
                      }`}
                      variant={isPopular ? "default" : "outline"}
                      disabled={!!isCurrentPlan}
                      onClick={() => handleSelectPlan(plan)}
                    >
                      {isCurrentPlan
                        ? t("payment.currentPlanLabel")
                        : t("payment.selectPlan")}
                    </Button>
                  </CardFooter>
                </Card>
              );
            })}
          </div>

          {/* Promo Code */}
          {!hasActiveSub && (
            <div className="flex justify-center mt-10">
              <div className="flex items-center gap-2 max-w-sm w-full">
                <div className="relative flex-1">
                  <Tag className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                  <Input
                    placeholder={t("payment.promoCodePlaceholder")}
                    value={promoCode}
                    onChange={(e) => setPromoCode(e.target.value)}
                    className="pl-9"
                  />
                </div>
                {promoCode && (
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => setPromoCode("")}
                  >
                    {t("payment.promoCodeClear")}
                  </Button>
                )}
              </div>
            </div>
          )}

          {/* Bottom Note */}
          <p className="text-center text-sm text-muted-foreground mt-12">
            {t("payment.bottomNote")}
          </p>
        </div>
      </section>

      {/* Cancel Confirmation Dialog */}
      <Dialog open={cancelDialogOpen} onOpenChange={setCancelDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("payment.cancelConfirmTitle")}</DialogTitle>
            <DialogDescription>{t("payment.cancelConfirmMessage")}</DialogDescription>
          </DialogHeader>
          <DialogFooter className="flex flex-col sm:flex-row gap-2">
            <Button
              variant="outline"
              onClick={() => {
                cancelMutation.mutate({ immediately: false }, {
                  onSuccess: () => setCancelDialogOpen(false),
                });
              }}
              disabled={cancelMutation.isPending}
            >
              {t("payment.cancelAtPeriodEnd")}
            </Button>
            <Button
              variant="destructive"
              onClick={() => {
                cancelMutation.mutate({ immediately: true }, {
                  onSuccess: () => setCancelDialogOpen(false),
                });
              }}
              disabled={cancelMutation.isPending}
            >
              {t("payment.cancelImmediate")}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
