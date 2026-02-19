import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "next-themes";
import { BrowserRouter } from "react-router-dom";

import { AppRoutes } from "@/app/routes";
import { Toaster } from "@/components/ui/sonner";
import { useLanguageSync } from "@/hooks/use_language_sync";

const queryClient = new QueryClient();

function LanguageSync() {
  useLanguageSync();
  return null;
}

function App() {
  return (
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem disableTransitionOnChange>
      <QueryClientProvider client={queryClient}>
        <LanguageSync />
        <BrowserRouter>
          <AppRoutes />
        </BrowserRouter>
        <Toaster />
      </QueryClientProvider>
    </ThemeProvider>
  );
}

export default App;
