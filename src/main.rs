mod modules;
use actix_web::{
    get, http::header::HeaderValue, middleware::Logger, web, App, HttpResponse, HttpServer,
    Responder,
};
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

fn parse_jwt_id(_req: Option<&HeaderValue>) -> String {
    print!("#####-entrou no loger");
    print!("{:?}", &_req);
    "jwt_uid".to_owned()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(
                Logger::new("%P Authorization %{Authorization}xi %a %{User-Agent}i")
                    .exclude("/healthcheck")
                    .custom_request_replace("Authorization", |req| {
                        parse_jwt_id(req.headers().get("Authorization"))
                    }),
            )
            .configure(config)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
