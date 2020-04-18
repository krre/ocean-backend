use crate::model::schema::topics;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub description: String,
    #[serde(with = "date_serializer")]
    pub create_ts: NaiveDateTime,
    #[serde(with = "date_serializer")]
    pub update_ts: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "topics"]
pub struct NewTopic<'a> {
    pub title: &'a str,
    pub description: &'a str,
}

pub fn time_to_json(t: NaiveDateTime) -> String {
    DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
}

mod date_serializer {
    use super::*;
    // use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use serde::{Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        time: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        time_to_json(time.clone()).serialize(serializer)
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
