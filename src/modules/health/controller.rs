use actix_web::{get, web, HttpResponse, Responder};

#[get("/healthcheck")]
async fn show() -> impl Responder {
    HttpResponse::Ok().body("health check")
}

pub fn health_scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(show);
}
