use actix_web::HttpResponse;

pub async fn index_handler() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "/dashboard"))
        .finish()
}
