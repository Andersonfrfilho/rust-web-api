use serde::Serialize;

trait Custom {
    fn new(code: i32, message: &String) -> Self;
}

#[derive(utoipa::ToSchema, Serialize)]
pub struct BadRequest {
    code: i32,
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
    /// Property responsible for application health status.
    #[schema(example = true, example = false, default = false)]
    code: i32,
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
    code: i32,
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
