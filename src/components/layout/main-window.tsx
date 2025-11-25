import { Toaster } from "sonner";
import { CommandPalette } from "@/components/command-palette/command-palette";
import { PreferencesDialog } from "@/components/preferences/preferences-dialog";
import { TitleBar } from "@/components/titlebar/title-bar";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { useMainWindowEventListeners } from "@/hooks/use-main-window-event-listeners";
import { useTheme } from "@/hooks/use-theme";
import { cn } from "@/lib/utils";
import { useUIStore } from "@/store/ui-store";
import { LeftSideBar } from "./left-sidebar";
import { MainWindowContent } from "./main-window-content";
import { RightSideBar } from "./right-sidebar";

export function MainWindow() {
  const { theme } = useTheme();
  const { leftSidebarVisible, rightSidebarVisible } = useUIStore();

  // Set up global event listeners (keyboard shortcuts, etc.)
  useMainWindowEventListeners();

  return (
    <div className="flex h-screen w-full flex-col overflow-hidden rounded-xl bg-background">
      {/* Title Bar */}
      <TitleBar />

      {/* Main Content Area with Resizable Panels */}
      <div className="flex flex-1 overflow-hidden">
        <ResizablePanelGroup direction="horizontal">
          {/* Left Sidebar */}
          <ResizablePanel
            className={cn(!leftSidebarVisible && "hidden")}
            defaultSize={20}
            maxSize={40}
            minSize={15}
          >
            <LeftSideBar />
          </ResizablePanel>

          <ResizableHandle className={cn(!leftSidebarVisible && "hidden")} />

          {/* Main Content */}
          <ResizablePanel defaultSize={60} minSize={30}>
            <MainWindowContent />
          </ResizablePanel>

          <ResizableHandle className={cn(!rightSidebarVisible && "hidden")} />

          {/* Right Sidebar */}
          <ResizablePanel
            className={cn(!rightSidebarVisible && "hidden")}
            defaultSize={20}
            maxSize={40}
            minSize={15}
          >
            <RightSideBar />
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>

      {/* Global UI Components (hidden until triggered) */}
      <CommandPalette />
      <PreferencesDialog />
      <Toaster
        className="toaster group"
        position="bottom-right"
        theme={theme === "dark" || theme === "light" ? theme : "system"}
        toastOptions={{
          classNames: {
            toast:
              "group toast group-[.toaster]:bg-background group-[.toaster]:text-foreground group-[.toaster]:border-border group-[.toaster]:shadow-lg",
            description: "group-[.toast]:text-muted-foreground",
            actionButton:
              "group-[.toast]:bg-primary group-[.toast]:text-primary-foreground",
            cancelButton:
              "group-[.toast]:bg-muted group-[.toast]:text-muted-foreground",
          },
        }}
      />
    </div>
  );
}

export default MainWindow;
