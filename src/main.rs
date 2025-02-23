use anyhow::Result;
use clap::Parser;
use sellershut_services::{tracing::TracingBuilder, Configuration, Services};
use tracing::error;
use users_service::{state::AppState, AppConfig};

/// users-service
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config_file: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(not(debug_assertions))]
    let args = Args::parse();

    let _config_path = "users.toml";
    #[cfg(not(debug_assertions))]
    let config_path = args
        .config_file
        .to_str()
        .expect("config file path is not valid");

    let config = config::Config::builder()
        .add_source(config::File::new(_config_path, config::FileFormat::Toml))
        .build()
        .inspect_err(|e| error!("config file: {e}"))?;

    let config = config
        .try_deserialize::<Configuration>()
        .inspect_err(|e| error!("config deserialise: {e}"))?;

    let _app_config: AppConfig = serde_json::from_value(config.misc.clone())
        .inspect_err(|e| error!("config misc deserialise: {e}"))?;

    let _tracing = TracingBuilder::new().build(config.application.log_level);

    let (tx, _rx) = tokio::sync::oneshot::channel();

    let services = Services::builder()
        .postgres(&config.database)
        .await
        .inspect_err(|e| error!("database: {e}"))?
        .build();

    let state = AppState::new(config.application.port, services);

    users_service::run(state, tx).await?;

    Ok(())
}
