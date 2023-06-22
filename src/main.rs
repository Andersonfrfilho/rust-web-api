mod constants;
mod middlewares;
mod modules;
mod utils;

use actix_web::{
    get, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use constants::AUTHORIZATION;
use env_logger::Env;
use modules::{health::controllers::users_scope_config, users::controllers::health_scope_config};
use utils::obfuscator_part_of_value;
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
    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
        .wrap(
            Logger::new(
                "payload: %b %Authorization: %{Authorization}xi requestid: %{x-request-id}i %a %{User-Agent}i",
            )
            .exclude("/healthcheck")
            .custom_request_replace(&AUTHORIZATION, |req| {
                obfuscator_part_of_value(req.headers().get(AUTHORIZATION))
            }),
        )
        .wrap(middlewares::request_id::RequestId::default())
            .configure(config)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
