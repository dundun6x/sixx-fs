use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Serialize, Deserialize)]
pub struct FileTimes {
    pub created: Option<i64>,
    pub modified: Option<i64>,
    pub accessed: Option<i64>,
}

impl From<std::fs::Metadata> for FileTimes {
    fn from(value: std::fs::Metadata) -> Self {
        Self {
            created: nanos(value.created()),
            modified: nanos(value.modified()),
            accessed: nanos(value.accessed()),
        }
    }
}

fn nanos(result: std::io::Result<SystemTime>) -> Option<i64> {
    match result.ok() {
        None => None,
        Some(time) => match time.duration_since(UNIX_EPOCH) {
            Ok(dur) => u128_to_i64(dur.as_nanos()),
            Err(err) => u128_to_i64(err.duration().as_nanos()).map(|num| -num)
        }
    }
}

fn u128_to_i64(num: u128) -> Option<i64> {
    match num <= i64::MAX as u128 {
        true => Some(num as i64),
        false => None
    }
}