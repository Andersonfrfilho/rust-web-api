mod modules;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use modules::users::controllers;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello_two")]
async fn hello_two() -> impl Responder {
    println!("###############");
    HttpResponse::Ok().body("Hello world!")
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_two);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("##########9#");
    HttpServer::new(|| App::new().configure(config))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
