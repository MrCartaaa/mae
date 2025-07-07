use chrono::{DateTime, Utc};
use serde_json::{Map, Value};

#[domain]
#[allow(dead_code)]
pub struct Domain {
    pub value: u64,
}
