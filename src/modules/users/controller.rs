use actix_web::Result;
use actix_web::{delete, patch, post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::modules::error::custom::{CustomError, CustomErrorType};
use crate::modules::users::services::find_by_id;
use validator::{Validate, ValidationError};

use super::structs::User;

async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("traz usuario")
}

#[derive(Deserialize, Debug, Validate)]
struct PathShow {
    #[validate(length(min = 5, max = 10, message = "id invalido", code = "400"))]
    id: String,
    // #[validate(length(min = 5, max = 10, message = "name invalido", code = "404"))]
    // name: String,
}

async fn show(data: web::Path<PathShow>) -> Result<web::Json<User>, CustomError> {
    match data.validate() {
        Ok(_) => (),
        Err(err) => {
            return Err(err.into());
        }
    };
    let id = data.id.to_string();
    let mut user: User = User::origin();
    match find_by_id::execute(&id) {
        Ok(value) => user = value,
        Err(err) => return Err(err),
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
