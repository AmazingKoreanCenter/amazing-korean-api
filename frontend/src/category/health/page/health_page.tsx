import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";

import { useHealth } from "../hook/use_health";

export function HealthPage() {
  const { data, isPending, isError, isFetching, refetch, error } = useHealth();
  const isSuccess = Boolean(data) && !isError;

  const badgeVariant = isError ? "destructive" : data ? "success" : "secondary";
  const badgeLabel = isError ? "offline" : data ? data.status : "checking";

  return (
    <div className="flex h-screen items-center justify-center bg-gradient-to-br from-slate-50 via-white to-emerald-50 px-4">
      <Card className="w-full max-w-md animate-in fade-in slide-in-from-bottom-4">
        <CardHeader className="space-y-3">
          <div className="flex items-center justify-between">
            <CardTitle>Health Check</CardTitle>
            <Badge variant={badgeVariant}>{badgeLabel}</Badge>
          </div>
          <CardDescription>
            Live status from the API health endpoint.
          </CardDescription>
        </CardHeader>
        <CardContent>
          {isPending && (
            <p className="text-sm text-muted-foreground">
              Checking Server Status...
            </p>
          )}
          {isError && (
            <div className="space-y-2">
              <p className="text-sm font-medium text-destructive">
                Server Offline
              </p>
              <p className="text-xs text-muted-foreground">
                {error instanceof Error
                  ? error.message
                  : "Please try again in a moment."}
              </p>
            </div>
          )}
          {isSuccess && data && (
            <div className="grid gap-4 text-sm">
              <div className="flex items-center justify-between">
                <span className="text-muted-foreground">uptime_ms</span>
                <span className="font-mono font-semibold">
                  {data.uptime_ms}
                </span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-muted-foreground">version</span>
                <span className="font-mono font-semibold">{data.version}</span>
              </div>
            </div>
          )}
        </CardContent>
        <CardFooter className="justify-end">
          <Button onClick={() => refetch()} disabled={isFetching}>
            {isFetching ? "새로고침 중..." : "새로고침"}
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}
