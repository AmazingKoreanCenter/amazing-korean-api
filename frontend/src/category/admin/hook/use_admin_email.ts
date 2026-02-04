import { useMutation } from "@tanstack/react-query";

import { sendTestEmail } from "../admin_api";
import type { TestEmailReq } from "../types";

/**
 * 테스트 이메일 발송 Mutation
 */
export const useSendTestEmail = () => {
  return useMutation({
    mutationFn: (data: TestEmailReq) => sendTestEmail(data),
  });
};
