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
import { findIdReqSchema, type FindIdReq } from "@/category/auth/types"; // ✅ 경로 정확함

import { useFindId } from "../hook/use_find_id";

export function FindIdPage() {
  const findIdMutation = useFindId();
  
  const form = useForm<FindIdReq>({
    resolver: zodResolver(findIdReqSchema),
    mode: "onChange",
    defaultValues: {
      name: "",
      email: "",
    },
  });

  const onSubmit = (values: FindIdReq) => {
    findIdMutation.mutate(values);
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2">
          <CardTitle>아이디 찾기</CardTitle>
          <CardDescription>
            등록된 이름과 이메일을 입력하시면 안내 메일을 발송해드립니다.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              {/* 이름 */}
              <FormField
                control={form.control}
                name="name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>이름</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="홍길동"
                        autoComplete="name"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 이메일 */}
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

              <div className="flex items-center justify-end text-sm">
                <Link
                  to="/login"
                  className="text-primary underline-offset-4 hover:underline"
                >
                  로그인으로 돌아가기
                </Link>
                {/* 🚨 [삭제됨] 작동하지 않는 '비밀번호 찾기' 링크 제거
                  추후 '비밀번호 재설정 요청 API'가 구현되면 다시 추가하세요.
                */}
              </div>

              <Button
                type="submit"
                className="w-full"
                disabled={findIdMutation.isPending}
              >
                {findIdMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin mr-2" />
                    요청 중...
                  </>
                ) : (
                  "아이디 찾기"
                )}
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}