use actix_web::{get, web, HttpResponse, Responder, Result};
use serde::Serialize;
use utoipa::openapi::schema;

use crate::modules::common::error::{BadRequest, InternalServerError, NotFound};

#[derive(Serialize, utoipa::ToSchema)]
#[schema(example = json!(SuccessResponse{up: false}))]
pub struct SuccessResponse {
    /// Property responsible for application health status.
    #[schema(example = true, example = false, default = false)]
    up: bool,
}

#[derive(utoipa::IntoResponses)]
enum HealthResponses {
    /// Success response
    #[response(status = 200)]
    Success(#[to_schema] SuccessResponse),

    /// Success response description.
    #[response(status = 404)]
    NotFound(#[to_schema] NotFound),
    #[response(status = 400)]
    BadRequest(#[to_schema] BadRequest, NotFound),
    #[response(status = 500)]
    ServerError(#[to_schema] InternalServerError),
}

/// Route to verify health application
#[utoipa::path(get, path = "/health", tag = "health", responses(HealthResponses),
// request_body(content = SuccessResponse,
//     examples(
//         ("Value1" = (value = json!({"value": "this is value"}) ) ),
//         ("Value2" = (value = json!({"value": "this is value2"}) ) )
//     )
// )
)]
pub async fn health() -> Result<impl Responder> {
    let obj = SuccessResponse { up: true };
    Ok(web::Json(obj))
}

pub fn health_scope_config(cfg: &mut web::ServiceConfig) {
    let route_health = web::resource("/health").route(web::get().to(health));
    cfg.service(route_health);
}
