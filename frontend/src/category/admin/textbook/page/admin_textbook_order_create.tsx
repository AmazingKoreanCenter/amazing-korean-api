import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { ArrowLeft, Plus, Trash2 } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

import { useCatalog } from "@/category/textbook/hook/use_catalog";
import { useAdminCreateTextbookOrder } from "../hook/use_admin_textbook";
import type { AdminCreateOrderReq } from "../types";
import type {
  TextbookLanguage,
  TextbookType,
} from "@/category/textbook/types";
import { UserSearchCombobox } from "@/category/admin/components/user_search_combobox";
import type { AdminUserSummary } from "@/category/admin/types";

const UNIT_PRICE = 25_000;

type OrderItem = {
  language: TextbookLanguage | "";
  textbook_type: TextbookType;
  // controlled number input 에서 사용자가 전량 삭제 후 재입력할 수 있도록
  // 중간 상태로 "" 허용. onBlur/submit 에서 1 fallback.
  quantity: number | "";
};

export function AdminTextbookOrderCreate() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { data: catalog } = useCatalog();
  const createMutation = useAdminCreateTextbookOrder();

  // ---- 기본 폼 상태 ----
  const [ordererName, setOrdererName] = useState("");
  const [ordererEmail, setOrdererEmail] = useState("");
  const [ordererPhone, setOrdererPhone] = useState("");
  const [orgName, setOrgName] = useState("");
  const [orgType, setOrgType] = useState("");
  const [deliveryPostalCode, setDeliveryPostalCode] = useState("");
  const [deliveryAddress, setDeliveryAddress] = useState("");
  const [deliveryDetail, setDeliveryDetail] = useState("");
  const [depositorName, setDepositorName] = useState("");
  const [notes, setNotes] = useState("");

  // ---- 관리자 전용 옵션 ----
  const [initialStatus, setInitialStatus] = useState<
    "pending" | "confirmed" | "paid"
  >("paid");
  const [enforceMinQuantity, setEnforceMinQuantity] = useState(false);
  // 2026-04-23: 주문 모드 명시적 선택 — "guest" (비회원) / "member" (회원 귀속).
  // 비회원은 user_id 미전송, 회원은 검색 콤보박스 또는 수동 user_id 입력.
  const [orderMode, setOrderMode] = useState<"guest" | "member">("guest");
  // Q5: 검색 콤보박스로 user 선택. 수동 입력 토글 시 직접 user_id 입력.
  const [selectedUser, setSelectedUser] = useState<AdminUserSummary | null>(
    null,
  );
  const [manualUserIdMode, setManualUserIdMode] = useState(false);
  const [userId, setUserId] = useState("");

  // ---- 세금계산서 ----
  const [taxInvoice, setTaxInvoice] = useState(false);
  const [taxBizNumber, setTaxBizNumber] = useState("");
  const [taxCompanyName, setTaxCompanyName] = useState("");
  const [taxRepName, setTaxRepName] = useState("");
  const [taxAddress, setTaxAddress] = useState("");
  const [taxBizType, setTaxBizType] = useState("");
  const [taxBizItem, setTaxBizItem] = useState("");
  const [taxEmail, setTaxEmail] = useState("");

  // ---- 품목 ----
  const [items, setItems] = useState<OrderItem[]>([
    { language: "", textbook_type: "student", quantity: 1 },
  ]);

  // ---- 할인 (2026-04-23 신규) ----
  // discountAmount 는 number | "" 로 수량과 동일한 패턴. onBlur 에서 "" → 0 fallback.
  const [discountAmount, setDiscountAmount] = useState<number | "">(0);
  const [discountReason, setDiscountReason] = useState("");

  const addItem = () =>
    setItems((prev) => [
      ...prev,
      { language: "", textbook_type: "student", quantity: 1 },
    ]);
  const removeItem = (idx: number) =>
    setItems((prev) => prev.filter((_, i) => i !== idx));
  const updateItem = (idx: number, patch: Partial<OrderItem>) =>
    setItems((prev) =>
      prev.map((it, i) => (i === idx ? { ...it, ...patch } : it)),
    );

  const totalQuantity = items.reduce(
    (sum, it) => sum + (Number(it.quantity) || 0),
    0,
  );
  // grossAmount: 할인 전 총액 (수량 × 단가, VAT 포함)
  const grossAmount = totalQuantity * UNIT_PRICE;
  const discountNum = Math.max(0, Math.floor(Number(discountAmount) || 0));
  // UI 표시용 최종 금액. 할인이 gross 초과 시 0 으로 clamp 표시 (실제 submit 은 검증으로 거부).
  const discountApplied = Math.min(discountNum, grossAmount);
  const finalAmount = grossAmount - discountApplied;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // 기본 유효성 — 2026-04-23: 이메일은 optional, 이름·전화만 필수
    if (!ordererName.trim() || !ordererPhone.trim()) {
      toast.error(t("admin.textbook.create.err.ordererRequired"));
      return;
    }
    // 이메일이 입력됐다면 대략적 형식 검증 (백엔드가 validator crate 로 재검증)
    if (ordererEmail.trim() && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(ordererEmail.trim())) {
      toast.error(t("admin.textbook.create.err.emailInvalid"));
      return;
    }
    if (!deliveryAddress.trim()) {
      toast.error(t("admin.textbook.create.err.addressRequired"));
      return;
    }
    if (items.length === 0 || items.some((it) => !it.language)) {
      toast.error(t("admin.textbook.create.err.itemsRequired"));
      return;
    }
    // 할인 검증: 0 ≤ discount ≤ gross. 서버에서도 검증하지만 UX 개선 위해 선확인.
    if (discountNum < 0) {
      toast.error(t("admin.textbook.create.err.discountNegative"));
      return;
    }
    if (discountNum > grossAmount) {
      toast.error(t("admin.textbook.create.err.discountExceedsGross"));
      return;
    }
    if (taxInvoice) {
      if (
        !taxBizNumber.trim() ||
        !taxCompanyName.trim() ||
        !taxRepName.trim() ||
        !taxEmail.trim()
      ) {
        toast.error(t("admin.textbook.create.err.taxRequired"));
        return;
      }
    }

    // user_id 결정 — 2026-04-23: orderMode 기반.
    // guest 모드: user_id 전송 안 함.
    // member 모드: 검색 선택 우선, 없으면 수동 입력 파싱. 둘 다 없으면 검증 실패.
    let parsedUserId: number | undefined = undefined;
    if (orderMode === "member") {
      if (selectedUser) {
        parsedUserId = selectedUser.id;
      } else if (manualUserIdMode) {
        // 수동 입력 엄격 파싱: 양의 정수 문자열만 허용.
        // Number() 는 "1.5", "1e3" 등을 허용해 백엔드 i64 deserialize 실패를
        // 유발하고, NaN 을 JSON.stringify 가 null 로 변환하면 의도한 귀속이
        // silently 소실됨.
        const userIdInput = userId.trim();
        if (!userIdInput) {
          toast.error(t("admin.textbook.create.err.memberRequired"));
          return;
        }
        if (!/^\d+$/.test(userIdInput)) {
          toast.error(t("admin.textbook.create.err.userIdInvalid"));
          return;
        }
        parsedUserId = Number(userIdInput);
        if (!Number.isSafeInteger(parsedUserId) || parsedUserId <= 0) {
          toast.error(t("admin.textbook.create.err.userIdInvalid"));
          return;
        }
      } else {
        toast.error(t("admin.textbook.create.err.memberRequired"));
        return;
      }
    }

    const payload: AdminCreateOrderReq = {
      user_id: parsedUserId,
      initial_status: initialStatus,
      enforce_min_quantity: enforceMinQuantity,
      orderer_name: ordererName,
      orderer_email: ordererEmail.trim() || undefined,
      orderer_phone: ordererPhone,
      org_name: orgName.trim() || undefined,
      org_type: orgType.trim() || undefined,
      delivery_postal_code: deliveryPostalCode.trim() || undefined,
      delivery_address: deliveryAddress,
      delivery_detail: deliveryDetail.trim() || undefined,
      payment_method: "bank_transfer",
      depositor_name: depositorName.trim() || undefined,
      tax_invoice: taxInvoice,
      tax_biz_number: taxInvoice ? taxBizNumber : undefined,
      tax_company_name: taxInvoice ? taxCompanyName : undefined,
      tax_rep_name: taxInvoice ? taxRepName : undefined,
      tax_address: taxInvoice ? taxAddress || undefined : undefined,
      tax_biz_type: taxInvoice ? taxBizType || undefined : undefined,
      tax_biz_item: taxInvoice ? taxBizItem || undefined : undefined,
      tax_email: taxInvoice ? taxEmail : undefined,
      items: items.map((it) => ({
        language: it.language as TextbookLanguage,
        textbook_type: it.textbook_type,
        // quantity 는 백엔드 i32. 브라우저가 소수점 입력을 허용하는
        // 경우가 있으므로 정수 보장. "" (편집 중 빈 상태) 는 1 fallback.
        quantity: Math.max(1, Math.floor(Number(it.quantity) || 1)),
      })),
      discount_amount: discountNum > 0 ? discountNum : undefined,
      discount_reason:
        discountNum > 0 && discountReason.trim()
          ? discountReason.trim()
          : undefined,
      notes: notes.trim() || undefined,
    };

    createMutation.mutate(payload, {
      onSuccess: (order) => {
        toast.success(t("admin.textbook.create.success"));
        navigate(`/admin/textbook/orders/${order.order_id}`);
      },
      onError: (err: unknown) => {
        const message =
          err instanceof Error
            ? err.message
            : t("admin.textbook.create.err.generic");
        toast.error(message);
      },
    });
  };

  return (
    <div className="space-y-6">
      {/* 헤더 */}
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" asChild>
          <Link to="/admin/textbook/orders">
            <ArrowLeft className="h-5 w-5" />
          </Link>
        </Button>
        <div>
          <h1 className="text-2xl font-bold">
            {t("admin.textbook.create.title")}
          </h1>
          <p className="text-sm text-muted-foreground">
            {t("admin.textbook.create.subtitle")}
          </p>
        </div>
      </div>

      <form onSubmit={handleSubmit} className="space-y-6">
        {/* 관리자 전용 옵션 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base">
              {t("admin.textbook.create.adminOptions")}
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* 주문 모드 세그먼트: 비회원 / 회원 (2026-04-23 신규) */}
            <div>
              <Label>{t("admin.textbook.create.orderMode")}</Label>
              <div
                className="mt-1 inline-flex rounded-md border p-1 bg-muted"
                role="radiogroup"
                aria-label={t("admin.textbook.create.orderMode")}
              >
                <button
                  type="button"
                  role="radio"
                  aria-checked={orderMode === "guest"}
                  onClick={() => {
                    setOrderMode("guest");
                    setSelectedUser(null);
                    setUserId("");
                    setManualUserIdMode(false);
                  }}
                  className={`px-4 py-1.5 text-sm rounded ${
                    orderMode === "guest"
                      ? "bg-background shadow font-medium"
                      : "text-muted-foreground hover:text-foreground"
                  }`}
                >
                  {t("admin.textbook.create.guestOrder")}
                </button>
                <button
                  type="button"
                  role="radio"
                  aria-checked={orderMode === "member"}
                  onClick={() => setOrderMode("member")}
                  className={`px-4 py-1.5 text-sm rounded ${
                    orderMode === "member"
                      ? "bg-background shadow font-medium"
                      : "text-muted-foreground hover:text-foreground"
                  }`}
                >
                  {t("admin.textbook.create.memberOrder")}
                </button>
              </div>
              <p className="text-xs text-muted-foreground mt-1">
                {orderMode === "guest"
                  ? t("admin.textbook.create.guestOrderHint")
                  : t("admin.textbook.create.memberOrderHint")}
              </p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <Label htmlFor="initialStatus">
                  {t("admin.textbook.create.initialStatus")}
                </Label>
                <Select
                  value={initialStatus}
                  onValueChange={(v) =>
                    setInitialStatus(v as "pending" | "confirmed" | "paid")
                  }
                >
                  <SelectTrigger id="initialStatus" className="mt-1">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="pending">
                      {t("admin.textbook.status.pending")}
                    </SelectItem>
                    <SelectItem value="confirmed">
                      {t("admin.textbook.status.confirmed")}
                    </SelectItem>
                    <SelectItem value="paid">
                      {t("admin.textbook.status.paid")}
                    </SelectItem>
                  </SelectContent>
                </Select>
                <p className="text-xs text-muted-foreground mt-1">
                  {t("admin.textbook.create.initialStatusHint")}
                </p>
              </div>

              {orderMode === "member" && (
                <div>
                  <div className="flex items-center justify-between">
                    <Label htmlFor="userId">
                      {t("admin.textbook.create.userId")} *
                    </Label>
                    <button
                      type="button"
                      className="text-xs text-primary hover:underline"
                      onClick={() => {
                        setManualUserIdMode((prev) => !prev);
                        setSelectedUser(null);
                        setUserId("");
                      }}
                    >
                      {manualUserIdMode
                        ? t("admin.textbook.create.userSearch.toggleSearch")
                        : t("admin.textbook.create.userSearch.toggleManual")}
                    </button>
                  </div>
                  <div className="mt-1">
                    {manualUserIdMode ? (
                      <Input
                        id="userId"
                        type="number" dir="ltr"
                        value={userId}
                        onChange={(e) => setUserId(e.target.value)}
                        placeholder={t("admin.textbook.create.userIdPlaceholder")}
                      />
                    ) : (
                      <UserSearchCombobox
                        value={selectedUser}
                        onChange={setSelectedUser}
                      />
                    )}
                  </div>
                  <p className="text-xs text-muted-foreground mt-1">
                    {manualUserIdMode
                      ? t("admin.textbook.create.userIdHint")
                      : t("admin.textbook.create.userSearch.hint")}
                  </p>
                </div>
              )}

              <div className="flex items-end">
                <label className="flex items-center gap-2 pb-1">
                  <Checkbox
                    checked={enforceMinQuantity}
                    onCheckedChange={(v) => setEnforceMinQuantity(v === true)}
                  />
                  <span className="text-sm">
                    {t("admin.textbook.create.enforceMinQty")}
                  </span>
                </label>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* 신청자 정보 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base">
              {t("admin.textbook.create.orderer")}
            </CardTitle>
          </CardHeader>
          <CardContent className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <Label htmlFor="ordererName">
                {t("admin.textbook.create.name")} *
              </Label>
              <Input
                id="ordererName"
                value={ordererName}
                onChange={(e) => setOrdererName(e.target.value)}
                required
              />
            </div>
            <div>
              <Label htmlFor="ordererEmail">
                {t("admin.textbook.create.email")}
              </Label>
              <Input
                id="ordererEmail"
                type="email" dir="ltr"
                value={ordererEmail}
                onChange={(e) => setOrdererEmail(e.target.value)}
              />
              <p className="text-xs text-muted-foreground mt-1">
                {t("admin.textbook.create.emailOptional")}
              </p>
            </div>
            <div>
              <Label htmlFor="ordererPhone">
                {t("admin.textbook.create.phone")} *
              </Label>
              <Input
                id="ordererPhone"
                value={ordererPhone}
                onChange={(e) => setOrdererPhone(e.target.value)}
                required
              />
            </div>
            <div>
              <Label htmlFor="orgName">
                {t("admin.textbook.create.orgName")}
              </Label>
              <Input
                id="orgName"
                value={orgName}
                onChange={(e) => setOrgName(e.target.value)}
              />
            </div>
            <div>
              <Label htmlFor="orgType">
                {t("admin.textbook.create.orgType")}
              </Label>
              <Input
                id="orgType"
                value={orgType}
                onChange={(e) => setOrgType(e.target.value)}
              />
            </div>
            <div>
              <Label htmlFor="depositorName">
                {t("admin.textbook.create.depositorName")}
              </Label>
              <Input
                id="depositorName"
                value={depositorName}
                onChange={(e) => setDepositorName(e.target.value)}
              />
            </div>
          </CardContent>
        </Card>

        {/* 배송 정보 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base">
              {t("admin.textbook.create.delivery")}
            </CardTitle>
          </CardHeader>
          <CardContent className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <Label htmlFor="deliveryPostalCode">
                {t("admin.textbook.create.postalCode")}
              </Label>
              <Input
                id="deliveryPostalCode"
                value={deliveryPostalCode}
                onChange={(e) => setDeliveryPostalCode(e.target.value)}
              />
            </div>
            <div>
              <Label htmlFor="deliveryAddress">
                {t("admin.textbook.create.address")} *
              </Label>
              <Input
                id="deliveryAddress"
                value={deliveryAddress}
                onChange={(e) => setDeliveryAddress(e.target.value)}
                required
              />
            </div>
            <div className="md:col-span-2">
              <Label htmlFor="deliveryDetail">
                {t("admin.textbook.create.addressDetail")}
              </Label>
              <Input
                id="deliveryDetail"
                value={deliveryDetail}
                onChange={(e) => setDeliveryDetail(e.target.value)}
              />
            </div>
          </CardContent>
        </Card>

        {/* 품목 */}
        <Card>
          <CardHeader className="flex flex-row items-center justify-between">
            <CardTitle className="text-base">
              {t("admin.textbook.create.items")}
            </CardTitle>
            <Button type="button" variant="outline" size="sm" onClick={addItem}>
              <Plus className="h-4 w-4 me-1" />
              {t("admin.textbook.create.addItem")}
            </Button>
          </CardHeader>
          <CardContent className="space-y-3">
            {items.map((item, idx) => (
              <div
                key={idx}
                className="grid grid-cols-1 md:grid-cols-[2fr_1fr_1fr_auto] gap-3 items-end"
              >
                <div>
                  <Label>{t("admin.textbook.create.language")} *</Label>
                  <Select
                    value={item.language}
                    onValueChange={(v) =>
                      updateItem(idx, {
                        language: v as TextbookLanguage,
                      })
                    }
                  >
                    <SelectTrigger>
                      <SelectValue
                        placeholder={t(
                          "admin.textbook.create.languagePlaceholder",
                        )}
                      />
                    </SelectTrigger>
                    <SelectContent>
                      {catalog?.items
                        ?.filter((c) => c.available)
                        .map((c) => (
                          <SelectItem key={c.language} value={c.language}>
                            {c.language_name_ko} ({c.language_name_en})
                          </SelectItem>
                        ))}
                    </SelectContent>
                  </Select>
                </div>
                <div>
                  <Label>{t("admin.textbook.create.bookType")}</Label>
                  <Select
                    value={item.textbook_type}
                    onValueChange={(v) =>
                      updateItem(idx, {
                        textbook_type: v as TextbookType,
                      })
                    }
                  >
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="student">
                        {t("admin.textbook.typeStudent")}
                      </SelectItem>
                      <SelectItem value="teacher">
                        {t("admin.textbook.typeTeacher")}
                      </SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                <div>
                  <Label>{t("admin.textbook.create.quantity")} *</Label>
                  <Input
                    type="number" dir="ltr"
                    min={1}
                    value={item.quantity}
                    onChange={(e) => {
                      const v = e.target.value;
                      // 빈 문자열도 잠시 허용해야 사용자가 "1" 을 지우고
                      // 다른 숫자를 입력할 수 있음. 숫자 입력 시에만 정수 보정.
                      updateItem(idx, {
                        quantity:
                          v === ""
                            ? ""
                            : Math.max(1, Math.floor(Number(v) || 1)),
                      });
                    }}
                    onBlur={() => {
                      // 포커스 이탈 시 빈 값은 1 로 복귀.
                      if (item.quantity === "") {
                        updateItem(idx, { quantity: 1 });
                      }
                    }}
                  />
                </div>
                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  onClick={() => removeItem(idx)}
                  disabled={items.length === 1}
                >
                  <Trash2 className="h-4 w-4 text-destructive" />
                </Button>
              </div>
            ))}
            <div className="pt-3 border-t text-sm flex justify-between">
              <span>
                {t("admin.textbook.create.totalQuantity")}:{" "}
                <strong>{totalQuantity}</strong>
              </span>
              <span>
                {t("admin.textbook.create.grossAmount")}:{" "}
                <strong>{grossAmount.toLocaleString()} KRW</strong>
              </span>
            </div>
          </CardContent>
        </Card>

        {/* 할인 (관리자 임의 입력) — 2026-04-23 신규 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base">
              {t("admin.textbook.create.discount")}
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <Label htmlFor="discountAmount">
                  {t("admin.textbook.create.discountAmount")} (KRW)
                </Label>
                <Input
                  id="discountAmount"
                  type="number" dir="ltr"
                  min={0}
                  max={grossAmount}
                  value={discountAmount}
                  onChange={(e) => {
                    const v = e.target.value;
                    if (v === "") {
                      setDiscountAmount("");
                    } else {
                      setDiscountAmount(
                        Math.max(0, Math.floor(Number(v) || 0)),
                      );
                    }
                  }}
                  onBlur={() => {
                    if (discountAmount === "") setDiscountAmount(0);
                  }}
                  placeholder="0"
                />
                <p className="text-xs text-muted-foreground mt-1">
                  {t("admin.textbook.create.discountHint")}
                </p>
              </div>
              <div>
                <Label htmlFor="discountReason">
                  {t("admin.textbook.create.discountReason")}
                </Label>
                <Input
                  id="discountReason"
                  value={discountReason}
                  onChange={(e) => setDiscountReason(e.target.value)}
                  placeholder={t(
                    "admin.textbook.create.discountReasonPlaceholder",
                  )}
                  maxLength={500}
                  disabled={discountNum === 0}
                />
              </div>
            </div>
            {/* 요약: 품목 합계 / 할인 / 최종 합계 */}
            <div className="pt-3 border-t text-sm space-y-1">
              <div className="flex justify-between">
                <span>{t("admin.textbook.create.grossAmount")}</span>
                <span>{grossAmount.toLocaleString()} KRW</span>
              </div>
              {discountNum > 0 && (
                <div className="flex justify-between text-destructive">
                  <span>- {t("admin.textbook.create.discountAmount")}</span>
                  <span>- {discountApplied.toLocaleString()} KRW</span>
                </div>
              )}
              <div className="flex justify-between pt-1 border-t font-semibold">
                <span>{t("admin.textbook.create.finalAmount")}</span>
                <span>{finalAmount.toLocaleString()} KRW</span>
              </div>
              {discountNum > grossAmount && (
                <p className="text-xs text-destructive mt-1">
                  {t("admin.textbook.create.err.discountExceedsGross")}
                </p>
              )}
            </div>
          </CardContent>
        </Card>

        {/* 세금계산서 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center gap-2">
              <Checkbox
                checked={taxInvoice}
                onCheckedChange={(v) => setTaxInvoice(v === true)}
                id="taxInvoice"
              />
              <Label htmlFor="taxInvoice" className="cursor-pointer">
                {t("admin.textbook.create.taxInvoice")}
              </Label>
            </CardTitle>
          </CardHeader>
          {taxInvoice && (
            <CardContent className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <Label>{t("admin.textbook.create.bizNumber")} *</Label>
                <Input
                  value={taxBizNumber}
                  onChange={(e) => setTaxBizNumber(e.target.value)}
                />
              </div>
              <div>
                <Label>{t("admin.textbook.create.companyName")} *</Label>
                <Input
                  value={taxCompanyName}
                  onChange={(e) => setTaxCompanyName(e.target.value)}
                />
              </div>
              <div>
                <Label>{t("admin.textbook.create.repName")} *</Label>
                <Input
                  value={taxRepName}
                  onChange={(e) => setTaxRepName(e.target.value)}
                />
              </div>
              <div>
                <Label>{t("admin.textbook.create.taxEmail")} *</Label>
                <Input
                  type="email" dir="ltr"
                  value={taxEmail}
                  onChange={(e) => setTaxEmail(e.target.value)}
                />
              </div>
              <div>
                <Label>{t("admin.textbook.create.taxAddress")}</Label>
                <Input
                  value={taxAddress}
                  onChange={(e) => setTaxAddress(e.target.value)}
                />
              </div>
              <div>
                <Label>{t("admin.textbook.create.taxBizType")}</Label>
                <Input
                  value={taxBizType}
                  onChange={(e) => setTaxBizType(e.target.value)}
                />
              </div>
              <div>
                <Label>{t("admin.textbook.create.taxBizItem")}</Label>
                <Input
                  value={taxBizItem}
                  onChange={(e) => setTaxBizItem(e.target.value)}
                />
              </div>
            </CardContent>
          )}
        </Card>

        {/* 비고 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base">
              {t("admin.textbook.create.notes")}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <Textarea
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              placeholder={t("admin.textbook.create.notesPlaceholder")}
              rows={3}
            />
          </CardContent>
        </Card>

        {/* 제출 */}
        <div className="flex justify-end gap-2">
          <Button variant="outline" type="button" asChild>
            <Link to="/admin/textbook/orders">
              {t("admin.textbook.create.cancel")}
            </Link>
          </Button>
          <Button type="submit" disabled={createMutation.isPending}>
            {createMutation.isPending
              ? t("admin.textbook.create.submitting")
              : t("admin.textbook.create.submit")}
          </Button>
        </div>
      </form>
    </div>
  );
}
