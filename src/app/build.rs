use crate::app::configuration::Settings;
use actix_web::dev::Server;
use secrecy::SecretString;
use sqlx::PgPool;
use std::net::TcpListener;

pub trait Run: App {
    async fn run_until_stopped(self) -> Result<(), std::io::Error>
    where
        Self: Sized,
    {
        self.server().await
    }

    async fn run<C: Clone + Send + 'static>(
        listener: TcpListener,
        db_pool: PgPool,
        base_url: String,
        hmac_secret: SecretString,
        redis_uri: SecretString,
        custom_context: C,
    ) -> Result<Server, anyhow::Error>;
}

pub trait App {
    fn new(port: u16, server: Server) -> Self;
    fn port(&self) -> u16
    where
        Self: Sized;
    fn server(self) -> Server
    where
        Self: Sized;

    async fn build<T, C>(config: Settings<T>) -> Result<Self, anyhow::Error>
    where
        Self: Sized + Run,
        T: DeriveContext<C>,
        C: Clone + Send + 'static,
    {
        let connection_pool = config.database.get_connection_pool();

        let address = format!("{}:{}", config.application.host, config.application.port);

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let context = config.custom.context();

        let server = Self::run(
            listener,
            connection_pool,
            config.application.base_url,
            config.application.hmac_secret,
            config.redis_uri,
            context,
        )
        .await?;

        Ok(Self::new(port, server))
    }
}

pub trait DeriveContext<C> {
    fn context(&self) -> C
    where
        Self: Sized;
}

pub struct ApplicationBaseUrl(pub String);

#[derive(Clone)]
pub struct HmacSecret(pub SecretString);
