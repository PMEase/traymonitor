import { listen } from "@tauri-apps/api/event";
import {
  AlertCircleIcon,
  CheckCircleIcon,
  ClockIcon,
  Loader2Icon,
  XCircleIcon,
} from "lucide-react";
import { useEffect } from "react";
import { Loading } from "@/components/loading";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import type { Build } from "@/lib/bindings";
import { logger } from "@/lib/logger";
import { formatDuration, formatTimeAgo } from "@/lib/time";
import { cn } from "@/lib/utils";
import { useBuilds } from "@/services/builds";

export const BuildsView = () => {
  // const navigate = useNavigate();

  const { data: builds, isLoading, isError, error, refetch } = useBuilds();

  // Listen for new-builds-available event and refetch data
  useEffect(() => {
    let unlistenFn: (() => void) | null = null;
    let isMounted = true;

    const setupListener = async () => {
      try {
        const unlisten = await listen("new-builds-available", () => {
          if (isMounted) {
            logger.debug(
              "Received new-builds-available event, refreshing builds"
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
        logger.error("Failed to set up new-builds-available listener", {
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

  let buildContent: React.ReactNode = null;

  if (isLoading) {
    buildContent = <Loading className="py-6" message="Loading builds..." />;
  } else if (isError) {
    buildContent = (
      <div className="flex bg-red-100 px-6 py-4 dark:bg-red-900">
        <div className="flex-0">
          <AlertCircleIcon className="size-6 text-red-900 dark:text-red-100" />
        </div>
        <div className="flex flex-1 flex-col gap-3 pl-2">
          <h3 className="font-semibold">Error loading builds</h3>
          <div className="text-sm">{error?.message}</div>
        </div>
      </div>
    );
  } else if (builds) {
    buildContent = builds.map((build) => (
      <BuildPanel
        build={build}
        className="border-gray-200 border-b px-6 py-4 dark:border-gray-800"
        key={build.id}
      />
    ));
  } else {
    buildContent = <div>No builds</div>;
  }

  return (
    <Card className="m-2 gap-0 py-0">
      <CardHeader className="border-gray-200 border-b py-4! font-bold text-xl dark:border-gray-800">
        <CardTitle>Build Notifications</CardTitle>
      </CardHeader>
      <CardContent className="p-0">{buildContent}</CardContent>
      <CardFooter>
        <span className="py-2 text-muted-foreground text-sm">
          Last updated: 10 seconds ago
        </span>
      </CardFooter>
    </Card>
  );
};

function BuildPanel({
  build,
  className,
}: {
  build: Build;
  className?: string;
}): React.ReactNode {
  let statusIcon: React.ReactNode = null;
  switch (build.status) {
    case "SUCCESSFUL":
      statusIcon = <CheckCircleIcon className="size-6 text-green-500" />;
      break;
    case "CANCELLED":
      statusIcon = <XCircleIcon className="size-6 text-red-500" />;
      break;
    case "FAILED":
      statusIcon = <AlertCircleIcon className="size-6 text-red-500" />;
      break;
    case "TIMEOUT":
      statusIcon = <ClockIcon className="size-6 text-yellow-500" />;
      break;
    case "RECOMMENDED":
      statusIcon = (
        <CheckCircleIcon className="size-6 text-green-900 dark:text-green-100" />
      );
      break;

    default:
      statusIcon = (
        <Loader2Icon className="size-6 animate-spin text-gray-500" />
      );
      break;
  }
  return (
    <div className={cn("flex gap-2", className)}>
      <div className="">{statusIcon}</div>
      <div className="flex-1">
        <div className="flex items-center font-medium text-md">
          <span className="flex-1">{build.version}</span>
          <span className="text-muted-foreground text-sm">#{build.id}</span>
        </div>
        <div className="text-muted-foreground text-sm">
          {build.configurationPath}
        </div>
        <div className="text-muted-foreground text-sm">
          {formatTimeAgo(build.beginDate)}
        </div>
        <div className="text-muted-foreground text-sm">
          {build.statusDate ? formatTimeAgo(build.statusDate) : ""}
        </div>
        <div className="text-muted-foreground text-sm">
          {formatDuration(Number(build.duration))}
        </div>
        <div className="text-muted-foreground text-sm">
          {formatDuration(Number(build.waitDuration))}
        </div>
      </div>
    </div>
  );
}
