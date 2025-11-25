import { Palette, Settings, Zap } from "lucide-react";
import { useState } from "react";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
} from "@/components/ui/sidebar";
import { useUIStore } from "@/store/ui-store";
import { AdvancedPane } from "./panes/advanced-pane";
import { AppearancePane } from "./panes/appearance-pane";
import { GeneralPane } from "./panes/general-pane";

type PreferencePane = "general" | "appearance" | "advanced";

const navigationItems = [
  {
    id: "general" as const,
    name: "General",
    icon: Settings,
  },
  {
    id: "appearance" as const,
    name: "Appearance",
    icon: Palette,
  },
  {
    id: "advanced" as const,
    name: "Advanced",
    icon: Zap,
  },
];

const getPaneTitle = (pane: PreferencePane): string => {
  switch (pane) {
    case "general":
      return "General";
    case "appearance":
      return "Appearance";
    case "advanced":
      return "Advanced";
    default:
      return "General";
  }
};

export function PreferencesDialog() {
  const [activePane, setActivePane] = useState<PreferencePane>("general");
  const { preferencesOpen, setPreferencesOpen } = useUIStore();

  return (
    <Dialog onOpenChange={setPreferencesOpen} open={preferencesOpen}>
      <DialogContent className="overflow-hidden rounded-xl p-0 font-sans md:max-h-[600px] md:max-w-[900px] lg:max-w-[1000px]">
        <DialogTitle className="sr-only">Preferences</DialogTitle>
        <DialogDescription className="sr-only">
          Customize your application preferences here.
        </DialogDescription>

        <SidebarProvider className="items-start">
          <Sidebar className="hidden md:flex" collapsible="none">
            <SidebarContent>
              <SidebarGroup>
                <SidebarGroupContent>
                  <SidebarMenu>
                    {navigationItems.map((item) => (
                      <SidebarMenuItem key={item.id}>
                        <SidebarMenuButton
                          asChild
                          isActive={activePane === item.id}
                        >
                          <button
                            className="w-full"
                            onClick={() => setActivePane(item.id)}
                            type="button"
                          >
                            <item.icon />
                            <span>{item.name}</span>
                          </button>
                        </SidebarMenuButton>
                      </SidebarMenuItem>
                    ))}
                  </SidebarMenu>
                </SidebarGroupContent>
              </SidebarGroup>
            </SidebarContent>
          </Sidebar>

          <main className="flex flex-1 flex-col overflow-hidden">
            <header className="flex h-16 shrink-0 items-center gap-2">
              <div className="flex items-center gap-2 px-4">
                <Breadcrumb>
                  <BreadcrumbList>
                    <BreadcrumbItem className="hidden md:block">
                      <BreadcrumbLink href="#">Preferences</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator className="hidden md:block" />
                    <BreadcrumbItem>
                      <BreadcrumbPage>
                        {getPaneTitle(activePane)}
                      </BreadcrumbPage>
                    </BreadcrumbItem>
                  </BreadcrumbList>
                </Breadcrumb>
              </div>
            </header>

            <div className="flex max-h-[calc(600px-4rem)] flex-1 flex-col gap-4 overflow-y-auto p-4 pt-0">
              {activePane === "general" && <GeneralPane />}
              {activePane === "appearance" && <AppearancePane />}
              {activePane === "advanced" && <AdvancedPane />}
            </div>
          </main>
        </SidebarProvider>
      </DialogContent>
    </Dialog>
  );
}

export default PreferencesDialog;
