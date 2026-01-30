import { useEffect } from "react";
import { Route, Routes } from "react-router-dom";
import ErrorBoundary from "./components/error-boundary";
import Layout from "./components/layout/layout";
import { initializeCommandSystem } from "./lib/commands";
import { logger } from "./lib/logger";
import { runUpdateFlow } from "./lib/updater";
import { AboutView } from "./views/about";
import { AlertsView } from "./views/alerts";
import { BuildsView } from "./views/builds";
import { MainView } from "./views/main";
import { SettingsView } from "./views/settings";

function App() {
  useEffect(() => {
    logger.info("🚀 Frontend application starting up");
    initializeCommandSystem();
    logger.debug("Command system initialized");

    // Example of logging with context
    logger.info("App environment", {
      isDev: import.meta.env.DEV,
      mode: import.meta.env.MODE,
    });
  }, []);

  // Auto-check for updates 5 seconds after launch (skip in dev)
  useEffect(() => {
    if (import.meta.env.DEV) {
      return;
    }
    const timer = setTimeout(() => {
      runUpdateFlow({ silent: true }).catch(() => {
        // Fail silently on auto-check (e.g. network issues)
      });
    }, 5000);
    return () => clearTimeout(timer);
  }, []);

  return (
    <ErrorBoundary>
      <Layout>
        <Routes>
          <Route Component={MainView} path="/" />
          <Route Component={SettingsView} path="/settings" />
          <Route Component={AboutView} path="/about" />
          <Route Component={BuildsView} path="/builds" />
          <Route Component={AlertsView} path="/alerts" />
        </Routes>
      </Layout>
    </ErrorBoundary>
  );
}

export default App;
