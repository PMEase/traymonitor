use std::{
    num::NonZeroU8,
    time::{SystemTime, UNIX_EPOCH},
};

use time::{
    OffsetDateTime, PrimitiveDateTime,
    format_description::{
        self,
        well_known::{
            Iso8601,
            iso8601::{Config, EncodedConfig, TimePrecision},
        },
    },
};

pub fn format_date_time(date_time: PrimitiveDateTime, format: &str) -> Result<String, String> {
    let fmt = format_description::parse(format).map_err(|e| e.to_string())?;
    date_time.format(&fmt).map_err(|e| e.to_string())
}

const FORMAT_CONFIG: EncodedConfig = Config::DEFAULT
    .set_time_precision(TimePrecision::Second {
        decimal_digits: NonZeroU8::new(3),
    })
    .encode();

pub fn format_iso8601(dt: &PrimitiveDateTime) -> Result<String, time::error::Format> {
    dt.assume_utc().format(&Iso8601::<FORMAT_CONFIG>)
}

pub fn parse_iso8601(s: &str) -> Result<PrimitiveDateTime, time::error::Parse> {
    OffsetDateTime::parse(s, &Iso8601::<FORMAT_CONFIG>).map(odt_to_pdt)
}

const ONE_SECOND: i64 = 1000;
const ONE_MINUTE: i64 = 60 * ONE_SECOND;
const ONE_HOUR: i64 = 60 * ONE_MINUTE;
const ONE_DAY: i64 = 24 * ONE_HOUR;

pub fn format_duration(duration: i64) -> String {
    if duration < 1000 {
        return format!("{}ms", duration);
    }

    let mut duration = duration;
    let mut parts = Vec::new();

    let days = duration / ONE_DAY;
    if days > 0 {
        parts.push(format!("{}d", days));
        duration -= days * ONE_DAY;
    }
    let hours = duration / ONE_HOUR;
    if hours > 0 {
        parts.push(format!("{}h", hours));
        duration -= hours * ONE_HOUR;
    }
    let minutes = duration / ONE_MINUTE;
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
        duration -= minutes * ONE_MINUTE;
    }
    let seconds = duration / ONE_SECOND;
    if seconds > 0 {
        parts.push(format!("{}s", seconds));
        duration -= seconds * ONE_SECOND;
    }

    let p = parts.join(" ");
    if duration > 0 {
        let ms = format!("{}ms", duration);
        format!("{}, {}", p, ms)
    } else {
        p
    }
}

// now_millis returns the current time in millis
pub fn now_millis() -> u128 {
    let duration_since = SystemTime::now().duration_since(UNIX_EPOCH);
    duration_since.unwrap().as_millis()
}

/// Create a new [`PrimitiveDateTime`] with the current date and time in UTC.
pub fn now() -> PrimitiveDateTime {
    let utc_date_time = OffsetDateTime::now_utc();
    odt_to_pdt(utc_date_time)
}

/// Convert from OffsetDateTime to PrimitiveDateTime
pub fn odt_to_pdt(offset_time: OffsetDateTime) -> PrimitiveDateTime {
    PrimitiveDateTime::new(offset_time.date(), offset_time.time())
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn test_format_iso8601() {
        let date_time = datetime!(2026-01-17 10:08:23 +08:00);
        assert_eq!(
            format_iso8601(&odt_to_pdt(date_time)).unwrap(),
            "2026-01-17T10:08:23.000Z"
        );
    }

    #[test]
    fn test_parse_iso8601() {
        let date_time = "2026-01-17T22:35:13.304000000+08:00";
        let odt = datetime!(2026-01-17 22:35:13.304 +08:00);

        assert_eq!(parse_iso8601(date_time).unwrap(), odt_to_pdt(odt));
    }

    #[test]
    fn test_format_date_time() {
        let date_time = datetime!(2026-01-17 10:08:23 +08:00);

        assert_eq!(
            format_date_time(odt_to_pdt(date_time), crate::constants::DATE_TIME_FORMAT).unwrap(),
            "2026-01-17 10:08:23"
        );
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "0ms");
        assert_eq!(format_duration(100), "100ms");
        assert_eq!(format_duration(1000), "1s");
        assert_eq!(format_duration(60400), "1m, 400ms");
        assert_eq!(format_duration(3600000), "1h");
        assert_eq!(format_duration(3602000), "1h 2s");
    }
}
