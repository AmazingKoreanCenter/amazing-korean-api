import { useState } from "react";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2, KeyRound, ArrowLeft } from "lucide-react";
import { useForm } from "react-hook-form";
import { useLocation, useNavigate, Link } from "react-router-dom";
import { toast } from "sonner";
import { useTranslation } from "react-i18next";
import i18n from "@/i18n";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";

import { verifyEmail, resendVerification } from "../auth_api";
import { ApiError } from "@/api/client";

const verificationFormSchema = z.object({
  code: z.string().min(6, i18n.t("auth.validationCodeLength")).max(6),
});

type VerificationForm = z.infer<typeof verificationFormSchema>;

export function VerifyEmailPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const location = useLocation();

  // state로 전달받은 email (signup/login에서 넘어옴)
  const email = (location.state as { email?: string })?.email;

  const [isVerifying, setIsVerifying] = useState(false);
  const [isResending, setIsResending] = useState(false);
  const [remainingAttempts, setRemainingAttempts] = useState<number | null>(null);

  const form = useForm<VerificationForm>({
    resolver: zodResolver(verificationFormSchema),
    mode: "onChange",
    defaultValues: { code: "" },
  });

  // email 없으면 회원가입으로 이동
  if (!email) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle>{t("auth.verifyEmailTitle")}</CardTitle>
            <CardDescription>{t("auth.verifyEmailNoEmail")}</CardDescription>
          </CardHeader>
          <CardContent>
            <Link to="/signup">
              <Button className="w-full">{t("auth.goToSignup")}</Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    );
  }

  const handleVerify = async (values: VerificationForm) => {
    setIsVerifying(true);
    try {
      await verifyEmail({ email, code: values.code });
      toast.success(t("auth.toastEmailVerified"));
      navigate("/login", { replace: true });
    } catch (error) {
      if (error instanceof ApiError) {
        if (error.status === 429) {
          toast.error(t("auth.toastTooManyVerifyAttempts"));
        } else {
          toast.error(t("auth.toastCodeInvalid"));
        }
      } else {
        toast.error(t("common.requestFailed"));
      }
    } finally {
      setIsVerifying(false);
    }
  };

  const handleResend = async () => {
    setIsResending(true);
    try {
      const res = await resendVerification({ email });
      setRemainingAttempts(res.remaining_attempts);
      toast.success(t("auth.toastCodeResent"));
    } catch (error) {
      if (error instanceof ApiError && error.status === 429) {
        setRemainingAttempts(0);
        toast.error(t("auth.toastTooManyResendRequests"));
      } else {
        toast.error(t("auth.toastResendFailed"));
      }
    } finally {
      setIsResending(false);
    }
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>{t("auth.verifyEmailTitle")}</CardTitle>
          <CardDescription>
            {t("auth.verifyEmailDescription", { email })}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form
              onSubmit={form.handleSubmit(handleVerify)}
              className="space-y-4"
            >
              <FormField
                control={form.control}
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
                <Link
                  to="/login"
                  className="text-muted-foreground underline-offset-4 hover:underline flex items-center gap-1"
                >
                  <ArrowLeft className="h-4 w-4" />
                  {t("auth.backToLogin")}
                </Link>
                <button
                  type="button"
                  onClick={handleResend}
                  disabled={isResending || remainingAttempts === 0}
                  className="text-primary underline-offset-4 hover:underline disabled:opacity-50"
                >
                  {isResending ? t("auth.sending") : t("auth.resendCode")}
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
        </CardContent>
      </Card>
    </div>
  );
}
