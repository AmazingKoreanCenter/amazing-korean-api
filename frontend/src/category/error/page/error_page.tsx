import { ServerCrash } from "lucide-react";
import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

export function ErrorPage() {
  const handleRetry = () => {
    window.location.reload();
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-background p-4">
      <Card className="w-full max-w-md text-center">
        <CardHeader className="space-y-4">
          <div className="mx-auto w-16 h-16 rounded-full bg-orange-100 dark:bg-orange-900/20 flex items-center justify-center">
            <ServerCrash className="w-8 h-8 text-orange-600 dark:text-orange-400" />
          </div>
          <CardTitle className="text-2xl">문제가 발생했습니다</CardTitle>
          <CardDescription className="text-base">
            서버에 일시적인 문제가 발생했습니다.
            <br />
            잠시 후 다시 시도해 주세요.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <p className="text-sm text-muted-foreground">
            문제가 지속되면 관리자에게 문의해 주세요.
          </p>
          <div className="flex flex-col sm:flex-row gap-2 justify-center">
            <Button variant="outline" onClick={handleRetry}>
              다시 시도
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
