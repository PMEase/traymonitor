import type React from "react";
import { useState } from "react";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { Switch } from "@/components/ui/switch";

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
  // Example local state - these are NOT persisted to disk
  // To add persistent preferences:
  // 1. Add the field to AppPreferences in both Rust and TypeScript
  // 2. Use usePreferencesManager() and updatePreferences()
  const [exampleText, setExampleText] = useState("Example value");
  const [exampleToggle, setExampleToggle] = useState(true);

  return (
    <div className="space-y-6">
      <SettingsSection title="Example Settings">
        <SettingsField
          description="This is an example text input setting (not persisted)"
          label="Example Text Setting"
        >
          <Input
            onChange={(e) => setExampleText(e.target.value)}
            placeholder="Enter example text"
            value={exampleText}
          />
        </SettingsField>

        <SettingsField
          description="This is an example switch/toggle setting (not persisted)"
          label="Example Toggle Setting"
        >
          <div className="flex items-center space-x-2">
            <Switch
              checked={exampleToggle}
              id="example-toggle"
              onCheckedChange={setExampleToggle}
            />
            <Label className="text-sm" htmlFor="example-toggle">
              {exampleToggle ? "Enabled" : "Disabled"}
            </Label>
          </div>
        </SettingsField>
      </SettingsSection>
    </div>
  );
};
