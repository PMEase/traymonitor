import { describe, expect, it } from "vitest";
import { formatDuration } from "./time";

describe("formatDuration", () => {
  describe("edge cases", () => {
    it("returns '0s' for zero duration", () => {
      expect(formatDuration(0)).toBe("0s");
    });

    it("returns '0s' for negative duration", () => {
      expect(formatDuration(-1000)).toBe("0s");
      expect(formatDuration(-5000)).toBe("0s");
    });
  });

  describe("seconds only", () => {
    it("formats single second correctly", () => {
      expect(formatDuration(1000)).toBe("1s");
    });

    it("formats multiple seconds correctly", () => {
      expect(formatDuration(5000)).toBe("5s");
      expect(formatDuration(30_000)).toBe("30s");
      expect(formatDuration(59_000)).toBe("59s");
    });
  });

  describe("minutes only", () => {
    it("formats single minute correctly", () => {
      expect(formatDuration(60_000)).toBe("1m");
    });

    it("formats multiple minutes correctly", () => {
      expect(formatDuration(120_000)).toBe("2m");
      expect(formatDuration(300_000)).toBe("5m");
      expect(formatDuration(3_540_000)).toBe("59m");
    });

    it("formats minutes with seconds correctly", () => {
      expect(formatDuration(90_000)).toBe("1m 30s");
      expect(formatDuration(125_000)).toBe("2m 5s");
      expect(formatDuration(3_599_000)).toBe("59m 59s");
    });
  });

  describe("hours only", () => {
    it("formats single hour correctly", () => {
      expect(formatDuration(3_600_000)).toBe("1h");
    });

    it("formats multiple hours correctly", () => {
      expect(formatDuration(7_200_000)).toBe("2h");
      expect(formatDuration(18_000_000)).toBe("5h");
      expect(formatDuration(86_400_000)).toBe("24h");
    });

    it("formats hours with seconds correctly", () => {
      expect(formatDuration(3_605_000)).toBe("1h 5s");
      expect(formatDuration(7_230_000)).toBe("2h 30s");
    });

    it("formats hours with minutes correctly", () => {
      expect(formatDuration(5_400_000)).toBe("1h 30m");
      expect(formatDuration(9_000_000)).toBe("2h 30m");
    });

    it("formats hours with minutes and seconds correctly", () => {
      expect(formatDuration(3_665_000)).toBe("1h 1m 5s");
      expect(formatDuration(7_323_000)).toBe("2h 2m 3s");
      expect(formatDuration(9_005_000)).toBe("2h 30m 5s");
    });
  });

  describe("millisecond precision", () => {
    it("rounds down milliseconds correctly", () => {
      expect(formatDuration(999)).toBe("999ms");
      expect(formatDuration(1999)).toBe("1s");
      expect(formatDuration(59_999)).toBe("59s");
      expect(formatDuration(60_001)).toBe("1m");
    });
  });

  describe("large durations", () => {
    it("handles very large durations correctly", () => {
      expect(formatDuration(36_000_000)).toBe("10h");
      expect(formatDuration(86_400_000)).toBe("24h");
      expect(formatDuration(90_061_000)).toBe("25h 1m 1s");
    });
  });
});
