import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { useState } from "react";
import { toast } from "sonner";

import i18n from "@/i18n";
import { ApiError } from "@/api/client";
import type { LoginReq, LoginRes, MfaChallengeRes } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";

import { login } from "../auth_api";

// MFA 챌린지 응답 여부 판별
const isMfaChallenge = (data: LoginRes | MfaChallengeRes): data is MfaChallengeRes =>
  "mfa_required" in data && data.mfa_required === true;

const statusMessageMap: Record<number, string> = {
  400: "auth.statusBadRequest",
  401: "auth.statusUnauthorized",
  423: "auth.statusLocked",
  429: "auth.statusTooMany",
  500: "auth.statusServerError",
};

export interface SocialOnlyError {
  isSocialOnly: true;
  providers: string[];
}

export interface EmailNotVerifiedError {
  isEmailNotVerified: true;
  email: string;
}

export interface MfaPending {
  mfa_token: string;
  user_id: number;
}

// 소셜 전용 계정 에러 파싱
const parseSocialOnlyError = (error: unknown): SocialOnlyError | null => {
  if (error instanceof ApiError && error.status === 401) {
    // 에러 메시지 형식: "AUTH_401_SOCIAL_ONLY_ACCOUNT:google,apple"
    if (error.message.startsWith("AUTH_401_SOCIAL_ONLY_ACCOUNT:")) {
      const providers = error.message
        .replace("AUTH_401_SOCIAL_ONLY_ACCOUNT:", "")
        .split(",")
        .filter(Boolean);
      return { isSocialOnly: true, providers };
    }
  }
  return null;
};

// 이메일 미인증 에러 파싱
const parseEmailNotVerifiedError = (error: unknown): EmailNotVerifiedError | null => {
  if (error instanceof ApiError && error.status === 403) {
    // 에러 메시지 형식: "AUTH_403_EMAIL_NOT_VERIFIED:user@example.com"
    if (error.message.startsWith("AUTH_403_EMAIL_NOT_VERIFIED:")) {
      const email = error.message.replace("AUTH_403_EMAIL_NOT_VERIFIED:", "");
      return { isEmailNotVerified: true, email };
    }
  }
  return null;
};

const getErrorMessage = (error: unknown) => {
  // 소셜 전용 계정 / 이메일 미인증 에러는 별도 처리 (toast 표시 안함)
  if (parseSocialOnlyError(error)) return null;
  if (parseEmailNotVerifiedError(error)) return null;

  if (error instanceof ApiError) {
    const key = statusMessageMap[error.status];
    return key ? i18n.t(key) : error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return i18n.t("common.requestFailed");
};

export const useLogin = () => {
  const navigate = useNavigate();
  const [mfaPending, setMfaPending] = useState<MfaPending | null>(null);

  const mutation = useMutation({
    mutationFn: (data: LoginReq) => login(data),
    onSuccess: (data) => {
      // MFA 챌린지 응답인 경우
      if (isMfaChallenge(data)) {
        setMfaPending({ mfa_token: data.mfa_token, user_id: data.user_id });
        return;
      }

      // 일반 로그인 성공
      useAuthStore.getState().login(data);
      toast.success(i18n.t("auth.toastLoginSuccess"));
      navigate("/about");
    },
    onError: (error) => {
      // 이메일 미인증 → 인증 페이지로 이동
      const emailError = parseEmailNotVerifiedError(error);
      if (emailError) {
        toast.warning(i18n.t("auth.toastEmailNotVerified"));
        navigate("/verify-email", {
          state: { email: emailError.email },
          replace: true,
        });
        return;
      }

      const message = getErrorMessage(error);
      if (message) {
        toast.error(message);
      }
    },
  });

  // 소셜 전용 계정 에러 상태
  const socialOnlyError = mutation.error
    ? parseSocialOnlyError(mutation.error)
    : null;

  const clearMfaPending = () => setMfaPending(null);

  return {
    ...mutation,
    socialOnlyError,
    mfaPending,
    setMfaPending,
    clearMfaPending,
  };
};
