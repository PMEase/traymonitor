import {
  PanelLeft,
  PanelLeftClose,
  PanelRight,
  PanelRightClose,
  Settings,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { executeCommand, useCommandContext } from "@/lib/commands";
import { cn } from "@/lib/utils";
import { useUIStore } from "@/store/ui-store";
import { MacOSWindowControls } from "./macos-window-controls";

interface TitleBarProps {
  className?: string;
  title?: string;
}

export function TitleBar({ className, title = "Tauri App" }: TitleBarProps) {
  const {
    leftSidebarVisible,
    rightSidebarVisible,
    toggleLeftSidebar,
    toggleRightSidebar,
  } = useUIStore();
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
        <div className="flex items-center gap-1">
          <Button
            className="h-6 w-6 text-foreground/70 hover:text-foreground"
            onClick={toggleLeftSidebar}
            size="icon"
            title={
              leftSidebarVisible ? "Hide Left Sidebar" : "Show Left Sidebar"
            }
            variant="ghost"
          >
            {leftSidebarVisible ? (
              <PanelLeftClose className="h-3 w-3" />
            ) : (
              <PanelLeft className="h-3 w-3" />
            )}
          </Button>
        </div>
      </div>

      {/* Center - Title */}
      <div className="-translate-x-1/2 -translate-y-1/2 absolute top-1/2 left-1/2">
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

        <Button
          className="h-6 w-6 text-foreground/70 hover:text-foreground"
          onClick={toggleRightSidebar}
          size="icon"
          title={
            rightSidebarVisible ? "Hide Right Sidebar" : "Show Right Sidebar"
          }
          variant="ghost"
        >
          {rightSidebarVisible ? (
            <PanelRightClose className="h-3 w-3" />
          ) : (
            <PanelRight className="h-3 w-3" />
          )}
        </Button>
      </div>
    </div>
  );
}

export default TitleBar;
