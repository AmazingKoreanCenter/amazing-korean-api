import { useState, useEffect } from "react";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2, Mail, KeyRound, ArrowLeft } from "lucide-react";
import { useForm } from "react-hook-form";
import { Link, useNavigate } from "react-router-dom";
import { toast } from "sonner";
import { useTranslation } from "react-i18next";
import i18n from "@/i18n";

import { ApiError } from "@/api/client";
import { useUserMe } from "@/category/user/hook/use_user_me";
import { requestPasswordReset, verifyResetCode } from "../auth_api";

import {
  Card,
  CardContent,
  CardDescription,
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

// Step 1: 이메일 입력 스키마
const emailFormSchema = z.object({
  email: z.string().email(i18n.t("auth.validationEmailFormat")),
});

// Step 2: 인증번호 입력 스키마
const verificationFormSchema = z.object({
  code: z.string().min(6, i18n.t("auth.validationCodeLength")).max(6),
});

type EmailForm = z.infer<typeof emailFormSchema>;
type VerificationForm = z.infer<typeof verificationFormSchema>;

export function RequestResetPasswordPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { data: userData, isLoading } = useUserMe();
  const [step, setStep] = useState<"email" | "verification">("email");
  const [submittedEmail, setSubmittedEmail] = useState("");
  const [isSending, setIsSending] = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);
  const [remainingAttempts, setRemainingAttempts] = useState<number | null>(null);

  // OAuth 전용 계정은 비밀번호 재설정 불가 -> 마이페이지로 리다이렉트 (로그인 상태일 때만)
  useEffect(() => {
    if (!isLoading && userData && userData.has_password === false) {
      toast.error(t("auth.toastSocialAccountNoPassword"));
      navigate("/user/me", { replace: true });
    }
  }, [userData, isLoading, navigate, t]);

  // Step 1: 이메일 입력 폼
  const emailForm = useForm<EmailForm>({
    resolver: zodResolver(emailFormSchema),
    mode: "onChange",
    defaultValues: {
      email: "",
    },
  });

  // 로그인한 사용자의 이메일로 자동 채우기
  useEffect(() => {
    if (userData?.email) {
      emailForm.setValue("email", userData.email);
    }
  }, [userData, emailForm]);

  // Step 2: 인증번호 입력 폼
  const verificationForm = useForm<VerificationForm>({
    resolver: zodResolver(verificationFormSchema),
    mode: "onChange",
    defaultValues: {
      code: "",
    },
  });

  // OAuth 전용 계정이면 렌더링 안 함 (로그인 상태일 때만 체크)
  if (isLoading || (userData && userData.has_password === false)) {
    return null;
  }

  // 인증번호 전송
  const handleSendCode = async (values: EmailForm) => {
    setIsSending(true);
    try {
      const res = await requestPasswordReset({ email: values.email });
      setRemainingAttempts(res.remaining_attempts);
      setSubmittedEmail(values.email);
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

  // 인증번호 확인
  const handleVerifyCode = async (values: VerificationForm) => {
    setIsVerifying(true);
    try {
      const res = await verifyResetCode({ email: submittedEmail, code: values.code });
      navigate("/reset-password", { state: { token: res.reset_token }, replace: true });
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

  // 인증번호 재전송
  const handleResendCode = async () => {
    setIsSending(true);
    try {
      const res = await requestPasswordReset({ email: submittedEmail });
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
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <PageMeta titleKey="seo.requestResetPassword.title" descriptionKey="seo.requestResetPassword.description" />
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>{t("auth.resetPasswordTitle")}</CardTitle>
          <CardDescription>
            {step === "email"
              ? t("auth.resetPasswordEmailStep")
              : t("auth.resetPasswordCodeStep", { email: submittedEmail })}
          </CardDescription>
        </CardHeader>
        <CardContent>
          {step === "email" ? (
            /* Step 1: 이메일 입력 */
            <Form key="email" {...emailForm}>
              <form
                onSubmit={emailForm.handleSubmit(handleSendCode)}
                className="space-y-4"
              >
                <FormField
                  control={emailForm.control}
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

                <Button
                  type="submit"
                  className="w-full"
                  disabled={isSending}
                >
                  {isSending ? (
                    <>
                      <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                      {t("auth.sending")}
                    </>
                  ) : (
                    t("auth.sendVerificationCode")
                  )}
                </Button>

                <div className="flex items-center justify-center text-sm">
                  <Link
                    to="/login"
                    className="text-muted-foreground underline-offset-4 hover:underline flex items-center gap-1"
                  >
                    <ArrowLeft className="h-4 w-4" />
                    {t("auth.backToLogin")}
                  </Link>
                </div>
              </form>
            </Form>
          ) : (
            /* Step 2: 인증번호 입력 */
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

                <Button
                  type="submit"
                  className="w-full"
                  disabled={isVerifying}
                >
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
                    onClick={() => setStep("email")}
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
        </CardContent>
      </Card>
    </div>
  );
}
