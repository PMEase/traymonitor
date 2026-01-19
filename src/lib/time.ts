export function formatTimeAgo(date: Date | string, shortFmt = false): string {
  if (!date || date === "") {
    return "N/A";
  }

  // Initialize a default date as the current date
  let _date: Date = new Date();

  if (typeof date === "string") {
    // Check if the input is a string and convert it to a Date object
    _date = new Date(date);
  } else {
    _date = date;
  }

  // Calculate the time difference in seconds
  const seconds: number = Math.floor((Date.now() - _date.getTime()) / 1000);

  // Define intervals for different time units in seconds
  const intervals: Record<string, number> = {
    year: 31_536_000,
    month: 2_628_000,
    day: 86_400,
    hour: 3600,
    minute: 60,
  };

  // Iterate through the intervals and determine the appropriate unit
  for (const [unit, secondsInUnit] of Object.entries(intervals)) {
    const interval: number = Math.floor(seconds / secondsInUnit);
    if (interval > 1) {
      return shortFmt
        ? `${interval}${unit.charAt(0)}`
        : `${interval} ${unit}s ago`;
    }
    if (interval === 1) {
      return shortFmt
        ? `${interval}${unit.charAt(0)}`
        : `${interval} ${unit} ago`;
    }
  }

  // If no larger unit is found, return "just now"
  return shortFmt ? "now" : "just now";
}

export function formatDuration(duration: number): string {
  // Handle edge cases
  if (duration <= 0) {
    return "0s";
  }

  if (duration < 1000) {
    return `${duration}ms`;
  }

  // Convert milliseconds to seconds
  const totalSeconds = Math.floor(duration / 1000);

  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  const parts: string[] = [];

  // Only add non-zero units
  if (hours > 0) {
    parts.push(`${hours}h`);
  }
  if (minutes > 0) {
    parts.push(`${minutes}m`);
  }
  if (seconds > 0 || parts.length === 0) {
    parts.push(`${seconds}s`);
  }

  return parts.join(" ");
}
