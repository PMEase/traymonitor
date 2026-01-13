import { useEffect } from "react";
import ErrorBoundary from "./components/error-boundary";
import MainWindow from "./components/layout/main-window";
import { initializeCommandSystem } from "./lib/commands";
import { logger } from "./lib/logger";
import { cleanupOldFiles } from "./lib/recovery";

function LegacyApp() {
  // Initialize command system and cleanup on app startup
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

  return (
    <ErrorBoundary>
      <MainWindow />
    </ErrorBoundary>
  );
}

export default LegacyApp;
