import { ShieldX } from "lucide-react";
import { Link, useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

export function AccessDeniedPage() {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen flex items-center justify-center bg-background p-4">
      <Card className="w-full max-w-md text-center">
        <CardHeader className="space-y-4">
          <div className="mx-auto w-16 h-16 rounded-full bg-destructive/10 flex items-center justify-center">
            <ShieldX className="w-8 h-8 text-destructive" />
          </div>
          <CardTitle className="text-2xl">접근 권한 없음</CardTitle>
          <CardDescription className="text-base">
            이 페이지에 접근할 권한이 없습니다.
            <br />
            관리자 권한이 필요한 페이지입니다.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <p className="text-sm text-muted-foreground">
            잘못된 접근이라고 생각되시면 관리자에게 문의해 주세요.
          </p>
          <div className="flex flex-col sm:flex-row gap-2 justify-center">
            <Button variant="outline" onClick={() => navigate(-1)}>
              이전 페이지
            </Button>
            <Button asChild>
              <Link to="/">홈으로 이동</Link>
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
