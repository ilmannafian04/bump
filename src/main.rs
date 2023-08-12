mod api;
mod cli;
mod config;
mod error;
mod handlers;
mod route;
mod services;
mod stores;
mod worker;

use clap::Parser;
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    };
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = cli::Args::parse();
    debug!("args dump: {:?}", &args);

    info!(
        "starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    info!("building configuration");
    let app_config = config::AppConfig::new(&args);
    debug!("config dump: {:?}", &app_config);

    info!("spawning tasks");
    let config_clone = app_config.clone();
    actix_rt::spawn(async move { worker::run(&config_clone).await });
    let api_handle = actix_rt::spawn(async move { api::run(&app_config).await });

    api_handle.await.expect("failed to spawn api")
}
