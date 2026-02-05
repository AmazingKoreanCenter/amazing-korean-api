import { Loader2 } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Button } from "@/components/ui/button";

import { useLogout } from "../hook/use_logout";

export function LogoutButton() {
  const { t } = useTranslation();
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
          {t("nav.loggingOut")}
        </>
      ) : (
        t("nav.logout")
      )}
    </Button>
  );
}
