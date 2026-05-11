import { describe, expect, it, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MemoryRouter } from "react-router-dom";
import { PricingPage } from "./pricing_page";
import { useAuthStore } from "@/hooks/use_auth_store";

const mockNavigate = vi.fn();
const mockSetSearchParams = vi.fn();
const mockOpenCheckout = vi.fn();
const mockCancelMutate = vi.fn();
let mockSearchParams = new URLSearchParams();
const toastInfo = vi.fn();
const toastSuccess = vi.fn();

vi.mock("react-router-dom", async () => {
  const actual = await vi.importActual<typeof import("react-router-dom")>(
    "react-router-dom",
  );
  return {
    ...actual,
    useNavigate: () => mockNavigate,
    useSearchParams: () => [mockSearchParams, mockSetSearchParams],
  };
});

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
    i18n: { language: "ko" },
  }),
}));

vi.mock("sonner", () => ({
  toast: {
    info: (...args: unknown[]) => toastInfo(...args),
    success: (...args: unknown[]) => toastSuccess(...args),
    error: vi.fn(),
  },
}));

let mockPlansData: unknown = {
  client_token: "ctk",
  sandbox: true,
  plans: [
    {
      interval: "month_1",
      months: 1,
      price_cents: 9900,
      price_display: "$9.90",
      price_id: "pri_1",
      trial_days: 1,
      label: "1 month",
    },
    {
      interval: "month_12",
      months: 12,
      price_cents: 99000,
      price_display: "$990",
      price_id: "pri_12",
      trial_days: 1,
      label: "12 month",
    },
  ],
};
let mockPlansLoading = false;
let mockSubData: unknown = { subscription: null };
let mockSubLoading = false;
let mockUserMe: unknown = { email: "u@e.com" };

vi.mock("../hook/use_payment_plans", () => ({
  usePaymentPlans: () => ({
    data: mockPlansData,
    isLoading: mockPlansLoading,
  }),
}));

vi.mock("../hook/use_subscription", () => ({
  useSubscription: () => ({
    data: mockSubData,
    isLoading: mockSubLoading,
  }),
}));

vi.mock("@/category/user/hook/use_user_me", () => ({
  useUserMe: () => ({ data: mockUserMe }),
}));

vi.mock("../hook/use_paddle", () => ({
  usePaddle: () => ({ openCheckout: mockOpenCheckout }),
}));

vi.mock("../hook/use_manage_subscription", () => ({
  useCancelSubscription: () => ({
    mutate: (...args: unknown[]) => mockCancelMutate(...args),
    isPending: false,
  }),
}));

const renderPage = () =>
  render(
    <MemoryRouter>
      <PricingPage />
    </MemoryRouter>,
  );

describe("PricingPage", () => {
  beforeEach(() => {
    mockNavigate.mockReset();
    mockSetSearchParams.mockReset();
    mockOpenCheckout.mockReset();
    mockCancelMutate.mockReset();
    toastInfo.mockReset();
    toastSuccess.mockReset();
    mockSearchParams = new URLSearchParams();
    mockPlansData = {
      client_token: "ctk",
      sandbox: true,
      plans: [
        {
          interval: "month_1",
          months: 1,
          price_cents: 9900,
          price_display: "$9.90",
          price_id: "pri_1",
          trial_days: 1,
          label: "1 month",
        },
        {
          interval: "month_12",
          months: 12,
          price_cents: 99000,
          price_display: "$990",
          price_id: "pri_12",
          trial_days: 1,
          label: "12 month",
        },
      ],
    };
    mockPlansLoading = false;
    mockSubData = { subscription: null };
    mockSubLoading = false;
    mockUserMe = { email: "u@e.com" };

    useAuthStore.setState({
      user: null,
      accessToken: null,
      isLoggedIn: false,
    });
  });

  it("renders loading skeleton while plansLoading", () => {
    mockPlansLoading = true;
    const { container } = renderPage();
    // skeleton = animate-pulse 클래스가 있는 div (위 카드 + 4개 plan 카드)
    expect(container.querySelectorAll(".animate-pulse").length).toBeGreaterThanOrEqual(4);
  });

  it("renders plan cards when data is loaded", () => {
    renderPage();
    expect(screen.getByText("payment.title")).toBeInTheDocument();
    expect(screen.getByText("$9.90")).toBeInTheDocument();
    expect(screen.getByText("$990")).toBeInTheDocument();
  });

  it("redirects to /login when not logged in and plan clicked", async () => {
    useAuthStore.setState({ isLoggedIn: false });
    const user = userEvent.setup();
    renderPage();
    // 첫 plan 의 select 버튼
    const buttons = screen.getAllByRole("button", { name: /payment\.selectPlan|payment\.startFreeTrial/ });
    if (buttons.length > 0) {
      await user.click(buttons[0]);
      expect(mockNavigate).toHaveBeenCalledWith("/login?redirect=/pricing");
    }
  });

  it("calls openCheckout when logged in and no active sub", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "tok",
      isLoggedIn: true,
    });
    mockSubData = { subscription: null };

    const user = userEvent.setup();
    renderPage();
    const buttons = screen.getAllByRole("button", { name: /payment\.selectPlan|payment\.startFreeTrial/ });
    if (buttons.length > 0) {
      await user.click(buttons[0]);
      await waitFor(() => {
        expect(mockOpenCheckout).toHaveBeenCalled();
      });
    }
  });

  it("shows alreadySubscribed toast when active sub exists", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "tok",
      isLoggedIn: true,
    });
    mockSubData = {
      subscription: {
        subscription_id: 1,
        status: "active",
        billing_interval: "month_1",
        current_price_cents: 9900,
        trial_ends_at: null,
        current_period_start: "2026-01-01",
        current_period_end: "2026-02-01",
        canceled_at: null,
        paused_at: null,
        created_at: "2026-01-01",
      },
    };

    const user = userEvent.setup();
    renderPage();
    const buttons = screen.getAllByRole("button", { name: /payment\.selectPlan|payment\.startFreeTrial/ });
    if (buttons.length > 0) {
      await user.click(buttons[0]);
      await waitFor(() => {
        expect(toastInfo).toHaveBeenCalledWith("payment.alreadySubscribed");
      });
    }
  });

  it("shows checkout success toast and clears params when ?success=true", () => {
    mockSearchParams = new URLSearchParams("success=true");
    renderPage();
    expect(toastSuccess).toHaveBeenCalledWith("payment.checkoutSuccess");
    expect(mockSetSearchParams).toHaveBeenCalledWith({}, { replace: true });
  });

  it("opens cancel dialog → cancelAtPeriodEnd → mutate { immediately: false }", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "tok",
      isLoggedIn: true,
    });
    mockSubData = {
      subscription: {
        subscription_id: 1,
        status: "active",
        billing_interval: "month_1",
        current_price_cents: 9900,
        trial_ends_at: null,
        current_period_start: "2026-01-01",
        current_period_end: "2026-02-01",
        canceled_at: null,
        paused_at: null,
        created_at: "2026-01-01",
      },
    };

    const user = userEvent.setup();
    renderPage();
    // 취소 버튼 = banner 안의 outline 버튼
    await user.click(
      screen.getByRole("button", { name: "payment.cancelSubscription" }),
    );
    // Dialog 안의 "기간 종료 시 취소" 버튼
    await user.click(
      screen.getByRole("button", { name: "payment.cancelAtPeriodEnd" }),
    );
    expect(mockCancelMutate).toHaveBeenCalledWith(
      { immediately: false },
      expect.any(Object),
    );
  });

  it("opens cancel dialog → cancelImmediate → mutate { immediately: true }", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "tok",
      isLoggedIn: true,
    });
    mockSubData = {
      subscription: {
        subscription_id: 1,
        status: "active",
        billing_interval: "month_1",
        current_price_cents: 9900,
        trial_ends_at: null,
        current_period_start: "2026-01-01",
        current_period_end: "2026-02-01",
        canceled_at: null,
        paused_at: null,
        created_at: "2026-01-01",
      },
    };

    const user = userEvent.setup();
    renderPage();
    await user.click(
      screen.getByRole("button", { name: "payment.cancelSubscription" }),
    );
    await user.click(
      screen.getByRole("button", { name: "payment.cancelImmediate" }),
    );
    expect(mockCancelMutate).toHaveBeenCalledWith(
      { immediately: true },
      expect.any(Object),
    );
  });

  it("clears promo code via clear button", async () => {
    useAuthStore.setState({ isLoggedIn: false });
    const user = userEvent.setup();
    renderPage();
    const input = screen.getByPlaceholderText("payment.promoCodePlaceholder");
    await user.type(input, "PROMO10");
    expect((input as HTMLInputElement).value).toBe("PROMO10");
    await user.click(
      screen.getByRole("button", { name: "payment.promoCodeClear" }),
    );
    expect((input as HTMLInputElement).value).toBe("");
  });
});
