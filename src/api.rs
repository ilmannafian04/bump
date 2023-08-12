use actix_web::{App, HttpServer};
use log::info;

use crate::{config::AppConfig, route};

pub async fn run(app_config: &AppConfig) -> std::io::Result<()> {
    let bind_address = (app_config.api_host.clone(), app_config.api_port);
    info!("binding server to {}:{}", bind_address.0, bind_address.1);
    HttpServer::new(|| App::new().configure(route::configuration))
        .bind(bind_address)?
        .run()
        .await
}
