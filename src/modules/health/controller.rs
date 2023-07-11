use actix_web::{web, Responder, Result};
use serde::Serialize;

use crate::modules::common::error::{BadRequest, InternalServerError};

#[derive(Serialize, utoipa::ToSchema)]
#[schema(example = json!(SuccessResponse{up: false}))]
pub struct SuccessResponse {
    /// Property responsible for application health status.
    #[schema(example = true, default = true)]
    up: bool,
}

#[derive(utoipa::IntoResponses)]
enum HealthResponses {
    /// Success response
    #[response(status = 200)]
    #[allow(dead_code)]
    Success(#[to_schema] SuccessResponse),

    /// Bad request response error
    #[response(status = 400)]
    #[allow(dead_code)]
    BadRequest(#[to_schema] BadRequest),

    /// Internal Server error
    #[response(status = 500)]
    #[allow(dead_code)]
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
