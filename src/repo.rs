pub mod prelude {
    pub use crate::request_context::RequestContext;
    pub use chrono::{DateTime, Utc};
    pub use domain_macro::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{Map, Value};
    use sqlx;

    #[derive(sqlx::Type, Deserialize, Serialize)]
    #[sqlx(type_name = "status", rename_all = "lowercase")]
    pub enum DomainStatus {
        Incomplete,
        Active,
        Deleted,
        Archived,
    }
}
