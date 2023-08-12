mod api;
mod config;
mod error;
mod handlers;
mod route;
mod services;
mod stores;
mod worker;

use dotenv::dotenv;
use env_logger::Env;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    };
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!(
        "starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    info!("building configuration");
    let app_config = config::AppConfig::new();

    info!("spawning tasks");
    let worker_handle = actix_rt::spawn(async move { worker::run().await });
    let api_handle = actix_rt::spawn(async move { api::run(&app_config).await });

    worker_handle.await.expect("failed to spawn worker");
    api_handle.await.expect("failed to spawn api")
}
