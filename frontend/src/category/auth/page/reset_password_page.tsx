import { useEffect, useState } from "react";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2 } from "lucide-react";
import { useForm } from "react-hook-form";
import { Link, useNavigate, useSearchParams } from "react-router-dom";
import { toast } from "sonner";

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
import { ApiError } from "@/api/client";

import { useResetPassword } from "../hook/use_reset_password";

const resetPasswordFormSchema = z
  .object({
    new_password: z.string().min(6, "비밀번호는 최소 6자 이상이어야 합니다."),
    confirm_password: z
      .string()
      .min(6, "비밀번호는 최소 6자 이상이어야 합니다."),
  })
  .refine((data) => data.new_password === data.confirm_password, {
    message: "비밀번호가 일치하지 않습니다.",
    path: ["confirm_password"],
  });

type ResetPasswordForm = z.infer<typeof resetPasswordFormSchema>;

export function ResetPasswordPage() {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const resetPasswordMutation = useResetPassword();
  const [formError, setFormError] = useState<string | null>(null);
  const token = searchParams.get("token") ?? "";

  const form = useForm<ResetPasswordForm>({
    resolver: zodResolver(resetPasswordFormSchema),
    mode: "onChange",
    defaultValues: {
      new_password: "",
      confirm_password: "",
    },
  });

  useEffect(() => {
    if (!token) {
      toast.error("잘못된 접근입니다.");
      navigate("/", { replace: true });
    }
  }, [navigate, token]);

  useEffect(() => {
    if (
      resetPasswordMutation.error instanceof ApiError &&
      resetPasswordMutation.error.status === 422
    ) {
      setFormError(
        resetPasswordMutation.error.message ||
          "비밀번호 형식을 다시 확인해주세요."
      );
      return;
    }

    setFormError(null);
  }, [resetPasswordMutation.error]);

  const onSubmit = (values: ResetPasswordForm) => {
    if (!token) {
      return;
    }

    setFormError(null);
    resetPasswordMutation.mutate({
      token,
      new_password: values.new_password,
    });
  };

  if (!token) {
    return null;
  }

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2">
          <CardTitle>비밀번호 재설정</CardTitle>
          <CardDescription>
            새 비밀번호를 입력하여 계정 보안을 유지하세요.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              <FormField
                control={form.control}
                name="new_password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>새 비밀번호</FormLabel>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder="새 비밀번호"
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
                    <FormLabel>새 비밀번호 확인</FormLabel>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder="새 비밀번호 확인"
                        autoComplete="new-password"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              {formError ? (
                <p className="text-sm text-destructive">{formError}</p>
              ) : null}
              <div className="flex items-center justify-between text-sm">
                <Link
                  to="/login"
                  className="text-primary underline-offset-4 hover:underline"
                >
                  로그인으로 돌아가기
                </Link>
              </div>
              <Button
                type="submit"
                className="w-full"
                disabled={resetPasswordMutation.isPending}
              >
                {resetPasswordMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin" />
                    변경 중...
                  </>
                ) : (
                  "비밀번호 변경"
                )}
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
