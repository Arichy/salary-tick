use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::i18n::Language;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub daily_salary: f64,
    #[serde(with = "serde_time")]
    pub start_time: NaiveTime,
    #[serde(with = "serde_time")]
    pub end_time: NaiveTime,
    pub currency_symbol: char,
    pub language: Language,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            daily_salary: 0.0,
            start_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            end_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            currency_symbol: '$',
            language: Language::EN,
        }
    }
}

mod serde_time {
    use chrono::NaiveTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%H:%M";

    pub fn serialize<S: Serializer>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error> {
        let s = format!("{}", time.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveTime, D::Error> {
        let s = String::deserialize(deserializer)?;
        NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
