import { notifications } from "@/lib/notifications";
import type { AppCommand } from "./types";

export const notificationCommands: AppCommand[] = [
  {
    id: "notification.test-toast",
    label: "Test Toast Notification",
    description: "Show a test toast notification",
    group: "debug",
    keywords: ["test", "toast", "notification", "debug"],
    async execute() {
      await notifications.success(
        "Test Toast",
        "This is a test notification",
        true
      );
    },
  },
];
