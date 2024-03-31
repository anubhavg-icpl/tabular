use actix_web::{get, App, HttpServer, Responder};
use askama::Template;
use std::fs::File;
use std::io::{BufReader, Error};
use csv::Reader;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    data: Vec<Vec<String>>,
}

fn read_csv_data(file_path: &str) -> Result<Vec<Vec<String>>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = Reader::from_reader(reader);
    let mut data = Vec::new();
    for result in csv_reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        data.push(row);
    }
    Ok(data)
}

#[get("/dashboard")]
async fn dashboard() -> impl Responder {
    let data = match read_csv_data("src/data.csv") {
        Ok(data) => data,
        Err(_) => Vec::new(), // Handle error appropriately
    };
    let template = DashboardTemplate { data };
    template.render().unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(dashboard)
    })
    .bind("127.0.0.1:8080")? 
    .run()
    .await
}
