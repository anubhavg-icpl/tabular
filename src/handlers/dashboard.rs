use actix_web::{web, HttpResponse};
use tera::Context;

use crate::services::CsvService;
use crate::template_engine::TEMPLATES;

pub async fn dashboard_handler(dataset: web::Query<DatasetQuery>) -> HttpResponse {
    let file_path = dataset.file.as_deref().unwrap_or("data/data.csv");

    match CsvService::read_csv_data(file_path) {
        Ok(csv_data) => {
            let mut context = Context::new();

            // Split headers and data
            let (headers, data) = if !csv_data.is_empty() {
                let headers = csv_data[0].clone();
                let data = csv_data[1..].to_vec();
                (headers, data)
            } else {
                (vec![], vec![])
            };

            context.insert("title", "Security Dashboard");
            context.insert("headers", &headers);
            context.insert("data", &data);

            match TEMPLATES.render("dashboard.html", &context) {
                Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
                Err(err) => {
                    eprintln!("Template rendering error: {}", err);
                    HttpResponse::InternalServerError().body("Error rendering template")
                }
            }
        }
        Err(err) => {
            eprintln!("CSV reading error from {}: {}", file_path, err);
            let mut context = Context::new();
            context.insert("title", "Security Dashboard");
            context.insert("headers", &Vec::<String>::new());
            context.insert("data", &Vec::<Vec<String>>::new());

            match TEMPLATES.render("dashboard.html", &context) {
                Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
                Err(_) => HttpResponse::InternalServerError().body("Error loading data"),
            }
        }
    }
}

#[derive(serde::Deserialize)]
pub struct DatasetQuery {
    pub file: Option<String>,
}