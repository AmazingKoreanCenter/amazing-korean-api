import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2, ChevronDown, ChevronUp, AlertCircle } from "lucide-react";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { Link, useNavigate } from "react-router-dom";

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
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { loginReqSchema, type LoginReq } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";

import { useLogin } from "../hook/use_login";
import { useGoogleLogin } from "../hook/use_google_login";
import { useOAuthCallback } from "../hook/use_oauth_callback";

// Provider 이름 한글화
const providerNameMap: Record<string, string> = {
  google: "Google",
  apple: "Apple",
  social: "소셜",
};

const getProviderDisplayName = (provider: string) =>
  providerNameMap[provider.toLowerCase()] ?? provider;

export function LoginPage() {
  const loginMutation = useLogin();
  const googleLoginMutation = useGoogleLogin();
  const navigate = useNavigate();
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
  const [showEmailForm, setShowEmailForm] = useState(false);

  // OAuth 콜백 처리 (Google 로그인 완료 후 리다이렉트 처리)
  const { isProcessing: isOAuthProcessing, hasOAuthParams } = useOAuthCallback();

  // 이미 로그인된 경우 홈으로 리다이렉트
  // 단, OAuth 콜백 처리 중에는 건너뛰기 (useOAuthCallback에서 리다이렉트 처리)
  useEffect(() => {
    if (isLoggedIn && !isOAuthProcessing && !hasOAuthParams) {
      navigate("/", { replace: true });
    }
  }, [isLoggedIn, isOAuthProcessing, hasOAuthParams, navigate]);

  const form = useForm<LoginReq>({
    resolver: zodResolver(loginReqSchema),
    mode: "onChange",
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const onSubmit = (values: LoginReq) => {
    loginMutation.mutate(values);
  };

  const handleGoogleLogin = () => {
    // 소셜 전용 에러 상태 초기화
    loginMutation.reset();
    googleLoginMutation.mutate();
  };

  // 소셜 전용 계정 에러 UI
  const renderSocialOnlyAlert = () => {
    if (!loginMutation.socialOnlyError) return null;

    const { providers } = loginMutation.socialOnlyError;
    const providerNames = providers.map(getProviderDisplayName).join(", ");

    return (
      <div className="rounded-lg border border-amber-200 bg-amber-50 p-4 mb-4">
        <div className="flex items-start gap-3">
          <AlertCircle className="h-5 w-5 text-amber-600 mt-0.5 shrink-0" />
          <div className="space-y-2">
            <p className="text-sm font-medium text-amber-800">
              소셜 계정으로 가입된 이메일입니다
            </p>
            <p className="text-sm text-amber-700">
              이 이메일은 <strong>{providerNames}</strong> 계정으로 가입되었습니다.
              <br />
              아래 버튼을 눌러 소셜 로그인을 이용해주세요.
            </p>
            {providers.includes("google") && (
              <Button
                type="button"
                variant="outline"
                className="w-full mt-2 border-amber-300 hover:bg-amber-100"
                onClick={handleGoogleLogin}
                disabled={googleLoginMutation.isPending}
              >
                {googleLoginMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin mr-2" />
                    Google 로그인 중...
                  </>
                ) : (
                  <>
                    <svg className="h-4 w-4 mr-2" viewBox="0 0 24 24">
                      <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" />
                      <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" />
                      <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" />
                      <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" />
                    </svg>
                    Google로 로그인
                  </>
                )}
              </Button>
            )}
          </div>
        </div>
      </div>
    );
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2 text-center">
          <CardTitle className="text-2xl">로그인</CardTitle>
          <CardDescription>
            Amazing Korean에 오신 것을 환영합니다
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* 소셜 로그인 버튼 */}
          <div className="space-y-3">
            {/* Google 로그인 */}
            <Button
              type="button"
              variant="outline"
              className="w-full h-12 text-base"
              onClick={handleGoogleLogin}
              disabled={googleLoginMutation.isPending}
            >
              {googleLoginMutation.isPending ? (
                <>
                  <Loader2 className="h-5 w-5 animate-spin mr-3" />
                  Google 로그인 중...
                </>
              ) : (
                <>
                  <svg className="h-5 w-5 mr-3" viewBox="0 0 24 24">
                    <path
                      fill="#4285F4"
                      d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                    />
                    <path
                      fill="#34A853"
                      d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                    />
                    <path
                      fill="#FBBC05"
                      d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                    />
                    <path
                      fill="#EA4335"
                      d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                    />
                  </svg>
                  Google로 계속하기
                </>
              )}
            </Button>

            {/* Apple 로그인 (비활성화) */}
            <Button
              type="button"
              variant="outline"
              className="w-full h-12 text-base"
              disabled
            >
              <svg className="h-5 w-5 mr-3" viewBox="0 0 24 24" fill="currentColor">
                <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z" />
              </svg>
              Apple로 계속하기 (준비 중)
            </Button>
          </div>

          {/* 구분선 */}
          <div className="relative my-6">
            <div className="absolute inset-0 flex items-center">
              <span className="w-full border-t" />
            </div>
            <div className="relative flex justify-center text-xs uppercase">
              <span className="bg-background px-2 text-muted-foreground"></span>
            </div>
          </div>

          {/* 이메일 로그인 (접이식) */}
          <Collapsible open={showEmailForm} onOpenChange={setShowEmailForm}>
            <CollapsibleTrigger asChild>
              <Button
                variant="ghost"
                className="w-full justify-between text-muted-foreground hover:text-foreground"
              >
                이메일로 로그인
                {showEmailForm ? (
                  <ChevronUp className="h-4 w-4" />
                ) : (
                  <ChevronDown className="h-4 w-4" />
                )}
              </Button>
            </CollapsibleTrigger>
            <CollapsibleContent className="pt-4">
              {/* 소셜 전용 계정 안내 */}
              {renderSocialOnlyAlert()}

              <Form {...form}>
                <form
                  onSubmit={form.handleSubmit(onSubmit)}
                  className="space-y-4"
                >
                  {/* 이메일 입력 */}
                  <FormField
                    control={form.control}
                    name="email"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>이메일</FormLabel>
                        <FormControl>
                          <Input
                            type="email"
                            placeholder="email@example.com"
                            autoComplete="email"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />

                  {/* 비밀번호 입력 */}
                  <FormField
                    control={form.control}
                    name="password"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>비밀번호</FormLabel>
                        <FormControl>
                          <Input
                            type="password"
                            placeholder="비밀번호를 입력하세요"
                            autoComplete="current-password"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />

                  <Button
                    type="submit"
                    className="w-full"
                    disabled={loginMutation.isPending}
                  >
                    {loginMutation.isPending ? (
                      <>
                        <Loader2 className="h-4 w-4 animate-spin mr-2" />
                        로그인 중...
                      </>
                    ) : (
                      "로그인"
                    )}
                  </Button>
                </form>
              </Form>

              {/* 이메일 로그인 하단 링크 */}
              <div className="flex items-center justify-between text-sm mt-4">
                <Link
                  to="/signup"
                  className="text-primary underline-offset-4 hover:underline"
                >
                  이메일로 회원가입
                </Link>
                <Link
                  to="/find-id"
                  className="text-muted-foreground underline-offset-4 hover:underline"
                >
                  아이디/비밀번호 찾기
                </Link>
              </div>
            </CollapsibleContent>
          </Collapsible>

          {/* 회원가입 안내 */}
          <p className="text-center text-sm text-muted-foreground pt-4">
            계정이 없으신가요?{" "}
            <Link
              to="/signup"
              className="text-primary underline-offset-4 hover:underline font-medium"
            >
              회원가입
            </Link>
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
