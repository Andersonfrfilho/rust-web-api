use core::fmt;

use serde::Serialize;

trait Custom {
    fn new(code: i32, message: &String) -> Self;
}

#[derive(utoipa::ToSchema, Serialize)]
pub struct BadRequest {
    /// Property represented custom code error
    #[schema(example = 4000, default = 0)]
    code: i32,
    /// Property represented custom message error
    #[schema(example = "Bad request error in flow", default = "")]
    message: String,
}

impl Custom for BadRequest {
    fn new(code: i32, message: &String) -> BadRequest {
        return BadRequest {
            code,
            message: message.clone(),
        };
    }
}

#[derive(Serialize, utoipa::ToSchema)]
#[schema(example = json!(NotFound{code: 4001, message: "Resource not found".to_string()}))]
pub struct NotFound {
    /// Property represented custom code error
    #[schema(example = 4004, default = 0)]
    code: i32,
    /// Property represented custom message error
    #[schema(example = "Route not found", default = "")]
    message: String,
}

impl Custom for NotFound {
    fn new(code: i32, message: &String) -> NotFound {
        return NotFound {
            code,
            message: message.clone(),
        };
    }
}

#[derive(utoipa::ToSchema, Serialize)]
pub struct InternalServerError {
    /// Property represented custom code error
    #[schema(example = 5000, default = 0)]
    code: i32,
    /// Property represented custom message error
    #[schema(example = "Internal Server Error verify logs", default = "")]
    message: String,
}

impl Custom for InternalServerError {
    fn new(code: i32, message: &String) -> InternalServerError {
        return InternalServerError {
            code,
            message: message.clone(),
        };
    }
}

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
