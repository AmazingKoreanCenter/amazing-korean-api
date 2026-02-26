import { useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2, Mail, KeyRound, ArrowLeft, AlertTriangle } from "lucide-react";
import { useForm } from "react-hook-form";
import { Link, useNavigate } from "react-router-dom";
import { toast } from "sonner";
import { useTranslation } from "react-i18next";

import i18n from "@/i18n";
import { ApiError } from "@/api/client";
import {
  findIdReqSchema,
  findPasswordReqSchema,
  type FindIdReq,
  type FindPasswordReq,
} from "@/category/auth/types";
import { findPassword, verifyResetCode } from "../auth_api";
import { useFindId } from "../hook/use_find_id";

import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { PageMeta } from "@/components/page_meta";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs";

// ==========================================
// 인증코드 입력 스키마
// ==========================================
import { z } from "zod";

const verificationFormSchema = z.object({
  code: z.string().min(6, i18n.t("auth.validationCodeLength")).max(6),
});
type VerificationForm = z.infer<typeof verificationFormSchema>;

// ==========================================
// 탭 1: 아이디 찾기
// ==========================================

function FindIdTab({ onSwitchTab }: { onSwitchTab: () => void }) {
  const { t } = useTranslation();
  const findIdMutation = useFindId();

  const form = useForm<FindIdReq>({
    resolver: zodResolver(findIdReqSchema),
    mode: "onChange",
    defaultValues: { name: "", birthday: "" },
  });

  const onSubmit = (values: FindIdReq) => {
    findIdMutation.mutate(values);
  };

  const result = findIdMutation.data;

  return (
    <div className="space-y-4 pt-4">
      <p className="text-sm text-muted-foreground">
        {t("auth.findIdDescription")}
      </p>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
          <FormField
            control={form.control}
            name="name"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("auth.nameLabel")}</FormLabel>
                <FormControl>
                  <Input
                    placeholder={t("auth.namePlaceholder")}
                    autoComplete="name"
                    {...field}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="birthday"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("auth.birthdayLabel")}</FormLabel>
                <FormControl>
                  <Input type="date" autoComplete="bday" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button
            type="submit"
            className="w-full"
            disabled={findIdMutation.isPending}
          >
            {findIdMutation.isPending ? (
              <>
                <Loader2 className="h-4 w-4 animate-spin mr-2" />
                {t("auth.requesting")}
              </>
            ) : (
              t("auth.findIdButton")
            )}
          </Button>
        </form>
      </Form>

      {/* 결과 표시 */}
      {result && (
        <div className="rounded-lg border p-4">
          {result.masked_emails.length === 0 ? (
            <p className="text-sm text-muted-foreground text-center">
              {t("auth.findIdNotFound")}
            </p>
          ) : (
            <div className="space-y-3">
              <p className="text-sm font-medium">
                {t("auth.findIdResultTitle")}
              </p>
              <ul className="space-y-2">
                {result.masked_emails.map((email, idx) => (
                  <li
                    key={idx}
                    className="flex items-center gap-2 text-sm"
                  >
                    <Mail className="h-4 w-4 text-muted-foreground" />
                    <span className="font-mono">{email}</span>
                  </li>
                ))}
              </ul>
              <div className="flex items-center justify-between pt-2">
                <Link
                  to="/login"
                  className="text-sm text-primary underline-offset-4 hover:underline"
                >
                  {t("auth.backToLogin")}
                </Link>
                <button
                  type="button"
                  onClick={onSwitchTab}
                  className="text-sm text-primary underline-offset-4 hover:underline"
                >
                  {t("auth.forgotPassword")}
                </button>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

// ==========================================
// 탭 2: 비밀번호 찾기 (3단계)
// ==========================================

function FindPasswordTab() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [step, setStep] = useState<"identity" | "verification">("identity");
  const [submittedData, setSubmittedData] = useState<FindPasswordReq | null>(null);
  const [isSending, setIsSending] = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);
  const [remainingAttempts, setRemainingAttempts] = useState<number | null>(null);

  // Step 1: 본인 확인 폼
  const identityForm = useForm<FindPasswordReq>({
    resolver: zodResolver(findPasswordReqSchema),
    mode: "onChange",
    defaultValues: { name: "", birthday: "", email: "" },
  });

  // Step 2: 인증코드 폼
  const verificationForm = useForm<VerificationForm>({
    resolver: zodResolver(verificationFormSchema),
    mode: "onChange",
    defaultValues: { code: "" },
  });

  // Step 1: 본인 확인 + 인증코드 발송
  const handleIdentityVerify = async (values: FindPasswordReq) => {
    setIsSending(true);
    try {
      const res = await findPassword(values);
      setRemainingAttempts(res.remaining_attempts);
      setSubmittedData(values);
      setStep("verification");
      toast.success(t("auth.toastCodeSent", { email: values.email }));
    } catch (error) {
      if (error instanceof ApiError && error.status === 429) {
        setRemainingAttempts(0);
        toast.warning(t("auth.toastTooManyAttempts"));
      } else {
        toast.error(t("auth.toastCodeSendFailed"));
      }
    } finally {
      setIsSending(false);
    }
  };

  // Step 2: 인증코드 확인
  const handleVerifyCode = async (values: VerificationForm) => {
    if (!submittedData) return;
    setIsVerifying(true);
    try {
      const res = await verifyResetCode({
        email: submittedData.email,
        code: values.code,
      });
      navigate(`/reset-password?token=${res.reset_token}`);
    } catch (error) {
      if (error instanceof ApiError && error.status === 429) {
        toast.warning(t("auth.toastTooManyAttempts"));
      } else {
        toast.error(t("auth.toastCodeInvalid"));
      }
    } finally {
      setIsVerifying(false);
    }
  };

  // 인증코드 재전송
  const handleResendCode = async () => {
    if (!submittedData) return;
    setIsSending(true);
    try {
      const res = await findPassword(submittedData);
      setRemainingAttempts(res.remaining_attempts);
      toast.success(t("auth.toastCodeResent"));
    } catch (error) {
      if (error instanceof ApiError && error.status === 429) {
        setRemainingAttempts(0);
        toast.warning(t("auth.toastTooManyAttempts"));
      } else {
        toast.error(t("auth.toastResendFailed"));
      }
    } finally {
      setIsSending(false);
    }
  };

  return (
    <div className="space-y-4 pt-4">
      <p className="text-sm text-muted-foreground">
        {step === "identity"
          ? t("auth.findPasswordDescription")
          : t("auth.resetPasswordCodeStep", { email: submittedData?.email })}
      </p>

      {step === "identity" && (
        <div className="flex items-start gap-2 rounded-lg border border-yellow-200 bg-yellow-50 p-3 text-sm text-yellow-800 dark:border-yellow-900 dark:bg-yellow-950 dark:text-yellow-200">
          <AlertTriangle className="h-4 w-4 mt-0.5 shrink-0" />
          <p>{t("auth.oauthPasswordWarning")}</p>
        </div>
      )}

      {step === "identity" ? (
        /* Step 1: 본인 확인 */
        <Form key="identity" {...identityForm}>
          <form
            onSubmit={identityForm.handleSubmit(handleIdentityVerify)}
            className="space-y-4"
          >
            <FormField
              control={identityForm.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("auth.nameLabel")}</FormLabel>
                  <FormControl>
                    <Input
                      placeholder={t("auth.namePlaceholder")}
                      autoComplete="name"
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={identityForm.control}
              name="birthday"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("auth.birthdayLabel")}</FormLabel>
                  <FormControl>
                    <Input type="date" autoComplete="bday" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={identityForm.control}
              name="email"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("auth.emailLabel")}</FormLabel>
                  <FormControl>
                    <div className="relative">
                      <Mail className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                      <Input
                        type="email"
                        placeholder={t("auth.emailPlaceholder")}
                        className="pl-10"
                        autoComplete="email"
                        {...field}
                      />
                    </div>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button type="submit" className="w-full" disabled={isSending}>
              {isSending ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  {t("auth.findPasswordSending")}
                </>
              ) : (
                t("auth.findPasswordButton")
              )}
            </Button>
          </form>
        </Form>
      ) : (
        /* Step 2: 인증코드 입력 */
        <Form key="verification" {...verificationForm}>
          <form
            onSubmit={verificationForm.handleSubmit(handleVerifyCode)}
            className="space-y-4"
          >
            <FormField
              control={verificationForm.control}
              name="code"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("auth.verificationCodeLabel")}</FormLabel>
                  <FormControl>
                    <div className="relative">
                      <KeyRound className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                      <Input
                        type="text"
                        placeholder={t("auth.verificationCodePlaceholder")}
                        className="pl-10 text-center tracking-widest"
                        maxLength={6}
                        autoComplete="one-time-code"
                        {...field}
                      />
                    </div>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button type="submit" className="w-full" disabled={isVerifying}>
              {isVerifying ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  {t("auth.verifying")}
                </>
              ) : (
                t("auth.verifyCode")
              )}
            </Button>
            <div className="flex items-center justify-between text-sm">
              <button
                type="button"
                onClick={() => setStep("identity")}
                className="text-muted-foreground underline-offset-4 hover:underline flex items-center gap-1"
              >
                <ArrowLeft className="h-4 w-4" />
                {t("auth.changeEmail")}
              </button>
              <button
                type="button"
                onClick={handleResendCode}
                disabled={isSending || remainingAttempts === 0}
                className="text-primary underline-offset-4 hover:underline disabled:opacity-50"
              >
                {isSending ? t("auth.sending") : t("auth.resendCode")}
              </button>
            </div>
            {remainingAttempts !== null && (
              <p className="text-xs text-muted-foreground text-center">
                {remainingAttempts > 0
                  ? t("auth.remainingAttempts", { count: remainingAttempts })
                  : t("auth.noAttemptsRemaining")}
              </p>
            )}
          </form>
        </Form>
      )}
    </div>
  );
}

// ==========================================
// 메인 페이지
// ==========================================

export function AccountRecoveryPage() {
  const { t } = useTranslation();
  const [activeTab, setActiveTab] = useState("find-id");

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <PageMeta titleKey="seo.findId.title" descriptionKey="seo.findId.description" />
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>{t("auth.accountRecoveryTitle")}</CardTitle>
        </CardHeader>
        <CardContent>
          <Tabs value={activeTab} onValueChange={setActiveTab}>
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="find-id">
                {t("auth.findIdTabTitle")}
              </TabsTrigger>
              <TabsTrigger value="find-password">
                {t("auth.findPasswordTabTitle")}
              </TabsTrigger>
            </TabsList>
            <TabsContent value="find-id">
              <FindIdTab onSwitchTab={() => setActiveTab("find-password")} />
            </TabsContent>
            <TabsContent value="find-password">
              <FindPasswordTab />
            </TabsContent>
          </Tabs>

          <div className="flex items-center justify-center text-sm mt-6">
            <Link
              to="/login"
              className="text-muted-foreground underline-offset-4 hover:underline flex items-center gap-1"
            >
              <ArrowLeft className="h-4 w-4" />
              {t("auth.backToLogin")}
            </Link>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
