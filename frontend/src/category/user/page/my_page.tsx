import { useEffect, useMemo, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import {
  Loader2,
  Pencil,
  Settings,
  PartyPopper,
  User,
  Mail,
  Calendar,
  Globe,
  Languages,
  Shield,
  KeyRound,
  AtSign,
} from "lucide-react";

import { ApiError } from "@/api/client";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";

import { useUserMe } from "../hook/use_user_me";
import { useUpdateUser } from "../hook/use_update_user";
import { updateUserReqSchema, type UpdateUserReq } from "../types";

const formatDate = (value: string | null) => {
  if (!value) return "-";
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;
  return date.toLocaleDateString("ko-KR", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
};

const languageOptions = [
  { value: "ko", label: "한국어" },
  { value: "en", label: "English" },
  { value: "ja", label: "日本語" },
  { value: "zh", label: "中文" },
];

const countryOptions = [
  { value: "KR", label: "대한민국" },
  { value: "US", label: "United States" },
  { value: "JP", label: "Japan" },
  { value: "CN", label: "China" },
];

export function MyPage() {
  const { t } = useTranslation();
  const { data, isLoading, error, refetch } = useUserMe();
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();
  const [isEditing, setIsEditing] = useState(false);

  const genderLabels: Record<string, string> = {
    male: t("auth.genderMale"),
    female: t("auth.genderFemale"),
    other: t("auth.genderOther"),
  };

  const genderOptions = [
    { value: "male", label: t("auth.genderMale") },
    { value: "female", label: t("auth.genderFemale") },
    { value: "other", label: t("auth.genderOther") },
  ];

  const authLabels: Record<string, string> = {
    learner: t("user.authLearner"),
    manager: t("user.authManager"),
    admin: t("user.authAdmin"),
    hymn: t("user.authHymn"),
  };

  // 신규 OAuth 사용자 환영 메시지
  const isNewUser = searchParams.get("welcome") === "true";

  const updateUserMutation = useUpdateUser({
    onConflict: () => {
      form.setError("nickname", { message: t("user.nicknameConflict") });
    },
  });

  const form = useForm<UpdateUserReq>({
    resolver: zodResolver(updateUserReqSchema),
    mode: "onChange",
    defaultValues: {
      nickname: "",
      language: "",
      country: "",
      birthday: "",
      gender: undefined,
    },
  });

  const fallbackInitial = useMemo(() => {
    if (!data) return "?";
    const seed = data.nickname || data.name || data.email;
    return seed?.trim().charAt(0).toUpperCase() || "?";
  }, [data]);

  // 데이터 로드 시 폼 초기화
  useEffect(() => {
    if (data) {
      form.reset({
        nickname: data.nickname || "",
        language: data.language || "",
        country: data.country || "",
        birthday: data.birthday || "",
        gender: data.gender,
      });
    }
  }, [data, form]);

  // 인증 에러 처리
  useEffect(() => {
    if (error instanceof ApiError && error.status === 401) {
      toast.error(t("common.loginRequired"));
      navigate("/login", { replace: true });
    }
  }, [error, navigate, t]);

  const dismissWelcome = () => {
    setSearchParams({});
  };

  const handleEdit = () => {
    setIsEditing(true);
  };

  const handleCancel = () => {
    setIsEditing(false);
    if (data) {
      form.reset({
        nickname: data.nickname || "",
        language: data.language || "",
        country: data.country || "",
        birthday: data.birthday || "",
        gender: data.gender,
      });
    }
  };

  const onSubmit = (values: UpdateUserReq) => {
    updateUserMutation.mutate(values, {
      onSuccess: () => {
        setIsEditing(false);
        void refetch();
      },
    });
  };

  if (isLoading) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-2xl">
          <CardHeader>
            <CardTitle>{t("user.myPageTitle")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="flex items-center gap-4">
              <Skeleton className="h-20 w-20 rounded-full" />
              <div className="space-y-2">
                <Skeleton className="h-6 w-48" />
                <Skeleton className="h-4 w-32" />
              </div>
            </div>
            <Separator />
            <div className="space-y-4">
              {[...Array(6)].map((_, i) => (
                <div key={i} className="flex justify-between">
                  <Skeleton className="h-4 w-24" />
                  <Skeleton className="h-4 w-40" />
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (error instanceof ApiError && error.status === 401) {
    return null;
  }

  if (error instanceof ApiError && error.status === 404) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-2xl">
          <CardHeader>
            <CardTitle>{t("user.myPageTitle")}</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-destructive">
              {t("user.userNotFound")}
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (!data) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-2xl">
          <CardHeader>
            <CardTitle>{t("user.myPageTitle")}</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              {t("user.userLoadFailed")}
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-2xl">
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle className="text-2xl">{t("user.myPageTitle")}</CardTitle>
              <CardDescription>{t("user.myPageDescription")}</CardDescription>
            </div>
            {!isEditing && (
              <div className="flex items-center gap-2">
                <Button variant="outline" size="sm" onClick={handleEdit}>
                  <Pencil className="h-4 w-4 mr-2" />
                  {t("common.edit")}
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => navigate("/user/settings")}
                >
                  <Settings className="h-4 w-4 mr-2" />
                  {t("user.settingsButton")}
                </Button>
              </div>
            )}
          </div>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* 신규 사용자 환영 메시지 */}
          {isNewUser && (
            <div className="rounded-lg border border-emerald-200 bg-emerald-50 p-4">
              <div className="flex items-start gap-3">
                <PartyPopper className="h-5 w-5 text-emerald-600 mt-0.5 shrink-0" />
                <div className="flex-1 space-y-1">
                  <p className="text-sm font-medium text-emerald-800">
                    {t("user.welcomeTitle")}
                  </p>
                  <p className="text-sm text-emerald-700">
                    {t("user.welcomeDescription")}
                  </p>
                </div>
                <button
                  type="button"
                  onClick={dismissWelcome}
                  className="text-emerald-600 hover:text-emerald-800 text-sm"
                >
                  ✕
                </button>
              </div>
            </div>
          )}

          {/* 프로필 헤더 */}
          <div className="flex items-center gap-4">
            <Avatar className="h-20 w-20">
              <AvatarFallback className="text-2xl">{fallbackInitial}</AvatarFallback>
            </Avatar>
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <h2 className="text-xl font-semibold">
                  {data.nickname || data.name || t("user.defaultUser")}
                </h2>
              </div>
              <Badge variant="secondary">
                {authLabels[data.user_auth] || data.user_auth}
              </Badge>
            </div>
          </div>

          <Separator />

          {isEditing ? (
            /* 수정 모드 */
            <Form {...form}>
              <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
                <FormField
                  control={form.control}
                  name="nickname"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("user.fieldNickname")}</FormLabel>
                      <FormControl>
                        <Input placeholder={t("user.nicknamePlaceholder")} {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <FormField
                  control={form.control}
                  name="birthday"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("user.fieldBirthday")}</FormLabel>
                      <FormControl>
                        <Input type="date" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <FormField
                  control={form.control}
                  name="gender"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("user.fieldGender")}</FormLabel>
                      <Select onValueChange={field.onChange} value={field.value}>
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

                <FormField
                  control={form.control}
                  name="country"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("user.fieldCountry")}</FormLabel>
                      <Select onValueChange={field.onChange} value={field.value || ""}>
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

                <FormField
                  control={form.control}
                  name="language"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("user.fieldLanguage")}</FormLabel>
                      <Select onValueChange={field.onChange} value={field.value || ""}>
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

                <div className="flex gap-2 pt-4">
                  <Button
                    type="submit"
                    className="flex-1"
                    disabled={updateUserMutation.isPending}
                  >
                    {updateUserMutation.isPending ? (
                      <>
                        <Loader2 className="h-4 w-4 animate-spin mr-2" />
                        {t("common.saving")}
                      </>
                    ) : (
                      t("common.save")
                    )}
                  </Button>
                  <Button
                    type="button"
                    variant="outline"
                    className="flex-1"
                    onClick={handleCancel}
                    disabled={updateUserMutation.isPending}
                  >
                    {t("common.cancel")}
                  </Button>
                </div>
              </form>
            </Form>
          ) : (
            /* 보기 모드 */
            <div className="space-y-4">
              <div className="grid gap-4">
                {/* 닉네임 */}
                <div className="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
                  <AtSign className="h-5 w-5 text-muted-foreground" />
                  <div className="flex-1">
                    <p className="text-sm text-muted-foreground">{t("user.fieldNickname")}</p>
                    <p className="font-medium">{data.nickname || "-"}</p>
                  </div>
                </div>

                {/* 이름 */}
                <div className="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
                  <User className="h-5 w-5 text-muted-foreground" />
                  <div className="flex-1">
                    <p className="text-sm text-muted-foreground">{t("user.fieldName")}</p>
                    <p className="font-medium">{data.name}</p>
                  </div>
                </div>

                {/* 이메일 */}
                <div className="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
                  <Mail className="h-5 w-5 text-muted-foreground" />
                  <div className="flex-1">
                    <p className="text-sm text-muted-foreground">{t("user.fieldEmail")}</p>
                    <p className="font-medium">{data.email}</p>
                  </div>
                </div>

                {/* 가입일 */}
                <div className="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
                  <Calendar className="h-5 w-5 text-muted-foreground" />
                  <div className="flex-1">
                    <p className="text-sm text-muted-foreground">{t("user.fieldJoinDate")}</p>
                    <p className="font-medium">{formatDate(data.created_at)}</p>
                  </div>
                </div>

                {/* 생년월일 */}
                <div className="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
                  <Calendar className="h-5 w-5 text-muted-foreground" />
                  <div className="flex-1">
                    <p className="text-sm text-muted-foreground">{t("user.fieldBirthday")}</p>
                    <p className="font-medium">{formatDate(data.birthday)}</p>
                  </div>
                </div>

                {/* 언어 */}
                <div className="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
                  <Languages className="h-5 w-5 text-muted-foreground" />
                  <div className="flex-1">
                    <p className="text-sm text-muted-foreground">{t("user.fieldLanguage")}</p>
                    <p className="font-medium">
                      {languageOptions.find((l) => l.value === data.language)?.label ||
                        data.language ||
                        "-"}
                    </p>
                  </div>
                </div>

                {/* 국가 */}
                <div className="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
                  <Globe className="h-5 w-5 text-muted-foreground" />
                  <div className="flex-1">
                    <p className="text-sm text-muted-foreground">{t("user.fieldCountry")}</p>
                    <p className="font-medium">
                      {countryOptions.find((c) => c.value === data.country)?.label ||
                        data.country ||
                        "-"}
                    </p>
                  </div>
                </div>

                {/* 성별 */}
                <div className="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
                  <Shield className="h-5 w-5 text-muted-foreground" />
                  <div className="flex-1">
                    <p className="text-sm text-muted-foreground">{t("user.fieldGender")}</p>
                    <p className="font-medium">
                      {genderLabels[data.gender] || data.gender}
                    </p>
                  </div>
                </div>
              </div>

              {/* 비밀번호 재설정 버튼 (OAuth 전용 계정이 아닌 경우에만 표시) */}
              {data.has_password !== false && (
                <>
                  <Separator />
                  <Button
                    variant="outline"
                    className="w-full"
                    onClick={() => navigate("/request-reset-password")}
                  >
                    <KeyRound className="h-4 w-4 mr-2" />
                    {t("user.resetPassword")}
                  </Button>
                </>
              )}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
