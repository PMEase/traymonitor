import { InfoIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { commands } from "@/lib/bindings";
import { runUpdateFlow } from "@/lib/updater";

const COPYRIGHT = "Copyright © 2025 PMEase. All rights reserved.";

export const AboutView = () => {
  const [appName, setAppName] = useState<string>("");
  const [appVersion, setAppVersion] = useState<string>("");

  useEffect(() => {
    commands
      .getAppInfo()
      .then(([name, version]) => {
        setAppName(name);
        setAppVersion(version);
      })
      .catch(() => {
        setAppName("QuickBuild Tray Monitor");
        setAppVersion("—");
      });
  }, []);

  const handleCheckUpdates = () => {
    runUpdateFlow({ silent: false });
  };

  return (
    <Card className="m-0 gap-0 rounded-none border-none py-0">
      <CardHeader className="border-gray-200 border-b py-6 dark:border-gray-800">
        <CardTitle className="flex items-center gap-2 font-bold text-xl">
          <InfoIcon className="size-6" />
          About
        </CardTitle>
        <CardDescription>Application information and version</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-6 py-6">
        <div className="flex flex-col gap-1">
          <p className="font-semibold text-lg">
            {appName || "QuickBuild Tray Monitor"}
          </p>
          <p className="text-muted-foreground text-sm">
            Version {appVersion || "—"}
          </p>
        </div>
        <Separator />
        <p className="text-muted-foreground text-sm">{COPYRIGHT}</p>
        <p className="text-muted-foreground text-sm">
          Tray Monitor is a desktop application built with Tauri and React that
          provides real-time monitoring and management for QuickBuild servers
          from your system tray.
        </p>
        <Button
          className="w-fit"
          onClick={handleCheckUpdates}
          variant="outline"
        >
          Check for Updates
        </Button>
      </CardContent>
    </Card>
  );
};
