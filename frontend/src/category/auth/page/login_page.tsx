import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2 } from "lucide-react";
import { useForm } from "react-hook-form";
import { Link } from "react-router-dom";

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
import { loginReqSchema, type LoginReq } from "@/category/auth/types"; // ✅ 경로 정확함

import { useLogin } from "../hook/use_login";

export function LoginPage() {
  const loginMutation = useLogin();
  
  const form = useForm<LoginReq>({
    resolver: zodResolver(loginReqSchema),
    mode: "onChange",
    defaultValues: {
      email: "",
      password: "",
      // device, browser 등은 Optional이므로 생략 가능
    },
  });

  const onSubmit = (values: LoginReq) => {
    loginMutation.mutate(values);
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2">
          <CardTitle>로그인</CardTitle>
          <CardDescription>
            다시 돌아오신 것을 환영합니다. 계정에 로그인하세요.
          </CardDescription>
        </CardHeader>
        <CardContent>
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

              {/* 하단 링크 영역 */}
              <div className="flex items-center justify-between text-sm">
                <Link
                  to="/signup"
                  className="text-primary underline-offset-4 hover:underline"
                >
                  계정이 없으신가요? 회원가입
                </Link>
                
                {/* 🚨 [수정됨] a 태그 -> Link 컴포넌트 & 경로 변경 */}
                <Link
                  to="/find-id" 
                  className="text-muted-foreground underline-offset-4 hover:underline"
                >
                  아이디/비밀번호 찾기
                </Link>
              </div>

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
        </CardContent>
      </Card>
    </div>
  );
}