/**
 * OAuth 콜백 처리 Hook
 *
 * OAuth 인증 완료 후 프론트엔드로 리다이렉트될 때의 콜백 처리를 담당합니다.
 *
 * ## 전체 OAuth 플로우
 *
 * 1단계: 사용자가 "Google로 계속하기" 클릭
 *   → useGoogleLogin hook이 GET /auth/google 호출
 *   → 백엔드가 auth_url 반환
 *   → window.location.href = auth_url (Google 로그인 페이지로 이동)
 *
 * 2단계: Google에서 계정 선택 및 동의
 *   → 사용자가 Google 계정 선택 후 동의
 *   → Google이 백엔드 콜백 URL로 리다이렉트 (GET /auth/google/callback)
 *
 * 3단계: 백엔드에서 OAuth 처리 후 프론트엔드로 리다이렉트
 *   → 백엔드: 토큰 교환 → 사용자 조회/생성 → 세션 생성 → 쿠키에 refresh_token 설정
 *   → 프론트엔드로 리다이렉트: /login?login=success&user_id=xxx&is_new_user=true|false
 *
 * 4단계: 이 Hook이 처리 (LoginPage에서 호출)
 *   → URL 파라미터 확인 (login=success, is_new_user, error 등)
 *   → refreshToken() 호출하여 access_token 획득
 *   → 로그인 상태 저장 후 적절한 페이지로 리다이렉트
 *     - 신규 사용자: /user/me?welcome=true (마이페이지 + 환영 메시지)
 *     - 기존 사용자: /about (소개 페이지)
 *
 * ## 경쟁 조건(Race Condition) 처리
 *
 * 페이지 로드 시 다음 두 가지가 동시에 발생할 수 있습니다:
 * - [A] 이 Hook의 refreshToken() 호출
 * - [B] Header 등 다른 컴포넌트의 API 호출 → 401 → axios interceptor의 refreshToken() 호출
 *
 * [B]가 먼저 완료되면 refresh token rotation으로 인해 [A]가 409 Conflict로 실패합니다.
 * 이 경우에도 interceptor가 이미 로그인 처리를 완료했으므로, isLoggedIn 상태를 확인하여 리다이렉트합니다.
 */

import { useEffect, useRef } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { toast } from "sonner";

import { useAuthStore } from "@/hooks/use_auth_store";

import { refreshToken } from "../auth_api";

interface UseOAuthCallbackReturn {
  /** OAuth 콜백 처리 중인지 여부 */
  isProcessing: boolean;
  /** URL에 OAuth 관련 파라미터가 있는지 여부 */
  hasOAuthParams: boolean;
}

export function useOAuthCallback(): UseOAuthCallbackReturn {
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();

  // ─────────────────────────────────────────────────────────────────────────
  // 중복 처리 방지용 Ref
  // React StrictMode에서 useEffect가 두 번 호출되는 것을 방지합니다.
  // ─────────────────────────────────────────────────────────────────────────
  const processedRef = useRef(false);

  // ─────────────────────────────────────────────────────────────────────────
  // URL 파라미터 추출
  // ─────────────────────────────────────────────────────────────────────────
  const loginSuccess = searchParams.get("login");
  const isNewUser = searchParams.get("is_new_user");
  const error = searchParams.get("error");
  const errorDescription = searchParams.get("error_description");

  const hasOAuthParams = !!(loginSuccess || error);

  useEffect(() => {
    // ───────────────────────────────────────────────────────────────────────
    // Step 1: 이미 처리 완료된 경우 스킵
    // ───────────────────────────────────────────────────────────────────────
    if (processedRef.current) {
      return;
    }

    // ───────────────────────────────────────────────────────────────────────
    // Step 2: OAuth 에러 처리
    // 백엔드에서 에러와 함께 리다이렉트된 경우 (예: 사용자가 동의 취소)
    // URL 예시: /login?error=access_denied&error_description=User%20cancelled
    // ───────────────────────────────────────────────────────────────────────
    if (error) {
      processedRef.current = true;
      const message = errorDescription
        ? decodeURIComponent(errorDescription)
        : "Google 로그인에 실패했습니다.";
      toast.error(message);
      setSearchParams({});
      return;
    }

    // ───────────────────────────────────────────────────────────────────────
    // Step 3: OAuth 성공 처리
    // URL 예시: /login?login=success&user_id=35&is_new_user=true
    // ───────────────────────────────────────────────────────────────────────
    if (loginSuccess === "success") {
      processedRef.current = true;

      // ─────────────────────────────────────────────────────────────────────
      // Step 3-1: 리다이렉트 목적지 결정 함수
      // 신규 사용자 → 마이페이지 (프로필 완성 유도)
      // 기존 사용자 → 소개 페이지
      // ─────────────────────────────────────────────────────────────────────
      const redirectAfterLogin = () => {
        toast.success("Google 로그인 성공!");
        setSearchParams({});

        if (isNewUser === "true") {
          // 신규 OAuth 사용자: 마이페이지로 이동 + 환영 메시지
          navigate("/user/me?welcome=true", { replace: true });
        } else {
          // 기존 사용자: 소개 페이지로 이동
          navigate("/about", { replace: true });
        }
      };

      // ─────────────────────────────────────────────────────────────────────
      // Step 3-2: 토큰 갱신 및 로그인 상태 저장
      // 백엔드가 설정한 HttpOnly 쿠키(refresh_token)를 사용하여
      // access_token을 획득하고 로그인 상태를 저장합니다.
      // ─────────────────────────────────────────────────────────────────────
      refreshToken()
        .then((data) => {
          // 성공: 로그인 상태 저장 후 리다이렉트
          useAuthStore.getState().login(data);
          redirectAfterLogin();
        })
        .catch(() => {
          // ───────────────────────────────────────────────────────────────
          // Step 3-3: 경쟁 조건 처리
          // refreshToken()이 실패해도 axios interceptor가 이미
          // 토큰 갱신 및 로그인 처리를 완료했을 수 있습니다.
          // 이 경우 isLoggedIn 상태를 확인하여 리다이렉트합니다.
          // ───────────────────────────────────────────────────────────────
          if (useAuthStore.getState().isLoggedIn) {
            redirectAfterLogin();
          } else {
            // 실제 실패: 에러 메시지 표시
            toast.error("로그인 세션을 가져오는데 실패했습니다.");
            processedRef.current = false; // 재시도 허용
            setSearchParams({});
          }
        });
    }
  }, [
    loginSuccess,
    isNewUser,
    error,
    errorDescription,
    navigate,
    setSearchParams,
  ]);

  return {
    isProcessing: processedRef.current,
    hasOAuthParams,
  };
}
