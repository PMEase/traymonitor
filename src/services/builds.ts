import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import type { Build } from "@/lib/bindings";
import { logger } from "@/lib/logger";

// Query keys for preferences
export const buildsQueryKeys = {
  all: ["builds"] as const,
  builds: () => [...buildsQueryKeys.all] as const,
};

// TanStack Query hooks following the architectural patterns
export function useBuilds() {
  return useQuery({
    queryKey: buildsQueryKeys.builds(),
    queryFn: async (): Promise<Build[]> => {
      try {
        logger.debug("Loading builds from backend");
        const builds = await invoke<Build[]>("get_builds");
        logger.info("Builds loaded successfully", { builds });
        return builds;
      } catch (error) {
        logger.error("Failed to load builds", { error });
        throw new Error(
          `${error instanceof Error ? error.message : String(error)}`
        );
      }
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
    gcTime: 1000 * 60 * 10, // 10 minutes
  });
}
