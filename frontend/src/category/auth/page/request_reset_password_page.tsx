import { useState, useEffect } from "react";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2, Mail, KeyRound, ArrowLeft } from "lucide-react";
import { useForm } from "react-hook-form";
import { Link, useNavigate } from "react-router-dom";
import { toast } from "sonner";

import { useUserMe } from "@/category/user/hook/use_user_me";

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
// Step 1: 이메일 입력 스키마
const emailFormSchema = z.object({
  email: z.string().email("올바른 이메일 형식을 입력해주세요."),
});

// Step 2: 인증번호 입력 스키마
const verificationFormSchema = z.object({
  code: z.string().min(6, "인증번호 6자리를 입력해주세요.").max(6),
});

type EmailForm = z.infer<typeof emailFormSchema>;
type VerificationForm = z.infer<typeof verificationFormSchema>;

export function RequestResetPasswordPage() {
  const navigate = useNavigate();
  const { data: userData, isLoading } = useUserMe();
  const [step, setStep] = useState<"email" | "verification">("email");
  const [submittedEmail, setSubmittedEmail] = useState("");
  const [isSending, setIsSending] = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);

  // OAuth 전용 계정은 비밀번호 재설정 불가 → 마이페이지로 리다이렉트
  useEffect(() => {
    if (!isLoading && userData?.has_password === false) {
      toast.error("소셜 로그인 계정은 비밀번호 재설정이 필요하지 않습니다.");
      navigate("/user/me", { replace: true });
    }
  }, [userData, isLoading, navigate]);

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

  // 로딩 중이거나 OAuth 전용 계정이면 렌더링 안 함
  if (isLoading || userData?.has_password === false) {
    return null;
  }

  // Step 2: 인증번호 입력 폼
  const verificationForm = useForm<VerificationForm>({
    resolver: zodResolver(verificationFormSchema),
    mode: "onChange",
    defaultValues: {
      code: "",
    },
  });

  // 인증번호 전송
  const handleSendCode = async (values: EmailForm) => {
    setIsSending(true);
    try {
      // TODO: 백엔드 API 연동 (POST /auth/request-reset-password)
      // await requestResetPassword({ email: values.email });

      // 임시: API 연동 전까지 알림만 표시
      toast.info("이메일 전송 기능은 준비 중입니다.");

      setSubmittedEmail(values.email);
      setStep("verification");
      toast.success(`${values.email}로 인증번호를 전송했습니다.`);
    } catch {
      toast.error("인증번호 전송에 실패했습니다. 잠시 후 다시 시도해주세요.");
    } finally {
      setIsSending(false);
    }
  };

  // 인증번호 확인
  const handleVerifyCode = async (values: VerificationForm) => {
    setIsVerifying(true);
    try {
      // TODO: 백엔드 API 연동 (POST /auth/verify-reset-code)
      // const { token } = await verifyResetCode({ email: submittedEmail, code: values.code });
      // navigate(`/reset-password?token=${token}`);

      // 임시: API 연동 전까지 알림만 표시
      toast.info("인증번호 확인 기능은 준비 중입니다.");
      console.log("Verification attempt:", { email: submittedEmail, code: values.code });
    } catch {
      toast.error("인증번호가 올바르지 않습니다.");
    } finally {
      setIsVerifying(false);
    }
  };

  // 인증번호 재전송
  const handleResendCode = async () => {
    setIsSending(true);
    try {
      // TODO: 백엔드 API 연동
      toast.info("이메일 전송 기능은 준비 중입니다.");
      toast.success("인증번호를 다시 전송했습니다.");
    } catch {
      toast.error("재전송에 실패했습니다.");
    } finally {
      setIsSending(false);
    }
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>비밀번호 재설정</CardTitle>
          <CardDescription>
            {step === "email"
              ? "가입하신 이메일 주소를 입력해주세요."
              : `${submittedEmail}로 전송된 인증번호를 입력해주세요.`}
          </CardDescription>
        </CardHeader>
        <CardContent>
          {step === "email" ? (
            /* Step 1: 이메일 입력 */
            <Form {...emailForm}>
              <form
                onSubmit={emailForm.handleSubmit(handleSendCode)}
                className="space-y-4"
              >
                <FormField
                  control={emailForm.control}
                  name="email"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>이메일</FormLabel>
                      <FormControl>
                        <div className="relative">
                          <Mail className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                          <Input
                            type="email"
                            placeholder="email@example.com"
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
                      전송 중...
                    </>
                  ) : (
                    "인증번호 전송"
                  )}
                </Button>

                <div className="flex items-center justify-center text-sm">
                  <Link
                    to="/user/me"
                    className="text-muted-foreground underline-offset-4 hover:underline flex items-center gap-1"
                  >
                    <ArrowLeft className="h-4 w-4" />
                    마이페이지로 돌아가기
                  </Link>
                </div>
              </form>
            </Form>
          ) : (
            /* Step 2: 인증번호 입력 */
            <Form {...verificationForm}>
              <form
                onSubmit={verificationForm.handleSubmit(handleVerifyCode)}
                className="space-y-4"
              >
                <FormField
                  control={verificationForm.control}
                  name="code"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>인증번호</FormLabel>
                      <FormControl>
                        <div className="relative">
                          <KeyRound className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                          <Input
                            type="text"
                            placeholder="6자리 인증번호"
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
                      확인 중...
                    </>
                  ) : (
                    "인증번호 확인"
                  )}
                </Button>

                <div className="flex items-center justify-between text-sm">
                  <button
                    type="button"
                    onClick={() => setStep("email")}
                    className="text-muted-foreground underline-offset-4 hover:underline flex items-center gap-1"
                  >
                    <ArrowLeft className="h-4 w-4" />
                    이메일 변경
                  </button>
                  <button
                    type="button"
                    onClick={handleResendCode}
                    disabled={isSending}
                    className="text-primary underline-offset-4 hover:underline disabled:opacity-50"
                  >
                    {isSending ? "전송 중..." : "인증번호 재전송"}
                  </button>
                </div>
              </form>
            </Form>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
