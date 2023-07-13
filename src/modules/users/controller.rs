use actix_web::dev::Path;
use actix_web::web::Json;
use actix_web::Result;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::modules::users::services::find_by_id;
use crate::modules::users::services::find_by_id::MyError;

use super::structs::User;

async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("traz usuario")
}

#[derive(Deserialize)]
struct PathShow {
    id: String,
}

async fn show(data: web::Path<PathShow>) -> Result<web::Json<User>, actix_web::Error> {
    let id = data.id.to_string();
    let mut user: User = User::origin();
    let mut error: MyError = MyError::origin();
    match find_by_id::execute(&id) {
        Ok(result) => user = result,
        Err(err) => error = err,
    };

    Ok(web::Json(user))
}

#[patch("/:user_id")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("update usuario")
}

#[post("/")]
async fn create(info: web::Json<String>) -> Result<String> {
    Ok(format!("Welcome {}!", info.to_string()))
}

#[delete("/:user_id")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("deleta usuario")
}

pub fn users_scope_config(cfg: &mut web::ServiceConfig) {
    let route_index = web::resource("/").route(web::get().to(index));
    let route_show = web::resource("/{id}").route(web::get().to(show));
    cfg.service(
        web::scope("/v1/users")
            .service(create)
            .service(route_index)
            .service(route_show)
            .service(update)
            .service(delete),
    );
}
