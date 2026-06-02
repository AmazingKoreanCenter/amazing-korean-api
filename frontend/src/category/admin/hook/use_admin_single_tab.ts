import { useEffect, useState } from "react";

/**
 * 관리자 단일 탭 강제 (관리자 세션 v2 ⑦).
 *
 * 우리 사이트 관리자 영역이 이미 다른 탭에서 열려 있는 상태에서 2번째 탭이 열리면,
 * **새로 연 탭만** 차단한다(기존 작업 탭은 그대로 유지 — 로그아웃/강제이동 아님).
 *
 * 메커니즘 (BroadcastChannel):
 *  - 진입 시 `hello` 브로드캐스트 → 이미 활성인 기존 탭이 `present` 로 응답.
 *  - `present` 를 받은 쪽(= 새 탭)이 스스로 차단. BroadcastChannel 은 송신자 본인에게
 *    메시지를 되돌리지 않으므로, 응답한 기존 탭은 차단되지 않는다.
 *  - 이미 차단된 탭은 다른 탭의 `hello` 에 응답하지 않는다(활성 holder 1개만 응답).
 *
 * 미지원 환경(BroadcastChannel 없음) → graceful: 차단하지 않음(접근 보존).
 * 백엔드 max 1 세션이 실제 보안 경계이고, 본 훅은 UX 가드다.
 */
const CHANNEL_NAME = "amk_admin_single_tab";

export function useAdminSingleTab(): boolean {
  const [blocked, setBlocked] = useState(false);

  useEffect(() => {
    if (typeof BroadcastChannel === "undefined") return;

    const channel = new BroadcastChannel(CHANNEL_NAME);
    let isBlocked = false;

    const onMessage = (event: MessageEvent) => {
      const data = event.data as { type?: string } | null;
      if (!data) return;

      // 활성 holder 탭: 신규 탭의 hello 에 present 로 응답 (이미 차단된 탭은 응답 안 함).
      if (data.type === "hello" && !isBlocked) {
        channel.postMessage({ type: "present" });
        return;
      }
      // 신규 탭: present 수신 = 이미 다른 탭이 활성 → 자신을 차단.
      if (data.type === "present") {
        isBlocked = true;
        setBlocked(true);
      }
    };

    channel.addEventListener("message", onMessage);
    channel.postMessage({ type: "hello" });

    return () => {
      channel.removeEventListener("message", onMessage);
      channel.close();
    };
  }, []);

  return blocked;
}
