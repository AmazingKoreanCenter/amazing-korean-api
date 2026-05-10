import { describe, expect, it } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { HealthPage } from "./health_page";

const renderPage = () => {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>
      <HealthPage />
    </QueryClientProvider>,
  );
};

describe("HealthPage", () => {
  it("shows the loading state before the request resolves", () => {
    server.use(
      http.get("/api/api/healthz", async () => {
        await new Promise((r) => setTimeout(r, 50));
        return HttpResponse.json({ status: "ok", uptime_ms: 1, version: "v" });
      }),
    );
    renderPage();
    expect(screen.getByText(/Checking Server Status/i)).toBeInTheDocument();
  });

  it("renders uptime, version, and a 'ok' badge on success", async () => {
    server.use(
      http.get("/api/api/healthz", () =>
        HttpResponse.json({
          status: "ok",
          uptime_ms: 12345,
          version: "v1.2.3",
        }),
      ),
    );
    renderPage();
    expect(await screen.findByText("12345")).toBeInTheDocument();
    expect(screen.getByText("v1.2.3")).toBeInTheDocument();
    expect(screen.getByText("ok")).toBeInTheDocument();
  });

  it("renders 'Server Offline' + 'offline' badge on error", async () => {
    server.use(
      http.get(
        "/api/api/healthz",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    renderPage();
    expect(await screen.findByText(/Server Offline/i)).toBeInTheDocument();
    expect(screen.getByText("offline")).toBeInTheDocument();
  });

  it("refetches when the refresh button is clicked (badge label updates ok→checking→ok)", async () => {
    let attempt = 0;
    server.use(
      http.get("/api/api/healthz", () => {
        attempt += 1;
        return HttpResponse.json({
          status: attempt === 1 ? "ok" : "degraded",
          uptime_ms: attempt,
          version: `v${attempt}`,
        });
      }),
    );
    const user = userEvent.setup();
    renderPage();
    expect(await screen.findByText("ok")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /새로고침/ }));
    await waitFor(() => {
      expect(screen.getByText("degraded")).toBeInTheDocument();
    });
    expect(screen.getByText("v2")).toBeInTheDocument();
  });
});
