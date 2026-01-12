import type React from "react";
import { useEffect, useState } from "react";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { Switch } from "@/components/ui/switch";
import { usePreferences, useSavePreferences } from "@/services/preferences";
import { logger } from "@/lib/logger";

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

export const GeneralPane: React.FC = () => {
  const { data: preferences, isLoading } = usePreferences();
  const savePreferences = useSavePreferences();

  // Local state for form fields
  const [serverUrl, setServerUrl] = useState("");
  const [enableNotifications, setEnableNotifications] = useState(true);

  // Update local state when preferences load
  useEffect(() => {
    if (preferences) {
      setServerUrl(preferences.server_url || "");
      setEnableNotifications(preferences.enable_notifications ?? true);
    }
  }, [preferences]);

  // Handle saving preferences
  const handleSave = async () => {
    if (!preferences) return;

    try {
      await savePreferences.mutateAsync({
        ...preferences,
        server_url: serverUrl.trim(),
        enable_notifications: enableNotifications,
      });
      logger.info("Preferences saved successfully");
    } catch (error) {
      logger.error("Failed to save preferences", { error });
    }
  };

  // Auto-save on blur for server URL
  const handleServerUrlBlur = () => {
    if (preferences && serverUrl.trim() !== (preferences.server_url || "")) {
      handleSave();
    }
  };

  // Auto-save on toggle change
  const handleNotificationToggle = (checked: boolean) => {
    setEnableNotifications(checked);
    if (preferences) {
      savePreferences.mutate({
        ...preferences,
        enable_notifications: checked,
      });
    }
  };

  if (isLoading) {
    return <div className="text-sm text-muted-foreground">Loading preferences...</div>;
  }

  return (
    <div className="space-y-6">
      <SettingsSection title="Server Configuration">
        <SettingsField
          description="The base URL of your QuickBuild server. The dashboard will be displayed at {server_url}/lite"
          label="Server URL"
        >
          <Input
            onChange={(e) => setServerUrl(e.target.value)}
            onBlur={handleServerUrlBlur}
            placeholder="http://quickbuild:8810"
            value={serverUrl}
            disabled={savePreferences.isPending}
          />
        </SettingsField>
      </SettingsSection>

      <SettingsSection title="Notifications">
        <SettingsField
          description="Enable or disable system notifications"
          label="Enable Notifications"
        >
          <div className="flex items-center space-x-2">
            <Switch
              checked={enableNotifications}
              id="enable-notifications"
              onCheckedChange={handleNotificationToggle}
              disabled={savePreferences.isPending}
            />
            <Label className="text-sm" htmlFor="enable-notifications">
              {enableNotifications ? "Enabled" : "Disabled"}
            </Label>
          </div>
        </SettingsField>
      </SettingsSection>
    </div>
  );
};
