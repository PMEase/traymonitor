import { listen } from "@tauri-apps/api/event";
import {
  AlertCircleIcon,
  BanIcon,
  CheckCircleIcon,
  CircleStarIcon,
  CircleXIcon,
  ClockAlertIcon,
  Loader2Icon,
  RefreshCcwIcon,
} from "lucide-react";
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
import type { Build } from "@/lib/bindings";
import { logger } from "@/lib/logger";
import { formatDuration, formatTimeAgo } from "@/lib/time";
import { cn } from "@/lib/utils";
import { useBuilds } from "@/services/builds";

export const BuildsView = () => {
  // const navigate = useNavigate();

  const { data, isLoading, isError, error, refetch } = useBuilds();

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
  } else {
    const builds = data?.builds ?? [];
    let errorMessage: React.ReactNode | null = null;
    if (data?.error) {
      errorMessage = (
        <div className="flex bg-red-100 px-6 py-4 dark:bg-red-900">
          <div className="flex-0">
            <AlertCircleIcon className="size-6 text-red-900 dark:text-red-100" />
          </div>
          <div className="flex flex-1 flex-col gap-3 pl-2">
            <h3 className="font-semibold">Error loading builds</h3>
            <div className="text-sm">{data.error}</div>
          </div>
        </div>
      );
    }
    const buildHistory = builds.map((build) => (
      <BuildPanel
        build={build}
        className="border-gray-200 border-b px-6 py-4 dark:border-gray-800"
        key={build.id}
      />
    ));

    buildContent = (
      <div>
        {errorMessage}
        {buildHistory}
      </div>
    );
  }

  return (
    <Card className="m-2 gap-0 py-0">
      <CardHeader className="border-gray-200 border-b py-4! font-bold text-xl dark:border-gray-800">
        <CardTitle className="flex items-center">
          <span className="flex-1">Build Notifications</span>
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
      <CardContent className="p-0">{buildContent}</CardContent>
      <CardFooter>
        <span className="py-2 text-muted-foreground text-sm">
          Last updated: 10 seconds ago
        </span>
      </CardFooter>
    </Card>
  );
};

const statusColors = {
  SUCCESSFUL: "text-green-9",
  CANCELLED: "text-purple-9",
  FAILED: "text-red-9",
  TIMEOUT: "text-amber-9",
  RECOMMENDED: "text-jade-9",
  RUNNING: "text-indigo-9",
};

function getBuildStatusIcon(status: string): React.ReactNode {
  switch (status) {
    case "SUCCESSFUL":
      return (
        <CheckCircleIcon className={cn("size-6", statusColors.SUCCESSFUL)} />
      );
    case "CANCELLED":
      return <BanIcon className={cn("size-6", statusColors.CANCELLED)} />;
    case "FAILED":
      return <CircleXIcon className={cn("size-6", statusColors.FAILED)} />;
    case "TIMEOUT":
      return <ClockAlertIcon className={cn("size-6", statusColors.TIMEOUT)} />;
    case "RECOMMENDED":
      return (
        <CircleStarIcon className={cn("size-6", statusColors.RECOMMENDED)} />
      );
    case "RUNNING":
      return (
        <Loader2Icon
          className={cn("size-6 animate-spin", statusColors.RUNNING)}
        />
      );
    default: {
      return (
        <Loader2Icon
          className={cn("size-6 animate-spin", statusColors.RUNNING)}
        />
      );
    }
  }
}

function getBuildStatusTitle(build: Build): string {
  switch (build.status) {
    case "SUCCESSFUL":
      return `Build ${build.version} finished successfully 🎉`;
    case "CANCELLED":
      return `Build ${build.version} cancelled`;
    case "FAILED":
      return `Build ${build.version} failed`;
    case "TIMEOUT":
      return `Build ${build.version} timed out`;
    case "RECOMMENDED":
      return `Build ${build.version} is recommended`;
    case "RUNNING":
      return `Build ${build.version} is running`;
    default:
      return `Build ${build.version} is running`;
  }
}

export const BuildPanel = ({
  build,
  className,
}: {
  build: Build;
  className?: string;
}) => {
  const statusIcon = getBuildStatusIcon(build.status);
  const title = getBuildStatusTitle(build);

  return (
    <div className={cn("flex gap-2", className)}>
      <div className="flex-0">{statusIcon}</div>
      <div className="flex-1">
        <div className="mb-3 flex w-full items-center">
          <span className="flex-1 font-semibold text-md">{title}</span>
        </div>
        <div className="flex w-full gap-2">
          <div className="w-1/3 truncate font-semibold text-sm lg:w-1/4">
            Configuration
          </div>
          <div className="w-2/3 truncate text-muted-foreground text-sm lg:w-3/4">
            {build.configurationPath}
          </div>
        </div>
        <div className="flex w-full gap-2">
          <div className="w-1/3 truncate font-semibold text-sm lg:w-1/4">
            Triggered by
          </div>
          <div className="w-2/3 truncate text-muted-foreground text-sm lg:w-3/4">
            {build.requesterName}
          </div>
        </div>
        <div className="flex w-full gap-2">
          <div className="w-1/3 truncate font-semibold text-sm lg:w-1/4">
            Started at
          </div>
          <div className="w-2/3 truncate text-muted-foreground text-sm lg:w-3/4">
            {formatTimeAgo(build.beginDate)}
          </div>
        </div>
        <div className="flex w-full gap-2">
          <div className="w-1/3 truncate font-semibold text-sm lg:w-1/4">
            Duration
          </div>
          <div className="w-2/3 truncate text-muted-foreground text-sm lg:w-3/4">
            {formatDuration(Number(build.duration))}
          </div>
        </div>
      </div>
    </div>
  );
};
