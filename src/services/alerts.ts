import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { logger } from "@/lib/logger";

// Query keys for preferences
export const alertsQueryKeys = {
  all: ["alerts"] as const,
  alerts: () => [...alertsQueryKeys.all] as const,
};

// TanStack Query hooks following the architectural patterns
export function useAlerts() {
  return useQuery({
    queryKey: alertsQueryKeys.alerts(),
    queryFn: async (): Promise<GetAlertsResponse> => {
      try {
        logger.debug("Loading alerts from backend");
        const response = await invoke<GetAlertsResponse>("get_alerts");
        logger.info("Alerts loaded successfully");
        return response;
      } catch (error) {
        logger.error("Failed to load alerts", { error });
        throw new Error(
          `${error instanceof Error ? error.message : String(error)}`
        );
      }
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
    gcTime: 1000 * 60 * 10, // 10 minutes
  });
}
