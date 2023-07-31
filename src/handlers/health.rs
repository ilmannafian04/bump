use actix_web::{HttpResponse, Responder};

pub async fn live() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn ready() -> impl Responder {
    HttpResponse::Ok().finish()
}
