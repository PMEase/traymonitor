import { useEffect } from "react";
import { Route, Routes } from "react-router-dom";
import ErrorBoundary from "./components/error-boundary";
import Layout from "./components/layout/layout";
import { initializeCommandSystem } from "./lib/commands";
import { logger } from "./lib/logger";
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
    // cleanupOldFiles().catch((error) => {
    //   logger.warn("Failed to cleanup old recovery files", { error });
    // });

    // Example of logging with context
    logger.info("App environment", {
      isDev: import.meta.env.DEV,
      mode: import.meta.env.MODE,
    });
  }, []);

  return (
    <ErrorBoundary>
      <Layout>
        <Routes>
          <Route Component={MainView} path="/" />
          <Route Component={SettingsView} path="/settings" />
          <Route Component={BuildsView} path="/builds" />
          <Route Component={AlertsView} path="/alerts" />
        </Routes>
      </Layout>
    </ErrorBoundary>
  );
}

export default App;
