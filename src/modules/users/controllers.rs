use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web::{App, HttpServer, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("traz usuario")
}

#[get("/:user_id")]
async fn show(name: web::Path<String>) -> Result<impl Responder> {
    println!("{}", name);
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

#[patch("/:user_id")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("update usuario")
}

#[post("/")]
async fn create(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[delete("/:user_id")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("deleta usuario")
}

pub fn users_scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .service(create)
            .service(index)
            .service(show)
            .service(update)
            .service(delete),
    );
}
