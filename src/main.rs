mod modules;
mod utils;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_web_requestid::RequestIDMiddleware;
use env_logger::Env;
use modules::{health::controllers::users_scope_config, users::controllers::health_scope_config};
use utils::obfuscator_part_of_value;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello_two")]
async fn hello_two() -> impl Responder {
    println!("###############");
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
    HttpServer::new(|| {
        App::new()
            .wrap(RequestIDMiddleware::default())
            .wrap(
                Logger::new(
                    "%Authorization: %{Authorization}xi request-id: %{request-id} %a %{User-Agent}i",
                )
                .exclude("/healthcheck")
                .custom_request_replace("Authorization", |req| {
                    obfuscator_part_of_value(req.headers().get("Authorization"))
                }),
            )
            .configure(config)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
