"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";
import { fetch } from "@tauri-apps/plugin-http";
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
import { Switch } from "@/components/ui/switch";
import { useTheme } from "@/hooks/use-theme";
import { logger } from "@/lib/logger";
import { usePreferences, useSavePreferences } from "@/services/preferences";
import { defaultPreferences } from "@/types/preferences";

const formSchema = z.object({
  theme: z.enum(["system", "light", "dark"]),
  enable_notifications: z.boolean(),
  paused: z.boolean(),
  notifications_total: z
    .number()
    .min(1)
    .max(1000, "Notifications total must be between 1 and 1000"),
  server_url: z.url("Server URL must be a valid HTTP URL"),
  user: z
    .string()
    .min(1, "Username is required")
    .max(100, "Username must be less than 100 characters")
    .regex(
      /^[a-zA-Z0-9_-]+$/,
      "Username can only contain letters, numbers, underscores, and hyphens"
    ),
  token: z
    .string()
    .min(1, "Token is required")
    .max(256, "Token must be less than 256 characters"),
  poll_interval_in_secs: z.coerce
    .number<number>()
    .int()
    .min(0, "Poll interval must be greater than 0"),
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

  const { theme, setTheme } = useTheme();

  const onSubmit = async (data: SettingsFormValues) => {
    // Validate server URL by calling the version API
    const trimmedServerUrl = data.server_url.trim();
    logger.info("Validating server URL", {
      trimmedServerUrl,
      user: data.user,
      token: data.token,
    });
    const statusCode = await validateServerUrl(
      trimmedServerUrl,
      data.user,
      data.token
    );
    logger.info("Server URL validated", { statusCode });
    if (statusCode === 503) {
      form.setError("server_url", {
        type: "manual",
        message:
          "Unable to connect to the server. Please check the server URL and try again.",
      });
      return;
    }
    if (statusCode === 401) {
      form.setError("user", {
        type: "manual",
        message:
          "Invalid token or password. Please check the token and try again.",
      });
      return;
    }
    if (statusCode >= 400) {
      form.setError("server_url", {
        type: "manual",
        message: `Server returned status ${statusCode}. Please check the server URL and try again.`,
      });
      return;
    }

    logger.debug("Submitting settings", { data });
    savePreferences.mutate({
      ...data,
      theme,
      enable_notifications: data.enable_notifications,
      paused: data.paused,
      server_url: trimmedServerUrl,
    });

    try {
      await invoke("close_main_window");
    } catch (error) {
      logger.error("Failed to close main window", { error });
    }
  };

  const handleThemeChange = (value: "light" | "dark" | "system") => {
    // Update the theme provider immediately for instant UI feedback
    setTheme(value);

    // Persist the theme preference to disk, preserving other preferences
    // if (preferences) {
    //   savePreferences.mutate({ ...preferences, theme: value });
    // }
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
            <div className="flex items-center space-x-2">
              <Controller
                control={form.control}
                name="enable_notifications"
                render={({ field }) => (
                  <Field className="mr-auto">
                    <div className="flex items-center space-x-2">
                      <Switch
                        checked={Boolean(field.value)}
                        id="form-settings-enable-notifications"
                        onCheckedChange={field.onChange}
                        value={field.value ? "true" : "false"}
                      />
                      <FieldLabel
                        className="flex-1 font-semibold"
                        htmlFor="form-settings-enable-notifications"
                      >
                        Enable Notifications
                      </FieldLabel>
                    </div>
                  </Field>
                )}
              />
              <Controller
                control={form.control}
                name="paused"
                render={({ field }) => (
                  <Field className="ml-auto">
                    <div className="flex items-center space-x-2">
                      <Switch
                        checked={Boolean(field.value)}
                        id="form-settings-paused"
                        onCheckedChange={field.onChange}
                        value={field.value ? "true" : "false"}
                      />
                      <FieldLabel
                        className="flex-1 font-semibold"
                        htmlFor="form-settings-enable-notifications"
                      >
                        Pause Monitoring
                      </FieldLabel>
                    </div>
                  </Field>
                )}
              />
            </div>
          </FieldGroup>
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
                    <SelectTrigger id="form-settings-theme">
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

export const validateServerUrl = async (
  serverUrl: string,
  user: string,
  token: string
): Promise<number> => {
  const versionUrl = `${serverUrl}/rest/version`;
  logger.info("Validating server URL", { versionUrl, user, token });

  try {
    const statusCode = await fetch(versionUrl, {
      method: "GET",
      headers: {
        Accept: "application/json",
        Authorization: `Basic ${btoa(`${user}:${token}`)}`,
      },
    })
      .then((response) => {
        logger.debug("Response status", { status: response.status });
        return response.status;
      })
      .catch((error) => {
        logger.error("Failed to validate server URL", { error });
        return 503;
      });
    return statusCode;
  } catch (error) {
    logger.error("Failed to validate server URL", { error });
    return 503;
  }
};
