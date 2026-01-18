pub mod four_year_iso8601 {
    use serde::de::Error as _;
    use serde::ser::Error as _;
    use serde::{Deserialize, Deserializer};
    use serde::{Serialize, Serializer};
    use time::OffsetDateTime;
    use time::format_description::well_known::iso8601;
    use time::format_description::well_known::iso8601::EncodedConfig;

    use time::format_description::well_known::Iso8601;

    pub(crate) const SERDE_CONFIG: EncodedConfig = iso8601::Config::DEFAULT
        .set_year_is_six_digits(false)
        .encode();

    pub fn serialize<S: Serializer>(
        datetime: &OffsetDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        datetime
            .format(&Iso8601::<SERDE_CONFIG>)
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        OffsetDateTime::parse(&s, &Iso8601::<SERDE_CONFIG>).map_err(D::Error::custom)
    }
}

pub mod option_four_year_iso8601 {
    use serde::de::Error as _;
    use serde::{Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;
    use time::format_description::well_known::Iso8601;

    use crate::serde::four_year_iso8601::SERDE_CONFIG;

    pub fn serialize<S>(value: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => crate::serde::four_year_iso8601::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => Ok(Some(
                OffsetDateTime::parse(&s, &Iso8601::<SERDE_CONFIG>).map_err(D::Error::custom)?,
            )),
            None => Ok(None),
        }
    }
}

#[allow(dead_code)]
pub mod option_u64_as_string {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => Ok(Some(u64::from_str(&s).map_err(serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}

// Custom module to handle Option<OffsetDateTime> with ISO8601
pub mod option_time_iso8601 {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => time::serde::iso8601::serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => {
                let dt = OffsetDateTime::parse(
                    &s,
                    &time::format_description::well_known::Iso8601::DEFAULT,
                )
                .map_err(serde::de::Error::custom)?;
                Ok(Some(dt))
            }
            None => Ok(None),
        }
    }
}

#[allow(dead_code)]
pub mod bigint_as_string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{Deserialize, Deserializer, Serializer, de};

    #[allow(dead_code)]
    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    #[allow(dead_code)]
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

#[allow(dead_code)]
pub mod unix_time_milliseconds_to_offset_date_time {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(
            (date.unix_timestamp_nanos() / 1_000_000_i128)
                .try_into()
                .unwrap(),
        )
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let time = i64::deserialize(deserializer)?;
        OffsetDateTime::from_unix_timestamp_nanos(time as i128 * 1_000_000_i128)
            .map_err(serde::de::Error::custom)
    }
}

#[allow(dead_code)]
pub mod unix_time_milliseconds_to_option_offset_date_time {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    use crate::serde::unix_time_milliseconds_to_offset_date_time;

    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => unix_time_milliseconds_to_offset_date_time::serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let time = i64::deserialize(deserializer)?;
        if time < 0 {
            return Ok(None);
        }

        let dt = OffsetDateTime::from_unix_timestamp_nanos(time as i128 * 1_000_000_i128)
            .map_err(serde::de::Error::custom)?;
        Ok(Some(dt))
    }
}
