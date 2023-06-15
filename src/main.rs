mod modules;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use modules::{health::controllers::users_scope_config, users::controllers::health_scope_config};

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
            .wrap(Logger::default())
            .wrap(Logger::new(
                "%P Authorization %{Authorization}i(0..2) %a %{User-Agent}i",
            ))
            .configure(config)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
