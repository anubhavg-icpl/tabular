use crate::handlers::{dashboard_handler, index_handler, list_datasets_handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index_handler))
        .route("/dashboard", web::get().to(dashboard_handler))
        .route("/datasets", web::get().to(list_datasets_handler));
}
