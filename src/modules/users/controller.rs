use actix_web::Result;
use actix_web::{delete, patch, post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::modules::error::custom::Custom;
use crate::modules::users::services::find_by_id;

use super::structs::User;

async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("traz usuario")
}

#[derive(Deserialize)]
struct PathShow {
    id: String,
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

async fn show(data: web::Path<PathShow>) -> Result<web::Json<User>, Custom> {
    let id = data.id.to_string();
    let mut user: User = User::origin();
    let mut error: Custom = Custom::from(String::from(
        "I know it doesn't make sense but had to give an example of UserError",
    ));
    match find_by_id::execute(&id) {
        Ok(result) => user = result,
        Err(err) => {
            error = Custom::from(String::from(
                "I know it doesn't make sense but had to give an example of UserError",
            ))
        }
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
