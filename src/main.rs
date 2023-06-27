mod constants;
mod middlewares;
mod modules;
mod utils;

use actix_web::{
    get, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use constants::AUTHORIZATION;
use env_logger::Env;
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::schema::{Object, ObjectBuilder},
    IntoParams, OpenApi, PartialSchema, ToSchema,
};
use utoipa_swagger_ui::SwaggerUi;

use modules::{health::controller::health_scope_config, users::controller::users_scope_config};
use utils::obfuscator_part_of_value;

#[utoipa::path(
     get,
     path = "/hello",
     responses(
         (status = 200,
             examples(
                //  ("Demo" = (summary = "This is summary", description = "Long description",
                //              value = json!(User{name: "Demo".to_string()}))),
                 ("John" = (summary = "Another user", value = json!({"name": "John"})))
              )
         )
     )
 )]
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello_two")]
async fn hello_two(_req: HttpRequest) -> impl Responder {
    println!("{:?}", _req);
    HttpResponse::Ok().body("Hello world! two")
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(users_scope_config)
        .configure(health_scope_config)
        .service(hello_two);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    #[derive(OpenApi)]
    #[openapi(paths(hello))]
    struct ApiDoc;

    HttpServer::new(|| {
        App::new()
        .wrap(
            Logger::new(
                "%a %t %r %s %b %{Referer}i %{User-Agent}i %T payload: %b %Authorization: %{Authorization}xi requestid: %{x-request-id}i",
            )
            .exclude("/healthcheck")
            .custom_request_replace(&AUTHORIZATION, |req| {
                obfuscator_part_of_value(req.headers().get(AUTHORIZATION))
            }),
        )
        .wrap(middlewares::request_id::RequestId::default())
            .configure(config).service(
                                 SwaggerUi::new("/doc/{_:.*}")
                                     .url("/api-docs/openapi.json", ApiDoc::openapi()),
                             )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
