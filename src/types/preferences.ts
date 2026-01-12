// Re-export the auto-generated AppPreference type from bindings
import type { AppSettings } from "@/lib/bindings";

// Use the auto-generated type for consistency with Rust backend
export type AppPreferences = AppSettings;

export const defaultPreferences: AppPreferences = {
  theme: "system",
  enable_notifications: true,
  notifications_total: 100,
  server_url: "http://quickbuild:8810",
  user: "user",
  token: "token",
  poll_interval_in_secs: 10,
};
