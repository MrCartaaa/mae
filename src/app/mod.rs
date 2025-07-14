pub mod app;
pub mod build;
pub mod configuration;
mod run;
pub use run::*;

pub mod prelude {
    pub use crate::app::app;
    pub use crate::app::build::{App, ApplicationBaseUrl, HmacSecret, Run};
    pub use crate::app::run::run;
    pub use crate::middleware::get_session;

    pub use actix_web::dev::Server;
    pub use actix_web::middleware::from_fn;
    pub use actix_web::{App as ActixWebApp, HttpServer, web};
    pub use secrecy::SecretString;
    pub use sqlx::PgPool;
    pub use std::net::TcpListener;
    pub use tracing_actix_web::TracingLogger;
}
