import type React from "react";
import { useCallback } from "react";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { useTheme } from "@/hooks/use-theme";
import { useSavePreferences } from "@/services/preferences";

const SettingsField: React.FC<{
  label: string;
  children: React.ReactNode;
  description?: string;
}> = ({ label, children, description }) => (
  <div className="space-y-2">
    <Label className="font-medium text-foreground text-sm">{label}</Label>
    {children}
    {description && (
      <p className="text-muted-foreground text-sm">{description}</p>
    )}
  </div>
);

const SettingsSection: React.FC<{
  title: string;
  children: React.ReactNode;
}> = ({ title, children }) => (
  <div className="space-y-4">
    <div>
      <h3 className="font-medium text-foreground text-lg">{title}</h3>
      <Separator className="mt-2" />
    </div>
    <div className="space-y-4">{children}</div>
  </div>
);

export const AppearancePane: React.FC = () => {
  const { theme, setTheme } = useTheme();
  const savePreferences = useSavePreferences();

  const handleThemeChange = useCallback(
    // biome-ignore lint/suspicious/useAwait: suppress
    async (value: "light" | "dark" | "system") => {
      // Update the theme provider immediately for instant UI feedback
      setTheme(value);

      // Persist the theme preference to disk
      savePreferences.mutate({ theme: value });
    },
    [setTheme, savePreferences]
  );

  return (
    <div className="space-y-6">
      <SettingsSection title="Theme">
        <SettingsField
          description="Choose your preferred color theme"
          label="Color Theme"
        >
          <Select
            disabled={savePreferences.isPending}
            onValueChange={handleThemeChange}
            value={theme}
          >
            <SelectTrigger>
              <SelectValue placeholder="Select theme" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="light">Light</SelectItem>
              <SelectItem value="dark">Dark</SelectItem>
              <SelectItem value="system">System</SelectItem>
            </SelectContent>
          </Select>
        </SettingsField>
      </SettingsSection>
    </div>
  );
};
