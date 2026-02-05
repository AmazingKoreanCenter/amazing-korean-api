import { useEffect } from "react";
import type { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2, ArrowLeft } from "lucide-react";
import { useForm } from "react-hook-form";
import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";

import { changeLanguage } from "@/i18n";
import { ApiError } from "@/api/client";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Form,
  FormControl,
  FormDescription,
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
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import { settingsUpdateReqSchema } from "@/category/user/types";

import { useUpdateSettings } from "../hook/use_update_settings";
import { useUserSettings } from "../hook/use_user_settings";

type SettingsForm = z.infer<typeof settingsUpdateReqSchema>;

const getDefaultTimezone = () => {
  return Intl.DateTimeFormat().resolvedOptions().timeZone || "Asia/Seoul";
};

const defaultSettings: SettingsForm = {
  user_set_language: "ko",
  user_set_note_email: false,
  user_set_note_push: false,
  user_set_timezone: getDefaultTimezone(),
};

const languageOptions: Array<{
  value: NonNullable<SettingsForm["user_set_language"]>;
  label: string;
}> = [
  { value: "ko", label: "Korean" },
  { value: "en", label: "English" },
];

export function SettingsPage() {
  const { t, i18n } = useTranslation();
  const { data, isLoading, error } = useUserSettings();
  const isNotFound = error instanceof ApiError && error.status === 404;

  const form = useForm<SettingsForm>({
    resolver: zodResolver(settingsUpdateReqSchema),
    mode: "onChange",
    defaultValues: defaultSettings,
  });

  const updateMutation = useUpdateSettings({
    onSuccess: (values) => {
      form.reset({
        user_set_language:
          values.user_set_language ?? defaultSettings.user_set_language,
        user_set_note_email:
          values.user_set_note_email ?? defaultSettings.user_set_note_email,
        user_set_note_push:
          values.user_set_note_push ?? defaultSettings.user_set_note_push,
        user_set_timezone:
          values.user_set_timezone ?? defaultSettings.user_set_timezone,
      });
      if (values.user_set_language) {
        changeLanguage(values.user_set_language);
      }
    },
  });

  useEffect(() => {
    if (data) {
      form.reset({
        user_set_language: data.user_set_language ?? defaultSettings.user_set_language,
        user_set_note_email: data.user_set_note_email ?? defaultSettings.user_set_note_email,
        user_set_note_push: data.user_set_note_push ?? defaultSettings.user_set_note_push,
        user_set_timezone: data.user_set_timezone ?? defaultSettings.user_set_timezone,
      });
      return;
    }

    if (isNotFound) {
      form.reset(defaultSettings);
    }
  }, [data, form, isNotFound]);

  // 헤더 언어 토글 변경 시 form에 즉시 반영
  useEffect(() => {
    const lang = i18n.language;
    if ((lang === "ko" || lang === "en") && form.getValues("user_set_language") !== lang) {
      form.setValue("user_set_language", lang, { shouldDirty: false });
    }
  }, [i18n.language, form]);

  const controlsDisabled = updateMutation.isPending;

  const onSubmit = (values: SettingsForm) => {
    updateMutation.mutate(values);
  };

  if (isLoading) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-lg">
          <CardHeader>
            <CardTitle>{t("user.settingsTitle")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="space-y-2">
              <Skeleton className="h-4 w-24" />
              <Skeleton className="h-10 w-full" />
            </div>
            <div className="flex items-center justify-between">
              <div className="space-y-2">
                <Skeleton className="h-4 w-32" />
                <Skeleton className="h-4 w-40" />
              </div>
              <Skeleton className="h-6 w-11 rounded-full" />
            </div>
            <div className="space-y-2">
              <Skeleton className="h-4 w-24" />
              <Skeleton className="h-10 w-full" />
            </div>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (error && !isNotFound) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-lg">
          <CardHeader>
            <CardTitle>{t("user.settingsTitle")}</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-destructive">
              {t("user.settingsLoadFailed")}
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-lg">
        <CardHeader>
          <CardTitle>{t("user.settingsTitle")}</CardTitle>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
              <FormField
                control={form.control}
                name="user_set_note_email"
                render={({ field }) => (
                  <FormItem className="space-y-4 rounded-lg border p-4">
                    <div className="flex items-center justify-between gap-4">
                      <div>
                        <FormLabel>{t("user.emailMarketing")}</FormLabel>
                        <FormDescription>
                          {t("user.emailMarketingDescription")}
                        </FormDescription>
                      </div>
                      <FormControl>
                        <Switch
                          checked={field.value ?? false}
                          onCheckedChange={field.onChange}
                          disabled={controlsDisabled}
                        />
                      </FormControl>
                    </div>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="user_set_note_push"
                render={({ field }) => (
                  <FormItem className="space-y-4 rounded-lg border p-4">
                    <div className="flex items-center justify-between gap-4">
                      <div>
                        <FormLabel>{t("user.pushNotifications")}</FormLabel>
                        <FormDescription>
                          {t("user.pushNotificationsDescription")}
                        </FormDescription>
                      </div>
                      <FormControl>
                        <Switch
                          checked={field.value ?? false}
                          onCheckedChange={field.onChange}
                          disabled={controlsDisabled}
                        />
                      </FormControl>
                    </div>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="user_set_language"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>{t("user.languageLabel")}</FormLabel>
                    <Select
                      value={field.value ?? "ko"}
                      onValueChange={field.onChange}
                      disabled={controlsDisabled}
                    >
                      <FormControl>
                        <SelectTrigger>
                        <SelectValue placeholder={t("user.languageDescription")} />
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

              <FormField
                control={form.control}
                name="user_set_timezone"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>{t("user.timezoneLabel")}</FormLabel>
                    <FormControl>
                      <Input
                        value={field.value ?? ""}
                        onChange={field.onChange}
                        disabled
                      />
                    </FormControl>
                    <FormDescription>
                      {t("user.timezoneDescription")}
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <Button
                type="submit"
                className="w-full"
                disabled={!form.formState.isDirty || updateMutation.isPending}
              >
                {updateMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin" />
                    {t("common.saving")}
                  </>
                ) : (
                  t("common.save")
                )}
              </Button>

              <div className="flex items-center justify-center text-sm pt-2">
                <Link
                  to="/user/me"
                  className="text-muted-foreground underline-offset-4 hover:underline flex items-center gap-1"
                >
                  <ArrowLeft className="h-4 w-4" />
                  {t("auth.backToMyPage")}
                </Link>
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
