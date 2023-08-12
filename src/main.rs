mod api;
mod cli;
mod config;
mod error;
mod handlers;
mod route;
mod services;
mod stores;
mod worker;

use std::env;

use clap::Parser;
use dotenv::dotenv;
use log::{debug, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    };

    let args = cli::Args::parse();
    let app_config = config::AppConfig::new(&args);

    if env::var("RUST_LOG").is_err() || app_config.log_level != env::var("RUST_LOG").unwrap() {
        env::set_var("RUST_LOG", &app_config.log_level);
    };
    env_logger::init();

    info!(
        "starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    debug!("args dump: {:?}", &args);
    debug!("config dump: {:?}", &app_config);

    info!("spawning tasks");
    let config_clone = app_config.clone();
    actix_rt::spawn(async move { worker::run(&config_clone).await });
    actix_rt::spawn(async move { api::run(&app_config).await })
        .await
        .expect("failed spawning api")
}
