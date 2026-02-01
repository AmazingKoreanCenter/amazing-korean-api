import { FileQuestion } from "lucide-react";
import { Link, useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

export function NotFoundPage() {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen flex items-center justify-center bg-background p-4">
      <Card className="w-full max-w-md text-center">
        <CardHeader className="space-y-4">
          <div className="mx-auto w-16 h-16 rounded-full bg-muted flex items-center justify-center">
            <FileQuestion className="w-8 h-8 text-muted-foreground" />
          </div>
          <CardTitle className="text-2xl">페이지를 찾을 수 없음</CardTitle>
          <CardDescription className="text-base">
            요청하신 페이지가 존재하지 않거나
            <br />
            이동되었을 수 있습니다.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <p className="text-sm text-muted-foreground">
            주소가 올바른지 확인해 주세요.
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
