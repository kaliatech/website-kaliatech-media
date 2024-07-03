use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

// const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

// pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     match date.is_some() {
//         true => {
//             let s = format!("{}", date.as_ref().unwrap().format(FORMAT));
//             serializer.serialize_str(&s)
//         }
//         false => serializer.serialize_none(),
//     }
// }

// Serialize DateTime<Utc> to ISO 8601 string
pub fn serialize_dt<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.to_rfc3339();
    serializer.serialize_str(&s)
}

// Deserialize ISO 8601 string to DateTime<Utc>
pub fn deserialize_dt<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<DateTime<Utc>>().map_err(serde::de::Error::custom)
}
