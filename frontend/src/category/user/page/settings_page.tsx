import { useEffect, useState } from "react";

import { ApiError } from "@/api/client";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import type { UserSetting } from "@/category/user/types";

import { useUserSettings } from "../hook/use_user_settings";

type LocalSettings = {
  theme: UserSetting["theme"];
  is_email_marketing_agreed: boolean;
  language: UserSetting["language"];
};

const defaultSettings: LocalSettings = {
  theme: "system",
  is_email_marketing_agreed: false,
  language: "ko",
};

const themeOptions: Array<{ value: LocalSettings["theme"]; label: string }> = [
  { value: "light", label: "Light" },
  { value: "dark", label: "Dark" },
  { value: "system", label: "System" },
];

const languageOptions: Array<{ value: LocalSettings["language"]; label: string }> = [
  { value: "ko", label: "Korean" },
  { value: "en", label: "English" },
];

export function SettingsPage() {
  const { data, isLoading, error } = useUserSettings();
  const [settings, setSettings] = useState<LocalSettings>(defaultSettings);

  useEffect(() => {
    if (data) {
      setSettings({
        theme: data.theme,
        is_email_marketing_agreed: data.is_email_marketing_agreed,
        language: data.language,
      });
      return;
    }

    if (error instanceof ApiError && error.status === 404) {
      setSettings(defaultSettings);
    }
  }, [data, error]);

  if (isLoading) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-lg">
          <CardHeader>
            <CardTitle>환경 설정</CardTitle>
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

  if (error && !(error instanceof ApiError && error.status === 404)) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-lg">
          <CardHeader>
            <CardTitle>환경 설정</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-destructive">
              설정 정보를 불러오지 못했습니다.
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
          <CardTitle>환경 설정</CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="space-y-2">
            <Label htmlFor="theme">테마</Label>
            <Select
              value={settings.theme}
              onValueChange={(value) =>
                setSettings((prev) => ({
                  ...prev,
                  theme: value as LocalSettings["theme"],
                }))
              }
            >
              <SelectTrigger id="theme">
                <SelectValue placeholder="테마를 선택하세요" />
              </SelectTrigger>
              <SelectContent>
                {themeOptions.map((option) => (
                  <SelectItem key={option.value} value={option.value}>
                    {option.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="flex items-center justify-between gap-4 rounded-lg border p-4">
            <div>
              <Label htmlFor="marketing">마케팅 이메일 수신</Label>
              <p className="text-sm text-muted-foreground">
                새로운 소식과 혜택을 이메일로 받아보세요.
              </p>
            </div>
            <Switch
              id="marketing"
              checked={settings.is_email_marketing_agreed}
              onCheckedChange={(checked) =>
                setSettings((prev) => ({
                  ...prev,
                  is_email_marketing_agreed: checked,
                }))
              }
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="language">언어</Label>
            <Select
              value={settings.language}
              onValueChange={(value) =>
                setSettings((prev) => ({
                  ...prev,
                  language: value as LocalSettings["language"],
                }))
              }
            >
              <SelectTrigger id="language">
                <SelectValue placeholder="언어를 선택하세요" />
              </SelectTrigger>
              <SelectContent>
                {languageOptions.map((option) => (
                  <SelectItem key={option.value} value={option.value}>
                    {option.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
