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
