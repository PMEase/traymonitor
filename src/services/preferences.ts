import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import { logger } from "@/lib/logger";
import type { AppPreferences } from "@/types/preferences";

// Query keys for preferences
export const preferencesQueryKeys = {
  all: ["preferences"] as const,
  preferences: () => [...preferencesQueryKeys.all] as const,
};

// TanStack Query hooks following the architectural patterns
export function usePreferences() {
  return useQuery({
    queryKey: preferencesQueryKeys.preferences(),
    queryFn: async (): Promise<AppPreferences> => {
      try {
        logger.debug("Loading preferences from backend");
        const preferences = await invoke<AppPreferences>("load_settings");
        logger.info("Preferences loaded successfully", { preferences });
        return preferences;
      } catch (error) {
        // Return defaults if preferences file doesn't exist yet
        logger.warn("Failed to load preferences, using defaults", { error });
        return {
          theme: "system",
          enable_notifications: true,
          notifications_total: 100,
          server_url: "http://quickbuild:8810",
          user: "user",
          token: "token",
          poll_interval_in_secs: 10,
        };
      }
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
    gcTime: 1000 * 60 * 10, // 10 minutes
  });
}

export function useSavePreferences() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (preferences: AppPreferences) => {
      try {
        logger.debug("Saving preferences to backend", { preferences });
        await invoke("save_settings", { settings: preferences });
        logger.info("Preferences saved successfully");
      } catch (error) {
        const message =
          error instanceof Error ? error.message : (error as string);
        logger.error("Failed to save preferences", { error, preferences });
        toast.error("Failed to save preferences", { description: message });
        throw error;
      }
    },
    onSuccess: (_, preferences) => {
      // Update the cache with the new preferences
      queryClient.setQueryData(preferencesQueryKeys.preferences(), preferences);
      logger.info("Preferences cache updated");
      // toast.success("Preferences saved");
    },
  });
}
