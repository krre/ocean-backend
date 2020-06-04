use chrono::{DateTime, NaiveDateTime, Utc};

pub mod mandela;
pub mod schema;
pub mod user;
pub mod user_group;

pub fn time_to_json(t: NaiveDateTime) -> String {
    DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
}

pub mod date_serializer {
    use super::*;
    // use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use serde::{Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        time: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        time_to_json(*time).serialize(serializer)
    }

    // pub fn deserialize<'de, D: Deserializer<'de>>(
    //     deserializer: D,
    // ) -> Result<NaiveDateTime, D::Error> {
    //     let time: &str = Deserialize::deserialize(deserializer)?;
    //     Ok(DateTime::parse_from_rfc3339(time)
    //         .map_err(D::Error::custom)?
    //         .naive_utc())
    // }
}
