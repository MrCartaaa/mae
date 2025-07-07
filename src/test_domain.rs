use chrono::{DateTime, Utc};
use serde_json::{Map, Value};

// TODO: should have a prelude dir so that we dont have to explicitely import the above (these are
// required for the #[domain] attr)

#[domain]
#[allow(dead_code)]
pub struct Domain {
    pub value: u64,
}
