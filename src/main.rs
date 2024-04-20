use clap::{Parser, Subcommand};
use anyhow::{Ok, Result};
use tracing::info;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use validator::Validate;
use axum::{routing::get, Router};
use std::{env, fs, time::Duration};

use pump::{config, utils};

#[derive(Debug, Parser)]
#[clap(name = "pump", version = utils::version::get_version(), author = "Aidan")]
#[clap(propagate_version = false)]
struct Cli {
    #[clap(short, long, default_value = "app.toml")]
    config: Option<String>,

    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {

    // mirror new pool
    #[clap(name = "pump")]
    Pump,

    // start web server
    #[clap(name = "web")]
    Web,

    // start alter server, will send alter massage to dingtalk or telegram
    #[clap(name = "alter")]
    Alter,
}

#[tokio::main]
async fn main() -> Result<()> {
    utils::log::init_tracing();

    let cli = Cli::parse();

    let config_path = cli.config.unwrap();
    let c: config::Config = fs::read_to_string(config_path.clone())?.parse()?;
    c.validate()?;

    env::set_var("PUMP_CONFIG", config_path);

    match cli.subcmd {
        Commands::Pump => {
            info!("start pump");
        },
        Commands::Alter => {
            info!("start alter");
        },
        Commands::Web => {
            info!("start web");
            let layer = ServiceBuilder::new()
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .layer(TimeoutLayer::new(Duration::from_secs(c.web.timeout)))
            .into_inner();

            let app = Router::new()
                .route("/", get(|| async { "Hello, axum!" }))
                .layer(layer);
            let listener = tokio::net::TcpListener::bind(&c.web.socket_addr).await?;
            info!("Listening on: {}", c.web.socket_addr);

            axum::serve(listener, app)
            .with_graceful_shutdown(async {
                tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler")
            })
            .await?;
        }

    }
    Ok(())
}