import { describe, expect, it, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MemoryRouter } from "react-router-dom";
import { SignupPage } from "./signup_page";

const mockNavigate = vi.fn();
const mockSignupMutate = vi.fn();
const mockGoogleMutate = vi.fn();
let mockSignupPending = false;
let mockGooglePending = false;

vi.mock("react-router-dom", async () => {
  const actual = await vi.importActual<typeof import("react-router-dom")>(
    "react-router-dom",
  );
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  };
});

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
    i18n: { language: "ko" },
  }),
}));

vi.mock("@/i18n", () => ({
  default: { t: (key: string) => key },
}));

vi.mock("sonner", () => ({
  toast: { error: vi.fn(), success: vi.fn(), warning: vi.fn() },
}));

vi.mock("../hook/use_signup", () => ({
  useSignup: () => ({
    mutate: (
      data: unknown,
      opts?: {
        onSuccess?: (res: { requires_verification: boolean }) => void;
      },
    ) => {
      mockSignupMutate(data);
      opts?.onSuccess?.(mockSignupResponse);
    },
    isPending: mockSignupPending,
  }),
}));

vi.mock("../hook/use_google_login", () => ({
  useGoogleLogin: () => ({
    mutate: () => mockGoogleMutate(),
    isPending: mockGooglePending,
  }),
}));

let mockSignupResponse: { requires_verification: boolean } = {
  requires_verification: true,
};

const renderPage = () =>
  render(
    <MemoryRouter>
      <SignupPage />
    </MemoryRouter>,
  );

describe("SignupPage", () => {
  beforeEach(() => {
    mockNavigate.mockReset();
    mockSignupMutate.mockReset();
    mockGoogleMutate.mockReset();
    mockSignupPending = false;
    mockGooglePending = false;
    mockSignupResponse = { requires_verification: true };
  });

  it("renders title + Google/Apple buttons + email collapsible trigger", () => {
    renderPage();
    expect(screen.getByText("auth.signupTitle")).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /auth.signupWithGoogle/ }),
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /auth.signupWithEmailButton/ }),
    ).toBeInTheDocument();
  });

  it("triggers google mutation when google button is clicked", async () => {
    const user = userEvent.setup();
    renderPage();
    await user.click(
      screen.getByRole("button", { name: "auth.signupWithGoogle" }),
    );
    expect(mockGoogleMutate).toHaveBeenCalledTimes(1);
  });

  it("disables google button while googleLoginMutation.isPending", () => {
    mockGooglePending = true;
    renderPage();
    const btn = screen.getByRole("button", {
      name: "auth.signupWithGoogleLoading",
    });
    expect(btn).toBeDisabled();
  });

  it("expands email form when collapsible trigger is clicked", async () => {
    const user = userEvent.setup();
    renderPage();
    // 초기 = email input 숨김
    expect(screen.queryByLabelText("auth.emailLabel")).not.toBeInTheDocument();
    await user.click(
      screen.getByRole("button", { name: /auth.signupWithEmailButton/ }),
    );
    await waitFor(() => {
      expect(screen.getByLabelText("auth.emailLabel")).toBeInTheDocument();
    });
  });

  it("navigates to /verify-email when signup requires verification", async () => {
    mockSignupResponse = { requires_verification: true };
    const user = userEvent.setup();
    renderPage();

    // Expand email form
    await user.click(
      screen.getByRole("button", { name: /auth.signupWithEmailButton/ }),
    );

    // 필드 채우기 (zod superRefine = password match + terms 필수)
    await user.type(
      screen.getByLabelText("auth.emailLabel"),
      "test@example.com",
    );
    await user.type(
      screen.getByLabelText("auth.passwordLabel"),
      "Password123",
    );
    await user.type(
      screen.getByLabelText("auth.confirmPasswordLabel"),
      "Password123",
    );
    await user.type(screen.getByLabelText("auth.nameLabel"), "테스트");
    await user.type(screen.getByLabelText("auth.nicknameLabel"), "tester");
    await user.type(screen.getByLabelText("auth.birthdayLabel"), "1990-01-15");

    // 약관 체크
    await user.click(screen.getByText("auth.termsServiceAgree"));
    await user.click(screen.getByText("auth.termsPersonalAgree"));

    // submit
    await user.click(
      screen.getByRole("button", { name: "auth.signupButton" }),
    );

    await waitFor(() => {
      expect(mockSignupMutate).toHaveBeenCalled();
    });
    // confirm_password 가 제거된 apiData 전달 검증
    const callArg = mockSignupMutate.mock.calls[0][0] as Record<string, unknown>;
    expect(callArg.email).toBe("test@example.com");
    expect(callArg.password).toBe("Password123");
    expect(callArg).not.toHaveProperty("confirm_password");

    // onSuccess(requires_verification=true) → /verify-email + state.email
    expect(mockNavigate).toHaveBeenCalledWith("/verify-email", {
      state: { email: "test@example.com" },
      replace: true,
    });
  });

  it("navigates to /login when signup does not require verification", async () => {
    mockSignupResponse = { requires_verification: false };
    const user = userEvent.setup();
    renderPage();

    await user.click(
      screen.getByRole("button", { name: /auth.signupWithEmailButton/ }),
    );
    await user.type(
      screen.getByLabelText("auth.emailLabel"),
      "noverify@example.com",
    );
    await user.type(
      screen.getByLabelText("auth.passwordLabel"),
      "Password123",
    );
    await user.type(
      screen.getByLabelText("auth.confirmPasswordLabel"),
      "Password123",
    );
    await user.type(screen.getByLabelText("auth.nameLabel"), "테스트");
    await user.type(screen.getByLabelText("auth.nicknameLabel"), "tester");
    await user.type(screen.getByLabelText("auth.birthdayLabel"), "1990-01-15");
    await user.click(screen.getByText("auth.termsServiceAgree"));
    await user.click(screen.getByText("auth.termsPersonalAgree"));
    await user.click(
      screen.getByRole("button", { name: "auth.signupButton" }),
    );

    await waitFor(() => {
      expect(mockNavigate).toHaveBeenCalledWith("/login");
    });
  });

  it("disables submit button while signupMutation.isPending", async () => {
    mockSignupPending = true;
    const user = userEvent.setup();
    renderPage();
    await user.click(
      screen.getByRole("button", { name: /auth.signupWithEmailButton/ }),
    );
    await waitFor(() => {
      const btn = screen.getByRole("button", { name: "auth.signingUp" });
      expect(btn).toBeDisabled();
    });
  });
});
