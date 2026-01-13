import { Loader2 } from "lucide-react";

import { Button } from "@/components/ui/button";

import { useLogout } from "../hook/use_logout";

export function LogoutButton() {
  const logoutMutation = useLogout();

  return (
    <Button
      type="button"
      variant="ghost"
      onClick={() => logoutMutation.mutate()}
      disabled={logoutMutation.isPending}
    >
      {logoutMutation.isPending ? (
        <>
          <Loader2 className="h-4 w-4 animate-spin" />
          Logging out...
        </>
      ) : (
        "로그아웃"
      )}
    </Button>
  );
}
