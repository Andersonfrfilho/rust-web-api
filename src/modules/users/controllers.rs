use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, Scope};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/:user_id")]
async fn show() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[patch("/:user_id")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/")]
async fn create(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[delete("/:user_id")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// pub fn routes() -> impl Scope {
//     return web::scope("/users");
// }
