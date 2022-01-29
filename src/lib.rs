pub mod changetime;
pub mod metrics;

use std::path::PathBuf;

use chrono::serde::ts_milliseconds_option;

use chrono::{DateTime, Duration, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    // as milliseconds
    pub debt: u64,
    pub sleep_duration: Option<u64>,
    #[serde(with = "ts_milliseconds_option")]
    pub date_sleep: Option<DateTime<Utc>>,
}
pub struct State {
    pub data_path: Option<PathBuf>,
    pub now: Option<DateTime<Local>>,
    pub tomorrow: Option<DateTime<Local>>,
    pub duration_sleep: Option<Duration>,
    pub user: Option<User>,
}
