use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::services;

pub async fn get_html(req: HttpRequest) -> impl Responder {
    let path = &req.uri().to_string()[1..];
    let file_name = format!("{}.html", &path);

    match services::assets::get_text_file(&file_name) {
        Ok(asset) => HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(&file_name)
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .body(asset),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_raw_or_index(path: web::Path<String>) -> impl Responder {
    let file_name = if &path.to_string() == &"" {
        "index.html"
    } else {
        &path
    };

    if let Ok(asset) = services::assets::get_text_file(&file_name) {
        return HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(file_name)
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .body(asset);
    };

    match services::assets::get_raw(&file_name) {
        Ok(data) => HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(file_name)
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .body(data),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
