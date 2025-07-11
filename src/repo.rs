pub mod prelude {
    pub use crate::request_context::RequestContext;
    pub use chrono::{DateTime, Utc};
    pub use domain_macro::*;
    pub use serde_json::{Map, Value};
    use sqlx;
    use sqlx::Database;

    #[derive(sqlx::Type)]
    #[sqlx(type_name = "domain_status")]
    pub enum DomainStatus {
        Incomplete,
        Active,
        Deleted,
        Archived,
    }
}
