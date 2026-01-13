import { QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./app";
import { queryClient } from "./lib/query-client";
import "./app.css";
import { HashRouter } from "react-router-dom";
import { ThemeProvider } from "./components/theme-provider";
import { TooltipProvider } from "./components/ui/tooltip";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <HashRouter>
        <TooltipProvider>
          <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
            <App />
            <ReactQueryDevtools initialIsOpen={false} />
          </ThemeProvider>
        </TooltipProvider>
      </HashRouter>
    </QueryClientProvider>
  </React.StrictMode>
);
