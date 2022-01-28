use chrono::serde::ts_milliseconds_option;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    // as milliseconds
    pub debt: u64,
    pub sleep_duration: Option<u64>,
    #[serde(with = "ts_milliseconds_option")]
    pub date_sleep: Option<DateTime<Utc>>
}