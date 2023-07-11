use actix_web::Result;
use actix_web::{delete, get, patch, post, test, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::modules::users::services::find_by_id;
use crate::modules::users::services::find_by_id::MyError;
#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("traz usuario")
}

#[get("/:user_id")]
async fn show(name: web::Path<String>) -> Result<impl Responder, MyError> {
    const value_string: &String = &String::from("strisd");
    let result = find_by_id::execute(&value_string);
    result.map_err(|_e| MyError { name: "asd" })?;
    Ok(web::Json(result))
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
    let route_index = web::resource("/").route(web::get().to(index));
    cfg.service(
        web::scope("/v1/users")
            .service(create)
            .service(route_index)
            .service(show)
            .service(update)
            .service(delete),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = index(req.clone()).await.respond_to(&req);
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = index(req.clone()).await.respond_to(&req);
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
