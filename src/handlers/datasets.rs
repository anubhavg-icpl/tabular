use actix_web::HttpResponse;
use serde::Serialize;
use tera::Context;

use crate::services::CsvService;
use crate::template_engine::TEMPLATES;

#[derive(Serialize)]
struct Dataset {
    name: String,
    path: String,
}

pub async fn list_datasets_handler() -> HttpResponse {
    let datasets = CsvService::get_available_datasets()
        .into_iter()
        .map(|(name, path)| Dataset { name, path })
        .collect::<Vec<_>>();

    let mut context = Context::new();
    context.insert("datasets", &datasets);

    match TEMPLATES.render("datasets.html", &context) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(err) => {
            eprintln!("Template rendering error: {}", err);
            HttpResponse::InternalServerError().body("Error rendering template")
        }
    }
}