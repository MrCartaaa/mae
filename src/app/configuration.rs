use secrecy::{ExposeSecret, SecretString};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgSslMode};

// get the configuration from the YAML files located in 'configuration' directory
pub fn get_configuration<S: for<'a> serde::Deserialize<'a>>()
-> Result<Settings<S>, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    settings.try_deserialize::<Settings<S>>()
}

//
// DEFAULT CONFIGS
//

// Settings: where T is the application specific settings, gathered under 'custom' in the
// configuration YAML files
#[derive(serde::Deserialize, Clone)]
pub struct Settings<T> {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub redis_uri: SecretString,
    pub custom: T,
}
// DATABASE SETTINGS
#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn connect_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .password(self.password.expose_secret())
            .username(&self.username)
            .ssl_mode(ssl_mode)
            .database(&self.database_name)
    }

    pub fn get_connection_pool(&self) -> PgPool {
        PgPoolOptions::new().connect_lazy_with(self.connect_options())
    }
}

// APPLICATION & ENVIRONMENT
#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub base_url: String,
    pub hmac_secret: SecretString,
}

pub enum Environment {
    Local,
    Production,
    Test,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
            Environment::Test => "test",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            "test" => Ok(Self::Test),
            other => Err(format!("{} is not a supported environment", other)),
        }
    }
}
