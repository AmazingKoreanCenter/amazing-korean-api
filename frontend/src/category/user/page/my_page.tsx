import { useEffect, useMemo } from "react";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";

import { useUserMe } from "../hook/use_user_me";

const formatDate = (value: string) => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }

  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}.${month}.${day}`;
};

export function MyPage() {
  const { data, isLoading, error } = useUserMe();
  const navigate = useNavigate();

  const fallbackInitial = useMemo(() => {
    if (!data) {
      return "?";
    }
    const seed = data.nickname || data.name || data.email;
    return seed?.trim().charAt(0).toUpperCase() || "?";
  }, [data]);

  useEffect(() => {
    if (error instanceof ApiError && error.status === 401) {
      toast.error("로그인이 필요합니다");
      navigate("/login", { replace: true });
    }
  }, [error, navigate]);

  if (isLoading) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-lg">
          <CardHeader>
            <CardTitle>마이 페이지</CardTitle>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="flex items-center gap-4">
              <Skeleton className="h-16 w-16 rounded-full" />
              <div className="space-y-2">
                <Skeleton className="h-4 w-36" />
                <Skeleton className="h-4 w-24" />
              </div>
            </div>
            <div className="space-y-3">
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-full" />
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
        <Card className="w-full max-w-lg">
          <CardHeader>
            <CardTitle>마이 페이지</CardTitle>
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
            <CardTitle>마이 페이지</CardTitle>
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
        <CardHeader>
          <CardTitle>마이 페이지</CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="flex items-center gap-4">
            <Avatar className="h-16 w-16">
              <AvatarFallback>{fallbackInitial}</AvatarFallback>
            </Avatar>
            <div>
              <p className="text-lg font-semibold text-foreground">
                {data.nickname}
              </p>
              <p className="text-sm text-muted-foreground">{data.email}</p>
            </div>
          </div>
          <div className="space-y-3">
            <div className="flex items-center justify-between border-b pb-2 text-sm">
              <span className="text-muted-foreground">이메일</span>
              <span className="font-medium text-foreground">{data.email}</span>
            </div>
            <div className="flex items-center justify-between border-b pb-2 text-sm">
              <span className="text-muted-foreground">이름</span>
              <span className="font-medium text-foreground">{data.name}</span>
            </div>
            <div className="flex items-center justify-between border-b pb-2 text-sm">
              <span className="text-muted-foreground">닉네임</span>
              <span className="font-medium text-foreground">
                {data.nickname}
              </span>
            </div>
            <div className="flex items-center justify-between border-b pb-2 text-sm">
              <span className="text-muted-foreground">가입일</span>
              <span className="font-medium text-foreground">
                {formatDate(data.created_at)}
              </span>
            </div>
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">권한</span>
              <span className="font-medium text-foreground">{data.user_auth}</span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
