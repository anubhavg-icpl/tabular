use actix_web::{web, HttpResponse};
use std::path::Path;
use tera::Context;

use crate::services::CsvService;
use crate::template_engine::TEMPLATES;

pub async fn dashboard_handler(dataset: web::Query<DatasetQuery>) -> HttpResponse {
    let default_file = CsvService::get_available_datasets()
        .first()
        .map(|(_, path)| path.clone())
        .unwrap_or_else(|| "data/data.csv".to_string());

    let file_path = dataset.file.as_deref().unwrap_or(&default_file);

    match CsvService::read_csv_data(file_path) {
        Ok(csv_data) => {
            let mut context = Context::new();

            // Analyze the data for charts and statistics
            let analysis = CsvService::analyze_csv_data(&csv_data);

            // Split headers and data
            let (headers, data) = if !csv_data.is_empty() {
                let headers = csv_data[0].clone();
                let data = csv_data[1..].to_vec();
                (headers, data)
            } else {
                (vec![], vec![])
            };

            // Get filename for display
            let filename = Path::new(file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("data");

            context.insert("title", "Data Analytics Dashboard");
            context.insert("filename", filename);
            context.insert("headers", &headers);
            context.insert("data", &data);
            context.insert("analysis", &analysis);

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
            context.insert("title", "Data Analytics Dashboard");
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
