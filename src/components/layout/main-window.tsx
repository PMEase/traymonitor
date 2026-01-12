import { Toaster } from "sonner";
import { CommandPalette } from "@/components/command-palette/command-palette";
import { PreferencesDialog } from "@/components/preferences/preferences-dialog";
import { TitleBar } from "@/components/titlebar/title-bar";
import { useMainWindowEventListeners } from "@/hooks/use-main-window-event-listeners";
import { useTheme } from "@/hooks/use-theme";
import { MainWindowContent } from "./main-window-content";

export function MainWindow() {
  const { theme } = useTheme();

  // Set up global event listeners (keyboard shortcuts, etc.)
  useMainWindowEventListeners();

  return (
    <div className="flex h-screen w-full flex-col overflow-hidden bg-background">
      {/* Title Bar */}
      <TitleBar />

      {/* Main Content Area with Resizable Panels */}
      <div className="flex-1 overflow-hidden">
        <MainWindowContent />
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
