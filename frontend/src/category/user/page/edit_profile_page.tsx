import { useEffect } from "react";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2 } from "lucide-react";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { Textarea } from "@/components/ui/textarea";

import { useUpdateUser } from "../hook/use_update_user";
import { useUserMe } from "../hook/use_user_me";

const editProfileSchema = z.object({
  nickname: z.string().min(1, "닉네임을 입력해주세요.").max(100),
  name: z.string().min(1, "이름을 입력해주세요.").max(100),
  email: z.string().email(),
  bio: z.string().max(500, "자기소개는 500자 이내로 입력해주세요.").optional(),
});

type EditProfileForm = z.infer<typeof editProfileSchema>;

export function EditProfilePage() {
  const navigate = useNavigate();
  const { data, isLoading, error } = useUserMe();

  const form = useForm<EditProfileForm>({
    resolver: zodResolver(editProfileSchema),
    mode: "onChange",
    defaultValues: {
      nickname: "",
      name: "",
      email: "",
      bio: "",
    },
  });

  const updateUserMutation = useUpdateUser({
    onConflict: () => {
      form.setError("nickname", {
        message: "Nickname already taken",
      });
    },
  });

  useEffect(() => {
    if (data) {
      form.reset({
        nickname: data.nickname ?? "",
        name: data.name ?? "",
        email: data.email ?? "",
        bio: data.bio ?? "",
      });
    }
  }, [data, form]);

  useEffect(() => {
    if (error instanceof ApiError && error.status === 401) {
      toast.error("로그인이 필요합니다");
      navigate("/login", { replace: true });
    }
  }, [error, navigate]);

  const onSubmit = (values: EditProfileForm) => {
    updateUserMutation.mutate({
      nickname: values.nickname,
      name: values.name,
      bio: values.bio?.trim() ? values.bio : undefined,
    });
  };

  if (isLoading) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-lg">
          <CardHeader>
            <CardTitle>프로필 수정</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <Skeleton className="h-4 w-32" />
            <Skeleton className="h-9 w-full" />
            <Skeleton className="h-4 w-32" />
            <Skeleton className="h-9 w-full" />
            <Skeleton className="h-4 w-32" />
            <Skeleton className="h-9 w-full" />
            <Skeleton className="h-4 w-32" />
            <Skeleton className="h-24 w-full" />
          </CardContent>
        </Card>
      </div>
    );
  }

  if (error instanceof ApiError && error.status === 404) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-lg">
          <CardHeader>
            <CardTitle>프로필 수정</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-destructive">
              사용자 정보를 찾을 수 없습니다.
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (!data) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-lg">
          <CardHeader>
            <CardTitle>프로필 수정</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              사용자 정보를 불러오지 못했습니다.
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-lg">
        <CardHeader className="space-y-2">
          <CardTitle>프로필 수정</CardTitle>
          <CardDescription>
            기본 정보를 업데이트하고 변경 사항을 저장하세요.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              <FormField
                control={form.control}
                name="nickname"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>닉네임</FormLabel>
                    <FormControl>
                      <Input placeholder="닉네임" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>이름</FormLabel>
                    <FormControl>
                      <Input placeholder="이름" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
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
                        disabled
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="bio"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>자기소개</FormLabel>
                    <FormControl>
                      <Textarea placeholder="자기소개를 입력하세요" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <div className="flex gap-2">
                <Button
                  type="submit"
                  className="flex-1"
                  disabled={updateUserMutation.isPending}
                >
                  {updateUserMutation.isPending ? (
                    <>
                      <Loader2 className="h-4 w-4 animate-spin" />
                      저장 중...
                    </>
                  ) : (
                    "저장하기"
                  )}
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  className="flex-1"
                  onClick={() => navigate(-1)}
                  disabled={updateUserMutation.isPending}
                >
                  취소
                </Button>
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
