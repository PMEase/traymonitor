import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";
import { logger } from "@/lib/logger";

export interface UpdateFlowOptions {
  /** When true, do not show toasts for "no update" or network errors (e.g. auto-check on launch) */
  silent?: boolean;
  /** Called when an update is available (before confirm). Use to show toast in manual check. */
  onUpdateAvailable?: (version: string) => void;
}

/**
 * Check for updates, and if available: prompt user, download & install, then prompt restart and relaunch.
 * Use from App (auto-check on launch with silent: true) and from menu/command (manual check with silent: false).
 */
export async function runUpdateFlow(
  options: UpdateFlowOptions = {}
): Promise<void> {
  const { silent = false, onUpdateAvailable } = options;

  try {
    logger.info("Checking for updates...");
    const update = await check();

    if (!update) {
      if (!silent) {
        onUpdateAvailable?.("none");
      }
      logger.info("No updates available");
      return;
    }

    logger.info("Update available", {
      version: update.version,
      currentVersion: update.currentVersion,
    });
    onUpdateAvailable?.(update.version);

    // User confirmation for download (native dialog for updater flow)
    // biome-ignore lint: native confirm dialog for updater flow
    const shouldUpdate = window.confirm(
      `A new version ${update.version} is available.\n\n
      Current version: ${update.currentVersion}\n\n
      Would you like to download and install this update?`
    );

    if (!shouldUpdate) {
      logger.debug("User declined update");
      return;
    }

    logger.info("User accepted update, downloading...");

    await update.downloadAndInstall((event) => {
      switch (event.event) {
        case "Started":
          logger.debug("Update download started", {
            contentLength: event.data?.contentLength,
          });
          break;
        case "Progress":
          logger.debug("Update download progress", {
            chunkLength: event.data?.chunkLength,
          });
          break;
        case "Finished":
          logger.info("Update download finished");
          break;
        default:
          logger.debug("Update event", { event });
      }
    });

    logger.info("Update installed successfully");

    // User confirmation for restart (native dialog for updater flow)
    // biome-ignore lint: native confirm dialog for updater flow
    const shouldRestart = window.confirm(
      "Update installed successfully.\n\nThe application needs to restart to apply the update. Would you like to restart now?"
    );

    if (shouldRestart) {
      logger.info("Relaunching application...");
      await relaunch();
    }
  } catch (error) {
    logger.error("Update check or install failed", { error: String(error) });
    if (!silent) {
      onUpdateAvailable?.("error");
    }
    throw error;
  }
}
