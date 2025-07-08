pub mod prelude {
    pub use chrono::{DateTime, Utc};
    pub use domain_macro::*;
    pub use serde_json::{Map, Value};
    pub enum DomainStatus {
        Incomplete,
        Active,
        Deleted,
        Archived,
    }
}
