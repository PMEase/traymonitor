import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import type { GetBuildsResponse } from "@/lib/bindings";
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
    queryFn: async (): Promise<GetBuildsResponse> => {
      try {
        logger.debug("Loading builds from backend");
        const response = await invoke<GetBuildsResponse>("get_builds");
        logger.info("Builds loaded successfully");
        return response;
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
