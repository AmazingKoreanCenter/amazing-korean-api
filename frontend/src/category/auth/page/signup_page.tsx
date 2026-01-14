import { z } from "zod";
import { useNavigate } from "react-router-dom";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { Loader2 } from "lucide-react";
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
import { Checkbox } from "@/components/ui/checkbox";
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

// 🚨 [수정됨] User -> Auth로 경로 변경 (Critical!)
import { signupReqSchema, type SignupReq } from "@/category/auth/types";

// (주의: Hook 내부에서도 auth_api를 참조하는지 확인 필요)
import { useSignup } from "../hook/use_signup";

// Form Schema Refinement (약관 동의 검증)
const signupFormSchema = signupReqSchema.superRefine((data, ctx) => {
  if (!data.terms_service) {
    ctx.addIssue({
      code: z.ZodIssueCode.custom,
      path: ["terms_service"],
      message: "이용약관에 동의해야 합니다.",
    });
  }

  if (!data.terms_personal) {
    ctx.addIssue({
      code: z.ZodIssueCode.custom,
      path: ["terms_personal"],
      message: "개인정보 처리방침에 동의해야 합니다.",
    });
  }
});

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

export function SignupPage() {
  const navigate = useNavigate();
  const signupMutation = useSignup();

  const form = useForm<SignupReq>({
    resolver: zodResolver(signupFormSchema),
    mode: "onChange",
    defaultValues: {
      email: "",
      password: "",
      name: "",
      nickname: "",
      birthday: "",
      terms_service: false,
      terms_personal: false,
      // Select 컴포넌트 제어를 위해 undefined 방지 (필요 시 추가)
      language: "ko", 
      country: "KR",
      gender: "male",
    },
  });

  const onSubmit = (values: SignupReq) => {
    signupMutation.mutate(values, {
      onSuccess: () => {
        toast.success("회원가입을 축하합니다! 로그인해주세요.");
        navigate("/login"); // 가입 후 보통 로그인 페이지로 이동
      },
    });
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2">
          <CardTitle>회원가입</CardTitle>
          <CardDescription>
            새 계정을 만들고 Amazing Korean을 시작하세요.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form
              onSubmit={form.handleSubmit(onSubmit)}
              className="space-y-4"
            >
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
                        placeholder="8자 이상 입력하세요"
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
                      <Input type="date" placeholder="YYYY-MM-DD" {...field} />
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

              {/* 이용약관 */}
              <FormField
                control={form.control}
                name="terms_service"
                render={({ field }) => (
                  <FormItem className="space-y-1">
                    <div className="flex items-center gap-2">
                      <FormControl>
                        <Checkbox
                          checked={field.value}
                          onCheckedChange={field.onChange}
                          // Checkbox ref 전달 시 에러 방지용
                          ref={field.ref} 
                        />
                      </FormControl>
                      <FormLabel className="text-sm font-normal cursor-pointer">
                        이용약관에 동의합니다.
                      </FormLabel>
                    </div>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 개인정보 처리방침 */}
              <FormField
                control={form.control}
                name="terms_personal"
                render={({ field }) => (
                  <FormItem className="space-y-1">
                    <div className="flex items-center gap-2">
                      <FormControl>
                        <Checkbox
                          checked={field.value}
                          onCheckedChange={field.onChange}
                          ref={field.ref}
                        />
                      </FormControl>
                      <FormLabel className="text-sm font-normal cursor-pointer">
                        개인정보 처리방침에 동의합니다.
                      </FormLabel>
                    </div>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <Button
                type="submit"
                className="w-full"
                disabled={signupMutation.isPending}
              >
                {signupMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin mr-2" />
                    가입하는 중...
                  </>
                ) : (
                  "가입하기"
                )}
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}