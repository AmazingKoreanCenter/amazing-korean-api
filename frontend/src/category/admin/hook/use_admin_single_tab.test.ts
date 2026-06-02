import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { renderHook } from "@testing-library/react";

import { useAdminSingleTab } from "./use_admin_single_tab";

/**
 * BroadcastChannel 결정적 mock — 같은 name 의 다른 인스턴스에 **동기**로 메시지를 전달한다
 * (송신자 본인 제외, 실제 BroadcastChannel 의미 보존). jsdom 의 BroadcastChannel 부재/비결정성
 * 을 피하고 hello/present 교환을 결정적으로 검증한다.
 */
type Listener = (e: MessageEvent) => void;

class MockBroadcastChannel {
  static channels: MockBroadcastChannel[] = [];
  name: string;
  onmessage: Listener | null = null;
  private listeners: Listener[] = [];
  private closed = false;

  constructor(name: string) {
    this.name = name;
    MockBroadcastChannel.channels.push(this);
  }

  postMessage(data: unknown) {
    for (const ch of MockBroadcastChannel.channels) {
      if (ch === this || ch.closed || ch.name !== this.name) continue;
      const ev = { data } as MessageEvent;
      ch.onmessage?.(ev);
      ch.listeners.forEach((l) => l(ev));
    }
  }

  addEventListener(_type: string, cb: Listener) {
    this.listeners.push(cb);
  }

  removeEventListener(_type: string, cb: Listener) {
    this.listeners = this.listeners.filter((l) => l !== cb);
  }

  close() {
    this.closed = true;
    MockBroadcastChannel.channels = MockBroadcastChannel.channels.filter(
      (c) => c !== this,
    );
  }
}

describe("useAdminSingleTab", () => {
  beforeEach(() => {
    MockBroadcastChannel.channels = [];
    vi.stubGlobal("BroadcastChannel", MockBroadcastChannel);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("첫 탭은 active(차단 안 됨)", () => {
    const tab1 = renderHook(() => useAdminSingleTab());
    expect(tab1.result.current).toBe(false);
  });

  it("두 번째 탭만 차단되고 첫 탭은 active 유지된다", () => {
    const tab1 = renderHook(() => useAdminSingleTab());
    expect(tab1.result.current).toBe(false);

    // 두 번째 탭 마운트 → hello → 첫 탭이 present 응답 → 두 번째 탭 차단.
    const tab2 = renderHook(() => useAdminSingleTab());
    expect(tab2.result.current).toBe(true); // 새 탭 차단
    expect(tab1.result.current).toBe(false); // 기존 탭은 영향 없음
  });

  it("BroadcastChannel 미지원 환경에서는 차단하지 않는다 (graceful)", () => {
    vi.stubGlobal("BroadcastChannel", undefined);
    const tab = renderHook(() => useAdminSingleTab());
    expect(tab.result.current).toBe(false);
  });
});
