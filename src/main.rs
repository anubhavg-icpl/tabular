use actix_web::{get, middleware, App, HttpResponse, HttpServer};
use askama::Template;
use csv::Reader;
use std::fs::File;
use std::io::BufReader;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    data: Vec<Vec<String>>,
}

fn read_csv_data(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = Reader::from_reader(reader);

    let mut data = Vec::new();

    // Read headers first
    let headers = csv_reader.headers()?;
    let headers: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    data.push(headers);

    // Read data rows
    for result in csv_reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        data.push(row);
    }

    Ok(data)
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "/dashboard"))
        .finish()
}

#[get("/dashboard")]
async fn dashboard() -> HttpResponse {
    match read_csv_data("src/data.csv") {
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
            eprintln!("CSV reading error: {}", err);
            let template = DashboardTemplate { data: Vec::new() };
            match template.render() {
                Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
                Err(_) => HttpResponse::InternalServerError().body("Error loading data"),
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("🚀 Starting Tabular Dashboard Server");
    println!("📊 Server running at http://127.0.0.1:8080");
    println!("📁 Reading data from: src/data.csv");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(dashboard)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
