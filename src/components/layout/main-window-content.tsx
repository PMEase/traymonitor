import { useMemo } from "react";
import { Spinner } from "@/components/ui/spinner";
import { logger } from "@/lib/logger";
import { cn } from "@/lib/utils";
import { usePreferences, useSavePreferences } from "@/services/preferences";
import type { AppPreferences } from "@/types/preferences";
import { Button } from "../ui/button";

// import { Button } from "@/components/ui/button";
// import { ExternalLink } from "lucide-react";
// import { commands } from "@/lib/bindings";
// import { logger } from "@/lib/logger";

interface MainWindowContentProps {
  children?: React.ReactNode;
  className?: string;
}

const REGEX_TRAILING_SLASH = /\/+$/;

export function MainWindowContent({
  children,
  className,
}: MainWindowContentProps) {
  const { data: preferences, isLoading } = usePreferences();
  const savePreferences = useSavePreferences();

  // Build dashboard URL from server_url preference
  const dashboardUrl = useMemo(() => {
    if (!preferences?.server_url) {
      return null;
    }
    const serverUrl = preferences.server_url.trim();
    if (!serverUrl) {
      return null;
    }
    // Remove trailing slash if present, then append /lite
    const baseUrl = serverUrl.replace(REGEX_TRAILING_SLASH, "");
    return `${baseUrl}`;
  }, [preferences?.server_url]);

  const handleSaveSettings = async (newSettings: AppPreferences) => {
    try {
      await savePreferences.mutateAsync(newSettings);
      logger.info("Preferences saved successfully");
    } catch (error) {
      logger.error("Failed to save preferences", { error });
    }
  };

  const content = dashboardUrl ? (
    <>
      <h1>Hello, {preferences?.server_url}</h1>
      <Button
        onClick={() =>
          handleSaveSettings({
            ...preferences,
            server_url: "http://build.pmease.com:8810",
          })
        }
      >
        Update Server URL
      </Button>
    </>
  ) : (
    <div className="flex flex-col items-center gap-2 text-center">
      <p className="font-medium text-sm">No dashboard URL configured</p>
      <p className="text-muted-foreground text-xs">
        Please configure the server URL in preferences
      </p>
    </div>
  );

  return (
    <div className={cn("flex h-full flex-col bg-background", className)}>
      {children || (
        <div className="flex flex-1 items-center justify-center">
          {isLoading ? (
            <div className="flex flex-col items-center gap-2">
              <Spinner />
              <p className="text-muted-foreground text-sm">
                Loading preferences...
              </p>
            </div>
          ) : (
            content
          )}
        </div>
      )}
    </div>
  );
}

export default MainWindowContent;
