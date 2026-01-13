import { usePreferences } from "@/services/preferences";

export const DashboardView = () => {
  const { data: preferences, isLoading } = usePreferences();
  const serverUrl = preferences?.server_url;
  if (isLoading) {
    return <div>Loading...</div>;
  }
  if (!serverUrl) {
    return <div>No server URL found</div>;
  }
  return <div>Connecting to server {serverUrl}...</div>;
};
