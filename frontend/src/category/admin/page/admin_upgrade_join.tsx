import { useEffect, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { Loader2, ShieldCheck, AlertCircle, CheckCircle2 } from "lucide-react";
import { toast } from "sonner";
import { useMutation, useQuery } from "@tanstack/react-query";

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
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";

import { upgradeAcceptReqSchema, type UpgradeAcceptReq } from "../types";
import { verifyAdminInvite, acceptAdminInvite } from "../admin_api";

const genderOptions = [
  { value: "male", label: "남성" },
  { value: "female", label: "여성" },
  { value: "other", label: "기타" },
];

const countryOptions = [
  { value: "KR", label: "대한민국 (KR)" },
  { value: "US", label: "United States (US)" },
  { value: "JP", label: "Japan (JP)" },
];

const languageOptions = [
  { value: "ko", label: "한국어 (ko)" },
  { value: "en", label: "English (en)" },
];

const roleLabels: Record<string, string> = {
  admin: "관리자 (Admin)",
  manager: "매니저 (Manager)",
};

export function AdminUpgradeJoin() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const code = searchParams.get("code") || "";
  const [isSuccess, setIsSuccess] = useState(false);

  // 초대 코드 검증
  const {
    data: inviteData,
    isLoading: isVerifying,
    error: verifyError,
  } = useQuery({
    queryKey: ["adminInviteVerify", code],
    queryFn: () => verifyAdminInvite(code),
    enabled: !!code,
    retry: false,
  });

  // 계정 생성 뮤테이션
  const acceptMutation = useMutation({
    mutationFn: acceptAdminInvite,
    onSuccess: () => {
      setIsSuccess(true);
      toast.success("관리자 계정이 생성되었습니다. 로그인해주세요.");
      setTimeout(() => navigate("/login"), 2000);
    },
    onError: (error: Error) => {
      toast.error(error.message || "계정 생성에 실패했습니다.");
    },
  });

  const form = useForm<UpgradeAcceptReq>({
    resolver: zodResolver(upgradeAcceptReqSchema),
    mode: "onChange",
    defaultValues: {
      code: code,
      password: "",
      name: "",
      nickname: "",
      country: "KR",
      birthday: "",
      gender: "male",
      language: "ko",
    },
  });

  // code가 변경되면 form에 반영
  useEffect(() => {
    if (code) {
      form.setValue("code", code);
    }
  }, [code, form]);

  const onSubmit = (values: UpgradeAcceptReq) => {
    acceptMutation.mutate(values);
  };

  // 코드가 없는 경우
  if (!code) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-md">
          <CardContent className="pt-6">
            <Alert variant="destructive">
              <AlertCircle className="h-4 w-4" />
              <AlertTitle>유효하지 않은 접근</AlertTitle>
              <AlertDescription>
                초대 코드가 필요합니다. 초대 이메일의 링크를 다시 확인해주세요.
              </AlertDescription>
            </Alert>
          </CardContent>
        </Card>
      </div>
    );
  }

  // 검증 중
  if (isVerifying) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-md">
          <CardContent className="pt-6 flex flex-col items-center gap-4">
            <Loader2 className="h-8 w-8 animate-spin text-primary" />
            <p className="text-muted-foreground">초대 코드를 확인하고 있습니다...</p>
          </CardContent>
        </Card>
      </div>
    );
  }

  // 검증 실패
  if (verifyError || !inviteData) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-md">
          <CardContent className="pt-6">
            <Alert variant="destructive">
              <AlertCircle className="h-4 w-4" />
              <AlertTitle>초대 코드가 유효하지 않습니다</AlertTitle>
              <AlertDescription>
                초대 코드가 만료되었거나 이미 사용된 코드입니다.
                <br />
                관리자에게 새로운 초대를 요청해주세요.
              </AlertDescription>
            </Alert>
          </CardContent>
        </Card>
      </div>
    );
  }

  // 계정 생성 성공
  if (isSuccess) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-md">
          <CardContent className="pt-6 flex flex-col items-center gap-4">
            <CheckCircle2 className="h-12 w-12 text-status-success" />
            <h3 className="text-xl font-semibold">계정이 생성되었습니다!</h3>
            <p className="text-muted-foreground text-center">
              잠시 후 로그인 페이지로 이동합니다...
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2 text-center">
          <div className="flex justify-center mb-2">
            <ShieldCheck className="h-10 w-10 text-primary" />
          </div>
          <CardTitle className="text-2xl">관리자 계정 생성</CardTitle>
          <CardDescription>
            관리자로 초대되었습니다. 계정 정보를 입력해주세요.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* 초대 정보 표시 */}
          <div className="bg-muted/50 rounded-lg p-4 space-y-2">
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">이메일</span>
              <span className="font-medium">{inviteData.email}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">역할</span>
              <Badge variant="secondary">
                {roleLabels[inviteData.role] || inviteData.role}
              </Badge>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">초대자</span>
              <span className="text-sm">{inviteData.invited_by}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">만료 시간</span>
              <span className="text-sm">
                {new Date(inviteData.expires_at).toLocaleString("ko-KR")}
              </span>
            </div>
          </div>

          {/* 계정 생성 폼 */}
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              {/* 비밀번호 */}
              <FormField
                control={form.control}
                name="password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>비밀번호</FormLabel>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder="8자 이상, 영문+숫자 포함"
                        autoComplete="new-password"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

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

              {/* 닉네임 */}
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

              {/* 생년월일 */}
              <FormField
                control={form.control}
                name="birthday"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>생년월일</FormLabel>
                    <FormControl>
                      <Input type="date" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 성별 */}
              <FormField
                control={form.control}
                name="gender"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>성별</FormLabel>
                    <Select
                      onValueChange={field.onChange}
                      defaultValue={field.value}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="성별을 선택하세요" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {genderOptions.map((option) => (
                          <SelectItem key={option.value} value={option.value}>
                            {option.label}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 국가 */}
              <FormField
                control={form.control}
                name="country"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>국가</FormLabel>
                    <Select
                      onValueChange={field.onChange}
                      defaultValue={field.value}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="국가를 선택하세요" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {countryOptions.map((option) => (
                          <SelectItem key={option.value} value={option.value}>
                            {option.label}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 언어 */}
              <FormField
                control={form.control}
                name="language"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>언어</FormLabel>
                    <Select
                      onValueChange={field.onChange}
                      defaultValue={field.value}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="언어를 선택하세요" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {languageOptions.map((option) => (
                          <SelectItem key={option.value} value={option.value}>
                            {option.label}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <Button
                type="submit"
                className="w-full"
                disabled={acceptMutation.isPending}
              >
                {acceptMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin mr-2" />
                    계정 생성 중...
                  </>
                ) : (
                  "관리자 계정 생성"
                )}
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
