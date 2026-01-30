import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { logger } from "@/lib/logger";
import { runUpdateFlow } from "@/lib/updater";
import { useCommandContext } from "./use-command-context";

/**
 * Main window event listeners - handles global keyboard shortcuts and other app-level events
 *
 * This hook provides a centralized place for all global event listeners, keeping
 * the MainWindow component clean while maintaining good separation of concerns.
 */
export function useMainWindowEventListeners() {
  const commandContext = useCommandContext();
  const navigate = useNavigate();

  useEffect(() => {
    // Set up native menu event listeners
    const setupMenuListeners = async () => {
      logger.debug("Setting up menu event listeners");
      const unlisteners = await Promise.all([
        listen("menu-view-builds", () => {
          logger.info("View builds menu event received");
          navigate("/builds");
        }),

        listen("menu-view-alerts", () => {
          logger.info("View alerts menu event received");
          navigate("/alerts");
        }),

        listen("menu-view-settings", () => {
          logger.debug("Preferences menu event received");
          navigate("/settings");
        }),

        listen("menu-check-updates", async () => {
          logger.debug("Check for updates menu event received");
          try {
            await runUpdateFlow({
              silent: false,
              onUpdateAvailable: (version) => {
                if (version === "none") {
                  commandContext.showToast(
                    "You are running the latest version",
                    "success"
                  );
                } else if (version === "error") {
                  commandContext.showToast(
                    "Failed to check for updates",
                    "error"
                  );
                } else {
                  commandContext.showToast(
                    `Update available: ${version}`,
                    "info"
                  );
                }
              },
            });
          } catch {
            // Error toast already shown via onUpdateAvailable("error") when applicable
            commandContext.showToast("Failed to check for updates", "error");
          }
        }),

        listen("menu-preferences", () => {
          logger.debug("Preferences menu event received");
          commandContext.openPreferences();
        }),
      ]);

      logger.debug(
        `Menu listeners set up successfully: ${unlisteners.length} listeners`
      );
      return unlisteners;
    };

    let menuUnlisteners: (() => void)[] = [];
    setupMenuListeners()
      .then((unlisteners) => {
        menuUnlisteners = unlisteners;
        logger.debug("Menu listeners initialized successfully");
      })
      .catch((error) => {
        logger.error("Failed to setup menu listeners:", error);
      });

    return () => {
      for (const unlisten of menuUnlisteners) {
        if (unlisten && typeof unlisten === "function") {
          unlisten();
        }
      }
    };
  }, [commandContext, navigate]);

  // Future: Other global event listeners can be added here
  // useWindowFocusListeners()
}
