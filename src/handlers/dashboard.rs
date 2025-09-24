use actix_web::{web, HttpResponse};
use askama::Template;

use crate::models::DashboardTemplate;
use crate::services::CsvService;

pub async fn dashboard_handler(dataset: web::Query<DatasetQuery>) -> HttpResponse {
    let file_path = dataset.file.as_deref().unwrap_or("data/security_alerts.csv");

    match CsvService::read_csv_data(file_path) {
        Ok(data) => {
            let template = DashboardTemplate { data };
            match template.render() {
                Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
                Err(err) => {
                    eprintln!("Template rendering error: {}", err);
                    HttpResponse::InternalServerError().body("Error rendering template")
                }
            }
        }
        Err(err) => {
            eprintln!("CSV reading error from {}: {}", file_path, err);
            let template = DashboardTemplate { data: Vec::new() };
            match template.render() {
                Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
                Err(_) => HttpResponse::InternalServerError().body("Error loading data"),
            }
        }
    }
}

#[derive(serde::Deserialize)]
pub struct DatasetQuery {
    file: Option<String>,
}