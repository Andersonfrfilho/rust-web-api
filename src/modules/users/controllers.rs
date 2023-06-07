use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("traz usuario")
}

#[get("/:user_id")]
async fn show() -> impl Responder {
    HttpResponse::Ok().body("mostra usuario")
}

#[patch("/:user_id")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("update usuario")
}

#[post("/")]
async fn create(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[delete("/:user_id")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("deleta usuario")
}

pub fn health_scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .service(index)
            .service(show)
            .service(update)
            .service(delete),
    );
}
