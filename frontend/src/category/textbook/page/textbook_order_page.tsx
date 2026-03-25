import { useRef, useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { useSearchParams } from "react-router-dom";
import { useForm, useFieldArray } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import {
  BookOpen,
  Plus,
  Trash2,
  Loader2,
  CheckCircle2,
  Copy,
  Package,
  ScrollText,
  FileText,
} from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Checkbox } from "@/components/ui/checkbox";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { HeroSection } from "@/components/sections/hero_section";
import { PageMeta } from "@/components/page_meta";

import { useUserMe } from "@/category/user/hook/use_user_me";

import { useCatalog } from "../hook/use_catalog";
import { useCreateOrder } from "../hook/use_create_order";
import type {
  TextbookLanguage,
  TextbookType,
  OrderRes,
} from "../types";

// =============================================================================
// Form Schema
// =============================================================================

const orderItemSchema = z.object({
  language: z.string().min(1),
  textbook_type: z.enum(["student", "teacher"]),
  quantity: z.number().int().min(1).max(9999),
});

const orderFormSchema = z.object({
  orderer_name: z.string().min(1).max(100),
  orderer_email: z.string().email().max(255),
  orderer_phone: z.string().min(1).max(30),
  org_name: z.string().max(200),
  org_type: z.string().max(100),
  delivery_postal_code: z.string().max(20),
  delivery_address: z.string().min(1),
  delivery_detail: z.string().max(200),
  depositor_name: z.string().max(100),
  tax_invoice: z.boolean(),
  tax_biz_number: z.string().max(20),
  tax_company_name: z.string().max(200),
  tax_rep_name: z.string().max(100),
  tax_address: z.string().max(500),
  tax_biz_type: z.string().max(100),
  tax_biz_item: z.string().max(100),
  tax_email: z.string().email().max(255).or(z.literal("")),
  items: z.array(orderItemSchema).min(1),
  notes: z.string(),
});

type OrderFormValues = z.infer<typeof orderFormSchema>;

// =============================================================================
// Constants
// =============================================================================

const UNIT_PRICE = 25_000;
const MIN_TOTAL_QUANTITY = 10;
const BANK_ACCOUNT = "하나은행 915-910012-71304 주식회사 힘";

// =============================================================================
// Component
// =============================================================================

export function TextbookOrderPage() {
  const { t } = useTranslation();
  const { data: catalog, isLoading: catalogLoading } = useCatalog();
  const { data: userMe } = useUserMe();
  const createMutation = useCreateOrder();
  const [orderResult, setOrderResult] = useState<OrderRes | null>(null);
  const [termsOpen, setTermsOpen] = useState(false);
  const [termsAgreed, setTermsAgreed] = useState(false);
  const termsScrollRef = useRef<HTMLDivElement>(null);
  const pendingFormData = useRef<OrderFormValues | null>(null);

  const form = useForm<OrderFormValues>({
    resolver: zodResolver(orderFormSchema),
    mode: "onChange",
    defaultValues: {
      orderer_name: "",
      orderer_email: "",
      orderer_phone: "",
      org_name: "",
      org_type: "",
      delivery_postal_code: "",
      delivery_address: "",
      delivery_detail: "",
      depositor_name: "",
      tax_invoice: false,
      tax_biz_number: "",
      tax_company_name: "",
      tax_rep_name: "",
      tax_address: "",
      tax_biz_type: "",
      tax_biz_item: "",
      tax_email: "",
      items: [{ language: "", textbook_type: "student", quantity: 10 }],
      notes: "",
    },
  });

  const { fields, append, remove } = useFieldArray({
    control: form.control,
    name: "items",
  });

  const [searchParams] = useSearchParams();

  // 카탈로그에서 선택한 항목 자동 추가
  useEffect(() => {
    const lang = searchParams.get("lang");
    const type = searchParams.get("type");
    if (lang && (type === "student" || type === "teacher")) {
      form.setValue("items", [{ language: lang, textbook_type: type, quantity: 10 }]);
    }
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  // 로그인 사용자 정보 자동 채움
  useEffect(() => {
    if (userMe) {
      const currentName = form.getValues("orderer_name");
      const currentEmail = form.getValues("orderer_email");
      if (!currentName && userMe.name) {
        form.setValue("orderer_name", userMe.name);
      }
      if (!currentEmail && userMe.email) {
        form.setValue("orderer_email", userMe.email);
      }
    }
  }, [userMe]); // eslint-disable-line react-hooks/exhaustive-deps

  const watchItems = form.watch("items");
  const watchTaxInvoice = form.watch("tax_invoice");

  const totalQuantity = watchItems.reduce(
    (sum, item) => sum + (Number(item.quantity) || 0),
    0,
  );
  const totalAmount = totalQuantity * UNIT_PRICE;

  // 이미 선택된 language+type 조합 (중복 방지용)
  const usedCombinations = new Set(
    watchItems.map((item) => `${item.language}:${item.textbook_type}`),
  );

  // ISBN 미발급 언어 포함 여부
  const hasIsbnPending = watchItems.some((item) => {
    const cat = catalogItems.find((c) => c.language === item.language);
    return cat && !cat.isbn_ready;
  });

  // 폼 제출 → 검증 → 약관 모달 표시
  const onSubmit = (values: OrderFormValues) => {
    if (totalQuantity < MIN_TOTAL_QUANTITY) {
      toast.error(
        t("textbook.order.minQuantityError", { min: MIN_TOTAL_QUANTITY }),
      );
      return;
    }

    // 중복 항목 체크
    const seen = new Set<string>();
    for (const item of values.items) {
      const key = `${item.language}:${item.textbook_type}`;
      if (seen.has(key)) {
        toast.error(t("textbook.order.duplicateItemError"));
        return;
      }
      seen.add(key);
    }

    // 검증 통과 → 폼 데이터 저장 후 약관 모달 표시
    pendingFormData.current = values;
    setTermsAgreed(false);
    setTermsOpen(true);
  };

  // 약관 동의 후 실제 주문 실행
  const executeOrder = () => {
    const values = pendingFormData.current;
    if (!values) return;
    setTermsOpen(false);

    const data = {
      ...values,
      org_name: values.org_name || undefined,
      org_type: values.org_type || undefined,
      delivery_postal_code: values.delivery_postal_code || undefined,
      delivery_detail: values.delivery_detail || undefined,
      depositor_name: values.depositor_name || undefined,
      tax_biz_number: values.tax_biz_number || undefined,
      tax_company_name: values.tax_company_name || undefined,
      tax_rep_name: values.tax_rep_name || undefined,
      tax_address: values.tax_address || undefined,
      tax_biz_type: values.tax_biz_type || undefined,
      tax_biz_item: values.tax_biz_item || undefined,
      tax_email: values.tax_email || undefined,
      notes: values.notes || undefined,
      payment_method: "bank_transfer" as const,
      items: values.items.map((item) => ({
        language: item.language as TextbookLanguage,
        textbook_type: item.textbook_type as TextbookType,
        quantity: Number(item.quantity),
      })),
    };

    createMutation.mutate(data, {
      onSuccess: (result) => {
        setOrderResult(result);
        toast.success(t("textbook.order.successToast"));
        window.scrollTo({ top: 0, behavior: "smooth" });
      },
      onError: () => {
        toast.error(t("textbook.order.errorToast"));
      },
    });
  };

  const copyOrderCode = () => {
    if (orderResult?.order_code) {
      navigator.clipboard.writeText(orderResult.order_code);
      toast.success(t("textbook.order.codeCopied"));
    }
  };

  // =========================================================================
  // 주문 완료 화면
  // =========================================================================

  if (orderResult) {
    return (
      <div className="flex flex-col">
        <PageMeta
          titleKey="seo.textbook.title"
          descriptionKey="seo.textbook.description"
        />
        <div className="max-w-2xl mx-auto px-6 py-20">
          <Card>
            <CardHeader className="text-center">
              <div className="mx-auto w-16 h-16 rounded-full bg-status-success/10 flex items-center justify-center mb-4">
                <CheckCircle2 className="h-8 w-8 text-status-success" />
              </div>
              <CardTitle className="text-2xl">
                {t("textbook.order.completeTitle")}
              </CardTitle>
              <p className="text-muted-foreground mt-2">
                {t("textbook.order.completeDesc")}
              </p>
            </CardHeader>
            <CardContent className="space-y-6">
              {/* 주문번호 */}
              <div className="p-4 rounded-lg bg-muted/50 border border-border text-center">
                <p className="text-sm text-muted-foreground mb-1">
                  {t("textbook.order.orderCode")}
                </p>
                <div className="flex items-center justify-center gap-2">
                  <span className="text-2xl font-bold font-mono">
                    {orderResult.order_code}
                  </span>
                  <Button variant="ghost" size="icon" onClick={copyOrderCode}>
                    <Copy className="h-4 w-4" />
                  </Button>
                </div>
              </div>

              {/* 주문 요약 */}
              <div className="space-y-2 text-sm">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">
                    {t("textbook.order.totalQuantity")}
                  </span>
                  <span className="font-medium">
                    {orderResult.total_quantity}
                    {t("textbook.order.unit")}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">
                    {t("textbook.order.totalAmount")}
                  </span>
                  <span className="font-bold text-lg">
                    {orderResult.total_amount.toLocaleString()}
                    {t("textbook.order.currency")}
                  </span>
                </div>
              </div>

              {/* 입금 안내 */}
              <div className="p-4 rounded-lg border border-status-warning/30 bg-status-warning/5">
                <p className="font-semibold mb-2">
                  {t("textbook.order.bankTransferGuide")}
                </p>
                <p className="text-sm text-muted-foreground whitespace-pre-line">
                  {t("textbook.order.bankAccount", { account: BANK_ACCOUNT })}
                </p>
                <p className="text-sm text-muted-foreground mt-2">
                  {t("textbook.order.bankNote")}
                </p>
              </div>

              {/* 주문 조회 안내 */}
              <div className="text-center space-y-3">
                <p className="text-sm text-muted-foreground">
                  {t("textbook.order.trackGuide")}
                </p>
                <div className="flex flex-wrap justify-center gap-2">
                  <Button
                    variant="outline"
                    onClick={() =>
                      (window.location.href = `/textbook/order/${orderResult.order_code}`)
                    }
                  >
                    {t("textbook.order.trackButton")}
                  </Button>
                  <Button
                    variant="outline"
                    onClick={() =>
                      window.open(
                        `/textbook/order/${orderResult.order_code}/print?type=quote`,
                        "_blank",
                      )
                    }
                  >
                    <FileText className="h-4 w-4 mr-1" />
                    {t("textbook.print.quoteTitle")}
                  </Button>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    );
  }

  // =========================================================================
  // 로딩
  // =========================================================================

  if (catalogLoading) {
    return (
      <div className="max-w-4xl mx-auto px-6 py-20 space-y-6">
        <Skeleton className="h-10 w-64 mx-auto" />
        <Skeleton className="h-6 w-96 mx-auto" />
        <Skeleton className="h-96 rounded-2xl" />
      </div>
    );
  }

  const catalogItems = catalog?.items ?? [];

  // =========================================================================
  // 주문 폼
  // =========================================================================

  return (
    <div className="flex flex-col">
      <PageMeta
        titleKey="seo.textbook.title"
        descriptionKey="seo.textbook.description"
      />
      <HeroSection
        size="sm"
        badge={
          <>
            <BookOpen className="h-4 w-4 text-primary" />
            <span className="text-sm text-muted-foreground">
              {t("textbook.heroBadge")}
            </span>
          </>
        }
        title={t("textbook.heroTitle")}
        subtitle={t("textbook.heroSubtitle")}
      />

      <section className="py-12 lg:py-16">
        <div className="max-w-4xl mx-auto px-6 lg:px-8">
          <Form {...form}>
            <form
              onSubmit={form.handleSubmit(onSubmit)}
              className="space-y-8"
            >
              {/* ─── 1. 교재 선택 ─── */}
              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    <Package className="h-5 w-5" />
                    {t("textbook.order.sectionItems")}
                  </CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  {fields.map((field, index) => (
                    <div
                      key={field.id}
                      className="flex flex-col sm:flex-row gap-3 p-4 rounded-lg border border-border bg-muted/30"
                    >
                      {/* 표지 썸네일 */}
                      {watchItems[index]?.language && (
                        <div className="hidden sm:flex items-center">
                          <img
                            src={`/covers/${watchItems[index]?.textbook_type ?? "student"}-${watchItems[index]?.language}.webp`}
                            alt=""
                            className="w-12 h-16 rounded object-cover border"
                          />
                        </div>
                      )}
                      {/* 언어 */}
                      <FormField
                        control={form.control}
                        name={`items.${index}.language`}
                        render={({ field: f }) => (
                          <FormItem className="flex-1">
                            <FormLabel>
                              {t("textbook.order.language")}
                            </FormLabel>
                            <Select
                              onValueChange={f.onChange}
                              value={f.value}
                            >
                              <FormControl>
                                <SelectTrigger>
                                  <SelectValue
                                    placeholder={t(
                                      "textbook.order.languagePlaceholder",
                                    )}
                                  />
                                </SelectTrigger>
                              </FormControl>
                              <SelectContent>
                                {catalogItems.map((cat) => {
                                  const currentType = watchItems[index]?.textbook_type ?? "student";
                                  const dupKey = `${cat.language}:${currentType}`;
                                  const isUsedByOther =
                                    usedCombinations.has(dupKey) &&
                                    watchItems[index]?.language !== cat.language;
                                  return (
                                    <SelectItem
                                      key={cat.language}
                                      value={cat.language}
                                      disabled={isUsedByOther || !cat.available}
                                    >
                                      {cat.language_name_ko} (
                                      {cat.language_name_en})
                                    </SelectItem>
                                  );
                                })}
                              </SelectContent>
                            </Select>
                            <FormMessage />
                          </FormItem>
                        )}
                      />

                      {/* 유형 */}
                      <FormField
                        control={form.control}
                        name={`items.${index}.textbook_type`}
                        render={({ field: f }) => (
                          <FormItem className="w-full sm:w-40">
                            <FormLabel>
                              {t("textbook.order.textbookType")}
                            </FormLabel>
                            <Select
                              onValueChange={f.onChange}
                              value={f.value}
                            >
                              <FormControl>
                                <SelectTrigger>
                                  <SelectValue />
                                </SelectTrigger>
                              </FormControl>
                              <SelectContent>
                                <SelectItem value="student">
                                  {t("textbook.order.typeStudent")}
                                </SelectItem>
                                <SelectItem value="teacher">
                                  {t("textbook.order.typeTeacher")}
                                </SelectItem>
                              </SelectContent>
                            </Select>
                            <FormMessage />
                          </FormItem>
                        )}
                      />

                      {/* 수량 */}
                      <FormField
                        control={form.control}
                        name={`items.${index}.quantity`}
                        render={({ field: f }) => (
                          <FormItem className="w-full sm:w-28">
                            <FormLabel>
                              {t("textbook.order.quantity")}
                            </FormLabel>
                            <FormControl>
                              <Input
                                type="number"
                                min={1}
                                {...f}
                                onChange={(e) =>
                                  f.onChange(Number(e.target.value))
                                }
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />

                      {/* 소계 */}
                      <div className="flex items-end gap-2">
                        <div className="text-sm font-medium whitespace-nowrap pb-2">
                          {(
                            (Number(watchItems[index]?.quantity) || 0) *
                            UNIT_PRICE
                          ).toLocaleString()}
                          {t("textbook.order.currency")}
                        </div>
                        {fields.length > 1 && (
                          <Button
                            type="button"
                            variant="ghost"
                            size="icon"
                            className="text-destructive hover:text-destructive"
                            onClick={() => remove(index)}
                          >
                            <Trash2 className="h-4 w-4" />
                          </Button>
                        )}
                      </div>
                    </div>
                  ))}

                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() =>
                      append({
                        language: "",
                        textbook_type: "student",
                        quantity: 10,
                      })
                    }
                  >
                    <Plus className="h-4 w-4 mr-1" />
                    {t("textbook.order.addItem")}
                  </Button>

                  {/* 합계 */}
                  <div className="flex justify-between items-center p-4 rounded-lg bg-muted/50 border border-border">
                    <div>
                      <span className="text-sm text-muted-foreground">
                        {t("textbook.order.totalQuantity")}:{" "}
                      </span>
                      <span
                        className={`font-bold ${totalQuantity < MIN_TOTAL_QUANTITY ? "text-destructive" : ""}`}
                      >
                        {totalQuantity}
                        {t("textbook.order.unit")}
                      </span>
                      {totalQuantity < MIN_TOTAL_QUANTITY && (
                        <span className="text-sm text-destructive ml-2">
                          ({t("textbook.order.minQuantityHint", { min: MIN_TOTAL_QUANTITY })})
                        </span>
                      )}
                    </div>
                    <div className="text-right">
                      <span className="text-sm text-muted-foreground">
                        {t("textbook.order.totalAmount")}:{" "}
                      </span>
                      <span className="text-xl font-bold">
                        {totalAmount.toLocaleString()}
                        {t("textbook.order.currency")}
                      </span>
                    </div>
                  </div>

                  {/* ISBN 미발급 언어 안내 */}
                  {hasIsbnPending && (
                    <p className="text-sm text-muted-foreground bg-muted rounded-lg p-3">
                      {t("textbook.order.isbnNotice")}
                    </p>
                  )}
                </CardContent>
              </Card>

              {/* ─── 2. 신청자 정보 ─── */}
              <Card>
                <CardHeader>
                  <CardTitle>{t("textbook.order.sectionOrderer")}</CardTitle>
                </CardHeader>
                <CardContent className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <FormField
                    control={form.control}
                    name="orderer_name"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>
                          {t("textbook.order.ordererName")} *
                        </FormLabel>
                        <FormControl>
                          <Input
                            placeholder={t(
                              "textbook.order.ordererNamePlaceholder",
                            )}
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="orderer_phone"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>
                          {t("textbook.order.ordererPhone")} *
                        </FormLabel>
                        <FormControl>
                          <Input
                            type="tel"
                            placeholder="010-1234-5678"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="orderer_email"
                    render={({ field }) => (
                      <FormItem className="sm:col-span-2">
                        <FormLabel>
                          {t("textbook.order.ordererEmail")} *
                        </FormLabel>
                        <FormControl>
                          <Input
                            type="email"
                            placeholder="example@email.com"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </CardContent>
              </Card>

              {/* ─── 3. 기관 정보 (선택) ─── */}
              <Card>
                <CardHeader>
                  <CardTitle>
                    {t("textbook.order.sectionOrg")}
                    <span className="text-sm font-normal text-muted-foreground ml-2">
                      ({t("textbook.order.optional")})
                    </span>
                  </CardTitle>
                </CardHeader>
                <CardContent className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <FormField
                    control={form.control}
                    name="org_name"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>{t("textbook.order.orgName")}</FormLabel>
                        <FormControl>
                          <Input
                            placeholder={t(
                              "textbook.order.orgNamePlaceholder",
                            )}
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="org_type"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>{t("textbook.order.orgType")}</FormLabel>
                        <Select
                          onValueChange={field.onChange}
                          value={field.value}
                        >
                          <FormControl>
                            <SelectTrigger>
                              <SelectValue
                                placeholder={t(
                                  "textbook.order.orgTypePlaceholder",
                                )}
                              />
                            </SelectTrigger>
                          </FormControl>
                          <SelectContent>
                            <SelectItem value="academy">
                              {t("textbook.order.orgTypeAcademy")}
                            </SelectItem>
                            <SelectItem value="university">
                              {t("textbook.order.orgTypeUniversity")}
                            </SelectItem>
                            <SelectItem value="school">
                              {t("textbook.order.orgTypeSchool")}
                            </SelectItem>
                            <SelectItem value="individual">
                              {t("textbook.order.orgTypeIndividual")}
                            </SelectItem>
                            <SelectItem value="other">
                              {t("textbook.order.orgTypeOther")}
                            </SelectItem>
                          </SelectContent>
                        </Select>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </CardContent>
              </Card>

              {/* ─── 4. 배송 정보 ─── */}
              <Card>
                <CardHeader>
                  <CardTitle>{t("textbook.order.sectionDelivery")}</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  <FormField
                    control={form.control}
                    name="delivery_postal_code"
                    render={({ field }) => (
                      <FormItem className="max-w-xs">
                        <FormLabel>
                          {t("textbook.order.postalCode")}
                        </FormLabel>
                        <FormControl>
                          <Input placeholder="12345" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="delivery_address"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>
                          {t("textbook.order.address")} *
                        </FormLabel>
                        <FormControl>
                          <Input
                            placeholder={t(
                              "textbook.order.addressPlaceholder",
                            )}
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="delivery_detail"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>
                          {t("textbook.order.addressDetail")}
                        </FormLabel>
                        <FormControl>
                          <Input
                            placeholder={t(
                              "textbook.order.addressDetailPlaceholder",
                            )}
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </CardContent>
              </Card>

              {/* ─── 5. 결제 정보 ─── */}
              <Card>
                <CardHeader>
                  <CardTitle>{t("textbook.order.sectionPayment")}</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div className="p-3 rounded-lg bg-muted/50 border border-border text-sm">
                    {t("textbook.order.paymentMethodNote")}
                  </div>
                  <FormField
                    control={form.control}
                    name="depositor_name"
                    render={({ field }) => (
                      <FormItem className="max-w-sm">
                        <FormLabel>
                          {t("textbook.order.depositorName")}
                        </FormLabel>
                        <FormControl>
                          <Input
                            placeholder={t(
                              "textbook.order.depositorNamePlaceholder",
                            )}
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </CardContent>
              </Card>

              {/* ─── 6. 세금계산서 ─── */}
              <Card>
                <CardHeader>
                  <CardTitle>{t("textbook.order.sectionTax")}</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  <FormField
                    control={form.control}
                    name="tax_invoice"
                    render={({ field }) => (
                      <FormItem className="flex items-center gap-2">
                        <FormControl>
                          <Checkbox
                            checked={field.value}
                            onCheckedChange={field.onChange}
                            ref={field.ref}
                          />
                        </FormControl>
                        <FormLabel className="text-sm font-normal cursor-pointer !mt-0">
                          {t("textbook.order.taxInvoiceLabel")}
                        </FormLabel>
                      </FormItem>
                    )}
                  />

                  {watchTaxInvoice && (
                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                      <FormField
                        control={form.control}
                        name="tax_biz_number"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>
                              {t("textbook.order.taxBizNumber")} *
                            </FormLabel>
                            <FormControl>
                              <Input
                                placeholder="000-00-00000"
                                {...field}
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="tax_company_name"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>
                              {t("textbook.order.taxCompanyName")} *
                            </FormLabel>
                            <FormControl>
                              <Input
                                placeholder={t("textbook.order.taxCompanyNamePlaceholder")}
                                {...field}
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="tax_rep_name"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>
                              {t("textbook.order.taxRepName")} *
                            </FormLabel>
                            <FormControl>
                              <Input
                                placeholder={t("textbook.order.taxRepNamePlaceholder")}
                                {...field}
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="tax_email"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>
                              {t("textbook.order.taxEmail")} *
                            </FormLabel>
                            <FormControl>
                              <Input
                                type="email"
                                placeholder="tax@company.com"
                                {...field}
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="tax_address"
                        render={({ field }) => (
                          <FormItem className="sm:col-span-2">
                            <FormLabel>
                              {t("textbook.order.taxAddress")}
                            </FormLabel>
                            <FormControl>
                              <Input
                                placeholder={t("textbook.order.taxAddressPlaceholder")}
                                {...field}
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="tax_biz_type"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>
                              {t("textbook.order.taxBizType")}
                            </FormLabel>
                            <FormControl>
                              <Input
                                placeholder={t("textbook.order.taxBizTypePlaceholder")}
                                {...field}
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="tax_biz_item"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>
                              {t("textbook.order.taxBizItem")}
                            </FormLabel>
                            <FormControl>
                              <Input
                                placeholder={t("textbook.order.taxBizItemPlaceholder")}
                                {...field}
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>
                  )}
                </CardContent>
              </Card>

              {/* ─── 7. 비고 ─── */}
              <Card>
                <CardHeader>
                  <CardTitle>
                    {t("textbook.order.sectionNotes")}
                    <span className="text-sm font-normal text-muted-foreground ml-2">
                      ({t("textbook.order.optional")})
                    </span>
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <FormField
                    control={form.control}
                    name="notes"
                    render={({ field }) => (
                      <FormItem>
                        <FormControl>
                          <Textarea
                            placeholder={t(
                              "textbook.order.notesPlaceholder",
                            )}
                            rows={3}
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </CardContent>
              </Card>

              {/* ─── Submit ─── */}
              <Button
                type="submit"
                size="lg"
                className="w-full h-14 text-lg"
                disabled={
                  createMutation.isPending ||
                  totalQuantity < MIN_TOTAL_QUANTITY
                }
              >
                {createMutation.isPending ? (
                  <>
                    <Loader2 className="h-5 w-5 animate-spin mr-2" />
                    {t("textbook.order.submitting")}
                  </>
                ) : (
                  t("textbook.order.submitButton")
                )}
              </Button>
            </form>
          </Form>
        </div>
      </section>

      {/* ─── 약관 동의 모달 ─── */}
      <Dialog open={termsOpen} onOpenChange={setTermsOpen}>
        <DialogContent className="max-w-2xl max-h-[85vh] flex flex-col">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2">
              <ScrollText className="h-5 w-5" />
              {t("textbook.terms.title")}
            </DialogTitle>
            <DialogDescription className="sr-only">
              {t("textbook.terms.title")}
            </DialogDescription>
          </DialogHeader>

          <div
            ref={termsScrollRef}
            className="flex-1 overflow-y-auto pr-2 space-y-6 text-sm leading-relaxed"
          >
            {[1, 2, 3, 4, 5, 6].map((n) => (
              <div key={n}>
                <h3 className="font-semibold text-base mb-2">
                  {t(`textbook.terms.article${n}Title`)}
                </h3>
                <p className="text-muted-foreground whitespace-pre-line">
                  {t(`textbook.terms.article${n}Content`)}
                </p>
              </div>
            ))}

            <div className="border-t pt-4 space-y-1">
              <p className="text-muted-foreground">
                {t("textbook.terms.closing")}
              </p>
              <p className="font-semibold">
                {t("textbook.terms.publisher")}
              </p>
            </div>
          </div>

          <div className="border-t pt-4 space-y-4">
            <div className="flex items-center gap-2">
              <Checkbox
                id="terms-agree"
                checked={termsAgreed}
                onCheckedChange={(v) => setTermsAgreed(v === true)}
              />
              <label
                htmlFor="terms-agree"
                className="text-sm cursor-pointer select-none"
              >
                {t("textbook.terms.agreeLabel")}
              </label>
            </div>

            <DialogFooter className="gap-2 sm:gap-0">
              <Button
                variant="outline"
                onClick={() => setTermsOpen(false)}
              >
                {t("textbook.terms.declineButton")}
              </Button>
              <Button
                disabled={!termsAgreed || createMutation.isPending}
                onClick={executeOrder}
              >
                {createMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin mr-2" />
                    {t("textbook.order.submitting")}
                  </>
                ) : (
                  t("textbook.terms.agreeButton")
                )}
              </Button>
            </DialogFooter>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
