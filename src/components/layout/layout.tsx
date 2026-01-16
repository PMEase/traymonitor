import { Toaster } from "sonner";
import { useMainWindowEventListeners } from "@/hooks/use-main-window-event-listeners";
import { useTheme } from "@/hooks/use-theme";

export function Layout({ children }: { children: React.ReactNode }) {
  const { theme } = useTheme();

  // Set up global event listeners (keyboard shortcuts, etc.)
  useMainWindowEventListeners();

  return (
    <div className="flex h-screen w-full flex-col overflow-hidden bg-background">
      {/* Main Content Area with Resizable Panels */}
      <div className="flex h-full flex-1 flex-col overflow-auto">
        {children}
      </div>

      {/* <CommandPalette /> */}
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

export default Layout;
