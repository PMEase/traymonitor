import { useEffect } from "react";
import { Route, Routes } from "react-router-dom";
import { Toaster } from "sonner";
import ErrorBoundary from "./components/error-boundary";
import { useMainWindowEventListeners } from "./hooks/use-main-window-event-listeners";
import { useTheme } from "./hooks/use-theme";
import { initializeCommandSystem } from "./lib/commands";
import { logger } from "./lib/logger";
import { cleanupOldFiles } from "./lib/recovery";
import { AlertsView } from "./views/alerts";
import { BuildsView } from "./views/builds";
import { MainView } from "./views/main";
import { SettingsView } from "./views/settings";

function App() {
  useEffect(() => {
    logger.info("🚀 Frontend application starting up");
    initializeCommandSystem();
    logger.debug("Command system initialized");

    // Clean up old recovery files on startup
    cleanupOldFiles().catch((error) => {
      logger.warn("Failed to cleanup old recovery files", { error });
    });

    // Example of logging with context
    logger.info("App environment", {
      isDev: import.meta.env.DEV,
      mode: import.meta.env.MODE,
    });
  }, []);

  const { theme } = useTheme();

  // Set up global event listeners (keyboard shortcuts, etc.)
  useMainWindowEventListeners();

  return (
    <ErrorBoundary>
      <Toaster
        className="toaster group"
        position="bottom-right"
        theme={theme === "dark" || theme === "light" ? theme : "system"}
        toastOptions={{
          classNames: {
            toast:
              "group toast group-[.toaster]:bg-background group-[.toaster]:text-foreground group-[.toaster]:border-border group-[.toaster]:shadow-lg",
            description: "group-[.toast]:text-muted-foreground",
            actionButton:
              "group-[.toast]:bg-primary group-[.toast]:text-primary-foreground",
            cancelButton:
              "group-[.toast]:bg-muted group-[.toast]:text-muted-foreground",
          },
        }}
      />
      <Routes>
        <Route Component={MainView} path="/" />
        <Route Component={SettingsView} path="/settings" />
        <Route Component={BuildsView} path="/builds" />
        <Route Component={AlertsView} path="/alerts" />
      </Routes>
    </ErrorBoundary>
  );
}

export default App;
