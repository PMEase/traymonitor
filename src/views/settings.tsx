"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { Controller, useForm } from "react-hook-form";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useTheme } from "@/hooks/use-theme";
import { logger } from "@/lib/logger";
import { usePreferences, useSavePreferences } from "@/services/preferences";
import { defaultPreferences } from "@/types/preferences";

const formSchema = z.object({
  theme: z.enum(["system", "light", "dark"]),
  enable_notifications: z.boolean(),
  notifications_total: z.number().min(1).max(1000),
  server_url: z.url("Server URL must be a valid HTTP URL"),
  user: z
    .string()
    .min(1)
    .max(100)
    .regex(
      /^[a-zA-Z0-9_-]+$/,
      "Username can only contain letters, numbers, underscores, and hyphens"
    ),
  token: z.string().min(1).max(256),
  poll_interval_in_secs: z.coerce.number<number>().int().min(0),
});

type SettingsFormValues = z.infer<typeof formSchema>;

export const SettingsView = () => {
  const { data: preferences } = usePreferences();
  const savePreferences = useSavePreferences();

  const form = useForm<SettingsFormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      ...(preferences ?? defaultPreferences),
    },
  });

  const onSubmit = (data: SettingsFormValues) => {
    logger.debug("Submitting settings", { data });
    savePreferences.mutate({
      ...data,
      server_url: data.server_url.trim(),
    });
  };
  const { theme, setTheme } = useTheme();

  const handleThemeChange = (value: "light" | "dark" | "system") => {
    // Update the theme provider immediately for instant UI feedback
    setTheme(value);

    // Persist the theme preference to disk, preserving other preferences
    if (preferences) {
      savePreferences.mutate({ ...preferences, theme: value });
    }
  };

  return (
    <Card className="m-2">
      <CardHeader className="border-gray-200 border-b font-bold text-xl dark:border-gray-800">
        <CardTitle>Preferences</CardTitle>
      </CardHeader>
      <CardContent>
        <form
          className="space-y-8"
          id="form-settings"
          onSubmit={form.handleSubmit(onSubmit)}
        >
          <FieldGroup>
            <Controller
              control={form.control}
              name="theme"
              render={({ fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel
                    className="font-semibold"
                    htmlFor="form-settings-theme"
                  >
                    Theme
                  </FieldLabel>
                  <Select onValueChange={handleThemeChange} value={theme}>
                    <SelectTrigger>
                      <SelectValue placeholder="Select theme" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="system">System</SelectItem>
                      <SelectItem value="light">Light</SelectItem>
                      <SelectItem value="dark">Dark</SelectItem>
                    </SelectContent>
                  </Select>
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
            <Controller
              control={form.control}
              name="server_url"
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel
                    className="font-semibold"
                    htmlFor="form-settings-server-url"
                  >
                    QuickBuild Server
                  </FieldLabel>
                  <Input
                    {...field}
                    aria-invalid={fieldState.invalid}
                    autoComplete="off"
                    id="form-settings-server-url"
                    placeholder="http://quickbuild:8810"
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
            <Controller
              control={form.control}
              name="user"
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel
                    className="font-semibold"
                    htmlFor="form-settings-user"
                  >
                    User
                  </FieldLabel>
                  <Input
                    {...field}
                    aria-invalid={fieldState.invalid}
                    autoComplete="off"
                    id="form-settings-user"
                    placeholder="username"
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
            <Controller
              control={form.control}
              name="token"
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel
                    className="font-semibold"
                    htmlFor="form-settings-token"
                  >
                    Token/Password
                  </FieldLabel>
                  <Input
                    {...field}
                    aria-invalid={fieldState.invalid}
                    autoComplete="off"
                    id="form-settings-token"
                    placeholder="token or password"
                    type="password"
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
            <Controller
              control={form.control}
              name="poll_interval_in_secs"
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel
                    className="font-semibold"
                    htmlFor="form-settings-poll-interval-in-secs"
                  >
                    Poll Interval (seconds)
                  </FieldLabel>
                  <Input
                    {...field}
                    aria-invalid={fieldState.invalid}
                    autoComplete="off"
                    id="form-settings-poll-interval-in-secs"
                    placeholder="10"
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
          </FieldGroup>
          <Field className="justify-end" orientation="horizontal">
            <Button className="w-full" form="form-settings" type="submit">
              Save Settings
            </Button>
          </Field>
        </form>
      </CardContent>
    </Card>
  );
};
