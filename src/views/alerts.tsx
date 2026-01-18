import { listen } from "@tauri-apps/api/event";
import { AlertCircleIcon, RefreshCcwIcon } from "lucide-react";
import { useEffect } from "react";
import { Loading } from "@/components/loading";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import type { Alert } from "@/lib/bindings";
import { logger } from "@/lib/logger";
import { useAlerts } from "@/services/alerts";

export const AlertsView = () => {
  const { data, isLoading, isError, error, refetch } = useAlerts();

  // Listen for new-builds-available event and refetch data
  useEffect(() => {
    let unlistenFn: (() => void) | null = null;
    let isMounted = true;

    const setupListener = async () => {
      try {
        const unlisten = await listen("refresh-page", () => {
          if (isMounted) {
            logger.debug("Received refresh-page event, refreshing page now...");
            refetch();
          }
        });
        if (isMounted) {
          unlistenFn = unlisten;
        } else {
          // Component unmounted before listener was set up, clean up immediately
          unlisten();
        }
      } catch (error) {
        logger.error("Failed to set up refresh-page listener", {
          error,
        });
      }
    };

    setupListener();

    return () => {
      isMounted = false;
      if (unlistenFn) {
        unlistenFn();
      }
    };
  }, [refetch]);

  let alertContent: React.ReactNode = null;
  if (isLoading) {
    alertContent = <Loading className="py-6" message="Loading alerts..." />;
  } else if (isError) {
    alertContent = (
      <div className="flex bg-red-100 px-6 py-4 dark:bg-red-900">
        <div className="flex-0">
          <AlertCircleIcon className="size-6 text-red-900 dark:text-red-100" />
        </div>
        <div className="flex flex-1 flex-col gap-3 overflow-hidden pl-2">
          <h3 className="font-semibold">Error loading alerts</h3>
          <div className="wrap-break-word text-sm">{error?.message}</div>
        </div>
      </div>
    );
  } else {
    const alerts: Alert[] = data?.alerts ?? [];
    let errorMessage: React.ReactNode | null = null;

    if (data?.error) {
      errorMessage = (
        <div className="flex bg-red-100 px-6 py-4 dark:bg-red-900">
          <div className="flex-0">
            <AlertCircleIcon className="size-6 text-red-900 dark:text-red-100" />
          </div>
          <div className="flex flex-1 flex-col gap-3 overflow-hidden pl-2">
            <h3 className="font-semibold">Error loading alerts</h3>
            <div className="wrap-break-word text-sm">{data.error}</div>
          </div>
        </div>
      );
    }
    const alertHistory = alerts.map((alert) => (
      <AlertPanel
        alert={alert}
        className="border-gray-200 border-b px-6 py-4 dark:border-gray-800"
        key={alert.id}
      />
    ));

    alertContent = (
      <div>
        {errorMessage}
        {alertHistory}
      </div>
    );
  }

  return (
    <Card className="m-2 gap-0 py-0">
      <CardHeader className="border-gray-200 border-b py-4! font-bold text-xl dark:border-gray-800">
        <CardTitle className="flex items-center">
          <span className="flex-1">Alerts</span>
          <Button
            disabled={isLoading}
            onClick={() => refetch()}
            size="icon"
            variant="outline"
          >
            <RefreshCcwIcon className="size-4" />
          </Button>
        </CardTitle>
      </CardHeader>
      <CardContent className="p-0">{alertContent}</CardContent>
      <CardFooter>
        <span className="py-2 text-muted-foreground text-sm">
          Last updated: 10 seconds ago
        </span>
      </CardFooter>
    </Card>
  );
};

const AlertPanel = ({
  alert,
  className,
}: {
  alert: Alert;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="flex items-center">
        <AlertCircleIcon className="size-4 text-red-500" />
        <span className="ml-2 font-medium text-sm">{alert.subject}</span>
      </div>
      <div className="flex items-center">
        <span className="text-muted-foreground text-sm">
          {alert.alertMessage}
        </span>
      </div>
      <div className="flex items-center">
        <span className="text-muted-foreground text-sm">
          fixed: {alert.fixed}
        </span>
      </div>
    </div>
  );
};
