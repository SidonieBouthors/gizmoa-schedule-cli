use std::collections::HashMap;

use chrono::{NaiveTime, Timelike};
use serde::{ser::SerializeMap, Deserialize, Serialize};

use crate::model::MeetTime;

impl Serialize for MeetTime {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_map(Some(2))?;
    state.serialize_entry("hour", &self.0.hour())?;
    state.serialize_entry("minute", &self.0.minute())?;
    state.end()
  }
}

impl<'de> Deserialize<'de> for MeetTime {
  fn deserialize<D>(deserializer: D) -> Result<MeetTime, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = HashMap::<String, u32>::deserialize(deserializer)?;
    NaiveTime::from_hms_opt(s["Hour"], s["Minute"], 0)
      .map(|time| MeetTime(time))
      .ok_or_else(|| serde::de::Error::custom("Invalid time"))
  }
}

pub mod weekday_ser {
  use std::collections::HashMap;

  use crate::model::Weekday;
  use serde::{ser::SerializeMap, Deserialize, Deserializer, Serializer};
  use strum::IntoEnumIterator;

  pub fn serialize<S>(value: &Vec<Weekday>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_map(Some(0))?;
    for weekday in Weekday::iter() {
      state.serialize_entry(
        weekday.to_string().to_lowercase().as_str(),
        &value.contains(&weekday),
      )?;
    }
    state.end()
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Weekday>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = HashMap::<String, bool>::deserialize(deserializer)?;
    Ok(
      Weekday::iter()
        .filter(|weekday| {
          s.get(&weekday.to_string().to_lowercase())
            .unwrap_or(&false)
            .clone()
        })
        .collect(),
    )
  }
}
