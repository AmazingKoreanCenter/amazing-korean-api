import { describe, expect, it, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MemoryRouter } from "react-router-dom";
import { ResetPasswordPage } from "./reset_password_page";

const mockNavigate = vi.fn();
const mockMutate = vi.fn();
let mockIsPending = false;
const toastError = vi.fn();
const toastSuccess = vi.fn();

vi.mock("react-router-dom", async () => {
  const actual = await vi.importActual<typeof import("react-router-dom")>(
    "react-router-dom",
  );
  return {
    ...actual,
    useNavigate: () => mockNavigate,
    useLocation: () => ({ state: mockLocationState }),
  };
});

let mockLocationState: { token?: string } | null = { token: "tok_test" };

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

vi.mock("@/i18n", () => ({
  default: { t: (key: string) => key },
}));

vi.mock("sonner", () => ({
  toast: {
    error: (...args: unknown[]) => toastError(...args),
    success: (...args: unknown[]) => toastSuccess(...args),
  },
}));

vi.mock("../hook/use_reset_password", () => ({
  useResetPassword: () => ({
    mutate: (data: unknown, opts?: { onSuccess?: () => void }) => {
      mockMutate(data);
      opts?.onSuccess?.();
    },
    isPending: mockIsPending,
  }),
}));

const renderPage = () =>
  render(
    <MemoryRouter>
      <ResetPasswordPage />
    </MemoryRouter>,
  );

describe("ResetPasswordPage", () => {
  beforeEach(() => {
    mockNavigate.mockReset();
    mockMutate.mockReset();
    toastError.mockReset();
    toastSuccess.mockReset();
    mockIsPending = false;
    mockLocationState = { token: "tok_test" };
  });

  it("renders new-password + confirm-password form fields", () => {
    renderPage();
    expect(screen.getByLabelText("auth.newPasswordLabel")).toBeInTheDocument();
    expect(
      screen.getByLabelText("auth.resetConfirmPasswordLabel"),
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "auth.changePasswordButton" }),
    ).toBeInTheDocument();
  });

  it("when token is missing → toast error + navigate /login", async () => {
    mockLocationState = null;
    renderPage();
    await waitFor(() => {
      expect(toastError).toHaveBeenCalledWith("auth.toastInvalidAccess");
      expect(mockNavigate).toHaveBeenCalledWith("/login", { replace: true });
    });
  });

  it("submits new_password + reset_token when form values match and pass validation", async () => {
    const user = userEvent.setup();
    renderPage();

    await user.type(
      screen.getByLabelText("auth.newPasswordLabel"),
      "Password123",
    );
    await user.type(
      screen.getByLabelText("auth.resetConfirmPasswordLabel"),
      "Password123",
    );
    await user.click(
      screen.getByRole("button", { name: "auth.changePasswordButton" }),
    );

    await waitFor(() => {
      expect(mockMutate).toHaveBeenCalledWith({
        reset_token: "tok_test",
        new_password: "Password123",
      });
    });
    // 본 page-level onSuccess → navigate("/login")
    expect(mockNavigate).toHaveBeenCalledWith("/login");
  });

  it("disables submit button while mutation isPending", () => {
    mockIsPending = true;
    renderPage();
    expect(
      screen.getByRole("button", { name: "auth.changingPassword" }),
    ).toBeDisabled();
  });
});
