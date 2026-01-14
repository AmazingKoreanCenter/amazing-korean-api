import { useEffect } from "react";
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
// import { ApiError } from "@/api/client"; // 사용하지 않는다면 제거

import { useResetPassword } from "../hook/use_reset_password";

// Form Schema: UI 검증용 (비밀번호 확인 포함)
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

  // URL에서 토큰 추출
  const token = searchParams.get("token");

  // 토큰 유효성 검사 (페이지 진입 시)
  useEffect(() => {
    if (!token) {
      toast.error("잘못된 접근입니다. 유효한 링크로 접속해주세요.");
      navigate("/login", { replace: true });
    }
  }, [token, navigate]);

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
      toast.error("인증 토큰이 없습니다.");
      return;
    }

    // ✅ types.ts의 ResetPasswordReq 구조 { token, new_password }에 맞춰 전송
    resetPasswordMutation.mutate(
      {
        token,
        new_password: values.new_password,
      },
      {
        onSuccess: () => {
          toast.success("비밀번호가 변경되었습니다. 다시 로그인해주세요.");
          navigate("/login");
        },
      }
    );
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>비밀번호 재설정</CardTitle>
          <CardDescription>새로운 비밀번호를 입력해주세요.</CardDescription>
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
                    <FormLabel>새 비밀번호</FormLabel>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder="새 비밀번호 입력"
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
              
              <div className="flex items-center justify-end text-sm">
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
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
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