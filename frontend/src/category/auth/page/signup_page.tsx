import { z } from "zod";
import { useNavigate } from "react-router-dom";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { Loader2, ChevronDown, ChevronUp } from "lucide-react";
import { toast } from "sonner";
import { useState } from "react";
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
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";

import { signupReqSchema, type SignupReq } from "@/category/auth/types";
import { useSignup } from "../hook/use_signup";
import { useGoogleLogin } from "../hook/use_google_login";

// Form Schema Refinement (약관 동의 검증)
const signupFormSchema = signupReqSchema.superRefine((data, ctx) => {
  if (!data.terms_service) {
    ctx.addIssue({
      code: z.ZodIssueCode.custom,
      path: ["terms_service"],
      message: i18n.t("auth.termsServiceRequired"),
    });
  }

  if (!data.terms_personal) {
    ctx.addIssue({
      code: z.ZodIssueCode.custom,
      path: ["terms_personal"],
      message: i18n.t("auth.termsPersonalRequired"),
    });
  }
});

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
  const { t } = useTranslation();
  const navigate = useNavigate();
  const signupMutation = useSignup();
  const googleLoginMutation = useGoogleLogin();
  const [showEmailForm, setShowEmailForm] = useState(false);

  const genderOptions = [
    { value: "male", label: t("auth.genderMale") },
    { value: "female", label: t("auth.genderFemale") },
    { value: "other", label: t("auth.genderOther") },
  ];

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
      language: "ko",
      country: "KR",
      gender: "male",
    },
  });

  const onSubmit = (values: SignupReq) => {
    signupMutation.mutate(values, {
      onSuccess: () => {
        toast.success(t("auth.signupSuccess"));
        navigate("/login");
      },
    });
  };

  const handleGoogleSignup = () => {
    googleLoginMutation.mutate();
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2 text-center">
          <CardTitle className="text-2xl">{t("auth.signupTitle")}</CardTitle>
          <CardDescription>
            {t("auth.signupDescription")}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* 소셜 회원가입 버튼 */}
          <div className="space-y-3">
            {/* Google 회원가입 */}
            <Button
              type="button"
              variant="outline"
              className="w-full h-12 text-base"
              onClick={handleGoogleSignup}
              disabled={googleLoginMutation.isPending}
            >
              {googleLoginMutation.isPending ? (
                <>
                  <Loader2 className="h-5 w-5 animate-spin mr-3" />
                  {t("auth.signupWithGoogleLoading")}
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
                  {t("auth.signupWithGoogle")}
                </>
              )}
            </Button>

            {/* Apple 회원가입 (비활성화) */}
            <Button
              type="button"
              variant="outline"
              className="w-full h-12 text-base"
              disabled
            >
              <svg className="h-5 w-5 mr-3" viewBox="0 0 24 24" fill="currentColor">
                <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z" />
              </svg>
              {t("auth.signupWithApple")}
            </Button>
          </div>

          {/* 소셜 가입 안내 */}
          <p className="text-center text-xs text-muted-foreground whitespace-pre-line">
            {t("auth.socialSignupNotice")}
          </p>

          {/* 구분선 */}
          <div className="relative my-6">
            <div className="absolute inset-0 flex items-center">
              <span className="w-full border-t" />
            </div>
            <div className="relative flex justify-center text-xs uppercase">
              <span className="bg-background px-2 text-muted-foreground">
                {t("common.or")}
              </span>
            </div>
          </div>

          {/* 이메일 회원가입 (접이식) */}
          <Collapsible open={showEmailForm} onOpenChange={setShowEmailForm}>
            <CollapsibleTrigger asChild>
              <Button
                variant="ghost"
                className="w-full justify-between text-muted-foreground hover:text-foreground"
              >
                {t("auth.signupWithEmailButton")}
                {showEmailForm ? (
                  <ChevronUp className="h-4 w-4" />
                ) : (
                  <ChevronDown className="h-4 w-4" />
                )}
              </Button>
            </CollapsibleTrigger>
            <CollapsibleContent className="pt-4">
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
                        <FormLabel>{t("auth.emailLabel")}</FormLabel>
                        <FormControl>
                          <Input
                            type="email"
                            placeholder={t("auth.emailPlaceholder")}
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
                        <FormLabel>{t("auth.passwordLabel")}</FormLabel>
                        <FormControl>
                          <Input
                            type="password"
                            placeholder={t("auth.passwordMinPlaceholder")}
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
                        <FormLabel>{t("auth.nameLabel")}</FormLabel>
                        <FormControl>
                          <Input
                            placeholder={t("auth.namePlaceholder")}
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
                        <FormLabel>{t("auth.nicknameLabel")}</FormLabel>
                        <FormControl>
                          <Input placeholder={t("auth.nicknamePlaceholder")} {...field} />
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
                        <FormLabel>{t("auth.birthdayLabel")}</FormLabel>
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
                        <FormLabel>{t("auth.genderLabel")}</FormLabel>
                        <Select
                          onValueChange={field.onChange}
                          defaultValue={field.value}
                        >
                          <FormControl>
                            <SelectTrigger>
                              <SelectValue placeholder={t("auth.genderPlaceholder")} />
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
                        <FormLabel>{t("auth.countryLabel")}</FormLabel>
                        <Select
                          onValueChange={field.onChange}
                          defaultValue={field.value}
                        >
                          <FormControl>
                            <SelectTrigger>
                              <SelectValue placeholder={t("auth.countryPlaceholder")} />
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
                        <FormLabel>{t("auth.languageLabel")}</FormLabel>
                        <Select
                          onValueChange={field.onChange}
                          defaultValue={field.value}
                        >
                          <FormControl>
                            <SelectTrigger>
                              <SelectValue placeholder={t("auth.languagePlaceholder")} />
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
                              ref={field.ref}
                            />
                          </FormControl>
                          <FormLabel className="text-sm font-normal cursor-pointer">
                            {t("auth.termsServiceAgree")}
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
                            {t("auth.termsPersonalAgree")}
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
                        {t("auth.signingUp")}
                      </>
                    ) : (
                      t("auth.signupButton")
                    )}
                  </Button>
                </form>
              </Form>
            </CollapsibleContent>
          </Collapsible>

          {/* 로그인 안내 */}
          <p className="text-center text-sm text-muted-foreground pt-4">
            {t("auth.haveAccount")}{" "}
            <a
              href="/login"
              className="text-primary underline-offset-4 hover:underline font-medium"
            >
              {t("auth.loginTitle")}
            </a>
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
