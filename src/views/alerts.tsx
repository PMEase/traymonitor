import { listen } from "@tauri-apps/api/event";
import { format } from "date-fns";
import { AlertCircleIcon, RefreshCcwIcon } from "lucide-react";
import { type ReactNode, useEffect } from "react";
import { Loading } from "@/components/loading";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import type { Alert, AlertPriority } from "@/lib/bindings";
import { logger } from "@/lib/logger";
import { cn } from "@/lib/utils";
import { useAlerts } from "@/services/alerts";

export const AlertsView = () => {
  const { data, isLoading, isError, error, refetch } = useAlerts();

  // Listen for new-builds-available event and refetch data
  useEffect(() => {
    let unlistenFn: (() => void) | null = null;
    let isMounted = true;

    const setupListener = async () => {
      try {
        const unlisten = await listen("alerts-refresh-page", () => {
          if (isMounted) {
            logger.debug(
              "Received alerts-refresh-page event, refreshing page now..."
            );
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
        logger.error("Failed to set up alerts-refresh-page listener", {
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
    <Card className="m-0 gap-0 rounded-none border-none py-0">
      <CardHeader className="border-gray-200 border-b py-2! font-bold text-xl dark:border-gray-800">
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
        <div className="py-5 text-muted-foreground text-sm">
          Last updated: 10 seconds ago
        </div>
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
    <div className={cn("flex gap-2", className)}>
      <div className="flex-0">
        <AlertPriorityLabel priority={alert.priority} />
      </div>
      <div className="flex-1 overflow-x-hidden">
        <div className="mb-2 flex flex-col gap-1">
          <div className="flex-1 font-semibold text-md">{alert.subject}</div>
          <div className="text-muted-foreground text-xs">
            {formatDateTime(alert.ctime)}
          </div>
        </div>
        <div className="flex flex-col gap-1">
          <div className="text-sm">{alert.alertMessage}</div>
        </div>
      </div>
    </div>
  );
};

const AlertPriorityLabel = ({
  priority,
}: {
  priority: AlertPriority;
}): ReactNode => {
  const classes = `flex items-center justify-center font-mono text-xl leading-none
  border rounded-full size-6
  `;

  switch (priority) {
    case "LOW":
      return (
        <div className={cn(classes, "border-green-9 text-green-9")}>L</div>
      );
    case "MEDIUM":
      return (
        <div className={cn(classes, "border-orange-9 text-orange-9")}>M</div>
      );
    case "HIGH":
      return <div className={cn(classes, "border-red-9 text-red-9")}>H</div>;
    default:
      return <div className={cn(classes, "border-gray-9 text-gray-9")}>?</div>;
  }
};

function formatDateTime(ctime: string): string {
  const dt = new Date(ctime);
  return format(dt, "yyyy-MM-dd HH:mm:ss");
}
