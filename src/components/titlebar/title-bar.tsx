import { Settings } from "lucide-react";
import { Button } from "@/components/ui/button";
import { executeCommand, useCommandContext } from "@/lib/commands";
import { cn } from "@/lib/utils";
import { MacOSWindowControls } from "./macos-window-controls";

interface TitleBarProps {
  className?: string;
  title?: string;
}

export function TitleBar({ className, title = "Tray Monitor" }: TitleBarProps) {
  const commandContext = useCommandContext();
  return (
    <div
      className={cn(
        "relative flex h-8 w-full shrink-0 items-center justify-between border-b bg-background",
        className
      )}
      data-tauri-drag-region
    >
      {/* Left side - Window Controls + Left Actions */}
      <div className="flex items-center">
        <MacOSWindowControls />

        {/* Left Action Buttons */}
      </div>

      {/* Center - Title */}
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
        <span className="font-medium text-foreground/80 text-sm">{title}</span>
      </div>

      {/* Right side - Right Actions */}
      <div className="flex items-center gap-1 pr-2">
        <Button
          className="h-6 w-6 text-foreground/70 hover:text-foreground"
          onClick={() => executeCommand("open-preferences", commandContext)}
          size="icon"
          title="Settings"
          variant="ghost"
        >
          <Settings className="h-3 w-3" />
        </Button>
      </div>
    </div>
  );
}

export default TitleBar;
