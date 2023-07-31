use actix_web::{middleware::Logger, web};

use crate::handlers as h;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/v1")
                .route("/ping", web::get().to(h::ping::ping))
                .wrap(Logger::default()),
        ),
    )
    .service(
        web::scope("/health")
            .route("/live", web::get().to(h::health::live))
            .route("/ready", web::get().to(h::health::ready)),
    );
}
