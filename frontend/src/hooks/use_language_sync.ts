import { useEffect, useRef } from "react";

import { changeLanguage } from "@/i18n";
import { useAuthStore } from "@/hooks/use_auth_store";
import { useUserSettings } from "@/category/user/hook/use_user_settings";

/**
 * DB 언어 설정 연동 Hook
 *
 * 로그인 상태에서 사용자 설정의 user_set_language를 가져와 i18n에 적용합니다.
 * - 로그인 시: DB의 언어 설정 → i18n + localStorage 적용
 * - 로그아웃 시: localStorage의 언어 유지
 */
export function useLanguageSync() {
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
  const { data: settings } = useUserSettings({ enabled: isLoggedIn });
  const appliedRef = useRef(false);

  useEffect(() => {
    if (!isLoggedIn) {
      appliedRef.current = false;
      return;
    }

    if (settings?.user_set_language && !appliedRef.current) {
      void changeLanguage(settings.user_set_language);
      appliedRef.current = true;
    }
  }, [isLoggedIn, settings]);
}
