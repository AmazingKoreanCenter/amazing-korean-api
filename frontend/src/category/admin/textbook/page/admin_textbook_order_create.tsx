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

const UNIT_PRICE = 25_000;

type OrderItem = {
  language: TextbookLanguage | "";
  textbook_type: TextbookType;
  quantity: number;
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

  const totalQuantity = items.reduce((sum, it) => sum + (it.quantity || 0), 0);
  const totalAmount = totalQuantity * UNIT_PRICE;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // 기본 유효성
    if (!ordererName.trim() || !ordererEmail.trim() || !ordererPhone.trim()) {
      toast.error(t("admin.textbook.create.err.ordererRequired"));
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

    const payload: AdminCreateOrderReq = {
      user_id: userId.trim() ? Number(userId) : undefined,
      initial_status: initialStatus,
      enforce_min_quantity: enforceMinQuantity,
      orderer_name: ordererName,
      orderer_email: ordererEmail,
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
        quantity: it.quantity,
      })),
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
          <CardContent className="grid grid-cols-1 md:grid-cols-3 gap-4">
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
            <div>
              <Label htmlFor="userId">
                {t("admin.textbook.create.userId")}
              </Label>
              <Input
                id="userId"
                type="number"
                value={userId}
                onChange={(e) => setUserId(e.target.value)}
                placeholder={t("admin.textbook.create.userIdPlaceholder")}
                className="mt-1"
              />
              <p className="text-xs text-muted-foreground mt-1">
                {t("admin.textbook.create.userIdHint")}
              </p>
            </div>
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
                {t("admin.textbook.create.email")} *
              </Label>
              <Input
                id="ordererEmail"
                type="email"
                value={ordererEmail}
                onChange={(e) => setOrdererEmail(e.target.value)}
                required
              />
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
              <Plus className="h-4 w-4 mr-1" />
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
                    type="number"
                    min={1}
                    value={item.quantity}
                    onChange={(e) =>
                      updateItem(idx, {
                        quantity: Math.max(1, Number(e.target.value) || 1),
                      })
                    }
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
                {t("admin.textbook.create.totalAmount")}:{" "}
                <strong>{totalAmount.toLocaleString()} KRW</strong>
              </span>
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
                  type="email"
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
