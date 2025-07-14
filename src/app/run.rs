use crate::app::build::{App, DeriveContext, Run};
use crate::app::configuration::get_configuration;
use crate::telemetry::{get_subscriber, init_subscriber};
use serde::Deserialize;

pub async fn run<
    Settings: for<'a> Deserialize<'a> + DeriveContext<Context> + Clone,
    Context: Clone + Send + 'static,
    Application: App + Run,
>(
    service_name: &str,
    log_level: &str,
) -> anyhow::Result<()> {
    let subscriber = get_subscriber(service_name.into(), log_level.into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration::<Settings>().expect("Failed to read config file");

    let application = Application::build::<Settings, Context>(config.clone()).await?;

    application.run_until_stopped().await?;

    Ok(())
}
