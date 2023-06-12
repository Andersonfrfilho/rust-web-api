mod modules;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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
    println!("##########9#");
    HttpServer::new(|| App::new().configure(config))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
