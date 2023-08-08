use actix_web::{middleware::Logger, web};

use crate::handlers as h;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    health_configuration(cfg);
    api_configuration(cfg);
    asset_configuration(cfg);
}

fn health_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("/live", web::get().to(h::health::live))
            .route("/ready", web::get().to(h::health::ready)),
    );
}

fn api_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/v1").route("/ping", web::get().to(h::ping::ping)))
            .wrap(Logger::default()),
    );
}

fn asset_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/test", web::get().to(h::assets::get_html))
            .route("/{_:.*}", web::get().to(h::assets::get_raw_or_index))
            .wrap(Logger::default()),
    );
}
