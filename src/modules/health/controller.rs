use actix_web::{get, web, HttpResponse, Responder, Result};
use serde::Serialize;
use utoipa::openapi::schema;

use crate::modules::common::error::{BadRequest, InternalServerError, NotFound};

#[derive(utoipa::ToResponse, Serialize, utoipa::ToSchema)]
pub struct HealthSuccessResponse {
    #[schema(example = true, default = true)]
    up: bool,
}
#[derive(utoipa::ToResponse)]
struct Response {
    message: String,
}

#[derive(utoipa::IntoResponses)]
enum HealthResponses {
    /// Success response description.
    #[response(status = 200)]
    Success(#[to_response] HealthSuccessResponse),

    #[response(status = 404)]
    NotFound(#[ref_response] NotFound),

    #[response(status = 400)]
    BadRequest(#[ref_response] BadRequest, NotFound),

    #[response(status = 500)]
    ServerError(#[to_response] InternalServerError),
}
#[utoipa::path(get, path = "/health", tag = "health", responses(HealthResponses))]
pub async fn health() -> Result<impl Responder> {
    let obj = HealthSuccessResponse { up: true };
    Ok(web::Json(obj))
}

pub fn health_scope_config(cfg: &mut web::ServiceConfig) {
    let route_health = web::resource("/health").route(web::get().to(health));
    cfg.service(route_health);
}
