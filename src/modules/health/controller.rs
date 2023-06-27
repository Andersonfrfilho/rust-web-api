use actix_web::{get, web, HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/healthcheck",
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
#[get("/healthcheck")]
async fn show() -> impl Responder {
    HttpResponse::Ok().body("health check")
}

pub fn health_scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(show);
}
