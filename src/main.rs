mod config;
mod error;
mod handlers;
mod route;
mod services;
mod stores;

use actix_web::{App, HttpServer};
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

    info!(
        "binding http server to {}:{}",
        &app_config.host, &app_config.port
    );
    HttpServer::new(|| App::new().configure(route::configuration))
        .bind((app_config.host, app_config.port))?
        .run()
        .await
}
