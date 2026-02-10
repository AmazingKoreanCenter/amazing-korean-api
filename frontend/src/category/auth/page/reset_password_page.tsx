import { useEffect } from "react";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2 } from "lucide-react";
import { useForm } from "react-hook-form";
import { Link, useNavigate, useLocation } from "react-router-dom";
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

import { useResetPassword } from "../hook/use_reset_password";

// Form Schema: UI 검증용 (비밀번호 확인 포함)
const resetPasswordFormSchema = z
  .object({
    new_password: z.string()
      .min(8, i18n.t("auth.validationPasswordMin"))
      .max(72)
      .regex(/[a-zA-Z]/, i18n.t("auth.validationPasswordLetter"))
      .regex(/[0-9]/, i18n.t("auth.validationPasswordDigit")),
    confirm_password: z
      .string()
      .min(1, i18n.t("auth.validationConfirmPasswordRequired")),
  })
  .refine((data) => data.new_password === data.confirm_password, {
    message: i18n.t("auth.validationPasswordMismatch"),
    path: ["confirm_password"],
  });

type ResetPasswordForm = z.infer<typeof resetPasswordFormSchema>;

export function ResetPasswordPage() {
  const { t } = useTranslation();
  const location = useLocation();
  const navigate = useNavigate();
  const resetPasswordMutation = useResetPassword();

  // state에서 토큰 추출 (URL 노출 방지)
  const token = (location.state as { token?: string } | null)?.token ?? null;

  // 토큰 유효성 검사 (페이지 진입 시)
  useEffect(() => {
    if (!token) {
      toast.error(t("auth.toastInvalidAccess"));
      navigate("/login", { replace: true });
    }
  }, [token, navigate, t]);

  const form = useForm<ResetPasswordForm>({
    resolver: zodResolver(resetPasswordFormSchema),
    mode: "onChange",
    defaultValues: {
      new_password: "",
      confirm_password: "",
    },
  });

  const onSubmit = (values: ResetPasswordForm) => {
    if (!token) {
      toast.error(t("auth.toastNoToken"));
      return;
    }

    resetPasswordMutation.mutate(
      {
        reset_token: token,
        new_password: values.new_password,
      },
      {
        onSuccess: () => {
          toast.success(t("auth.toastResetPasswordPageSuccess"));
          navigate("/login");
        },
      }
    );
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>{t("auth.resetPasswordNewTitle")}</CardTitle>
          <CardDescription>{t("auth.resetPasswordNewDescription")}</CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form
              onSubmit={form.handleSubmit(onSubmit)}
              className="space-y-4"
            >
              <FormField
                control={form.control}
                name="new_password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>{t("auth.newPasswordLabel")}</FormLabel>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder={t("auth.newPasswordPlaceholder")}
                        autoComplete="new-password"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="confirm_password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>{t("auth.confirmPasswordLabel")}</FormLabel>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder={t("auth.confirmPasswordPlaceholder")}
                        autoComplete="new-password"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <div className="flex items-center justify-end text-sm">
                <Link
                  to="/login"
                  className="text-primary underline-offset-4 hover:underline"
                >
                  {t("auth.backToLogin")}
                </Link>
              </div>

              <Button
                type="submit"
                className="w-full"
                disabled={resetPasswordMutation.isPending}
              >
                {resetPasswordMutation.isPending ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    {t("auth.changingPassword")}
                  </>
                ) : (
                  t("auth.changePasswordButton")
                )}
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
